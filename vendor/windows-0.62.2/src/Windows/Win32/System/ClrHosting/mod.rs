#[inline]
pub unsafe fn CLRCreateInstance<T>(clsid: *const windows_core::GUID) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("mscoree.dll" "system" fn CLRCreateInstance(clsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppinterface : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CLRCreateInstance(clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CallFunctionShim<P0, P1, P4>(szdllname: P0, szfunctionname: P1, lpvargument1: *mut core::ffi::c_void, lpvargument2: *mut core::ffi::c_void, szversion: P4, pvreserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn CallFunctionShim(szdllname : windows_core::PCWSTR, szfunctionname : windows_core::PCSTR, lpvargument1 : *mut core::ffi::c_void, lpvargument2 : *mut core::ffi::c_void, szversion : windows_core::PCWSTR, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CallFunctionShim(szdllname.param().abi(), szfunctionname.param().abi(), lpvargument1 as _, lpvargument2 as _, szversion.param().abi(), pvreserved as _).ok() }
}
#[inline]
pub unsafe fn ClrCreateManagedInstance<P0>(ptypename: P0, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn ClrCreateManagedInstance(ptypename : windows_core::PCWSTR, riid : *const windows_core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ClrCreateManagedInstance(ptypename.param().abi(), riid, ppobject as _).ok() }
}
#[inline]
pub unsafe fn CorBindToCurrentRuntime<P0>(pwszfilename: P0, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn CorBindToCurrentRuntime(pwszfilename : windows_core::PCWSTR, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CorBindToCurrentRuntime(pwszfilename.param().abi(), rclsid, riid, ppv as _).ok() }
}
#[inline]
pub unsafe fn CorBindToRuntime<P0, P1>(pwszversion: P0, pwszbuildflavor: P1, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn CorBindToRuntime(pwszversion : windows_core::PCWSTR, pwszbuildflavor : windows_core::PCWSTR, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CorBindToRuntime(pwszversion.param().abi(), pwszbuildflavor.param().abi(), rclsid, riid, ppv as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CorBindToRuntimeByCfg<P0>(pcfgstream: P0, reserved: u32, startupflags: u32, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IStream>,
{
    windows_core::link!("mscoree.dll" "system" fn CorBindToRuntimeByCfg(pcfgstream : * mut core::ffi::c_void, reserved : u32, startupflags : u32, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CorBindToRuntimeByCfg(pcfgstream.param().abi(), reserved, startupflags, rclsid, riid, ppv as _).ok() }
}
#[inline]
pub unsafe fn CorBindToRuntimeEx<P0, P1>(pwszversion: P0, pwszbuildflavor: P1, startupflags: u32, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn CorBindToRuntimeEx(pwszversion : windows_core::PCWSTR, pwszbuildflavor : windows_core::PCWSTR, startupflags : u32, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CorBindToRuntimeEx(pwszversion.param().abi(), pwszbuildflavor.param().abi(), startupflags, rclsid, riid, ppv as _).ok() }
}
#[inline]
pub unsafe fn CorBindToRuntimeHost<P0, P1, P2>(pwszversion: P0, pwszbuildflavor: P1, pwszhostconfigfile: P2, preserved: *mut core::ffi::c_void, startupflags: u32, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn CorBindToRuntimeHost(pwszversion : windows_core::PCWSTR, pwszbuildflavor : windows_core::PCWSTR, pwszhostconfigfile : windows_core::PCWSTR, preserved : *mut core::ffi::c_void, startupflags : u32, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CorBindToRuntimeHost(pwszversion.param().abi(), pwszbuildflavor.param().abi(), pwszhostconfigfile.param().abi(), preserved as _, startupflags, rclsid, riid, ppv as _).ok() }
}
#[inline]
pub unsafe fn CorExitProcess(exitcode: i32) {
    windows_core::link!("mscoree.dll" "system" fn CorExitProcess(exitcode : i32));
    unsafe { CorExitProcess(exitcode) }
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn CorLaunchApplication<P1>(dwclickoncehost: HOST_TYPE, pwzappfullname: P1, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR, lpprocessinformation: *mut super::Threading::PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn CorLaunchApplication(dwclickoncehost : HOST_TYPE, pwzappfullname : windows_core::PCWSTR, dwmanifestpaths : u32, ppwzmanifestpaths : *const windows_core::PCWSTR, dwactivationdata : u32, ppwzactivationdata : *const windows_core::PCWSTR, lpprocessinformation : *mut super::Threading:: PROCESS_INFORMATION) -> windows_core::HRESULT);
    unsafe { CorLaunchApplication(dwclickoncehost, pwzappfullname.param().abi(), dwmanifestpaths, ppwzmanifestpaths, dwactivationdata, ppwzactivationdata, lpprocessinformation as _).ok() }
}
#[inline]
pub unsafe fn CorMarkThreadInThreadPool() {
    windows_core::link!("mscoree.dll" "system" fn CorMarkThreadInThreadPool());
    unsafe { CorMarkThreadInThreadPool() }
}
#[inline]
pub unsafe fn CreateDebuggingInterfaceFromVersion<P1>(idebuggerversion: i32, szdebuggeeversion: P1) -> windows_core::Result<windows_core::IUnknown>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn CreateDebuggingInterfaceFromVersion(idebuggerversion : i32, szdebuggeeversion : windows_core::PCWSTR, ppcordb : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDebuggingInterfaceFromVersion(idebuggerversion, szdebuggeeversion.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn GetCLRIdentityManager(riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("mscoree.dll" "system" fn GetCLRIdentityManager(riid : *const windows_core::GUID, ppmanager : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetCLRIdentityManager(riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn GetCORRequiredVersion(pbuffer: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn GetCORRequiredVersion(pbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    unsafe { GetCORRequiredVersion(core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), dwlength as _).ok() }
}
#[inline]
pub unsafe fn GetCORSystemDirectory(pbuffer: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn GetCORSystemDirectory(pbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    unsafe { GetCORSystemDirectory(core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), dwlength as _).ok() }
}
#[inline]
pub unsafe fn GetCORVersion(pbbuffer: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn GetCORVersion(pbbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    unsafe { GetCORVersion(core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len().try_into().unwrap(), dwlength as _).ok() }
}
#[inline]
pub unsafe fn GetFileVersion<P0>(szfilename: P0, szbuffer: Option<&mut [u16]>, dwlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn GetFileVersion(szfilename : windows_core::PCWSTR, szbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    unsafe { GetFileVersion(szfilename.param().abi(), core::mem::transmute(szbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwlength as _).ok() }
}
#[inline]
pub unsafe fn GetRealProcAddress<P0>(pwszprocname: P0, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn GetRealProcAddress(pwszprocname : windows_core::PCSTR, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetRealProcAddress(pwszprocname.param().abi(), ppv as _).ok() }
}
#[inline]
pub unsafe fn GetRequestedRuntimeInfo<P0, P1, P2>(pexe: P0, pwszversion: P1, pconfigurationfile: P2, startupflags: u32, runtimeinfoflags: u32, pdirectory: Option<&mut [u16]>, dwdirectorylength: Option<*mut u32>, pversion: Option<&mut [u16]>, dwlength: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn GetRequestedRuntimeInfo(pexe : windows_core::PCWSTR, pwszversion : windows_core::PCWSTR, pconfigurationfile : windows_core::PCWSTR, startupflags : u32, runtimeinfoflags : u32, pdirectory : windows_core::PWSTR, dwdirectory : u32, dwdirectorylength : *mut u32, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    unsafe {
        GetRequestedRuntimeInfo(
            pexe.param().abi(),
            pwszversion.param().abi(),
            pconfigurationfile.param().abi(),
            startupflags,
            runtimeinfoflags,
            core::mem::transmute(pdirectory.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pdirectory.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            dwdirectorylength.unwrap_or(core::mem::zeroed()) as _,
            core::mem::transmute(pversion.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pversion.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            dwlength.unwrap_or(core::mem::zeroed()) as _,
        )
        .ok()
    }
}
#[inline]
pub unsafe fn GetRequestedRuntimeVersion<P0>(pexe: P0, pversion: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn GetRequestedRuntimeVersion(pexe : windows_core::PCWSTR, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    unsafe { GetRequestedRuntimeVersion(pexe.param().abi(), core::mem::transmute(pversion.as_ptr()), pversion.len().try_into().unwrap(), dwlength as _).ok() }
}
#[inline]
pub unsafe fn GetRequestedRuntimeVersionForCLSID(rclsid: *const windows_core::GUID, pversion: Option<&mut [u16]>, dwlength: Option<*mut u32>, dwresolutionflags: CLSID_RESOLUTION_FLAGS) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn GetRequestedRuntimeVersionForCLSID(rclsid : *const windows_core::GUID, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32, dwresolutionflags : CLSID_RESOLUTION_FLAGS) -> windows_core::HRESULT);
    unsafe { GetRequestedRuntimeVersionForCLSID(rclsid, core::mem::transmute(pversion.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pversion.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwlength.unwrap_or(core::mem::zeroed()) as _, dwresolutionflags).ok() }
}
#[inline]
pub unsafe fn GetVersionFromProcess(hprocess: super::super::Foundation::HANDLE, pversion: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn GetVersionFromProcess(hprocess : super::super::Foundation:: HANDLE, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    unsafe { GetVersionFromProcess(hprocess, core::mem::transmute(pversion.as_ptr()), pversion.len().try_into().unwrap(), dwlength as _).ok() }
}
#[inline]
pub unsafe fn LoadLibraryShim<P0, P1>(szdllname: P0, szversion: P1, pvreserved: *mut core::ffi::c_void, phmoddll: *mut super::super::Foundation::HMODULE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn LoadLibraryShim(szdllname : windows_core::PCWSTR, szversion : windows_core::PCWSTR, pvreserved : *mut core::ffi::c_void, phmoddll : *mut super::super::Foundation:: HMODULE) -> windows_core::HRESULT);
    unsafe { LoadLibraryShim(szdllname.param().abi(), szversion.param().abi(), pvreserved as _, phmoddll as _).ok() }
}
#[inline]
pub unsafe fn LoadStringRC(iresouceid: u32, szbuffer: &mut [u16], bquiet: i32) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn LoadStringRC(iresouceid : u32, szbuffer : windows_core::PWSTR, imax : i32, bquiet : i32) -> windows_core::HRESULT);
    unsafe { LoadStringRC(iresouceid, core::mem::transmute(szbuffer.as_ptr()), szbuffer.len().try_into().unwrap(), bquiet).ok() }
}
#[inline]
pub unsafe fn LoadStringRCEx(lcid: u32, iresouceid: u32, szbuffer: &mut [u16], bquiet: i32, pcwchused: *mut i32) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn LoadStringRCEx(lcid : u32, iresouceid : u32, szbuffer : windows_core::PWSTR, imax : i32, bquiet : i32, pcwchused : *mut i32) -> windows_core::HRESULT);
    unsafe { LoadStringRCEx(lcid, iresouceid, core::mem::transmute(szbuffer.as_ptr()), szbuffer.len().try_into().unwrap(), bquiet, pcwchused as _).ok() }
}
#[inline]
pub unsafe fn LockClrVersion(hostcallback: FLockClrVersionCallback, pbeginhostsetup: *mut FLockClrVersionCallback, pendhostsetup: *mut FLockClrVersionCallback) -> windows_core::Result<()> {
    windows_core::link!("mscoree.dll" "system" fn LockClrVersion(hostcallback : FLockClrVersionCallback, pbeginhostsetup : *mut FLockClrVersionCallback, pendhostsetup : *mut FLockClrVersionCallback) -> windows_core::HRESULT);
    unsafe { LockClrVersion(hostcallback, pbeginhostsetup as _, pendhostsetup as _).ok() }
}
#[inline]
pub unsafe fn RunDll32ShimW<P2>(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HINSTANCE, lpszcmdline: P2, ncmdshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscoree.dll" "system" fn RunDll32ShimW(hwnd : super::super::Foundation:: HWND, hinst : super::super::Foundation:: HINSTANCE, lpszcmdline : windows_core::PCWSTR, ncmdshow : i32) -> windows_core::HRESULT);
    unsafe { RunDll32ShimW(hwnd, hinst, lpszcmdline.param().abi(), ncmdshow).ok() }
}
pub const APPDOMAIN_FORCE_TRIVIAL_WAIT_OPERATIONS: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(8i32);
pub const APPDOMAIN_SECURITY_DEFAULT: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPDOMAIN_SECURITY_FLAGS(pub i32);
pub const APPDOMAIN_SECURITY_FORBID_CROSSAD_REVERSE_PINVOKE: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(2i32);
pub const APPDOMAIN_SECURITY_SANDBOXED: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AssemblyBindInfo {
    pub dwAppDomainId: u32,
    pub lpReferencedIdentity: windows_core::PCWSTR,
    pub lpPostPolicyIdentity: windows_core::PCWSTR,
    pub ePolicyLevel: u32,
}
pub const BucketParamLength: u32 = 255u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BucketParameterIndex(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BucketParameters {
    pub fInited: windows_core::BOOL,
    pub pszEventTypeName: [u16; 255],
    pub pszParams: [u16; 2550],
}
impl Default for BucketParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BucketParamsCount: u32 = 10u32;
pub type CLRCreateInstanceFnPtr = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub const CLRRuntimeHost: windows_core::GUID = windows_core::GUID::from_u128(0x90f1a06e_7712_4762_86b5_7a5eba6bdb02);
pub const CLR_ASSEMBLY_BUILD_VERSION: u32 = 0u32;
pub const CLR_ASSEMBLY_IDENTITY_FLAGS_DEFAULT: ECLRAssemblyIdentityFlags = ECLRAssemblyIdentityFlags(0i32);
pub const CLR_ASSEMBLY_MAJOR_VERSION: u32 = 4u32;
pub const CLR_ASSEMBLY_MINOR_VERSION: u32 = 0u32;
pub const CLR_BUILD_VERSION: u32 = 22220u32;
pub const CLR_DEBUGGING_MANAGED_EVENT_DEBUGGER_LAUNCH: CLR_DEBUGGING_PROCESS_FLAGS = CLR_DEBUGGING_PROCESS_FLAGS(2i32);
pub const CLR_DEBUGGING_MANAGED_EVENT_PENDING: CLR_DEBUGGING_PROCESS_FLAGS = CLR_DEBUGGING_PROCESS_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLR_DEBUGGING_PROCESS_FLAGS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLR_DEBUGGING_VERSION {
    pub wStructVersion: u16,
    pub wMajor: u16,
    pub wMinor: u16,
    pub wBuild: u16,
    pub wRevision: u16,
}
pub const CLR_MAJOR_VERSION: u32 = 4u32;
pub const CLR_MINOR_VERSION: u32 = 0u32;
pub const CLSID_CLRDebugging: windows_core::GUID = windows_core::GUID::from_u128(0xbacc578d_fbdd_48a4_969f_02d932b74634);
pub const CLSID_CLRDebuggingLegacy: windows_core::GUID = windows_core::GUID::from_u128(0xdf8395b5_a4ba_450b_a77c_a9a47762c520);
pub const CLSID_CLRMetaHost: windows_core::GUID = windows_core::GUID::from_u128(0x9280188d_0e8e_4867_b30c_7fa83884e8de);
pub const CLSID_CLRMetaHostPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x2ebcd49a_1b47_4a61_b13a_4a03701e594b);
pub const CLSID_CLRProfiling: windows_core::GUID = windows_core::GUID::from_u128(0xbd097ed8_733e_43fe_8ed7_a95ff9a8448c);
pub const CLSID_CLRStrongName: windows_core::GUID = windows_core::GUID::from_u128(0xb79b0acd_f5cd_409b_b5a5_a16244610b92);
pub const CLSID_RESOLUTION_DEFAULT: CLSID_RESOLUTION_FLAGS = CLSID_RESOLUTION_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLSID_RESOLUTION_FLAGS(pub i32);
pub const CLSID_RESOLUTION_REGISTERED: CLSID_RESOLUTION_FLAGS = CLSID_RESOLUTION_FLAGS(1i32);
pub const COR_GC_COUNTS: COR_GC_STAT_TYPES = COR_GC_STAT_TYPES(1i32);
pub const COR_GC_MEMORYUSAGE: COR_GC_STAT_TYPES = COR_GC_STAT_TYPES(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_GC_STATS {
    pub Flags: u32,
    pub ExplicitGCCount: usize,
    pub GenCollectionsTaken: [usize; 3],
    pub CommittedKBytes: usize,
    pub ReservedKBytes: usize,
    pub Gen0HeapSizeKBytes: usize,
    pub Gen1HeapSizeKBytes: usize,
    pub Gen2HeapSizeKBytes: usize,
    pub LargeObjectHeapSizeKBytes: usize,
    pub KBytesPromotedFromGen0: usize,
    pub KBytesPromotedFromGen1: usize,
}
impl Default for COR_GC_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_GC_STAT_TYPES(pub i32);
pub const COR_GC_THREAD_HAS_PROMOTED_BYTES: COR_GC_THREAD_STATS_TYPES = COR_GC_THREAD_STATS_TYPES(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COR_GC_THREAD_STATS {
    pub PerThreadAllocation: u64,
    pub Flags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COR_GC_THREAD_STATS_TYPES(pub i32);
pub type CallbackThreadSetFnPtr = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type CallbackThreadUnsetFnPtr = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub const ComCallUnmarshal: windows_core::GUID = windows_core::GUID::from_u128(0x3f281000_e95a_11d2_886b_00c04f869f04);
pub const ComCallUnmarshalV4: windows_core::GUID = windows_core::GUID::from_u128(0x45fb4600_e6e8_4928_b25e_50476ff79425);
pub const CorRuntimeHost: windows_core::GUID = windows_core::GUID::from_u128(0xcb2f6723_ab3a_11d2_9c40_00c04fa30a3e);
pub type CreateInterfaceFnPtr = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CustomDumpItem {
    pub itemKind: ECustomDumpItemKind,
    pub Anonymous: CustomDumpItem_0,
}
impl Default for CustomDumpItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CustomDumpItem_0 {
    pub pReserved: usize,
}
impl Default for CustomDumpItem_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEPRECATED_CLR_API_MESG: windows_core::PCSTR = windows_core::s!("This API has been deprecated. Refer to https://go.microsoft.com/fwlink/?LinkId=143720 for more details.");
pub const DUMP_FLAVOR_CriticalCLRState: ECustomDumpFlavor = ECustomDumpFlavor(1i32);
pub const DUMP_FLAVOR_Default: ECustomDumpFlavor = ECustomDumpFlavor(0i32);
pub const DUMP_FLAVOR_Mini: ECustomDumpFlavor = ECustomDumpFlavor(0i32);
pub const DUMP_FLAVOR_NonHeapCLRState: ECustomDumpFlavor = ECustomDumpFlavor(2i32);
pub const DUMP_ITEM_None: ECustomDumpItemKind = ECustomDumpItemKind(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EApiCategories(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EBindPolicyLevels(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ECLRAssemblyIdentityFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EClrEvent(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EClrFailure(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EClrOperation(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EClrUnhandledException(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EContextType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ECustomDumpFlavor(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ECustomDumpItemKind(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EHostApplicationPolicy(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EHostBindingPolicyModifyFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EInitializeNewDomainFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EMemoryAvailable(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EMemoryCriticalLevel(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPolicyAction(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ESymbolReadingPolicy(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETaskType(pub i32);
pub const Event_ClrDisabled: EClrEvent = EClrEvent(1i32);
pub const Event_DomainUnload: EClrEvent = EClrEvent(0i32);
pub const Event_MDAFired: EClrEvent = EClrEvent(2i32);
pub const Event_StackOverflow: EClrEvent = EClrEvent(3i32);
pub const FAIL_AccessViolation: EClrFailure = EClrFailure(5i32);
pub const FAIL_CodeContract: EClrFailure = EClrFailure(6i32);
pub const FAIL_CriticalResource: EClrFailure = EClrFailure(1i32);
pub const FAIL_FatalRuntime: EClrFailure = EClrFailure(2i32);
pub const FAIL_NonCriticalResource: EClrFailure = EClrFailure(0i32);
pub const FAIL_OrphanedLock: EClrFailure = EClrFailure(3i32);
pub const FAIL_StackOverflow: EClrFailure = EClrFailure(4i32);
pub type FExecuteInAppDomainCallback = Option<unsafe extern "system" fn(cookie: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FLockClrVersionCallback = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub const HOST_APPLICATION_BINDING_POLICY: EHostApplicationPolicy = EHostApplicationPolicy(1i32);
pub const HOST_BINDING_POLICY_MODIFY_CHAIN: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(1i32);
pub const HOST_BINDING_POLICY_MODIFY_DEFAULT: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(0i32);
pub const HOST_BINDING_POLICY_MODIFY_MAX: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(3i32);
pub const HOST_BINDING_POLICY_MODIFY_REMOVE: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HOST_TYPE(pub i32);
pub const HOST_TYPE_APPLAUNCH: HOST_TYPE = HOST_TYPE(1i32);
pub const HOST_TYPE_CORFLAG: HOST_TYPE = HOST_TYPE(2i32);
pub const HOST_TYPE_DEFAULT: HOST_TYPE = HOST_TYPE(0i32);
windows_core::imp::define_interface!(IActionOnCLREvent, IActionOnCLREvent_Vtbl, 0x607be24b_d91b_4e28_a242_61871ce56e35);
windows_core::imp::interface_hierarchy!(IActionOnCLREvent, windows_core::IUnknown);
impl IActionOnCLREvent {
    pub unsafe fn OnEvent(&self, event: EClrEvent, data: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), event, data).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionOnCLREvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, EClrEvent, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActionOnCLREvent_Impl: windows_core::IUnknownImpl {
    fn OnEvent(&self, event: EClrEvent, data: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IActionOnCLREvent_Vtbl {
    pub const fn new<Identity: IActionOnCLREvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEvent<Identity: IActionOnCLREvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: EClrEvent, data: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionOnCLREvent_Impl::OnEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&data)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActionOnCLREvent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActionOnCLREvent {}
windows_core::imp::define_interface!(IApartmentCallback, IApartmentCallback_Vtbl, 0x178e5337_1528_4591_b1c9_1c6e484686d8);
windows_core::imp::interface_hierarchy!(IApartmentCallback, windows_core::IUnknown);
impl IApartmentCallback {
    pub unsafe fn DoCallback(&self, pfunc: usize, pdata: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoCallback)(windows_core::Interface::as_raw(self), pfunc, pdata).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoCallback: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
pub trait IApartmentCallback_Impl: windows_core::IUnknownImpl {
    fn DoCallback(&self, pfunc: usize, pdata: usize) -> windows_core::Result<()>;
}
impl IApartmentCallback_Vtbl {
    pub const fn new<Identity: IApartmentCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoCallback<Identity: IApartmentCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunc: usize, pdata: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApartmentCallback_Impl::DoCallback(this, core::mem::transmute_copy(&pfunc), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoCallback: DoCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApartmentCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IApartmentCallback {}
windows_core::imp::define_interface!(IAppDomainBinding, IAppDomainBinding_Vtbl, 0x5c2b07a7_1e98_11d3_872f_00c04f79ed0d);
windows_core::imp::interface_hierarchy!(IAppDomainBinding, windows_core::IUnknown);
impl IAppDomainBinding {
    pub unsafe fn OnAppDomain<P0>(&self, pappdomain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAppDomain)(windows_core::Interface::as_raw(self), pappdomain.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDomainBinding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppDomainBinding_Impl: windows_core::IUnknownImpl {
    fn OnAppDomain(&self, pappdomain: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IAppDomainBinding_Vtbl {
    pub const fn new<Identity: IAppDomainBinding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAppDomain<Identity: IAppDomainBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppDomainBinding_Impl::OnAppDomain(this, core::mem::transmute_copy(&pappdomain)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnAppDomain: OnAppDomain::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppDomainBinding as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppDomainBinding {}
windows_core::imp::define_interface!(ICLRAppDomainResourceMonitor, ICLRAppDomainResourceMonitor_Vtbl, 0xc62de18c_2e23_4aea_8423_b40c1fc59eae);
windows_core::imp::interface_hierarchy!(ICLRAppDomainResourceMonitor, windows_core::IUnknown);
impl ICLRAppDomainResourceMonitor {
    pub unsafe fn GetCurrentAllocated(&self, dwappdomainid: u32, pbytesallocated: *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentAllocated)(windows_core::Interface::as_raw(self), dwappdomainid, pbytesallocated as _).ok() }
    }
    pub unsafe fn GetCurrentSurvived(&self, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentSurvived)(windows_core::Interface::as_raw(self), dwappdomainid, pappdomainbytessurvived as _, ptotalbytessurvived as _).ok() }
    }
    pub unsafe fn GetCurrentCpuTime(&self, dwappdomainid: u32, pmilliseconds: *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentCpuTime)(windows_core::Interface::as_raw(self), dwappdomainid, pmilliseconds as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRAppDomainResourceMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub GetCurrentSurvived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetCurrentCpuTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
}
pub trait ICLRAppDomainResourceMonitor_Impl: windows_core::IUnknownImpl {
    fn GetCurrentAllocated(&self, dwappdomainid: u32, pbytesallocated: *mut u64) -> windows_core::Result<()>;
    fn GetCurrentSurvived(&self, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> windows_core::Result<()>;
    fn GetCurrentCpuTime(&self, dwappdomainid: u32, pmilliseconds: *mut u64) -> windows_core::Result<()>;
}
impl ICLRAppDomainResourceMonitor_Vtbl {
    pub const fn new<Identity: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentAllocated<Identity: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pbytesallocated: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRAppDomainResourceMonitor_Impl::GetCurrentAllocated(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pbytesallocated)).into()
            }
        }
        unsafe extern "system" fn GetCurrentSurvived<Identity: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRAppDomainResourceMonitor_Impl::GetCurrentSurvived(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pappdomainbytessurvived), core::mem::transmute_copy(&ptotalbytessurvived)).into()
            }
        }
        unsafe extern "system" fn GetCurrentCpuTime<Identity: ICLRAppDomainResourceMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pmilliseconds: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRAppDomainResourceMonitor_Impl::GetCurrentCpuTime(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pmilliseconds)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentAllocated: GetCurrentAllocated::<Identity, OFFSET>,
            GetCurrentSurvived: GetCurrentSurvived::<Identity, OFFSET>,
            GetCurrentCpuTime: GetCurrentCpuTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRAppDomainResourceMonitor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRAppDomainResourceMonitor {}
windows_core::imp::define_interface!(ICLRAssemblyIdentityManager, ICLRAssemblyIdentityManager_Vtbl, 0x15f0a9da_3ff6_4393_9da9_fdfd284e6972);
windows_core::imp::interface_hierarchy!(ICLRAssemblyIdentityManager, windows_core::IUnknown);
impl ICLRAssemblyIdentityManager {
    pub unsafe fn GetCLRAssemblyReferenceList(&self, ppwzassemblyreferences: *const windows_core::PCWSTR, dwnumofreferences: u32) -> windows_core::Result<ICLRAssemblyReferenceList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCLRAssemblyReferenceList)(windows_core::Interface::as_raw(self), ppwzassemblyreferences, dwnumofreferences, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBindingIdentityFromFile<P0>(&self, pwzfilepath: P0, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetBindingIdentityFromFile)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), dwflags, core::mem::transmute(pwzbuffer), pcchbuffersize as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBindingIdentityFromStream<P0>(&self, pstream: P0, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetBindingIdentityFromStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwflags, core::mem::transmute(pwzbuffer), pcchbuffersize as _).ok() }
    }
    pub unsafe fn GetReferencedAssembliesFromFile<P0, P2>(&self, pwzfilepath: P0, dwflags: u32, pexcludeassemblieslist: P2) -> windows_core::Result<ICLRReferenceAssemblyEnum>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<ICLRAssemblyReferenceList>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReferencedAssembliesFromFile)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), dwflags, pexcludeassemblieslist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReferencedAssembliesFromStream<P0, P2>(&self, pstream: P0, dwflags: u32, pexcludeassemblieslist: P2) -> windows_core::Result<ICLRReferenceAssemblyEnum>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P2: windows_core::Param<ICLRAssemblyReferenceList>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReferencedAssembliesFromStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwflags, pexcludeassemblieslist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetProbingAssembliesFromReference<P2>(&self, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: P2) -> windows_core::Result<ICLRProbingAssemblyEnum>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProbingAssembliesFromReference)(windows_core::Interface::as_raw(self), dwmachinetype, dwflags, pwzreferenceidentity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsStronglyNamed<P0>(&self, pwzassemblyidentity: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStronglyNamed)(windows_core::Interface::as_raw(self), pwzassemblyidentity.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRAssemblyIdentityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCLRAssemblyReferenceList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBindingIdentityFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBindingIdentityFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBindingIdentityFromStream: usize,
    pub GetReferencedAssembliesFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetReferencedAssembliesFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetReferencedAssembliesFromStream: usize,
    pub GetProbingAssembliesFromReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsStronglyNamed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRAssemblyIdentityManager_Impl: windows_core::IUnknownImpl {
    fn GetCLRAssemblyReferenceList(&self, ppwzassemblyreferences: *const windows_core::PCWSTR, dwnumofreferences: u32) -> windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetBindingIdentityFromFile(&self, pwzfilepath: &windows_core::PCWSTR, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
    fn GetBindingIdentityFromStream(&self, pstream: windows_core::Ref<super::Com::IStream>, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
    fn GetReferencedAssembliesFromFile(&self, pwzfilepath: &windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: windows_core::Ref<ICLRAssemblyReferenceList>) -> windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetReferencedAssembliesFromStream(&self, pstream: windows_core::Ref<super::Com::IStream>, dwflags: u32, pexcludeassemblieslist: windows_core::Ref<ICLRAssemblyReferenceList>) -> windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetProbingAssembliesFromReference(&self, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: &windows_core::PCWSTR) -> windows_core::Result<ICLRProbingAssemblyEnum>;
    fn IsStronglyNamed(&self, pwzassemblyidentity: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl ICLRAssemblyIdentityManager_Vtbl {
    pub const fn new<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCLRAssemblyReferenceList<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwzassemblyreferences: *const windows_core::PCWSTR, dwnumofreferences: u32, ppreferencelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRAssemblyIdentityManager_Impl::GetCLRAssemblyReferenceList(this, core::mem::transmute_copy(&ppwzassemblyreferences), core::mem::transmute_copy(&dwnumofreferences)) {
                    Ok(ok__) => {
                        ppreferencelist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBindingIdentityFromFile<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRAssemblyIdentityManager_Impl::GetBindingIdentityFromFile(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
            }
        }
        unsafe extern "system" fn GetBindingIdentityFromStream<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRAssemblyIdentityManager_Impl::GetBindingIdentityFromStream(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
            }
        }
        unsafe extern "system" fn GetReferencedAssembliesFromFile<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: *mut core::ffi::c_void, ppreferenceenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRAssemblyIdentityManager_Impl::GetReferencedAssembliesFromFile(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pexcludeassemblieslist)) {
                    Ok(ok__) => {
                        ppreferenceenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReferencedAssembliesFromStream<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwflags: u32, pexcludeassemblieslist: *mut core::ffi::c_void, ppreferenceenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRAssemblyIdentityManager_Impl::GetReferencedAssembliesFromStream(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pexcludeassemblieslist)) {
                    Ok(ok__) => {
                        ppreferenceenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProbingAssembliesFromReference<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: windows_core::PCWSTR, ppprobingassemblyenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRAssemblyIdentityManager_Impl::GetProbingAssembliesFromReference(this, core::mem::transmute_copy(&dwmachinetype), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pwzreferenceidentity)) {
                    Ok(ok__) => {
                        ppprobingassemblyenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsStronglyNamed<Identity: ICLRAssemblyIdentityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassemblyidentity: windows_core::PCWSTR, pbisstronglynamed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRAssemblyIdentityManager_Impl::IsStronglyNamed(this, core::mem::transmute(&pwzassemblyidentity)) {
                    Ok(ok__) => {
                        pbisstronglynamed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRAssemblyReferenceList: GetCLRAssemblyReferenceList::<Identity, OFFSET>,
            GetBindingIdentityFromFile: GetBindingIdentityFromFile::<Identity, OFFSET>,
            GetBindingIdentityFromStream: GetBindingIdentityFromStream::<Identity, OFFSET>,
            GetReferencedAssembliesFromFile: GetReferencedAssembliesFromFile::<Identity, OFFSET>,
            GetReferencedAssembliesFromStream: GetReferencedAssembliesFromStream::<Identity, OFFSET>,
            GetProbingAssembliesFromReference: GetProbingAssembliesFromReference::<Identity, OFFSET>,
            IsStronglyNamed: IsStronglyNamed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRAssemblyIdentityManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICLRAssemblyIdentityManager {}
windows_core::imp::define_interface!(ICLRAssemblyReferenceList, ICLRAssemblyReferenceList_Vtbl, 0x1b2c9750_2e66_4bda_8b44_0a642c5cd733);
windows_core::imp::interface_hierarchy!(ICLRAssemblyReferenceList, windows_core::IUnknown);
impl ICLRAssemblyReferenceList {
    pub unsafe fn IsStringAssemblyReferenceInList<P0>(&self, pwzassemblyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsStringAssemblyReferenceInList)(windows_core::Interface::as_raw(self), pwzassemblyname.param().abi()).ok() }
    }
    pub unsafe fn IsAssemblyReferenceInList<P0>(&self, pname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsAssemblyReferenceInList)(windows_core::Interface::as_raw(self), pname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRAssemblyReferenceList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsStringAssemblyReferenceInList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsAssemblyReferenceInList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICLRAssemblyReferenceList_Impl: windows_core::IUnknownImpl {
    fn IsStringAssemblyReferenceInList(&self, pwzassemblyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsAssemblyReferenceInList(&self, pname: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ICLRAssemblyReferenceList_Vtbl {
    pub const fn new<Identity: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsStringAssemblyReferenceInList<Identity: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassemblyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRAssemblyReferenceList_Impl::IsStringAssemblyReferenceInList(this, core::mem::transmute(&pwzassemblyname)).into()
            }
        }
        unsafe extern "system" fn IsAssemblyReferenceInList<Identity: ICLRAssemblyReferenceList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRAssemblyReferenceList_Impl::IsAssemblyReferenceInList(this, core::mem::transmute_copy(&pname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStringAssemblyReferenceInList: IsStringAssemblyReferenceInList::<Identity, OFFSET>,
            IsAssemblyReferenceInList: IsAssemblyReferenceInList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRAssemblyReferenceList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRAssemblyReferenceList {}
windows_core::imp::define_interface!(ICLRControl, ICLRControl_Vtbl, 0x9065597e_d1a1_4fb2_b6ba_7e1fce230f61);
windows_core::imp::interface_hierarchy!(ICLRControl, windows_core::IUnknown);
impl ICLRControl {
    pub unsafe fn GetCLRManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCLRManager)(windows_core::Interface::as_raw(self), riid, ppobject as _).ok() }
    }
    pub unsafe fn SetAppDomainManagerType<P0, P1>(&self, pwzappdomainmanagerassembly: P0, pwzappdomainmanagertype: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAppDomainManagerType)(windows_core::Interface::as_raw(self), pwzappdomainmanagerassembly.param().abi(), pwzappdomainmanagertype.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCLRManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppDomainManagerType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICLRControl_Impl: windows_core::IUnknownImpl {
    fn GetCLRManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetAppDomainManagerType(&self, pwzappdomainmanagerassembly: &windows_core::PCWSTR, pwzappdomainmanagertype: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICLRControl_Vtbl {
    pub const fn new<Identity: ICLRControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCLRManager<Identity: ICLRControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRControl_Impl::GetCLRManager(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ICLRControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzappdomainmanagerassembly: windows_core::PCWSTR, pwzappdomainmanagertype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRControl_Impl::SetAppDomainManagerType(this, core::mem::transmute(&pwzappdomainmanagerassembly), core::mem::transmute(&pwzappdomainmanagertype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRManager: GetCLRManager::<Identity, OFFSET>,
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRControl {}
windows_core::imp::define_interface!(ICLRDebugManager, ICLRDebugManager_Vtbl, 0x00dcaec6_2ac0_43a9_acf9_1e36c139b10d);
windows_core::imp::interface_hierarchy!(ICLRDebugManager, windows_core::IUnknown);
impl ICLRDebugManager {
    pub unsafe fn BeginConnection<P1>(&self, dwconnectionid: u32, szconnectionname: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginConnection)(windows_core::Interface::as_raw(self), dwconnectionid, szconnectionname.param().abi()).ok() }
    }
    pub unsafe fn SetConnectionTasks(&self, id: u32, ppclrtask: &[Option<ICLRTask>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectionTasks)(windows_core::Interface::as_raw(self), id, ppclrtask.len().try_into().unwrap(), core::mem::transmute(ppclrtask.as_ptr())).ok() }
    }
    pub unsafe fn EndConnection(&self, dwconnectionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndConnection)(windows_core::Interface::as_raw(self), dwconnectionid).ok() }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn SetDacl(&self, pacl: *const super::super::Security::ACL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDacl)(windows_core::Interface::as_raw(self), pacl).ok() }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetDacl(&self) -> windows_core::Result<*mut super::super::Security::ACL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDacl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDebuggerAttached(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDebuggerAttached)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSymbolReadingPolicy(&self, policy: ESymbolReadingPolicy) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSymbolReadingPolicy)(windows_core::Interface::as_raw(self), policy).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDebugManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetConnectionTasks: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub SetDacl: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::ACL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    SetDacl: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetDacl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::Security::ACL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetDacl: usize,
    pub IsDebuggerAttached: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetSymbolReadingPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, ESymbolReadingPolicy) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Security")]
pub trait ICLRDebugManager_Impl: windows_core::IUnknownImpl {
    fn BeginConnection(&self, dwconnectionid: u32, szconnectionname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetConnectionTasks(&self, id: u32, dwcount: u32, ppclrtask: *const Option<ICLRTask>) -> windows_core::Result<()>;
    fn EndConnection(&self, dwconnectionid: u32) -> windows_core::Result<()>;
    fn SetDacl(&self, pacl: *const super::super::Security::ACL) -> windows_core::Result<()>;
    fn GetDacl(&self) -> windows_core::Result<*mut super::super::Security::ACL>;
    fn IsDebuggerAttached(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetSymbolReadingPolicy(&self, policy: ESymbolReadingPolicy) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Security")]
impl ICLRDebugManager_Vtbl {
    pub const fn new<Identity: ICLRDebugManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginConnection<Identity: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnectionid: u32, szconnectionname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDebugManager_Impl::BeginConnection(this, core::mem::transmute_copy(&dwconnectionid), core::mem::transmute(&szconnectionname)).into()
            }
        }
        unsafe extern "system" fn SetConnectionTasks<Identity: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u32, dwcount: u32, ppclrtask: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDebugManager_Impl::SetConnectionTasks(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&ppclrtask)).into()
            }
        }
        unsafe extern "system" fn EndConnection<Identity: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnectionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDebugManager_Impl::EndConnection(this, core::mem::transmute_copy(&dwconnectionid)).into()
            }
        }
        unsafe extern "system" fn SetDacl<Identity: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacl: *const super::super::Security::ACL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDebugManager_Impl::SetDacl(this, core::mem::transmute_copy(&pacl)).into()
            }
        }
        unsafe extern "system" fn GetDacl<Identity: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacl: *mut *mut super::super::Security::ACL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRDebugManager_Impl::GetDacl(this) {
                    Ok(ok__) => {
                        pacl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDebuggerAttached<Identity: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbattached: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRDebugManager_Impl::IsDebuggerAttached(this) {
                    Ok(ok__) => {
                        pbattached.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSymbolReadingPolicy<Identity: ICLRDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: ESymbolReadingPolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDebugManager_Impl::SetSymbolReadingPolicy(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginConnection: BeginConnection::<Identity, OFFSET>,
            SetConnectionTasks: SetConnectionTasks::<Identity, OFFSET>,
            EndConnection: EndConnection::<Identity, OFFSET>,
            SetDacl: SetDacl::<Identity, OFFSET>,
            GetDacl: GetDacl::<Identity, OFFSET>,
            IsDebuggerAttached: IsDebuggerAttached::<Identity, OFFSET>,
            SetSymbolReadingPolicy: SetSymbolReadingPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDebugManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for ICLRDebugManager {}
windows_core::imp::define_interface!(ICLRDebugging, ICLRDebugging_Vtbl, 0xd28f3c5a_9634_4206_a509_477552eefb10);
windows_core::imp::interface_hierarchy!(ICLRDebugging, windows_core::IUnknown);
impl ICLRDebugging {
    pub unsafe fn OpenVirtualProcess<P1, P2>(&self, modulebaseaddress: u64, pdatatarget: P1, plibraryprovider: P2, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const windows_core::GUID, ppprocess: *mut Option<windows_core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<ICLRDebuggingLibraryProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenVirtualProcess)(windows_core::Interface::as_raw(self), modulebaseaddress, pdatatarget.param().abi(), plibraryprovider.param().abi(), pmaxdebuggersupportedversion, riidprocess, core::mem::transmute(ppprocess), pversion as _, pdwflags as _).ok() }
    }
    pub unsafe fn CanUnloadNow(&self, hmodule: super::super::Foundation::HMODULE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CanUnloadNow)(windows_core::Interface::as_raw(self), hmodule).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDebugging_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenVirtualProcess: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, *mut core::ffi::c_void, *const CLR_DEBUGGING_VERSION, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut CLR_DEBUGGING_VERSION, *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::HRESULT,
    pub CanUnloadNow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HMODULE) -> windows_core::HRESULT,
}
pub trait ICLRDebugging_Impl: windows_core::IUnknownImpl {
    fn OpenVirtualProcess(&self, modulebaseaddress: u64, pdatatarget: windows_core::Ref<windows_core::IUnknown>, plibraryprovider: windows_core::Ref<ICLRDebuggingLibraryProvider>, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const windows_core::GUID, ppprocess: windows_core::OutRef<windows_core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::Result<()>;
    fn CanUnloadNow(&self, hmodule: super::super::Foundation::HMODULE) -> windows_core::Result<()>;
}
impl ICLRDebugging_Vtbl {
    pub const fn new<Identity: ICLRDebugging_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenVirtualProcess<Identity: ICLRDebugging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modulebaseaddress: u64, pdatatarget: *mut core::ffi::c_void, plibraryprovider: *mut core::ffi::c_void, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const windows_core::GUID, ppprocess: *mut *mut core::ffi::c_void, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDebugging_Impl::OpenVirtualProcess(this, core::mem::transmute_copy(&modulebaseaddress), core::mem::transmute_copy(&pdatatarget), core::mem::transmute_copy(&plibraryprovider), core::mem::transmute_copy(&pmaxdebuggersupportedversion), core::mem::transmute_copy(&riidprocess), core::mem::transmute_copy(&ppprocess), core::mem::transmute_copy(&pversion), core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        unsafe extern "system" fn CanUnloadNow<Identity: ICLRDebugging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmodule: super::super::Foundation::HMODULE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDebugging_Impl::CanUnloadNow(this, core::mem::transmute_copy(&hmodule)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenVirtualProcess: OpenVirtualProcess::<Identity, OFFSET>,
            CanUnloadNow: CanUnloadNow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDebugging as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRDebugging {}
windows_core::imp::define_interface!(ICLRDebuggingLibraryProvider, ICLRDebuggingLibraryProvider_Vtbl, 0x3151c08d_4d09_4f9b_8838_2880bf18fe51);
windows_core::imp::interface_hierarchy!(ICLRDebuggingLibraryProvider, windows_core::IUnknown);
impl ICLRDebuggingLibraryProvider {
    pub unsafe fn ProvideLibrary<P0>(&self, pwszfilename: P0, dwtimestamp: u32, dwsizeofimage: u32) -> windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProvideLibrary)(windows_core::Interface::as_raw(self), pwszfilename.param().abi(), dwtimestamp, dwsizeofimage, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDebuggingLibraryProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProvideLibrary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT,
}
pub trait ICLRDebuggingLibraryProvider_Impl: windows_core::IUnknownImpl {
    fn ProvideLibrary(&self, pwszfilename: &windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32) -> windows_core::Result<super::super::Foundation::HMODULE>;
}
impl ICLRDebuggingLibraryProvider_Vtbl {
    pub const fn new<Identity: ICLRDebuggingLibraryProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProvideLibrary<Identity: ICLRDebuggingLibraryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRDebuggingLibraryProvider_Impl::ProvideLibrary(this, core::mem::transmute(&pwszfilename), core::mem::transmute_copy(&dwtimestamp), core::mem::transmute_copy(&dwsizeofimage)) {
                    Ok(ok__) => {
                        phmodule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideLibrary: ProvideLibrary::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDebuggingLibraryProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRDebuggingLibraryProvider {}
windows_core::imp::define_interface!(ICLRDomainManager, ICLRDomainManager_Vtbl, 0x270d00a2_8e15_4d0b_adeb_37bc3e47df77);
windows_core::imp::interface_hierarchy!(ICLRDomainManager, windows_core::IUnknown);
impl ICLRDomainManager {
    pub unsafe fn SetAppDomainManagerType<P0, P1>(&self, wszappdomainmanagerassembly: P0, wszappdomainmanagertype: P1, dwinitializedomainflags: EInitializeNewDomainFlags) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAppDomainManagerType)(windows_core::Interface::as_raw(self), wszappdomainmanagerassembly.param().abi(), wszappdomainmanagertype.param().abi(), dwinitializedomainflags).ok() }
    }
    pub unsafe fn SetPropertiesForDefaultAppDomain(&self, nproperties: u32, pwszpropertynames: *const windows_core::PCWSTR, pwszpropertyvalues: *const windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropertiesForDefaultAppDomain)(windows_core::Interface::as_raw(self), nproperties, pwszpropertynames, pwszpropertyvalues).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRDomainManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAppDomainManagerType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, EInitializeNewDomainFlags) -> windows_core::HRESULT,
    pub SetPropertiesForDefaultAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *const windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICLRDomainManager_Impl: windows_core::IUnknownImpl {
    fn SetAppDomainManagerType(&self, wszappdomainmanagerassembly: &windows_core::PCWSTR, wszappdomainmanagertype: &windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> windows_core::Result<()>;
    fn SetPropertiesForDefaultAppDomain(&self, nproperties: u32, pwszpropertynames: *const windows_core::PCWSTR, pwszpropertyvalues: *const windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICLRDomainManager_Vtbl {
    pub const fn new<Identity: ICLRDomainManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ICLRDomainManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszappdomainmanagerassembly: windows_core::PCWSTR, wszappdomainmanagertype: windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDomainManager_Impl::SetAppDomainManagerType(this, core::mem::transmute(&wszappdomainmanagerassembly), core::mem::transmute(&wszappdomainmanagertype), core::mem::transmute_copy(&dwinitializedomainflags)).into()
            }
        }
        unsafe extern "system" fn SetPropertiesForDefaultAppDomain<Identity: ICLRDomainManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperties: u32, pwszpropertynames: *const windows_core::PCWSTR, pwszpropertyvalues: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRDomainManager_Impl::SetPropertiesForDefaultAppDomain(this, core::mem::transmute_copy(&nproperties), core::mem::transmute_copy(&pwszpropertynames), core::mem::transmute_copy(&pwszpropertyvalues)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, OFFSET>,
            SetPropertiesForDefaultAppDomain: SetPropertiesForDefaultAppDomain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDomainManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRDomainManager {}
windows_core::imp::define_interface!(ICLRErrorReportingManager, ICLRErrorReportingManager_Vtbl, 0x980d2f1a_bf79_4c08_812a_bb9778928f78);
windows_core::imp::interface_hierarchy!(ICLRErrorReportingManager, windows_core::IUnknown);
impl ICLRErrorReportingManager {
    pub unsafe fn GetBucketParametersForCurrentException(&self, pparams: *mut BucketParameters) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBucketParametersForCurrentException)(windows_core::Interface::as_raw(self), pparams as _).ok() }
    }
    pub unsafe fn BeginCustomDump(&self, dwflavor: ECustomDumpFlavor, items: &[CustomDumpItem], dwreserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginCustomDump)(windows_core::Interface::as_raw(self), dwflavor, items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr()), dwreserved).ok() }
    }
    pub unsafe fn EndCustomDump(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndCustomDump)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRErrorReportingManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBucketParametersForCurrentException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BucketParameters) -> windows_core::HRESULT,
    pub BeginCustomDump: unsafe extern "system" fn(*mut core::ffi::c_void, ECustomDumpFlavor, u32, *const CustomDumpItem, u32) -> windows_core::HRESULT,
    pub EndCustomDump: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICLRErrorReportingManager_Impl: windows_core::IUnknownImpl {
    fn GetBucketParametersForCurrentException(&self, pparams: *mut BucketParameters) -> windows_core::Result<()>;
    fn BeginCustomDump(&self, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> windows_core::Result<()>;
    fn EndCustomDump(&self) -> windows_core::Result<()>;
}
impl ICLRErrorReportingManager_Vtbl {
    pub const fn new<Identity: ICLRErrorReportingManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBucketParametersForCurrentException<Identity: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *mut BucketParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRErrorReportingManager_Impl::GetBucketParametersForCurrentException(this, core::mem::transmute_copy(&pparams)).into()
            }
        }
        unsafe extern "system" fn BeginCustomDump<Identity: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRErrorReportingManager_Impl::BeginCustomDump(this, core::mem::transmute_copy(&dwflavor), core::mem::transmute_copy(&dwnumitems), core::mem::transmute_copy(&items), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn EndCustomDump<Identity: ICLRErrorReportingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRErrorReportingManager_Impl::EndCustomDump(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBucketParametersForCurrentException: GetBucketParametersForCurrentException::<Identity, OFFSET>,
            BeginCustomDump: BeginCustomDump::<Identity, OFFSET>,
            EndCustomDump: EndCustomDump::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRErrorReportingManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRErrorReportingManager {}
windows_core::imp::define_interface!(ICLRGCManager, ICLRGCManager_Vtbl, 0x54d9007e_a8e2_4885_b7bf_f998deee4f2a);
windows_core::imp::interface_hierarchy!(ICLRGCManager, windows_core::IUnknown);
impl ICLRGCManager {
    pub unsafe fn Collect(&self, generation: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Collect)(windows_core::Interface::as_raw(self), generation).ok() }
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStats)(windows_core::Interface::as_raw(self), pstats as _).ok() }
    }
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGCStartupLimits)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRGCManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Collect: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_GC_STATS) -> windows_core::HRESULT,
    pub SetGCStartupLimits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait ICLRGCManager_Impl: windows_core::IUnknownImpl {
    fn Collect(&self, generation: i32) -> windows_core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()>;
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()>;
}
impl ICLRGCManager_Vtbl {
    pub const fn new<Identity: ICLRGCManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Collect<Identity: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRGCManager_Impl::Collect(this, core::mem::transmute_copy(&generation)).into()
            }
        }
        unsafe extern "system" fn GetStats<Identity: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut COR_GC_STATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRGCManager_Impl::GetStats(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn SetGCStartupLimits<Identity: ICLRGCManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRGCManager_Impl::SetGCStartupLimits(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Collect: Collect::<Identity, OFFSET>,
            GetStats: GetStats::<Identity, OFFSET>,
            SetGCStartupLimits: SetGCStartupLimits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRGCManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRGCManager {}
windows_core::imp::define_interface!(ICLRGCManager2, ICLRGCManager2_Vtbl, 0x0603b793_a97a_4712_9cb4_0cd1c74c0f7c);
impl core::ops::Deref for ICLRGCManager2 {
    type Target = ICLRGCManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRGCManager2, windows_core::IUnknown, ICLRGCManager);
impl ICLRGCManager2 {
    pub unsafe fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGCStartupLimitsEx)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRGCManager2_Vtbl {
    pub base__: ICLRGCManager_Vtbl,
    pub SetGCStartupLimitsEx: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
pub trait ICLRGCManager2_Impl: ICLRGCManager_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> windows_core::Result<()>;
}
impl ICLRGCManager2_Vtbl {
    pub const fn new<Identity: ICLRGCManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: ICLRGCManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRGCManager2_Impl::SetGCStartupLimitsEx(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
            }
        }
        Self { base__: ICLRGCManager_Vtbl::new::<Identity, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRGCManager2 as windows_core::Interface>::IID || iid == &<ICLRGCManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRGCManager2 {}
windows_core::imp::define_interface!(ICLRHostBindingPolicyManager, ICLRHostBindingPolicyManager_Vtbl, 0x4b3545e7_1856_48c9_a8ba_24b21a753c09);
windows_core::imp::interface_hierarchy!(ICLRHostBindingPolicyManager, windows_core::IUnknown);
impl ICLRHostBindingPolicyManager {
    pub unsafe fn ModifyApplicationPolicy<P0, P1>(&self, pwzsourceassemblyidentity: P0, pwztargetassemblyidentity: P1, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ModifyApplicationPolicy)(windows_core::Interface::as_raw(self), pwzsourceassemblyidentity.param().abi(), pwztargetassemblyidentity.param().abi(), pbapplicationpolicy, cbapppolicysize, dwpolicymodifyflags, pbnewapplicationpolicy as _, pcbnewapppolicysize as _).ok() }
    }
    pub unsafe fn EvaluatePolicy<P0>(&self, pwzreferenceidentity: P0, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EvaluatePolicy)(windows_core::Interface::as_raw(self), pwzreferenceidentity.param().abi(), pbapplicationpolicy, cbapppolicysize, core::mem::transmute(pwzpostpolicyreferenceidentity), pcchpostpolicyreferenceidentity as _, pdwpoliciesapplied as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRHostBindingPolicyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ModifyApplicationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub EvaluatePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, windows_core::PWSTR, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait ICLRHostBindingPolicyManager_Impl: windows_core::IUnknownImpl {
    fn ModifyApplicationPolicy(&self, pwzsourceassemblyidentity: &windows_core::PCWSTR, pwztargetassemblyidentity: &windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> windows_core::Result<()>;
    fn EvaluatePolicy(&self, pwzreferenceidentity: &windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> windows_core::Result<()>;
}
impl ICLRHostBindingPolicyManager_Vtbl {
    pub const fn new<Identity: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModifyApplicationPolicy<Identity: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzsourceassemblyidentity: windows_core::PCWSTR, pwztargetassemblyidentity: windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRHostBindingPolicyManager_Impl::ModifyApplicationPolicy(this, core::mem::transmute(&pwzsourceassemblyidentity), core::mem::transmute(&pwztargetassemblyidentity), core::mem::transmute_copy(&pbapplicationpolicy), core::mem::transmute_copy(&cbapppolicysize), core::mem::transmute_copy(&dwpolicymodifyflags), core::mem::transmute_copy(&pbnewapplicationpolicy), core::mem::transmute_copy(&pcbnewapppolicysize)).into()
            }
        }
        unsafe extern "system" fn EvaluatePolicy<Identity: ICLRHostBindingPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzreferenceidentity: windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRHostBindingPolicyManager_Impl::EvaluatePolicy(this, core::mem::transmute(&pwzreferenceidentity), core::mem::transmute_copy(&pbapplicationpolicy), core::mem::transmute_copy(&cbapppolicysize), core::mem::transmute_copy(&pwzpostpolicyreferenceidentity), core::mem::transmute_copy(&pcchpostpolicyreferenceidentity), core::mem::transmute_copy(&pdwpoliciesapplied)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ModifyApplicationPolicy: ModifyApplicationPolicy::<Identity, OFFSET>,
            EvaluatePolicy: EvaluatePolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRHostBindingPolicyManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRHostBindingPolicyManager {}
windows_core::imp::define_interface!(ICLRHostProtectionManager, ICLRHostProtectionManager_Vtbl, 0x89f25f5c_ceef_43e1_9cfa_a68ce863aaac);
windows_core::imp::interface_hierarchy!(ICLRHostProtectionManager, windows_core::IUnknown);
impl ICLRHostProtectionManager {
    pub unsafe fn SetProtectedCategories(&self, categories: EApiCategories) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProtectedCategories)(windows_core::Interface::as_raw(self), categories).ok() }
    }
    pub unsafe fn SetEagerSerializeGrantSets(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEagerSerializeGrantSets)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRHostProtectionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProtectedCategories: unsafe extern "system" fn(*mut core::ffi::c_void, EApiCategories) -> windows_core::HRESULT,
    pub SetEagerSerializeGrantSets: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICLRHostProtectionManager_Impl: windows_core::IUnknownImpl {
    fn SetProtectedCategories(&self, categories: EApiCategories) -> windows_core::Result<()>;
    fn SetEagerSerializeGrantSets(&self) -> windows_core::Result<()>;
}
impl ICLRHostProtectionManager_Vtbl {
    pub const fn new<Identity: ICLRHostProtectionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProtectedCategories<Identity: ICLRHostProtectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categories: EApiCategories) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRHostProtectionManager_Impl::SetProtectedCategories(this, core::mem::transmute_copy(&categories)).into()
            }
        }
        unsafe extern "system" fn SetEagerSerializeGrantSets<Identity: ICLRHostProtectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRHostProtectionManager_Impl::SetEagerSerializeGrantSets(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProtectedCategories: SetProtectedCategories::<Identity, OFFSET>,
            SetEagerSerializeGrantSets: SetEagerSerializeGrantSets::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRHostProtectionManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRHostProtectionManager {}
windows_core::imp::define_interface!(ICLRIoCompletionManager, ICLRIoCompletionManager_Vtbl, 0x2d74ce86_b8d6_4c84_b3a7_9768933b3c12);
windows_core::imp::interface_hierarchy!(ICLRIoCompletionManager, windows_core::IUnknown);
impl ICLRIoCompletionManager {
    pub unsafe fn OnComplete(&self, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnComplete)(windows_core::Interface::as_raw(self), dwerrorcode, numberofbytestransferred, pvoverlapped).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRIoCompletionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICLRIoCompletionManager_Impl: windows_core::IUnknownImpl {
    fn OnComplete(&self, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICLRIoCompletionManager_Vtbl {
    pub const fn new<Identity: ICLRIoCompletionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnComplete<Identity: ICLRIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRIoCompletionManager_Impl::OnComplete(this, core::mem::transmute_copy(&dwerrorcode), core::mem::transmute_copy(&numberofbytestransferred), core::mem::transmute_copy(&pvoverlapped)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRIoCompletionManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRIoCompletionManager {}
windows_core::imp::define_interface!(ICLRMemoryNotificationCallback, ICLRMemoryNotificationCallback_Vtbl, 0x47eb8e57_0846_4546_af76_6f42fcfc2649);
windows_core::imp::interface_hierarchy!(ICLRMemoryNotificationCallback, windows_core::IUnknown);
impl ICLRMemoryNotificationCallback {
    pub unsafe fn OnMemoryNotification(&self, ememoryavailable: EMemoryAvailable) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMemoryNotification)(windows_core::Interface::as_raw(self), ememoryavailable).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRMemoryNotificationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMemoryNotification: unsafe extern "system" fn(*mut core::ffi::c_void, EMemoryAvailable) -> windows_core::HRESULT,
}
pub trait ICLRMemoryNotificationCallback_Impl: windows_core::IUnknownImpl {
    fn OnMemoryNotification(&self, ememoryavailable: EMemoryAvailable) -> windows_core::Result<()>;
}
impl ICLRMemoryNotificationCallback_Vtbl {
    pub const fn new<Identity: ICLRMemoryNotificationCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMemoryNotification<Identity: ICLRMemoryNotificationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ememoryavailable: EMemoryAvailable) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRMemoryNotificationCallback_Impl::OnMemoryNotification(this, core::mem::transmute_copy(&ememoryavailable)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMemoryNotification: OnMemoryNotification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRMemoryNotificationCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRMemoryNotificationCallback {}
windows_core::imp::define_interface!(ICLRMetaHost, ICLRMetaHost_Vtbl, 0xd332db9e_b9b3_4125_8207_a14884f53216);
windows_core::imp::interface_hierarchy!(ICLRMetaHost, windows_core::IUnknown);
impl ICLRMetaHost {
    pub unsafe fn GetRuntime<P0, T>(&self, pwzversion: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRuntime)(windows_core::Interface::as_raw(self), pwzversion.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetVersionFromFile<P0>(&self, pwzfilepath: P0, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetVersionFromFile)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), core::mem::transmute(pwzbuffer), pcchbuffer as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateInstalledRuntimes(&self) -> windows_core::Result<super::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateInstalledRuntimes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateLoadedRuntimes(&self, hndprocess: super::super::Foundation::HANDLE) -> windows_core::Result<super::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateLoadedRuntimes)(windows_core::Interface::as_raw(self), hndprocess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RequestRuntimeLoadedNotification(&self, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestRuntimeLoadedNotification)(windows_core::Interface::as_raw(self), pcallbackfunction).ok() }
    }
    pub unsafe fn QueryLegacyV2RuntimeBinding<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).QueryLegacyV2RuntimeBinding)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn ExitProcess(&self, iexitcode: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExitProcess)(windows_core::Interface::as_raw(self), iexitcode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRMetaHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRuntime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVersionFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateInstalledRuntimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateInstalledRuntimes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateLoadedRuntimes: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateLoadedRuntimes: usize,
    pub RequestRuntimeLoadedNotification: unsafe extern "system" fn(*mut core::ffi::c_void, RuntimeLoadedCallbackFnPtr) -> windows_core::HRESULT,
    pub QueryLegacyV2RuntimeBinding: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExitProcess: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRMetaHost_Impl: windows_core::IUnknownImpl {
    fn GetRuntime(&self, pwzversion: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetVersionFromFile(&self, pwzfilepath: &windows_core::PCWSTR, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>;
    fn EnumerateInstalledRuntimes(&self) -> windows_core::Result<super::Com::IEnumUnknown>;
    fn EnumerateLoadedRuntimes(&self, hndprocess: super::super::Foundation::HANDLE) -> windows_core::Result<super::Com::IEnumUnknown>;
    fn RequestRuntimeLoadedNotification(&self, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> windows_core::Result<()>;
    fn QueryLegacyV2RuntimeBinding(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ExitProcess(&self, iexitcode: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ICLRMetaHost_Vtbl {
    pub const fn new<Identity: ICLRMetaHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRuntime<Identity: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzversion: windows_core::PCWSTR, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRMetaHost_Impl::GetRuntime(this, core::mem::transmute(&pwzversion), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppruntime)).into()
            }
        }
        unsafe extern "system" fn GetVersionFromFile<Identity: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRMetaHost_Impl::GetVersionFromFile(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer)).into()
            }
        }
        unsafe extern "system" fn EnumerateInstalledRuntimes<Identity: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRMetaHost_Impl::EnumerateInstalledRuntimes(this) {
                    Ok(ok__) => {
                        ppenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateLoadedRuntimes<Identity: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRMetaHost_Impl::EnumerateLoadedRuntimes(this, core::mem::transmute_copy(&hndprocess)) {
                    Ok(ok__) => {
                        ppenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestRuntimeLoadedNotification<Identity: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRMetaHost_Impl::RequestRuntimeLoadedNotification(this, core::mem::transmute_copy(&pcallbackfunction)).into()
            }
        }
        unsafe extern "system" fn QueryLegacyV2RuntimeBinding<Identity: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRMetaHost_Impl::QueryLegacyV2RuntimeBinding(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn ExitProcess<Identity: ICLRMetaHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iexitcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRMetaHost_Impl::ExitProcess(this, core::mem::transmute_copy(&iexitcode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRuntime: GetRuntime::<Identity, OFFSET>,
            GetVersionFromFile: GetVersionFromFile::<Identity, OFFSET>,
            EnumerateInstalledRuntimes: EnumerateInstalledRuntimes::<Identity, OFFSET>,
            EnumerateLoadedRuntimes: EnumerateLoadedRuntimes::<Identity, OFFSET>,
            RequestRuntimeLoadedNotification: RequestRuntimeLoadedNotification::<Identity, OFFSET>,
            QueryLegacyV2RuntimeBinding: QueryLegacyV2RuntimeBinding::<Identity, OFFSET>,
            ExitProcess: ExitProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRMetaHost as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICLRMetaHost {}
windows_core::imp::define_interface!(ICLRMetaHostPolicy, ICLRMetaHostPolicy_Vtbl, 0xe2190695_77b2_492e_8e14_c4b3a7fdd593);
windows_core::imp::interface_hierarchy!(ICLRMetaHostPolicy, windows_core::IUnknown);
impl ICLRMetaHostPolicy {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRequestedRuntime<P1, P2, T>(&self, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: P1, pcfgstream: P2, pwzversion: Option<windows_core::PWSTR>, pcchversion: *mut u32, pwzimageversion: Option<windows_core::PWSTR>, pcchimageversion: *mut u32, pdwconfigflags: *mut u32) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::Com::IStream>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRequestedRuntime)(windows_core::Interface::as_raw(self), dwpolicyflags, pwzbinary.param().abi(), pcfgstream.param().abi(), pwzversion.unwrap_or(core::mem::zeroed()) as _, pcchversion as _, pwzimageversion.unwrap_or(core::mem::zeroed()) as _, pcchimageversion as _, pdwconfigflags as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRMetaHostPolicy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRequestedRuntime: unsafe extern "system" fn(*mut core::ffi::c_void, METAHOST_POLICY_FLAGS, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PWSTR, *mut u32, windows_core::PWSTR, *mut u32, *mut u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRequestedRuntime: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRMetaHostPolicy_Impl: windows_core::IUnknownImpl {
    fn GetRequestedRuntime(&self, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: &windows_core::PCWSTR, pcfgstream: windows_core::Ref<super::Com::IStream>, pwzversion: windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ICLRMetaHostPolicy_Vtbl {
    pub const fn new<Identity: ICLRMetaHostPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRequestedRuntime<Identity: ICLRMetaHostPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: windows_core::PCWSTR, pcfgstream: *mut core::ffi::c_void, pwzversion: windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRMetaHostPolicy_Impl::GetRequestedRuntime(this, core::mem::transmute_copy(&dwpolicyflags), core::mem::transmute(&pwzbinary), core::mem::transmute_copy(&pcfgstream), core::mem::transmute_copy(&pwzversion), core::mem::transmute_copy(&pcchversion), core::mem::transmute_copy(&pwzimageversion), core::mem::transmute_copy(&pcchimageversion), core::mem::transmute_copy(&pdwconfigflags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppruntime)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRequestedRuntime: GetRequestedRuntime::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRMetaHostPolicy as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICLRMetaHostPolicy {}
windows_core::imp::define_interface!(ICLROnEventManager, ICLROnEventManager_Vtbl, 0x1d0e0132_e64f_493d_9260_025c0e32c175);
windows_core::imp::interface_hierarchy!(ICLROnEventManager, windows_core::IUnknown);
impl ICLROnEventManager {
    pub unsafe fn RegisterActionOnEvent<P1>(&self, event: EClrEvent, paction: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IActionOnCLREvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterActionOnEvent)(windows_core::Interface::as_raw(self), event, paction.param().abi()).ok() }
    }
    pub unsafe fn UnregisterActionOnEvent<P1>(&self, event: EClrEvent, paction: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IActionOnCLREvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterActionOnEvent)(windows_core::Interface::as_raw(self), event, paction.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLROnEventManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterActionOnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, EClrEvent, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterActionOnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, EClrEvent, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICLROnEventManager_Impl: windows_core::IUnknownImpl {
    fn RegisterActionOnEvent(&self, event: EClrEvent, paction: windows_core::Ref<IActionOnCLREvent>) -> windows_core::Result<()>;
    fn UnregisterActionOnEvent(&self, event: EClrEvent, paction: windows_core::Ref<IActionOnCLREvent>) -> windows_core::Result<()>;
}
impl ICLROnEventManager_Vtbl {
    pub const fn new<Identity: ICLROnEventManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterActionOnEvent<Identity: ICLROnEventManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: EClrEvent, paction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLROnEventManager_Impl::RegisterActionOnEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&paction)).into()
            }
        }
        unsafe extern "system" fn UnregisterActionOnEvent<Identity: ICLROnEventManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: EClrEvent, paction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLROnEventManager_Impl::UnregisterActionOnEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&paction)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterActionOnEvent: RegisterActionOnEvent::<Identity, OFFSET>,
            UnregisterActionOnEvent: UnregisterActionOnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLROnEventManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLROnEventManager {}
windows_core::imp::define_interface!(ICLRPolicyManager, ICLRPolicyManager_Vtbl, 0x7d290010_d781_45da_a6f8_aa5d711a730e);
windows_core::imp::interface_hierarchy!(ICLRPolicyManager, windows_core::IUnknown);
impl ICLRPolicyManager {
    pub unsafe fn SetDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultAction)(windows_core::Interface::as_raw(self), operation, action).ok() }
    }
    pub unsafe fn SetTimeout(&self, operation: EClrOperation, dwmilliseconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimeout)(windows_core::Interface::as_raw(self), operation, dwmilliseconds).ok() }
    }
    pub unsafe fn SetActionOnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActionOnTimeout)(windows_core::Interface::as_raw(self), operation, action).ok() }
    }
    pub unsafe fn SetTimeoutAndAction(&self, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimeoutAndAction)(windows_core::Interface::as_raw(self), operation, dwmilliseconds, action).ok() }
    }
    pub unsafe fn SetActionOnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActionOnFailure)(windows_core::Interface::as_raw(self), failure, action).ok() }
    }
    pub unsafe fn SetUnhandledExceptionPolicy(&self, policy: EClrUnhandledException) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUnhandledExceptionPolicy)(windows_core::Interface::as_raw(self), policy).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRPolicyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, u32) -> windows_core::HRESULT,
    pub SetActionOnTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub SetTimeoutAndAction: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, u32, EPolicyAction) -> windows_core::HRESULT,
    pub SetActionOnFailure: unsafe extern "system" fn(*mut core::ffi::c_void, EClrFailure, EPolicyAction) -> windows_core::HRESULT,
    pub SetUnhandledExceptionPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, EClrUnhandledException) -> windows_core::HRESULT,
}
pub trait ICLRPolicyManager_Impl: windows_core::IUnknownImpl {
    fn SetDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetTimeout(&self, operation: EClrOperation, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn SetActionOnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetTimeoutAndAction(&self, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetActionOnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetUnhandledExceptionPolicy(&self, policy: EClrUnhandledException) -> windows_core::Result<()>;
}
impl ICLRPolicyManager_Vtbl {
    pub const fn new<Identity: ICLRPolicyManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDefaultAction<Identity: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRPolicyManager_Impl::SetDefaultAction(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn SetTimeout<Identity: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRPolicyManager_Impl::SetTimeout(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn SetActionOnTimeout<Identity: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRPolicyManager_Impl::SetActionOnTimeout(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn SetTimeoutAndAction<Identity: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRPolicyManager_Impl::SetTimeoutAndAction(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn SetActionOnFailure<Identity: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRPolicyManager_Impl::SetActionOnFailure(this, core::mem::transmute_copy(&failure), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn SetUnhandledExceptionPolicy<Identity: ICLRPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: EClrUnhandledException) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRPolicyManager_Impl::SetUnhandledExceptionPolicy(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultAction: SetDefaultAction::<Identity, OFFSET>,
            SetTimeout: SetTimeout::<Identity, OFFSET>,
            SetActionOnTimeout: SetActionOnTimeout::<Identity, OFFSET>,
            SetTimeoutAndAction: SetTimeoutAndAction::<Identity, OFFSET>,
            SetActionOnFailure: SetActionOnFailure::<Identity, OFFSET>,
            SetUnhandledExceptionPolicy: SetUnhandledExceptionPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRPolicyManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRPolicyManager {}
windows_core::imp::define_interface!(ICLRProbingAssemblyEnum, ICLRProbingAssemblyEnum_Vtbl, 0xd0c5fb1f_416b_4f97_81f4_7ac7dc24dd5d);
windows_core::imp::interface_hierarchy!(ICLRProbingAssemblyEnum, windows_core::IUnknown);
impl ICLRProbingAssemblyEnum {
    pub unsafe fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pwzbuffer), pcchbuffersize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRProbingAssemblyEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait ICLRProbingAssemblyEnum_Impl: windows_core::IUnknownImpl {
    fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
}
impl ICLRProbingAssemblyEnum_Vtbl {
    pub const fn new<Identity: ICLRProbingAssemblyEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: ICLRProbingAssemblyEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRProbingAssemblyEnum_Impl::Get(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRProbingAssemblyEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRProbingAssemblyEnum {}
windows_core::imp::define_interface!(ICLRProfiling, ICLRProfiling_Vtbl, 0xb349abe3_b56f_4689_bfcd_76bf39d888ea);
windows_core::imp::interface_hierarchy!(ICLRProfiling, windows_core::IUnknown);
impl ICLRProfiling {
    pub unsafe fn AttachProfiler<P3>(&self, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const windows_core::GUID, wszprofilerpath: P3, pvclientdata: &[u8]) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AttachProfiler)(windows_core::Interface::as_raw(self), dwprofileeprocessid, dwmillisecondsmax, pclsidprofiler, wszprofilerpath.param().abi(), core::mem::transmute(pvclientdata.as_ptr()), pvclientdata.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRProfiling_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AttachProfiler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, windows_core::PCWSTR, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ICLRProfiling_Impl: windows_core::IUnknownImpl {
    fn AttachProfiler(&self, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const windows_core::GUID, wszprofilerpath: &windows_core::PCWSTR, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::Result<()>;
}
impl ICLRProfiling_Vtbl {
    pub const fn new<Identity: ICLRProfiling_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachProfiler<Identity: ICLRProfiling_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const windows_core::GUID, wszprofilerpath: windows_core::PCWSTR, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRProfiling_Impl::AttachProfiler(this, core::mem::transmute_copy(&dwprofileeprocessid), core::mem::transmute_copy(&dwmillisecondsmax), core::mem::transmute_copy(&pclsidprofiler), core::mem::transmute(&wszprofilerpath), core::mem::transmute_copy(&pvclientdata), core::mem::transmute_copy(&cbclientdata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AttachProfiler: AttachProfiler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRProfiling as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRProfiling {}
windows_core::imp::define_interface!(ICLRReferenceAssemblyEnum, ICLRReferenceAssemblyEnum_Vtbl, 0xd509cb5d_cf32_4876_ae61_67770cf91973);
windows_core::imp::interface_hierarchy!(ICLRReferenceAssemblyEnum, windows_core::IUnknown);
impl ICLRReferenceAssemblyEnum {
    pub unsafe fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pwzbuffer), pcchbuffersize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRReferenceAssemblyEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait ICLRReferenceAssemblyEnum_Impl: windows_core::IUnknownImpl {
    fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
}
impl ICLRReferenceAssemblyEnum_Vtbl {
    pub const fn new<Identity: ICLRReferenceAssemblyEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: ICLRReferenceAssemblyEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRReferenceAssemblyEnum_Impl::Get(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRReferenceAssemblyEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRReferenceAssemblyEnum {}
windows_core::imp::define_interface!(ICLRRuntimeHost, ICLRRuntimeHost_Vtbl, 0x90f1a06c_7712_4762_86b5_7a5eba6bdb02);
windows_core::imp::interface_hierarchy!(ICLRRuntimeHost, windows_core::IUnknown);
impl ICLRRuntimeHost {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetHostControl<P0>(&self, phostcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IHostControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHostControl)(windows_core::Interface::as_raw(self), phostcontrol.param().abi()).ok() }
    }
    pub unsafe fn GetCLRControl(&self) -> windows_core::Result<ICLRControl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCLRControl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UnloadAppDomain(&self, dwappdomainid: u32, fwaituntildone: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnloadAppDomain)(windows_core::Interface::as_raw(self), dwappdomainid, fwaituntildone.into()).ok() }
    }
    pub unsafe fn ExecuteInAppDomain(&self, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecuteInAppDomain)(windows_core::Interface::as_raw(self), dwappdomainid, pcallback, cookie).ok() }
    }
    pub unsafe fn GetCurrentAppDomainId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentAppDomainId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExecuteApplication<P0>(&self, pwzappfullname: P0, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecuteApplication)(windows_core::Interface::as_raw(self), pwzappfullname.param().abi(), dwmanifestpaths, ppwzmanifestpaths, dwactivationdata, ppwzactivationdata, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExecuteInDefaultAppDomain<P0, P1, P2, P3>(&self, pwzassemblypath: P0, pwztypename: P1, pwzmethodname: P2, pwzargument: P3) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecuteInDefaultAppDomain)(windows_core::Interface::as_raw(self), pwzassemblypath.param().abi(), pwztypename.param().abi(), pwzmethodname.param().abi(), pwzargument.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRRuntimeHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHostControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCLRControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnloadAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub ExecuteInAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, FExecuteInAppDomainCallback, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentAppDomainId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExecuteApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, u32, *const windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub ExecuteInDefaultAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait ICLRRuntimeHost_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn SetHostControl(&self, phostcontrol: windows_core::Ref<IHostControl>) -> windows_core::Result<()>;
    fn GetCLRControl(&self) -> windows_core::Result<ICLRControl>;
    fn UnloadAppDomain(&self, dwappdomainid: u32, fwaituntildone: windows_core::BOOL) -> windows_core::Result<()>;
    fn ExecuteInAppDomain(&self, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCurrentAppDomainId(&self) -> windows_core::Result<u32>;
    fn ExecuteApplication(&self, pwzappfullname: &windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR) -> windows_core::Result<i32>;
    fn ExecuteInDefaultAppDomain(&self, pwzassemblypath: &windows_core::PCWSTR, pwztypename: &windows_core::PCWSTR, pwzmethodname: &windows_core::PCWSTR, pwzargument: &windows_core::PCWSTR) -> windows_core::Result<u32>;
}
impl ICLRRuntimeHost_Vtbl {
    pub const fn new<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeHost_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeHost_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn SetHostControl<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phostcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeHost_Impl::SetHostControl(this, core::mem::transmute_copy(&phostcontrol)).into()
            }
        }
        unsafe extern "system" fn GetCLRControl<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclrcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeHost_Impl::GetCLRControl(this) {
                    Ok(ok__) => {
                        pclrcontrol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnloadAppDomain<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, fwaituntildone: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeHost_Impl::UnloadAppDomain(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&fwaituntildone)).into()
            }
        }
        unsafe extern "system" fn ExecuteInAppDomain<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeHost_Impl::ExecuteInAppDomain(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&cookie)).into()
            }
        }
        unsafe extern "system" fn GetCurrentAppDomainId<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwappdomainid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeHost_Impl::GetCurrentAppDomainId(this) {
                    Ok(ok__) => {
                        pdwappdomainid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecuteApplication<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzappfullname: windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR, preturnvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeHost_Impl::ExecuteApplication(this, core::mem::transmute(&pwzappfullname), core::mem::transmute_copy(&dwmanifestpaths), core::mem::transmute_copy(&ppwzmanifestpaths), core::mem::transmute_copy(&dwactivationdata), core::mem::transmute_copy(&ppwzactivationdata)) {
                    Ok(ok__) => {
                        preturnvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecuteInDefaultAppDomain<Identity: ICLRRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassemblypath: windows_core::PCWSTR, pwztypename: windows_core::PCWSTR, pwzmethodname: windows_core::PCWSTR, pwzargument: windows_core::PCWSTR, preturnvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeHost_Impl::ExecuteInDefaultAppDomain(this, core::mem::transmute(&pwzassemblypath), core::mem::transmute(&pwztypename), core::mem::transmute(&pwzmethodname), core::mem::transmute(&pwzargument)) {
                    Ok(ok__) => {
                        preturnvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            SetHostControl: SetHostControl::<Identity, OFFSET>,
            GetCLRControl: GetCLRControl::<Identity, OFFSET>,
            UnloadAppDomain: UnloadAppDomain::<Identity, OFFSET>,
            ExecuteInAppDomain: ExecuteInAppDomain::<Identity, OFFSET>,
            GetCurrentAppDomainId: GetCurrentAppDomainId::<Identity, OFFSET>,
            ExecuteApplication: ExecuteApplication::<Identity, OFFSET>,
            ExecuteInDefaultAppDomain: ExecuteInDefaultAppDomain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRRuntimeHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRRuntimeHost {}
windows_core::imp::define_interface!(ICLRRuntimeInfo, ICLRRuntimeInfo_Vtbl, 0xbd39d1d2_ba2f_486a_89b0_b4b0cb466891);
windows_core::imp::interface_hierarchy!(ICLRRuntimeInfo, windows_core::IUnknown);
impl ICLRRuntimeInfo {
    pub unsafe fn GetVersionString(&self, pwzbuffer: Option<windows_core::PWSTR>, pcchbuffer: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVersionString)(windows_core::Interface::as_raw(self), pwzbuffer.unwrap_or(core::mem::zeroed()) as _, pcchbuffer as _).ok() }
    }
    pub unsafe fn GetRuntimeDirectory(&self, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRuntimeDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute(pwzbuffer), pcchbuffer as _).ok() }
    }
    pub unsafe fn IsLoaded(&self, hndprocess: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLoaded)(windows_core::Interface::as_raw(self), hndprocess, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LoadErrorString(&self, iresourceid: u32, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadErrorString)(windows_core::Interface::as_raw(self), iresourceid, core::mem::transmute(pwzbuffer), pcchbuffer as _, ilocaleid).ok() }
    }
    pub unsafe fn LoadLibraryA<P0>(&self, pwzdllname: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadLibraryA)(windows_core::Interface::as_raw(self), pwzdllname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProcAddress<P0>(&self, pszprocname: P0) -> windows_core::Result<*mut core::ffi::c_void>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcAddress)(windows_core::Interface::as_raw(self), pszprocname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInterface<T>(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetInterface)(windows_core::Interface::as_raw(self), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn IsLoadable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLoadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultStartupFlags<P1>(&self, dwstartupflags: u32, pwzhostconfigfile: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultStartupFlags)(windows_core::Interface::as_raw(self), dwstartupflags, pwzhostconfigfile.param().abi()).ok() }
    }
    pub unsafe fn GetDefaultStartupFlags(&self, pdwstartupflags: *mut u32, pwzhostconfigfile: Option<windows_core::PWSTR>, pcchhostconfigfile: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDefaultStartupFlags)(windows_core::Interface::as_raw(self), pdwstartupflags as _, pwzhostconfigfile.unwrap_or(core::mem::zeroed()) as _, pcchhostconfigfile as _).ok() }
    }
    pub unsafe fn BindAsLegacyV2Runtime(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BindAsLegacyV2Runtime)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsStarted(&self, pbstarted: *mut windows_core::BOOL, pdwstartupflags: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsStarted)(windows_core::Interface::as_raw(self), pbstarted as _, pdwstartupflags as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRRuntimeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVersionString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetRuntimeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub IsLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub LoadErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32, i32) -> windows_core::HRESULT,
    pub LoadLibraryA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT,
    pub GetProcAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLoadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetDefaultStartupFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDefaultStartupFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub BindAsLegacyV2Runtime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
}
pub trait ICLRRuntimeInfo_Impl: windows_core::IUnknownImpl {
    fn GetVersionString(&self, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>;
    fn GetRuntimeDirectory(&self, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>;
    fn IsLoaded(&self, hndprocess: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::BOOL>;
    fn LoadErrorString(&self, iresourceid: u32, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> windows_core::Result<()>;
    fn LoadLibraryA(&self, pwzdllname: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetProcAddress(&self, pszprocname: &windows_core::PCSTR) -> windows_core::Result<*mut core::ffi::c_void>;
    fn GetInterface(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsLoadable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetDefaultStartupFlags(&self, dwstartupflags: u32, pwzhostconfigfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDefaultStartupFlags(&self, pdwstartupflags: *mut u32, pwzhostconfigfile: windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> windows_core::Result<()>;
    fn BindAsLegacyV2Runtime(&self) -> windows_core::Result<()>;
    fn IsStarted(&self, pbstarted: *mut windows_core::BOOL, pdwstartupflags: *mut u32) -> windows_core::Result<()>;
}
impl ICLRRuntimeInfo_Vtbl {
    pub const fn new<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersionString<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::GetVersionString(this, core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer)).into()
            }
        }
        unsafe extern "system" fn GetRuntimeDirectory<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::GetRuntimeDirectory(this, core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer)).into()
            }
        }
        unsafe extern "system" fn IsLoaded<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, pbloaded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeInfo_Impl::IsLoaded(this, core::mem::transmute_copy(&hndprocess)) {
                    Ok(ok__) => {
                        pbloaded.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadErrorString<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iresourceid: u32, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::LoadErrorString(this, core::mem::transmute_copy(&iresourceid), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer), core::mem::transmute_copy(&ilocaleid)).into()
            }
        }
        unsafe extern "system" fn LoadLibraryA<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzdllname: windows_core::PCWSTR, phndmodule: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeInfo_Impl::LoadLibraryA(this, core::mem::transmute(&pwzdllname)) {
                    Ok(ok__) => {
                        phndmodule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProcAddress<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprocname: windows_core::PCSTR, ppproc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeInfo_Impl::GetProcAddress(this, core::mem::transmute(&pszprocname)) {
                    Ok(ok__) => {
                        ppproc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInterface<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::GetInterface(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn IsLoadable<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbloadable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRRuntimeInfo_Impl::IsLoadable(this) {
                    Ok(ok__) => {
                        pbloadable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultStartupFlags<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstartupflags: u32, pwzhostconfigfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::SetDefaultStartupFlags(this, core::mem::transmute_copy(&dwstartupflags), core::mem::transmute(&pwzhostconfigfile)).into()
            }
        }
        unsafe extern "system" fn GetDefaultStartupFlags<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstartupflags: *mut u32, pwzhostconfigfile: windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::GetDefaultStartupFlags(this, core::mem::transmute_copy(&pdwstartupflags), core::mem::transmute_copy(&pwzhostconfigfile), core::mem::transmute_copy(&pcchhostconfigfile)).into()
            }
        }
        unsafe extern "system" fn BindAsLegacyV2Runtime<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::BindAsLegacyV2Runtime(this).into()
            }
        }
        unsafe extern "system" fn IsStarted<Identity: ICLRRuntimeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstarted: *mut windows_core::BOOL, pdwstartupflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRRuntimeInfo_Impl::IsStarted(this, core::mem::transmute_copy(&pbstarted), core::mem::transmute_copy(&pdwstartupflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersionString: GetVersionString::<Identity, OFFSET>,
            GetRuntimeDirectory: GetRuntimeDirectory::<Identity, OFFSET>,
            IsLoaded: IsLoaded::<Identity, OFFSET>,
            LoadErrorString: LoadErrorString::<Identity, OFFSET>,
            LoadLibraryA: LoadLibraryA::<Identity, OFFSET>,
            GetProcAddress: GetProcAddress::<Identity, OFFSET>,
            GetInterface: GetInterface::<Identity, OFFSET>,
            IsLoadable: IsLoadable::<Identity, OFFSET>,
            SetDefaultStartupFlags: SetDefaultStartupFlags::<Identity, OFFSET>,
            GetDefaultStartupFlags: GetDefaultStartupFlags::<Identity, OFFSET>,
            BindAsLegacyV2Runtime: BindAsLegacyV2Runtime::<Identity, OFFSET>,
            IsStarted: IsStarted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRRuntimeInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRRuntimeInfo {}
windows_core::imp::define_interface!(ICLRStrongName, ICLRStrongName_Vtbl, 0x9fd93ccf_3280_4391_b3a9_96e1cde77c8d);
windows_core::imp::interface_hierarchy!(ICLRStrongName, windows_core::IUnknown);
impl ICLRStrongName {
    pub unsafe fn GetHashFromAssemblyFile<P0>(&self, pszfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetHashFromAssemblyFile)(windows_core::Interface::as_raw(self), pszfilepath.param().abi(), pihashalg as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash as _).ok() }
    }
    pub unsafe fn GetHashFromAssemblyFileW<P0>(&self, pwzfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetHashFromAssemblyFileW)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pihashalg as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash as _).ok() }
    }
    pub unsafe fn GetHashFromBlob(&self, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHashFromBlob)(windows_core::Interface::as_raw(self), pbblob, cchblob, pihashalg as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash as _).ok() }
    }
    pub unsafe fn GetHashFromFile<P0>(&self, pszfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetHashFromFile)(windows_core::Interface::as_raw(self), pszfilepath.param().abi(), pihashalg as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash as _).ok() }
    }
    pub unsafe fn GetHashFromFileW<P0>(&self, pwzfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetHashFromFileW)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pihashalg as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash as _).ok() }
    }
    pub unsafe fn GetHashFromHandle(&self, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHashFromHandle)(windows_core::Interface::as_raw(self), hfile, pihashalg as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash as _).ok() }
    }
    pub unsafe fn StrongNameCompareAssemblies<P0, P1>(&self, pwzassembly1: P0, pwzassembly2: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrongNameCompareAssemblies)(windows_core::Interface::as_raw(self), pwzassembly1.param().abi(), pwzassembly2.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StrongNameFreeBuffer(&self, pbmemory: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StrongNameFreeBuffer)(windows_core::Interface::as_raw(self), pbmemory).ok() }
    }
    pub unsafe fn StrongNameGetBlob<P0>(&self, pwzfilepath: P0, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameGetBlob)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pbblob as _, pcbblob as _).ok() }
    }
    pub unsafe fn StrongNameGetBlobFromImage(&self, pbbase: &[u8], pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StrongNameGetBlobFromImage)(windows_core::Interface::as_raw(self), core::mem::transmute(pbbase.as_ptr()), pbbase.len().try_into().unwrap(), pbblob as _, pcbblob as _).ok() }
    }
    pub unsafe fn StrongNameGetPublicKey<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameGetPublicKey)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbpublickeyblob as _, pcbpublickeyblob as _).ok() }
    }
    pub unsafe fn StrongNameHashSize(&self, ulhashalg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrongNameHashSize)(windows_core::Interface::as_raw(self), ulhashalg, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StrongNameKeyDelete<P0>(&self, pwzkeycontainer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameKeyDelete)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi()).ok() }
    }
    pub unsafe fn StrongNameKeyGen<P0>(&self, pwzkeycontainer: P0, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameKeyGen)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), dwflags, ppbkeyblob as _, pcbkeyblob as _).ok() }
    }
    pub unsafe fn StrongNameKeyGenEx<P0>(&self, pwzkeycontainer: P0, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameKeyGenEx)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), dwflags, dwkeysize, ppbkeyblob as _, pcbkeyblob as _).ok() }
    }
    pub unsafe fn StrongNameKeyInstall<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *const u8, cbkeyblob: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameKeyInstall)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob).ok() }
    }
    pub unsafe fn StrongNameSignatureGeneration<P0, P1>(&self, pwzfilepath: P0, pwzkeycontainer: P1, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameSignatureGeneration)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbsignatureblob as _, pcbsignatureblob as _).ok() }
    }
    pub unsafe fn StrongNameSignatureGenerationEx<P0, P1>(&self, wszfilepath: P0, wszkeycontainer: P1, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameSignatureGenerationEx)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), wszkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbsignatureblob as _, pcbsignatureblob as _, dwflags).ok() }
    }
    pub unsafe fn StrongNameSignatureSize(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StrongNameSignatureSize)(windows_core::Interface::as_raw(self), pbpublickeyblob, cbpublickeyblob, pcbsize).ok() }
    }
    pub unsafe fn StrongNameSignatureVerification<P0>(&self, pwzfilepath: P0, dwinflags: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrongNameSignatureVerification)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), dwinflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StrongNameSignatureVerificationEx<P0>(&self, pwzfilepath: P0, fforceverification: bool) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrongNameSignatureVerificationEx)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), fforceverification, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StrongNameSignatureVerificationFromImage(&self, pbbase: *const u8, dwlength: u32, dwinflags: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrongNameSignatureVerificationFromImage)(windows_core::Interface::as_raw(self), pbbase, dwlength, dwinflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StrongNameTokenFromAssembly<P0>(&self, pwzfilepath: P0, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameTokenFromAssembly)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), ppbstrongnametoken as _, pcbstrongnametoken as _).ok() }
    }
    pub unsafe fn StrongNameTokenFromAssemblyEx<P0>(&self, pwzfilepath: P0, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameTokenFromAssemblyEx)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), ppbstrongnametoken as _, pcbstrongnametoken as _, ppbpublickeyblob as _, pcbpublickeyblob as _).ok() }
    }
    pub unsafe fn StrongNameTokenFromPublicKey(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StrongNameTokenFromPublicKey)(windows_core::Interface::as_raw(self), pbpublickeyblob, cbpublickeyblob, ppbstrongnametoken as _, pcbstrongnametoken as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRStrongName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHashFromAssemblyFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetHashFromAssemblyFileW: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetHashFromBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetHashFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetHashFromFileW: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetHashFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub StrongNameCompareAssemblies: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub StrongNameFreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8) -> windows_core::HRESULT,
    pub StrongNameGetBlob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameGetBlobFromImage: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameGetPublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameHashSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub StrongNameKeyDelete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StrongNameKeyGen: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameKeyGenEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameKeyInstall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32) -> windows_core::HRESULT,
    pub StrongNameSignatureGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameSignatureGenerationEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, *mut *mut u8, *mut u32, u32) -> windows_core::HRESULT,
    pub StrongNameSignatureSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u32) -> windows_core::HRESULT,
    pub StrongNameSignatureVerification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub StrongNameSignatureVerificationEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, bool, *mut u8) -> windows_core::HRESULT,
    pub StrongNameSignatureVerificationFromImage: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub StrongNameTokenFromAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameTokenFromAssemblyEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameTokenFromPublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ICLRStrongName_Impl: windows_core::IUnknownImpl {
    fn GetHashFromAssemblyFile(&self, pszfilepath: &windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromAssemblyFileW(&self, pwzfilepath: &windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromBlob(&self, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromFile(&self, pszfilepath: &windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromFileW(&self, pwzfilepath: &windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromHandle(&self, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn StrongNameCompareAssemblies(&self, pwzassembly1: &windows_core::PCWSTR, pwzassembly2: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn StrongNameFreeBuffer(&self, pbmemory: *const u8) -> windows_core::Result<()>;
    fn StrongNameGetBlob(&self, pwzfilepath: &windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameGetBlobFromImage(&self, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameGetPublicKey(&self, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameHashSize(&self, ulhashalg: u32) -> windows_core::Result<u32>;
    fn StrongNameKeyDelete(&self, pwzkeycontainer: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StrongNameKeyGen(&self, pwzkeycontainer: &windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameKeyGenEx(&self, pwzkeycontainer: &windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameKeyInstall(&self, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> windows_core::Result<()>;
    fn StrongNameSignatureGeneration(&self, pwzfilepath: &windows_core::PCWSTR, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameSignatureGenerationEx(&self, wszfilepath: &windows_core::PCWSTR, wszkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn StrongNameSignatureSize(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> windows_core::Result<()>;
    fn StrongNameSignatureVerification(&self, pwzfilepath: &windows_core::PCWSTR, dwinflags: u32) -> windows_core::Result<u32>;
    fn StrongNameSignatureVerificationEx(&self, pwzfilepath: &windows_core::PCWSTR, fforceverification: bool) -> windows_core::Result<u8>;
    fn StrongNameSignatureVerificationFromImage(&self, pbbase: *const u8, dwlength: u32, dwinflags: u32) -> windows_core::Result<u32>;
    fn StrongNameTokenFromAssembly(&self, pwzfilepath: &windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()>;
    fn StrongNameTokenFromAssemblyEx(&self, pwzfilepath: &windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameTokenFromPublicKey(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()>;
}
impl ICLRStrongName_Vtbl {
    pub const fn new<Identity: ICLRStrongName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHashFromAssemblyFile<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilepath: windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::GetHashFromAssemblyFile(this, core::mem::transmute(&pszfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
            }
        }
        unsafe extern "system" fn GetHashFromAssemblyFileW<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::GetHashFromAssemblyFileW(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
            }
        }
        unsafe extern "system" fn GetHashFromBlob<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::GetHashFromBlob(this, core::mem::transmute_copy(&pbblob), core::mem::transmute_copy(&cchblob), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
            }
        }
        unsafe extern "system" fn GetHashFromFile<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilepath: windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::GetHashFromFile(this, core::mem::transmute(&pszfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
            }
        }
        unsafe extern "system" fn GetHashFromFileW<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::GetHashFromFileW(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
            }
        }
        unsafe extern "system" fn GetHashFromHandle<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::GetHashFromHandle(this, core::mem::transmute_copy(&hfile), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
            }
        }
        unsafe extern "system" fn StrongNameCompareAssemblies<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassembly1: windows_core::PCWSTR, pwzassembly2: windows_core::PCWSTR, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRStrongName_Impl::StrongNameCompareAssemblies(this, core::mem::transmute(&pwzassembly1), core::mem::transmute(&pwzassembly2)) {
                    Ok(ok__) => {
                        pdwresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrongNameFreeBuffer<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmemory: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameFreeBuffer(this, core::mem::transmute_copy(&pbmemory)).into()
            }
        }
        unsafe extern "system" fn StrongNameGetBlob<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameGetBlob(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pbblob), core::mem::transmute_copy(&pcbblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameGetBlobFromImage<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameGetBlobFromImage(this, core::mem::transmute_copy(&pbbase), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&pbblob), core::mem::transmute_copy(&pcbblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameGetPublicKey<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameGetPublicKey(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbpublickeyblob), core::mem::transmute_copy(&pcbpublickeyblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameHashSize<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulhashalg: u32, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRStrongName_Impl::StrongNameHashSize(this, core::mem::transmute_copy(&ulhashalg)) {
                    Ok(ok__) => {
                        pcbsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrongNameKeyDelete<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameKeyDelete(this, core::mem::transmute(&pwzkeycontainer)).into()
            }
        }
        unsafe extern "system" fn StrongNameKeyGen<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameKeyGen(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppbkeyblob), core::mem::transmute_copy(&pcbkeyblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameKeyGenEx<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameKeyGenEx(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwkeysize), core::mem::transmute_copy(&ppbkeyblob), core::mem::transmute_copy(&pcbkeyblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameKeyInstall<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameKeyInstall(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameSignatureGeneration<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameSignatureGeneration(this, core::mem::transmute(&pwzfilepath), core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbsignatureblob), core::mem::transmute_copy(&pcbsignatureblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameSignatureGenerationEx<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, wszkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameSignatureGenerationEx(this, core::mem::transmute(&wszfilepath), core::mem::transmute(&wszkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbsignatureblob), core::mem::transmute_copy(&pcbsignatureblob), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn StrongNameSignatureSize<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameSignatureSize(this, core::mem::transmute_copy(&pbpublickeyblob), core::mem::transmute_copy(&cbpublickeyblob), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerification<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, dwinflags: u32, pdwoutflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRStrongName_Impl::StrongNameSignatureVerification(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&dwinflags)) {
                    Ok(ok__) => {
                        pdwoutflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, fforceverification: bool, pfwasverified: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRStrongName_Impl::StrongNameSignatureVerificationEx(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&fforceverification)) {
                    Ok(ok__) => {
                        pfwasverified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerificationFromImage<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbase: *const u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRStrongName_Impl::StrongNameSignatureVerificationFromImage(this, core::mem::transmute_copy(&pbbase), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&dwinflags)) {
                    Ok(ok__) => {
                        pdwoutflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrongNameTokenFromAssembly<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameTokenFromAssembly(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&ppbstrongnametoken), core::mem::transmute_copy(&pcbstrongnametoken)).into()
            }
        }
        unsafe extern "system" fn StrongNameTokenFromAssemblyEx<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameTokenFromAssemblyEx(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&ppbstrongnametoken), core::mem::transmute_copy(&pcbstrongnametoken), core::mem::transmute_copy(&ppbpublickeyblob), core::mem::transmute_copy(&pcbpublickeyblob)).into()
            }
        }
        unsafe extern "system" fn StrongNameTokenFromPublicKey<Identity: ICLRStrongName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName_Impl::StrongNameTokenFromPublicKey(this, core::mem::transmute_copy(&pbpublickeyblob), core::mem::transmute_copy(&cbpublickeyblob), core::mem::transmute_copy(&ppbstrongnametoken), core::mem::transmute_copy(&pcbstrongnametoken)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHashFromAssemblyFile: GetHashFromAssemblyFile::<Identity, OFFSET>,
            GetHashFromAssemblyFileW: GetHashFromAssemblyFileW::<Identity, OFFSET>,
            GetHashFromBlob: GetHashFromBlob::<Identity, OFFSET>,
            GetHashFromFile: GetHashFromFile::<Identity, OFFSET>,
            GetHashFromFileW: GetHashFromFileW::<Identity, OFFSET>,
            GetHashFromHandle: GetHashFromHandle::<Identity, OFFSET>,
            StrongNameCompareAssemblies: StrongNameCompareAssemblies::<Identity, OFFSET>,
            StrongNameFreeBuffer: StrongNameFreeBuffer::<Identity, OFFSET>,
            StrongNameGetBlob: StrongNameGetBlob::<Identity, OFFSET>,
            StrongNameGetBlobFromImage: StrongNameGetBlobFromImage::<Identity, OFFSET>,
            StrongNameGetPublicKey: StrongNameGetPublicKey::<Identity, OFFSET>,
            StrongNameHashSize: StrongNameHashSize::<Identity, OFFSET>,
            StrongNameKeyDelete: StrongNameKeyDelete::<Identity, OFFSET>,
            StrongNameKeyGen: StrongNameKeyGen::<Identity, OFFSET>,
            StrongNameKeyGenEx: StrongNameKeyGenEx::<Identity, OFFSET>,
            StrongNameKeyInstall: StrongNameKeyInstall::<Identity, OFFSET>,
            StrongNameSignatureGeneration: StrongNameSignatureGeneration::<Identity, OFFSET>,
            StrongNameSignatureGenerationEx: StrongNameSignatureGenerationEx::<Identity, OFFSET>,
            StrongNameSignatureSize: StrongNameSignatureSize::<Identity, OFFSET>,
            StrongNameSignatureVerification: StrongNameSignatureVerification::<Identity, OFFSET>,
            StrongNameSignatureVerificationEx: StrongNameSignatureVerificationEx::<Identity, OFFSET>,
            StrongNameSignatureVerificationFromImage: StrongNameSignatureVerificationFromImage::<Identity, OFFSET>,
            StrongNameTokenFromAssembly: StrongNameTokenFromAssembly::<Identity, OFFSET>,
            StrongNameTokenFromAssemblyEx: StrongNameTokenFromAssemblyEx::<Identity, OFFSET>,
            StrongNameTokenFromPublicKey: StrongNameTokenFromPublicKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRStrongName as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRStrongName {}
windows_core::imp::define_interface!(ICLRStrongName2, ICLRStrongName2_Vtbl, 0xc22ed5c5_4b59_4975_90eb_85ea55c0069b);
windows_core::imp::interface_hierarchy!(ICLRStrongName2, windows_core::IUnknown);
impl ICLRStrongName2 {
    pub unsafe fn StrongNameGetPublicKeyEx<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameGetPublicKeyEx)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbpublickeyblob as _, pcbpublickeyblob as _, uhashalgid, ureserved).ok() }
    }
    pub unsafe fn StrongNameSignatureVerificationEx2<P0>(&self, wszfilepath: P0, fforceverification: bool, pbecmapublickey: *const u8, cbecmapublickey: u32) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrongNameSignatureVerificationEx2)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), fforceverification, pbecmapublickey, cbecmapublickey, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRStrongName2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StrongNameGetPublicKeyEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, *mut *mut u8, *mut u32, u32, u32) -> windows_core::HRESULT,
    pub StrongNameSignatureVerificationEx2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, bool, *const u8, u32, *mut u8) -> windows_core::HRESULT,
}
pub trait ICLRStrongName2_Impl: windows_core::IUnknownImpl {
    fn StrongNameGetPublicKeyEx(&self, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> windows_core::Result<()>;
    fn StrongNameSignatureVerificationEx2(&self, wszfilepath: &windows_core::PCWSTR, fforceverification: bool, pbecmapublickey: *const u8, cbecmapublickey: u32) -> windows_core::Result<u8>;
}
impl ICLRStrongName2_Vtbl {
    pub const fn new<Identity: ICLRStrongName2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StrongNameGetPublicKeyEx<Identity: ICLRStrongName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName2_Impl::StrongNameGetPublicKeyEx(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbpublickeyblob), core::mem::transmute_copy(&pcbpublickeyblob), core::mem::transmute_copy(&uhashalgid), core::mem::transmute_copy(&ureserved)).into()
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx2<Identity: ICLRStrongName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, fforceverification: bool, pbecmapublickey: *const u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRStrongName2_Impl::StrongNameSignatureVerificationEx2(this, core::mem::transmute(&wszfilepath), core::mem::transmute_copy(&fforceverification), core::mem::transmute_copy(&pbecmapublickey), core::mem::transmute_copy(&cbecmapublickey)) {
                    Ok(ok__) => {
                        pfwasverified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameGetPublicKeyEx: StrongNameGetPublicKeyEx::<Identity, OFFSET>,
            StrongNameSignatureVerificationEx2: StrongNameSignatureVerificationEx2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRStrongName2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRStrongName2 {}
windows_core::imp::define_interface!(ICLRStrongName3, ICLRStrongName3_Vtbl, 0x22c7089b_bbd3_414a_b698_210f263f1fed);
windows_core::imp::interface_hierarchy!(ICLRStrongName3, windows_core::IUnknown);
impl ICLRStrongName3 {
    pub unsafe fn StrongNameDigestGenerate<P0>(&self, wszfilepath: P0, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameDigestGenerate)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), ppbdigestblob as _, pcbdigestblob as _, dwflags).ok() }
    }
    pub unsafe fn StrongNameDigestSign<P0>(&self, wszkeycontainer: P0, pbkeyblob: &[u8], pbdigestblob: &[u8], hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameDigestSign)(windows_core::Interface::as_raw(self), wszkeycontainer.param().abi(), core::mem::transmute(pbkeyblob.as_ptr()), pbkeyblob.len().try_into().unwrap(), core::mem::transmute(pbdigestblob.as_ptr()), pbdigestblob.len().try_into().unwrap(), hashalgid, ppbsignatureblob as _, pcbsignatureblob as _, dwflags).ok() }
    }
    pub unsafe fn StrongNameDigestEmbed<P0>(&self, wszfilepath: P0, pbsignatureblob: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StrongNameDigestEmbed)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), core::mem::transmute(pbsignatureblob.as_ptr()), pbsignatureblob.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRStrongName3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StrongNameDigestGenerate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u8, *mut u32, u32) -> windows_core::HRESULT,
    pub StrongNameDigestSign: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, *const u8, u32, u32, *mut *mut u8, *mut u32, u32) -> windows_core::HRESULT,
    pub StrongNameDigestEmbed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32) -> windows_core::HRESULT,
}
pub trait ICLRStrongName3_Impl: windows_core::IUnknownImpl {
    fn StrongNameDigestGenerate(&self, wszfilepath: &windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn StrongNameDigestSign(&self, wszkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn StrongNameDigestEmbed(&self, wszfilepath: &windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> windows_core::Result<()>;
}
impl ICLRStrongName3_Vtbl {
    pub const fn new<Identity: ICLRStrongName3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StrongNameDigestGenerate<Identity: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName3_Impl::StrongNameDigestGenerate(this, core::mem::transmute(&wszfilepath), core::mem::transmute_copy(&ppbdigestblob), core::mem::transmute_copy(&pcbdigestblob), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn StrongNameDigestSign<Identity: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName3_Impl::StrongNameDigestSign(this, core::mem::transmute(&wszkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&pbdigestblob), core::mem::transmute_copy(&cbdigestblob), core::mem::transmute_copy(&hashalgid), core::mem::transmute_copy(&ppbsignatureblob), core::mem::transmute_copy(&pcbsignatureblob), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn StrongNameDigestEmbed<Identity: ICLRStrongName3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRStrongName3_Impl::StrongNameDigestEmbed(this, core::mem::transmute(&wszfilepath), core::mem::transmute_copy(&pbsignatureblob), core::mem::transmute_copy(&cbsignatureblob)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameDigestGenerate: StrongNameDigestGenerate::<Identity, OFFSET>,
            StrongNameDigestSign: StrongNameDigestSign::<Identity, OFFSET>,
            StrongNameDigestEmbed: StrongNameDigestEmbed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRStrongName3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRStrongName3 {}
windows_core::imp::define_interface!(ICLRSyncManager, ICLRSyncManager_Vtbl, 0x55ff199d_ad21_48f9_a16c_f24ebbb8727d);
windows_core::imp::interface_hierarchy!(ICLRSyncManager, windows_core::IUnknown);
impl ICLRSyncManager {
    pub unsafe fn GetMonitorOwner(&self, cookie: usize) -> windows_core::Result<IHostTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMonitorOwner)(windows_core::Interface::as_raw(self), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRWLockOwnerIterator(&self, cookie: usize) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRWLockOwnerIterator)(windows_core::Interface::as_raw(self), cookie, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRWLockOwnerNext(&self, iterator: usize) -> windows_core::Result<IHostTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRWLockOwnerNext)(windows_core::Interface::as_raw(self), iterator, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteRWLockOwnerIterator(&self, iterator: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteRWLockOwnerIterator)(windows_core::Interface::as_raw(self), iterator).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRSyncManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMonitorOwner: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRWLockOwnerIterator: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub GetRWLockOwnerNext: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRWLockOwnerIterator: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait ICLRSyncManager_Impl: windows_core::IUnknownImpl {
    fn GetMonitorOwner(&self, cookie: usize) -> windows_core::Result<IHostTask>;
    fn CreateRWLockOwnerIterator(&self, cookie: usize) -> windows_core::Result<usize>;
    fn GetRWLockOwnerNext(&self, iterator: usize) -> windows_core::Result<IHostTask>;
    fn DeleteRWLockOwnerIterator(&self, iterator: usize) -> windows_core::Result<()>;
}
impl ICLRSyncManager_Vtbl {
    pub const fn new<Identity: ICLRSyncManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMonitorOwner<Identity: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, ppownerhosttask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRSyncManager_Impl::GetMonitorOwner(this, core::mem::transmute_copy(&cookie)) {
                    Ok(ok__) => {
                        ppownerhosttask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRWLockOwnerIterator<Identity: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, piterator: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRSyncManager_Impl::CreateRWLockOwnerIterator(this, core::mem::transmute_copy(&cookie)) {
                    Ok(ok__) => {
                        piterator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRWLockOwnerNext<Identity: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iterator: usize, ppownerhosttask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRSyncManager_Impl::GetRWLockOwnerNext(this, core::mem::transmute_copy(&iterator)) {
                    Ok(ok__) => {
                        ppownerhosttask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteRWLockOwnerIterator<Identity: ICLRSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iterator: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRSyncManager_Impl::DeleteRWLockOwnerIterator(this, core::mem::transmute_copy(&iterator)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMonitorOwner: GetMonitorOwner::<Identity, OFFSET>,
            CreateRWLockOwnerIterator: CreateRWLockOwnerIterator::<Identity, OFFSET>,
            GetRWLockOwnerNext: GetRWLockOwnerNext::<Identity, OFFSET>,
            DeleteRWLockOwnerIterator: DeleteRWLockOwnerIterator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRSyncManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRSyncManager {}
windows_core::imp::define_interface!(ICLRTask, ICLRTask_Vtbl, 0x28e66a4a_9906_4225_b231_9187c3eb8611);
windows_core::imp::interface_hierarchy!(ICLRTask, windows_core::IUnknown);
impl ICLRTask {
    pub unsafe fn SwitchIn(&self, threadhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SwitchIn)(windows_core::Interface::as_raw(self), threadhandle).ok() }
    }
    pub unsafe fn SwitchOut(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SwitchOut)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMemStats(&self) -> windows_core::Result<COR_GC_THREAD_STATS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMemStats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self, ffull: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), ffull.into()).ok() }
    }
    pub unsafe fn ExitTask(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExitTask)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RudeAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RudeAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn NeedsPriorityScheduling(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NeedsPriorityScheduling)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn YieldTask(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).YieldTask)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn LocksHeld(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocksHeld)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTaskIdentifier(&self, asked: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskIdentifier)(windows_core::Interface::as_raw(self), asked).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRTask_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SwitchIn: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SwitchOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMemStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ExitTask: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RudeAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NeedsPriorityScheduling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub YieldTask: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocksHeld: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub SetTaskIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait ICLRTask_Impl: windows_core::IUnknownImpl {
    fn SwitchIn(&self, threadhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SwitchOut(&self) -> windows_core::Result<()>;
    fn GetMemStats(&self) -> windows_core::Result<COR_GC_THREAD_STATS>;
    fn Reset(&self, ffull: windows_core::BOOL) -> windows_core::Result<()>;
    fn ExitTask(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn RudeAbort(&self) -> windows_core::Result<()>;
    fn NeedsPriorityScheduling(&self) -> windows_core::Result<windows_core::BOOL>;
    fn YieldTask(&self) -> windows_core::Result<()>;
    fn LocksHeld(&self) -> windows_core::Result<usize>;
    fn SetTaskIdentifier(&self, asked: u64) -> windows_core::Result<()>;
}
impl ICLRTask_Vtbl {
    pub const fn new<Identity: ICLRTask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SwitchIn<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::SwitchIn(this, core::mem::transmute_copy(&threadhandle)).into()
            }
        }
        unsafe extern "system" fn SwitchOut<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::SwitchOut(this).into()
            }
        }
        unsafe extern "system" fn GetMemStats<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memusage: *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRTask_Impl::GetMemStats(this) {
                    Ok(ok__) => {
                        memusage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffull: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::Reset(this, core::mem::transmute_copy(&ffull)).into()
            }
        }
        unsafe extern "system" fn ExitTask<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::ExitTask(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn RudeAbort<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::RudeAbort(this).into()
            }
        }
        unsafe extern "system" fn NeedsPriorityScheduling<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbneedspriorityscheduling: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRTask_Impl::NeedsPriorityScheduling(this) {
                    Ok(ok__) => {
                        pbneedspriorityscheduling.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn YieldTask<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::YieldTask(this).into()
            }
        }
        unsafe extern "system" fn LocksHeld<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockcount: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRTask_Impl::LocksHeld(this) {
                    Ok(ok__) => {
                        plockcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskIdentifier<Identity: ICLRTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, asked: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask_Impl::SetTaskIdentifier(this, core::mem::transmute_copy(&asked)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SwitchIn: SwitchIn::<Identity, OFFSET>,
            SwitchOut: SwitchOut::<Identity, OFFSET>,
            GetMemStats: GetMemStats::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ExitTask: ExitTask::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            RudeAbort: RudeAbort::<Identity, OFFSET>,
            NeedsPriorityScheduling: NeedsPriorityScheduling::<Identity, OFFSET>,
            YieldTask: YieldTask::<Identity, OFFSET>,
            LocksHeld: LocksHeld::<Identity, OFFSET>,
            SetTaskIdentifier: SetTaskIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRTask as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRTask {}
windows_core::imp::define_interface!(ICLRTask2, ICLRTask2_Vtbl, 0x28e66a4a_9906_4225_b231_9187c3eb8612);
impl core::ops::Deref for ICLRTask2 {
    type Target = ICLRTask;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRTask2, windows_core::IUnknown, ICLRTask);
impl ICLRTask2 {
    pub unsafe fn BeginPreventAsyncAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginPreventAsyncAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EndPreventAsyncAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndPreventAsyncAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRTask2_Vtbl {
    pub base__: ICLRTask_Vtbl,
    pub BeginPreventAsyncAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndPreventAsyncAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICLRTask2_Impl: ICLRTask_Impl {
    fn BeginPreventAsyncAbort(&self) -> windows_core::Result<()>;
    fn EndPreventAsyncAbort(&self) -> windows_core::Result<()>;
}
impl ICLRTask2_Vtbl {
    pub const fn new<Identity: ICLRTask2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginPreventAsyncAbort<Identity: ICLRTask2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask2_Impl::BeginPreventAsyncAbort(this).into()
            }
        }
        unsafe extern "system" fn EndPreventAsyncAbort<Identity: ICLRTask2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTask2_Impl::EndPreventAsyncAbort(this).into()
            }
        }
        Self {
            base__: ICLRTask_Vtbl::new::<Identity, OFFSET>(),
            BeginPreventAsyncAbort: BeginPreventAsyncAbort::<Identity, OFFSET>,
            EndPreventAsyncAbort: EndPreventAsyncAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRTask2 as windows_core::Interface>::IID || iid == &<ICLRTask as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRTask2 {}
windows_core::imp::define_interface!(ICLRTaskManager, ICLRTaskManager_Vtbl, 0x4862efbe_3ae5_44f8_8feb_346190ee8a34);
windows_core::imp::interface_hierarchy!(ICLRTaskManager, windows_core::IUnknown);
impl ICLRTaskManager {
    pub unsafe fn CreateTask(&self) -> windows_core::Result<ICLRTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentTask(&self) -> windows_core::Result<ICLRTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentTask)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUILocale)(windows_core::Interface::as_raw(self), lcid).ok() }
    }
    pub unsafe fn SetLocale(&self, lcid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), lcid).ok() }
    }
    pub unsafe fn GetCurrentTaskType(&self) -> windows_core::Result<ETaskType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentTaskType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICLRTaskManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUILocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCurrentTaskType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ETaskType) -> windows_core::HRESULT,
}
pub trait ICLRTaskManager_Impl: windows_core::IUnknownImpl {
    fn CreateTask(&self) -> windows_core::Result<ICLRTask>;
    fn GetCurrentTask(&self) -> windows_core::Result<ICLRTask>;
    fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn GetCurrentTaskType(&self) -> windows_core::Result<ETaskType>;
}
impl ICLRTaskManager_Vtbl {
    pub const fn new<Identity: ICLRTaskManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTask<Identity: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRTaskManager_Impl::CreateTask(this) {
                    Ok(ok__) => {
                        ptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentTask<Identity: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRTaskManager_Impl::GetCurrentTask(this) {
                    Ok(ok__) => {
                        ptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUILocale<Identity: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTaskManager_Impl::SetUILocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn SetLocale<Identity: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICLRTaskManager_Impl::SetLocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn GetCurrentTaskType<Identity: ICLRTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptasktype: *mut ETaskType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICLRTaskManager_Impl::GetCurrentTaskType(this) {
                    Ok(ok__) => {
                        ptasktype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTask: CreateTask::<Identity, OFFSET>,
            GetCurrentTask: GetCurrentTask::<Identity, OFFSET>,
            SetUILocale: SetUILocale::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetCurrentTaskType: GetCurrentTaskType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRTaskManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICLRTaskManager {}
windows_core::imp::define_interface!(ICatalogServices, ICatalogServices_Vtbl, 0x04c6be1e_1db1_4058_ab7a_700cccfbf254);
windows_core::imp::interface_hierarchy!(ICatalogServices, windows_core::IUnknown);
impl ICatalogServices {
    pub unsafe fn Autodone(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Autodone)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn NotAutodone(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotAutodone)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Autodone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotAutodone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICatalogServices_Impl: windows_core::IUnknownImpl {
    fn Autodone(&self) -> windows_core::Result<()>;
    fn NotAutodone(&self) -> windows_core::Result<()>;
}
impl ICatalogServices_Vtbl {
    pub const fn new<Identity: ICatalogServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Autodone<Identity: ICatalogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogServices_Impl::Autodone(this).into()
            }
        }
        unsafe extern "system" fn NotAutodone<Identity: ICatalogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogServices_Impl::NotAutodone(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Autodone: Autodone::<Identity, OFFSET>,
            NotAutodone: NotAutodone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICatalogServices {}
windows_core::imp::define_interface!(ICorConfiguration, ICorConfiguration_Vtbl, 0x5c2b07a5_1e98_11d3_872f_00c04f79ed0d);
windows_core::imp::interface_hierarchy!(ICorConfiguration, windows_core::IUnknown);
impl ICorConfiguration {
    pub unsafe fn SetGCThreadControl<P0>(&self, pgcthreadcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGCThreadControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGCThreadControl)(windows_core::Interface::as_raw(self), pgcthreadcontrol.param().abi()).ok() }
    }
    pub unsafe fn SetGCHostControl<P0>(&self, pgchostcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGCHostControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGCHostControl)(windows_core::Interface::as_raw(self), pgchostcontrol.param().abi()).ok() }
    }
    pub unsafe fn SetDebuggerThreadControl<P0>(&self, pdebuggerthreadcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebuggerThreadControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDebuggerThreadControl)(windows_core::Interface::as_raw(self), pdebuggerthreadcontrol.param().abi()).ok() }
    }
    pub unsafe fn AddDebuggerSpecialThread(&self, dwspecialthreadid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDebuggerSpecialThread)(windows_core::Interface::as_raw(self), dwspecialthreadid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGCThreadControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGCHostControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDebuggerThreadControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDebuggerSpecialThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ICorConfiguration_Impl: windows_core::IUnknownImpl {
    fn SetGCThreadControl(&self, pgcthreadcontrol: windows_core::Ref<IGCThreadControl>) -> windows_core::Result<()>;
    fn SetGCHostControl(&self, pgchostcontrol: windows_core::Ref<IGCHostControl>) -> windows_core::Result<()>;
    fn SetDebuggerThreadControl(&self, pdebuggerthreadcontrol: windows_core::Ref<IDebuggerThreadControl>) -> windows_core::Result<()>;
    fn AddDebuggerSpecialThread(&self, dwspecialthreadid: u32) -> windows_core::Result<()>;
}
impl ICorConfiguration_Vtbl {
    pub const fn new<Identity: ICorConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGCThreadControl<Identity: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgcthreadcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorConfiguration_Impl::SetGCThreadControl(this, core::mem::transmute_copy(&pgcthreadcontrol)).into()
            }
        }
        unsafe extern "system" fn SetGCHostControl<Identity: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgchostcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorConfiguration_Impl::SetGCHostControl(this, core::mem::transmute_copy(&pgchostcontrol)).into()
            }
        }
        unsafe extern "system" fn SetDebuggerThreadControl<Identity: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdebuggerthreadcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorConfiguration_Impl::SetDebuggerThreadControl(this, core::mem::transmute_copy(&pdebuggerthreadcontrol)).into()
            }
        }
        unsafe extern "system" fn AddDebuggerSpecialThread<Identity: ICorConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspecialthreadid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorConfiguration_Impl::AddDebuggerSpecialThread(this, core::mem::transmute_copy(&dwspecialthreadid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCThreadControl: SetGCThreadControl::<Identity, OFFSET>,
            SetGCHostControl: SetGCHostControl::<Identity, OFFSET>,
            SetDebuggerThreadControl: SetDebuggerThreadControl::<Identity, OFFSET>,
            AddDebuggerSpecialThread: AddDebuggerSpecialThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorConfiguration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorConfiguration {}
windows_core::imp::define_interface!(ICorRuntimeHost, ICorRuntimeHost_Vtbl, 0xcb2f6722_ab3a_11d2_9c40_00c04fa30a3e);
windows_core::imp::interface_hierarchy!(ICorRuntimeHost, windows_core::IUnknown);
impl ICorRuntimeHost {
    pub unsafe fn CreateLogicalThreadState(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateLogicalThreadState)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DeleteLogicalThreadState(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteLogicalThreadState)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SwitchInLogicalThreadState(&self, pfibercookie: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SwitchInLogicalThreadState)(windows_core::Interface::as_raw(self), pfibercookie).ok() }
    }
    pub unsafe fn SwitchOutLogicalThreadState(&self) -> windows_core::Result<*mut u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SwitchOutLogicalThreadState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocksHeldByLogicalThread(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocksHeldByLogicalThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MapFile(&self, hfile: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::HMODULE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MapFile)(windows_core::Interface::as_raw(self), hfile, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConfiguration(&self) -> windows_core::Result<ICorConfiguration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CreateDomain<P0, P1>(&self, pwzfriendlyname: P0, pidentityarray: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDomain)(windows_core::Interface::as_raw(self), pwzfriendlyname.param().abi(), pidentityarray.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDefaultDomain(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultDomain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumDomains(&self, henum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDomains)(windows_core::Interface::as_raw(self), henum as _).ok() }
    }
    pub unsafe fn NextDomain(&self, henum: *const core::ffi::c_void) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextDomain)(windows_core::Interface::as_raw(self), henum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CloseEnum(&self, henum: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseEnum)(windows_core::Interface::as_raw(self), henum).ok() }
    }
    pub unsafe fn CreateDomainEx<P0, P1, P2>(&self, pwzfriendlyname: P0, psetup: P1, pevidence: P2) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDomainEx)(windows_core::Interface::as_raw(self), pwzfriendlyname.param().abi(), psetup.param().abi(), pevidence.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDomainSetup(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDomainSetup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEvidence(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEvidence)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UnloadDomain<P0>(&self, pappdomain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnloadDomain)(windows_core::Interface::as_raw(self), pappdomain.param().abi()).ok() }
    }
    pub unsafe fn CurrentDomain(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDomain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorRuntimeHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateLogicalThreadState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteLogicalThreadState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SwitchInLogicalThreadState: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    pub SwitchOutLogicalThreadState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u32) -> windows_core::HRESULT,
    pub LocksHeldByLogicalThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MapFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT,
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDomain: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDomains: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDomainEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDomainSetup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEvidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnloadDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorRuntimeHost_Impl: windows_core::IUnknownImpl {
    fn CreateLogicalThreadState(&self) -> windows_core::Result<()>;
    fn DeleteLogicalThreadState(&self) -> windows_core::Result<()>;
    fn SwitchInLogicalThreadState(&self, pfibercookie: *const u32) -> windows_core::Result<()>;
    fn SwitchOutLogicalThreadState(&self) -> windows_core::Result<*mut u32>;
    fn LocksHeldByLogicalThread(&self) -> windows_core::Result<u32>;
    fn MapFile(&self, hfile: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetConfiguration(&self) -> windows_core::Result<ICorConfiguration>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn CreateDomain(&self, pwzfriendlyname: &windows_core::PCWSTR, pidentityarray: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDefaultDomain(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn EnumDomains(&self, henum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn NextDomain(&self, henum: *const core::ffi::c_void) -> windows_core::Result<windows_core::IUnknown>;
    fn CloseEnum(&self, henum: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateDomainEx(&self, pwzfriendlyname: &windows_core::PCWSTR, psetup: windows_core::Ref<windows_core::IUnknown>, pevidence: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateDomainSetup(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateEvidence(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn UnloadDomain(&self, pappdomain: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CurrentDomain(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl ICorRuntimeHost_Vtbl {
    pub const fn new<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateLogicalThreadState<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::CreateLogicalThreadState(this).into()
            }
        }
        unsafe extern "system" fn DeleteLogicalThreadState<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::DeleteLogicalThreadState(this).into()
            }
        }
        unsafe extern "system" fn SwitchInLogicalThreadState<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfibercookie: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::SwitchInLogicalThreadState(this, core::mem::transmute_copy(&pfibercookie)).into()
            }
        }
        unsafe extern "system" fn SwitchOutLogicalThreadState<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfibercookie: *mut *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::SwitchOutLogicalThreadState(this) {
                    Ok(ok__) => {
                        pfibercookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocksHeldByLogicalThread<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::LocksHeldByLogicalThread(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MapFile<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfile: super::super::Foundation::HANDLE, hmapaddress: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::MapFile(this, core::mem::transmute_copy(&hfile)) {
                    Ok(ok__) => {
                        hmapaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConfiguration<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::GetConfiguration(this) {
                    Ok(ok__) => {
                        pconfiguration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn CreateDomain<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfriendlyname: windows_core::PCWSTR, pidentityarray: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::CreateDomain(this, core::mem::transmute(&pwzfriendlyname), core::mem::transmute_copy(&pidentityarray)) {
                    Ok(ok__) => {
                        pappdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultDomain<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::GetDefaultDomain(this) {
                    Ok(ok__) => {
                        pappdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDomains<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::EnumDomains(this, core::mem::transmute_copy(&henum)).into()
            }
        }
        unsafe extern "system" fn NextDomain<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *const core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::NextDomain(this, core::mem::transmute_copy(&henum)) {
                    Ok(ok__) => {
                        pappdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseEnum<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::CloseEnum(this, core::mem::transmute_copy(&henum)).into()
            }
        }
        unsafe extern "system" fn CreateDomainEx<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfriendlyname: windows_core::PCWSTR, psetup: *mut core::ffi::c_void, pevidence: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::CreateDomainEx(this, core::mem::transmute(&pwzfriendlyname), core::mem::transmute_copy(&psetup), core::mem::transmute_copy(&pevidence)) {
                    Ok(ok__) => {
                        pappdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDomainSetup<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomainsetup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::CreateDomainSetup(this) {
                    Ok(ok__) => {
                        pappdomainsetup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEvidence<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevidence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::CreateEvidence(this) {
                    Ok(ok__) => {
                        pevidence.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnloadDomain<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorRuntimeHost_Impl::UnloadDomain(this, core::mem::transmute_copy(&pappdomain)).into()
            }
        }
        unsafe extern "system" fn CurrentDomain<Identity: ICorRuntimeHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorRuntimeHost_Impl::CurrentDomain(this) {
                    Ok(ok__) => {
                        pappdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateLogicalThreadState: CreateLogicalThreadState::<Identity, OFFSET>,
            DeleteLogicalThreadState: DeleteLogicalThreadState::<Identity, OFFSET>,
            SwitchInLogicalThreadState: SwitchInLogicalThreadState::<Identity, OFFSET>,
            SwitchOutLogicalThreadState: SwitchOutLogicalThreadState::<Identity, OFFSET>,
            LocksHeldByLogicalThread: LocksHeldByLogicalThread::<Identity, OFFSET>,
            MapFile: MapFile::<Identity, OFFSET>,
            GetConfiguration: GetConfiguration::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            CreateDomain: CreateDomain::<Identity, OFFSET>,
            GetDefaultDomain: GetDefaultDomain::<Identity, OFFSET>,
            EnumDomains: EnumDomains::<Identity, OFFSET>,
            NextDomain: NextDomain::<Identity, OFFSET>,
            CloseEnum: CloseEnum::<Identity, OFFSET>,
            CreateDomainEx: CreateDomainEx::<Identity, OFFSET>,
            CreateDomainSetup: CreateDomainSetup::<Identity, OFFSET>,
            CreateEvidence: CreateEvidence::<Identity, OFFSET>,
            UnloadDomain: UnloadDomain::<Identity, OFFSET>,
            CurrentDomain: CurrentDomain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorRuntimeHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorRuntimeHost {}
windows_core::imp::define_interface!(ICorThreadpool, ICorThreadpool_Vtbl, 0x84680d3a_b2c1_46e8_acc2_dbc0a359159a);
windows_core::imp::interface_hierarchy!(ICorThreadpool, windows_core::IUnknown);
impl ICorThreadpool {
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorRegisterWaitForSingleObject(&self, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const core::ffi::c_void, timeout: u32, executeonlyonce: bool) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorRegisterWaitForSingleObject)(windows_core::Interface::as_raw(self), phnewwaitobject, hwaitobject, callback, context, timeout, executeonlyonce.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CorUnregisterWait(&self, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorUnregisterWait)(windows_core::Interface::as_raw(self), hwaitobject, completionevent, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, executeonlyonce: bool) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorQueueUserWorkItem)(windows_core::Interface::as_raw(self), function, context, executeonlyonce.into(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorCreateTimer(&self, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const core::ffi::c_void, duetime: u32, period: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorCreateTimer)(windows_core::Interface::as_raw(self), phnewtimer, callback, parameter, duetime, period, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CorChangeTimer(&self, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorChangeTimer)(windows_core::Interface::as_raw(self), timer, duetime, period, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CorDeleteTimer(&self, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorDeleteTimer)(windows_core::Interface::as_raw(self), timer, completionevent, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn CorBindIoCompletionCallback(&self, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CorBindIoCompletionCallback)(windows_core::Interface::as_raw(self), filehandle, callback).ok() }
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorCallOrQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorCallOrQueueUserWorkItem)(windows_core::Interface::as_raw(self), function, context, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CorSetMaxThreads(&self, maxworkerthreads: u32, maxiocompletionthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CorSetMaxThreads)(windows_core::Interface::as_raw(self), maxworkerthreads, maxiocompletionthreads).ok() }
    }
    pub unsafe fn CorGetMaxThreads(&self, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CorGetMaxThreads)(windows_core::Interface::as_raw(self), maxworkerthreads as _, maxiocompletionthreads as _).ok() }
    }
    pub unsafe fn CorGetAvailableThreads(&self, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CorGetAvailableThreads)(windows_core::Interface::as_raw(self), availableworkerthreads as _, availableiocompletionthreads as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorThreadpool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorRegisterWaitForSingleObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, super::Threading::WAITORTIMERCALLBACK, *const core::ffi::c_void, u32, windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorRegisterWaitForSingleObject: usize,
    pub CorUnregisterWait: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorQueueUserWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::Threading::LPTHREAD_START_ROUTINE, *const core::ffi::c_void, windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorQueueUserWorkItem: usize,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorCreateTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::HANDLE, super::Threading::WAITORTIMERCALLBACK, *const core::ffi::c_void, u32, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorCreateTimer: usize,
    pub CorChangeTimer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, u32, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CorDeleteTimer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_IO")]
    pub CorBindIoCompletionCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    CorBindIoCompletionCallback: usize,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorCallOrQueueUserWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::Threading::LPTHREAD_START_ROUTINE, *const core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorCallOrQueueUserWorkItem: usize,
    pub CorSetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub CorGetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub CorGetAvailableThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
pub trait ICorThreadpool_Impl: windows_core::IUnknownImpl {
    fn CorRegisterWaitForSingleObject(&self, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const core::ffi::c_void, timeout: u32, executeonlyonce: windows_core::BOOL) -> windows_core::Result<windows_core::BOOL>;
    fn CorUnregisterWait(&self, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::BOOL>;
    fn CorQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, executeonlyonce: windows_core::BOOL) -> windows_core::Result<windows_core::BOOL>;
    fn CorCreateTimer(&self, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const core::ffi::c_void, duetime: u32, period: u32) -> windows_core::Result<windows_core::BOOL>;
    fn CorChangeTimer(&self, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> windows_core::Result<windows_core::BOOL>;
    fn CorDeleteTimer(&self, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::BOOL>;
    fn CorBindIoCompletionCallback(&self, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::Result<()>;
    fn CorCallOrQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void) -> windows_core::Result<windows_core::BOOL>;
    fn CorSetMaxThreads(&self, maxworkerthreads: u32, maxiocompletionthreads: u32) -> windows_core::Result<()>;
    fn CorGetMaxThreads(&self, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> windows_core::Result<()>;
    fn CorGetAvailableThreads(&self, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl ICorThreadpool_Vtbl {
    pub const fn new<Identity: ICorThreadpool_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CorRegisterWaitForSingleObject<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const core::ffi::c_void, timeout: u32, executeonlyonce: windows_core::BOOL, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorThreadpool_Impl::CorRegisterWaitForSingleObject(this, core::mem::transmute_copy(&phnewwaitobject), core::mem::transmute_copy(&hwaitobject), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&context), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&executeonlyonce)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorUnregisterWait<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorThreadpool_Impl::CorUnregisterWait(this, core::mem::transmute_copy(&hwaitobject), core::mem::transmute_copy(&completionevent)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorQueueUserWorkItem<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, executeonlyonce: windows_core::BOOL, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorThreadpool_Impl::CorQueueUserWorkItem(this, core::mem::transmute_copy(&function), core::mem::transmute_copy(&context), core::mem::transmute_copy(&executeonlyonce)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorCreateTimer<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const core::ffi::c_void, duetime: u32, period: u32, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorThreadpool_Impl::CorCreateTimer(this, core::mem::transmute_copy(&phnewtimer), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&parameter), core::mem::transmute_copy(&duetime), core::mem::transmute_copy(&period)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorChangeTimer<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorThreadpool_Impl::CorChangeTimer(this, core::mem::transmute_copy(&timer), core::mem::transmute_copy(&duetime), core::mem::transmute_copy(&period)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorDeleteTimer<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorThreadpool_Impl::CorDeleteTimer(this, core::mem::transmute_copy(&timer), core::mem::transmute_copy(&completionevent)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorBindIoCompletionCallback<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorThreadpool_Impl::CorBindIoCompletionCallback(this, core::mem::transmute_copy(&filehandle), core::mem::transmute_copy(&callback)).into()
            }
        }
        unsafe extern "system" fn CorCallOrQueueUserWorkItem<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, result: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorThreadpool_Impl::CorCallOrQueueUserWorkItem(this, core::mem::transmute_copy(&function), core::mem::transmute_copy(&context)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorSetMaxThreads<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxworkerthreads: u32, maxiocompletionthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorThreadpool_Impl::CorSetMaxThreads(this, core::mem::transmute_copy(&maxworkerthreads), core::mem::transmute_copy(&maxiocompletionthreads)).into()
            }
        }
        unsafe extern "system" fn CorGetMaxThreads<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorThreadpool_Impl::CorGetMaxThreads(this, core::mem::transmute_copy(&maxworkerthreads), core::mem::transmute_copy(&maxiocompletionthreads)).into()
            }
        }
        unsafe extern "system" fn CorGetAvailableThreads<Identity: ICorThreadpool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorThreadpool_Impl::CorGetAvailableThreads(this, core::mem::transmute_copy(&availableworkerthreads), core::mem::transmute_copy(&availableiocompletionthreads)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CorRegisterWaitForSingleObject: CorRegisterWaitForSingleObject::<Identity, OFFSET>,
            CorUnregisterWait: CorUnregisterWait::<Identity, OFFSET>,
            CorQueueUserWorkItem: CorQueueUserWorkItem::<Identity, OFFSET>,
            CorCreateTimer: CorCreateTimer::<Identity, OFFSET>,
            CorChangeTimer: CorChangeTimer::<Identity, OFFSET>,
            CorDeleteTimer: CorDeleteTimer::<Identity, OFFSET>,
            CorBindIoCompletionCallback: CorBindIoCompletionCallback::<Identity, OFFSET>,
            CorCallOrQueueUserWorkItem: CorCallOrQueueUserWorkItem::<Identity, OFFSET>,
            CorSetMaxThreads: CorSetMaxThreads::<Identity, OFFSET>,
            CorGetMaxThreads: CorGetMaxThreads::<Identity, OFFSET>,
            CorGetAvailableThreads: CorGetAvailableThreads::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorThreadpool as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl windows_core::RuntimeName for ICorThreadpool {}
windows_core::imp::define_interface!(IDebuggerInfo, IDebuggerInfo_Vtbl, 0xbf24142d_a47d_4d24_a66d_8c2141944e44);
windows_core::imp::interface_hierarchy!(IDebuggerInfo, windows_core::IUnknown);
impl IDebuggerInfo {
    pub unsafe fn IsDebuggerAttached(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDebuggerAttached)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebuggerInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDebuggerAttached: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IDebuggerInfo_Impl: windows_core::IUnknownImpl {
    fn IsDebuggerAttached(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IDebuggerInfo_Vtbl {
    pub const fn new<Identity: IDebuggerInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDebuggerAttached<Identity: IDebuggerInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbattached: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebuggerInfo_Impl::IsDebuggerAttached(this) {
                    Ok(ok__) => {
                        pbattached.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsDebuggerAttached: IsDebuggerAttached::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebuggerInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebuggerInfo {}
windows_core::imp::define_interface!(IDebuggerThreadControl, IDebuggerThreadControl_Vtbl, 0x23d86786_0bb5_4774_8fb5_e3522add6246);
windows_core::imp::interface_hierarchy!(IDebuggerThreadControl, windows_core::IUnknown);
impl IDebuggerThreadControl {
    pub unsafe fn ThreadIsBlockingForDebugger(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadIsBlockingForDebugger)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReleaseAllRuntimeThreads(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseAllRuntimeThreads)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StartBlockingForDebugger(&self, dwunused: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartBlockingForDebugger)(windows_core::Interface::as_raw(self), dwunused).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebuggerThreadControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadIsBlockingForDebugger: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseAllRuntimeThreads: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartBlockingForDebugger: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDebuggerThreadControl_Impl: windows_core::IUnknownImpl {
    fn ThreadIsBlockingForDebugger(&self) -> windows_core::Result<()>;
    fn ReleaseAllRuntimeThreads(&self) -> windows_core::Result<()>;
    fn StartBlockingForDebugger(&self, dwunused: u32) -> windows_core::Result<()>;
}
impl IDebuggerThreadControl_Vtbl {
    pub const fn new<Identity: IDebuggerThreadControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThreadIsBlockingForDebugger<Identity: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebuggerThreadControl_Impl::ThreadIsBlockingForDebugger(this).into()
            }
        }
        unsafe extern "system" fn ReleaseAllRuntimeThreads<Identity: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebuggerThreadControl_Impl::ReleaseAllRuntimeThreads(this).into()
            }
        }
        unsafe extern "system" fn StartBlockingForDebugger<Identity: IDebuggerThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwunused: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebuggerThreadControl_Impl::StartBlockingForDebugger(this, core::mem::transmute_copy(&dwunused)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForDebugger: ThreadIsBlockingForDebugger::<Identity, OFFSET>,
            ReleaseAllRuntimeThreads: ReleaseAllRuntimeThreads::<Identity, OFFSET>,
            StartBlockingForDebugger: StartBlockingForDebugger::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebuggerThreadControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebuggerThreadControl {}
windows_core::imp::define_interface!(IGCHost, IGCHost_Vtbl, 0xfac34f6e_0dcd_47b5_8021_531bc5ecca63);
windows_core::imp::interface_hierarchy!(IGCHost, windows_core::IUnknown);
impl IGCHost {
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGCStartupLimits)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok() }
    }
    pub unsafe fn Collect(&self, generation: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Collect)(windows_core::Interface::as_raw(self), generation).ok() }
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStats)(windows_core::Interface::as_raw(self), pstats as _).ok() }
    }
    pub unsafe fn GetThreadStats(&self, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetThreadStats)(windows_core::Interface::as_raw(self), pfibercookie, pstats as _).ok() }
    }
    pub unsafe fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVirtualMemLimit)(windows_core::Interface::as_raw(self), sztmaxvirtualmemmb).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGCStartupLimits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Collect: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_GC_STATS) -> windows_core::HRESULT,
    pub GetThreadStats: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT,
    pub SetVirtualMemLimit: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait IGCHost_Impl: windows_core::IUnknownImpl {
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()>;
    fn Collect(&self, generation: i32) -> windows_core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()>;
    fn GetThreadStats(&self, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> windows_core::Result<()>;
    fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> windows_core::Result<()>;
}
impl IGCHost_Vtbl {
    pub const fn new<Identity: IGCHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGCStartupLimits<Identity: IGCHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCHost_Impl::SetGCStartupLimits(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
            }
        }
        unsafe extern "system" fn Collect<Identity: IGCHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCHost_Impl::Collect(this, core::mem::transmute_copy(&generation)).into()
            }
        }
        unsafe extern "system" fn GetStats<Identity: IGCHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut COR_GC_STATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCHost_Impl::GetStats(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn GetThreadStats<Identity: IGCHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCHost_Impl::GetThreadStats(this, core::mem::transmute_copy(&pfibercookie), core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn SetVirtualMemLimit<Identity: IGCHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztmaxvirtualmemmb: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCHost_Impl::SetVirtualMemLimit(this, core::mem::transmute_copy(&sztmaxvirtualmemmb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCStartupLimits: SetGCStartupLimits::<Identity, OFFSET>,
            Collect: Collect::<Identity, OFFSET>,
            GetStats: GetStats::<Identity, OFFSET>,
            GetThreadStats: GetThreadStats::<Identity, OFFSET>,
            SetVirtualMemLimit: SetVirtualMemLimit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGCHost {}
windows_core::imp::define_interface!(IGCHost2, IGCHost2_Vtbl, 0xa1d70cec_2dbe_4e2f_9291_fdf81438a1df);
impl core::ops::Deref for IGCHost2 {
    type Target = IGCHost;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGCHost2, windows_core::IUnknown, IGCHost);
impl IGCHost2 {
    pub unsafe fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGCStartupLimitsEx)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCHost2_Vtbl {
    pub base__: IGCHost_Vtbl,
    pub SetGCStartupLimitsEx: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
pub trait IGCHost2_Impl: IGCHost_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> windows_core::Result<()>;
}
impl IGCHost2_Vtbl {
    pub const fn new<Identity: IGCHost2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: IGCHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCHost2_Impl::SetGCStartupLimitsEx(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
            }
        }
        Self { base__: IGCHost_Vtbl::new::<Identity, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCHost2 as windows_core::Interface>::IID || iid == &<IGCHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGCHost2 {}
windows_core::imp::define_interface!(IGCHostControl, IGCHostControl_Vtbl, 0x5513d564_8374_4cb9_aed9_0083f4160a1d);
windows_core::imp::interface_hierarchy!(IGCHostControl, windows_core::IUnknown);
impl IGCHostControl {
    pub unsafe fn RequestVirtualMemLimit(&self, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestVirtualMemLimit)(windows_core::Interface::as_raw(self), sztmaxvirtualmemmb, psztnewmaxvirtualmemmb as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCHostControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestVirtualMemLimit: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
}
pub trait IGCHostControl_Impl: windows_core::IUnknownImpl {
    fn RequestVirtualMemLimit(&self, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> windows_core::Result<()>;
}
impl IGCHostControl_Vtbl {
    pub const fn new<Identity: IGCHostControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestVirtualMemLimit<Identity: IGCHostControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCHostControl_Impl::RequestVirtualMemLimit(this, core::mem::transmute_copy(&sztmaxvirtualmemmb), core::mem::transmute_copy(&psztnewmaxvirtualmemmb)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestVirtualMemLimit: RequestVirtualMemLimit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCHostControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGCHostControl {}
windows_core::imp::define_interface!(IGCThreadControl, IGCThreadControl_Vtbl, 0xf31d1788_c397_4725_87a5_6af3472c2791);
windows_core::imp::interface_hierarchy!(IGCThreadControl, windows_core::IUnknown);
impl IGCThreadControl {
    pub unsafe fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadIsBlockingForSuspension)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SuspensionStarting(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspensionStarting)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspensionEnding)(windows_core::Interface::as_raw(self), generation).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGCThreadControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadIsBlockingForSuspension: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionStarting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionEnding: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IGCThreadControl_Impl: windows_core::IUnknownImpl {
    fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()>;
    fn SuspensionStarting(&self) -> windows_core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()>;
}
impl IGCThreadControl_Vtbl {
    pub const fn new<Identity: IGCThreadControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCThreadControl_Impl::ThreadIsBlockingForSuspension(this).into()
            }
        }
        unsafe extern "system" fn SuspensionStarting<Identity: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCThreadControl_Impl::SuspensionStarting(this).into()
            }
        }
        unsafe extern "system" fn SuspensionEnding<Identity: IGCThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGCThreadControl_Impl::SuspensionEnding(this, core::mem::transmute_copy(&generation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCThreadControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGCThreadControl {}
windows_core::imp::define_interface!(IHostAssemblyManager, IHostAssemblyManager_Vtbl, 0x613dabd7_62b2_493e_9e65_c1e32a1e0c5e);
windows_core::imp::interface_hierarchy!(IHostAssemblyManager, windows_core::IUnknown);
impl IHostAssemblyManager {
    pub unsafe fn GetNonHostStoreAssemblies(&self) -> windows_core::Result<ICLRAssemblyReferenceList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNonHostStoreAssemblies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAssemblyStore(&self) -> windows_core::Result<IHostAssemblyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAssemblyStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostAssemblyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNonHostStoreAssemblies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAssemblyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostAssemblyManager_Impl: windows_core::IUnknownImpl {
    fn GetNonHostStoreAssemblies(&self) -> windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetAssemblyStore(&self) -> windows_core::Result<IHostAssemblyStore>;
}
impl IHostAssemblyManager_Vtbl {
    pub const fn new<Identity: IHostAssemblyManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNonHostStoreAssemblies<Identity: IHostAssemblyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreferencelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostAssemblyManager_Impl::GetNonHostStoreAssemblies(this) {
                    Ok(ok__) => {
                        ppreferencelist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAssemblyStore<Identity: IHostAssemblyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppassemblystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostAssemblyManager_Impl::GetAssemblyStore(this) {
                    Ok(ok__) => {
                        ppassemblystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNonHostStoreAssemblies: GetNonHostStoreAssemblies::<Identity, OFFSET>,
            GetAssemblyStore: GetAssemblyStore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostAssemblyManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostAssemblyManager {}
windows_core::imp::define_interface!(IHostAssemblyStore, IHostAssemblyStore_Vtbl, 0x7b102a88_3f7f_496d_8fa2_c35374e01af3);
windows_core::imp::interface_hierarchy!(IHostAssemblyStore, windows_core::IUnknown);
impl IHostAssemblyStore {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProvideAssembly(&self, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut Option<super::Com::IStream>, ppstmpdb: *mut Option<super::Com::IStream>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProvideAssembly)(windows_core::Interface::as_raw(self), pbindinfo, passemblyid as _, pcontext as _, core::mem::transmute(ppstmassemblyimage), core::mem::transmute(ppstmpdb)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProvideModule(&self, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut Option<super::Com::IStream>, ppstmpdb: *mut Option<super::Com::IStream>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProvideModule)(windows_core::Interface::as_raw(self), pbindinfo, pdwmoduleid as _, core::mem::transmute(ppstmmoduleimage), core::mem::transmute(ppstmpdb)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostAssemblyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ProvideAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, *const AssemblyBindInfo, *mut u64, *mut u64, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProvideAssembly: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProvideModule: unsafe extern "system" fn(*mut core::ffi::c_void, *const ModuleBindInfo, *mut u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProvideModule: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHostAssemblyStore_Impl: windows_core::IUnknownImpl {
    fn ProvideAssembly(&self, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: windows_core::OutRef<super::Com::IStream>, ppstmpdb: windows_core::OutRef<super::Com::IStream>) -> windows_core::Result<()>;
    fn ProvideModule(&self, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: windows_core::OutRef<super::Com::IStream>, ppstmpdb: windows_core::OutRef<super::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IHostAssemblyStore_Vtbl {
    pub const fn new<Identity: IHostAssemblyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProvideAssembly<Identity: IHostAssemblyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut *mut core::ffi::c_void, ppstmpdb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostAssemblyStore_Impl::ProvideAssembly(this, core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&passemblyid), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&ppstmassemblyimage), core::mem::transmute_copy(&ppstmpdb)).into()
            }
        }
        unsafe extern "system" fn ProvideModule<Identity: IHostAssemblyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut *mut core::ffi::c_void, ppstmpdb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostAssemblyStore_Impl::ProvideModule(this, core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&pdwmoduleid), core::mem::transmute_copy(&ppstmmoduleimage), core::mem::transmute_copy(&ppstmpdb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProvideAssembly: ProvideAssembly::<Identity, OFFSET>,
            ProvideModule: ProvideModule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostAssemblyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHostAssemblyStore {}
windows_core::imp::define_interface!(IHostAutoEvent, IHostAutoEvent_Vtbl, 0x50b0cfce_4063_4278_9673_e5cb4ed0bdb8);
windows_core::imp::interface_hierarchy!(IHostAutoEvent, windows_core::IUnknown);
impl IHostAutoEvent {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok() }
    }
    pub unsafe fn Set(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostAutoEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostAutoEvent_Impl: windows_core::IUnknownImpl {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn Set(&self) -> windows_core::Result<()>;
}
impl IHostAutoEvent_Vtbl {
    pub const fn new<Identity: IHostAutoEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Wait<Identity: IHostAutoEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostAutoEvent_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn Set<Identity: IHostAutoEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostAutoEvent_Impl::Set(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Wait: Wait::<Identity, OFFSET>, Set: Set::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostAutoEvent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostAutoEvent {}
windows_core::imp::define_interface!(IHostControl, IHostControl_Vtbl, 0x02ca073c_7079_4860_880a_c2f7a449c991);
windows_core::imp::interface_hierarchy!(IHostControl, windows_core::IUnknown);
impl IHostControl {
    pub unsafe fn GetHostManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHostManager)(windows_core::Interface::as_raw(self), riid, ppobject as _).ok() }
    }
    pub unsafe fn SetAppDomainManager<P1>(&self, dwappdomainid: u32, punkappdomainmanager: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAppDomainManager)(windows_core::Interface::as_raw(self), dwappdomainid, punkappdomainmanager.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHostManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppDomainManager: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostControl_Impl: windows_core::IUnknownImpl {
    fn GetHostManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetAppDomainManager(&self, dwappdomainid: u32, punkappdomainmanager: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IHostControl_Vtbl {
    pub const fn new<Identity: IHostControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHostManager<Identity: IHostControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostControl_Impl::GetHostManager(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn SetAppDomainManager<Identity: IHostControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, punkappdomainmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostControl_Impl::SetAppDomainManager(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&punkappdomainmanager)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHostManager: GetHostManager::<Identity, OFFSET>,
            SetAppDomainManager: SetAppDomainManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostControl {}
windows_core::imp::define_interface!(IHostCrst, IHostCrst_Vtbl, 0x6df710a6_26a4_4a65_8cd5_7237b8bda8dc);
windows_core::imp::interface_hierarchy!(IHostCrst, windows_core::IUnknown);
impl IHostCrst {
    pub unsafe fn Enter(&self, option: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self), option).ok() }
    }
    pub unsafe fn Leave(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TryEnter(&self, option: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryEnter)(windows_core::Interface::as_raw(self), option, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpinCount(&self, dwspincount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSpinCount)(windows_core::Interface::as_raw(self), dwspincount).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostCrst_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryEnter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetSpinCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IHostCrst_Impl: windows_core::IUnknownImpl {
    fn Enter(&self, option: u32) -> windows_core::Result<()>;
    fn Leave(&self) -> windows_core::Result<()>;
    fn TryEnter(&self, option: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetSpinCount(&self, dwspincount: u32) -> windows_core::Result<()>;
}
impl IHostCrst_Vtbl {
    pub const fn new<Identity: IHostCrst_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enter<Identity: IHostCrst_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostCrst_Impl::Enter(this, core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn Leave<Identity: IHostCrst_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostCrst_Impl::Leave(this).into()
            }
        }
        unsafe extern "system" fn TryEnter<Identity: IHostCrst_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: u32, pbsucceeded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostCrst_Impl::TryEnter(this, core::mem::transmute_copy(&option)) {
                    Ok(ok__) => {
                        pbsucceeded.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpinCount<Identity: IHostCrst_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspincount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostCrst_Impl::SetSpinCount(this, core::mem::transmute_copy(&dwspincount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
            TryEnter: TryEnter::<Identity, OFFSET>,
            SetSpinCount: SetSpinCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostCrst as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostCrst {}
windows_core::imp::define_interface!(IHostGCManager, IHostGCManager_Vtbl, 0x5d4ec34e_f248_457b_b603_255faaba0d21);
windows_core::imp::interface_hierarchy!(IHostGCManager, windows_core::IUnknown);
impl IHostGCManager {
    pub unsafe fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadIsBlockingForSuspension)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SuspensionStarting(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspensionStarting)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspensionEnding)(windows_core::Interface::as_raw(self), generation).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostGCManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadIsBlockingForSuspension: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionStarting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionEnding: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IHostGCManager_Impl: windows_core::IUnknownImpl {
    fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()>;
    fn SuspensionStarting(&self) -> windows_core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()>;
}
impl IHostGCManager_Vtbl {
    pub const fn new<Identity: IHostGCManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: IHostGCManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostGCManager_Impl::ThreadIsBlockingForSuspension(this).into()
            }
        }
        unsafe extern "system" fn SuspensionStarting<Identity: IHostGCManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostGCManager_Impl::SuspensionStarting(this).into()
            }
        }
        unsafe extern "system" fn SuspensionEnding<Identity: IHostGCManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostGCManager_Impl::SuspensionEnding(this, core::mem::transmute_copy(&generation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostGCManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostGCManager {}
windows_core::imp::define_interface!(IHostIoCompletionManager, IHostIoCompletionManager_Vtbl, 0x8bde9d80_ec06_41d6_83e6_22580effcc20);
windows_core::imp::interface_hierarchy!(IHostIoCompletionManager, windows_core::IUnknown);
impl IHostIoCompletionManager {
    pub unsafe fn CreateIoCompletionPort(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateIoCompletionPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CloseIoCompletionPort(&self, hport: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseIoCompletionPort)(windows_core::Interface::as_raw(self), hport).ok() }
    }
    pub unsafe fn SetMaxThreads(&self, dwmaxiocompletionthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxThreads)(windows_core::Interface::as_raw(self), dwmaxiocompletionthreads).ok() }
    }
    pub unsafe fn GetMaxThreads(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAvailableThreads(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHostOverlappedSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHostOverlappedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCLRIoCompletionManager<P0>(&self, pmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRIoCompletionManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCLRIoCompletionManager)(windows_core::Interface::as_raw(self), pmanager.param().abi()).ok() }
    }
    pub unsafe fn InitializeHostOverlapped(&self, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeHostOverlapped)(windows_core::Interface::as_raw(self), pvoverlapped).ok() }
    }
    pub unsafe fn Bind(&self, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), hport, hhandle).ok() }
    }
    pub unsafe fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinThreads)(windows_core::Interface::as_raw(self), dwminiocompletionthreads).ok() }
    }
    pub unsafe fn GetMinThreads(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostIoCompletionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateIoCompletionPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CloseIoCompletionPort: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAvailableThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetHostOverlappedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCLRIoCompletionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeHostOverlapped: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMinThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IHostIoCompletionManager_Impl: windows_core::IUnknownImpl {
    fn CreateIoCompletionPort(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn CloseIoCompletionPort(&self, hport: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetMaxThreads(&self, dwmaxiocompletionthreads: u32) -> windows_core::Result<()>;
    fn GetMaxThreads(&self) -> windows_core::Result<u32>;
    fn GetAvailableThreads(&self) -> windows_core::Result<u32>;
    fn GetHostOverlappedSize(&self) -> windows_core::Result<u32>;
    fn SetCLRIoCompletionManager(&self, pmanager: windows_core::Ref<ICLRIoCompletionManager>) -> windows_core::Result<()>;
    fn InitializeHostOverlapped(&self, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Bind(&self, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()>;
    fn GetMinThreads(&self) -> windows_core::Result<u32>;
}
impl IHostIoCompletionManager_Vtbl {
    pub const fn new<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateIoCompletionPort<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phport: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostIoCompletionManager_Impl::CreateIoCompletionPort(this) {
                    Ok(ok__) => {
                        phport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseIoCompletionPort<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hport: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostIoCompletionManager_Impl::CloseIoCompletionPort(this, core::mem::transmute_copy(&hport)).into()
            }
        }
        unsafe extern "system" fn SetMaxThreads<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxiocompletionthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostIoCompletionManager_Impl::SetMaxThreads(this, core::mem::transmute_copy(&dwmaxiocompletionthreads)).into()
            }
        }
        unsafe extern "system" fn GetMaxThreads<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxiocompletionthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostIoCompletionManager_Impl::GetMaxThreads(this) {
                    Ok(ok__) => {
                        pdwmaxiocompletionthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwavailableiocompletionthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostIoCompletionManager_Impl::GetAvailableThreads(this) {
                    Ok(ok__) => {
                        pdwavailableiocompletionthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHostOverlappedSize<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostIoCompletionManager_Impl::GetHostOverlappedSize(this) {
                    Ok(ok__) => {
                        pcbsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCLRIoCompletionManager<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostIoCompletionManager_Impl::SetCLRIoCompletionManager(this, core::mem::transmute_copy(&pmanager)).into()
            }
        }
        unsafe extern "system" fn InitializeHostOverlapped<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoverlapped: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostIoCompletionManager_Impl::InitializeHostOverlapped(this, core::mem::transmute_copy(&pvoverlapped)).into()
            }
        }
        unsafe extern "system" fn Bind<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostIoCompletionManager_Impl::Bind(this, core::mem::transmute_copy(&hport), core::mem::transmute_copy(&hhandle)).into()
            }
        }
        unsafe extern "system" fn SetMinThreads<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminiocompletionthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostIoCompletionManager_Impl::SetMinThreads(this, core::mem::transmute_copy(&dwminiocompletionthreads)).into()
            }
        }
        unsafe extern "system" fn GetMinThreads<Identity: IHostIoCompletionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostIoCompletionManager_Impl::GetMinThreads(this) {
                    Ok(ok__) => {
                        pdwminiocompletionthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIoCompletionPort: CreateIoCompletionPort::<Identity, OFFSET>,
            CloseIoCompletionPort: CloseIoCompletionPort::<Identity, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, OFFSET>,
            GetHostOverlappedSize: GetHostOverlappedSize::<Identity, OFFSET>,
            SetCLRIoCompletionManager: SetCLRIoCompletionManager::<Identity, OFFSET>,
            InitializeHostOverlapped: InitializeHostOverlapped::<Identity, OFFSET>,
            Bind: Bind::<Identity, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostIoCompletionManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostIoCompletionManager {}
windows_core::imp::define_interface!(IHostMalloc, IHostMalloc_Vtbl, 0x1831991c_cc53_4a31_b218_04e910446479);
windows_core::imp::interface_hierarchy!(IHostMalloc, windows_core::IUnknown);
impl IHostMalloc {
    pub unsafe fn Alloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cbsize, ecriticallevel, ppmem as _).ok() }
    }
    pub unsafe fn DebugAlloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DebugAlloc)(windows_core::Interface::as_raw(self), cbsize, ecriticallevel, pszfilename, ilineno, ppmem as _).ok() }
    }
    pub unsafe fn Free(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pmem).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostMalloc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize, EMemoryCriticalLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize, EMemoryCriticalLevel, *const u8, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostMalloc_Impl: windows_core::IUnknownImpl {
    fn Alloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DebugAlloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Free(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IHostMalloc_Vtbl {
    pub const fn new<Identity: IHostMalloc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alloc<Identity: IHostMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&ecriticallevel), core::mem::transmute_copy(&ppmem)).into()
            }
        }
        unsafe extern "system" fn DebugAlloc<Identity: IHostMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMalloc_Impl::DebugAlloc(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&ecriticallevel), core::mem::transmute_copy(&pszfilename), core::mem::transmute_copy(&ilineno), core::mem::transmute_copy(&ppmem)).into()
            }
        }
        unsafe extern "system" fn Free<Identity: IHostMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMalloc_Impl::Free(this, core::mem::transmute_copy(&pmem)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, OFFSET>,
            DebugAlloc: DebugAlloc::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostMalloc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostMalloc {}
windows_core::imp::define_interface!(IHostManualEvent, IHostManualEvent_Vtbl, 0x1bf4ec38_affe_4fb9_85a6_525268f15b54);
windows_core::imp::interface_hierarchy!(IHostManualEvent, windows_core::IUnknown);
impl IHostManualEvent {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Set(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostManualEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostManualEvent_Impl: windows_core::IUnknownImpl {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Set(&self) -> windows_core::Result<()>;
}
impl IHostManualEvent_Vtbl {
    pub const fn new<Identity: IHostManualEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Wait<Identity: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostManualEvent_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostManualEvent_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Set<Identity: IHostManualEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostManualEvent_Impl::Set(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Set: Set::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostManualEvent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostManualEvent {}
windows_core::imp::define_interface!(IHostMemoryManager, IHostMemoryManager_Vtbl, 0x7bc698d1_f9e3_4460_9cde_d04248e9fa25);
windows_core::imp::interface_hierarchy!(IHostMemoryManager, windows_core::IUnknown);
impl IHostMemoryManager {
    pub unsafe fn CreateMalloc(&self, dwmalloctype: u32) -> windows_core::Result<IHostMalloc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMalloc)(windows_core::Interface::as_raw(self), dwmalloctype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn VirtualAlloc(&self, paddress: *const core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).VirtualAlloc)(windows_core::Interface::as_raw(self), paddress, dwsize, flallocationtype, flprotect, ecriticallevel, ppmem as _).ok() }
    }
    pub unsafe fn VirtualFree(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).VirtualFree)(windows_core::Interface::as_raw(self), lpaddress, dwsize, dwfreetype).ok() }
    }
    pub unsafe fn VirtualQuery(&self, lpaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwlength: usize, presult: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).VirtualQuery)(windows_core::Interface::as_raw(self), lpaddress, lpbuffer as _, dwlength, presult as _).ok() }
    }
    pub unsafe fn VirtualProtect(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VirtualProtect)(windows_core::Interface::as_raw(self), lpaddress, dwsize, flnewprotect, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMemoryLoad(&self, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMemoryLoad)(windows_core::Interface::as_raw(self), pmemoryload as _, pavailablebytes as _).ok() }
    }
    pub unsafe fn RegisterMemoryNotificationCallback<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRMemoryNotificationCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterMemoryNotificationCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn NeedsVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NeedsVirtualAddressSpace)(windows_core::Interface::as_raw(self), startaddress, size).ok() }
    }
    pub unsafe fn AcquiredVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcquiredVirtualAddressSpace)(windows_core::Interface::as_raw(self), startaddress, size).ok() }
    }
    pub unsafe fn ReleasedVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleasedVirtualAddressSpace)(windows_core::Interface::as_raw(self), startaddress).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostMemoryManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateMalloc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VirtualAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, u32, u32, EMemoryCriticalLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VirtualFree: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub VirtualQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub VirtualProtect: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMemoryLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub RegisterMemoryNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NeedsVirtualAddressSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AcquiredVirtualAddressSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ReleasedVirtualAddressSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostMemoryManager_Impl: windows_core::IUnknownImpl {
    fn CreateMalloc(&self, dwmalloctype: u32) -> windows_core::Result<IHostMalloc>;
    fn VirtualAlloc(&self, paddress: *const core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn VirtualFree(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::Result<()>;
    fn VirtualQuery(&self, lpaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwlength: usize, presult: *mut usize) -> windows_core::Result<()>;
    fn VirtualProtect(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32) -> windows_core::Result<u32>;
    fn GetMemoryLoad(&self, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> windows_core::Result<()>;
    fn RegisterMemoryNotificationCallback(&self, pcallback: windows_core::Ref<ICLRMemoryNotificationCallback>) -> windows_core::Result<()>;
    fn NeedsVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()>;
    fn AcquiredVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()>;
    fn ReleasedVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IHostMemoryManager_Vtbl {
    pub const fn new<Identity: IHostMemoryManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMalloc<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmalloctype: u32, ppmalloc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostMemoryManager_Impl::CreateMalloc(this, core::mem::transmute_copy(&dwmalloctype)) {
                    Ok(ok__) => {
                        ppmalloc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VirtualAlloc<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *const core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::VirtualAlloc(this, core::mem::transmute_copy(&paddress), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&flallocationtype), core::mem::transmute_copy(&flprotect), core::mem::transmute_copy(&ecriticallevel), core::mem::transmute_copy(&ppmem)).into()
            }
        }
        unsafe extern "system" fn VirtualFree<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpaddress: *const core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::VirtualFree(this, core::mem::transmute_copy(&lpaddress), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwfreetype)).into()
            }
        }
        unsafe extern "system" fn VirtualQuery<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwlength: usize, presult: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::VirtualQuery(this, core::mem::transmute_copy(&lpaddress), core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn VirtualProtect<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostMemoryManager_Impl::VirtualProtect(this, core::mem::transmute_copy(&lpaddress), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&flnewprotect)) {
                    Ok(ok__) => {
                        pfloldprotect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMemoryLoad<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::GetMemoryLoad(this, core::mem::transmute_copy(&pmemoryload), core::mem::transmute_copy(&pavailablebytes)).into()
            }
        }
        unsafe extern "system" fn RegisterMemoryNotificationCallback<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::RegisterMemoryNotificationCallback(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn NeedsVirtualAddressSpace<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::NeedsVirtualAddressSpace(this, core::mem::transmute_copy(&startaddress), core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn AcquiredVirtualAddressSpace<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::AcquiredVirtualAddressSpace(this, core::mem::transmute_copy(&startaddress), core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn ReleasedVirtualAddressSpace<Identity: IHostMemoryManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startaddress: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostMemoryManager_Impl::ReleasedVirtualAddressSpace(this, core::mem::transmute_copy(&startaddress)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMalloc: CreateMalloc::<Identity, OFFSET>,
            VirtualAlloc: VirtualAlloc::<Identity, OFFSET>,
            VirtualFree: VirtualFree::<Identity, OFFSET>,
            VirtualQuery: VirtualQuery::<Identity, OFFSET>,
            VirtualProtect: VirtualProtect::<Identity, OFFSET>,
            GetMemoryLoad: GetMemoryLoad::<Identity, OFFSET>,
            RegisterMemoryNotificationCallback: RegisterMemoryNotificationCallback::<Identity, OFFSET>,
            NeedsVirtualAddressSpace: NeedsVirtualAddressSpace::<Identity, OFFSET>,
            AcquiredVirtualAddressSpace: AcquiredVirtualAddressSpace::<Identity, OFFSET>,
            ReleasedVirtualAddressSpace: ReleasedVirtualAddressSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostMemoryManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostMemoryManager {}
windows_core::imp::define_interface!(IHostPolicyManager, IHostPolicyManager_Vtbl, 0x7ae49844_b1e3_4683_ba7c_1e8212ea3b79);
windows_core::imp::interface_hierarchy!(IHostPolicyManager, windows_core::IUnknown);
impl IHostPolicyManager {
    pub unsafe fn OnDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDefaultAction)(windows_core::Interface::as_raw(self), operation, action).ok() }
    }
    pub unsafe fn OnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTimeout)(windows_core::Interface::as_raw(self), operation, action).ok() }
    }
    pub unsafe fn OnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnFailure)(windows_core::Interface::as_raw(self), failure, action).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostPolicyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub OnTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub OnFailure: unsafe extern "system" fn(*mut core::ffi::c_void, EClrFailure, EPolicyAction) -> windows_core::HRESULT,
}
pub trait IHostPolicyManager_Impl: windows_core::IUnknownImpl {
    fn OnDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn OnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn OnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()>;
}
impl IHostPolicyManager_Vtbl {
    pub const fn new<Identity: IHostPolicyManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDefaultAction<Identity: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostPolicyManager_Impl::OnDefaultAction(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn OnTimeout<Identity: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostPolicyManager_Impl::OnTimeout(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn OnFailure<Identity: IHostPolicyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostPolicyManager_Impl::OnFailure(this, core::mem::transmute_copy(&failure), core::mem::transmute_copy(&action)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDefaultAction: OnDefaultAction::<Identity, OFFSET>,
            OnTimeout: OnTimeout::<Identity, OFFSET>,
            OnFailure: OnFailure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostPolicyManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostPolicyManager {}
windows_core::imp::define_interface!(IHostSecurityContext, IHostSecurityContext_Vtbl, 0x7e573ce4_0343_4423_98d7_6318348a1d3c);
windows_core::imp::interface_hierarchy!(IHostSecurityContext, windows_core::IUnknown);
impl IHostSecurityContext {
    pub unsafe fn Capture(&self) -> windows_core::Result<IHostSecurityContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSecurityContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostSecurityContext_Impl: windows_core::IUnknownImpl {
    fn Capture(&self) -> windows_core::Result<IHostSecurityContext>;
}
impl IHostSecurityContext_Vtbl {
    pub const fn new<Identity: IHostSecurityContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capture<Identity: IHostSecurityContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclonedcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSecurityContext_Impl::Capture(this) {
                    Ok(ok__) => {
                        ppclonedcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Capture: Capture::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSecurityContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostSecurityContext {}
windows_core::imp::define_interface!(IHostSecurityManager, IHostSecurityManager_Vtbl, 0x75ad2468_a349_4d02_a764_76a68aee0c4f);
windows_core::imp::interface_hierarchy!(IHostSecurityManager, windows_core::IUnknown);
impl IHostSecurityManager {
    pub unsafe fn ImpersonateLoggedOnUser(&self, htoken: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImpersonateLoggedOnUser)(windows_core::Interface::as_raw(self), htoken).ok() }
    }
    pub unsafe fn RevertToSelf(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevertToSelf)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OpenThreadToken(&self, dwdesiredaccess: u32, bopenasself: bool) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenThreadToken)(windows_core::Interface::as_raw(self), dwdesiredaccess, bopenasself.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetThreadToken(&self, htoken: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetThreadToken)(windows_core::Interface::as_raw(self), htoken).ok() }
    }
    pub unsafe fn GetSecurityContext(&self, econtexttype: EContextType) -> windows_core::Result<IHostSecurityContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityContext)(windows_core::Interface::as_raw(self), econtexttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSecurityContext<P1>(&self, econtexttype: EContextType, psecuritycontext: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IHostSecurityContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityContext)(windows_core::Interface::as_raw(self), econtexttype, psecuritycontext.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSecurityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ImpersonateLoggedOnUser: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenThreadToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetThreadToken: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void, EContextType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void, EContextType, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostSecurityManager_Impl: windows_core::IUnknownImpl {
    fn ImpersonateLoggedOnUser(&self, htoken: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn RevertToSelf(&self) -> windows_core::Result<()>;
    fn OpenThreadToken(&self, dwdesiredaccess: u32, bopenasself: windows_core::BOOL) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn SetThreadToken(&self, htoken: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetSecurityContext(&self, econtexttype: EContextType) -> windows_core::Result<IHostSecurityContext>;
    fn SetSecurityContext(&self, econtexttype: EContextType, psecuritycontext: windows_core::Ref<IHostSecurityContext>) -> windows_core::Result<()>;
}
impl IHostSecurityManager_Vtbl {
    pub const fn new<Identity: IHostSecurityManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImpersonateLoggedOnUser<Identity: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostSecurityManager_Impl::ImpersonateLoggedOnUser(this, core::mem::transmute_copy(&htoken)).into()
            }
        }
        unsafe extern "system" fn RevertToSelf<Identity: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostSecurityManager_Impl::RevertToSelf(this).into()
            }
        }
        unsafe extern "system" fn OpenThreadToken<Identity: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdesiredaccess: u32, bopenasself: windows_core::BOOL, phthreadtoken: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSecurityManager_Impl::OpenThreadToken(this, core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&bopenasself)) {
                    Ok(ok__) => {
                        phthreadtoken.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetThreadToken<Identity: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostSecurityManager_Impl::SetThreadToken(this, core::mem::transmute_copy(&htoken)).into()
            }
        }
        unsafe extern "system" fn GetSecurityContext<Identity: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, econtexttype: EContextType, ppsecuritycontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSecurityManager_Impl::GetSecurityContext(this, core::mem::transmute_copy(&econtexttype)) {
                    Ok(ok__) => {
                        ppsecuritycontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityContext<Identity: IHostSecurityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, econtexttype: EContextType, psecuritycontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostSecurityManager_Impl::SetSecurityContext(this, core::mem::transmute_copy(&econtexttype), core::mem::transmute_copy(&psecuritycontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ImpersonateLoggedOnUser: ImpersonateLoggedOnUser::<Identity, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, OFFSET>,
            OpenThreadToken: OpenThreadToken::<Identity, OFFSET>,
            SetThreadToken: SetThreadToken::<Identity, OFFSET>,
            GetSecurityContext: GetSecurityContext::<Identity, OFFSET>,
            SetSecurityContext: SetSecurityContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSecurityManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostSecurityManager {}
windows_core::imp::define_interface!(IHostSemaphore, IHostSemaphore_Vtbl, 0x855efd47_cc09_463a_a97d_16acab882661);
windows_core::imp::interface_hierarchy!(IHostSemaphore, windows_core::IUnknown);
impl IHostSemaphore {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok() }
    }
    pub unsafe fn ReleaseSemaphore(&self, lreleasecount: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReleaseSemaphore)(windows_core::Interface::as_raw(self), lreleasecount, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSemaphore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ReleaseSemaphore: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IHostSemaphore_Impl: windows_core::IUnknownImpl {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn ReleaseSemaphore(&self, lreleasecount: i32) -> windows_core::Result<i32>;
}
impl IHostSemaphore_Vtbl {
    pub const fn new<Identity: IHostSemaphore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Wait<Identity: IHostSemaphore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostSemaphore_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn ReleaseSemaphore<Identity: IHostSemaphore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lreleasecount: i32, lppreviouscount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSemaphore_Impl::ReleaseSemaphore(this, core::mem::transmute_copy(&lreleasecount)) {
                    Ok(ok__) => {
                        lppreviouscount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, OFFSET>,
            ReleaseSemaphore: ReleaseSemaphore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSemaphore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostSemaphore {}
windows_core::imp::define_interface!(IHostSyncManager, IHostSyncManager_Vtbl, 0x234330c7_5f10_4f20_9615_5122dab7a0ac);
windows_core::imp::interface_hierarchy!(IHostSyncManager, windows_core::IUnknown);
impl IHostSyncManager {
    pub unsafe fn SetCLRSyncManager<P0>(&self, pmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRSyncManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCLRSyncManager)(windows_core::Interface::as_raw(self), pmanager.param().abi()).ok() }
    }
    pub unsafe fn CreateCrst(&self) -> windows_core::Result<IHostCrst> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCrst)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCrstWithSpinCount(&self, dwspincount: u32) -> windows_core::Result<IHostCrst> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCrstWithSpinCount)(windows_core::Interface::as_raw(self), dwspincount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAutoEvent(&self) -> windows_core::Result<IHostAutoEvent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAutoEvent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateManualEvent(&self, binitialstate: bool) -> windows_core::Result<IHostManualEvent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateManualEvent)(windows_core::Interface::as_raw(self), binitialstate.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMonitorEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMonitorEvent)(windows_core::Interface::as_raw(self), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRWLockWriterEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRWLockWriterEvent)(windows_core::Interface::as_raw(self), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRWLockReaderEvent(&self, binitialstate: bool, cookie: usize) -> windows_core::Result<IHostManualEvent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRWLockReaderEvent)(windows_core::Interface::as_raw(self), binitialstate.into(), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSemaphoreA(&self, dwinitial: u32, dwmax: u32) -> windows_core::Result<IHostSemaphore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSemaphoreA)(windows_core::Interface::as_raw(self), dwinitial, dwmax, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostSyncManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCLRSyncManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCrst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCrstWithSpinCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAutoEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateManualEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMonitorEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRWLockWriterEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRWLockReaderEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSemaphoreA: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostSyncManager_Impl: windows_core::IUnknownImpl {
    fn SetCLRSyncManager(&self, pmanager: windows_core::Ref<ICLRSyncManager>) -> windows_core::Result<()>;
    fn CreateCrst(&self) -> windows_core::Result<IHostCrst>;
    fn CreateCrstWithSpinCount(&self, dwspincount: u32) -> windows_core::Result<IHostCrst>;
    fn CreateAutoEvent(&self) -> windows_core::Result<IHostAutoEvent>;
    fn CreateManualEvent(&self, binitialstate: windows_core::BOOL) -> windows_core::Result<IHostManualEvent>;
    fn CreateMonitorEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockWriterEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockReaderEvent(&self, binitialstate: windows_core::BOOL, cookie: usize) -> windows_core::Result<IHostManualEvent>;
    fn CreateSemaphoreA(&self, dwinitial: u32, dwmax: u32) -> windows_core::Result<IHostSemaphore>;
}
impl IHostSyncManager_Vtbl {
    pub const fn new<Identity: IHostSyncManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCLRSyncManager<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostSyncManager_Impl::SetCLRSyncManager(this, core::mem::transmute_copy(&pmanager)).into()
            }
        }
        unsafe extern "system" fn CreateCrst<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcrst: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateCrst(this) {
                    Ok(ok__) => {
                        ppcrst.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCrstWithSpinCount<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspincount: u32, ppcrst: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateCrstWithSpinCount(this, core::mem::transmute_copy(&dwspincount)) {
                    Ok(ok__) => {
                        ppcrst.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAutoEvent<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateAutoEvent(this) {
                    Ok(ok__) => {
                        ppevent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateManualEvent<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binitialstate: windows_core::BOOL, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateManualEvent(this, core::mem::transmute_copy(&binitialstate)) {
                    Ok(ok__) => {
                        ppevent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMonitorEvent<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateMonitorEvent(this, core::mem::transmute_copy(&cookie)) {
                    Ok(ok__) => {
                        ppevent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRWLockWriterEvent<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateRWLockWriterEvent(this, core::mem::transmute_copy(&cookie)) {
                    Ok(ok__) => {
                        ppevent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRWLockReaderEvent<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binitialstate: windows_core::BOOL, cookie: usize, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateRWLockReaderEvent(this, core::mem::transmute_copy(&binitialstate), core::mem::transmute_copy(&cookie)) {
                    Ok(ok__) => {
                        ppevent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSemaphoreA<Identity: IHostSyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinitial: u32, dwmax: u32, ppsemaphore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostSyncManager_Impl::CreateSemaphoreA(this, core::mem::transmute_copy(&dwinitial), core::mem::transmute_copy(&dwmax)) {
                    Ok(ok__) => {
                        ppsemaphore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCLRSyncManager: SetCLRSyncManager::<Identity, OFFSET>,
            CreateCrst: CreateCrst::<Identity, OFFSET>,
            CreateCrstWithSpinCount: CreateCrstWithSpinCount::<Identity, OFFSET>,
            CreateAutoEvent: CreateAutoEvent::<Identity, OFFSET>,
            CreateManualEvent: CreateManualEvent::<Identity, OFFSET>,
            CreateMonitorEvent: CreateMonitorEvent::<Identity, OFFSET>,
            CreateRWLockWriterEvent: CreateRWLockWriterEvent::<Identity, OFFSET>,
            CreateRWLockReaderEvent: CreateRWLockReaderEvent::<Identity, OFFSET>,
            CreateSemaphoreA: CreateSemaphoreA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSyncManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostSyncManager {}
windows_core::imp::define_interface!(IHostTask, IHostTask_Vtbl, 0xc2275828_c4b1_4b55_82c9_92135f74df1a);
windows_core::imp::interface_hierarchy!(IHostTask, windows_core::IUnknown);
impl IHostTask {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Alert(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Alert)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Join(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Join)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok() }
    }
    pub unsafe fn SetPriority(&self, newpriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), newpriority).ok() }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCLRTask<P0>(&self, pclrtask: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRTask>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCLRTask)(windows_core::Interface::as_raw(self), pclrtask.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostTask_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Alert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Join: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCLRTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHostTask_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Alert(&self) -> windows_core::Result<()>;
    fn Join(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn SetPriority(&self, newpriority: i32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn SetCLRTask(&self, pclrtask: windows_core::Ref<ICLRTask>) -> windows_core::Result<()>;
}
impl IHostTask_Vtbl {
    pub const fn new<Identity: IHostTask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IHostTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTask_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Alert<Identity: IHostTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTask_Impl::Alert(this).into()
            }
        }
        unsafe extern "system" fn Join<Identity: IHostTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTask_Impl::Join(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IHostTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTask_Impl::SetPriority(this, core::mem::transmute_copy(&newpriority)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IHostTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostTask_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        ppriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCLRTask<Identity: IHostTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclrtask: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTask_Impl::SetCLRTask(this, core::mem::transmute_copy(&pclrtask)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Alert: Alert::<Identity, OFFSET>,
            Join: Join::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetCLRTask: SetCLRTask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostTask as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHostTask {}
windows_core::imp::define_interface!(IHostTaskManager, IHostTaskManager_Vtbl, 0x997ff24c_43b7_4352_8667_0dc04fafd354);
windows_core::imp::interface_hierarchy!(IHostTaskManager, windows_core::IUnknown);
impl IHostTaskManager {
    pub unsafe fn GetCurrentTask(&self) -> windows_core::Result<IHostTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentTask)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CreateTask(&self, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const core::ffi::c_void) -> windows_core::Result<IHostTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), dwstacksize, pstartaddress, pparameter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Sleep(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Sleep)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok() }
    }
    pub unsafe fn SwitchToTask(&self, option: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SwitchToTask)(windows_core::Interface::as_raw(self), option).ok() }
    }
    pub unsafe fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUILocale)(windows_core::Interface::as_raw(self), lcid).ok() }
    }
    pub unsafe fn SetLocale(&self, lcid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), lcid).ok() }
    }
    pub unsafe fn CallNeedsHostHook(&self, target: usize) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallNeedsHostHook)(windows_core::Interface::as_raw(self), target, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LeaveRuntime(&self, target: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LeaveRuntime)(windows_core::Interface::as_raw(self), target).ok() }
    }
    pub unsafe fn EnterRuntime(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnterRuntime)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReverseLeaveRuntime(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReverseLeaveRuntime)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReverseEnterRuntime(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReverseEnterRuntime)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BeginDelayAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginDelayAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EndDelayAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndDelayAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BeginThreadAffinity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginThreadAffinity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EndThreadAffinity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndThreadAffinity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetStackGuarantee(&self, guarantee: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStackGuarantee)(windows_core::Interface::as_raw(self), guarantee).ok() }
    }
    pub unsafe fn GetStackGuarantee(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStackGuarantee)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCLRTaskManager<P0>(&self, ppmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRTaskManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCLRTaskManager)(windows_core::Interface::as_raw(self), ppmanager.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostTaskManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Threading")]
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::Threading::LPTHREAD_START_ROUTINE, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CreateTask: usize,
    pub Sleep: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SwitchToTask: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetUILocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CallNeedsHostHook: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub LeaveRuntime: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub EnterRuntime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReverseLeaveRuntime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReverseEnterRuntime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginDelayAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndDelayAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginThreadAffinity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndThreadAffinity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStackGuarantee: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStackGuarantee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCLRTaskManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Threading")]
pub trait IHostTaskManager_Impl: windows_core::IUnknownImpl {
    fn GetCurrentTask(&self) -> windows_core::Result<IHostTask>;
    fn CreateTask(&self, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const core::ffi::c_void) -> windows_core::Result<IHostTask>;
    fn Sleep(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn SwitchToTask(&self, option: u32) -> windows_core::Result<()>;
    fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn CallNeedsHostHook(&self, target: usize) -> windows_core::Result<windows_core::BOOL>;
    fn LeaveRuntime(&self, target: usize) -> windows_core::Result<()>;
    fn EnterRuntime(&self) -> windows_core::Result<()>;
    fn ReverseLeaveRuntime(&self) -> windows_core::Result<()>;
    fn ReverseEnterRuntime(&self) -> windows_core::Result<()>;
    fn BeginDelayAbort(&self) -> windows_core::Result<()>;
    fn EndDelayAbort(&self) -> windows_core::Result<()>;
    fn BeginThreadAffinity(&self) -> windows_core::Result<()>;
    fn EndThreadAffinity(&self) -> windows_core::Result<()>;
    fn SetStackGuarantee(&self, guarantee: u32) -> windows_core::Result<()>;
    fn GetStackGuarantee(&self) -> windows_core::Result<u32>;
    fn SetCLRTaskManager(&self, ppmanager: windows_core::Ref<ICLRTaskManager>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Threading")]
impl IHostTaskManager_Vtbl {
    pub const fn new<Identity: IHostTaskManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentTask<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostTaskManager_Impl::GetCurrentTask(this) {
                    Ok(ok__) => {
                        ptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTask<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const core::ffi::c_void, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostTaskManager_Impl::CreateTask(this, core::mem::transmute_copy(&dwstacksize), core::mem::transmute_copy(&pstartaddress), core::mem::transmute_copy(&pparameter)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Sleep<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::Sleep(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn SwitchToTask<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::SwitchToTask(this, core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn SetUILocale<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::SetUILocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn SetLocale<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::SetLocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn CallNeedsHostHook<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: usize, pbcallneedshosthook: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostTaskManager_Impl::CallNeedsHostHook(this, core::mem::transmute_copy(&target)) {
                    Ok(ok__) => {
                        pbcallneedshosthook.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LeaveRuntime<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::LeaveRuntime(this, core::mem::transmute_copy(&target)).into()
            }
        }
        unsafe extern "system" fn EnterRuntime<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::EnterRuntime(this).into()
            }
        }
        unsafe extern "system" fn ReverseLeaveRuntime<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::ReverseLeaveRuntime(this).into()
            }
        }
        unsafe extern "system" fn ReverseEnterRuntime<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::ReverseEnterRuntime(this).into()
            }
        }
        unsafe extern "system" fn BeginDelayAbort<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::BeginDelayAbort(this).into()
            }
        }
        unsafe extern "system" fn EndDelayAbort<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::EndDelayAbort(this).into()
            }
        }
        unsafe extern "system" fn BeginThreadAffinity<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::BeginThreadAffinity(this).into()
            }
        }
        unsafe extern "system" fn EndThreadAffinity<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::EndThreadAffinity(this).into()
            }
        }
        unsafe extern "system" fn SetStackGuarantee<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guarantee: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::SetStackGuarantee(this, core::mem::transmute_copy(&guarantee)).into()
            }
        }
        unsafe extern "system" fn GetStackGuarantee<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguarantee: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostTaskManager_Impl::GetStackGuarantee(this) {
                    Ok(ok__) => {
                        pguarantee.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCLRTaskManager<Identity: IHostTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostTaskManager_Impl::SetCLRTaskManager(this, core::mem::transmute_copy(&ppmanager)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentTask: GetCurrentTask::<Identity, OFFSET>,
            CreateTask: CreateTask::<Identity, OFFSET>,
            Sleep: Sleep::<Identity, OFFSET>,
            SwitchToTask: SwitchToTask::<Identity, OFFSET>,
            SetUILocale: SetUILocale::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            CallNeedsHostHook: CallNeedsHostHook::<Identity, OFFSET>,
            LeaveRuntime: LeaveRuntime::<Identity, OFFSET>,
            EnterRuntime: EnterRuntime::<Identity, OFFSET>,
            ReverseLeaveRuntime: ReverseLeaveRuntime::<Identity, OFFSET>,
            ReverseEnterRuntime: ReverseEnterRuntime::<Identity, OFFSET>,
            BeginDelayAbort: BeginDelayAbort::<Identity, OFFSET>,
            EndDelayAbort: EndDelayAbort::<Identity, OFFSET>,
            BeginThreadAffinity: BeginThreadAffinity::<Identity, OFFSET>,
            EndThreadAffinity: EndThreadAffinity::<Identity, OFFSET>,
            SetStackGuarantee: SetStackGuarantee::<Identity, OFFSET>,
            GetStackGuarantee: GetStackGuarantee::<Identity, OFFSET>,
            SetCLRTaskManager: SetCLRTaskManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostTaskManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl windows_core::RuntimeName for IHostTaskManager {}
windows_core::imp::define_interface!(IHostThreadpoolManager, IHostThreadpoolManager_Vtbl, 0x983d50e2_cb15_466b_80fc_845dc6e8c5fd);
windows_core::imp::interface_hierarchy!(IHostThreadpoolManager, windows_core::IUnknown);
impl IHostThreadpoolManager {
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn QueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueueUserWorkItem)(windows_core::Interface::as_raw(self), function, context, flags).ok() }
    }
    pub unsafe fn SetMaxThreads(&self, dwmaxworkerthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxThreads)(windows_core::Interface::as_raw(self), dwmaxworkerthreads).ok() }
    }
    pub unsafe fn GetMaxThreads(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAvailableThreads(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinThreads)(windows_core::Interface::as_raw(self), dwminiocompletionthreads).ok() }
    }
    pub unsafe fn GetMinThreads(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostThreadpoolManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Threading")]
    pub QueueUserWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::Threading::LPTHREAD_START_ROUTINE, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    QueueUserWorkItem: usize,
    pub SetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAvailableThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMinThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Threading")]
pub trait IHostThreadpoolManager_Impl: windows_core::IUnknownImpl {
    fn QueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn SetMaxThreads(&self, dwmaxworkerthreads: u32) -> windows_core::Result<()>;
    fn GetMaxThreads(&self) -> windows_core::Result<u32>;
    fn GetAvailableThreads(&self) -> windows_core::Result<u32>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()>;
    fn GetMinThreads(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Threading")]
impl IHostThreadpoolManager_Vtbl {
    pub const fn new<Identity: IHostThreadpoolManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueUserWorkItem<Identity: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostThreadpoolManager_Impl::QueueUserWorkItem(this, core::mem::transmute_copy(&function), core::mem::transmute_copy(&context), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn SetMaxThreads<Identity: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxworkerthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostThreadpoolManager_Impl::SetMaxThreads(this, core::mem::transmute_copy(&dwmaxworkerthreads)).into()
            }
        }
        unsafe extern "system" fn GetMaxThreads<Identity: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxworkerthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostThreadpoolManager_Impl::GetMaxThreads(this) {
                    Ok(ok__) => {
                        pdwmaxworkerthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwavailableworkerthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostThreadpoolManager_Impl::GetAvailableThreads(this) {
                    Ok(ok__) => {
                        pdwavailableworkerthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinThreads<Identity: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminiocompletionthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHostThreadpoolManager_Impl::SetMinThreads(this, core::mem::transmute_copy(&dwminiocompletionthreads)).into()
            }
        }
        unsafe extern "system" fn GetMinThreads<Identity: IHostThreadpoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHostThreadpoolManager_Impl::GetMinThreads(this) {
                    Ok(ok__) => {
                        pdwminiocompletionthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueUserWorkItem: QueueUserWorkItem::<Identity, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostThreadpoolManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl windows_core::RuntimeName for IHostThreadpoolManager {}
windows_core::imp::define_interface!(IManagedObject, IManagedObject_Vtbl, 0xc3fcc19e_a970_11d2_8b5a_00a0c9b7c9c4);
windows_core::imp::interface_hierarchy!(IManagedObject, windows_core::IUnknown);
impl IManagedObject {
    pub unsafe fn GetSerializedBuffer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSerializedBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetObjectIdentity(&self, pbstrguid: *mut windows_core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectIdentity)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrguid), appdomainid as _, pccw as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSerializedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IManagedObject_Impl: windows_core::IUnknownImpl {
    fn GetSerializedBuffer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetObjectIdentity(&self, pbstrguid: *mut windows_core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> windows_core::Result<()>;
}
impl IManagedObject_Vtbl {
    pub const fn new<Identity: IManagedObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSerializedBuffer<Identity: IManagedObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManagedObject_Impl::GetSerializedBuffer(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectIdentity<Identity: IManagedObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguid: *mut *mut core::ffi::c_void, appdomainid: *mut i32, pccw: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManagedObject_Impl::GetObjectIdentity(this, core::mem::transmute_copy(&pbstrguid), core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&pccw)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSerializedBuffer: GetSerializedBuffer::<Identity, OFFSET>,
            GetObjectIdentity: GetObjectIdentity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IManagedObject {}
windows_core::imp::define_interface!(IObjectHandle, IObjectHandle_Vtbl, 0xc460e2b4_e199_412a_8456_84dc3e4838c3);
windows_core::imp::interface_hierarchy!(IObjectHandle, windows_core::IUnknown);
impl IObjectHandle {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Unwrap(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Unwrap)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Unwrap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Unwrap: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectHandle_Impl: windows_core::IUnknownImpl {
    fn Unwrap(&self) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IObjectHandle_Vtbl {
    pub const fn new<Identity: IObjectHandle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Unwrap<Identity: IObjectHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppv: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IObjectHandle_Impl::Unwrap(this) {
                    Ok(ok__) => {
                        ppv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Unwrap: Unwrap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectHandle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IObjectHandle {}
windows_core::imp::define_interface!(ITypeName, ITypeName_Vtbl, 0xb81ff171_20f3_11d2_8dcc_00a0c9b00522);
windows_core::imp::interface_hierarchy!(ITypeName, windows_core::IUnknown);
impl ITypeName {
    pub unsafe fn GetNameCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNames(&self, count: u32, rgbsznames: *mut windows_core::BSTR) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), count, core::mem::transmute(rgbsznames), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeArgumentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeArgumentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeArguments(&self, count: u32, rgparguments: *mut Option<ITypeName>) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeArguments)(windows_core::Interface::as_raw(self), count, core::mem::transmute(rgparguments), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetModifierLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModifierLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetModifiers(&self, count: u32, rgmodifiers: *mut u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModifiers)(windows_core::Interface::as_raw(self), count, rgmodifiers as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAssemblyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAssemblyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTypeArgumentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTypeArguments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetModifierLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetAssemblyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITypeName_Impl: windows_core::IUnknownImpl {
    fn GetNameCount(&self) -> windows_core::Result<u32>;
    fn GetNames(&self, count: u32, rgbsznames: *mut windows_core::BSTR) -> windows_core::Result<u32>;
    fn GetTypeArgumentCount(&self) -> windows_core::Result<u32>;
    fn GetTypeArguments(&self, count: u32, rgparguments: windows_core::OutRef<ITypeName>) -> windows_core::Result<u32>;
    fn GetModifierLength(&self) -> windows_core::Result<u32>;
    fn GetModifiers(&self, count: u32, rgmodifiers: *mut u32) -> windows_core::Result<u32>;
    fn GetAssemblyName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITypeName_Vtbl {
    pub const fn new<Identity: ITypeName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNameCount<Identity: ITypeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeName_Impl::GetNameCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNames<Identity: ITypeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, rgbsznames: *mut *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeName_Impl::GetNames(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&rgbsznames)) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeArgumentCount<Identity: ITypeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeName_Impl::GetTypeArgumentCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeArguments<Identity: ITypeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, rgparguments: *mut *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeName_Impl::GetTypeArguments(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&rgparguments)) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetModifierLength<Identity: ITypeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeName_Impl::GetModifierLength(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetModifiers<Identity: ITypeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeName_Impl::GetModifiers(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&rgmodifiers)) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAssemblyName<Identity: ITypeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgbszassemblynames: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeName_Impl::GetAssemblyName(this) {
                    Ok(ok__) => {
                        rgbszassemblynames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNameCount: GetNameCount::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            GetTypeArgumentCount: GetTypeArgumentCount::<Identity, OFFSET>,
            GetTypeArguments: GetTypeArguments::<Identity, OFFSET>,
            GetModifierLength: GetModifierLength::<Identity, OFFSET>,
            GetModifiers: GetModifiers::<Identity, OFFSET>,
            GetAssemblyName: GetAssemblyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeName as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeName {}
windows_core::imp::define_interface!(ITypeNameBuilder, ITypeNameBuilder_Vtbl, 0xb81ff171_20f3_11d2_8dcc_00a0c9b00523);
windows_core::imp::interface_hierarchy!(ITypeNameBuilder, windows_core::IUnknown);
impl ITypeNameBuilder {
    pub unsafe fn OpenGenericArguments(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenGenericArguments)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CloseGenericArguments(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseGenericArguments)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OpenGenericArgument(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenGenericArgument)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CloseGenericArgument(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseGenericArgument)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddName<P0>(&self, szname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddName)(windows_core::Interface::as_raw(self), szname.param().abi()).ok() }
    }
    pub unsafe fn AddPointer(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPointer)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddByRef(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddByRef)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddSzArray(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddSzArray)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddArray(&self, rank: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddArray)(windows_core::Interface::as_raw(self), rank).ok() }
    }
    pub unsafe fn AddAssemblySpec<P0>(&self, szassemblyspec: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAssemblySpec)(windows_core::Interface::as_raw(self), szassemblyspec.param().abi()).ok() }
    }
    pub unsafe fn ToString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeNameBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenGenericArguments: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseGenericArguments: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenGenericArgument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseGenericArgument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddByRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSzArray: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AddAssemblySpec: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITypeNameBuilder_Impl: windows_core::IUnknownImpl {
    fn OpenGenericArguments(&self) -> windows_core::Result<()>;
    fn CloseGenericArguments(&self) -> windows_core::Result<()>;
    fn OpenGenericArgument(&self) -> windows_core::Result<()>;
    fn CloseGenericArgument(&self) -> windows_core::Result<()>;
    fn AddName(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddPointer(&self) -> windows_core::Result<()>;
    fn AddByRef(&self) -> windows_core::Result<()>;
    fn AddSzArray(&self) -> windows_core::Result<()>;
    fn AddArray(&self, rank: u32) -> windows_core::Result<()>;
    fn AddAssemblySpec(&self, szassemblyspec: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ToString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl ITypeNameBuilder_Vtbl {
    pub const fn new<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenGenericArguments<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::OpenGenericArguments(this).into()
            }
        }
        unsafe extern "system" fn CloseGenericArguments<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::CloseGenericArguments(this).into()
            }
        }
        unsafe extern "system" fn OpenGenericArgument<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::OpenGenericArgument(this).into()
            }
        }
        unsafe extern "system" fn CloseGenericArgument<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::CloseGenericArgument(this).into()
            }
        }
        unsafe extern "system" fn AddName<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::AddName(this, core::mem::transmute(&szname)).into()
            }
        }
        unsafe extern "system" fn AddPointer<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::AddPointer(this).into()
            }
        }
        unsafe extern "system" fn AddByRef<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::AddByRef(this).into()
            }
        }
        unsafe extern "system" fn AddSzArray<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::AddSzArray(this).into()
            }
        }
        unsafe extern "system" fn AddArray<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rank: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::AddArray(this, core::mem::transmute_copy(&rank)).into()
            }
        }
        unsafe extern "system" fn AddAssemblySpec<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szassemblyspec: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::AddAssemblySpec(this, core::mem::transmute(&szassemblyspec)).into()
            }
        }
        unsafe extern "system" fn ToString<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstringrepresentation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeNameBuilder_Impl::ToString(this) {
                    Ok(ok__) => {
                        pszstringrepresentation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: ITypeNameBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITypeNameBuilder_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenGenericArguments: OpenGenericArguments::<Identity, OFFSET>,
            CloseGenericArguments: CloseGenericArguments::<Identity, OFFSET>,
            OpenGenericArgument: OpenGenericArgument::<Identity, OFFSET>,
            CloseGenericArgument: CloseGenericArgument::<Identity, OFFSET>,
            AddName: AddName::<Identity, OFFSET>,
            AddPointer: AddPointer::<Identity, OFFSET>,
            AddByRef: AddByRef::<Identity, OFFSET>,
            AddSzArray: AddSzArray::<Identity, OFFSET>,
            AddArray: AddArray::<Identity, OFFSET>,
            AddAssemblySpec: AddAssemblySpec::<Identity, OFFSET>,
            ToString: ToString::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeNameBuilder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeNameBuilder {}
windows_core::imp::define_interface!(ITypeNameFactory, ITypeNameFactory_Vtbl, 0xb81ff171_20f3_11d2_8dcc_00a0c9b00521);
windows_core::imp::interface_hierarchy!(ITypeNameFactory, windows_core::IUnknown);
impl ITypeNameFactory {
    pub unsafe fn ParseTypeName<P0>(&self, szname: P0, perror: *mut u32) -> windows_core::Result<ITypeName>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseTypeName)(windows_core::Interface::as_raw(self), szname.param().abi(), perror as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTypeNameBuilder(&self) -> windows_core::Result<ITypeNameBuilder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeNameBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeNameFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ParseTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypeNameBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITypeNameFactory_Impl: windows_core::IUnknownImpl {
    fn ParseTypeName(&self, szname: &windows_core::PCWSTR, perror: *mut u32) -> windows_core::Result<ITypeName>;
    fn GetTypeNameBuilder(&self) -> windows_core::Result<ITypeNameBuilder>;
}
impl ITypeNameFactory_Vtbl {
    pub const fn new<Identity: ITypeNameFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseTypeName<Identity: ITypeNameFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, perror: *mut u32, pptypename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeNameFactory_Impl::ParseTypeName(this, core::mem::transmute(&szname), core::mem::transmute_copy(&perror)) {
                    Ok(ok__) => {
                        pptypename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeNameBuilder<Identity: ITypeNameFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITypeNameFactory_Impl::GetTypeNameBuilder(this) {
                    Ok(ok__) => {
                        pptypebuilder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseTypeName: ParseTypeName::<Identity, OFFSET>,
            GetTypeNameBuilder: GetTypeNameBuilder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeNameFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITypeNameFactory {}
pub const InvalidBucketParamIndex: BucketParameterIndex = BucketParameterIndex(9i32);
pub const LIBID_mscoree: windows_core::GUID = windows_core::GUID::from_u128(0x5477469e_83b1_11d2_8b49_00a0c9b7c9c4);
pub const MALLOC_EXECUTABLE: MALLOC_TYPE = MALLOC_TYPE(2i32);
pub const MALLOC_THREADSAFE: MALLOC_TYPE = MALLOC_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MALLOC_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MDAInfo {
    pub lpMDACaption: windows_core::PCWSTR,
    pub lpMDAMessage: windows_core::PCWSTR,
    pub lpStackTrace: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct METAHOST_CONFIG_FLAGS(pub i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_FALSE: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(2i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_MASK: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(3i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_TRUE: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(1i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_UNSET: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(0i32);
pub const METAHOST_POLICY_APPLY_UPGRADE_POLICY: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(8i32);
pub const METAHOST_POLICY_EMULATE_EXE_LAUNCH: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(16i32);
pub const METAHOST_POLICY_ENSURE_SKU_SUPPORTED: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(128i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct METAHOST_POLICY_FLAGS(pub i32);
pub const METAHOST_POLICY_HIGHCOMPAT: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(0i32);
pub const METAHOST_POLICY_IGNORE_ERROR_MODE: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(4096i32);
pub const METAHOST_POLICY_SHOW_ERROR_DIALOG: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(32i32);
pub const METAHOST_POLICY_USE_PROCESS_IMAGE_PATH: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(64i32);
pub const MaxClrEvent: EClrEvent = EClrEvent(4i32);
pub const MaxClrFailure: EClrFailure = EClrFailure(7i32);
pub const MaxClrOperation: EClrOperation = EClrOperation(7i32);
pub const MaxPolicyAction: EPolicyAction = EPolicyAction(10i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ModuleBindInfo {
    pub dwAppDomainId: u32,
    pub lpAssemblyIdentity: windows_core::PCWSTR,
    pub lpModuleName: windows_core::PCWSTR,
}
pub const OPR_AppDomainRudeUnload: EClrOperation = EClrOperation(4i32);
pub const OPR_AppDomainUnload: EClrOperation = EClrOperation(3i32);
pub const OPR_FinalizerRun: EClrOperation = EClrOperation(6i32);
pub const OPR_ProcessExit: EClrOperation = EClrOperation(5i32);
pub const OPR_ThreadAbort: EClrOperation = EClrOperation(0i32);
pub const OPR_ThreadRudeAbortInCriticalRegion: EClrOperation = EClrOperation(2i32);
pub const OPR_ThreadRudeAbortInNonCriticalRegion: EClrOperation = EClrOperation(1i32);
pub type PTLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(__midl____midl_itf_mscoree_0000_00040005: *mut core::ffi::c_void)>;
pub const Parameter1: BucketParameterIndex = BucketParameterIndex(0i32);
pub const Parameter2: BucketParameterIndex = BucketParameterIndex(1i32);
pub const Parameter3: BucketParameterIndex = BucketParameterIndex(2i32);
pub const Parameter4: BucketParameterIndex = BucketParameterIndex(3i32);
pub const Parameter5: BucketParameterIndex = BucketParameterIndex(4i32);
pub const Parameter6: BucketParameterIndex = BucketParameterIndex(5i32);
pub const Parameter7: BucketParameterIndex = BucketParameterIndex(6i32);
pub const Parameter8: BucketParameterIndex = BucketParameterIndex(7i32);
pub const Parameter9: BucketParameterIndex = BucketParameterIndex(8i32);
pub const RUNTIME_INFO_DONT_RETURN_DIRECTORY: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(16i32);
pub const RUNTIME_INFO_DONT_RETURN_VERSION: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(32i32);
pub const RUNTIME_INFO_DONT_SHOW_ERROR_DIALOG: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(64i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RUNTIME_INFO_FLAGS(pub i32);
pub const RUNTIME_INFO_IGNORE_ERROR_MODE: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(4096i32);
pub const RUNTIME_INFO_REQUEST_AMD64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(4i32);
pub const RUNTIME_INFO_REQUEST_ARM64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(8192i32);
pub const RUNTIME_INFO_REQUEST_IA64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(2i32);
pub const RUNTIME_INFO_REQUEST_X86: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(8i32);
pub const RUNTIME_INFO_UPGRADE_VERSION: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(1i32);
pub type RuntimeLoadedCallbackFnPtr = Option<unsafe extern "system" fn(pruntimeinfo: windows_core::Ref<ICLRRuntimeInfo>, pfncallbackthreadset: CallbackThreadSetFnPtr, pfncallbackthreadunset: CallbackThreadUnsetFnPtr)>;
pub const SO_ClrEngine: StackOverflowType = StackOverflowType(1i32);
pub const SO_Managed: StackOverflowType = StackOverflowType(0i32);
pub const SO_Other: StackOverflowType = StackOverflowType(2i32);
pub const STARTUP_ALWAYSFLOW_IMPERSONATION: STARTUP_FLAGS = STARTUP_FLAGS(262144i32);
pub const STARTUP_ARM: STARTUP_FLAGS = STARTUP_FLAGS(4194304i32);
pub const STARTUP_CONCURRENT_GC: STARTUP_FLAGS = STARTUP_FLAGS(1i32);
pub const STARTUP_DISABLE_COMMITTHREADSTACK: STARTUP_FLAGS = STARTUP_FLAGS(131072i32);
pub const STARTUP_ETW: STARTUP_FLAGS = STARTUP_FLAGS(1048576i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STARTUP_FLAGS(pub i32);
pub const STARTUP_HOARD_GC_VM: STARTUP_FLAGS = STARTUP_FLAGS(8192i32);
pub const STARTUP_LEGACY_IMPERSONATION: STARTUP_FLAGS = STARTUP_FLAGS(65536i32);
pub const STARTUP_LOADER_OPTIMIZATION_MASK: STARTUP_FLAGS = STARTUP_FLAGS(6i32);
pub const STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN: STARTUP_FLAGS = STARTUP_FLAGS(4i32);
pub const STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN_HOST: STARTUP_FLAGS = STARTUP_FLAGS(6i32);
pub const STARTUP_LOADER_OPTIMIZATION_SINGLE_DOMAIN: STARTUP_FLAGS = STARTUP_FLAGS(2i32);
pub const STARTUP_LOADER_SAFEMODE: STARTUP_FLAGS = STARTUP_FLAGS(16i32);
pub const STARTUP_LOADER_SETPREFERENCE: STARTUP_FLAGS = STARTUP_FLAGS(256i32);
pub const STARTUP_SERVER_GC: STARTUP_FLAGS = STARTUP_FLAGS(4096i32);
pub const STARTUP_SINGLE_VERSION_HOSTING_INTERFACE: STARTUP_FLAGS = STARTUP_FLAGS(16384i32);
pub const STARTUP_TRIM_GC_COMMIT: STARTUP_FLAGS = STARTUP_FLAGS(524288i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StackOverflowInfo {
    pub soType: StackOverflowType,
    pub pExceptionInfo: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl Default for StackOverflowInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StackOverflowType(pub i32);
pub const TT_ADUNLOAD: ETaskType = ETaskType(128i32);
pub const TT_DEBUGGERHELPER: ETaskType = ETaskType(1i32);
pub const TT_FINALIZER: ETaskType = ETaskType(4i32);
pub const TT_GC: ETaskType = ETaskType(2i32);
pub const TT_THREADPOOL_GATE: ETaskType = ETaskType(16i32);
pub const TT_THREADPOOL_IOCOMPLETION: ETaskType = ETaskType(64i32);
pub const TT_THREADPOOL_TIMER: ETaskType = ETaskType(8i32);
pub const TT_THREADPOOL_WAIT: ETaskType = ETaskType(512i32);
pub const TT_THREADPOOL_WORKER: ETaskType = ETaskType(32i32);
pub const TT_UNKNOWN: ETaskType = ETaskType(-2147483648i32);
pub const TT_USER: ETaskType = ETaskType(256i32);
pub const TypeNameFactory: windows_core::GUID = windows_core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b00525);
pub const WAIT_ALERTABLE: WAIT_OPTION = WAIT_OPTION(2i32);
pub const WAIT_MSGPUMP: WAIT_OPTION = WAIT_OPTION(1i32);
pub const WAIT_NOTINDEADLOCK: WAIT_OPTION = WAIT_OPTION(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WAIT_OPTION(pub i32);
pub const eAbortThread: EPolicyAction = EPolicyAction(2i32);
pub const eAll: EApiCategories = EApiCategories(511i32);
pub const eAppDomainCritical: EMemoryCriticalLevel = EMemoryCriticalLevel(1i32);
pub const eCurrentContext: EContextType = EContextType(0i32);
pub const eDisableRuntime: EPolicyAction = EPolicyAction(9i32);
pub const eExitProcess: EPolicyAction = EPolicyAction(6i32);
pub const eExternalProcessMgmt: EApiCategories = EApiCategories(4i32);
pub const eExternalThreading: EApiCategories = EApiCategories(16i32);
pub const eFastExitProcess: EPolicyAction = EPolicyAction(7i32);
pub const eHostDeterminedPolicy: EClrUnhandledException = EClrUnhandledException(1i32);
pub const eInitializeNewDomainFlags_NoSecurityChanges: EInitializeNewDomainFlags = EInitializeNewDomainFlags(2i32);
pub const eInitializeNewDomainFlags_None: EInitializeNewDomainFlags = EInitializeNewDomainFlags(0i32);
pub const eMayLeakOnAbort: EApiCategories = EApiCategories(256i32);
pub const eMemoryAvailableHigh: EMemoryAvailable = EMemoryAvailable(3i32);
pub const eMemoryAvailableLow: EMemoryAvailable = EMemoryAvailable(1i32);
pub const eMemoryAvailableNeutral: EMemoryAvailable = EMemoryAvailable(2i32);
pub const eNoAction: EPolicyAction = EPolicyAction(0i32);
pub const eNoChecks: EApiCategories = EApiCategories(0i32);
pub const ePolicyLevelAdmin: EBindPolicyLevels = EBindPolicyLevels(32i32);
pub const ePolicyLevelApp: EBindPolicyLevels = EBindPolicyLevels(4i32);
pub const ePolicyLevelHost: EBindPolicyLevels = EBindPolicyLevels(16i32);
pub const ePolicyLevelNone: EBindPolicyLevels = EBindPolicyLevels(0i32);
pub const ePolicyLevelPublisher: EBindPolicyLevels = EBindPolicyLevels(8i32);
pub const ePolicyLevelRetargetable: EBindPolicyLevels = EBindPolicyLevels(1i32);
pub const ePolicyPortability: EBindPolicyLevels = EBindPolicyLevels(64i32);
pub const ePolicyUnifiedToCLR: EBindPolicyLevels = EBindPolicyLevels(2i32);
pub const eProcessCritical: EMemoryCriticalLevel = EMemoryCriticalLevel(2i32);
pub const eRestrictedContext: EContextType = EContextType(1i32);
pub const eRudeAbortThread: EPolicyAction = EPolicyAction(3i32);
pub const eRudeExitProcess: EPolicyAction = EPolicyAction(8i32);
pub const eRudeUnloadAppDomain: EPolicyAction = EPolicyAction(5i32);
pub const eRuntimeDeterminedPolicy: EClrUnhandledException = EClrUnhandledException(0i32);
pub const eSecurityInfrastructure: EApiCategories = EApiCategories(64i32);
pub const eSelfAffectingProcessMgmt: EApiCategories = EApiCategories(8i32);
pub const eSelfAffectingThreading: EApiCategories = EApiCategories(32i32);
pub const eSharedState: EApiCategories = EApiCategories(2i32);
pub const eSymbolReadingAlways: ESymbolReadingPolicy = ESymbolReadingPolicy(1i32);
pub const eSymbolReadingFullTrustOnly: ESymbolReadingPolicy = ESymbolReadingPolicy(2i32);
pub const eSymbolReadingNever: ESymbolReadingPolicy = ESymbolReadingPolicy(0i32);
pub const eSynchronization: EApiCategories = EApiCategories(1i32);
pub const eTaskCritical: EMemoryCriticalLevel = EMemoryCriticalLevel(0i32);
pub const eThrowException: EPolicyAction = EPolicyAction(1i32);
pub const eUI: EApiCategories = EApiCategories(128i32);
pub const eUnloadAppDomain: EPolicyAction = EPolicyAction(4i32);
