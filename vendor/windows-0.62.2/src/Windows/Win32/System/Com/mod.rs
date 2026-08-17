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
    windows_core::link!("ole32.dll" "system" fn BindMoniker(pmk : * mut core::ffi::c_void, grfopt : u32, iidresult : *const windows_core::GUID, ppvresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { BindMoniker(pmk.param().abi(), grfopt, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CLSIDFromProgID<P0>(lpszprogid: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CLSIDFromProgID(lpszprogid : windows_core::PCWSTR, lpclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CLSIDFromProgID(lpszprogid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CLSIDFromProgIDEx<P0>(lpszprogid: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CLSIDFromProgIDEx(lpszprogid : windows_core::PCWSTR, lpclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CLSIDFromString<P0>(lpsz: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CLSIDFromString(lpsz : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CLSIDFromString(lpsz.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoAddRefServerProcess() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoAddRefServerProcess() -> u32);
    unsafe { CoAddRefServerProcess() }
}
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<P0>(punk: P0, lpvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : * mut core::ffi::c_void, lpvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoAllowSetForegroundWindow(punk.param().abi(), lpvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoAllowUnmarshalerCLSID(clsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { CoAllowUnmarshalerCLSID(clsid).ok() }
}
#[inline]
pub unsafe fn CoBuildVersion() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoBuildVersion() -> u32);
    unsafe { CoBuildVersion() }
}
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoCancelCall(dwthreadid : u32, ultimeout : u32) -> windows_core::HRESULT);
    unsafe { CoCancelCall(dwthreadid, ultimeout).ok() }
}
#[inline]
pub unsafe fn CoCopyProxy<P0>(pproxy: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoCopyProxy(pproxy : * mut core::ffi::c_void, ppcopy : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoCopyProxy(pproxy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<P0>(punkouter: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("combase.dll" "system" fn CoCreateFreeThreadedMarshaler(punkouter : * mut core::ffi::c_void, ppunkmarshal : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoCreateFreeThreadedMarshaler(punkouter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoCreateGuid() -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoCreateGuid(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoCreateInstance<P1, T>(rclsid: *const windows_core::GUID, punkouter: P1, dwclscontext: CLSCTX) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclscontext : CLSCTX, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoCreateInstance(rclsid, punkouter.param().abi(), dwclscontext, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoCreateInstanceEx<P1>(clsid: *const windows_core::GUID, punkouter: P1, dwclsctx: CLSCTX, pserverinfo: Option<*const COSERVERINFO>, presults: &mut [MULTI_QI]) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstanceEx(clsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclsctx : CLSCTX, pserverinfo : *const COSERVERINFO, dwcount : u32, presults : *mut MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoCreateInstanceEx(clsid, punkouter.param().abi(), dwclsctx, pserverinfo.unwrap_or(core::mem::zeroed()) as _, presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CoCreateInstanceFromApp<P1>(clsid: *const windows_core::GUID, punkouter: P1, dwclsctx: CLSCTX, reserved: Option<*const core::ffi::c_void>, presults: &mut [MULTI_QI]) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstanceFromApp(clsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclsctx : CLSCTX, reserved : *const core::ffi::c_void, dwcount : u32, presults : *mut MULTI_QI) -> windows_core::HRESULT);
    unsafe { CoCreateInstanceFromApp(clsid, punkouter.param().abi(), dwclsctx, reserved.unwrap_or(core::mem::zeroed()) as _, presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())).ok() }
}
#[inline]
pub unsafe fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoDecrementMTAUsage(cookie : CO_MTA_USAGE_COOKIE) -> windows_core::HRESULT);
    unsafe { CoDecrementMTAUsage(cookie).ok() }
}
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoDisableCallCancellation(preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoDisableCallCancellation(preserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoDisconnectContext(dwtimeout : u32) -> windows_core::HRESULT);
    unsafe { CoDisconnectContext(dwtimeout).ok() }
}
#[inline]
pub unsafe fn CoDisconnectObject<P0>(punk: P0, dwreserved: Option<u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoDisconnectObject(punk : * mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    unsafe { CoDisconnectObject(punk.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : u16, ndostime : u16, lpfiletime : *mut super::super::Foundation:: FILETIME) -> windows_core::BOOL);
    unsafe { CoDosDateTimeToFileTime(ndosdate, ndostime, lpfiletime as _) }
}
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoEnableCallCancellation(preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoEnableCallCancellation(preserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CoFileTimeNow() -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_core::link!("ole32.dll" "system" fn CoFileTimeNow(lpfiletime : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoFileTimeNow(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::super::Foundation:: FILETIME, lpdosdate : *mut u16, lpdostime : *mut u16) -> windows_core::BOOL);
    unsafe { CoFileTimeToDosDateTime(lpfiletime, lpdosdate as _, lpdostime as _) }
}
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    windows_core::link!("ole32.dll" "system" fn CoFreeAllLibraries());
    unsafe { CoFreeAllLibraries() }
}
#[inline]
pub unsafe fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE) {
    windows_core::link!("ole32.dll" "system" fn CoFreeLibrary(hinst : super::super::Foundation:: HINSTANCE));
    unsafe { CoFreeLibrary(hinst) }
}
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    windows_core::link!("ole32.dll" "system" fn CoFreeUnusedLibraries());
    unsafe { CoFreeUnusedLibraries() }
}
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: Option<u32>) {
    windows_core::link!("ole32.dll" "system" fn CoFreeUnusedLibrariesEx(dwunloaddelay : u32, dwreserved : u32));
    unsafe { CoFreeUnusedLibrariesEx(dwunloaddelay, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoGetApartmentType(papttype : *mut APTTYPE, paptqualifier : *mut APTTYPEQUALIFIER) -> windows_core::HRESULT);
    unsafe { CoGetApartmentType(papttype as _, paptqualifier as _).ok() }
}
#[inline]
pub unsafe fn CoGetCallContext<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetCallContext(riid : *const windows_core::GUID, ppinterface : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetCallContext(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoGetCallerTID() -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetCallerTID(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoGetCancelObject<T>(dwthreadid: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetCancelObject(dwthreadid : u32, iid : *const windows_core::GUID, ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetCancelObject(dwthreadid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoGetClassObject<T>(rclsid: *const windows_core::GUID, dwclscontext: CLSCTX, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetClassObject(rclsid : *const windows_core::GUID, dwclscontext : u32, pvreserved : *const core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetClassObject(rclsid, dwclscontext.0 as _, pvreserved.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoGetContextToken() -> windows_core::Result<usize> {
    windows_core::link!("ole32.dll" "system" fn CoGetContextToken(ptoken : *mut usize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetContextToken(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CoGetCurrentLogicalThreadId(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetCurrentLogicalThreadId(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoGetCurrentProcess() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoGetCurrentProcess() -> u32);
    unsafe { CoGetCurrentProcess() }
}
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> windows_core::Result<IMalloc> {
    windows_core::link!("ole32.dll" "system" fn CoGetMalloc(dwmemcontext : u32, ppmalloc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetMalloc(dwmemcontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoGetObject<P0, T>(pszname: P0, pbindoptions: Option<*const BIND_OPTS>) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetObject(pszname : windows_core::PCWSTR, pbindoptions : *const BIND_OPTS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetObject(pszname.param().abi(), pbindoptions.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoGetObjectContext<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoGetObjectContext(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoGetObjectContext(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CoGetPSClsid(riid : *const windows_core::GUID, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetPSClsid(riid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut super::super::Security::PSECURITY_DESCRIPTOR) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoGetSystemSecurityPermissions(comsdtype : COMSD, ppsd : *mut super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_core::HRESULT);
    unsafe { CoGetSystemSecurityPermissions(comsdtype, ppsd as _).ok() }
}
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const windows_core::GUID, pclsidnew: *mut windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoGetTreatAsClass(clsidold : *const windows_core::GUID, pclsidnew : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe { CoGetTreatAsClass(clsidold, pclsidnew as _) }
}
#[inline]
pub unsafe fn CoImpersonateClient() -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoImpersonateClient() -> windows_core::HRESULT);
    unsafe { CoImpersonateClient().ok() }
}
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> windows_core::Result<CO_MTA_USAGE_COOKIE> {
    windows_core::link!("combase.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoIncrementMTAUsage(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoInitialize(pvreserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitialize(pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoInitialize(pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: Option<*const core::ffi::c_void>, dwcoinit: COINIT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit.0 as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoInitializeSecurity(psecdesc: Option<super::super::Security::PSECURITY_DESCRIPTOR>, cauthsvc: i32, asauthsvc: Option<*const SOLE_AUTHENTICATION_SERVICE>, preserved1: Option<*const core::ffi::c_void>, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: Option<*const core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoInitializeSecurity(psecdesc : super::super::Security:: PSECURITY_DESCRIPTOR, cauthsvc : i32, asauthsvc : *const SOLE_AUTHENTICATION_SERVICE, preserved1 : *const core::ffi::c_void, dwauthnlevel : RPC_C_AUTHN_LEVEL, dwimplevel : RPC_C_IMP_LEVEL, pauthlist : *const core::ffi::c_void, dwcapabilities : u32, preserved3 : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoInitializeSecurity(psecdesc.unwrap_or(core::mem::zeroed()) as _, cauthsvc, asauthsvc.unwrap_or(core::mem::zeroed()) as _, preserved1.unwrap_or(core::mem::zeroed()) as _, dwauthnlevel, dwimplevel, pauthlist.unwrap_or(core::mem::zeroed()) as _, dwcapabilities.0 as _, preserved3.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CoInstall<P0, P4>(pbc: P0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: P4) -> windows_core::Result<()>
where
    P0: windows_core::Param<IBindCtx>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoInstall(pbc : * mut core::ffi::c_void, dwflags : u32, pclassspec : *const uCLSSPEC, pquery : *const QUERYCONTEXT, pszcodebase : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { CoInstall(pbc.param().abi(), dwflags, pclassspec, pquery, pszcodebase.param().abi()).ok() }
}
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<P0>(pszmachinename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoInvalidateRemoteMachineBindings(pszmachinename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { CoInvalidateRemoteMachineBindings(pszmachinename.param().abi()).ok() }
}
#[inline]
pub unsafe fn CoIsHandlerConnected<P0>(punk: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : * mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CoIsHandlerConnected(punk.param().abi()) }
}
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const windows_core::GUID) -> windows_core::BOOL);
    unsafe { CoIsOle1Class(rclsid) }
}
#[inline]
pub unsafe fn CoLoadLibrary<P0>(lpszlibname: P0, bautofree: bool) -> super::super::Foundation::HINSTANCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : windows_core::PCWSTR, bautofree : windows_core::BOOL) -> super::super::Foundation:: HINSTANCE);
    unsafe { CoLoadLibrary(lpszlibname.param().abi(), bautofree.into()) }
}
#[inline]
pub unsafe fn CoLockObjectExternal<P0>(punk: P0, flock: bool, flastunlockreleases: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoLockObjectExternal(punk : * mut core::ffi::c_void, flock : windows_core::BOOL, flastunlockreleases : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CoLockObjectExternal(punk.param().abi(), flock.into(), flastunlockreleases.into()).ok() }
}
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoQueryAuthenticationServices(pcauthsvc : *mut u32, asauthsvc : *mut *mut SOLE_AUTHENTICATION_SERVICE) -> windows_core::HRESULT);
    unsafe { CoQueryAuthenticationServices(pcauthsvc as _, asauthsvc as _).ok() }
}
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: Option<*mut windows_core::PWSTR>, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pprivs: Option<*mut *mut core::ffi::c_void>, pcapabilities: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoQueryClientBlanket(pauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pprivs : *mut *mut core::ffi::c_void, pcapabilities : *mut u32) -> windows_core::HRESULT);
    unsafe { CoQueryClientBlanket(pauthnsvc.unwrap_or(core::mem::zeroed()) as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname.unwrap_or(core::mem::zeroed()) as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pprivs.unwrap_or(core::mem::zeroed()) as _, pcapabilities.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CoQueryProxyBlanket<P0>(pproxy: P0, pwauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: Option<*mut windows_core::PWSTR>, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pauthinfo: Option<*mut *mut core::ffi::c_void>, pcapabilites: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoQueryProxyBlanket(pproxy : * mut core::ffi::c_void, pwauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pauthinfo : *mut *mut core::ffi::c_void, pcapabilites : *mut u32) -> windows_core::HRESULT);
    unsafe { CoQueryProxyBlanket(pproxy.param().abi(), pwauthnsvc.unwrap_or(core::mem::zeroed()) as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname.unwrap_or(core::mem::zeroed()) as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pauthinfo.unwrap_or(core::mem::zeroed()) as _, pcapabilites.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CoRegisterActivationFilter<P0>(pactivationfilter: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IActivationFilter>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterActivationFilter(pactivationfilter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterActivationFilter(pactivationfilter.param().abi()).ok() }
}
#[inline]
pub unsafe fn CoRegisterChannelHook<P1>(extensionuuid: *const windows_core::GUID, pchannelhook: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<IChannelHook>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterChannelHook(extensionuuid : *const windows_core::GUID, pchannelhook : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterChannelHook(extensionuuid, pchannelhook.param().abi()).ok() }
}
#[inline]
pub unsafe fn CoRegisterClassObject<P1>(rclsid: *const windows_core::GUID, punk: P1, dwclscontext: CLSCTX, flags: REGCLS) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterClassObject(rclsid : *const windows_core::GUID, punk : * mut core::ffi::c_void, dwclscontext : CLSCTX, flags : u32, lpdwregister : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoRegisterClassObject(rclsid, punk.param().abi(), dwclscontext, flags.0 as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<P0>(deviceinstanceid: P0) -> windows_core::Result<CO_DEVICE_CATALOG_COOKIE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterDeviceCatalog(deviceinstanceid : windows_core::PCWSTR, cookie : *mut CO_DEVICE_CATALOG_COOKIE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoRegisterDeviceCatalog(deviceinstanceid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoRegisterInitializeSpy<P0>(pspy: P0) -> windows_core::Result<u64>
where
    P0: windows_core::Param<IInitializeSpy>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : * mut core::ffi::c_void, pulicookie : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoRegisterInitializeSpy(pspy.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoRegisterMallocSpy<P0>(pmallocspy: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMallocSpy>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterMallocSpy(pmallocspy.param().abi()).ok() }
}
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const windows_core::GUID, rclsid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoRegisterPSClsid(riid : *const windows_core::GUID, rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { CoRegisterPSClsid(riid, rclsid).ok() }
}
#[inline]
pub unsafe fn CoRegisterSurrogate<P0>(psurrogate: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<ISurrogate>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterSurrogate(psurrogate : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterSurrogate(psurrogate.param().abi()).ok() }
}
#[inline]
pub unsafe fn CoReleaseServerProcess() -> u32 {
    windows_core::link!("ole32.dll" "system" fn CoReleaseServerProcess() -> u32);
    unsafe { CoReleaseServerProcess() }
}
#[inline]
pub unsafe fn CoResumeClassObjects() -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoResumeClassObjects() -> windows_core::HRESULT);
    unsafe { CoResumeClassObjects().ok() }
}
#[inline]
pub unsafe fn CoRevertToSelf() -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoRevertToSelf() -> windows_core::HRESULT);
    unsafe { CoRevertToSelf().ok() }
}
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoRevokeClassObject(dwregister : u32) -> windows_core::HRESULT);
    unsafe { CoRevokeClassObject(dwregister).ok() }
}
#[inline]
pub unsafe fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoRevokeDeviceCatalog(cookie : CO_DEVICE_CATALOG_COOKIE) -> windows_core::HRESULT);
    unsafe { CoRevokeDeviceCatalog(cookie).ok() }
}
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : u64) -> windows_core::HRESULT);
    unsafe { CoRevokeInitializeSpy(ulicookie).ok() }
}
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_core::HRESULT);
    unsafe { CoRevokeMallocSpy().ok() }
}
#[inline]
pub unsafe fn CoSetCancelObject<P0>(punk: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoSetCancelObject(punk : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoSetCancelObject(punk.param().abi()).ok() }
}
#[inline]
pub unsafe fn CoSetProxyBlanket<P0, P3>(pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: P3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: Option<*const core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CoSetProxyBlanket(pproxy : * mut core::ffi::c_void, dwauthnsvc : u32, dwauthzsvc : u32, pserverprincname : windows_core::PCWSTR, dwauthnlevel : RPC_C_AUTHN_LEVEL, dwimplevel : RPC_C_IMP_LEVEL, pauthinfo : *const core::ffi::c_void, dwcapabilities : u32) -> windows_core::HRESULT);
    unsafe { CoSetProxyBlanket(pproxy.param().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.param().abi(), dwauthnlevel, dwimplevel, pauthinfo.unwrap_or(core::mem::zeroed()) as _, dwcapabilities.0 as _).ok() }
}
#[inline]
pub unsafe fn CoSuspendClassObjects() -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoSuspendClassObjects() -> windows_core::HRESULT);
    unsafe { CoSuspendClassObjects().ok() }
}
#[inline]
pub unsafe fn CoSwitchCallContext<P0>(pnewobject: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoSwitchCallContext(pnewobject : * mut core::ffi::c_void, ppoldobject : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoSwitchCallContext(pnewobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("combase.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
    unsafe { CoTaskMemAlloc(cb) }
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: Option<*const core::ffi::c_void>) {
    windows_core::link!("combase.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
    unsafe { CoTaskMemFree(pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: Option<*const core::ffi::c_void>, cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemRealloc(pv : *const core::ffi::c_void, cb : usize) -> *mut core::ffi::c_void);
    unsafe { CoTaskMemRealloc(pv.unwrap_or(core::mem::zeroed()) as _, cb) }
}
#[inline]
pub unsafe fn CoTestCancel() -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoTestCancel() -> windows_core::HRESULT);
    unsafe { CoTestCancel().ok() }
}
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const windows_core::GUID, clsidnew: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const windows_core::GUID, clsidnew : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { CoTreatAsClass(clsidold, clsidnew).ok() }
}
#[inline]
pub unsafe fn CoUninitialize() {
    windows_core::link!("ole32.dll" "system" fn CoUninitialize());
    unsafe { CoUninitialize() }
}
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoWaitForMultipleHandles(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::super::Foundation:: HANDLE, lpdwindex : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoWaitForMultipleHandles(dwflags, dwtimeout, phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoWaitForMultipleObjects(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::super::Foundation:: HANDLE, lpdwindex : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoWaitForMultipleObjects(dwflags, dwtimeout, phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CreateAntiMoniker() -> windows_core::Result<IMoniker> {
    windows_core::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateAntiMoniker(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> windows_core::Result<IBindCtx> {
    windows_core::link!("ole32.dll" "system" fn CreateBindCtx(reserved : u32, ppbc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateBindCtx(reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const windows_core::GUID) -> windows_core::Result<IMoniker> {
    windows_core::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const windows_core::GUID, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateClassMoniker(rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> windows_core::Result<IDataAdviseHolder> {
    windows_core::link!("ole32.dll" "system" fn CreateDataAdviseHolder(ppdaholder : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDataAdviseHolder(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateDataCache<P0, T>(punkouter: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CreateDataCache(punkouter : * mut core::ffi::c_void, rclsid : *const windows_core::GUID, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDataCache(punkouter.param().abi(), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateFileMoniker<P0>(lpszpathname: P0) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : windows_core::PCWSTR, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateFileMoniker(lpszpathname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateGenericComposite<P0, P1>(pmkfirst: P0, pmkrest: P1) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<IMoniker>,
    P1: windows_core::Param<IMoniker>,
{
    windows_core::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : * mut core::ffi::c_void, pmkrest : * mut core::ffi::c_void, ppmkcomposite : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateGenericComposite(pmkfirst.param().abi(), pmkrest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateIUriBuilder<P0>(piuri: P0, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUriBuilder>
where
    P0: windows_core::Param<IUri>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateIUriBuilder(piuri : * mut core::ffi::c_void, dwflags : u32, dwreserved : usize, ppiuribuilder : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateIUriBuilder(piuri.param().abi(), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateItemMoniker<P0, P1>(lpszdelim: P0, lpszitem: P1) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : windows_core::PCWSTR, lpszitem : windows_core::PCWSTR, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateItemMoniker(lpszdelim.param().abi(), lpszitem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateObjrefMoniker<P0>(punk: P0) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : * mut core::ffi::c_void, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateObjrefMoniker(punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreatePointerMoniker<P0>(punk: P0) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : * mut core::ffi::c_void, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreatePointerMoniker(punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateStdProgressIndicator<P1, P2>(hwndparent: super::super::Foundation::HWND, psztitle: P1, pibsccaller: P2) -> windows_core::Result<IBindStatusCallback>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IBindStatusCallback>,
{
    windows_core::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::super::Foundation:: HWND, psztitle : windows_core::PCWSTR, pibsccaller : * mut core::ffi::c_void, ppibsc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateStdProgressIndicator(hwndparent, psztitle.param().abi(), pibsccaller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateUri<P0>(pwzuri: P0, dwflags: URI_CREATE_FLAGS, dwreserved: Option<usize>) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateUri(pwzuri : windows_core::PCWSTR, dwflags : URI_CREATE_FLAGS, dwreserved : usize, ppuri : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateUri(pwzuri.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateUriFromMultiByteString<P0>(pszansiinputuri: P0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: Option<usize>) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateUriFromMultiByteString(pszansiinputuri : windows_core::PCSTR, dwencodingflags : u32, dwcodepage : u32, dwcreateflags : u32, dwreserved : usize, ppuri : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.param().abi(), dwencodingflags, dwcodepage, dwcreateflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateUriWithFragment<P0, P1>(pwzuri: P0, pwzfragment: P1, dwflags: u32, dwreserved: Option<usize>) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("urlmon.dll" "system" fn CreateUriWithFragment(pwzuri : windows_core::PCWSTR, pwzfragment : windows_core::PCWSTR, dwflags : u32, dwreserved : usize, ppuri : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateUriWithFragment(pwzuri.param().abi(), pwzfragment.param().abi(), dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: Option<*const core::ffi::c_void>, pulreserved: Option<*const u32>, appshr: windows_core::HRESULT) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : *const core::ffi::c_void, pulreserved : *const u32, appshr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { DcomChannelSetHResult(pvreserved.unwrap_or(core::mem::zeroed()) as _, pulreserved.unwrap_or(core::mem::zeroed()) as _, appshr).ok() }
}
#[inline]
pub unsafe fn GetClassFile<P0>(szfilename: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn GetClassFile(szfilename : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetClassFile(szfilename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> windows_core::Result<IErrorInfo> {
    windows_core::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : u32, pperrinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetErrorInfo(dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> windows_core::Result<IRunningObjectTable> {
    windows_core::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : u32, pprot : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetRunningObjectTable(reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn IIDFromString<P0>(lpsz: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn IIDFromString(lpsz : windows_core::PCWSTR, lpiid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IIDFromString(lpsz.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MkParseDisplayName<P0, P1>(pbc: P0, szusername: P1, pcheaten: *mut u32, ppmk: *mut Option<IMoniker>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : * mut core::ffi::c_void, szusername : windows_core::PCWSTR, pcheaten : *mut u32, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MkParseDisplayName(pbc.param().abi(), szusername.param().abi(), pcheaten as _, core::mem::transmute(ppmk)).ok() }
}
#[inline]
pub unsafe fn MonikerCommonPrefixWith<P0, P1>(pmkthis: P0, pmkother: P1) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<IMoniker>,
    P1: windows_core::Param<IMoniker>,
{
    windows_core::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : * mut core::ffi::c_void, pmkother : * mut core::ffi::c_void, ppmkcommon : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MonikerCommonPrefixWith(pmkthis.param().abi(), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MonikerRelativePathTo<P0, P1>(pmksrc: P0, pmkdest: P1, ppmkrelpath: *mut Option<IMoniker>, dwreserved: bool) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMoniker>,
    P1: windows_core::Param<IMoniker>,
{
    windows_core::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : * mut core::ffi::c_void, pmkdest : * mut core::ffi::c_void, ppmkrelpath : *mut * mut core::ffi::c_void, dwreserved : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { MonikerRelativePathTo(pmksrc.param().abi(), pmkdest.param().abi(), core::mem::transmute(ppmkrelpath), dwreserved.into()).ok() }
}
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("ole32.dll" "system" fn ProgIDFromCLSID(clsid : *const windows_core::GUID, lplpszprogid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ProgIDFromCLSID(clsid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SetErrorInfo<P1>(dwreserved: u32, perrinfo: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<IErrorInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : u32, perrinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SetErrorInfo(dwreserved, perrinfo.param().abi()).ok() }
}
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("ole32.dll" "system" fn StringFromCLSID(rclsid : *const windows_core::GUID, lplpsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StringFromCLSID(rclsid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const windows_core::GUID, lpsz: &mut [u16]) -> i32 {
    windows_core::link!("ole32.dll" "system" fn StringFromGUID2(rguid : *const windows_core::GUID, lpsz : windows_core::PWSTR, cchmax : i32) -> i32);
    unsafe { StringFromGUID2(rguid, core::mem::transmute(lpsz.as_ptr()), lpsz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StringFromIID(rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("ole32.dll" "system" fn StringFromIID(rclsid : *const windows_core::GUID, lplpsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StringFromIID(rclsid, &mut result__).map(|| result__)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADVANCED_FEATURE_FLAGS(pub u16);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADVF(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APTTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APTTYPEQUALIFIER(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ApplicationType(pub i32);
windows_core::imp::define_interface!(AsyncIAdviseSink, AsyncIAdviseSink_Vtbl, 0x00000150_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(AsyncIAdviseSink, windows_core::IUnknown);
impl AsyncIAdviseSink {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        unsafe { (windows_core::Interface::vtable(self).Begin_OnDataChange)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pstgmed)) }
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        unsafe { (windows_core::Interface::vtable(self).Finish_OnDataChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        unsafe { (windows_core::Interface::vtable(self).Begin_OnViewChange)(windows_core::Interface::as_raw(self), dwaspect, lindex) }
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        unsafe { (windows_core::Interface::vtable(self).Finish_OnViewChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_OnRename<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_OnRename)(windows_core::Interface::as_raw(self), pmk.param().abi()) }
    }
    pub unsafe fn Finish_OnRename(&self) {
        unsafe { (windows_core::Interface::vtable(self).Finish_OnRename)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_OnSave(&self) {
        unsafe { (windows_core::Interface::vtable(self).Begin_OnSave)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Finish_OnSave(&self) {
        unsafe { (windows_core::Interface::vtable(self).Finish_OnSave)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_OnClose(&self) {
        unsafe { (windows_core::Interface::vtable(self).Begin_OnClose)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Finish_OnClose(&self) {
        unsafe { (windows_core::Interface::vtable(self).Finish_OnClose)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink_Impl: windows_core::IUnknownImpl {
    fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn Finish_OnDataChange(&self);
    fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn Finish_OnViewChange(&self);
    fn Begin_OnRename(&self, pmk: windows_core::Ref<IMoniker>);
    fn Finish_OnRename(&self);
    fn Begin_OnSave(&self);
    fn Finish_OnSave(&self);
    fn Begin_OnClose(&self);
    fn Finish_OnClose(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSink_Vtbl {
    pub const fn new<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_OnDataChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnDataChange(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed))
            }
        }
        unsafe extern "system" fn Finish_OnDataChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnDataChange(this)
            }
        }
        unsafe extern "system" fn Begin_OnViewChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, lindex: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnViewChange(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex))
            }
        }
        unsafe extern "system" fn Finish_OnViewChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnViewChange(this)
            }
        }
        unsafe extern "system" fn Begin_OnRename<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnRename(this, core::mem::transmute_copy(&pmk))
            }
        }
        unsafe extern "system" fn Finish_OnRename<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnRename(this)
            }
        }
        unsafe extern "system" fn Begin_OnSave<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnSave(this)
            }
        }
        unsafe extern "system" fn Finish_OnSave<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnSave(this)
            }
        }
        unsafe extern "system" fn Begin_OnClose<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnClose(this)
            }
        }
        unsafe extern "system" fn Finish_OnClose<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnClose(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_OnDataChange: Begin_OnDataChange::<Identity, OFFSET>,
            Finish_OnDataChange: Finish_OnDataChange::<Identity, OFFSET>,
            Begin_OnViewChange: Begin_OnViewChange::<Identity, OFFSET>,
            Finish_OnViewChange: Finish_OnViewChange::<Identity, OFFSET>,
            Begin_OnRename: Begin_OnRename::<Identity, OFFSET>,
            Finish_OnRename: Finish_OnRename::<Identity, OFFSET>,
            Begin_OnSave: Begin_OnSave::<Identity, OFFSET>,
            Finish_OnSave: Finish_OnSave::<Identity, OFFSET>,
            Begin_OnClose: Begin_OnClose::<Identity, OFFSET>,
            Finish_OnClose: Finish_OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for AsyncIAdviseSink {}
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
        unsafe { (windows_core::Interface::vtable(self).Begin_OnLinkSrcChange)(windows_core::Interface::as_raw(self), pmk.param().abi()) }
    }
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        unsafe { (windows_core::Interface::vtable(self).Finish_OnLinkSrcChange)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_Vtbl {
    pub base__: AsyncIAdviseSink_Vtbl,
    pub Begin_OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Finish_OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink2_Impl: AsyncIAdviseSink_Impl {
    fn Begin_OnLinkSrcChange(&self, pmk: windows_core::Ref<IMoniker>);
    fn Finish_OnLinkSrcChange(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSink2_Vtbl {
    pub const fn new<Identity: AsyncIAdviseSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_OnLinkSrcChange<Identity: AsyncIAdviseSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink2_Impl::Begin_OnLinkSrcChange(this, core::mem::transmute_copy(&pmk))
            }
        }
        unsafe extern "system" fn Finish_OnLinkSrcChange<Identity: AsyncIAdviseSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink2_Impl::Finish_OnLinkSrcChange(this)
            }
        }
        Self {
            base__: AsyncIAdviseSink_Vtbl::new::<Identity, OFFSET>(),
            Begin_OnLinkSrcChange: Begin_OnLinkSrcChange::<Identity, OFFSET>,
            Finish_OnLinkSrcChange: Finish_OnLinkSrcChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIAdviseSink2 as windows_core::Interface>::IID || iid == &<AsyncIAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for AsyncIAdviseSink2 {}
windows_core::imp::define_interface!(AsyncIMultiQI, AsyncIMultiQI_Vtbl, 0x000e0020_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(AsyncIMultiQI, windows_core::IUnknown);
impl AsyncIMultiQI {
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), core::mem::transmute(pmqis.as_ptr())).ok() }
    }
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), core::mem::transmute(pmqis)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MULTI_QI) -> windows_core::HRESULT,
    pub Finish_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MULTI_QI) -> windows_core::HRESULT,
}
pub trait AsyncIMultiQI_Impl: windows_core::IUnknownImpl {
    fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
    fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
impl AsyncIMultiQI_Vtbl {
    pub const fn new<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIMultiQI_Impl::Begin_QueryMultipleInterfaces(this, core::mem::transmute_copy(&cmqis), core::mem::transmute_copy(&pmqis)).into()
            }
        }
        unsafe extern "system" fn Finish_QueryMultipleInterfaces<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIMultiQI_Impl::Finish_QueryMultipleInterfaces(this, core::mem::transmute_copy(&pmqis)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_QueryMultipleInterfaces: Begin_QueryMultipleInterfaces::<Identity, OFFSET>,
            Finish_QueryMultipleInterfaces: Finish_QueryMultipleInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIMultiQI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIMultiQI {}
windows_core::imp::define_interface!(AsyncIPipeByte, AsyncIPipeByte_Vtbl, 0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeByte, windows_core::IUnknown);
impl AsyncIPipeByte {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest).ok() }
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _).ok() }
    }
    pub unsafe fn Begin_Push(&self, buf: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIPipeByte_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const u8, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl AsyncIPipeByte_Vtbl {
    pub const fn new<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const u8, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        unsafe extern "system" fn Finish_Push<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Finish_Push(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeByte as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIPipeByte {}
windows_core::imp::define_interface!(AsyncIPipeDouble, AsyncIPipeDouble_Vtbl, 0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeDouble, windows_core::IUnknown);
impl AsyncIPipeDouble {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest).ok() }
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _).ok() }
    }
    pub unsafe fn Begin_Push(&self, buf: &[f64]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIPipeDouble_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const f64, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl AsyncIPipeDouble_Vtbl {
    pub const fn new<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const f64, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        unsafe extern "system" fn Finish_Push<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Finish_Push(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeDouble as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIPipeDouble {}
windows_core::imp::define_interface!(AsyncIPipeLong, AsyncIPipeLong_Vtbl, 0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeLong, windows_core::IUnknown);
impl AsyncIPipeLong {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest).ok() }
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _).ok() }
    }
    pub unsafe fn Begin_Push(&self, buf: &[i32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIPipeLong_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const i32, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl AsyncIPipeLong_Vtbl {
    pub const fn new<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const i32, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        unsafe extern "system" fn Finish_Push<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Finish_Push(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeLong as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIPipeLong {}
windows_core::imp::define_interface!(AsyncIUnknown, AsyncIUnknown_Vtbl, 0x000e0000_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(AsyncIUnknown, windows_core::IUnknown);
impl AsyncIUnknown {
    pub unsafe fn Begin_QueryInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_QueryInterface)(windows_core::Interface::as_raw(self), riid).ok() }
    }
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_QueryInterface)(windows_core::Interface::as_raw(self), ppvobject as _).ok() }
    }
    pub unsafe fn Begin_AddRef(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_AddRef)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Finish_AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_Release(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_Release)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Finish_Release)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Finish_QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Begin_Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait AsyncIUnknown_Impl: windows_core::IUnknownImpl {
    fn Begin_QueryInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Finish_QueryInterface(&self, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Begin_AddRef(&self) -> windows_core::Result<()>;
    fn Finish_AddRef(&self) -> u32;
    fn Begin_Release(&self) -> windows_core::Result<()>;
    fn Finish_Release(&self) -> u32;
}
impl AsyncIUnknown_Vtbl {
    pub const fn new<Identity: AsyncIUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_QueryInterface<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Begin_QueryInterface(this, core::mem::transmute_copy(&riid)).into()
            }
        }
        unsafe extern "system" fn Finish_QueryInterface<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Finish_QueryInterface(this, core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn Begin_AddRef<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Begin_AddRef(this).into()
            }
        }
        unsafe extern "system" fn Finish_AddRef<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Finish_AddRef(this)
            }
        }
        unsafe extern "system" fn Begin_Release<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Begin_Release(this).into()
            }
        }
        unsafe extern "system" fn Finish_Release<Identity: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIUnknown_Impl::Finish_Release(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_QueryInterface: Begin_QueryInterface::<Identity, OFFSET>,
            Finish_QueryInterface: Finish_QueryInterface::<Identity, OFFSET>,
            Begin_AddRef: Begin_AddRef::<Identity, OFFSET>,
            Finish_AddRef: Finish_AddRef::<Identity, OFFSET>,
            Begin_Release: Begin_Release::<Identity, OFFSET>,
            Finish_Release: Finish_Release::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIUnknown {}
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
impl Default for BINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BINDINFOF(pub i32);
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
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
impl Default for BINDPTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BIND_FLAGS(pub i32);
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl Default for BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALLCONV(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALLTYPE(pub i32);
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CATEGORYINFO {
    pub catid: windows_core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLSCTX(pub u32);
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
pub const CLSID_GlobalOptions: windows_core::GUID = windows_core::GUID::from_u128(0x0000034b_0000_0000_c000_000000000046);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: windows_core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COINIT(pub i32);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COINITBASE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMSD(pub i32);
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CONNECTDATA {
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwCookie: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: windows_core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COWAIT_FLAGS(pub i32);
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
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub *mut core::ffi::c_void);
impl CO_DEVICE_CATALOG_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for CO_DEVICE_CATALOG_COOKIE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("ole32.dll" "system" fn CoRevokeDeviceCatalog(cookie : *mut core::ffi::c_void) -> i32);
            unsafe {
                CoRevokeDeviceCatalog(self.0);
            }
        }
    }
}
impl Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CO_MTA_USAGE_COOKIE(pub *mut core::ffi::c_void);
impl CO_MTA_USAGE_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for CO_MTA_USAGE_COOKIE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("ole32.dll" "system" fn CoDecrementMTAUsage(cookie : *mut core::ffi::c_void) -> i32);
            unsafe {
                CoDecrementMTAUsage(self.0);
            }
        }
    }
}
impl Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for CUSTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub struct CUSTDATAITEM {
    pub guid: windows_core::GUID,
    pub varValue: super::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CWMO_FLAGS(pub i32);
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
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut core::ffi::c_void,
}
impl Default for ComCallData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContextProperty {
    pub policyId: windows_core::GUID,
    pub flags: u32,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DATADIR(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCOM_CALL_STATE(pub i32);
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DESCKIND(pub i32);
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPATCH_FLAGS(pub u16);
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
pub const DISPATCH_METHOD: DISPATCH_FLAGS = DISPATCH_FLAGS(1u16);
pub const DISPATCH_PROPERTYGET: DISPATCH_FLAGS = DISPATCH_FLAGS(2u16);
pub const DISPATCH_PROPERTYPUT: DISPATCH_FLAGS = DISPATCH_FLAGS(4u16);
pub const DISPATCH_PROPERTYPUTREF: DISPATCH_FLAGS = DISPATCH_FLAGS(8u16);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPPARAMS {
    pub rgvarg: *mut super::Variant::VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMUS_ERRBASE: u32 = 4096u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DVASPECT(pub u32);
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1u32);
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8u32);
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4u32);
pub const DVASPECT_OPAQUE: DVASPECT = DVASPECT(16u32);
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2u32);
pub const DVASPECT_TRANSPARENT: DVASPECT = DVASPECT(32u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
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
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
#[repr(C)]
#[derive(Clone, Debug)]
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
impl Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXTCONN(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
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
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
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
impl Default for FUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FUNCFLAGS(pub u16);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FUNCKIND(pub i32);
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
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
impl Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBALOPT_PROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBALOPT_RO_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IActivationFilter, IActivationFilter_Vtbl, 0x00000017_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IActivationFilter, windows_core::IUnknown);
impl IActivationFilter {
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HandleActivation)(windows_core::Interface::as_raw(self), dwactivationtype, rclsid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleActivation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IActivationFilter_Impl: windows_core::IUnknownImpl {
    fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID>;
}
impl IActivationFilter_Vtbl {
    pub const fn new<Identity: IActivationFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleActivation<Identity: IActivationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwactivationtype: u32, rclsid: *const windows_core::GUID, preplacementclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivationFilter_Impl::HandleActivation(this, core::mem::transmute_copy(&dwactivationtype), core::mem::transmute_copy(&rclsid)) {
                    Ok(ok__) => {
                        preplacementclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleActivation: HandleActivation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivationFilter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivationFilter {}
windows_core::imp::define_interface!(IAddrExclusionControl, IAddrExclusionControl_Vtbl, 0x00000148_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAddrExclusionControl, windows_core::IUnknown);
impl IAddrExclusionControl {
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentAddrExclusionList)(windows_core::Interface::as_raw(self), riid, ppenumerator as _).ok() }
    }
    pub unsafe fn UpdateAddrExclusionList<P0>(&self, penumerator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateAddrExclusionList)(windows_core::Interface::as_raw(self), penumerator.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAddrExclusionControl_Impl: windows_core::IUnknownImpl {
    fn GetCurrentAddrExclusionList(&self, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateAddrExclusionList(&self, penumerator: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IAddrExclusionControl_Vtbl {
    pub const fn new<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrExclusionControl_Impl::GetCurrentAddrExclusionList(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppenumerator)).into()
            }
        }
        unsafe extern "system" fn UpdateAddrExclusionList<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumerator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrExclusionControl_Impl::UpdateAddrExclusionList(this, core::mem::transmute_copy(&penumerator)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentAddrExclusionList: GetCurrentAddrExclusionList::<Identity, OFFSET>,
            UpdateAddrExclusionList: UpdateAddrExclusionList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrExclusionControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAddrExclusionControl {}
windows_core::imp::define_interface!(IAddrTrackingControl, IAddrTrackingControl_Vtbl, 0x00000147_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAddrTrackingControl, windows_core::IUnknown);
impl IAddrTrackingControl {
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableCOMDynamicAddrTracking)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableCOMDynamicAddrTracking)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableCOMDynamicAddrTracking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCOMDynamicAddrTracking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAddrTrackingControl_Impl: windows_core::IUnknownImpl {
    fn EnableCOMDynamicAddrTracking(&self) -> windows_core::Result<()>;
    fn DisableCOMDynamicAddrTracking(&self) -> windows_core::Result<()>;
}
impl IAddrTrackingControl_Vtbl {
    pub const fn new<Identity: IAddrTrackingControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableCOMDynamicAddrTracking<Identity: IAddrTrackingControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrTrackingControl_Impl::EnableCOMDynamicAddrTracking(this).into()
            }
        }
        unsafe extern "system" fn DisableCOMDynamicAddrTracking<Identity: IAddrTrackingControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrTrackingControl_Impl::DisableCOMDynamicAddrTracking(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableCOMDynamicAddrTracking: EnableCOMDynamicAddrTracking::<Identity, OFFSET>,
            DisableCOMDynamicAddrTracking: DisableCOMDynamicAddrTracking::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrTrackingControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAddrTrackingControl {}
windows_core::imp::define_interface!(IAdviseSink, IAdviseSink_Vtbl, 0x0000010f_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAdviseSink, windows_core::IUnknown);
impl IAdviseSink {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        unsafe { (windows_core::Interface::vtable(self).OnDataChange)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pstgmed)) }
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        unsafe { (windows_core::Interface::vtable(self).OnViewChange)(windows_core::Interface::as_raw(self), dwaspect, lindex) }
    }
    pub unsafe fn OnRename<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRename)(windows_core::Interface::as_raw(self), pmk.param().abi()) }
    }
    pub unsafe fn OnSave(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnSave)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnClose(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink_Impl: windows_core::IUnknownImpl {
    fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn OnRename(&self, pmk: windows_core::Ref<IMoniker>);
    fn OnSave(&self);
    fn OnClose(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSink_Vtbl {
    pub const fn new<Identity: IAdviseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDataChange<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnDataChange(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed))
            }
        }
        unsafe extern "system" fn OnViewChange<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, lindex: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnViewChange(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex))
            }
        }
        unsafe extern "system" fn OnRename<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnRename(this, core::mem::transmute_copy(&pmk))
            }
        }
        unsafe extern "system" fn OnSave<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnSave(this)
            }
        }
        unsafe extern "system" fn OnClose<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnClose(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataChange: OnDataChange::<Identity, OFFSET>,
            OnViewChange: OnViewChange::<Identity, OFFSET>,
            OnRename: OnRename::<Identity, OFFSET>,
            OnSave: OnSave::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IAdviseSink {}
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
        unsafe { (windows_core::Interface::vtable(self).OnLinkSrcChange)(windows_core::Interface::as_raw(self), pmk.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_Vtbl {
    pub base__: IAdviseSink_Vtbl,
    pub OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink2_Impl: IAdviseSink_Impl {
    fn OnLinkSrcChange(&self, pmk: windows_core::Ref<IMoniker>);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSink2_Vtbl {
    pub const fn new<Identity: IAdviseSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLinkSrcChange<Identity: IAdviseSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink2_Impl::OnLinkSrcChange(this, core::mem::transmute_copy(&pmk))
            }
        }
        Self { base__: IAdviseSink_Vtbl::new::<Identity, OFFSET>(), OnLinkSrcChange: OnLinkSrcChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSink2 as windows_core::Interface>::IID || iid == &<IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IAdviseSink2 {}
windows_core::imp::define_interface!(IAgileObject, IAgileObject_Vtbl, 0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
windows_core::imp::interface_hierarchy!(IAgileObject, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IAgileObject_Impl: windows_core::IUnknownImpl {}
impl IAgileObject_Vtbl {
    pub const fn new<Identity: IAgileObject_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAgileObject {}
windows_core::imp::define_interface!(IAsyncManager, IAsyncManager_Vtbl, 0x0000002a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAsyncManager, windows_core::IUnknown);
impl IAsyncManager {
    pub unsafe fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CompleteCall)(windows_core::Interface::as_raw(self), result).ok() }
    }
    pub unsafe fn GetCallContext(&self, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), riid, pinterface as _).ok() }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompleteCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAsyncManager_Impl: windows_core::IUnknownImpl {
    fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCallContext(&self, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<u32>;
}
impl IAsyncManager_Vtbl {
    pub const fn new<Identity: IAsyncManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompleteCall<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncManager_Impl::CompleteCall(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn GetCallContext<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncManager_Impl::GetCallContext(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstateflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncManager_Impl::GetState(this) {
                    Ok(ok__) => {
                        pulstateflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompleteCall: CompleteCall::<Identity, OFFSET>,
            GetCallContext: GetCallContext::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAsyncManager {}
windows_core::imp::define_interface!(IAsyncRpcChannelBuffer, IAsyncRpcChannelBuffer_Vtbl, 0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
impl core::ops::Deref for IAsyncRpcChannelBuffer {
    type Target = IRpcChannelBuffer2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAsyncRpcChannelBuffer, windows_core::IUnknown, IRpcChannelBuffer, IRpcChannelBuffer2);
impl IAsyncRpcChannelBuffer {
    pub unsafe fn Send<P1>(&self, pmsg: *mut RPCOLEMESSAGE, psync: P1, pulstatus: *mut u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ISynchronize>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg as _, psync.param().abi(), pulstatus as _).ok() }
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg as _, pulstatus as _).ok() }
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext as _, ppvdestcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAsyncRpcChannelBuffer_Impl: IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, psync: windows_core::Ref<ISynchronize>, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IAsyncRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Send<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: *mut core::ffi::c_void, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::Send(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&psync), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Receive<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::Receive(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::GetDestCtxEx(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
            }
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, OFFSET>(),
            Send: Send::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAsyncRpcChannelBuffer {}
windows_core::imp::define_interface!(IAuthenticate, IAuthenticate_Vtbl, 0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IAuthenticate, windows_core::IUnknown);
impl IAuthenticate {
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), phwnd as _, pszusername as _, pszpassword as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAuthenticate_Impl: windows_core::IUnknownImpl {
    fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl IAuthenticate_Vtbl {
    pub const fn new<Identity: IAuthenticate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Authenticate<Identity: IAuthenticate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAuthenticate_Impl::Authenticate(this, core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&pszusername), core::mem::transmute_copy(&pszpassword)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Authenticate: Authenticate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAuthenticate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAuthenticate {}
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
        unsafe { (windows_core::Interface::vtable(self).AuthenticateEx)(windows_core::Interface::as_raw(self), phwnd as _, pszusername as _, pszpassword as _, pauthinfo).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateEx_Vtbl {
    pub base__: IAuthenticate_Vtbl,
    pub AuthenticateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *const AUTHENTICATEINFO) -> windows_core::HRESULT,
}
pub trait IAuthenticateEx_Impl: IAuthenticate_Impl {
    fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::Result<()>;
}
impl IAuthenticateEx_Vtbl {
    pub const fn new<Identity: IAuthenticateEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AuthenticateEx<Identity: IAuthenticateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAuthenticateEx_Impl::AuthenticateEx(this, core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&pszusername), core::mem::transmute_copy(&pszpassword), core::mem::transmute_copy(&pauthinfo)).into()
            }
        }
        Self { base__: IAuthenticate_Vtbl::new::<Identity, OFFSET>(), AuthenticateEx: AuthenticateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAuthenticateEx as windows_core::Interface>::IID || iid == &<IAuthenticate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAuthenticateEx {}
windows_core::imp::define_interface!(IBindCtx, IBindCtx_Vtbl, 0x0000000e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IBindCtx, windows_core::IUnknown);
impl IBindCtx {
    pub unsafe fn RegisterObjectBound<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterObjectBound)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
    pub unsafe fn RevokeObjectBound<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RevokeObjectBound)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
    pub unsafe fn ReleaseBoundObjects(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseBoundObjects)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBindOptions)(windows_core::Interface::as_raw(self), pbindopts).ok() }
    }
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBindOptions)(windows_core::Interface::as_raw(self), pbindopts as _).ok() }
    }
    pub unsafe fn GetRunningObjectTable(&self) -> windows_core::Result<IRunningObjectTable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunningObjectTable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterObjectParam<P0, P1>(&self, pszkey: P0, punk: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi(), punk.param().abi()).ok() }
    }
    pub unsafe fn GetObjectParam<P0>(&self, pszkey: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumObjectParam(&self) -> windows_core::Result<IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumObjectParam)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RevokeObjectParam<P0>(&self, pszkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RevokeObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IBindCtx_Impl: windows_core::IUnknownImpl {
    fn RegisterObjectBound(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RevokeObjectBound(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ReleaseBoundObjects(&self) -> windows_core::Result<()>;
    fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> windows_core::Result<()>;
    fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> windows_core::Result<()>;
    fn GetRunningObjectTable(&self) -> windows_core::Result<IRunningObjectTable>;
    fn RegisterObjectParam(&self, pszkey: &windows_core::PCWSTR, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetObjectParam(&self, pszkey: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn EnumObjectParam(&self) -> windows_core::Result<IEnumString>;
    fn RevokeObjectParam(&self, pszkey: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IBindCtx_Vtbl {
    pub const fn new<Identity: IBindCtx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterObjectBound<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RegisterObjectBound(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn RevokeObjectBound<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RevokeObjectBound(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn ReleaseBoundObjects<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::ReleaseBoundObjects(this).into()
            }
        }
        unsafe extern "system" fn SetBindOptions<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindopts: *const BIND_OPTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::SetBindOptions(this, core::mem::transmute_copy(&pbindopts)).into()
            }
        }
        unsafe extern "system" fn GetBindOptions<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::GetBindOptions(this, core::mem::transmute_copy(&pbindopts)).into()
            }
        }
        unsafe extern "system" fn GetRunningObjectTable<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindCtx_Impl::GetRunningObjectTable(this) {
                    Ok(ok__) => {
                        pprot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RegisterObjectParam(this, core::mem::transmute(&pszkey), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn GetObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindCtx_Impl::GetObjectParam(this, core::mem::transmute(&pszkey)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindCtx_Impl::EnumObjectParam(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RevokeObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RevokeObjectParam(this, core::mem::transmute(&pszkey)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterObjectBound: RegisterObjectBound::<Identity, OFFSET>,
            RevokeObjectBound: RevokeObjectBound::<Identity, OFFSET>,
            ReleaseBoundObjects: ReleaseBoundObjects::<Identity, OFFSET>,
            SetBindOptions: SetBindOptions::<Identity, OFFSET>,
            GetBindOptions: GetBindOptions::<Identity, OFFSET>,
            GetRunningObjectTable: GetRunningObjectTable::<Identity, OFFSET>,
            RegisterObjectParam: RegisterObjectParam::<Identity, OFFSET>,
            GetObjectParam: GetObjectParam::<Identity, OFFSET>,
            EnumObjectParam: EnumObjectParam::<Identity, OFFSET>,
            RevokeObjectParam: RevokeObjectParam::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindCtx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBindCtx {}
windows_core::imp::define_interface!(IBindHost, IBindHost_Vtbl, 0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
windows_core::imp::interface_hierarchy!(IBindHost, windows_core::IUnknown);
impl IBindHost {
    pub unsafe fn CreateMoniker<P0, P1>(&self, szname: P0, pbc: P1, ppmk: *mut Option<IMoniker>, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateMoniker)(windows_core::Interface::as_raw(self), szname.param().abi(), pbc.param().abi(), core::mem::transmute(ppmk), dwreserved).ok() }
    }
    pub unsafe fn MonikerBindToStorage<P0, P1, P2>(&self, pmk: P0, pbc: P1, pbsc: P2, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMoniker>,
        P1: windows_core::Param<IBindCtx>,
        P2: windows_core::Param<IBindStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).MonikerBindToStorage)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), pbsc.param().abi(), riid, ppvobj as _).ok() }
    }
    pub unsafe fn MonikerBindToObject<P0, P1, P2>(&self, pmk: P0, pbc: P1, pbsc: P2, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMoniker>,
        P1: windows_core::Param<IBindCtx>,
        P2: windows_core::Param<IBindStatusCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).MonikerBindToObject)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), pbsc.param().abi(), riid, ppvobj as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MonikerBindToStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MonikerBindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBindHost_Impl: windows_core::IUnknownImpl {
    fn CreateMoniker(&self, szname: &windows_core::PCWSTR, pbc: windows_core::Ref<IBindCtx>, ppmk: windows_core::OutRef<IMoniker>, dwreserved: u32) -> windows_core::Result<()>;
    fn MonikerBindToStorage(&self, pmk: windows_core::Ref<IMoniker>, pbc: windows_core::Ref<IBindCtx>, pbsc: windows_core::Ref<IBindStatusCallback>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn MonikerBindToObject(&self, pmk: windows_core::Ref<IMoniker>, pbc: windows_core::Ref<IBindCtx>, pbsc: windows_core::Ref<IBindStatusCallback>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IBindHost_Vtbl {
    pub const fn new<Identity: IBindHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMoniker<Identity: IBindHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindHost_Impl::CreateMoniker(this, core::mem::transmute(&szname), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&ppmk), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn MonikerBindToStorage<Identity: IBindHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pbsc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindHost_Impl::MonikerBindToStorage(this, core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pbsc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn MonikerBindToObject<Identity: IBindHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pbsc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindHost_Impl::MonikerBindToObject(this, core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pbsc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMoniker: CreateMoniker::<Identity, OFFSET>,
            MonikerBindToStorage: MonikerBindToStorage::<Identity, OFFSET>,
            MonikerBindToObject: MonikerBindToObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBindHost {}
windows_core::imp::define_interface!(IBindStatusCallback, IBindStatusCallback_Vtbl, 0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IBindStatusCallback, windows_core::IUnknown);
impl IBindStatusCallback {
    pub unsafe fn OnStartBinding<P1>(&self, dwreserved: u32, pib: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IBinding>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStartBinding)(windows_core::Interface::as_raw(self), dwreserved, pib.param().abi()).ok() }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLowResource)(windows_core::Interface::as_raw(self), reserved).ok() }
    }
    pub unsafe fn OnProgress<P3>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), ulprogress, ulprogressmax, ulstatuscode, szstatustext.param().abi()).ok() }
    }
    pub unsafe fn OnStopBinding<P1>(&self, hresult: windows_core::HRESULT, szerror: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStopBinding)(windows_core::Interface::as_raw(self), hresult, szerror.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBindInfo)(windows_core::Interface::as_raw(self), grfbindf as _, core::mem::transmute(pbindinfo)).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDataAvailable)(windows_core::Interface::as_raw(self), grfbscf, dwsize, pformatetc, core::mem::transmute(pstgmed)).ok() }
    }
    pub unsafe fn OnObjectAvailable<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnObjectAvailable)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallback_Impl: windows_core::IUnknownImpl {
    fn OnStartBinding(&self, dwreserved: u32, pib: windows_core::Ref<IBinding>) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn OnLowResource(&self, reserved: u32) -> windows_core::Result<()>;
    fn OnProgress(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnStopBinding(&self, hresult: windows_core::HRESULT, szerror: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::Result<()>;
    fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> windows_core::Result<()>;
    fn OnObjectAvailable(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallback_Vtbl {
    pub const fn new<Identity: IBindStatusCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartBinding<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, pib: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnStartBinding(this, core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&pib)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindStatusCallback_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        pnpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnLowResource<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnLowResource(this, core::mem::transmute_copy(&reserved)).into()
            }
        }
        unsafe extern "system" fn OnProgress<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnProgress(this, core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax), core::mem::transmute_copy(&ulstatuscode), core::mem::transmute(&szstatustext)).into()
            }
        }
        unsafe extern "system" fn OnStopBinding<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, szerror: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnStopBinding(this, core::mem::transmute_copy(&hresult), core::mem::transmute(&szerror)).into()
            }
        }
        unsafe extern "system" fn GetBindInfo<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::GetBindInfo(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo)).into()
            }
        }
        unsafe extern "system" fn OnDataAvailable<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnDataAvailable(this, core::mem::transmute_copy(&grfbscf), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed)).into()
            }
        }
        unsafe extern "system" fn OnObjectAvailable<Identity: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallback_Impl::OnObjectAvailable(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartBinding: OnStartBinding::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            OnLowResource: OnLowResource::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnStopBinding: OnStopBinding::<Identity, OFFSET>,
            GetBindInfo: GetBindInfo::<Identity, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, OFFSET>,
            OnObjectAvailable: OnObjectAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IBindStatusCallback {}
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
        unsafe { (windows_core::Interface::vtable(self).GetBindInfoEx)(windows_core::Interface::as_raw(self), grfbindf as _, core::mem::transmute(pbindinfo), grfbindf2 as _, pdwreserved as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackEx_Vtbl {
    pub base__: IBindStatusCallback_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BINDINFO, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfoEx: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallbackEx_Impl: IBindStatusCallback_Impl {
    fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallbackEx_Vtbl {
    pub const fn new<Identity: IBindStatusCallbackEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBindInfoEx<Identity: IBindStatusCallbackEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindStatusCallbackEx_Impl::GetBindInfoEx(this, core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&grfbindf2), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: IBindStatusCallback_Vtbl::new::<Identity, OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindStatusCallbackEx as windows_core::Interface>::IID || iid == &<IBindStatusCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IBindStatusCallbackEx {}
windows_core::imp::define_interface!(IBinding, IBinding_Vtbl, 0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IBinding, windows_core::IUnknown);
impl IBinding {
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Suspend(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetPriority(&self, npriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), npriority).ok() }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBindResult)(windows_core::Interface::as_raw(self), pclsidprotocol as _, pdwresult as _, pszresult as _, pdwreserved as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetBindResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IBinding_Impl: windows_core::IUnknownImpl {
    fn Abort(&self) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetPriority(&self, npriority: i32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn GetBindResult(&self, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl IBinding_Vtbl {
    pub const fn new<Identity: IBinding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Abort<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn Suspend<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::Suspend(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, npriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::SetPriority(this, core::mem::transmute_copy(&npriority)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBinding_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        pnpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBindResult<Identity: IBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBinding_Impl::GetBindResult(this, core::mem::transmute_copy(&pclsidprotocol), core::mem::transmute_copy(&pdwresult), core::mem::transmute_copy(&pszresult), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Abort: Abort::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            GetBindResult: GetBindResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBinding as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBinding {}
windows_core::imp::define_interface!(IBlockingLock, IBlockingLock_Vtbl, 0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
windows_core::imp::interface_hierarchy!(IBlockingLock, windows_core::IUnknown);
impl IBlockingLock {
    pub unsafe fn Lock(&self, dwtimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwtimeout).ok() }
    }
    pub unsafe fn Unlock(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBlockingLock_Impl: windows_core::IUnknownImpl {
    fn Lock(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
}
impl IBlockingLock_Vtbl {
    pub const fn new<Identity: IBlockingLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IBlockingLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBlockingLock_Impl::Lock(this, core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IBlockingLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBlockingLock_Impl::Unlock(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lock: Lock::<Identity, OFFSET>, Unlock: Unlock::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockingLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBlockingLock {}
windows_core::imp::define_interface!(ICallFactory, ICallFactory_Vtbl, 0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICallFactory, windows_core::IUnknown);
impl ICallFactory {
    pub unsafe fn CreateCall<P1>(&self, riid: *const windows_core::GUID, pctrlunk: P1, riid2: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCall)(windows_core::Interface::as_raw(self), riid, pctrlunk.param().abi(), riid2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICallFactory_Impl: windows_core::IUnknownImpl {
    fn CreateCall(&self, riid: *const windows_core::GUID, pctrlunk: windows_core::Ref<windows_core::IUnknown>, riid2: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl ICallFactory_Vtbl {
    pub const fn new<Identity: ICallFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCall<Identity: ICallFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pctrlunk: *mut core::ffi::c_void, riid2: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallFactory_Impl::CreateCall(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pctrlunk), core::mem::transmute_copy(&riid2)) {
                    Ok(ok__) => {
                        ppv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateCall: CreateCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICallFactory {}
windows_core::imp::define_interface!(ICancelMethodCalls, ICancelMethodCalls_Vtbl, 0x00000029_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICancelMethodCalls, windows_core::IUnknown);
impl ICancelMethodCalls {
    pub unsafe fn Cancel(&self, ulseconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), ulseconds).ok() }
    }
    pub unsafe fn TestCancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TestCancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCalls_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TestCancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICancelMethodCalls_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self, ulseconds: u32) -> windows_core::Result<()>;
    fn TestCancel(&self) -> windows_core::Result<()>;
}
impl ICancelMethodCalls_Vtbl {
    pub const fn new<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICancelMethodCalls_Impl::Cancel(this, core::mem::transmute_copy(&ulseconds)).into()
            }
        }
        unsafe extern "system" fn TestCancel<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICancelMethodCalls_Impl::TestCancel(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET>, TestCancel: TestCancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICancelMethodCalls as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICancelMethodCalls {}
windows_core::imp::define_interface!(ICatInformation, ICatInformation_Vtbl, 0x0002e013_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICatInformation, windows_core::IUnknown);
impl ICatInformation {
    pub unsafe fn EnumCategories(&self, lcid: u32) -> windows_core::Result<IEnumCATEGORYINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCategories)(windows_core::Interface::as_raw(self), lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const windows_core::GUID, lcid: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategoryDesc)(windows_core::Interface::as_raw(self), rcatid, lcid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumClassesOfCategories(&self, rgcatidimpl: &[windows_core::GUID], rgcatidreq: &[windows_core::GUID]) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumClassesOfCategories)(windows_core::Interface::as_raw(self), rgcatidimpl.len().try_into().unwrap(), core::mem::transmute(rgcatidimpl.as_ptr()), rgcatidreq.len().try_into().unwrap(), core::mem::transmute(rgcatidreq.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const windows_core::GUID, rgcatidimpl: &[windows_core::GUID], rgcatidreq: &[windows_core::GUID]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsClassOfCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatidimpl.len().try_into().unwrap(), core::mem::transmute(rgcatidimpl.as_ptr()), rgcatidreq.len().try_into().unwrap(), core::mem::transmute(rgcatidreq.as_ptr())).ok() }
    }
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumImplCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumReqCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCategoryDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub EnumClassesOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClassOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub EnumImplCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumReqCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICatInformation_Impl: windows_core::IUnknownImpl {
    fn EnumCategories(&self, lcid: u32) -> windows_core::Result<IEnumCATEGORYINFO>;
    fn GetCategoryDesc(&self, rcatid: *const windows_core::GUID, lcid: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
    fn IsClassOfCategories(&self, rclsid: *const windows_core::GUID, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumImplCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
    fn EnumReqCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID>;
}
impl ICatInformation_Vtbl {
    pub const fn new<Identity: ICatInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumCategories(this, core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        ppenumcategoryinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategoryDesc<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcatid: *const windows_core::GUID, lcid: u32, pszdesc: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::GetCategoryDesc(this, core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        pszdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumClassesOfCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID, ppenumclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumClassesOfCategories(this, core::mem::transmute_copy(&cimplemented), core::mem::transmute_copy(&rgcatidimpl), core::mem::transmute_copy(&crequired), core::mem::transmute_copy(&rgcatidreq)) {
                    Ok(ok__) => {
                        ppenumclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsClassOfCategories<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, cimplemented: u32, rgcatidimpl: *const windows_core::GUID, crequired: u32, rgcatidreq: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatInformation_Impl::IsClassOfCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&cimplemented), core::mem::transmute_copy(&rgcatidimpl), core::mem::transmute_copy(&crequired), core::mem::transmute_copy(&rgcatidreq)).into()
            }
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumImplCategoriesOfClass(this, core::mem::transmute_copy(&rclsid)) {
                    Ok(ok__) => {
                        ppenumcatid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumReqCategoriesOfClass<Identity: ICatInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ppenumcatid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatInformation_Impl::EnumReqCategoriesOfClass(this, core::mem::transmute_copy(&rclsid)) {
                    Ok(ok__) => {
                        ppenumcatid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumCategories: EnumCategories::<Identity, OFFSET>,
            GetCategoryDesc: GetCategoryDesc::<Identity, OFFSET>,
            EnumClassesOfCategories: EnumClassesOfCategories::<Identity, OFFSET>,
            IsClassOfCategories: IsClassOfCategories::<Identity, OFFSET>,
            EnumImplCategoriesOfClass: EnumImplCategoriesOfClass::<Identity, OFFSET>,
            EnumReqCategoriesOfClass: EnumReqCategoriesOfClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICatInformation {}
windows_core::imp::define_interface!(ICatRegister, ICatRegister_Vtbl, 0x0002e012_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICatRegister, windows_core::IUnknown);
impl ICatRegister {
    pub unsafe fn RegisterCategories(&self, rgcategoryinfo: &[CATEGORYINFO]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterCategories)(windows_core::Interface::as_raw(self), rgcategoryinfo.len().try_into().unwrap(), core::mem::transmute(rgcategoryinfo.as_ptr())).ok() }
    }
    pub unsafe fn UnRegisterCategories(&self, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterCategories)(windows_core::Interface::as_raw(self), rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok() }
    }
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok() }
    }
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok() }
    }
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok() }
    }
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const CATEGORYINFO) -> windows_core::HRESULT,
    pub UnRegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnRegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnRegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ICatRegister_Impl: windows_core::IUnknownImpl {
    fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::Result<()>;
    fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnRegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnRegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ICatRegister_Vtbl {
    pub const fn new<Identity: ICatRegister_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcategoryinfo)).into()
            }
        }
        unsafe extern "system" fn UnRegisterCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterCategories(this, core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn RegisterClassImplCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterClassImplCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn RegisterClassReqCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::RegisterClassReqCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Identity: ICatRegister_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ccategories: u32, rgcatid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatRegister_Impl::UnRegisterClassReqCategories(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&ccategories), core::mem::transmute_copy(&rgcatid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCategories: RegisterCategories::<Identity, OFFSET>,
            UnRegisterCategories: UnRegisterCategories::<Identity, OFFSET>,
            RegisterClassImplCategories: RegisterClassImplCategories::<Identity, OFFSET>,
            UnRegisterClassImplCategories: UnRegisterClassImplCategories::<Identity, OFFSET>,
            RegisterClassReqCategories: RegisterClassReqCategories::<Identity, OFFSET>,
            UnRegisterClassReqCategories: UnRegisterClassReqCategories::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatRegister as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICatRegister {}
windows_core::imp::define_interface!(IChannelHook, IChannelHook_Vtbl, 0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
windows_core::imp::interface_hierarchy!(IChannelHook, windows_core::IUnknown);
impl IChannelHook {
    pub unsafe fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID) -> u32 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientGetSize)(windows_core::Interface::as_raw(self), uextent, riid, &mut result__);
            result__
        }
    }
    pub unsafe fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void) {
        unsafe { (windows_core::Interface::vtable(self).ClientFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize as _, pdatabuffer) }
    }
    pub unsafe fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT) {
        unsafe { (windows_core::Interface::vtable(self).ClientNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep, hrfault) }
    }
    pub unsafe fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32) {
        unsafe { (windows_core::Interface::vtable(self).ServerNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep) }
    }
    pub unsafe fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT) -> u32 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerGetSize)(windows_core::Interface::as_raw(self), uextent, riid, hrfault, &mut result__);
            result__
        }
    }
    pub unsafe fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT) {
        unsafe { (windows_core::Interface::vtable(self).ServerFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize as _, pdatabuffer, hrfault) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ClientGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32),
    pub ClientFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *const core::ffi::c_void),
    pub ClientNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *const core::ffi::c_void, u32, windows_core::HRESULT),
    pub ServerNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *const core::ffi::c_void, u32),
    pub ServerGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT, *mut u32),
    pub ServerFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *const core::ffi::c_void, windows_core::HRESULT),
}
pub trait IChannelHook_Impl: windows_core::IUnknownImpl {
    fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32);
    fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void);
    fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT);
    fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32);
    fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT, pdatasize: *mut u32);
    fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT);
}
impl IChannelHook_Vtbl {
    pub const fn new<Identity: IChannelHook_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClientGetSize<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize))
            }
        }
        unsafe extern "system" fn ClientFillBuffer<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientFillBuffer(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdatabuffer))
            }
        }
        unsafe extern "system" fn ClientNotify<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep), core::mem::transmute_copy(&hrfault))
            }
        }
        unsafe extern "system" fn ServerNotify<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep))
            }
        }
        unsafe extern "system" fn ServerGetSize<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT, pdatasize: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&hrfault), core::mem::transmute_copy(&pdatasize))
            }
        }
        unsafe extern "system" fn ServerFillBuffer<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerFillBuffer(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&hrfault))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ClientGetSize: ClientGetSize::<Identity, OFFSET>,
            ClientFillBuffer: ClientFillBuffer::<Identity, OFFSET>,
            ClientNotify: ClientNotify::<Identity, OFFSET>,
            ServerNotify: ServerNotify::<Identity, OFFSET>,
            ServerGetSize: ServerGetSize::<Identity, OFFSET>,
            ServerFillBuffer: ServerFillBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelHook as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IChannelHook {}
windows_core::imp::define_interface!(IClassActivator, IClassActivator_Vtbl, 0x00000140_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IClassActivator, windows_core::IUnknown);
impl IClassActivator {
    pub unsafe fn GetClassObject<T>(&self, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetClassObject)(windows_core::Interface::as_raw(self), rclsid, dwclasscontext, locale, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IClassActivator_Impl: windows_core::IUnknownImpl {
    fn GetClassObject(&self, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IClassActivator_Vtbl {
    pub const fn new<Identity: IClassActivator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassObject<Identity: IClassActivator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassActivator_Impl::GetClassObject(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwclasscontext), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassObject: GetClassObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassActivator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClassActivator {}
windows_core::imp::define_interface!(IClassFactory, IClassFactory_Vtbl, 0x00000001_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IClassFactory, windows_core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn LockServer(&self, flock: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockServer)(windows_core::Interface::as_raw(self), flock.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn LockServer(&self, flock: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IClassFactory_Vtbl {
    pub const fn new<Identity: IClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn LockServer<Identity: IClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassFactory_Impl::LockServer(this, core::mem::transmute_copy(&flock)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            LockServer: LockServer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClassFactory {}
windows_core::imp::define_interface!(IClientSecurity, IClientSecurity_Vtbl, 0x0000013d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IClientSecurity, windows_core::IUnknown);
impl IClientSecurity {
    pub unsafe fn QueryBlanket<P0>(&self, pproxy: P0, pauthnsvc: *mut u32, pauthzsvc: Option<*mut u32>, pserverprincname: *mut *mut u16, pauthnlevel: Option<*mut RPC_C_AUTHN_LEVEL>, pimplevel: Option<*mut RPC_C_IMP_LEVEL>, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), pauthnsvc as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pauthinfo as _, pcapabilites.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetBlanket<P0, P3>(&self, pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: P3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: Option<*const core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.param().abi(), dwauthnlevel, dwimplevel, pauthinfo.unwrap_or(core::mem::zeroed()) as _, dwcapabilities.0 as _).ok() }
    }
    pub unsafe fn CopyProxy<P0>(&self, pproxy: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyProxy)(windows_core::Interface::as_raw(self), pproxy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut u16, *mut RPC_C_AUTHN_LEVEL, *mut RPC_C_IMP_LEVEL, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, RPC_C_AUTHN_LEVEL, RPC_C_IMP_LEVEL, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CopyProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IClientSecurity_Impl: windows_core::IUnknownImpl {
    fn QueryBlanket(&self, pproxy: windows_core::Ref<windows_core::IUnknown>, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut u32) -> windows_core::Result<()>;
    fn SetBlanket(&self, pproxy: windows_core::Ref<windows_core::IUnknown>, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: &windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const core::ffi::c_void, dwcapabilities: &EOLE_AUTHENTICATION_CAPABILITIES) -> windows_core::Result<()>;
    fn CopyProxy(&self, pproxy: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl IClientSecurity_Vtbl {
    pub const fn new<Identity: IClientSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryBlanket<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClientSecurity_Impl::QueryBlanket(this, core::mem::transmute_copy(&pproxy), core::mem::transmute_copy(&pauthnsvc), core::mem::transmute_copy(&pauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&pauthnlevel), core::mem::transmute_copy(&pimplevel), core::mem::transmute_copy(&pauthinfo), core::mem::transmute_copy(&pcapabilites)).into()
            }
        }
        unsafe extern "system" fn SetBlanket<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const core::ffi::c_void, dwcapabilities: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClientSecurity_Impl::SetBlanket(this, core::mem::transmute_copy(&pproxy), core::mem::transmute_copy(&dwauthnsvc), core::mem::transmute_copy(&dwauthzsvc), core::mem::transmute(&pserverprincname), core::mem::transmute_copy(&dwauthnlevel), core::mem::transmute_copy(&dwimplevel), core::mem::transmute_copy(&pauthinfo), core::mem::transmute(&dwcapabilities)).into()
            }
        }
        unsafe extern "system" fn CopyProxy<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, ppcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClientSecurity_Impl::CopyProxy(this, core::mem::transmute_copy(&pproxy)) {
                    Ok(ok__) => {
                        ppcopy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, OFFSET>,
            SetBlanket: SetBlanket::<Identity, OFFSET>,
            CopyProxy: CopyProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClientSecurity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IClientSecurity {}
windows_core::imp::define_interface!(IComThreadingInfo, IComThreadingInfo_Vtbl, 0x000001ce_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IComThreadingInfo, windows_core::IUnknown);
impl IComThreadingInfo {
    pub unsafe fn GetCurrentApartmentType(&self) -> windows_core::Result<APTTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentApartmentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentThreadType(&self) -> windows_core::Result<THDTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentThreadType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentLogicalThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentLogicalThreadId)(windows_core::Interface::as_raw(self), rguid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentApartmentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APTTYPE) -> windows_core::HRESULT,
    pub GetCurrentThreadType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut THDTYPE) -> windows_core::HRESULT,
    pub GetCurrentLogicalThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetCurrentLogicalThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComThreadingInfo_Impl: windows_core::IUnknownImpl {
    fn GetCurrentApartmentType(&self) -> windows_core::Result<APTTYPE>;
    fn GetCurrentThreadType(&self) -> windows_core::Result<THDTYPE>;
    fn GetCurrentLogicalThreadId(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetCurrentLogicalThreadId(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IComThreadingInfo_Vtbl {
    pub const fn new<Identity: IComThreadingInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentApartmentType<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papttype: *mut APTTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComThreadingInfo_Impl::GetCurrentApartmentType(this) {
                    Ok(ok__) => {
                        papttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentThreadType<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthreadtype: *mut THDTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComThreadingInfo_Impl::GetCurrentThreadType(this) {
                    Ok(ok__) => {
                        pthreadtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentLogicalThreadId<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidlogicalthreadid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComThreadingInfo_Impl::GetCurrentLogicalThreadId(this) {
                    Ok(ok__) => {
                        pguidlogicalthreadid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentLogicalThreadId<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadingInfo_Impl::SetCurrentLogicalThreadId(this, core::mem::transmute_copy(&rguid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentApartmentType: GetCurrentApartmentType::<Identity, OFFSET>,
            GetCurrentThreadType: GetCurrentThreadType::<Identity, OFFSET>,
            GetCurrentLogicalThreadId: GetCurrentLogicalThreadId::<Identity, OFFSET>,
            SetCurrentLogicalThreadId: SetCurrentLogicalThreadId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComThreadingInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComThreadingInfo {}
windows_core::imp::define_interface!(IConnectionPoint, IConnectionPoint_Vtbl, 0xb196b286_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IConnectionPoint, windows_core::IUnknown);
impl IConnectionPoint {
    pub unsafe fn GetConnectionInterface(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionInterface)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionPointContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Advise<P0>(&self, punksink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), punksink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
    pub unsafe fn EnumConnections(&self) -> windows_core::Result<IEnumConnections> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetConnectionPointContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IConnectionPoint_Impl: windows_core::IUnknownImpl {
    fn GetConnectionInterface(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer>;
    fn Advise(&self, punksink: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn EnumConnections(&self) -> windows_core::Result<IEnumConnections>;
}
impl IConnectionPoint_Vtbl {
    pub const fn new<Identity: IConnectionPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectionInterface<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::GetConnectionInterface(this) {
                    Ok(ok__) => {
                        piid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectionPointContainer<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcpc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::GetConnectionPointContainer(this) {
                    Ok(ok__) => {
                        ppcpc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punksink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::Advise(this, core::mem::transmute_copy(&punksink)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectionPoint_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn EnumConnections<Identity: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPoint_Impl::EnumConnections(this) {
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
            GetConnectionInterface: GetConnectionInterface::<Identity, OFFSET>,
            GetConnectionPointContainer: GetConnectionPointContainer::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumConnections: EnumConnections::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionPoint as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConnectionPoint {}
windows_core::imp::define_interface!(IConnectionPointContainer, IConnectionPointContainer_Vtbl, 0xb196b284_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IConnectionPointContainer, windows_core::IUnknown);
impl IConnectionPointContainer {
    pub unsafe fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumConnectionPoints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<IConnectionPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindConnectionPoint)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumConnectionPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindConnectionPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IConnectionPointContainer_Impl: windows_core::IUnknownImpl {
    fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints>;
    fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<IConnectionPoint>;
}
impl IConnectionPointContainer_Vtbl {
    pub const fn new<Identity: IConnectionPointContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumConnectionPoints<Identity: IConnectionPointContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPointContainer_Impl::EnumConnectionPoints(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindConnectionPoint<Identity: IConnectionPointContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnectionPointContainer_Impl::FindConnectionPoint(this, core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppcp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumConnectionPoints: EnumConnectionPoints::<Identity, OFFSET>,
            FindConnectionPoint: FindConnectionPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectionPointContainer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConnectionPointContainer {}
windows_core::imp::define_interface!(IContext, IContext_Vtbl, 0x000001c0_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IContext, windows_core::IUnknown);
impl IContext {
    pub unsafe fn SetProperty<P2>(&self, rpolicyid: *const windows_core::GUID, flags: u32, punk: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), rpolicyid, flags, punk.param().abi()).ok() }
    }
    pub unsafe fn RemoveProperty(&self, rpolicyid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveProperty)(windows_core::Interface::as_raw(self), rpolicyid).ok() }
    }
    pub unsafe fn GetProperty(&self, rguid: *const windows_core::GUID, pflags: *mut u32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), rguid, pflags as _, core::mem::transmute(ppunk)).ok() }
    }
    pub unsafe fn EnumContextProps(&self) -> windows_core::Result<IEnumContextProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumContextProps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumContextProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IContext_Impl: windows_core::IUnknownImpl {
    fn SetProperty(&self, rpolicyid: *const windows_core::GUID, flags: u32, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveProperty(&self, rpolicyid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetProperty(&self, rguid: *const windows_core::GUID, pflags: *mut u32, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumContextProps(&self) -> windows_core::Result<IEnumContextProps>;
}
impl IContext_Vtbl {
    pub const fn new<Identity: IContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: IContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rpolicyid: *const windows_core::GUID, flags: u32, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContext_Impl::SetProperty(this, core::mem::transmute_copy(&rpolicyid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn RemoveProperty<Identity: IContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rpolicyid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContext_Impl::RemoveProperty(this, core::mem::transmute_copy(&rpolicyid)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pflags: *mut u32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContext_Impl::GetProperty(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pflags), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn EnumContextProps<Identity: IContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcontextprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContext_Impl::EnumContextProps(this) {
                    Ok(ok__) => {
                        ppenumcontextprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            EnumContextProps: EnumContextProps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContext {}
windows_core::imp::define_interface!(IContextCallback, IContextCallback_Vtbl, 0x000001da_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IContextCallback, windows_core::IUnknown);
impl IContextCallback {
    pub unsafe fn ContextCallback<P4>(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const windows_core::GUID, imethod: i32, punk: P4) -> windows_core::Result<()>
    where
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ContextCallback)(windows_core::Interface::as_raw(self), pfncallback, pparam, riid, imethod, punk.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ContextCallback: unsafe extern "system" fn(*mut core::ffi::c_void, PFNCONTEXTCALL, *const ComCallData, *const windows_core::GUID, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IContextCallback_Impl: windows_core::IUnknownImpl {
    fn ContextCallback(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const windows_core::GUID, imethod: i32, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IContextCallback_Vtbl {
    pub const fn new<Identity: IContextCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContextCallback<Identity: IContextCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const windows_core::GUID, imethod: i32, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextCallback_Impl::ContextCallback(this, core::mem::transmute_copy(&pfncallback), core::mem::transmute_copy(&pparam), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&punk)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ContextCallback: ContextCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContextCallback {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDLFLAGS(pub u16);
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
pub const IDLFLAG_FIN: IDLFLAGS = IDLFLAGS(1u16);
pub const IDLFLAG_FLCID: IDLFLAGS = IDLFLAGS(4u16);
pub const IDLFLAG_FOUT: IDLFLAGS = IDLFLAGS(2u16);
pub const IDLFLAG_FRETVAL: IDLFLAGS = IDLFLAGS(8u16);
pub const IDLFLAG_NONE: IDLFLAGS = IDLFLAGS(0u16);
windows_core::imp::define_interface!(IDataAdviseHolder, IDataAdviseHolder_Vtbl, 0x00000110_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDataAdviseHolder, windows_core::IUnknown);
impl IDataAdviseHolder {
    pub unsafe fn Advise<P0, P3>(&self, pdataobject: P0, pfetc: *const FORMATETC, advf: u32, padvise: P3) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDataObject>,
        P3: windows_core::Param<IAdviseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), pfetc, advf, padvise.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwconnection).ok() }
    }
    pub unsafe fn EnumAdvise(&self) -> windows_core::Result<IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendOnDataChange<P0>(&self, pdataobject: P0, dwreserved: Option<u32>, advf: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendOnDataChange)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, advf).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const FORMATETC, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IDataAdviseHolder_Impl: windows_core::IUnknownImpl {
    fn Advise(&self, pdataobject: windows_core::Ref<IDataObject>, pfetc: *const FORMATETC, advf: u32, padvise: windows_core::Ref<IAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumAdvise(&self) -> windows_core::Result<IEnumSTATDATA>;
    fn SendOnDataChange(&self, pdataobject: windows_core::Ref<IDataObject>, dwreserved: u32, advf: u32) -> windows_core::Result<()>;
}
impl IDataAdviseHolder_Vtbl {
    pub const fn new<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, pfetc: *const FORMATETC, advf: u32, padvise: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataAdviseHolder_Impl::Advise(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pfetc), core::mem::transmute_copy(&advf), core::mem::transmute_copy(&padvise)) {
                    Ok(ok__) => {
                        pdwconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataAdviseHolder_Impl::Unadvise(this, core::mem::transmute_copy(&dwconnection)).into()
            }
        }
        unsafe extern "system" fn EnumAdvise<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataAdviseHolder_Impl::EnumAdvise(this) {
                    Ok(ok__) => {
                        ppenumadvise.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendOnDataChange<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, dwreserved: u32, advf: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataAdviseHolder_Impl::SendOnDataChange(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&advf)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, OFFSET>,
            SendOnDataChange: SendOnDataChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataAdviseHolder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDataAdviseHolder {}
windows_core::imp::define_interface!(IDataObject, IDataObject_Vtbl, 0x0000010e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDataObject, windows_core::IUnknown);
impl IDataObject {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> windows_core::Result<STGMEDIUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pformatetcin, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDataHere)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pmedium)).ok() }
    }
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryGetData)(windows_core::Interface::as_raw(self), pformatetc) }
    }
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCanonicalFormatEtc)(windows_core::Interface::as_raw(self), pformatectin, pformatetcout as _) }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pmedium), frelease.into()).ok() }
    }
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> windows_core::Result<IEnumFORMATETC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFormatEtc)(windows_core::Interface::as_raw(self), dwdirection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DAdvise<P2>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: P2) -> windows_core::Result<u32>
    where
        P2: windows_core::Param<IAdviseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DAdvise)(windows_core::Interface::as_raw(self), pformatetc, advf, padvsink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DUnadvise)(windows_core::Interface::as_raw(self), dwconnection).ok() }
    }
    pub unsafe fn EnumDAdvise(&self) -> windows_core::Result<IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *const STGMEDIUM, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
    pub EnumFormatEtc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DUnadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumDAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDataObject_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, pformatetcin: *const FORMATETC) -> windows_core::Result<STGMEDIUM>;
    fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::Result<()>;
    fn QueryGetData(&self, pformatetc: *const FORMATETC) -> windows_core::HRESULT;
    fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT;
    fn SetData(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnumFormatEtc(&self, dwdirection: u32) -> windows_core::Result<IEnumFORMATETC>;
    fn DAdvise(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: windows_core::Ref<IAdviseSink>) -> windows_core::Result<u32>;
    fn DUnadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumDAdvise(&self) -> windows_core::Result<IEnumSTATDATA>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IDataObject_Vtbl {
    pub const fn new<Identity: IDataObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::GetData(this, core::mem::transmute_copy(&pformatetcin)) {
                    Ok(ok__) => {
                        pmedium.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataHere<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::GetDataHere(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium)).into()
            }
        }
        unsafe extern "system" fn QueryGetData<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::QueryGetData(this, core::mem::transmute_copy(&pformatetc))
            }
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::GetCanonicalFormatEtc(this, core::mem::transmute_copy(&pformatectin), core::mem::transmute_copy(&pformatetcout))
            }
        }
        unsafe extern "system" fn SetData<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::SetData(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&frelease)).into()
            }
        }
        unsafe extern "system" fn EnumFormatEtc<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::EnumFormatEtc(this, core::mem::transmute_copy(&dwdirection)) {
                    Ok(ok__) => {
                        ppenumformatetc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DAdvise<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::DAdvise(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&advf), core::mem::transmute_copy(&padvsink)) {
                    Ok(ok__) => {
                        pdwconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DUnadvise<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::DUnadvise(this, core::mem::transmute_copy(&dwconnection)).into()
            }
        }
        unsafe extern "system" fn EnumDAdvise<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::EnumDAdvise(this) {
                    Ok(ok__) => {
                        ppenumadvise.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, OFFSET>,
            GetDataHere: GetDataHere::<Identity, OFFSET>,
            QueryGetData: QueryGetData::<Identity, OFFSET>,
            GetCanonicalFormatEtc: GetCanonicalFormatEtc::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            EnumFormatEtc: EnumFormatEtc::<Identity, OFFSET>,
            DAdvise: DAdvise::<Identity, OFFSET>,
            DUnadvise: DUnadvise::<Identity, OFFSET>,
            EnumDAdvise: EnumDAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IDataObject {}
windows_core::imp::define_interface!(IDispatch, IDispatch_Vtbl, 0x00020400_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, rgdispid as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: Option<*mut super::Variant::VARIANT>, pexcepinfo: Option<*mut EXCEPINFO>, puargerr: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult.unwrap_or(core::mem::zeroed()) as _, pexcepinfo.unwrap_or(core::mem::zeroed()) as _, puargerr.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::PCWSTR, u32, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, u32, DISPATCH_FLAGS, *const DISPPARAMS, *mut super::Variant::VARIANT, *mut EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Invoke: usize,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDispatch_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> windows_core::Result<ITypeInfo>;
    fn GetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::Result<()>;
    fn Invoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDispatch_Vtbl {
    pub const fn new<Identity: IDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctinfo: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatch_Impl::GetTypeInfoCount(this) {
                    Ok(ok__) => {
                        pctinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatch_Impl::GetTypeInfo(this, core::mem::transmute_copy(&itinfo), core::mem::transmute_copy(&lcid)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatch_Impl::GetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&rgdispid)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: IDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatch_Impl::Invoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(IEnumCATEGORYINFO, IEnumCATEGORYINFO_Vtbl, 0x0002e011_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumCATEGORYINFO, windows_core::IUnknown);
impl IEnumCATEGORYINFO {
    pub unsafe fn Next(&self, rgelt: &mut [CATEGORYINFO], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCATEGORYINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CATEGORYINFO, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumCATEGORYINFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCATEGORYINFO>;
}
impl IEnumCATEGORYINFO_Vtbl {
    pub const fn new<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCATEGORYINFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCATEGORYINFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumCATEGORYINFO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumCATEGORYINFO_Impl::Clone(this) {
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
        iid == &<IEnumCATEGORYINFO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumCATEGORYINFO {}
windows_core::imp::define_interface!(IEnumConnectionPoints, IEnumConnectionPoints_Vtbl, 0xb196b285_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IEnumConnectionPoints, windows_core::IUnknown);
impl IEnumConnectionPoints {
    pub unsafe fn Next(&self, ppcp: &mut [Option<IConnectionPoint>], pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppcp.len().try_into().unwrap(), core::mem::transmute(ppcp.as_ptr()), pcfetched as _) }
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumConnectionPoints> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumConnectionPoints_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cconnections: u32, ppcp: *mut Option<IConnectionPoint>, pcfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, cconnections: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnectionPoints>;
}
impl IEnumConnectionPoints_Vtbl {
    pub const fn new<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32, ppcp: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnectionPoints_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&ppcp), core::mem::transmute_copy(&pcfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnectionPoints_Impl::Skip(this, core::mem::transmute_copy(&cconnections)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnectionPoints_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumConnectionPoints_Impl::Clone(this) {
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
        iid == &<IEnumConnectionPoints as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumConnectionPoints {}
windows_core::imp::define_interface!(IEnumConnections, IEnumConnections_Vtbl, 0xb196b287_bab4_101a_b69c_00aa00341d07);
windows_core::imp::interface_hierarchy!(IEnumConnections, windows_core::IUnknown);
impl IEnumConnections {
    pub unsafe fn Next(&self, rgcd: &mut [CONNECTDATA], pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgcd.len().try_into().unwrap(), core::mem::transmute(rgcd.as_ptr()), pcfetched as _) }
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumConnections> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CONNECTDATA, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumConnections_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, cconnections: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumConnections>;
}
impl IEnumConnections_Vtbl {
    pub const fn new<Identity: IEnumConnections_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnections_Impl::Next(this, core::mem::transmute_copy(&cconnections), core::mem::transmute_copy(&rgcd), core::mem::transmute_copy(&pcfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cconnections: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnections_Impl::Skip(this, core::mem::transmute_copy(&cconnections)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumConnections_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumConnections_Impl::Clone(this) {
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
        iid == &<IEnumConnections as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumConnections {}
windows_core::imp::define_interface!(IEnumContextProps, IEnumContextProps_Vtbl, 0x000001c1_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumContextProps, windows_core::IUnknown);
impl IEnumContextProps {
    pub unsafe fn Next(&self, pcontextproperties: &mut [ContextProperty], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pcontextproperties.len().try_into().unwrap(), core::mem::transmute(pcontextproperties.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumContextProps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumContextProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ContextProperty, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumContextProps_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, pcontextproperties: *mut ContextProperty, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumContextProps>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl IEnumContextProps_Vtbl {
    pub const fn new<Identity: IEnumContextProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumContextProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pcontextproperties: *mut ContextProperty, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumContextProps_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pcontextproperties), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumContextProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumContextProps_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumContextProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumContextProps_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumContextProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumcontextprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumContextProps_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumcontextprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IEnumContextProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumContextProps_Impl::Count(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
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
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumContextProps as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumContextProps {}
windows_core::imp::define_interface!(IEnumFORMATETC, IEnumFORMATETC_Vtbl, 0x00000103_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumFORMATETC, windows_core::IUnknown);
impl IEnumFORMATETC {
    pub unsafe fn Next(&self, rgelt: &mut [FORMATETC], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumFORMATETC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETC_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut FORMATETC, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumFORMATETC_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFORMATETC>;
}
impl IEnumFORMATETC_Vtbl {
    pub const fn new<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFORMATETC_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFORMATETC_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFORMATETC_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumFORMATETC_Impl::Clone(this) {
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
        iid == &<IEnumFORMATETC as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumFORMATETC {}
windows_core::imp::define_interface!(IEnumGUID, IEnumGUID_Vtbl, 0x0002e000_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumGUID, windows_core::IUnknown);
impl IEnumGUID {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::GUID], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumGUID_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumGUID>;
}
impl IEnumGUID_Vtbl {
    pub const fn new<Identity: IEnumGUID_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumGUID_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumGUID_Impl::Skip(this, core::mem::transmute_copy(&celt))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumGUID_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumGUID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumGUID_Impl::Clone(this) {
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
        iid == &<IEnumGUID as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumGUID {}
windows_core::imp::define_interface!(IEnumMoniker, IEnumMoniker_Vtbl, 0x00000102_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumMoniker, windows_core::IUnknown);
impl IEnumMoniker {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IMoniker>], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMoniker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumMoniker_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IMoniker>, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMoniker>;
}
impl IEnumMoniker_Vtbl {
    pub const fn new<Identity: IEnumMoniker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMoniker_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMoniker_Impl::Skip(this, core::mem::transmute_copy(&celt))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMoniker_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumMoniker_Impl::Clone(this) {
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
        iid == &<IEnumMoniker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumMoniker {}
windows_core::imp::define_interface!(IEnumSTATDATA, IEnumSTATDATA_Vtbl, 0x00000105_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATDATA, windows_core::IUnknown);
impl IEnumSTATDATA {
    pub unsafe fn Next(&self, rgelt: &mut [STATDATA], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATDATA, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSTATDATA_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATDATA>;
}
impl IEnumSTATDATA_Vtbl {
    pub const fn new<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATDATA_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATDATA_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATDATA_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATDATA_Impl::Clone(this) {
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
        iid == &<IEnumSTATDATA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSTATDATA {}
windows_core::imp::define_interface!(IEnumString, IEnumString_Vtbl, 0x00000101_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumString, windows_core::IUnknown);
impl IEnumString {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::PWSTR], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumString_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumString>;
}
impl IEnumString_Vtbl {
    pub const fn new<Identity: IEnumString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Skip(this, core::mem::transmute_copy(&celt))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumString_Impl::Clone(this) {
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
        iid == &<IEnumString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumString {}
windows_core::imp::define_interface!(IEnumUnknown, IEnumUnknown_Vtbl, 0x00000100_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumUnknown, windows_core::IUnknown);
impl IEnumUnknown {
    pub unsafe fn Next(&self, rgelt: &mut [Option<windows_core::IUnknown>], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumUnknown_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<windows_core::IUnknown>, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumUnknown>;
}
impl IEnumUnknown_Vtbl {
    pub const fn new<Identity: IEnumUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumUnknown_Impl::Clone(this) {
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
        iid == &<IEnumUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumUnknown {}
windows_core::imp::define_interface!(IErrorInfo, IErrorInfo_Vtbl, 0x1cf2b120_547d_101b_8e65_08002b2bd119);
windows_core::imp::interface_hierarchy!(IErrorInfo, windows_core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHelpFile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHelpContext(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpContext(&self) -> windows_core::Result<u32>;
}
impl IErrorInfo_Vtbl {
    pub const fn new<Identity: IErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGUID<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetGUID(this) {
                    Ok(ok__) => {
                        pguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetSource(this) {
                    Ok(ok__) => {
                        pbstrsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpFile<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetHelpFile(this) {
                    Ok(ok__) => {
                        pbstrhelpfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpContext<Identity: IErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhelpcontext: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IErrorInfo_Impl::GetHelpContext(this) {
                    Ok(ok__) => {
                        pdwhelpcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetHelpFile: GetHelpFile::<Identity, OFFSET>,
            GetHelpContext: GetHelpContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IErrorInfo {}
windows_core::imp::define_interface!(IErrorLog, IErrorLog_Vtbl, 0x3127ca40_446e_11ce_8135_00aa004bb851);
windows_core::imp::interface_hierarchy!(IErrorLog, windows_core::IUnknown);
impl IErrorLog {
    pub unsafe fn AddError<P0>(&self, pszpropname: P0, pexcepinfo: *const EXCEPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddError)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), core::mem::transmute(pexcepinfo)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const EXCEPINFO) -> windows_core::HRESULT,
}
pub trait IErrorLog_Impl: windows_core::IUnknownImpl {
    fn AddError(&self, pszpropname: &windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> windows_core::Result<()>;
}
impl IErrorLog_Vtbl {
    pub const fn new<Identity: IErrorLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddError<Identity: IErrorLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropname: windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IErrorLog_Impl::AddError(this, core::mem::transmute(&pszpropname), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddError: AddError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorLog as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IErrorLog {}
windows_core::imp::define_interface!(IExternalConnection, IExternalConnection_Vtbl, 0x00000019_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IExternalConnection, windows_core::IUnknown);
impl IExternalConnection {
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddConnection)(windows_core::Interface::as_raw(self), extconn, reserved) }
    }
    pub unsafe fn ReleaseConnection(&self, extconn: u32, reserved: u32, flastreleasecloses: bool) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).ReleaseConnection)(windows_core::Interface::as_raw(self), extconn, reserved, flastreleasecloses.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> u32,
    pub ReleaseConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::BOOL) -> u32,
}
pub trait IExternalConnection_Impl: windows_core::IUnknownImpl {
    fn AddConnection(&self, extconn: u32, reserved: u32) -> u32;
    fn ReleaseConnection(&self, extconn: u32, reserved: u32, flastreleasecloses: windows_core::BOOL) -> u32;
}
impl IExternalConnection_Vtbl {
    pub const fn new<Identity: IExternalConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddConnection<Identity: IExternalConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: u32, reserved: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExternalConnection_Impl::AddConnection(this, core::mem::transmute_copy(&extconn), core::mem::transmute_copy(&reserved))
            }
        }
        unsafe extern "system" fn ReleaseConnection<Identity: IExternalConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: windows_core::BOOL) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExternalConnection_Impl::ReleaseConnection(this, core::mem::transmute_copy(&extconn), core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&flastreleasecloses))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddConnection: AddConnection::<Identity, OFFSET>,
            ReleaseConnection: ReleaseConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExternalConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IExternalConnection {}
windows_core::imp::define_interface!(IFastRundown, IFastRundown_Vtbl, 0x00000040_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IFastRundown, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IFastRundown_Impl: windows_core::IUnknownImpl {}
impl IFastRundown_Vtbl {
    pub const fn new<Identity: IFastRundown_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFastRundown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFastRundown {}
windows_core::imp::define_interface!(IForegroundTransfer, IForegroundTransfer_Vtbl, 0x00000145_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IForegroundTransfer, windows_core::IUnknown);
impl IForegroundTransfer {
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllowForegroundTransfer)(windows_core::Interface::as_raw(self), lpvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllowForegroundTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IForegroundTransfer_Impl: windows_core::IUnknownImpl {
    fn AllowForegroundTransfer(&self, lpvreserved: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IForegroundTransfer_Vtbl {
    pub const fn new<Identity: IForegroundTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowForegroundTransfer<Identity: IForegroundTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvreserved: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IForegroundTransfer_Impl::AllowForegroundTransfer(this, core::mem::transmute_copy(&lpvreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AllowForegroundTransfer: AllowForegroundTransfer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IForegroundTransfer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IForegroundTransfer {}
windows_core::imp::define_interface!(IGlobalInterfaceTable, IGlobalInterfaceTable_Vtbl, 0x00000146_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IGlobalInterfaceTable, windows_core::IUnknown);
impl IGlobalInterfaceTable {
    pub unsafe fn RegisterInterfaceInGlobal<P0>(&self, punk: P0, riid: *const windows_core::GUID) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterInterfaceInGlobal)(windows_core::Interface::as_raw(self), punk.param().abi(), riid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevokeInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie, riid, ppv as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterInterfaceInGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub RevokeInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGlobalInterfaceTable_Impl: windows_core::IUnknownImpl {
    fn RegisterInterfaceInGlobal(&self, punk: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IGlobalInterfaceTable_Vtbl {
    pub const fn new<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, riid: *const windows_core::GUID, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGlobalInterfaceTable_Impl::RegisterInterfaceInGlobal(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalInterfaceTable_Impl::RevokeInterfaceFromGlobal(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalInterfaceTable_Impl::GetInterfaceFromGlobal(this, core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterInterfaceInGlobal: RegisterInterfaceInGlobal::<Identity, OFFSET>,
            RevokeInterfaceFromGlobal: RevokeInterfaceFromGlobal::<Identity, OFFSET>,
            GetInterfaceFromGlobal: GetInterfaceFromGlobal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGlobalInterfaceTable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGlobalInterfaceTable {}
windows_core::imp::define_interface!(IGlobalOptions, IGlobalOptions_Vtbl, 0x0000015b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IGlobalOptions, windows_core::IUnknown);
impl IGlobalOptions {
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), dwproperty, dwvalue).ok() }
    }
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), dwproperty, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, usize) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, *mut usize) -> windows_core::HRESULT,
}
pub trait IGlobalOptions_Impl: windows_core::IUnknownImpl {
    fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>;
    fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<usize>;
}
impl IGlobalOptions_Vtbl {
    pub const fn new<Identity: IGlobalOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalOptions_Impl::Set(this, core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGlobalOptions_Impl::Query(this, core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        pdwvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, OFFSET>, Query: Query::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGlobalOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGlobalOptions {}
windows_core::imp::define_interface!(IInitializeSpy, IInitializeSpy_Vtbl, 0x00000034_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IInitializeSpy, windows_core::IUnknown);
impl IInitializeSpy {
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreInitialize)(windows_core::Interface::as_raw(self), dwcoinit, dwcurthreadaptrefs).ok() }
    }
    pub unsafe fn PostInitialize(&self, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostInitialize)(windows_core::Interface::as_raw(self), hrcoinit, dwcoinit, dwnewthreadaptrefs).ok() }
    }
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreUninitialize)(windows_core::Interface::as_raw(self), dwcurthreadaptrefs).ok() }
    }
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostUninitialize)(windows_core::Interface::as_raw(self), dwnewthreadaptrefs).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreInitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub PostInitialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, u32) -> windows_core::HRESULT,
    pub PreUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PostUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IInitializeSpy_Impl: windows_core::IUnknownImpl {
    fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PostInitialize(&self, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> windows_core::Result<()>;
}
impl IInitializeSpy_Vtbl {
    pub const fn new<Identity: IInitializeSpy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreInitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PreInitialize(this, core::mem::transmute_copy(&dwcoinit), core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
            }
        }
        unsafe extern "system" fn PostInitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PostInitialize(this, core::mem::transmute_copy(&hrcoinit), core::mem::transmute_copy(&dwcoinit), core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
            }
        }
        unsafe extern "system" fn PreUninitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcurthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PreUninitialize(this, core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
            }
        }
        unsafe extern "system" fn PostUninitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnewthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PostUninitialize(this, core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreInitialize: PreInitialize::<Identity, OFFSET>,
            PostInitialize: PostInitialize::<Identity, OFFSET>,
            PreUninitialize: PreUninitialize::<Identity, OFFSET>,
            PostUninitialize: PostUninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeSpy as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInitializeSpy {}
windows_core::imp::define_interface!(IInternalUnknown, IInternalUnknown_Vtbl, 0x00000021_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IInternalUnknown, windows_core::IUnknown);
impl IInternalUnknown {
    pub unsafe fn QueryInternalInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryInternalInterface)(windows_core::Interface::as_raw(self), riid, ppv as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInternalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInternalUnknown_Impl: windows_core::IUnknownImpl {
    fn QueryInternalInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IInternalUnknown_Vtbl {
    pub const fn new<Identity: IInternalUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInternalInterface<Identity: IInternalUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternalUnknown_Impl::QueryInternalInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryInternalInterface: QueryInternalInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternalUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternalUnknown {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMPLTYPEFLAGS(pub i32);
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
pub const IMPLTYPEFLAG_FDEFAULT: IMPLTYPEFLAGS = IMPLTYPEFLAGS(1i32);
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: IMPLTYPEFLAGS = IMPLTYPEFLAGS(8i32);
pub const IMPLTYPEFLAG_FRESTRICTED: IMPLTYPEFLAGS = IMPLTYPEFLAGS(4i32);
pub const IMPLTYPEFLAG_FSOURCE: IMPLTYPEFLAGS = IMPLTYPEFLAGS(2i32);
windows_core::imp::define_interface!(IMachineGlobalObjectTable, IMachineGlobalObjectTable_Vtbl, 0x26d709ac_f70b_4421_a96f_d2878fafb00d);
windows_core::imp::interface_hierarchy!(IMachineGlobalObjectTable, windows_core::IUnknown);
impl IMachineGlobalObjectTable {
    pub unsafe fn RegisterObject<P1, P2>(&self, clsid: *const windows_core::GUID, identifier: P1, object: P2) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), object.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObject<P1, T>(&self, clsid: *const windows_core::GUID, identifier: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RevokeObject(&self, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevokeObject)(windows_core::Interface::as_raw(self), token).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *mut core::ffi::c_void, *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeObject: unsafe extern "system" fn(*mut core::ffi::c_void, MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
}
pub trait IMachineGlobalObjectTable_Impl: windows_core::IUnknownImpl {
    fn RegisterObject(&self, clsid: *const windows_core::GUID, identifier: &windows_core::PCWSTR, object: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>;
    fn GetObject(&self, clsid: *const windows_core::GUID, identifier: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RevokeObject(&self, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::Result<()>;
}
impl IMachineGlobalObjectTable_Vtbl {
    pub const fn new<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, identifier: windows_core::PCWSTR, object: *mut core::ffi::c_void, token: *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMachineGlobalObjectTable_Impl::RegisterObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&identifier), core::mem::transmute_copy(&object)) {
                    Ok(ok__) => {
                        token.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, identifier: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineGlobalObjectTable_Impl::GetObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&identifier), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn RevokeObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineGlobalObjectTable_Impl::RevokeObject(this, core::mem::transmute_copy(&token)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterObject: RegisterObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            RevokeObject: RevokeObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMachineGlobalObjectTable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMachineGlobalObjectTable {}
windows_core::imp::define_interface!(IMalloc, IMalloc_Vtbl, 0x00000002_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMalloc, windows_core::IUnknown);
impl IMalloc {
    pub unsafe fn Alloc(&self, cb: usize) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cb) }
    }
    pub unsafe fn Realloc(&self, pv: Option<*const core::ffi::c_void>, cb: usize) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Realloc)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _, cb) }
    }
    pub unsafe fn Free(&self, pv: Option<*const core::ffi::c_void>) {
        unsafe { (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetSize(&self, pv: Option<*const core::ffi::c_void>) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DidAlloc(&self, pv: Option<*const core::ffi::c_void>) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).DidAlloc)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn HeapMinimize(&self) {
        unsafe { (windows_core::Interface::vtable(self).HeapMinimize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> *mut core::ffi::c_void,
    pub Realloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> *mut core::ffi::c_void,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> usize,
    pub DidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> i32,
    pub HeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMalloc_Impl: windows_core::IUnknownImpl {
    fn Alloc(&self, cb: usize) -> *mut core::ffi::c_void;
    fn Realloc(&self, pv: *const core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void;
    fn Free(&self, pv: *const core::ffi::c_void);
    fn GetSize(&self, pv: *const core::ffi::c_void) -> usize;
    fn DidAlloc(&self, pv: *const core::ffi::c_void) -> i32;
    fn HeapMinimize(&self);
}
impl IMalloc_Vtbl {
    pub const fn new<Identity: IMalloc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cb))
            }
        }
        unsafe extern "system" fn Realloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::Realloc(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb))
            }
        }
        unsafe extern "system" fn Free<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::Free(this, core::mem::transmute_copy(&pv))
            }
        }
        unsafe extern "system" fn GetSize<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::GetSize(this, core::mem::transmute_copy(&pv))
            }
        }
        unsafe extern "system" fn DidAlloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::DidAlloc(this, core::mem::transmute_copy(&pv))
            }
        }
        unsafe extern "system" fn HeapMinimize<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::HeapMinimize(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, OFFSET>,
            Realloc: Realloc::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            DidAlloc: DidAlloc::<Identity, OFFSET>,
            HeapMinimize: HeapMinimize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMalloc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMalloc {}
windows_core::imp::define_interface!(IMallocSpy, IMallocSpy_Vtbl, 0x0000001d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMallocSpy, windows_core::IUnknown);
impl IMallocSpy {
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        unsafe { (windows_core::Interface::vtable(self).PreAlloc)(windows_core::Interface::as_raw(self), cbrequest) }
    }
    pub unsafe fn PostAlloc(&self, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PostAlloc)(windows_core::Interface::as_raw(self), pactual) }
    }
    pub unsafe fn PreFree(&self, prequest: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PreFree)(windows_core::Interface::as_raw(self), prequest, fspyed.into()) }
    }
    pub unsafe fn PostFree(&self, fspyed: bool) {
        unsafe { (windows_core::Interface::vtable(self).PostFree)(windows_core::Interface::as_raw(self), fspyed.into()) }
    }
    pub unsafe fn PreRealloc(&self, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: bool) -> usize {
        unsafe { (windows_core::Interface::vtable(self).PreRealloc)(windows_core::Interface::as_raw(self), prequest, cbrequest, ppnewrequest as _, fspyed.into()) }
    }
    pub unsafe fn PostRealloc(&self, pactual: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PostRealloc)(windows_core::Interface::as_raw(self), pactual, fspyed.into()) }
    }
    pub unsafe fn PreGetSize(&self, prequest: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PreGetSize)(windows_core::Interface::as_raw(self), prequest, fspyed.into()) }
    }
    pub unsafe fn PostGetSize(&self, cbactual: usize, fspyed: bool) -> usize {
        unsafe { (windows_core::Interface::vtable(self).PostGetSize)(windows_core::Interface::as_raw(self), cbactual, fspyed.into()) }
    }
    pub unsafe fn PreDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PreDidAlloc)(windows_core::Interface::as_raw(self), prequest, fspyed.into()) }
    }
    pub unsafe fn PostDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: bool, factual: i32) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).PostDidAlloc)(windows_core::Interface::as_raw(self), prequest, fspyed.into(), factual) }
    }
    pub unsafe fn PreHeapMinimize(&self) {
        unsafe { (windows_core::Interface::vtable(self).PreHeapMinimize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PostHeapMinimize(&self) {
        unsafe { (windows_core::Interface::vtable(self).PostHeapMinimize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> usize,
    pub PostAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> *mut core::ffi::c_void,
    pub PreFree: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PostFree: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    pub PreRealloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void, windows_core::BOOL) -> usize,
    pub PostRealloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PreGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PostGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::BOOL) -> usize,
    pub PreDidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PostDidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL, i32) -> i32,
    pub PreHeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PostHeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMallocSpy_Impl: windows_core::IUnknownImpl {
    fn PreAlloc(&self, cbrequest: usize) -> usize;
    fn PostAlloc(&self, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn PreFree(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PostFree(&self, fspyed: windows_core::BOOL);
    fn PreRealloc(&self, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: windows_core::BOOL) -> usize;
    fn PostRealloc(&self, pactual: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PreGetSize(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PostGetSize(&self, cbactual: usize, fspyed: windows_core::BOOL) -> usize;
    fn PreDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PostDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL, factual: i32) -> i32;
    fn PreHeapMinimize(&self);
    fn PostHeapMinimize(&self);
}
impl IMallocSpy_Vtbl {
    pub const fn new<Identity: IMallocSpy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbrequest: usize) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreAlloc(this, core::mem::transmute_copy(&cbrequest))
            }
        }
        unsafe extern "system" fn PostAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostAlloc(this, core::mem::transmute_copy(&pactual))
            }
        }
        unsafe extern "system" fn PreFree<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreFree(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostFree<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fspyed: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostFree(this, core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PreRealloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: windows_core::BOOL) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreRealloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&cbrequest), core::mem::transmute_copy(&ppnewrequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostRealloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactual: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostRealloc(this, core::mem::transmute_copy(&pactual), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PreGetSize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreGetSize(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostGetSize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbactual: usize, fspyed: windows_core::BOOL) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostGetSize(this, core::mem::transmute_copy(&cbactual), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PreDidAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreDidAlloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostDidAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL, factual: i32) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostDidAlloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed), core::mem::transmute_copy(&factual))
            }
        }
        unsafe extern "system" fn PreHeapMinimize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreHeapMinimize(this)
            }
        }
        unsafe extern "system" fn PostHeapMinimize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostHeapMinimize(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreAlloc: PreAlloc::<Identity, OFFSET>,
            PostAlloc: PostAlloc::<Identity, OFFSET>,
            PreFree: PreFree::<Identity, OFFSET>,
            PostFree: PostFree::<Identity, OFFSET>,
            PreRealloc: PreRealloc::<Identity, OFFSET>,
            PostRealloc: PostRealloc::<Identity, OFFSET>,
            PreGetSize: PreGetSize::<Identity, OFFSET>,
            PostGetSize: PostGetSize::<Identity, OFFSET>,
            PreDidAlloc: PreDidAlloc::<Identity, OFFSET>,
            PostDidAlloc: PostDidAlloc::<Identity, OFFSET>,
            PreHeapMinimize: PreHeapMinimize::<Identity, OFFSET>,
            PostHeapMinimize: PostHeapMinimize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMallocSpy as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMallocSpy {}
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
        unsafe { (windows_core::Interface::vtable(self).BindToObject)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn BindToStorage<P0, P1, T>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).BindToStorage)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Reduce<P0>(&self, pbc: P0, dwreducehowfar: u32, ppmktoleft: *mut Option<IMoniker>, ppmkreduced: *mut Option<IMoniker>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Reduce)(windows_core::Interface::as_raw(self), pbc.param().abi(), dwreducehowfar, core::mem::transmute(ppmktoleft), core::mem::transmute(ppmkreduced)).ok() }
    }
    pub unsafe fn ComposeWith<P0>(&self, pmkright: P0, fonlyifnotgeneric: bool) -> windows_core::Result<IMoniker>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComposeWith)(windows_core::Interface::as_raw(self), pmkright.param().abi(), fonlyifnotgeneric.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Enum(&self, fforward: bool) -> windows_core::Result<IEnumMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), fforward.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsEqual<P0>(&self, pmkothermoniker: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pmkothermoniker.param().abi()) }
    }
    pub unsafe fn Hash(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hash)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsRunning<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pmknewlyrunning: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
        P2: windows_core::Param<IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), pmknewlyrunning.param().abi()).ok() }
    }
    pub unsafe fn GetTimeOfLastChange<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<super::super::Foundation::FILETIME>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeOfLastChange)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Inverse(&self) -> windows_core::Result<IMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Inverse)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonPrefixWith<P0>(&self, pmkother: P0) -> windows_core::Result<IMoniker>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonPrefixWith)(windows_core::Interface::as_raw(self), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RelativePathTo<P0>(&self, pmkother: P0) -> windows_core::Result<IMoniker>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelativePathTo)(windows_core::Interface::as_raw(self), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDisplayName<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ParseDisplayName<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pszdisplayname: P2, pcheaten: *mut u32, ppmkout: *mut Option<IMoniker>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), pszdisplayname.param().abi(), pcheaten as _, core::mem::transmute(ppmkout)).ok() }
    }
    pub unsafe fn IsSystemMoniker(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSystemMoniker)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMoniker_Vtbl {
    pub base__: IPersistStream_Vtbl,
    pub BindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindToStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reduce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComposeWith: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
pub trait IMoniker_Impl: IPersistStream_Impl {
    fn BindToObject(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, riidresult: *const windows_core::GUID, ppvresult: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn BindToStorage(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Reduce(&self, pbc: windows_core::Ref<IBindCtx>, dwreducehowfar: u32, ppmktoleft: windows_core::OutRef<IMoniker>, ppmkreduced: windows_core::OutRef<IMoniker>) -> windows_core::Result<()>;
    fn ComposeWith(&self, pmkright: windows_core::Ref<IMoniker>, fonlyifnotgeneric: windows_core::BOOL) -> windows_core::Result<IMoniker>;
    fn Enum(&self, fforward: windows_core::BOOL) -> windows_core::Result<IEnumMoniker>;
    fn IsEqual(&self, pmkothermoniker: windows_core::Ref<IMoniker>) -> windows_core::HRESULT;
    fn Hash(&self) -> windows_core::Result<u32>;
    fn IsRunning(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, pmknewlyrunning: windows_core::Ref<IMoniker>) -> windows_core::Result<()>;
    fn GetTimeOfLastChange(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn Inverse(&self) -> windows_core::Result<IMoniker>;
    fn CommonPrefixWith(&self, pmkother: windows_core::Ref<IMoniker>) -> windows_core::Result<IMoniker>;
    fn RelativePathTo(&self, pmkother: windows_core::Ref<IMoniker>) -> windows_core::Result<IMoniker>;
    fn GetDisplayName(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>) -> windows_core::Result<windows_core::PWSTR>;
    fn ParseDisplayName(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, pszdisplayname: &windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: windows_core::OutRef<IMoniker>) -> windows_core::Result<()>;
    fn IsSystemMoniker(&self) -> windows_core::Result<u32>;
}
impl IMoniker_Vtbl {
    pub const fn new<Identity: IMoniker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindToObject<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, riidresult: *const windows_core::GUID, ppvresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::BindToObject(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute_copy(&riidresult), core::mem::transmute_copy(&ppvresult)).into()
            }
        }
        unsafe extern "system" fn BindToStorage<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::BindToStorage(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn Reduce<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, dwreducehowfar: u32, ppmktoleft: *mut *mut core::ffi::c_void, ppmkreduced: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::Reduce(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&dwreducehowfar), core::mem::transmute_copy(&ppmktoleft), core::mem::transmute_copy(&ppmkreduced)).into()
            }
        }
        unsafe extern "system" fn ComposeWith<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkright: *mut core::ffi::c_void, fonlyifnotgeneric: windows_core::BOOL, ppmkcomposite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::ComposeWith(this, core::mem::transmute_copy(&pmkright), core::mem::transmute_copy(&fonlyifnotgeneric)) {
                    Ok(ok__) => {
                        ppmkcomposite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enum<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforward: windows_core::BOOL, ppenummoniker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::Enum(this, core::mem::transmute_copy(&fforward)) {
                    Ok(ok__) => {
                        ppenummoniker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkothermoniker: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::IsEqual(this, core::mem::transmute_copy(&pmkothermoniker))
            }
        }
        unsafe extern "system" fn Hash<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::Hash(this) {
                    Ok(ok__) => {
                        pdwhash.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRunning<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pmknewlyrunning: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::IsRunning(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute_copy(&pmknewlyrunning)).into()
            }
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::GetTimeOfLastChange(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft)) {
                    Ok(ok__) => {
                        pfiletime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Inverse<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::Inverse(this) {
                    Ok(ok__) => {
                        ppmk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonPrefixWith<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkother: *mut core::ffi::c_void, ppmkprefix: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::CommonPrefixWith(this, core::mem::transmute_copy(&pmkother)) {
                    Ok(ok__) => {
                        ppmkprefix.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RelativePathTo<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkother: *mut core::ffi::c_void, ppmkrelpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::RelativePathTo(this, core::mem::transmute_copy(&pmkother)) {
                    Ok(ok__) => {
                        ppmkrelpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, ppszdisplayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::GetDisplayName(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft)) {
                    Ok(ok__) => {
                        ppszdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParseDisplayName<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pszdisplayname: windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::ParseDisplayName(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&pcheaten), core::mem::transmute_copy(&ppmkout)).into()
            }
        }
        unsafe extern "system" fn IsSystemMoniker<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmksys: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::IsSystemMoniker(this) {
                    Ok(ok__) => {
                        pdwmksys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPersistStream_Vtbl::new::<Identity, OFFSET>(),
            BindToObject: BindToObject::<Identity, OFFSET>,
            BindToStorage: BindToStorage::<Identity, OFFSET>,
            Reduce: Reduce::<Identity, OFFSET>,
            ComposeWith: ComposeWith::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Hash: Hash::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, OFFSET>,
            Inverse: Inverse::<Identity, OFFSET>,
            CommonPrefixWith: CommonPrefixWith::<Identity, OFFSET>,
            RelativePathTo: RelativePathTo::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            ParseDisplayName: ParseDisplayName::<Identity, OFFSET>,
            IsSystemMoniker: IsSystemMoniker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMoniker as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID || iid == &<IPersistStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMoniker {}
windows_core::imp::define_interface!(IMultiQI, IMultiQI_Vtbl, 0x00000020_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMultiQI, windows_core::IUnknown);
impl IMultiQI {
    pub unsafe fn QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), core::mem::transmute(pmqis.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MULTI_QI) -> windows_core::HRESULT,
}
pub trait IMultiQI_Impl: windows_core::IUnknownImpl {
    fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
impl IMultiQI_Vtbl {
    pub const fn new<Identity: IMultiQI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryMultipleInterfaces<Identity: IMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiQI_Impl::QueryMultipleInterfaces(this, core::mem::transmute_copy(&cmqis), core::mem::transmute_copy(&pmqis)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryMultipleInterfaces: QueryMultipleInterfaces::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiQI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMultiQI {}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct INTERFACEINFO {
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub iid: windows_core::GUID,
    pub wMethod: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INVOKEKIND(pub i32);
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
windows_core::imp::define_interface!(INoMarshal, INoMarshal_Vtbl, 0xecc8691b_c1db_4dc0_855e_65f6c551af49);
windows_core::imp::interface_hierarchy!(INoMarshal, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait INoMarshal_Impl: windows_core::IUnknownImpl {}
impl INoMarshal_Vtbl {
    pub const fn new<Identity: INoMarshal_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INoMarshal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INoMarshal {}
windows_core::imp::define_interface!(IOplockStorage, IOplockStorage_Vtbl, 0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
windows_core::imp::interface_hierarchy!(IOplockStorage, windows_core::IUnknown);
impl IOplockStorage {
    pub unsafe fn CreateStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateStorageEx)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, stgfmt, grfattrs, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn OpenStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenStorageEx)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, stgfmt, grfattrs, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStorageEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenStorageEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOplockStorage_Impl: windows_core::IUnknownImpl {
    fn CreateStorageEx(&self, pwcsname: &windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenStorageEx(&self, pwcsname: &windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IOplockStorage_Vtbl {
    pub const fn new<Identity: IOplockStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStorageEx<Identity: IOplockStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOplockStorage_Impl::CreateStorageEx(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&stgfmt), core::mem::transmute_copy(&grfattrs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstgopen)).into()
            }
        }
        unsafe extern "system" fn OpenStorageEx<Identity: IOplockStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOplockStorage_Impl::OpenStorageEx(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&stgfmt), core::mem::transmute_copy(&grfattrs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstgopen)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStorageEx: CreateStorageEx::<Identity, OFFSET>,
            OpenStorageEx: OpenStorageEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOplockStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOplockStorage {}
windows_core::imp::define_interface!(IPSFactoryBuffer, IPSFactoryBuffer_Vtbl, 0xd5f569d0_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IPSFactoryBuffer, windows_core::IUnknown);
impl IPSFactoryBuffer {
    pub unsafe fn CreateProxy<P0>(&self, punkouter: P0, riid: *const windows_core::GUID, ppproxy: *mut Option<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateProxy)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, core::mem::transmute(ppproxy), ppv as _).ok() }
    }
    pub unsafe fn CreateStub<P1>(&self, riid: *const windows_core::GUID, punkserver: P1) -> windows_core::Result<IRpcStubBuffer>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStub)(windows_core::Interface::as_raw(self), riid, punkserver.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStub: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPSFactoryBuffer_Impl: windows_core::IUnknownImpl {
    fn CreateProxy(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppproxy: windows_core::OutRef<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateStub(&self, riid: *const windows_core::GUID, punkserver: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IRpcStubBuffer>;
}
impl IPSFactoryBuffer_Vtbl {
    pub const fn new<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProxy<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppproxy: *mut *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPSFactoryBuffer_Impl::CreateProxy(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppproxy), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateStub<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punkserver: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPSFactoryBuffer_Impl::CreateStub(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punkserver)) {
                    Ok(ok__) => {
                        ppstub.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateProxy: CreateProxy::<Identity, OFFSET>,
            CreateStub: CreateStub::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPSFactoryBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPSFactoryBuffer {}
windows_core::imp::define_interface!(IPersist, IPersist_Vtbl, 0x0000010c_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPersist, windows_core::IUnknown);
impl IPersist {
    pub unsafe fn GetClassID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IPersist_Impl: windows_core::IUnknownImpl {
    fn GetClassID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IPersist_Vtbl {
    pub const fn new<Identity: IPersist_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassID<Identity: IPersist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclassid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersist_Impl::GetClassID(this) {
                    Ok(ok__) => {
                        pclassid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassID: GetClassID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersist {}
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
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Load<P0>(&self, pszfilename: P0, dwmode: STGM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), dwmode).ok() }
    }
    pub unsafe fn Save<P0>(&self, pszfilename: P0, fremember: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), fremember.into()).ok() }
    }
    pub unsafe fn SaveCompleted<P0>(&self, pszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), pszfilename.param().abi()).ok() }
    }
    pub unsafe fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, STGM) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IPersistFile_Impl: IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pszfilename: &windows_core::PCWSTR, dwmode: STGM) -> windows_core::Result<()>;
    fn Save(&self, pszfilename: &windows_core::PCWSTR, fremember: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IPersistFile_Vtbl {
    pub const fn new<Identity: IPersistFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::IsDirty(this)
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, dwmode: STGM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::Load(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, fremember: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::Save(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&fremember)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::SaveCompleted(this, core::mem::transmute(&pszfilename)).into()
            }
        }
        unsafe extern "system" fn GetCurFile<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistFile_Impl::GetCurFile(this) {
                    Ok(ok__) => {
                        ppszfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            GetCurFile: GetCurFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistFile as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistFile {}
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
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Load(&self, pmem: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), core::mem::transmute(pmem.as_ptr()), pmem.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Save(&self, pmem: &mut [u8], fcleardirty: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), core::mem::transmute(pmem.as_ptr()), fcleardirty.into(), pmem.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemory_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPersistMemory_Impl: IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pmem: *const core::ffi::c_void, cbsize: u32) -> windows_core::Result<()>;
    fn Save(&self, pmem: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL, cbsize: u32) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u32>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
impl IPersistMemory_Vtbl {
    pub const fn new<Identity: IPersistMemory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::IsDirty(this)
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void, cbsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::Load(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&cbsize)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL, cbsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::Save(this, core::mem::transmute_copy(&pmem), core::mem::transmute_copy(&fcleardirty), core::mem::transmute_copy(&cbsize)).into()
            }
        }
        unsafe extern "system" fn GetSizeMax<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistMemory_Impl::GetSizeMax(this) {
                    Ok(ok__) => {
                        pcbsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitNew<Identity: IPersistMemory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistMemory_Impl::InitNew(this).into()
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistMemory as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistMemory {}
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
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Load<P0>(&self, pstm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstm.param().abi()).ok() }
    }
    pub unsafe fn Save<P0>(&self, pstm: P0, fcleardirty: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstm.param().abi(), fcleardirty.into()).ok() }
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStream_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IPersistStream_Impl: IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pstm: windows_core::Ref<IStream>) -> windows_core::Result<()>;
    fn Save(&self, pstm: windows_core::Ref<IStream>, fcleardirty: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u64>;
}
impl IPersistStream_Vtbl {
    pub const fn new<Identity: IPersistStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStream_Impl::IsDirty(this)
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStream_Impl::Load(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStream_Impl::Save(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&fcleardirty)).into()
            }
        }
        unsafe extern "system" fn GetSizeMax<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistStream_Impl::GetSizeMax(this) {
                    Ok(ok__) => {
                        pcbsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStream as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistStream {}
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
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Load<P0>(&self, pstm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstm.param().abi()).ok() }
    }
    pub unsafe fn Save<P0>(&self, pstm: P0, fcleardirty: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstm.param().abi(), fcleardirty.into()).ok() }
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInit_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPersistStreamInit_Impl: IPersist_Impl {
    fn IsDirty(&self) -> windows_core::HRESULT;
    fn Load(&self, pstm: windows_core::Ref<IStream>) -> windows_core::Result<()>;
    fn Save(&self, pstm: windows_core::Ref<IStream>, fcleardirty: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u64>;
    fn InitNew(&self) -> windows_core::Result<()>;
}
impl IPersistStreamInit_Vtbl {
    pub const fn new<Identity: IPersistStreamInit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::IsDirty(this)
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::Load(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::Save(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&fcleardirty)).into()
            }
        }
        unsafe extern "system" fn GetSizeMax<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistStreamInit_Impl::GetSizeMax(this) {
                    Ok(ok__) => {
                        pcbsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitNew<Identity: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStreamInit_Impl::InitNew(this).into()
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStreamInit as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistStreamInit {}
windows_core::imp::define_interface!(IPipeByte, IPipeByte_Vtbl, 0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeByte, windows_core::IUnknown);
impl IPipeByte {
    pub unsafe fn Pull(&self, buf: &mut [u8], pcreturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), pcreturned as _).ok() }
    }
    pub unsafe fn Push(&self, buf: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
pub trait IPipeByte_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const u8, csent: u32) -> windows_core::Result<()>;
}
impl IPipeByte_Vtbl {
    pub const fn new<Identity: IPipeByte_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeByte_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const u8, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeByte_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeByte as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPipeByte {}
windows_core::imp::define_interface!(IPipeDouble, IPipeDouble_Vtbl, 0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeDouble, windows_core::IUnknown);
impl IPipeDouble {
    pub unsafe fn Pull(&self, buf: &mut [f64], pcreturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), pcreturned as _).ok() }
    }
    pub unsafe fn Push(&self, buf: &[f64]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
}
pub trait IPipeDouble_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const f64, csent: u32) -> windows_core::Result<()>;
}
impl IPipeDouble_Vtbl {
    pub const fn new<Identity: IPipeDouble_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeDouble_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const f64, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeDouble_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeDouble as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPipeDouble {}
windows_core::imp::define_interface!(IPipeLong, IPipeLong_Vtbl, 0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeLong, windows_core::IUnknown);
impl IPipeLong {
    pub unsafe fn Pull(&self, buf: &mut [i32], pcreturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), pcreturned as _).ok() }
    }
    pub unsafe fn Push(&self, buf: &[i32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32) -> windows_core::HRESULT,
}
pub trait IPipeLong_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const i32, csent: u32) -> windows_core::Result<()>;
}
impl IPipeLong_Vtbl {
    pub const fn new<Identity: IPipeLong_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeLong_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const i32, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeLong_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeLong as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPipeLong {}
windows_core::imp::define_interface!(IProcessInitControl, IProcessInitControl_Vtbl, 0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
windows_core::imp::interface_hierarchy!(IProcessInitControl, windows_core::IUnknown);
impl IProcessInitControl {
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetInitializerTimeout)(windows_core::Interface::as_raw(self), dwsecondsremaining).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResetInitializerTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IProcessInitControl_Impl: windows_core::IUnknownImpl {
    fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> windows_core::Result<()>;
}
impl IProcessInitControl_Vtbl {
    pub const fn new<Identity: IProcessInitControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResetInitializerTimeout<Identity: IProcessInitControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsecondsremaining: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessInitControl_Impl::ResetInitializerTimeout(this, core::mem::transmute_copy(&dwsecondsremaining)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResetInitializerTimeout: ResetInitializerTimeout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessInitControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProcessInitControl {}
windows_core::imp::define_interface!(IProcessLock, IProcessLock_Vtbl, 0x000001d5_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IProcessLock, windows_core::IUnknown);
impl IProcessLock {
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRefOnProcess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).ReleaseRefOnProcess)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefOnProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub ReleaseRefOnProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait IProcessLock_Impl: windows_core::IUnknownImpl {
    fn AddRefOnProcess(&self) -> u32;
    fn ReleaseRefOnProcess(&self) -> u32;
}
impl IProcessLock_Vtbl {
    pub const fn new<Identity: IProcessLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRefOnProcess<Identity: IProcessLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessLock_Impl::AddRefOnProcess(this)
            }
        }
        unsafe extern "system" fn ReleaseRefOnProcess<Identity: IProcessLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessLock_Impl::ReleaseRefOnProcess(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefOnProcess: AddRefOnProcess::<Identity, OFFSET>,
            ReleaseRefOnProcess: ReleaseRefOnProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProcessLock {}
windows_core::imp::define_interface!(IProgressNotify, IProgressNotify_Vtbl, 0xa9d758a0_4617_11cf_95fc_00aa00680db4);
windows_core::imp::interface_hierarchy!(IProgressNotify, windows_core::IUnknown);
impl IProgressNotify {
    pub unsafe fn OnProgress(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: bool, fowner: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), dwprogresscurrent, dwprogressmaximum, faccurate.into(), fowner.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IProgressNotify_Impl: windows_core::IUnknownImpl {
    fn OnProgress(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: windows_core::BOOL, fowner: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IProgressNotify_Vtbl {
    pub const fn new<Identity: IProgressNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnProgress<Identity: IProgressNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: windows_core::BOOL, fowner: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressNotify_Impl::OnProgress(this, core::mem::transmute_copy(&dwprogresscurrent), core::mem::transmute_copy(&dwprogressmaximum), core::mem::transmute_copy(&faccurate), core::mem::transmute_copy(&fowner)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnProgress: OnProgress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProgressNotify {}
windows_core::imp::define_interface!(IROTData, IROTData_Vtbl, 0xf29f6bc0_5021_11ce_aa15_00006901293f);
windows_core::imp::interface_hierarchy!(IROTData, windows_core::IUnknown);
impl IROTData {
    pub unsafe fn GetComparisonData(&self, pbdata: &mut [u8], pcbdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetComparisonData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), pcbdata as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetComparisonData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IROTData_Impl: windows_core::IUnknownImpl {
    fn GetComparisonData(&self, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> windows_core::Result<()>;
}
impl IROTData_Vtbl {
    pub const fn new<Identity: IROTData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetComparisonData<Identity: IROTData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IROTData_Impl::GetComparisonData(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbmax), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetComparisonData: GetComparisonData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IROTData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IROTData {}
windows_core::imp::define_interface!(IReleaseMarshalBuffers, IReleaseMarshalBuffers_Vtbl, 0xeb0cb9e8_7996_11d2_872e_0000f8080859);
windows_core::imp::interface_hierarchy!(IReleaseMarshalBuffers, windows_core::IUnknown);
impl IReleaseMarshalBuffers {
    pub unsafe fn ReleaseMarshalBuffer<P2>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMarshalBuffer)(windows_core::Interface::as_raw(self), pmsg as _, dwflags, pchnl.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReleaseMarshalBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IReleaseMarshalBuffers_Impl: windows_core::IUnknownImpl {
    fn ReleaseMarshalBuffer(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IReleaseMarshalBuffers_Vtbl {
    pub const fn new<Identity: IReleaseMarshalBuffers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReleaseMarshalBuffer<Identity: IReleaseMarshalBuffers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReleaseMarshalBuffers_Impl::ReleaseMarshalBuffer(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchnl)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReleaseMarshalBuffer: ReleaseMarshalBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReleaseMarshalBuffers as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReleaseMarshalBuffers {}
windows_core::imp::define_interface!(IRpcChannelBuffer, IRpcChannelBuffer_Vtbl, 0xd5f56b60_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer, windows_core::IUnknown);
impl IRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), pmessage as _, riid).ok() }
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendReceive)(windows_core::Interface::as_raw(self), pmessage as _, pstatus.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pmessage as _).ok() }
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtx)(windows_core::Interface::as_raw(self), pdwdestcontext as _, ppvdestcontext as _).ok() }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SendReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    pub GetDestCtx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRpcChannelBuffer_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> windows_core::Result<()>;
    fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<()>;
}
impl IRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::GetBuffer(this, core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&riid)).into()
            }
        }
        unsafe extern "system" fn SendReceive<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::SendReceive(this, core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::FreeBuffer(this, core::mem::transmute_copy(&pmessage)).into()
            }
        }
        unsafe extern "system" fn GetDestCtx<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::GetDestCtx(this, core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::IsConnected(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SendReceive: SendReceive::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            GetDestCtx: GetDestCtx::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcChannelBuffer {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProtocolVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2_Vtbl {
    pub base__: IRpcChannelBuffer_Vtbl,
    pub GetProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IRpcChannelBuffer2_Impl: IRpcChannelBuffer_Impl {
    fn GetProtocolVersion(&self) -> windows_core::Result<u32>;
}
impl IRpcChannelBuffer2_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProtocolVersion<Identity: IRpcChannelBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcChannelBuffer2_Impl::GetProtocolVersion(this) {
                    Ok(ok__) => {
                        pdwversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IRpcChannelBuffer_Vtbl::new::<Identity, OFFSET>(), GetProtocolVersion: GetProtocolVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcChannelBuffer2 {}
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
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg as _, pulstatus as _).ok() }
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg as _, ulsize, pulstatus as _).ok() }
    }
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pmsg as _).ok() }
    }
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), pmsg, riid, pinterface as _).ok() }
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext as _, ppvdestcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), pmsg, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterAsync<P1>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IAsyncManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterAsync)(windows_core::Interface::as_raw(self), pmsg as _, pasyncmgr.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IRpcChannelBuffer3_Impl: IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<u32>;
    fn RegisterAsync(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: windows_core::Ref<IAsyncManager>) -> windows_core::Result<()>;
}
impl IRpcChannelBuffer3_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Send<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::Send(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Receive<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::Receive(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::Cancel(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        unsafe extern "system" fn GetCallContext<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::GetCallContext(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::GetDestCtxEx(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcChannelBuffer3_Impl::GetState(this, core::mem::transmute_copy(&pmsg)) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterAsync<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::RegisterAsync(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pasyncmgr)).into()
            }
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, OFFSET>(),
            Send: Send::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetCallContext: GetCallContext::<Identity, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            RegisterAsync: RegisterAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer3 as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcChannelBuffer3 {}
windows_core::imp::define_interface!(IRpcHelper, IRpcHelper_Vtbl, 0x00000149_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRpcHelper, windows_core::IUnknown);
impl IRpcHelper {
    pub unsafe fn GetDCOMProtocolVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDCOMProtocolVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIIDFromOBJREF)(windows_core::Interface::as_raw(self), pobjref, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDCOMProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetIIDFromOBJREF: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IRpcHelper_Impl: windows_core::IUnknownImpl {
    fn GetDCOMProtocolVersion(&self) -> windows_core::Result<u32>;
    fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut windows_core::GUID>;
}
impl IRpcHelper_Vtbl {
    pub const fn new<Identity: IRpcHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDCOMProtocolVersion<Identity: IRpcHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcHelper_Impl::GetDCOMProtocolVersion(this) {
                    Ok(ok__) => {
                        pcomversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIIDFromOBJREF<Identity: IRpcHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjref: *const core::ffi::c_void, piid: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcHelper_Impl::GetIIDFromOBJREF(this, core::mem::transmute_copy(&pobjref)) {
                    Ok(ok__) => {
                        piid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDCOMProtocolVersion: GetDCOMProtocolVersion::<Identity, OFFSET>,
            GetIIDFromOBJREF: GetIIDFromOBJREF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcHelper {}
windows_core::imp::define_interface!(IRpcOptions, IRpcOptions_Vtbl, 0x00000144_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRpcOptions, windows_core::IUnknown);
impl IRpcOptions {
    pub unsafe fn Set<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), pprx.param().abi(), dwproperty, dwvalue).ok() }
    }
    pub unsafe fn Query<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), pprx.param().abi(), dwproperty, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, usize) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, *mut usize) -> windows_core::HRESULT,
}
pub trait IRpcOptions_Impl: windows_core::IUnknownImpl {
    fn Set(&self, pprx: windows_core::Ref<windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>;
    fn Query(&self, pprx: windows_core::Ref<windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<usize>;
}
impl IRpcOptions_Vtbl {
    pub const fn new<Identity: IRpcOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IRpcOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcOptions_Impl::Set(this, core::mem::transmute_copy(&pprx), core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IRpcOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcOptions_Impl::Query(this, core::mem::transmute_copy(&pprx), core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        pdwvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, OFFSET>, Query: Query::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcOptions {}
windows_core::imp::define_interface!(IRpcProxyBuffer, IRpcProxyBuffer_Vtbl, 0xd5f56a34_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IRpcProxyBuffer, windows_core::IUnknown);
impl IRpcProxyBuffer {
    pub unsafe fn Connect<P0>(&self, prpcchannelbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRpcChannelBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), prpcchannelbuffer.param().abi()).ok() }
    }
    pub unsafe fn Disconnect(&self) {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IRpcProxyBuffer_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, prpcchannelbuffer: windows_core::Ref<IRpcChannelBuffer>) -> windows_core::Result<()>;
    fn Disconnect(&self);
}
impl IRpcProxyBuffer_Vtbl {
    pub const fn new<Identity: IRpcProxyBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IRpcProxyBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prpcchannelbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcProxyBuffer_Impl::Connect(this, core::mem::transmute_copy(&prpcchannelbuffer)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IRpcProxyBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcProxyBuffer_Impl::Disconnect(this)
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Connect: Connect::<Identity, OFFSET>, Disconnect: Disconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcProxyBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcProxyBuffer {}
windows_core::imp::define_interface!(IRpcStubBuffer, IRpcStubBuffer_Vtbl, 0xd5f56afc_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IRpcStubBuffer, windows_core::IUnknown);
impl IRpcStubBuffer {
    pub unsafe fn Connect<P0>(&self, punkserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), punkserver.param().abi()).ok() }
    }
    pub unsafe fn Disconnect(&self) {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Invoke<P1>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IRpcChannelBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), _prpcmsg as _, _prpcchannelbuffer.param().abi()).ok() }
    }
    pub unsafe fn IsIIDSupported(&self, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer> {
        unsafe { (windows_core::Interface::vtable(self).IsIIDSupported)(windows_core::Interface::as_raw(self), riid) }
    }
    pub unsafe fn CountRefs(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).CountRefs)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DebugServerQueryInterface)(windows_core::Interface::as_raw(self), ppv as _).ok() }
    }
    pub unsafe fn DebugServerRelease(&self, pv: *const core::ffi::c_void) {
        unsafe { (windows_core::Interface::vtable(self).DebugServerRelease)(windows_core::Interface::as_raw(self), pv) }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IRpcStubBuffer_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, punkserver: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Disconnect(&self);
    fn Invoke(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: windows_core::Ref<IRpcChannelBuffer>) -> windows_core::Result<()>;
    fn IsIIDSupported(&self, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer>;
    fn CountRefs(&self) -> u32;
    fn DebugServerQueryInterface(&self, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DebugServerRelease(&self, pv: *const core::ffi::c_void);
}
impl IRpcStubBuffer_Vtbl {
    pub const fn new<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::Connect(this, core::mem::transmute_copy(&punkserver)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::Disconnect(this)
            }
        }
        unsafe extern "system" fn Invoke<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::Invoke(this, core::mem::transmute_copy(&_prpcmsg), core::mem::transmute_copy(&_prpcchannelbuffer)).into()
            }
        }
        unsafe extern "system" fn IsIIDSupported<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::IsIIDSupported(this, core::mem::transmute_copy(&riid))
            }
        }
        unsafe extern "system" fn CountRefs<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::CountRefs(this)
            }
        }
        unsafe extern "system" fn DebugServerQueryInterface<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::DebugServerQueryInterface(this, core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn DebugServerRelease<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::DebugServerRelease(this, core::mem::transmute_copy(&pv))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            IsIIDSupported: IsIIDSupported::<Identity, OFFSET>,
            CountRefs: CountRefs::<Identity, OFFSET>,
            DebugServerQueryInterface: DebugServerQueryInterface::<Identity, OFFSET>,
            DebugServerRelease: DebugServerRelease::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcStubBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcStubBuffer {}
windows_core::imp::define_interface!(IRpcSyntaxNegotiate, IRpcSyntaxNegotiate_Vtbl, 0x58a08519_24c8_4935_b482_3fd823333a4f);
windows_core::imp::interface_hierarchy!(IRpcSyntaxNegotiate, windows_core::IUnknown);
impl IRpcSyntaxNegotiate {
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NegotiateSyntax)(windows_core::Interface::as_raw(self), pmsg as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NegotiateSyntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
}
pub trait IRpcSyntaxNegotiate_Impl: windows_core::IUnknownImpl {
    fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
}
impl IRpcSyntaxNegotiate_Vtbl {
    pub const fn new<Identity: IRpcSyntaxNegotiate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NegotiateSyntax<Identity: IRpcSyntaxNegotiate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcSyntaxNegotiate_Impl::NegotiateSyntax(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NegotiateSyntax: NegotiateSyntax::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcSyntaxNegotiate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcSyntaxNegotiate {}
windows_core::imp::define_interface!(IRunnableObject, IRunnableObject_Vtbl, 0x00000126_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRunnableObject, windows_core::IUnknown);
impl IRunnableObject {
    pub unsafe fn GetRunningClass(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunningClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Run<P0>(&self, pbc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), pbc.param().abi()).ok() }
    }
    pub unsafe fn IsRunning(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn LockRunning(&self, flock: bool, flastunlockcloses: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockRunning)(windows_core::Interface::as_raw(self), flock.into(), flastunlockcloses.into()).ok() }
    }
    pub unsafe fn SetContainedObject(&self, fcontained: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetContainedObject)(windows_core::Interface::as_raw(self), fcontained.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRunningClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub LockRunning: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetContainedObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IRunnableObject_Impl: windows_core::IUnknownImpl {
    fn GetRunningClass(&self) -> windows_core::Result<windows_core::GUID>;
    fn Run(&self, pbc: windows_core::Ref<IBindCtx>) -> windows_core::Result<()>;
    fn IsRunning(&self) -> windows_core::BOOL;
    fn LockRunning(&self, flock: windows_core::BOOL, flastunlockcloses: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetContainedObject(&self, fcontained: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IRunnableObject_Vtbl {
    pub const fn new<Identity: IRunnableObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRunningClass<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunnableObject_Impl::GetRunningClass(this) {
                    Ok(ok__) => {
                        lpclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Run<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::Run(this, core::mem::transmute_copy(&pbc)).into()
            }
        }
        unsafe extern "system" fn IsRunning<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::IsRunning(this)
            }
        }
        unsafe extern "system" fn LockRunning<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: windows_core::BOOL, flastunlockcloses: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::LockRunning(this, core::mem::transmute_copy(&flock), core::mem::transmute_copy(&flastunlockcloses)).into()
            }
        }
        unsafe extern "system" fn SetContainedObject<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcontained: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::SetContainedObject(this, core::mem::transmute_copy(&fcontained)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRunningClass: GetRunningClass::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            LockRunning: LockRunning::<Identity, OFFSET>,
            SetContainedObject: SetContainedObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunnableObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRunnableObject {}
windows_core::imp::define_interface!(IRunningObjectTable, IRunningObjectTable_Vtbl, 0x00000010_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRunningObjectTable, windows_core::IUnknown);
impl IRunningObjectTable {
    pub unsafe fn Register<P1, P2>(&self, grfflags: ROT_FLAGS, punkobject: P1, pmkobjectname: P2) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), grfflags, punkobject.param().abi(), pmkobjectname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Revoke(&self, dwregister: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revoke)(windows_core::Interface::as_raw(self), dwregister).ok() }
    }
    pub unsafe fn IsRunning<P0>(&self, pmkobjectname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi()).ok() }
    }
    pub unsafe fn GetObject<P0>(&self, pmkobjectname: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NoteChangeTime)(windows_core::Interface::as_raw(self), dwregister, pfiletime).ok() }
    }
    pub unsafe fn GetTimeOfLastChange<P0>(&self, pmkobjectname: P0) -> windows_core::Result<super::super::Foundation::FILETIME>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeOfLastChange)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumRunning(&self) -> windows_core::Result<IEnumMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRunning)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IRunningObjectTable_Impl: windows_core::IUnknownImpl {
    fn Register(&self, grfflags: ROT_FLAGS, punkobject: windows_core::Ref<windows_core::IUnknown>, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<u32>;
    fn Revoke(&self, dwregister: u32) -> windows_core::Result<()>;
    fn IsRunning(&self, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<()>;
    fn GetObject(&self, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<windows_core::IUnknown>;
    fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetTimeOfLastChange(&self, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn EnumRunning(&self) -> windows_core::Result<IEnumMoniker>;
}
impl IRunningObjectTable_Vtbl {
    pub const fn new<Identity: IRunningObjectTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: ROT_FLAGS, punkobject: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, pdwregister: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::Register(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&punkobject), core::mem::transmute_copy(&pmkobjectname)) {
                    Ok(ok__) => {
                        pdwregister.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Revoke<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregister: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningObjectTable_Impl::Revoke(this, core::mem::transmute_copy(&dwregister)).into()
            }
        }
        unsafe extern "system" fn IsRunning<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningObjectTable_Impl::IsRunning(this, core::mem::transmute_copy(&pmkobjectname)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::GetObject(this, core::mem::transmute_copy(&pmkobjectname)) {
                    Ok(ok__) => {
                        ppunkobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NoteChangeTime<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningObjectTable_Impl::NoteChangeTime(this, core::mem::transmute_copy(&dwregister), core::mem::transmute_copy(&pfiletime)).into()
            }
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::GetTimeOfLastChange(this, core::mem::transmute_copy(&pmkobjectname)) {
                    Ok(ok__) => {
                        pfiletime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRunning<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenummoniker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::EnumRunning(this) {
                    Ok(ok__) => {
                        ppenummoniker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Revoke: Revoke::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            NoteChangeTime: NoteChangeTime::<Identity, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, OFFSET>,
            EnumRunning: EnumRunning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningObjectTable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRunningObjectTable {}
windows_core::imp::define_interface!(ISequentialStream, ISequentialStream_Vtbl, 0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv as _, cb, pcbread.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pv, cb, pcbwritten.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait ISequentialStream_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT;
    fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT;
}
impl ISequentialStream_Vtbl {
    pub const fn new<Identity: ISequentialStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread))
            }
        }
        unsafe extern "system" fn Write<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Write(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISequentialStream {}
windows_core::imp::define_interface!(IServerSecurity, IServerSecurity_Vtbl, 0x0000013e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IServerSecurity, windows_core::IUnknown);
impl IServerSecurity {
    pub unsafe fn QueryBlanket(&self, pauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: *mut *mut u16, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pprivs: *mut *mut core::ffi::c_void, pcapabilities: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryBlanket)(windows_core::Interface::as_raw(self), pauthnsvc.unwrap_or(core::mem::zeroed()) as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pprivs as _, pcapabilities.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ImpersonateClient(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImpersonateClient)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RevertToSelf(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevertToSelf)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsImpersonating(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsImpersonating)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut u16, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ImpersonateClient: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsImpersonating: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait IServerSecurity_Impl: windows_core::IUnknownImpl {
    fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut u32) -> windows_core::Result<()>;
    fn ImpersonateClient(&self) -> windows_core::Result<()>;
    fn RevertToSelf(&self) -> windows_core::Result<()>;
    fn IsImpersonating(&self) -> windows_core::BOOL;
}
impl IServerSecurity_Vtbl {
    pub const fn new<Identity: IServerSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryBlanket<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::QueryBlanket(this, core::mem::transmute_copy(&pauthnsvc), core::mem::transmute_copy(&pauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&pauthnlevel), core::mem::transmute_copy(&pimplevel), core::mem::transmute_copy(&pprivs), core::mem::transmute_copy(&pcapabilities)).into()
            }
        }
        unsafe extern "system" fn ImpersonateClient<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::ImpersonateClient(this).into()
            }
        }
        unsafe extern "system" fn RevertToSelf<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::RevertToSelf(this).into()
            }
        }
        unsafe extern "system" fn IsImpersonating<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::IsImpersonating(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, OFFSET>,
            ImpersonateClient: ImpersonateClient::<Identity, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, OFFSET>,
            IsImpersonating: IsImpersonating::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServerSecurity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServerSecurity {}
windows_core::imp::define_interface!(IServiceProvider, IServiceProvider_Vtbl, 0x6d5140c1_7436_11ce_8034_00aa006009fa);
windows_core::imp::interface_hierarchy!(IServiceProvider, windows_core::IUnknown);
impl IServiceProvider {
    pub unsafe fn QueryService<T>(&self, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).QueryService)(windows_core::Interface::as_raw(self), guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IServiceProvider_Impl: windows_core::IUnknownImpl {
    fn QueryService(&self, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IServiceProvider_Vtbl {
    pub const fn new<Identity: IServiceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryService<Identity: IServiceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceProvider_Impl::QueryService(this, core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryService: QueryService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceProvider {}
windows_core::imp::define_interface!(IStdMarshalInfo, IStdMarshalInfo_Vtbl, 0x00000018_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IStdMarshalInfo, windows_core::IUnknown);
impl IStdMarshalInfo {
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassForHandler)(windows_core::Interface::as_raw(self), dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassForHandler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IStdMarshalInfo_Impl: windows_core::IUnknownImpl {
    fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<windows_core::GUID>;
}
impl IStdMarshalInfo_Vtbl {
    pub const fn new<Identity: IStdMarshalInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassForHandler<Identity: IStdMarshalInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStdMarshalInfo_Impl::GetClassForHandler(this, core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext)) {
                    Ok(ok__) => {
                        pclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassForHandler: GetClassForHandler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStdMarshalInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStdMarshalInfo {}
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
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), dlibmove, dworigin, plibnewposition.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), libnewsize).ok() }
    }
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: Option<*mut u64>, pcbwritten: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), pstm.param().abi(), cb, pcbread.unwrap_or(core::mem::zeroed()) as _, pcbwritten.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Commit(&self, grfcommitflags: STGC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags.0 as _).ok() }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: LOCKTYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype.0 as _).ok() }
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok() }
    }
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: STATFLAG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg as _, grfstatflag.0 as _).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IStream_Impl: ISequentialStream_Impl {
    fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()>;
    fn CopyTo(&self, pstm: windows_core::Ref<IStream>, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: &STGC) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: &LOCKTYPE) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: &STATFLAG) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IStream>;
}
impl IStream_Vtbl {
    pub const fn new<Identity: IStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Seek<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Seek(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, libnewsize: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::SetSize(this, core::mem::transmute_copy(&libnewsize)).into()
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::CopyTo(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Commit(this, core::mem::transmute(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn LockRegion<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::LockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn UnlockRegion<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::UnlockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute(&grfstatflag)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStream_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISequentialStream_Vtbl::new::<Identity, OFFSET>(),
            Seek: Seek::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            LockRegion: LockRegion::<Identity, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStream as windows_core::Interface>::IID || iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStream {}
windows_core::imp::define_interface!(ISupportAllowLowerTrustActivation, ISupportAllowLowerTrustActivation_Vtbl, 0xe9956ef2_3828_4b4b_8fa9_7db61dee4954);
windows_core::imp::interface_hierarchy!(ISupportAllowLowerTrustActivation, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportAllowLowerTrustActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportAllowLowerTrustActivation_Impl: windows_core::IUnknownImpl {}
impl ISupportAllowLowerTrustActivation_Vtbl {
    pub const fn new<Identity: ISupportAllowLowerTrustActivation_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportAllowLowerTrustActivation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportAllowLowerTrustActivation {}
windows_core::imp::define_interface!(ISupportErrorInfo, ISupportErrorInfo_Vtbl, 0xdf0b3d60_548f_101b_8e65_08002b2bd119);
windows_core::imp::interface_hierarchy!(ISupportErrorInfo, windows_core::IUnknown);
impl ISupportErrorInfo {
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InterfaceSupportsErrorInfo)(windows_core::Interface::as_raw(self), riid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InterfaceSupportsErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ISupportErrorInfo_Impl: windows_core::IUnknownImpl {
    fn InterfaceSupportsErrorInfo(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ISupportErrorInfo_Vtbl {
    pub const fn new<Identity: ISupportErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Identity: ISupportErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISupportErrorInfo_Impl::InterfaceSupportsErrorInfo(this, core::mem::transmute_copy(&riid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InterfaceSupportsErrorInfo: InterfaceSupportsErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportErrorInfo {}
windows_core::imp::define_interface!(ISurrogate, ISurrogate_Vtbl, 0x00000022_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISurrogate, windows_core::IUnknown);
impl ISurrogate {
    pub unsafe fn LoadDllServer(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadDllServer)(windows_core::Interface::as_raw(self), clsid).ok() }
    }
    pub unsafe fn FreeSurrogate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeSurrogate)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadDllServer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub FreeSurrogate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISurrogate_Impl: windows_core::IUnknownImpl {
    fn LoadDllServer(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn FreeSurrogate(&self) -> windows_core::Result<()>;
}
impl ISurrogate_Vtbl {
    pub const fn new<Identity: ISurrogate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadDllServer<Identity: ISurrogate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogate_Impl::LoadDllServer(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn FreeSurrogate<Identity: ISurrogate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogate_Impl::FreeSurrogate(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadDllServer: LoadDllServer::<Identity, OFFSET>,
            FreeSurrogate: FreeSurrogate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurrogate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISurrogate {}
windows_core::imp::define_interface!(ISurrogateService, ISurrogateService_Vtbl, 0x000001d4_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISurrogateService, windows_core::IUnknown);
impl ISurrogateService {
    pub unsafe fn Init<P1>(&self, rguidprocessid: *const windows_core::GUID, pprocesslock: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<IProcessLock>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), rguidprocessid, pprocesslock.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplicationLaunch)(windows_core::Interface::as_raw(self), rguidapplid, apptype).ok() }
    }
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplicationFree)(windows_core::Interface::as_raw(self), rguidapplid).ok() }
    }
    pub unsafe fn CatalogRefresh(&self, ulreserved: Option<u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CatalogRefresh)(windows_core::Interface::as_raw(self), ulreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessShutdown)(windows_core::Interface::as_raw(self), shutdowntype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ApplicationLaunch: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, ApplicationType) -> windows_core::HRESULT,
    pub ApplicationFree: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub CatalogRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ProcessShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, ShutdownType) -> windows_core::HRESULT,
}
pub trait ISurrogateService_Impl: windows_core::IUnknownImpl {
    fn Init(&self, rguidprocessid: *const windows_core::GUID, pprocesslock: windows_core::Ref<IProcessLock>) -> windows_core::Result<windows_core::BOOL>;
    fn ApplicationLaunch(&self, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::Result<()>;
    fn ApplicationFree(&self, rguidapplid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn CatalogRefresh(&self, ulreserved: u32) -> windows_core::Result<()>;
    fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> windows_core::Result<()>;
}
impl ISurrogateService_Vtbl {
    pub const fn new<Identity: ISurrogateService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidprocessid: *const windows_core::GUID, pprocesslock: *mut core::ffi::c_void, pfapplicationaware: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISurrogateService_Impl::Init(this, core::mem::transmute_copy(&rguidprocessid), core::mem::transmute_copy(&pprocesslock)) {
                    Ok(ok__) => {
                        pfapplicationaware.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationLaunch<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::ApplicationLaunch(this, core::mem::transmute_copy(&rguidapplid), core::mem::transmute_copy(&apptype)).into()
            }
        }
        unsafe extern "system" fn ApplicationFree<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidapplid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::ApplicationFree(this, core::mem::transmute_copy(&rguidapplid)).into()
            }
        }
        unsafe extern "system" fn CatalogRefresh<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::CatalogRefresh(this, core::mem::transmute_copy(&ulreserved)).into()
            }
        }
        unsafe extern "system" fn ProcessShutdown<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shutdowntype: ShutdownType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::ProcessShutdown(this, core::mem::transmute_copy(&shutdowntype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            ApplicationLaunch: ApplicationLaunch::<Identity, OFFSET>,
            ApplicationFree: ApplicationFree::<Identity, OFFSET>,
            CatalogRefresh: CatalogRefresh::<Identity, OFFSET>,
            ProcessShutdown: ProcessShutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurrogateService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISurrogateService {}
windows_core::imp::define_interface!(ISynchronize, ISynchronize_Vtbl, 0x00000030_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISynchronize, windows_core::IUnknown);
impl ISynchronize {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwflags, dwmilliseconds).ok() }
    }
    pub unsafe fn Signal(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Signal)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronize_Impl: windows_core::IUnknownImpl {
    fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn Signal(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl ISynchronize_Vtbl {
    pub const fn new<Identity: ISynchronize_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Wait<Identity: ISynchronize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronize_Impl::Wait(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn Signal<Identity: ISynchronize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronize_Impl::Signal(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ISynchronize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronize_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, OFFSET>,
            Signal: Signal::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronize as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronize {}
windows_core::imp::define_interface!(ISynchronizeContainer, ISynchronizeContainer_Vtbl, 0x00000033_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISynchronizeContainer, windows_core::IUnknown);
impl ISynchronizeContainer {
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISynchronize>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSynchronize)(windows_core::Interface::as_raw(self), psync.param().abi()).ok() }
    }
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> windows_core::Result<ISynchronize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), dwflags, dwtimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronizeContainer_Impl: windows_core::IUnknownImpl {
    fn AddSynchronize(&self, psync: windows_core::Ref<ISynchronize>) -> windows_core::Result<()>;
    fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> windows_core::Result<ISynchronize>;
}
impl ISynchronizeContainer_Vtbl {
    pub const fn new<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddSynchronize<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psync: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeContainer_Impl::AddSynchronize(this, core::mem::transmute_copy(&psync)).into()
            }
        }
        unsafe extern "system" fn WaitMultiple<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISynchronizeContainer_Impl::WaitMultiple(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwtimeout)) {
                    Ok(ok__) => {
                        ppsync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddSynchronize: AddSynchronize::<Identity, OFFSET>,
            WaitMultiple: WaitMultiple::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeContainer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronizeContainer {}
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
        unsafe { (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), ph).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_Vtbl {
    pub base__: ISynchronizeHandle_Vtbl,
    pub SetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
pub trait ISynchronizeEvent_Impl: ISynchronizeHandle_Impl {
    fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
impl ISynchronizeEvent_Vtbl {
    pub const fn new<Identity: ISynchronizeEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEventHandle<Identity: ISynchronizeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeEvent_Impl::SetEventHandle(this, core::mem::transmute_copy(&ph)).into()
            }
        }
        Self { base__: ISynchronizeHandle_Vtbl::new::<Identity, OFFSET>(), SetEventHandle: SetEventHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeEvent as windows_core::Interface>::IID || iid == &<ISynchronizeHandle as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronizeEvent {}
windows_core::imp::define_interface!(ISynchronizeHandle, ISynchronizeHandle_Vtbl, 0x00000031_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISynchronizeHandle, windows_core::IUnknown);
impl ISynchronizeHandle {
    pub unsafe fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
pub trait ISynchronizeHandle_Impl: windows_core::IUnknownImpl {
    fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
impl ISynchronizeHandle_Vtbl {
    pub const fn new<Identity: ISynchronizeHandle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHandle<Identity: ISynchronizeHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISynchronizeHandle_Impl::GetHandle(this) {
                    Ok(ok__) => {
                        ph.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeHandle as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronizeHandle {}
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
        unsafe { (windows_core::Interface::vtable(self).ReleaseMutex)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutex_Vtbl {
    pub base__: ISynchronize_Vtbl,
    pub ReleaseMutex: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronizeMutex_Impl: ISynchronize_Impl {
    fn ReleaseMutex(&self) -> windows_core::Result<()>;
}
impl ISynchronizeMutex_Vtbl {
    pub const fn new<Identity: ISynchronizeMutex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReleaseMutex<Identity: ISynchronizeMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeMutex_Impl::ReleaseMutex(this).into()
            }
        }
        Self { base__: ISynchronize_Vtbl::new::<Identity, OFFSET>(), ReleaseMutex: ReleaseMutex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeMutex as windows_core::Interface>::IID || iid == &<ISynchronize as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronizeMutex {}
windows_core::imp::define_interface!(ITimeAndNoticeControl, ITimeAndNoticeControl_Vtbl, 0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
windows_core::imp::interface_hierarchy!(ITimeAndNoticeControl, windows_core::IUnknown);
impl ITimeAndNoticeControl {
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuppressChanges)(windows_core::Interface::as_raw(self), res1, res2).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SuppressChanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait ITimeAndNoticeControl_Impl: windows_core::IUnknownImpl {
    fn SuppressChanges(&self, res1: u32, res2: u32) -> windows_core::Result<()>;
}
impl ITimeAndNoticeControl_Vtbl {
    pub const fn new<Identity: ITimeAndNoticeControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SuppressChanges<Identity: ITimeAndNoticeControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, res1: u32, res2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimeAndNoticeControl_Impl::SuppressChanges(this, core::mem::transmute_copy(&res1), core::mem::transmute_copy(&res2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SuppressChanges: SuppressChanges::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimeAndNoticeControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITimeAndNoticeControl {}
windows_core::imp::define_interface!(ITypeComp, ITypeComp_Vtbl, 0x00020403_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeComp, windows_core::IUnknown);
impl ITypeComp {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Bind<P0>(&self, szname: P0, lhashval: u32, wflags: u16, pptinfo: *mut Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), szname.param().abi(), lhashval, wflags, core::mem::transmute(pptinfo), pdesckind as _, core::mem::transmute(pbindptr)).ok() }
    }
    pub unsafe fn BindType<P0>(&self, szname: P0, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, pptcomp: *mut Option<ITypeComp>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindType)(windows_core::Interface::as_raw(self), szname.param().abi(), lhashval, core::mem::transmute(pptinfo), core::mem::transmute(pptcomp)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeComp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u16, *mut *mut core::ffi::c_void, *mut DESCKIND, *mut BINDPTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Bind: usize,
    pub BindType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeComp_Impl: windows_core::IUnknownImpl {
    fn Bind(&self, szname: &windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: windows_core::OutRef<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::Result<()>;
    fn BindType(&self, szname: &windows_core::PCWSTR, lhashval: u32, pptinfo: windows_core::OutRef<ITypeInfo>, pptcomp: windows_core::OutRef<ITypeComp>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITypeComp_Vtbl {
    pub const fn new<Identity: ITypeComp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Bind<Identity: ITypeComp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut *mut core::ffi::c_void, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeComp_Impl::Bind(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pdesckind), core::mem::transmute_copy(&pbindptr)).into()
            }
        }
        unsafe extern "system" fn BindType<Identity: ITypeComp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, lhashval: u32, pptinfo: *mut *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeComp_Impl::BindType(this, core::mem::transmute(&szname), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&pptcomp)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, OFFSET>, BindType: BindType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeComp as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITypeComp {}
windows_core::imp::define_interface!(ITypeInfo, ITypeInfo_Vtbl, 0x00020401_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeInfo, windows_core::IUnknown);
impl ITypeInfo {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> windows_core::Result<*mut FUNCDESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> windows_core::Result<*mut VARDESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: &mut [windows_core::BSTR], pcnames: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), memid, core::mem::transmute(rgbstrnames.as_ptr()), rgbstrnames.len().try_into().unwrap(), pcnames as _).ok() }
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRefTypeOfImplType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> windows_core::Result<IMPLTYPEFLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImplTypeFlags)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), rgsznames, cnames, pmemid as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Invoke(&self, pvinstance: *const core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pvinstance, memid, wflags, pdispparams as _, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo), puargerr as _).ok() }
    }
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: Option<*mut windows_core::BSTR>, pbstrdocstring: Option<*mut windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), memid, pbstrname.unwrap_or(core::mem::zeroed()) as _, pbstrdocstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpcontext as _, pbstrhelpfile.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: Option<*mut windows_core::BSTR>, pbstrname: Option<*mut windows_core::BSTR>, pwordinal: *mut u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDllEntry)(windows_core::Interface::as_raw(self), memid, invkind, pbstrdllname.unwrap_or(core::mem::zeroed()) as _, pbstrname.unwrap_or(core::mem::zeroed()) as _, pwordinal as _).ok() }
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRefTypeInfo)(windows_core::Interface::as_raw(self), hreftype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddressOfMember)(windows_core::Interface::as_raw(self), memid, invkind, ppv as _).ok() }
    }
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetMops(&self, memid: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMops)(windows_core::Interface::as_raw(self), memid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut Option<ITypeLib>, pindex: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetContainingTypeLib)(windows_core::Interface::as_raw(self), core::mem::transmute(pptlib), pindex as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseTypeAttr)(windows_core::Interface::as_raw(self), ptypeattr) }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFuncDesc)(windows_core::Interface::as_raw(self), pfuncdesc) }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseVarDesc)(windows_core::Interface::as_raw(self), pvardesc) }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRefTypeOfImplType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetImplTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut IMPLTYPEFLAGS) -> windows_core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, i32, DISPATCH_FLAGS, *mut DISPPARAMS, *mut super::Variant::VARIANT, *mut EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Invoke: usize,
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDllEntry: unsafe extern "system" fn(*mut core::ffi::c_void, i32, INVOKEKIND, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetRefTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddressOfMember: unsafe extern "system" fn(*mut core::ffi::c_void, i32, INVOKEKIND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMops: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeInfo_Impl: windows_core::IUnknownImpl {
    fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetFuncDesc(&self, index: u32) -> windows_core::Result<*mut FUNCDESC>;
    fn GetVarDesc(&self, index: u32) -> windows_core::Result<*mut VARDESC>;
    fn GetNames(&self, memid: i32, rgbstrnames: *mut windows_core::BSTR, cmaxnames: u32, pcnames: *mut u32) -> windows_core::Result<()>;
    fn GetRefTypeOfImplType(&self, index: u32) -> windows_core::Result<u32>;
    fn GetImplTypeFlags(&self, index: u32) -> windows_core::Result<IMPLTYPEFLAGS>;
    fn GetIDsOfNames(&self, rgsznames: *const windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> windows_core::Result<()>;
    fn Invoke(&self, pvinstance: *const core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
    fn GetDocumentation(&self, memid: i32, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut windows_core::BSTR, pbstrname: *mut windows_core::BSTR, pwordinal: *mut u16) -> windows_core::Result<()>;
    fn GetRefTypeInfo(&self, hreftype: u32) -> windows_core::Result<ITypeInfo>;
    fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateInstance(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetMops(&self, memid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetContainingTypeLib(&self, pptlib: windows_core::OutRef<ITypeLib>, pindex: *mut u32) -> windows_core::Result<()>;
    fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR);
    fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC);
    fn ReleaseVarDesc(&self, pvardesc: *const VARDESC);
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITypeInfo_Vtbl {
    pub const fn new<Identity: ITypeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeAttr<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetTypeAttr(this) {
                    Ok(ok__) => {
                        pptypeattr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetTypeComp(this) {
                    Ok(ok__) => {
                        pptcomp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFuncDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetFuncDesc(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppfuncdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVarDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetVarDesc(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppvardesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNames<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, rgbstrnames: *mut *mut core::ffi::c_void, cmaxnames: u32, pcnames: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetNames(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&rgbstrnames), core::mem::transmute_copy(&cmaxnames), core::mem::transmute_copy(&pcnames)).into()
            }
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, preftype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetRefTypeOfImplType(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        preftype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImplTypeFlags<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pimpltypeflags: *mut IMPLTYPEFLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetImplTypeFlags(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pimpltypeflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgsznames: *const windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetIDsOfNames(this, core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&pmemid)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvinstance: *const core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::Invoke(this, core::mem::transmute_copy(&pvinstance), core::mem::transmute_copy(&memid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
            }
        }
        unsafe extern "system" fn GetDocumentation<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, pbstrname: *mut *mut core::ffi::c_void, pbstrdocstring: *mut *mut core::ffi::c_void, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetDocumentation(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
            }
        }
        unsafe extern "system" fn GetDllEntry<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void, pwordinal: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetDllEntry(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind), core::mem::transmute_copy(&pbstrdllname), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pwordinal)).into()
            }
        }
        unsafe extern "system" fn GetRefTypeInfo<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreftype: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetRefTypeInfo(this, core::mem::transmute_copy(&hreftype)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddressOfMember<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::AddressOfMember(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::CreateInstance(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn GetMops<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, pbstrmops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo_Impl::GetMops(this, core::mem::transmute_copy(&memid)) {
                    Ok(ok__) => {
                        pbstrmops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContainingTypeLib<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlib: *mut *mut core::ffi::c_void, pindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::GetContainingTypeLib(this, core::mem::transmute_copy(&pptlib), core::mem::transmute_copy(&pindex)).into()
            }
        }
        unsafe extern "system" fn ReleaseTypeAttr<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeattr: *const TYPEATTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::ReleaseTypeAttr(this, core::mem::transmute_copy(&ptypeattr))
            }
        }
        unsafe extern "system" fn ReleaseFuncDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncdesc: *const FUNCDESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::ReleaseFuncDesc(this, core::mem::transmute_copy(&pfuncdesc))
            }
        }
        unsafe extern "system" fn ReleaseVarDesc<Identity: ITypeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardesc: *const VARDESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo_Impl::ReleaseVarDesc(this, core::mem::transmute_copy(&pvardesc))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeAttr: GetTypeAttr::<Identity, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, OFFSET>,
            GetFuncDesc: GetFuncDesc::<Identity, OFFSET>,
            GetVarDesc: GetVarDesc::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            GetRefTypeOfImplType: GetRefTypeOfImplType::<Identity, OFFSET>,
            GetImplTypeFlags: GetImplTypeFlags::<Identity, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, OFFSET>,
            GetDllEntry: GetDllEntry::<Identity, OFFSET>,
            GetRefTypeInfo: GetRefTypeInfo::<Identity, OFFSET>,
            AddressOfMember: AddressOfMember::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            GetMops: GetMops::<Identity, OFFSET>,
            GetContainingTypeLib: GetContainingTypeLib::<Identity, OFFSET>,
            ReleaseTypeAttr: ReleaseTypeAttr::<Identity, OFFSET>,
            ReleaseFuncDesc: ReleaseFuncDesc::<Identity, OFFSET>,
            ReleaseVarDesc: ReleaseVarDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITypeInfo {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeKind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncIndexOfMemId)(windows_core::Interface::as_raw(self), memid, invkind, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarIndexOfMemId)(windows_core::Interface::as_raw(self), memid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFuncCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImplTypeCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: Option<*mut windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), memid, lcid, pbstrhelpstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpstringcontext as _, pbstrhelpstringdll.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllCustData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllFuncCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllVarCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllImplTypeCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2_Vtbl {
    pub base__: ITypeInfo_Vtbl,
    pub GetTypeKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TYPEKIND) -> windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFuncIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, i32, INVOKEKIND, *mut u32) -> windows_core::HRESULT,
    pub GetVarIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFuncCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetParamCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetVarCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetImplTypeCustData: usize,
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAllCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAllFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAllFuncCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAllParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAllParamCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAllVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAllVarCustData: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAllImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAllImplTypeCustData: usize,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeInfo2_Impl: ITypeInfo_Impl {
    fn GetTypeKind(&self) -> windows_core::Result<TYPEKIND>;
    fn GetTypeFlags(&self) -> windows_core::Result<u32>;
    fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> windows_core::Result<u32>;
    fn GetVarIndexOfMemId(&self, memid: i32) -> windows_core::Result<u32>;
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetFuncCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetVarCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
    fn GetAllFuncCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllVarCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
    fn GetAllImplTypeCustData(&self, index: u32) -> windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITypeInfo2_Vtbl {
    pub const fn new<Identity: ITypeInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeKind<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypekind: *mut TYPEKIND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetTypeKind(this) {
                    Ok(ok__) => {
                        ptypekind.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeFlags<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetTypeFlags(this) {
                    Ok(ok__) => {
                        ptypeflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFuncIndexOfMemId<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetFuncIndexOfMemId(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&invkind)) {
                    Ok(ok__) => {
                        pfuncindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVarIndexOfMemId<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetVarIndexOfMemId(this, core::mem::transmute_copy(&memid)) {
                    Ok(ok__) => {
                        pvarindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetCustData(this, core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFuncCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetFuncCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParamCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVarCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetVarCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImplTypeCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, guid: *const windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetImplTypeCustData(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut *mut core::ffi::c_void, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeInfo2_Impl::GetDocumentation2(this, core::mem::transmute_copy(&memid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpstring), core::mem::transmute_copy(&pdwhelpstringcontext), core::mem::transmute_copy(&pbstrhelpstringdll)).into()
            }
        }
        unsafe extern "system" fn GetAllCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllCustData(this) {
                    Ok(ok__) => {
                        pcustdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllFuncCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllFuncCustData(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcustdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllParamCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllParamCustData(this, core::mem::transmute_copy(&indexfunc), core::mem::transmute_copy(&indexparam)) {
                    Ok(ok__) => {
                        pcustdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllVarCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllVarCustData(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcustdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllImplTypeCustData<Identity: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeInfo2_Impl::GetAllImplTypeCustData(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pcustdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITypeInfo_Vtbl::new::<Identity, OFFSET>(),
            GetTypeKind: GetTypeKind::<Identity, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, OFFSET>,
            GetFuncIndexOfMemId: GetFuncIndexOfMemId::<Identity, OFFSET>,
            GetVarIndexOfMemId: GetVarIndexOfMemId::<Identity, OFFSET>,
            GetCustData: GetCustData::<Identity, OFFSET>,
            GetFuncCustData: GetFuncCustData::<Identity, OFFSET>,
            GetParamCustData: GetParamCustData::<Identity, OFFSET>,
            GetVarCustData: GetVarCustData::<Identity, OFFSET>,
            GetImplTypeCustData: GetImplTypeCustData::<Identity, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, OFFSET>,
            GetAllFuncCustData: GetAllFuncCustData::<Identity, OFFSET>,
            GetAllParamCustData: GetAllParamCustData::<Identity, OFFSET>,
            GetAllVarCustData: GetAllVarCustData::<Identity, OFFSET>,
            GetAllImplTypeCustData: GetAllImplTypeCustData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeInfo2 as windows_core::Interface>::IID || iid == &<ITypeInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITypeInfo2 {}
windows_core::imp::define_interface!(ITypeLib, ITypeLib_Vtbl, 0x00020402_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ITypeLib, windows_core::IUnknown);
impl ITypeLib {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> windows_core::Result<TYPEKIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoOfGuid)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLibAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: Option<*mut windows_core::BSTR>, pbstrdocstring: Option<*mut windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), index, pbstrname.unwrap_or(core::mem::zeroed()) as _, pbstrdocstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpcontext as _, pbstrhelpfile.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn IsName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsName)(windows_core::Interface::as_raw(self), core::mem::transmute(sznamebuf), lhashval, pfname as _).ok() }
    }
    pub unsafe fn FindName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FindName)(windows_core::Interface::as_raw(self), core::mem::transmute(sznamebuf), lhashval, core::mem::transmute(pptinfo), rgmemid as _, pcfound as _).ok() }
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseTLibAttr)(windows_core::Interface::as_raw(self), ptlibattr) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypeInfoType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TYPEKIND) -> windows_core::HRESULT,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TLIBATTR) -> windows_core::HRESULT,
    pub GetTypeComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub FindName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut *mut core::ffi::c_void, *mut i32, *mut u16) -> windows_core::HRESULT,
    pub ReleaseTLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TLIBATTR),
}
pub trait ITypeLib_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> u32;
    fn GetTypeInfo(&self, index: u32) -> windows_core::Result<ITypeInfo>;
    fn GetTypeInfoType(&self, index: u32) -> windows_core::Result<TYPEKIND>;
    fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo>;
    fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR>;
    fn GetTypeComp(&self) -> windows_core::Result<ITypeComp>;
    fn GetDocumentation(&self, index: i32, pbstrname: *mut windows_core::BSTR, pbstrdocstring: *mut windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn FindName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: windows_core::OutRef<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> windows_core::Result<()>;
    fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR);
}
impl ITypeLib_Vtbl {
    pub const fn new<Identity: ITypeLib_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::GetTypeInfoCount(this)
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeInfo(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeInfoType<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeInfoType(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ptkind.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeInfoOfGuid<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeInfoOfGuid(this, core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pptinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLibAttr<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetLibAttr(this) {
                    Ok(ok__) => {
                        pptlibattr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib_Impl::GetTypeComp(this) {
                    Ok(ok__) => {
                        pptcomp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentation<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pbstrname: *mut *mut core::ffi::c_void, pbstrdocstring: *mut *mut core::ffi::c_void, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::GetDocumentation(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pbstrdocstring), core::mem::transmute_copy(&pdwhelpcontext), core::mem::transmute_copy(&pbstrhelpfile)).into()
            }
        }
        unsafe extern "system" fn IsName<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::IsName(this, core::mem::transmute_copy(&sznamebuf), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pfname)).into()
            }
        }
        unsafe extern "system" fn FindName<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: *mut *mut core::ffi::c_void, rgmemid: *mut i32, pcfound: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::FindName(this, core::mem::transmute_copy(&sznamebuf), core::mem::transmute_copy(&lhashval), core::mem::transmute_copy(&pptinfo), core::mem::transmute_copy(&rgmemid), core::mem::transmute_copy(&pcfound)).into()
            }
        }
        unsafe extern "system" fn ReleaseTLibAttr<Identity: ITypeLib_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptlibattr: *const TLIBATTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib_Impl::ReleaseTLibAttr(this, core::mem::transmute_copy(&ptlibattr))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            GetTypeInfoType: GetTypeInfoType::<Identity, OFFSET>,
            GetTypeInfoOfGuid: GetTypeInfoOfGuid::<Identity, OFFSET>,
            GetLibAttr: GetLibAttr::<Identity, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, OFFSET>,
            IsName: IsName::<Identity, OFFSET>,
            FindName: FindName::<Identity, OFFSET>,
            ReleaseTLibAttr: ReleaseTLibAttr::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLib as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeLib {}
windows_core::imp::define_interface!(ITypeLib2, ITypeLib2_Vtbl, 0x00020411_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeLib2 {
    type Target = ITypeLib;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeLib2, windows_core::IUnknown, ITypeLib);
impl ITypeLib2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLibStatistics)(windows_core::Interface::as_raw(self), pcuniquenames as _, pcchuniquenames as _).ok() }
    }
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: Option<*mut windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), index, lcid, pbstrhelpstring.unwrap_or(core::mem::zeroed()) as _, pdwhelpstringcontext as _, pbstrhelpstringdll.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllCustData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2_Vtbl {
    pub base__: ITypeLib_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCustData: usize,
    pub GetLibStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetAllCustData: usize,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeLib2_Impl: ITypeLib_Impl {
    fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::Result<()>;
    fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: *mut windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITypeLib2_Vtbl {
    pub const fn new<Identity: ITypeLib2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCustData<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib2_Impl::GetCustData(this, core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        pvarval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLibStatistics<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib2_Impl::GetLibStatistics(this, core::mem::transmute_copy(&pcuniquenames), core::mem::transmute_copy(&pcchuniquenames)).into()
            }
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut *mut core::ffi::c_void, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeLib2_Impl::GetDocumentation2(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpstring), core::mem::transmute_copy(&pdwhelpstringcontext), core::mem::transmute_copy(&pbstrhelpstringdll)).into()
            }
        }
        unsafe extern "system" fn GetAllCustData<Identity: ITypeLib2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcustdata: *mut CUSTDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLib2_Impl::GetAllCustData(this) {
                    Ok(ok__) => {
                        pcustdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITypeLib_Vtbl::new::<Identity, OFFSET>(),
            GetCustData: GetCustData::<Identity, OFFSET>,
            GetLibStatistics: GetLibStatistics::<Identity, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLib2 as windows_core::Interface>::IID || iid == &<ITypeLib as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITypeLib2 {}
windows_core::imp::define_interface!(ITypeLibRegistration, ITypeLibRegistration_Vtbl, 0x76a3e735_02df_4a12_98eb_043ad3600af3);
windows_core::imp::interface_hierarchy!(ITypeLibRegistration, windows_core::IUnknown);
impl ITypeLibRegistration {
    pub unsafe fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLcid(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLcid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetWin32Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWin32Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetWin64Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWin64Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHelpDir(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpDir)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLcid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWin32Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWin64Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetHelpDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITypeLibRegistration_Impl: windows_core::IUnknownImpl {
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcid(&self) -> windows_core::Result<u32>;
    fn GetWin32Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetWin64Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn GetHelpDir(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITypeLibRegistration_Vtbl {
    pub const fn new<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGuid<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetGuid(this) {
                    Ok(ok__) => {
                        pguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        pversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLcid<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetLcid(this) {
                    Ok(ok__) => {
                        plcid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWin32Path<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwin32path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetWin32Path(this) {
                    Ok(ok__) => {
                        pwin32path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWin64Path<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwin64path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetWin64Path(this) {
                    Ok(ok__) => {
                        pwin64path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlags<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpDir<Identity: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phelpdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistration_Impl::GetHelpDir(this) {
                    Ok(ok__) => {
                        phelpdir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetLcid: GetLcid::<Identity, OFFSET>,
            GetWin32Path: GetWin32Path::<Identity, OFFSET>,
            GetWin64Path: GetWin64Path::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetHelpDir: GetHelpDir::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLibRegistration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeLibRegistration {}
windows_core::imp::define_interface!(ITypeLibRegistrationReader, ITypeLibRegistrationReader_Vtbl, 0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
windows_core::imp::interface_hierarchy!(ITypeLibRegistrationReader, windows_core::IUnknown);
impl ITypeLibRegistrationReader {
    pub unsafe fn EnumTypeLibRegistrations(&self) -> windows_core::Result<IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumTypeLibRegistrations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumTypeLibRegistrations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITypeLibRegistrationReader_Impl: windows_core::IUnknownImpl {
    fn EnumTypeLibRegistrations(&self) -> windows_core::Result<IEnumUnknown>;
}
impl ITypeLibRegistrationReader_Vtbl {
    pub const fn new<Identity: ITypeLibRegistrationReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumTypeLibRegistrations<Identity: ITypeLibRegistrationReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeLibRegistrationReader_Impl::EnumTypeLibRegistrations(this) {
                    Ok(ok__) => {
                        ppenumunknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumTypeLibRegistrations: EnumTypeLibRegistrations::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeLibRegistrationReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeLibRegistrationReader {}
windows_core::imp::define_interface!(IUri, IUri_Vtbl, 0xa39ee748_6a27_4817_a6f2_13914bef5890);
windows_core::imp::interface_hierarchy!(IUri, windows_core::IUnknown);
impl IUri {
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut windows_core::BSTR, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyBSTR)(windows_core::Interface::as_raw(self), uriprop, core::mem::transmute(pbstrproperty), dwflags).ok() }
    }
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyLength)(windows_core::Interface::as_raw(self), uriprop, pcchproperty as _, dwflags).ok() }
    }
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyDWORD)(windows_core::Interface::as_raw(self), uriprop, pdwproperty as _, dwflags).ok() }
    }
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasProperty)(windows_core::Interface::as_raw(self), uriprop, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAbsoluteUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAbsoluteUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAuthority(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAuthority)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDisplayUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFragment(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFragment)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHost(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPassword(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPathAndQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPathAndQuery)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRawUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRawUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSchemeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSchemeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUserInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHostType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHostType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPort(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScheme(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScheme)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetZone(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetZone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<IUri>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), puri.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyBSTR: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPropertyLength: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut u32, u32) -> windows_core::HRESULT,
    pub GetPropertyDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut u32, u32) -> windows_core::HRESULT,
    pub HasProperty: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAbsoluteUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAuthority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPathAndQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRawUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHostType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUri_Impl: windows_core::IUnknownImpl {
    fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut windows_core::BSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn HasProperty(&self, uriprop: Uri_PROPERTY) -> windows_core::Result<windows_core::BOOL>;
    fn GetAbsoluteUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAuthority(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetExtension(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFragment(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHost(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPassword(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPathAndQuery(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetQuery(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRawUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSchemeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHostType(&self) -> windows_core::Result<u32>;
    fn GetPort(&self) -> windows_core::Result<u32>;
    fn GetScheme(&self) -> windows_core::Result<u32>;
    fn GetZone(&self) -> windows_core::Result<u32>;
    fn GetProperties(&self) -> windows_core::Result<u32>;
    fn IsEqual(&self, puri: windows_core::Ref<IUri>) -> windows_core::Result<windows_core::BOOL>;
}
impl IUri_Vtbl {
    pub const fn new<Identity: IUri_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyBSTR<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUri_Impl::GetPropertyBSTR(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pbstrproperty), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetPropertyLength<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUri_Impl::GetPropertyLength(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pcchproperty), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetPropertyDWORD<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUri_Impl::GetPropertyDWORD(this, core::mem::transmute_copy(&uriprop), core::mem::transmute_copy(&pdwproperty), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn HasProperty<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::HasProperty(this, core::mem::transmute_copy(&uriprop)) {
                    Ok(ok__) => {
                        pfhasproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAbsoluteUri<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrabsoluteuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetAbsoluteUri(this) {
                    Ok(ok__) => {
                        pbstrabsoluteuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAuthority<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthority: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetAuthority(this) {
                    Ok(ok__) => {
                        pbstrauthority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayUri<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplaystring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetDisplayUri(this) {
                    Ok(ok__) => {
                        pbstrdisplaystring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDomain<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetDomain(this) {
                    Ok(ok__) => {
                        pbstrdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExtension<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrextension: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetExtension(this) {
                    Ok(ok__) => {
                        pbstrextension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFragment<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfragment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetFragment(this) {
                    Ok(ok__) => {
                        pbstrfragment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHost<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhost: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetHost(this) {
                    Ok(ok__) => {
                        pbstrhost.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPassword<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpassword: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPassword(this) {
                    Ok(ok__) => {
                        pbstrpassword.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPath<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPath(this) {
                    Ok(ok__) => {
                        pbstrpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPathAndQuery<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathandquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPathAndQuery(this) {
                    Ok(ok__) => {
                        pbstrpathandquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetQuery<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetQuery(this) {
                    Ok(ok__) => {
                        pbstrquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRawUri<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrawuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetRawUri(this) {
                    Ok(ok__) => {
                        pbstrrawuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSchemeName<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrschemename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetSchemeName(this) {
                    Ok(ok__) => {
                        pbstrschemename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserInfo<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruserinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetUserInfo(this) {
                    Ok(ok__) => {
                        pbstruserinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserName<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetUserName(this) {
                    Ok(ok__) => {
                        pbstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHostType<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhosttype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetHostType(this) {
                    Ok(ok__) => {
                        pdwhosttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPort<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetPort(this) {
                    Ok(ok__) => {
                        pdwport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScheme<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscheme: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetScheme(this) {
                    Ok(ok__) => {
                        pdwscheme.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetZone<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwzone: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetZone(this) {
                    Ok(ok__) => {
                        pdwzone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::GetProperties(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut core::ffi::c_void, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUri_Impl::IsEqual(this, core::mem::transmute_copy(&puri)) {
                    Ok(ok__) => {
                        pfequal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyBSTR: GetPropertyBSTR::<Identity, OFFSET>,
            GetPropertyLength: GetPropertyLength::<Identity, OFFSET>,
            GetPropertyDWORD: GetPropertyDWORD::<Identity, OFFSET>,
            HasProperty: HasProperty::<Identity, OFFSET>,
            GetAbsoluteUri: GetAbsoluteUri::<Identity, OFFSET>,
            GetAuthority: GetAuthority::<Identity, OFFSET>,
            GetDisplayUri: GetDisplayUri::<Identity, OFFSET>,
            GetDomain: GetDomain::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
            GetFragment: GetFragment::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            GetPassword: GetPassword::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetPathAndQuery: GetPathAndQuery::<Identity, OFFSET>,
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetRawUri: GetRawUri::<Identity, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, OFFSET>,
            GetUserInfo: GetUserInfo::<Identity, OFFSET>,
            GetUserName: GetUserName::<Identity, OFFSET>,
            GetHostType: GetHostType::<Identity, OFFSET>,
            GetPort: GetPort::<Identity, OFFSET>,
            GetScheme: GetScheme::<Identity, OFFSET>,
            GetZone: GetZone::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUri as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUri {}
windows_core::imp::define_interface!(IUriBuilder, IUriBuilder_Vtbl, 0x4221b2e1_8955_46c0_bd5b_de9897565de7);
windows_core::imp::interface_hierarchy!(IUriBuilder, windows_core::IUnknown);
impl IUriBuilder {
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUriSimple)(windows_core::Interface::as_raw(self), dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUri)(windows_core::Interface::as_raw(self), dwcreateflags, dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUriWithFlags)(windows_core::Interface::as_raw(self), dwcreateflags, dwuribuilderflags, dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIUri(&self) -> windows_core::Result<IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetIUri<P0>(&self, piuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIUri)(windows_core::Interface::as_raw(self), piuri.param().abi()).ok() }
    }
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFragment)(windows_core::Interface::as_raw(self), pcchfragment as _, ppwzfragment as _).ok() }
    }
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), pcchhost as _, ppwzhost as _).ok() }
    }
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPassword)(windows_core::Interface::as_raw(self), pcchpassword as _, ppwzpassword as _).ok() }
    }
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), pcchpath as _, ppwzpath as _).ok() }
    }
    pub unsafe fn GetPort(&self, pfhasport: *mut windows_core::BOOL, pdwport: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), pfhasport as _, pdwport as _).ok() }
    }
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), pcchquery as _, ppwzquery as _).ok() }
    }
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSchemeName)(windows_core::Interface::as_raw(self), pcchschemename as _, ppwzschemename as _).ok() }
    }
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUserName)(windows_core::Interface::as_raw(self), pcchusername as _, ppwzusername as _).ok() }
    }
    pub unsafe fn SetFragment<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFragment)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok() }
    }
    pub unsafe fn SetHost<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHost)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok() }
    }
    pub unsafe fn SetPassword<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok() }
    }
    pub unsafe fn SetPath<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok() }
    }
    pub unsafe fn SetPort(&self, fhasport: bool, dwnewvalue: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), fhasport.into(), dwnewvalue).ok() }
    }
    pub unsafe fn SetQuery<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetQuery)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok() }
    }
    pub unsafe fn SetSchemeName<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSchemeName)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok() }
    }
    pub unsafe fn SetUserName<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUserName)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok() }
    }
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveProperties)(windows_core::Interface::as_raw(self), dwpropertymask).ok() }
    }
    pub unsafe fn HasBeenModified(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasBeenModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHost: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub SetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HasBeenModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUriBuilder_Impl: windows_core::IUnknownImpl {
    fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri>;
    fn GetIUri(&self) -> windows_core::Result<IUri>;
    fn SetIUri(&self, piuri: windows_core::Ref<IUri>) -> windows_core::Result<()>;
    fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPort(&self, pfhasport: *mut windows_core::BOOL, pdwport: *mut u32) -> windows_core::Result<()>;
    fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFragment(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetHost(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPath(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPort(&self, fhasport: windows_core::BOOL, dwnewvalue: u32) -> windows_core::Result<()>;
    fn SetQuery(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSchemeName(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetUserName(&self, pwznewvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveProperties(&self, dwpropertymask: u32) -> windows_core::Result<()>;
    fn HasBeenModified(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUriBuilder_Vtbl {
    pub const fn new<Identity: IUriBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateUriSimple<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::CreateUriSimple(this, core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateUri<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::CreateUri(this, core::mem::transmute_copy(&dwcreateflags), core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateUriWithFlags<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::CreateUriWithFlags(this, core::mem::transmute_copy(&dwcreateflags), core::mem::transmute_copy(&dwuribuilderflags), core::mem::transmute_copy(&dwallowencodingpropertymask), core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIUri<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::GetIUri(this) {
                    Ok(ok__) => {
                        ppiuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIUri<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetIUri(this, core::mem::transmute_copy(&piuri)).into()
            }
        }
        unsafe extern "system" fn GetFragment<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetFragment(this, core::mem::transmute_copy(&pcchfragment), core::mem::transmute_copy(&ppwzfragment)).into()
            }
        }
        unsafe extern "system" fn GetHost<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetHost(this, core::mem::transmute_copy(&pcchhost), core::mem::transmute_copy(&ppwzhost)).into()
            }
        }
        unsafe extern "system" fn GetPassword<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetPassword(this, core::mem::transmute_copy(&pcchpassword), core::mem::transmute_copy(&ppwzpassword)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetPath(this, core::mem::transmute_copy(&pcchpath), core::mem::transmute_copy(&ppwzpath)).into()
            }
        }
        unsafe extern "system" fn GetPort<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasport: *mut windows_core::BOOL, pdwport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetPort(this, core::mem::transmute_copy(&pfhasport), core::mem::transmute_copy(&pdwport)).into()
            }
        }
        unsafe extern "system" fn GetQuery<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetQuery(this, core::mem::transmute_copy(&pcchquery), core::mem::transmute_copy(&ppwzquery)).into()
            }
        }
        unsafe extern "system" fn GetSchemeName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetSchemeName(this, core::mem::transmute_copy(&pcchschemename), core::mem::transmute_copy(&ppwzschemename)).into()
            }
        }
        unsafe extern "system" fn GetUserName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::GetUserName(this, core::mem::transmute_copy(&pcchusername), core::mem::transmute_copy(&ppwzusername)).into()
            }
        }
        unsafe extern "system" fn SetFragment<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetFragment(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetHost<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetHost(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetPassword(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetPath<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetPath(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetPort<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fhasport: windows_core::BOOL, dwnewvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetPort(this, core::mem::transmute_copy(&fhasport), core::mem::transmute_copy(&dwnewvalue)).into()
            }
        }
        unsafe extern "system" fn SetQuery<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetQuery(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetSchemeName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetSchemeName(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn SetUserName<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwznewvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::SetUserName(this, core::mem::transmute(&pwznewvalue)).into()
            }
        }
        unsafe extern "system" fn RemoveProperties<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpropertymask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUriBuilder_Impl::RemoveProperties(this, core::mem::transmute_copy(&dwpropertymask)).into()
            }
        }
        unsafe extern "system" fn HasBeenModified<Identity: IUriBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmodified: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriBuilder_Impl::HasBeenModified(this) {
                    Ok(ok__) => {
                        pfmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateUriSimple: CreateUriSimple::<Identity, OFFSET>,
            CreateUri: CreateUri::<Identity, OFFSET>,
            CreateUriWithFlags: CreateUriWithFlags::<Identity, OFFSET>,
            GetIUri: GetIUri::<Identity, OFFSET>,
            SetIUri: SetIUri::<Identity, OFFSET>,
            GetFragment: GetFragment::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            GetPassword: GetPassword::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetPort: GetPort::<Identity, OFFSET>,
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, OFFSET>,
            GetUserName: GetUserName::<Identity, OFFSET>,
            SetFragment: SetFragment::<Identity, OFFSET>,
            SetHost: SetHost::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            SetQuery: SetQuery::<Identity, OFFSET>,
            SetSchemeName: SetSchemeName::<Identity, OFFSET>,
            SetUserName: SetUserName::<Identity, OFFSET>,
            RemoveProperties: RemoveProperties::<Identity, OFFSET>,
            HasBeenModified: HasBeenModified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriBuilder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUriBuilder {}
windows_core::imp::define_interface!(IUrlMon, IUrlMon_Vtbl, 0x00000026_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IUrlMon, windows_core::IUnknown);
impl IUrlMon {
    pub unsafe fn AsyncGetClassBits<P1, P2, P5, P6>(&self, rclsid: *const windows_core::GUID, psztype: P1, pszext: P2, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: P5, pbc: P6, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncGetClassBits)(windows_core::Interface::as_raw(self), rclsid, psztype.param().abi(), pszext.param().abi(), dwfileversionms, dwfileversionls, pszcodebase.param().abi(), pbc.param().abi(), dwclasscontext, riid, flags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsyncGetClassBits: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait IUrlMon_Impl: windows_core::IUnknownImpl {
    fn AsyncGetClassBits(&self, rclsid: *const windows_core::GUID, psztype: &windows_core::PCWSTR, pszext: &windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: &windows_core::PCWSTR, pbc: windows_core::Ref<IBindCtx>, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::Result<()>;
}
impl IUrlMon_Vtbl {
    pub const fn new<Identity: IUrlMon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncGetClassBits<Identity: IUrlMon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, psztype: windows_core::PCWSTR, pszext: windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlMon_Impl::AsyncGetClassBits(this, core::mem::transmute_copy(&rclsid), core::mem::transmute(&psztype), core::mem::transmute(&pszext), core::mem::transmute_copy(&dwfileversionms), core::mem::transmute_copy(&dwfileversionls), core::mem::transmute(&pszcodebase), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&dwclasscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncGetClassBits: AsyncGetClassBits::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlMon as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUrlMon {}
windows_core::imp::define_interface!(IWaitMultiple, IWaitMultiple_Vtbl, 0x0000002b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IWaitMultiple, windows_core::IUnknown);
impl IWaitMultiple {
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> windows_core::Result<ISynchronize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISynchronize>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSynchronize)(windows_core::Interface::as_raw(self), psync.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultiple_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWaitMultiple_Impl: windows_core::IUnknownImpl {
    fn WaitMultiple(&self, timeout: u32) -> windows_core::Result<ISynchronize>;
    fn AddSynchronize(&self, psync: windows_core::Ref<ISynchronize>) -> windows_core::Result<()>;
}
impl IWaitMultiple_Vtbl {
    pub const fn new<Identity: IWaitMultiple_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WaitMultiple<Identity: IWaitMultiple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32, psync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWaitMultiple_Impl::WaitMultiple(this, core::mem::transmute_copy(&timeout)) {
                    Ok(ok__) => {
                        psync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSynchronize<Identity: IWaitMultiple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psync: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWaitMultiple_Impl::AddSynchronize(this, core::mem::transmute_copy(&psync)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WaitMultiple: WaitMultiple::<Identity, OFFSET>,
            AddSynchronize: AddSynchronize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWaitMultiple as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWaitMultiple {}
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCKTYPE(pub i32);
pub const LOCK_EXCLUSIVE: LOCKTYPE = LOCKTYPE(2i32);
pub const LOCK_ONLYONCE: LOCKTYPE = LOCKTYPE(4i32);
pub const LOCK_WRITE: LOCKTYPE = LOCKTYPE(1i32);
pub type LPEXCEPFINO_DEFERRED_FILLIN = Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> windows_core::HRESULT>;
pub type LPFNCANUNLOADNOW = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type LPFNGETCLASSOBJECT = Option<unsafe extern "system" fn(param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
pub const MAXLSN: u64 = 9223372036854775807u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMCTX(pub i32);
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MKRREDUCE(pub i32);
pub const MKRREDUCE_ALL: MKRREDUCE = MKRREDUCE(0i32);
pub const MKRREDUCE_ONE: MKRREDUCE = MKRREDUCE(196608i32);
pub const MKRREDUCE_THROUGHUSER: MKRREDUCE = MKRREDUCE(65536i32);
pub const MKRREDUCE_TOUSER: MKRREDUCE = MKRREDUCE(131072i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MKSYS(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSHCTX(pub i32);
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSHLFLAGS(pub i32);
pub const MSHLFLAGS_NOPING: MSHLFLAGS = MSHLFLAGS(4i32);
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = MSHLFLAGS(0i32);
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = MSHLFLAGS(8i32);
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = MSHLFLAGS(16i32);
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = MSHLFLAGS(32i32);
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = MSHLFLAGS(64i32);
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = MSHLFLAGS(1i32);
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = MSHLFLAGS(2i32);
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct MULTI_QI {
    pub pIID: *const windows_core::GUID,
    pub pItf: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub hr: windows_core::HRESULT,
}
impl Default for MULTI_QI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MachineGlobalObjectTableRegistrationToken(pub *mut core::ffi::c_void);
impl MachineGlobalObjectTableRegistrationToken {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for MachineGlobalObjectTableRegistrationToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PENDINGMSG(pub i32);
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PENDINGTYPE(pub i32);
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
pub type PFNCONTEXTCALL = Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REGCLS(pub i32);
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
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
pub const ROTFLAGS_ALLOWANYCLIENT: ROT_FLAGS = ROT_FLAGS(2u32);
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: ROT_FLAGS = ROT_FLAGS(1u32);
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROT_FLAGS(pub u32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPCOPT_PROPERTIES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_C_IMP_LEVEL(pub u32);
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SChannelHookCallInfo {
    pub iid: windows_core::GUID,
    pub cbSize: u32,
    pub uCausality: windows_core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut core::ffi::c_void,
}
impl Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVERCALL(pub i32);
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(1i32);
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(0i32);
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut core::ffi::c_void,
}
impl Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: windows_core::PWSTR,
    pub hr: windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: core::mem::ManuallyDrop<Option<IAdviseSink>>,
    pub dwConnection: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STATFLAG(pub i32);
pub const STATFLAG_DEFAULT: STATFLAG = STATFLAG(0i32);
pub const STATFLAG_NONAME: STATFLAG = STATFLAG(1i32);
pub const STATFLAG_NOOPEN: STATFLAG = STATFLAG(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STGC(pub i32);
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
pub const STGC_CONSOLIDATE: STGC = STGC(8i32);
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = STGC(4i32);
pub const STGC_DEFAULT: STGC = STGC(0i32);
pub const STGC_ONLYIFCURRENT: STGC = STGC(2i32);
pub const STGC_OVERWRITE: STGC = STGC(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STGM(pub u32);
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
impl Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STGTY(pub i32);
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
pub const STGTY_REPEAT: i32 = 256i32;
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
pub const STGTY_STREAM: STGTY = STGTY(2i32);
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STREAM_SEEK(pub u32);
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSKIND(pub i32);
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ShutdownType(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: windows_core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THDTYPE(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TLIBATTR {
    pub guid: windows_core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TYMED(pub i32);
pub const TYMED_ENHMF: TYMED = TYMED(64i32);
pub const TYMED_FILE: TYMED = TYMED(2i32);
pub const TYMED_GDI: TYMED = TYMED(16i32);
pub const TYMED_HGLOBAL: TYMED = TYMED(1i32);
pub const TYMED_ISTORAGE: TYMED = TYMED(8i32);
pub const TYMED_ISTREAM: TYMED = TYMED(4i32);
pub const TYMED_MFPICT: TYMED = TYMED(32i32);
pub const TYMED_NULL: TYMED = TYMED(0i32);
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
impl Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TYPEKIND(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TYSPEC(pub i32);
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URI_CREATE_FLAGS(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Uri_PROPERTY(pub i32);
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
    pub lpvarValue: *mut super::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VARFLAGS(pub u16);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VARKIND(pub i32);
pub const VAR_CONST: VARKIND = VARKIND(2i32);
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
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
impl Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: windows_core::PWSTR,
    pub PolicyId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: windows_core::GUID,
    pub PolicyId: windows_core::GUID,
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
impl Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
