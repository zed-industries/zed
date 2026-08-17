#[inline]
pub unsafe fn CLRCreateInstance<T>(clsid: *const windows_core::GUID) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("mscoree.dll" "system" fn CLRCreateInstance(clsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppinterface : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CLRCreateInstance(clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CallFunctionShim<P0, P1, P2>(szdllname: P0, szfunctionname: P1, lpvargument1: *mut core::ffi::c_void, lpvargument2: *mut core::ffi::c_void, szversion: P2, pvreserved: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn CallFunctionShim(szdllname : windows_core::PCWSTR, szfunctionname : windows_core::PCSTR, lpvargument1 : *mut core::ffi::c_void, lpvargument2 : *mut core::ffi::c_void, szversion : windows_core::PCWSTR, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    CallFunctionShim(szdllname.param().abi(), szfunctionname.param().abi(), lpvargument1, lpvargument2, szversion.param().abi(), pvreserved).ok()
}
#[inline]
pub unsafe fn ClrCreateManagedInstance<P0>(ptypename: P0, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn ClrCreateManagedInstance(ptypename : windows_core::PCWSTR, riid : *const windows_core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ClrCreateManagedInstance(ptypename.param().abi(), riid, ppobject).ok()
}
#[inline]
pub unsafe fn CorBindToCurrentRuntime<P0>(pwszfilename: P0, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn CorBindToCurrentRuntime(pwszfilename : windows_core::PCWSTR, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CorBindToCurrentRuntime(pwszfilename.param().abi(), rclsid, riid, ppv).ok()
}
#[inline]
pub unsafe fn CorBindToRuntime<P0, P1>(pwszversion: P0, pwszbuildflavor: P1, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn CorBindToRuntime(pwszversion : windows_core::PCWSTR, pwszbuildflavor : windows_core::PCWSTR, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CorBindToRuntime(pwszversion.param().abi(), pwszbuildflavor.param().abi(), rclsid, riid, ppv).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CorBindToRuntimeByCfg<P0>(pcfgstream: P0, reserved: u32, startupflags: u32, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IStream>,
{
    windows_targets::link!("mscoree.dll" "system" fn CorBindToRuntimeByCfg(pcfgstream : * mut core::ffi::c_void, reserved : u32, startupflags : u32, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CorBindToRuntimeByCfg(pcfgstream.param().abi(), reserved, startupflags, rclsid, riid, ppv).ok()
}
#[inline]
pub unsafe fn CorBindToRuntimeEx<P0, P1>(pwszversion: P0, pwszbuildflavor: P1, startupflags: u32, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn CorBindToRuntimeEx(pwszversion : windows_core::PCWSTR, pwszbuildflavor : windows_core::PCWSTR, startupflags : u32, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CorBindToRuntimeEx(pwszversion.param().abi(), pwszbuildflavor.param().abi(), startupflags, rclsid, riid, ppv).ok()
}
#[inline]
pub unsafe fn CorBindToRuntimeHost<P0, P1, P2>(pwszversion: P0, pwszbuildflavor: P1, pwszhostconfigfile: P2, preserved: *mut core::ffi::c_void, startupflags: u32, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn CorBindToRuntimeHost(pwszversion : windows_core::PCWSTR, pwszbuildflavor : windows_core::PCWSTR, pwszhostconfigfile : windows_core::PCWSTR, preserved : *mut core::ffi::c_void, startupflags : u32, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CorBindToRuntimeHost(pwszversion.param().abi(), pwszbuildflavor.param().abi(), pwszhostconfigfile.param().abi(), preserved, startupflags, rclsid, riid, ppv).ok()
}
#[inline]
pub unsafe fn CorExitProcess(exitcode: i32) {
    windows_targets::link!("mscoree.dll" "system" fn CorExitProcess(exitcode : i32));
    CorExitProcess(exitcode)
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn CorLaunchApplication<P0>(dwclickoncehost: HOST_TYPE, pwzappfullname: P0, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR, lpprocessinformation: *mut super::Threading::PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn CorLaunchApplication(dwclickoncehost : HOST_TYPE, pwzappfullname : windows_core::PCWSTR, dwmanifestpaths : u32, ppwzmanifestpaths : *const windows_core::PCWSTR, dwactivationdata : u32, ppwzactivationdata : *const windows_core::PCWSTR, lpprocessinformation : *mut super::Threading:: PROCESS_INFORMATION) -> windows_core::HRESULT);
    CorLaunchApplication(dwclickoncehost, pwzappfullname.param().abi(), dwmanifestpaths, ppwzmanifestpaths, dwactivationdata, ppwzactivationdata, lpprocessinformation).ok()
}
#[inline]
pub unsafe fn CorMarkThreadInThreadPool() {
    windows_targets::link!("mscoree.dll" "system" fn CorMarkThreadInThreadPool());
    CorMarkThreadInThreadPool()
}
#[inline]
pub unsafe fn CreateDebuggingInterfaceFromVersion<P0>(idebuggerversion: i32, szdebuggeeversion: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn CreateDebuggingInterfaceFromVersion(idebuggerversion : i32, szdebuggeeversion : windows_core::PCWSTR, ppcordb : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateDebuggingInterfaceFromVersion(idebuggerversion, szdebuggeeversion.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn GetCLRIdentityManager(riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
    windows_targets::link!("mscoree.dll" "system" fn GetCLRIdentityManager(riid : *const windows_core::GUID, ppmanager : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetCLRIdentityManager(riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn GetCORRequiredVersion(pbuffer: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mscoree.dll" "system" fn GetCORRequiredVersion(pbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    GetCORRequiredVersion(core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), dwlength).ok()
}
#[inline]
pub unsafe fn GetCORSystemDirectory(pbuffer: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mscoree.dll" "system" fn GetCORSystemDirectory(pbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    GetCORSystemDirectory(core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), dwlength).ok()
}
#[inline]
pub unsafe fn GetCORVersion(pbbuffer: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mscoree.dll" "system" fn GetCORVersion(pbbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    GetCORVersion(core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len().try_into().unwrap(), dwlength).ok()
}
#[inline]
pub unsafe fn GetFileVersion<P0>(szfilename: P0, szbuffer: Option<&mut [u16]>, dwlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn GetFileVersion(szfilename : windows_core::PCWSTR, szbuffer : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    GetFileVersion(szfilename.param().abi(), core::mem::transmute(szbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwlength).ok()
}
#[inline]
pub unsafe fn GetRealProcAddress<P0>(pwszprocname: P0, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn GetRealProcAddress(pwszprocname : windows_core::PCSTR, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    GetRealProcAddress(pwszprocname.param().abi(), ppv).ok()
}
#[inline]
pub unsafe fn GetRequestedRuntimeInfo<P0, P1, P2>(pexe: P0, pwszversion: P1, pconfigurationfile: P2, startupflags: u32, runtimeinfoflags: u32, pdirectory: Option<&mut [u16]>, dwdirectorylength: Option<*mut u32>, pversion: Option<&mut [u16]>, dwlength: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn GetRequestedRuntimeInfo(pexe : windows_core::PCWSTR, pwszversion : windows_core::PCWSTR, pconfigurationfile : windows_core::PCWSTR, startupflags : u32, runtimeinfoflags : u32, pdirectory : windows_core::PWSTR, dwdirectory : u32, dwdirectorylength : *mut u32, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    GetRequestedRuntimeInfo(
        pexe.param().abi(),
        pwszversion.param().abi(),
        pconfigurationfile.param().abi(),
        startupflags,
        runtimeinfoflags,
        core::mem::transmute(pdirectory.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        pdirectory.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(dwdirectorylength.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pversion.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        pversion.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(dwlength.unwrap_or(std::ptr::null_mut())),
    )
    .ok()
}
#[inline]
pub unsafe fn GetRequestedRuntimeVersion<P0>(pexe: P0, pversion: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn GetRequestedRuntimeVersion(pexe : windows_core::PCWSTR, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    GetRequestedRuntimeVersion(pexe.param().abi(), core::mem::transmute(pversion.as_ptr()), pversion.len().try_into().unwrap(), dwlength).ok()
}
#[inline]
pub unsafe fn GetRequestedRuntimeVersionForCLSID(rclsid: *const windows_core::GUID, pversion: Option<&mut [u16]>, dwlength: Option<*mut u32>, dwresolutionflags: CLSID_RESOLUTION_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("mscoree.dll" "system" fn GetRequestedRuntimeVersionForCLSID(rclsid : *const windows_core::GUID, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32, dwresolutionflags : CLSID_RESOLUTION_FLAGS) -> windows_core::HRESULT);
    GetRequestedRuntimeVersionForCLSID(rclsid, core::mem::transmute(pversion.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pversion.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(dwlength.unwrap_or(std::ptr::null_mut())), dwresolutionflags).ok()
}
#[inline]
pub unsafe fn GetVersionFromProcess<P0>(hprocess: P0, pversion: &mut [u16], dwlength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mscoree.dll" "system" fn GetVersionFromProcess(hprocess : super::super::Foundation:: HANDLE, pversion : windows_core::PWSTR, cchbuffer : u32, dwlength : *mut u32) -> windows_core::HRESULT);
    GetVersionFromProcess(hprocess.param().abi(), core::mem::transmute(pversion.as_ptr()), pversion.len().try_into().unwrap(), dwlength).ok()
}
#[inline]
pub unsafe fn LoadLibraryShim<P0, P1>(szdllname: P0, szversion: P1, pvreserved: *mut core::ffi::c_void, phmoddll: *mut super::super::Foundation::HMODULE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn LoadLibraryShim(szdllname : windows_core::PCWSTR, szversion : windows_core::PCWSTR, pvreserved : *mut core::ffi::c_void, phmoddll : *mut super::super::Foundation:: HMODULE) -> windows_core::HRESULT);
    LoadLibraryShim(szdllname.param().abi(), szversion.param().abi(), pvreserved, phmoddll).ok()
}
#[inline]
pub unsafe fn LoadStringRC(iresouceid: u32, szbuffer: &mut [u16], bquiet: i32) -> windows_core::Result<()> {
    windows_targets::link!("mscoree.dll" "system" fn LoadStringRC(iresouceid : u32, szbuffer : windows_core::PWSTR, imax : i32, bquiet : i32) -> windows_core::HRESULT);
    LoadStringRC(iresouceid, core::mem::transmute(szbuffer.as_ptr()), szbuffer.len().try_into().unwrap(), bquiet).ok()
}
#[inline]
pub unsafe fn LoadStringRCEx(lcid: u32, iresouceid: u32, szbuffer: &mut [u16], bquiet: i32, pcwchused: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("mscoree.dll" "system" fn LoadStringRCEx(lcid : u32, iresouceid : u32, szbuffer : windows_core::PWSTR, imax : i32, bquiet : i32, pcwchused : *mut i32) -> windows_core::HRESULT);
    LoadStringRCEx(lcid, iresouceid, core::mem::transmute(szbuffer.as_ptr()), szbuffer.len().try_into().unwrap(), bquiet, pcwchused).ok()
}
#[inline]
pub unsafe fn LockClrVersion(hostcallback: FLockClrVersionCallback, pbeginhostsetup: *mut FLockClrVersionCallback, pendhostsetup: *mut FLockClrVersionCallback) -> windows_core::Result<()> {
    windows_targets::link!("mscoree.dll" "system" fn LockClrVersion(hostcallback : FLockClrVersionCallback, pbeginhostsetup : *mut FLockClrVersionCallback, pendhostsetup : *mut FLockClrVersionCallback) -> windows_core::HRESULT);
    LockClrVersion(hostcallback, pbeginhostsetup, pendhostsetup).ok()
}
#[inline]
pub unsafe fn RunDll32ShimW<P0, P1, P2>(hwnd: P0, hinst: P1, lpszcmdline: P2, ncmdshow: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscoree.dll" "system" fn RunDll32ShimW(hwnd : super::super::Foundation:: HWND, hinst : super::super::Foundation:: HINSTANCE, lpszcmdline : windows_core::PCWSTR, ncmdshow : i32) -> windows_core::HRESULT);
    RunDll32ShimW(hwnd.param().abi(), hinst.param().abi(), lpszcmdline.param().abi(), ncmdshow).ok()
}
windows_core::imp::define_interface!(IActionOnCLREvent, IActionOnCLREvent_Vtbl, 0x607be24b_d91b_4e28_a242_61871ce56e35);
impl core::ops::Deref for IActionOnCLREvent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActionOnCLREvent, windows_core::IUnknown);
impl IActionOnCLREvent {
    pub unsafe fn OnEvent(&self, event: EClrEvent, data: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), event, data).ok()
    }
}
#[repr(C)]
pub struct IActionOnCLREvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, EClrEvent, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApartmentCallback, IApartmentCallback_Vtbl, 0x178e5337_1528_4591_b1c9_1c6e484686d8);
impl core::ops::Deref for IApartmentCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IApartmentCallback, windows_core::IUnknown);
impl IApartmentCallback {
    pub unsafe fn DoCallback(&self, pfunc: usize, pdata: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoCallback)(windows_core::Interface::as_raw(self), pfunc, pdata).ok()
    }
}
#[repr(C)]
pub struct IApartmentCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoCallback: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppDomainBinding, IAppDomainBinding_Vtbl, 0x5c2b07a7_1e98_11d3_872f_00c04f79ed0d);
impl core::ops::Deref for IAppDomainBinding {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppDomainBinding, windows_core::IUnknown);
impl IAppDomainBinding {
    pub unsafe fn OnAppDomain<P0>(&self, pappdomain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnAppDomain)(windows_core::Interface::as_raw(self), pappdomain.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppDomainBinding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRAppDomainResourceMonitor, ICLRAppDomainResourceMonitor_Vtbl, 0xc62de18c_2e23_4aea_8423_b40c1fc59eae);
impl core::ops::Deref for ICLRAppDomainResourceMonitor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRAppDomainResourceMonitor, windows_core::IUnknown);
impl ICLRAppDomainResourceMonitor {
    pub unsafe fn GetCurrentAllocated(&self, dwappdomainid: u32, pbytesallocated: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentAllocated)(windows_core::Interface::as_raw(self), dwappdomainid, pbytesallocated).ok()
    }
    pub unsafe fn GetCurrentSurvived(&self, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentSurvived)(windows_core::Interface::as_raw(self), dwappdomainid, pappdomainbytessurvived, ptotalbytessurvived).ok()
    }
    pub unsafe fn GetCurrentCpuTime(&self, dwappdomainid: u32, pmilliseconds: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentCpuTime)(windows_core::Interface::as_raw(self), dwappdomainid, pmilliseconds).ok()
    }
}
#[repr(C)]
pub struct ICLRAppDomainResourceMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub GetCurrentSurvived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetCurrentCpuTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRAssemblyIdentityManager, ICLRAssemblyIdentityManager_Vtbl, 0x15f0a9da_3ff6_4393_9da9_fdfd284e6972);
impl core::ops::Deref for ICLRAssemblyIdentityManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRAssemblyIdentityManager, windows_core::IUnknown);
impl ICLRAssemblyIdentityManager {
    pub unsafe fn GetCLRAssemblyReferenceList(&self, ppwzassemblyreferences: *const windows_core::PCWSTR, dwnumofreferences: u32) -> windows_core::Result<ICLRAssemblyReferenceList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCLRAssemblyReferenceList)(windows_core::Interface::as_raw(self), ppwzassemblyreferences, dwnumofreferences, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBindingIdentityFromFile<P0>(&self, pwzfilepath: P0, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetBindingIdentityFromFile)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), dwflags, core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBindingIdentityFromStream<P0>(&self, pstream: P0, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).GetBindingIdentityFromStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwflags, core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
    pub unsafe fn GetReferencedAssembliesFromFile<P0, P1>(&self, pwzfilepath: P0, dwflags: u32, pexcludeassemblieslist: P1) -> windows_core::Result<ICLRReferenceAssemblyEnum>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICLRAssemblyReferenceList>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReferencedAssembliesFromFile)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), dwflags, pexcludeassemblieslist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReferencedAssembliesFromStream<P0, P1>(&self, pstream: P0, dwflags: u32, pexcludeassemblieslist: P1) -> windows_core::Result<ICLRReferenceAssemblyEnum>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<ICLRAssemblyReferenceList>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReferencedAssembliesFromStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwflags, pexcludeassemblieslist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetProbingAssembliesFromReference<P0>(&self, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: P0) -> windows_core::Result<ICLRProbingAssemblyEnum>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProbingAssembliesFromReference)(windows_core::Interface::as_raw(self), dwmachinetype, dwflags, pwzreferenceidentity.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsStronglyNamed<P0>(&self, pwzassemblyidentity: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsStronglyNamed)(windows_core::Interface::as_raw(self), pwzassemblyidentity.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
    pub IsStronglyNamed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRAssemblyReferenceList, ICLRAssemblyReferenceList_Vtbl, 0x1b2c9750_2e66_4bda_8b44_0a642c5cd733);
impl core::ops::Deref for ICLRAssemblyReferenceList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRAssemblyReferenceList, windows_core::IUnknown);
impl ICLRAssemblyReferenceList {
    pub unsafe fn IsStringAssemblyReferenceInList<P0>(&self, pwzassemblyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsStringAssemblyReferenceInList)(windows_core::Interface::as_raw(self), pwzassemblyname.param().abi()).ok()
    }
    pub unsafe fn IsAssemblyReferenceInList<P0>(&self, pname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).IsAssemblyReferenceInList)(windows_core::Interface::as_raw(self), pname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICLRAssemblyReferenceList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsStringAssemblyReferenceInList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsAssemblyReferenceInList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRControl, ICLRControl_Vtbl, 0x9065597e_d1a1_4fb2_b6ba_7e1fce230f61);
impl core::ops::Deref for ICLRControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRControl, windows_core::IUnknown);
impl ICLRControl {
    pub unsafe fn GetCLRManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCLRManager)(windows_core::Interface::as_raw(self), riid, ppobject).ok()
    }
    pub unsafe fn SetAppDomainManagerType<P0, P1>(&self, pwzappdomainmanagerassembly: P0, pwzappdomainmanagertype: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAppDomainManagerType)(windows_core::Interface::as_raw(self), pwzappdomainmanagerassembly.param().abi(), pwzappdomainmanagertype.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICLRControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCLRManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppDomainManagerType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRDebugManager, ICLRDebugManager_Vtbl, 0x00dcaec6_2ac0_43a9_acf9_1e36c139b10d);
impl core::ops::Deref for ICLRDebugManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRDebugManager, windows_core::IUnknown);
impl ICLRDebugManager {
    pub unsafe fn BeginConnection<P0>(&self, dwconnectionid: u32, szconnectionname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).BeginConnection)(windows_core::Interface::as_raw(self), dwconnectionid, szconnectionname.param().abi()).ok()
    }
    pub unsafe fn SetConnectionTasks(&self, id: u32, ppclrtask: &[Option<ICLRTask>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectionTasks)(windows_core::Interface::as_raw(self), id, ppclrtask.len().try_into().unwrap(), core::mem::transmute(ppclrtask.as_ptr())).ok()
    }
    pub unsafe fn EndConnection(&self, dwconnectionid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndConnection)(windows_core::Interface::as_raw(self), dwconnectionid).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn SetDacl(&self, pacl: *const super::super::Security::ACL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDacl)(windows_core::Interface::as_raw(self), pacl).ok()
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetDacl(&self) -> windows_core::Result<*mut super::super::Security::ACL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDacl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDebuggerAttached(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDebuggerAttached)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSymbolReadingPolicy(&self, policy: ESymbolReadingPolicy) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSymbolReadingPolicy)(windows_core::Interface::as_raw(self), policy).ok()
    }
}
#[repr(C)]
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
    pub IsDebuggerAttached: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetSymbolReadingPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, ESymbolReadingPolicy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRDebugging, ICLRDebugging_Vtbl, 0xd28f3c5a_9634_4206_a509_477552eefb10);
impl core::ops::Deref for ICLRDebugging {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRDebugging, windows_core::IUnknown);
impl ICLRDebugging {
    pub unsafe fn OpenVirtualProcess<P0, P1>(&self, modulebaseaddress: u64, pdatatarget: P0, plibraryprovider: P1, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const windows_core::GUID, ppprocess: *mut Option<windows_core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<ICLRDebuggingLibraryProvider>,
    {
        (windows_core::Interface::vtable(self).OpenVirtualProcess)(windows_core::Interface::as_raw(self), modulebaseaddress, pdatatarget.param().abi(), plibraryprovider.param().abi(), pmaxdebuggersupportedversion, riidprocess, core::mem::transmute(ppprocess), pversion, pdwflags).ok()
    }
    pub unsafe fn CanUnloadNow<P0>(&self, hmodule: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HMODULE>,
    {
        (windows_core::Interface::vtable(self).CanUnloadNow)(windows_core::Interface::as_raw(self), hmodule.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICLRDebugging_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenVirtualProcess: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, *mut core::ffi::c_void, *const CLR_DEBUGGING_VERSION, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut CLR_DEBUGGING_VERSION, *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::HRESULT,
    pub CanUnloadNow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HMODULE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRDebuggingLibraryProvider, ICLRDebuggingLibraryProvider_Vtbl, 0x3151c08d_4d09_4f9b_8838_2880bf18fe51);
impl core::ops::Deref for ICLRDebuggingLibraryProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRDebuggingLibraryProvider, windows_core::IUnknown);
impl ICLRDebuggingLibraryProvider {
    pub unsafe fn ProvideLibrary<P0>(&self, pwszfilename: P0, dwtimestamp: u32, dwsizeofimage: u32) -> windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProvideLibrary)(windows_core::Interface::as_raw(self), pwszfilename.param().abi(), dwtimestamp, dwsizeofimage, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICLRDebuggingLibraryProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProvideLibrary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRDomainManager, ICLRDomainManager_Vtbl, 0x270d00a2_8e15_4d0b_adeb_37bc3e47df77);
impl core::ops::Deref for ICLRDomainManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRDomainManager, windows_core::IUnknown);
impl ICLRDomainManager {
    pub unsafe fn SetAppDomainManagerType<P0, P1>(&self, wszappdomainmanagerassembly: P0, wszappdomainmanagertype: P1, dwinitializedomainflags: EInitializeNewDomainFlags) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAppDomainManagerType)(windows_core::Interface::as_raw(self), wszappdomainmanagerassembly.param().abi(), wszappdomainmanagertype.param().abi(), dwinitializedomainflags).ok()
    }
    pub unsafe fn SetPropertiesForDefaultAppDomain(&self, nproperties: u32, pwszpropertynames: *const windows_core::PCWSTR, pwszpropertyvalues: *const windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPropertiesForDefaultAppDomain)(windows_core::Interface::as_raw(self), nproperties, pwszpropertynames, pwszpropertyvalues).ok()
    }
}
#[repr(C)]
pub struct ICLRDomainManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAppDomainManagerType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, EInitializeNewDomainFlags) -> windows_core::HRESULT,
    pub SetPropertiesForDefaultAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *const windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRErrorReportingManager, ICLRErrorReportingManager_Vtbl, 0x980d2f1a_bf79_4c08_812a_bb9778928f78);
impl core::ops::Deref for ICLRErrorReportingManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRErrorReportingManager, windows_core::IUnknown);
impl ICLRErrorReportingManager {
    pub unsafe fn GetBucketParametersForCurrentException(&self, pparams: *mut BucketParameters) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBucketParametersForCurrentException)(windows_core::Interface::as_raw(self), pparams).ok()
    }
    pub unsafe fn BeginCustomDump(&self, dwflavor: ECustomDumpFlavor, items: &[CustomDumpItem], dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginCustomDump)(windows_core::Interface::as_raw(self), dwflavor, items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr()), dwreserved).ok()
    }
    pub unsafe fn EndCustomDump(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndCustomDump)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICLRErrorReportingManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBucketParametersForCurrentException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BucketParameters) -> windows_core::HRESULT,
    pub BeginCustomDump: unsafe extern "system" fn(*mut core::ffi::c_void, ECustomDumpFlavor, u32, *const CustomDumpItem, u32) -> windows_core::HRESULT,
    pub EndCustomDump: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRGCManager, ICLRGCManager_Vtbl, 0x54d9007e_a8e2_4885_b7bf_f998deee4f2a);
impl core::ops::Deref for ICLRGCManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRGCManager, windows_core::IUnknown);
impl ICLRGCManager {
    pub unsafe fn Collect(&self, generation: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Collect)(windows_core::Interface::as_raw(self), generation).ok()
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStats)(windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGCStartupLimits)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
}
#[repr(C)]
pub struct ICLRGCManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Collect: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_GC_STATS) -> windows_core::HRESULT,
    pub SetGCStartupLimits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetGCStartupLimitsEx)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
}
#[repr(C)]
pub struct ICLRGCManager2_Vtbl {
    pub base__: ICLRGCManager_Vtbl,
    pub SetGCStartupLimitsEx: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRHostBindingPolicyManager, ICLRHostBindingPolicyManager_Vtbl, 0x4b3545e7_1856_48c9_a8ba_24b21a753c09);
impl core::ops::Deref for ICLRHostBindingPolicyManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRHostBindingPolicyManager, windows_core::IUnknown);
impl ICLRHostBindingPolicyManager {
    pub unsafe fn ModifyApplicationPolicy<P0, P1>(&self, pwzsourceassemblyidentity: P0, pwztargetassemblyidentity: P1, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ModifyApplicationPolicy)(windows_core::Interface::as_raw(self), pwzsourceassemblyidentity.param().abi(), pwztargetassemblyidentity.param().abi(), pbapplicationpolicy, cbapppolicysize, dwpolicymodifyflags, pbnewapplicationpolicy, pcbnewapppolicysize).ok()
    }
    pub unsafe fn EvaluatePolicy<P0>(&self, pwzreferenceidentity: P0, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EvaluatePolicy)(windows_core::Interface::as_raw(self), pwzreferenceidentity.param().abi(), pbapplicationpolicy, cbapppolicysize, core::mem::transmute(pwzpostpolicyreferenceidentity), pcchpostpolicyreferenceidentity, pdwpoliciesapplied).ok()
    }
}
#[repr(C)]
pub struct ICLRHostBindingPolicyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ModifyApplicationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub EvaluatePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, windows_core::PWSTR, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRHostProtectionManager, ICLRHostProtectionManager_Vtbl, 0x89f25f5c_ceef_43e1_9cfa_a68ce863aaac);
impl core::ops::Deref for ICLRHostProtectionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRHostProtectionManager, windows_core::IUnknown);
impl ICLRHostProtectionManager {
    pub unsafe fn SetProtectedCategories(&self, categories: EApiCategories) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProtectedCategories)(windows_core::Interface::as_raw(self), categories).ok()
    }
    pub unsafe fn SetEagerSerializeGrantSets(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEagerSerializeGrantSets)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICLRHostProtectionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProtectedCategories: unsafe extern "system" fn(*mut core::ffi::c_void, EApiCategories) -> windows_core::HRESULT,
    pub SetEagerSerializeGrantSets: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRIoCompletionManager, ICLRIoCompletionManager_Vtbl, 0x2d74ce86_b8d6_4c84_b3a7_9768933b3c12);
impl core::ops::Deref for ICLRIoCompletionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRIoCompletionManager, windows_core::IUnknown);
impl ICLRIoCompletionManager {
    pub unsafe fn OnComplete(&self, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnComplete)(windows_core::Interface::as_raw(self), dwerrorcode, numberofbytestransferred, pvoverlapped).ok()
    }
}
#[repr(C)]
pub struct ICLRIoCompletionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRMemoryNotificationCallback, ICLRMemoryNotificationCallback_Vtbl, 0x47eb8e57_0846_4546_af76_6f42fcfc2649);
impl core::ops::Deref for ICLRMemoryNotificationCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRMemoryNotificationCallback, windows_core::IUnknown);
impl ICLRMemoryNotificationCallback {
    pub unsafe fn OnMemoryNotification(&self, ememoryavailable: EMemoryAvailable) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMemoryNotification)(windows_core::Interface::as_raw(self), ememoryavailable).ok()
    }
}
#[repr(C)]
pub struct ICLRMemoryNotificationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMemoryNotification: unsafe extern "system" fn(*mut core::ffi::c_void, EMemoryAvailable) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRMetaHost, ICLRMetaHost_Vtbl, 0xd332db9e_b9b3_4125_8207_a14884f53216);
impl core::ops::Deref for ICLRMetaHost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRMetaHost, windows_core::IUnknown);
impl ICLRMetaHost {
    pub unsafe fn GetRuntime<P0, T>(&self, pwzversion: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetRuntime)(windows_core::Interface::as_raw(self), pwzversion.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVersionFromFile<P0>(&self, pwzfilepath: P0, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetVersionFromFile)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), core::mem::transmute(pwzbuffer), pcchbuffer).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateInstalledRuntimes(&self) -> windows_core::Result<super::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateInstalledRuntimes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateLoadedRuntimes<P0>(&self, hndprocess: P0) -> windows_core::Result<super::Com::IEnumUnknown>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateLoadedRuntimes)(windows_core::Interface::as_raw(self), hndprocess.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestRuntimeLoadedNotification(&self, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestRuntimeLoadedNotification)(windows_core::Interface::as_raw(self), pcallbackfunction).ok()
    }
    pub unsafe fn QueryLegacyV2RuntimeBinding<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).QueryLegacyV2RuntimeBinding)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExitProcess(&self, iexitcode: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExitProcess)(windows_core::Interface::as_raw(self), iexitcode).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(ICLRMetaHostPolicy, ICLRMetaHostPolicy_Vtbl, 0xe2190695_77b2_492e_8e14_c4b3a7fdd593);
impl core::ops::Deref for ICLRMetaHostPolicy {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRMetaHostPolicy, windows_core::IUnknown);
impl ICLRMetaHostPolicy {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRequestedRuntime<P0, P1, T>(&self, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: P0, pcfgstream: P1, pwzversion: windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::Com::IStream>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetRequestedRuntime)(windows_core::Interface::as_raw(self), dwpolicyflags, pwzbinary.param().abi(), pcfgstream.param().abi(), core::mem::transmute(pwzversion), pcchversion, core::mem::transmute(pwzimageversion), pcchimageversion, pdwconfigflags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICLRMetaHostPolicy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRequestedRuntime: unsafe extern "system" fn(*mut core::ffi::c_void, METAHOST_POLICY_FLAGS, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PWSTR, *mut u32, windows_core::PWSTR, *mut u32, *mut u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRequestedRuntime: usize,
}
windows_core::imp::define_interface!(ICLROnEventManager, ICLROnEventManager_Vtbl, 0x1d0e0132_e64f_493d_9260_025c0e32c175);
impl core::ops::Deref for ICLROnEventManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLROnEventManager, windows_core::IUnknown);
impl ICLROnEventManager {
    pub unsafe fn RegisterActionOnEvent<P0>(&self, event: EClrEvent, paction: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActionOnCLREvent>,
    {
        (windows_core::Interface::vtable(self).RegisterActionOnEvent)(windows_core::Interface::as_raw(self), event, paction.param().abi()).ok()
    }
    pub unsafe fn UnregisterActionOnEvent<P0>(&self, event: EClrEvent, paction: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActionOnCLREvent>,
    {
        (windows_core::Interface::vtable(self).UnregisterActionOnEvent)(windows_core::Interface::as_raw(self), event, paction.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICLROnEventManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterActionOnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, EClrEvent, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterActionOnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, EClrEvent, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRPolicyManager, ICLRPolicyManager_Vtbl, 0x7d290010_d781_45da_a6f8_aa5d711a730e);
impl core::ops::Deref for ICLRPolicyManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRPolicyManager, windows_core::IUnknown);
impl ICLRPolicyManager {
    pub unsafe fn SetDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultAction)(windows_core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn SetTimeout(&self, operation: EClrOperation, dwmilliseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTimeout)(windows_core::Interface::as_raw(self), operation, dwmilliseconds).ok()
    }
    pub unsafe fn SetActionOnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActionOnTimeout)(windows_core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn SetTimeoutAndAction(&self, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTimeoutAndAction)(windows_core::Interface::as_raw(self), operation, dwmilliseconds, action).ok()
    }
    pub unsafe fn SetActionOnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActionOnFailure)(windows_core::Interface::as_raw(self), failure, action).ok()
    }
    pub unsafe fn SetUnhandledExceptionPolicy(&self, policy: EClrUnhandledException) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUnhandledExceptionPolicy)(windows_core::Interface::as_raw(self), policy).ok()
    }
}
#[repr(C)]
pub struct ICLRPolicyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, u32) -> windows_core::HRESULT,
    pub SetActionOnTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub SetTimeoutAndAction: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, u32, EPolicyAction) -> windows_core::HRESULT,
    pub SetActionOnFailure: unsafe extern "system" fn(*mut core::ffi::c_void, EClrFailure, EPolicyAction) -> windows_core::HRESULT,
    pub SetUnhandledExceptionPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, EClrUnhandledException) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRProbingAssemblyEnum, ICLRProbingAssemblyEnum_Vtbl, 0xd0c5fb1f_416b_4f97_81f4_7ac7dc24dd5d);
impl core::ops::Deref for ICLRProbingAssemblyEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRProbingAssemblyEnum, windows_core::IUnknown);
impl ICLRProbingAssemblyEnum {
    pub unsafe fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
}
#[repr(C)]
pub struct ICLRProbingAssemblyEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRProfiling, ICLRProfiling_Vtbl, 0xb349abe3_b56f_4689_bfcd_76bf39d888ea);
impl core::ops::Deref for ICLRProfiling {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRProfiling, windows_core::IUnknown);
impl ICLRProfiling {
    pub unsafe fn AttachProfiler<P0>(&self, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const windows_core::GUID, wszprofilerpath: P0, pvclientdata: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AttachProfiler)(windows_core::Interface::as_raw(self), dwprofileeprocessid, dwmillisecondsmax, pclsidprofiler, wszprofilerpath.param().abi(), core::mem::transmute(pvclientdata.as_ptr()), pvclientdata.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct ICLRProfiling_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AttachProfiler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, windows_core::PCWSTR, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRReferenceAssemblyEnum, ICLRReferenceAssemblyEnum_Vtbl, 0xd509cb5d_cf32_4876_ae61_67770cf91973);
impl core::ops::Deref for ICLRReferenceAssemblyEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRReferenceAssemblyEnum, windows_core::IUnknown);
impl ICLRReferenceAssemblyEnum {
    pub unsafe fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(pwzbuffer), pcchbuffersize).ok()
    }
}
#[repr(C)]
pub struct ICLRReferenceAssemblyEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRRuntimeHost, ICLRRuntimeHost_Vtbl, 0x90f1a06c_7712_4762_86b5_7a5eba6bdb02);
impl core::ops::Deref for ICLRRuntimeHost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRRuntimeHost, windows_core::IUnknown);
impl ICLRRuntimeHost {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetHostControl<P0>(&self, phostcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IHostControl>,
    {
        (windows_core::Interface::vtable(self).SetHostControl)(windows_core::Interface::as_raw(self), phostcontrol.param().abi()).ok()
    }
    pub unsafe fn GetCLRControl(&self) -> windows_core::Result<ICLRControl> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCLRControl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnloadAppDomain<P0>(&self, dwappdomainid: u32, fwaituntildone: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UnloadAppDomain)(windows_core::Interface::as_raw(self), dwappdomainid, fwaituntildone.param().abi()).ok()
    }
    pub unsafe fn ExecuteInAppDomain(&self, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExecuteInAppDomain)(windows_core::Interface::as_raw(self), dwappdomainid, pcallback, cookie).ok()
    }
    pub unsafe fn GetCurrentAppDomainId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentAppDomainId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ExecuteApplication<P0>(&self, pwzappfullname: P0, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecuteApplication)(windows_core::Interface::as_raw(self), pwzappfullname.param().abi(), dwmanifestpaths, ppwzmanifestpaths, dwactivationdata, ppwzactivationdata, &mut result__).map(|| result__)
    }
    pub unsafe fn ExecuteInDefaultAppDomain<P0, P1, P2, P3>(&self, pwzassemblypath: P0, pwztypename: P1, pwzmethodname: P2, pwzargument: P3) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecuteInDefaultAppDomain)(windows_core::Interface::as_raw(self), pwzassemblypath.param().abi(), pwztypename.param().abi(), pwzmethodname.param().abi(), pwzargument.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICLRRuntimeHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHostControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCLRControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnloadAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ExecuteInAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, FExecuteInAppDomainCallback, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentAppDomainId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExecuteApplication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, u32, *const windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub ExecuteInDefaultAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRRuntimeInfo, ICLRRuntimeInfo_Vtbl, 0xbd39d1d2_ba2f_486a_89b0_b4b0cb466891);
impl core::ops::Deref for ICLRRuntimeInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRRuntimeInfo, windows_core::IUnknown);
impl ICLRRuntimeInfo {
    pub unsafe fn GetVersionString(&self, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVersionString)(windows_core::Interface::as_raw(self), core::mem::transmute(pwzbuffer), pcchbuffer).ok()
    }
    pub unsafe fn GetRuntimeDirectory(&self, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRuntimeDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute(pwzbuffer), pcchbuffer).ok()
    }
    pub unsafe fn IsLoaded<P0>(&self, hndprocess: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLoaded)(windows_core::Interface::as_raw(self), hndprocess.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn LoadErrorString(&self, iresourceid: u32, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadErrorString)(windows_core::Interface::as_raw(self), iresourceid, core::mem::transmute(pwzbuffer), pcchbuffer, ilocaleid).ok()
    }
    pub unsafe fn LoadLibraryA<P0>(&self, pwzdllname: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadLibraryA)(windows_core::Interface::as_raw(self), pwzdllname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProcAddress<P0>(&self, pszprocname: P0) -> windows_core::Result<*mut core::ffi::c_void>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProcAddress)(windows_core::Interface::as_raw(self), pszprocname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetInterface<T>(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetInterface)(windows_core::Interface::as_raw(self), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsLoadable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLoadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDefaultStartupFlags<P0>(&self, dwstartupflags: u32, pwzhostconfigfile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefaultStartupFlags)(windows_core::Interface::as_raw(self), dwstartupflags, pwzhostconfigfile.param().abi()).ok()
    }
    pub unsafe fn GetDefaultStartupFlags(&self, pdwstartupflags: *mut u32, pwzhostconfigfile: windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultStartupFlags)(windows_core::Interface::as_raw(self), pdwstartupflags, core::mem::transmute(pwzhostconfigfile), pcchhostconfigfile).ok()
    }
    pub unsafe fn BindAsLegacyV2Runtime(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindAsLegacyV2Runtime)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsStarted(&self, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsStarted)(windows_core::Interface::as_raw(self), pbstarted, pdwstartupflags).ok()
    }
}
#[repr(C)]
pub struct ICLRRuntimeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVersionString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetRuntimeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub IsLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub LoadErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32, i32) -> windows_core::HRESULT,
    pub LoadLibraryA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT,
    pub GetProcAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsLoadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetDefaultStartupFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDefaultStartupFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub BindAsLegacyV2Runtime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRStrongName, ICLRStrongName_Vtbl, 0x9fd93ccf_3280_4391_b3a9_96e1cde77c8d);
impl core::ops::Deref for ICLRStrongName {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRStrongName, windows_core::IUnknown);
impl ICLRStrongName {
    pub unsafe fn GetHashFromAssemblyFile<P0>(&self, pszfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetHashFromAssemblyFile)(windows_core::Interface::as_raw(self), pszfilepath.param().abi(), pihashalg, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash).ok()
    }
    pub unsafe fn GetHashFromAssemblyFileW<P0>(&self, pwzfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetHashFromAssemblyFileW)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pihashalg, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash).ok()
    }
    pub unsafe fn GetHashFromBlob(&self, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHashFromBlob)(windows_core::Interface::as_raw(self), pbblob, cchblob, pihashalg, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash).ok()
    }
    pub unsafe fn GetHashFromFile<P0>(&self, pszfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetHashFromFile)(windows_core::Interface::as_raw(self), pszfilepath.param().abi(), pihashalg, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash).ok()
    }
    pub unsafe fn GetHashFromFileW<P0>(&self, pwzfilepath: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetHashFromFileW)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pihashalg, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash).ok()
    }
    pub unsafe fn GetHashFromHandle<P0>(&self, hfile: P0, pihashalg: *mut u32, pbhash: &mut [u8], pchhash: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).GetHashFromHandle)(windows_core::Interface::as_raw(self), hfile.param().abi(), pihashalg, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), pchhash).ok()
    }
    pub unsafe fn StrongNameCompareAssemblies<P0, P1>(&self, pwzassembly1: P0, pwzassembly2: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrongNameCompareAssemblies)(windows_core::Interface::as_raw(self), pwzassembly1.param().abi(), pwzassembly2.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn StrongNameFreeBuffer(&self, pbmemory: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StrongNameFreeBuffer)(windows_core::Interface::as_raw(self), pbmemory).ok()
    }
    pub unsafe fn StrongNameGetBlob<P0>(&self, pwzfilepath: P0, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameGetBlob)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pbblob, pcbblob).ok()
    }
    pub unsafe fn StrongNameGetBlobFromImage(&self, pbbase: &[u8], pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StrongNameGetBlobFromImage)(windows_core::Interface::as_raw(self), core::mem::transmute(pbbase.as_ptr()), pbbase.len().try_into().unwrap(), pbblob, pcbblob).ok()
    }
    pub unsafe fn StrongNameGetPublicKey<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameGetPublicKey)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbpublickeyblob, pcbpublickeyblob).ok()
    }
    pub unsafe fn StrongNameHashSize(&self, ulhashalg: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrongNameHashSize)(windows_core::Interface::as_raw(self), ulhashalg, &mut result__).map(|| result__)
    }
    pub unsafe fn StrongNameKeyDelete<P0>(&self, pwzkeycontainer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameKeyDelete)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi()).ok()
    }
    pub unsafe fn StrongNameKeyGen<P0>(&self, pwzkeycontainer: P0, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameKeyGen)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), dwflags, ppbkeyblob, pcbkeyblob).ok()
    }
    pub unsafe fn StrongNameKeyGenEx<P0>(&self, pwzkeycontainer: P0, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameKeyGenEx)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), dwflags, dwkeysize, ppbkeyblob, pcbkeyblob).ok()
    }
    pub unsafe fn StrongNameKeyInstall<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *const u8, cbkeyblob: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameKeyInstall)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob).ok()
    }
    pub unsafe fn StrongNameSignatureGeneration<P0, P1>(&self, pwzfilepath: P0, pwzkeycontainer: P1, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameSignatureGeneration)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbsignatureblob, pcbsignatureblob).ok()
    }
    pub unsafe fn StrongNameSignatureGenerationEx<P0, P1>(&self, wszfilepath: P0, wszkeycontainer: P1, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameSignatureGenerationEx)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), wszkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbsignatureblob, pcbsignatureblob, dwflags).ok()
    }
    pub unsafe fn StrongNameSignatureSize(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StrongNameSignatureSize)(windows_core::Interface::as_raw(self), pbpublickeyblob, cbpublickeyblob, pcbsize).ok()
    }
    pub unsafe fn StrongNameSignatureVerification<P0>(&self, pwzfilepath: P0, dwinflags: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrongNameSignatureVerification)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), dwinflags, &mut result__).map(|| result__)
    }
    pub unsafe fn StrongNameSignatureVerificationEx<P0, P1>(&self, pwzfilepath: P0, fforceverification: P1) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrongNameSignatureVerificationEx)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), fforceverification.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn StrongNameSignatureVerificationFromImage(&self, pbbase: *const u8, dwlength: u32, dwinflags: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrongNameSignatureVerificationFromImage)(windows_core::Interface::as_raw(self), pbbase, dwlength, dwinflags, &mut result__).map(|| result__)
    }
    pub unsafe fn StrongNameTokenFromAssembly<P0>(&self, pwzfilepath: P0, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameTokenFromAssembly)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), ppbstrongnametoken, pcbstrongnametoken).ok()
    }
    pub unsafe fn StrongNameTokenFromAssemblyEx<P0>(&self, pwzfilepath: P0, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameTokenFromAssemblyEx)(windows_core::Interface::as_raw(self), pwzfilepath.param().abi(), ppbstrongnametoken, pcbstrongnametoken, ppbpublickeyblob, pcbpublickeyblob).ok()
    }
    pub unsafe fn StrongNameTokenFromPublicKey(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StrongNameTokenFromPublicKey)(windows_core::Interface::as_raw(self), pbpublickeyblob, cbpublickeyblob, ppbstrongnametoken, pcbstrongnametoken).ok()
    }
}
#[repr(C)]
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
    pub StrongNameSignatureVerificationEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOLEAN, *mut u8) -> windows_core::HRESULT,
    pub StrongNameSignatureVerificationFromImage: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub StrongNameTokenFromAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameTokenFromAssemblyEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u8, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub StrongNameTokenFromPublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRStrongName2, ICLRStrongName2_Vtbl, 0xc22ed5c5_4b59_4975_90eb_85ea55c0069b);
impl core::ops::Deref for ICLRStrongName2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRStrongName2, windows_core::IUnknown);
impl ICLRStrongName2 {
    pub unsafe fn StrongNameGetPublicKeyEx<P0>(&self, pwzkeycontainer: P0, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameGetPublicKeyEx)(windows_core::Interface::as_raw(self), pwzkeycontainer.param().abi(), pbkeyblob, cbkeyblob, ppbpublickeyblob, pcbpublickeyblob, uhashalgid, ureserved).ok()
    }
    pub unsafe fn StrongNameSignatureVerificationEx2<P0, P1>(&self, wszfilepath: P0, fforceverification: P1, pbecmapublickey: *const u8, cbecmapublickey: u32) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StrongNameSignatureVerificationEx2)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), fforceverification.param().abi(), pbecmapublickey, cbecmapublickey, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICLRStrongName2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StrongNameGetPublicKeyEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, *mut *mut u8, *mut u32, u32, u32) -> windows_core::HRESULT,
    pub StrongNameSignatureVerificationEx2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOLEAN, *const u8, u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRStrongName3, ICLRStrongName3_Vtbl, 0x22c7089b_bbd3_414a_b698_210f263f1fed);
impl core::ops::Deref for ICLRStrongName3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRStrongName3, windows_core::IUnknown);
impl ICLRStrongName3 {
    pub unsafe fn StrongNameDigestGenerate<P0>(&self, wszfilepath: P0, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameDigestGenerate)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), ppbdigestblob, pcbdigestblob, dwflags).ok()
    }
    pub unsafe fn StrongNameDigestSign<P0>(&self, wszkeycontainer: P0, pbkeyblob: &[u8], pbdigestblob: &[u8], hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameDigestSign)(windows_core::Interface::as_raw(self), wszkeycontainer.param().abi(), core::mem::transmute(pbkeyblob.as_ptr()), pbkeyblob.len().try_into().unwrap(), core::mem::transmute(pbdigestblob.as_ptr()), pbdigestblob.len().try_into().unwrap(), hashalgid, ppbsignatureblob, pcbsignatureblob, dwflags).ok()
    }
    pub unsafe fn StrongNameDigestEmbed<P0>(&self, wszfilepath: P0, pbsignatureblob: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).StrongNameDigestEmbed)(windows_core::Interface::as_raw(self), wszfilepath.param().abi(), core::mem::transmute(pbsignatureblob.as_ptr()), pbsignatureblob.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct ICLRStrongName3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StrongNameDigestGenerate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u8, *mut u32, u32) -> windows_core::HRESULT,
    pub StrongNameDigestSign: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, *const u8, u32, u32, *mut *mut u8, *mut u32, u32) -> windows_core::HRESULT,
    pub StrongNameDigestEmbed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRSyncManager, ICLRSyncManager_Vtbl, 0x55ff199d_ad21_48f9_a16c_f24ebbb8727d);
impl core::ops::Deref for ICLRSyncManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRSyncManager, windows_core::IUnknown);
impl ICLRSyncManager {
    pub unsafe fn GetMonitorOwner(&self, cookie: usize) -> windows_core::Result<IHostTask> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMonitorOwner)(windows_core::Interface::as_raw(self), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRWLockOwnerIterator(&self, cookie: usize) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRWLockOwnerIterator)(windows_core::Interface::as_raw(self), cookie, &mut result__).map(|| result__)
    }
    pub unsafe fn GetRWLockOwnerNext(&self, iterator: usize) -> windows_core::Result<IHostTask> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRWLockOwnerNext)(windows_core::Interface::as_raw(self), iterator, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteRWLockOwnerIterator(&self, iterator: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteRWLockOwnerIterator)(windows_core::Interface::as_raw(self), iterator).ok()
    }
}
#[repr(C)]
pub struct ICLRSyncManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMonitorOwner: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRWLockOwnerIterator: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub GetRWLockOwnerNext: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRWLockOwnerIterator: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRTask, ICLRTask_Vtbl, 0x28e66a4a_9906_4225_b231_9187c3eb8611);
impl core::ops::Deref for ICLRTask {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRTask, windows_core::IUnknown);
impl ICLRTask {
    pub unsafe fn SwitchIn<P0>(&self, threadhandle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SwitchIn)(windows_core::Interface::as_raw(self), threadhandle.param().abi()).ok()
    }
    pub unsafe fn SwitchOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SwitchOut)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMemStats(&self) -> windows_core::Result<COR_GC_THREAD_STATS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMemStats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Reset<P0>(&self, ffull: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), ffull.param().abi()).ok()
    }
    pub unsafe fn ExitTask(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExitTask)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RudeAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RudeAbort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NeedsPriorityScheduling(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NeedsPriorityScheduling)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn YieldTask(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).YieldTask)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LocksHeld(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocksHeld)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTaskIdentifier(&self, asked: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTaskIdentifier)(windows_core::Interface::as_raw(self), asked).ok()
    }
}
#[repr(C)]
pub struct ICLRTask_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SwitchIn: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SwitchOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMemStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ExitTask: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RudeAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NeedsPriorityScheduling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub YieldTask: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocksHeld: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub SetTaskIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).BeginPreventAsyncAbort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndPreventAsyncAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndPreventAsyncAbort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICLRTask2_Vtbl {
    pub base__: ICLRTask_Vtbl,
    pub BeginPreventAsyncAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndPreventAsyncAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICLRTaskManager, ICLRTaskManager_Vtbl, 0x4862efbe_3ae5_44f8_8feb_346190ee8a34);
impl core::ops::Deref for ICLRTaskManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICLRTaskManager, windows_core::IUnknown);
impl ICLRTaskManager {
    pub unsafe fn CreateTask(&self) -> windows_core::Result<ICLRTask> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCurrentTask(&self) -> windows_core::Result<ICLRTask> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentTask)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUILocale)(windows_core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLocale(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn GetCurrentTaskType(&self) -> windows_core::Result<ETaskType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentTaskType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICLRTaskManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUILocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCurrentTaskType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ETaskType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICatalogServices, ICatalogServices_Vtbl, 0x04c6be1e_1db1_4058_ab7a_700cccfbf254);
impl core::ops::Deref for ICatalogServices {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICatalogServices, windows_core::IUnknown);
impl ICatalogServices {
    pub unsafe fn Autodone(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Autodone)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotAutodone(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotAutodone)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICatalogServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Autodone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotAutodone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorConfiguration, ICorConfiguration_Vtbl, 0x5c2b07a5_1e98_11d3_872f_00c04f79ed0d);
impl core::ops::Deref for ICorConfiguration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorConfiguration, windows_core::IUnknown);
impl ICorConfiguration {
    pub unsafe fn SetGCThreadControl<P0>(&self, pgcthreadcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGCThreadControl>,
    {
        (windows_core::Interface::vtable(self).SetGCThreadControl)(windows_core::Interface::as_raw(self), pgcthreadcontrol.param().abi()).ok()
    }
    pub unsafe fn SetGCHostControl<P0>(&self, pgchostcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IGCHostControl>,
    {
        (windows_core::Interface::vtable(self).SetGCHostControl)(windows_core::Interface::as_raw(self), pgchostcontrol.param().abi()).ok()
    }
    pub unsafe fn SetDebuggerThreadControl<P0>(&self, pdebuggerthreadcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebuggerThreadControl>,
    {
        (windows_core::Interface::vtable(self).SetDebuggerThreadControl)(windows_core::Interface::as_raw(self), pdebuggerthreadcontrol.param().abi()).ok()
    }
    pub unsafe fn AddDebuggerSpecialThread(&self, dwspecialthreadid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddDebuggerSpecialThread)(windows_core::Interface::as_raw(self), dwspecialthreadid).ok()
    }
}
#[repr(C)]
pub struct ICorConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGCThreadControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGCHostControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDebuggerThreadControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDebuggerSpecialThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorRuntimeHost, ICorRuntimeHost_Vtbl, 0xcb2f6722_ab3a_11d2_9c40_00c04fa30a3e);
impl core::ops::Deref for ICorRuntimeHost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorRuntimeHost, windows_core::IUnknown);
impl ICorRuntimeHost {
    pub unsafe fn CreateLogicalThreadState(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateLogicalThreadState)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteLogicalThreadState(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteLogicalThreadState)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SwitchInLogicalThreadState(&self, pfibercookie: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SwitchInLogicalThreadState)(windows_core::Interface::as_raw(self), pfibercookie).ok()
    }
    pub unsafe fn SwitchOutLogicalThreadState(&self) -> windows_core::Result<*mut u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SwitchOutLogicalThreadState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocksHeldByLogicalThread(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocksHeldByLogicalThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MapFile<P0>(&self, hfile: P0) -> windows_core::Result<super::super::Foundation::HMODULE>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MapFile)(windows_core::Interface::as_raw(self), hfile.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConfiguration(&self) -> windows_core::Result<ICorConfiguration> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDomain<P0, P1>(&self, pwzfriendlyname: P0, pidentityarray: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDomain)(windows_core::Interface::as_raw(self), pwzfriendlyname.param().abi(), pidentityarray.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDefaultDomain(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultDomain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumDomains(&self, henum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumDomains)(windows_core::Interface::as_raw(self), henum).ok()
    }
    pub unsafe fn NextDomain(&self, henum: *const core::ffi::c_void) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextDomain)(windows_core::Interface::as_raw(self), henum, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CloseEnum(&self, henum: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseEnum)(windows_core::Interface::as_raw(self), henum).ok()
    }
    pub unsafe fn CreateDomainEx<P0, P1, P2>(&self, pwzfriendlyname: P0, psetup: P1, pevidence: P2) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDomainEx)(windows_core::Interface::as_raw(self), pwzfriendlyname.param().abi(), psetup.param().abi(), pevidence.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDomainSetup(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDomainSetup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEvidence(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEvidence)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnloadDomain<P0>(&self, pappdomain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).UnloadDomain)(windows_core::Interface::as_raw(self), pappdomain.param().abi()).ok()
    }
    pub unsafe fn CurrentDomain(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDomain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(ICorThreadpool, ICorThreadpool_Vtbl, 0x84680d3a_b2c1_46e8_acc2_dbc0a359159a);
impl core::ops::Deref for ICorThreadpool {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorThreadpool, windows_core::IUnknown);
impl ICorThreadpool {
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorRegisterWaitForSingleObject<P0, P1>(&self, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: P0, callback: super::Threading::WAITORTIMERCALLBACK, context: *const core::ffi::c_void, timeout: u32, executeonlyonce: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorRegisterWaitForSingleObject)(windows_core::Interface::as_raw(self), phnewwaitobject, hwaitobject.param().abi(), callback, context, timeout, executeonlyonce.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CorUnregisterWait<P0, P1>(&self, hwaitobject: P0, completionevent: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorUnregisterWait)(windows_core::Interface::as_raw(self), hwaitobject.param().abi(), completionevent.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorQueueUserWorkItem<P0>(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, executeonlyonce: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorQueueUserWorkItem)(windows_core::Interface::as_raw(self), function, context, executeonlyonce.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorCreateTimer(&self, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const core::ffi::c_void, duetime: u32, period: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorCreateTimer)(windows_core::Interface::as_raw(self), phnewtimer, callback, parameter, duetime, period, &mut result__).map(|| result__)
    }
    pub unsafe fn CorChangeTimer<P0>(&self, timer: P0, duetime: u32, period: u32) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorChangeTimer)(windows_core::Interface::as_raw(self), timer.param().abi(), duetime, period, &mut result__).map(|| result__)
    }
    pub unsafe fn CorDeleteTimer<P0, P1>(&self, timer: P0, completionevent: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorDeleteTimer)(windows_core::Interface::as_raw(self), timer.param().abi(), completionevent.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn CorBindIoCompletionCallback<P0>(&self, filehandle: P0, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).CorBindIoCompletionCallback)(windows_core::Interface::as_raw(self), filehandle.param().abi(), callback).ok()
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CorCallOrQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorCallOrQueueUserWorkItem)(windows_core::Interface::as_raw(self), function, context, &mut result__).map(|| result__)
    }
    pub unsafe fn CorSetMaxThreads(&self, maxworkerthreads: u32, maxiocompletionthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CorSetMaxThreads)(windows_core::Interface::as_raw(self), maxworkerthreads, maxiocompletionthreads).ok()
    }
    pub unsafe fn CorGetMaxThreads(&self, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CorGetMaxThreads)(windows_core::Interface::as_raw(self), maxworkerthreads, maxiocompletionthreads).ok()
    }
    pub unsafe fn CorGetAvailableThreads(&self, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CorGetAvailableThreads)(windows_core::Interface::as_raw(self), availableworkerthreads, availableiocompletionthreads).ok()
    }
}
#[repr(C)]
pub struct ICorThreadpool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorRegisterWaitForSingleObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, super::Threading::WAITORTIMERCALLBACK, *const core::ffi::c_void, u32, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorRegisterWaitForSingleObject: usize,
    pub CorUnregisterWait: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorQueueUserWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::Threading::LPTHREAD_START_ROUTINE, *const core::ffi::c_void, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorQueueUserWorkItem: usize,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorCreateTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::HANDLE, super::Threading::WAITORTIMERCALLBACK, *const core::ffi::c_void, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorCreateTimer: usize,
    pub CorChangeTimer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, u32, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CorDeleteTimer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_IO")]
    pub CorBindIoCompletionCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    CorBindIoCompletionCallback: usize,
    #[cfg(feature = "Win32_System_Threading")]
    pub CorCallOrQueueUserWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::Threading::LPTHREAD_START_ROUTINE, *const core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Threading"))]
    CorCallOrQueueUserWorkItem: usize,
    pub CorSetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub CorGetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub CorGetAvailableThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDebuggerInfo, IDebuggerInfo_Vtbl, 0xbf24142d_a47d_4d24_a66d_8c2141944e44);
impl core::ops::Deref for IDebuggerInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebuggerInfo, windows_core::IUnknown);
impl IDebuggerInfo {
    pub unsafe fn IsDebuggerAttached(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDebuggerAttached)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDebuggerInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDebuggerAttached: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDebuggerThreadControl, IDebuggerThreadControl_Vtbl, 0x23d86786_0bb5_4774_8fb5_e3522add6246);
impl core::ops::Deref for IDebuggerThreadControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebuggerThreadControl, windows_core::IUnknown);
impl IDebuggerThreadControl {
    pub unsafe fn ThreadIsBlockingForDebugger(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadIsBlockingForDebugger)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseAllRuntimeThreads(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseAllRuntimeThreads)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartBlockingForDebugger(&self, dwunused: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartBlockingForDebugger)(windows_core::Interface::as_raw(self), dwunused).ok()
    }
}
#[repr(C)]
pub struct IDebuggerThreadControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadIsBlockingForDebugger: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseAllRuntimeThreads: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartBlockingForDebugger: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGCHost, IGCHost_Vtbl, 0xfac34f6e_0dcd_47b5_8021_531bc5ecca63);
impl core::ops::Deref for IGCHost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGCHost, windows_core::IUnknown);
impl IGCHost {
    pub unsafe fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGCStartupLimits)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
    pub unsafe fn Collect(&self, generation: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Collect)(windows_core::Interface::as_raw(self), generation).ok()
    }
    pub unsafe fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStats)(windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetThreadStats(&self, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetThreadStats)(windows_core::Interface::as_raw(self), pfibercookie, pstats).ok()
    }
    pub unsafe fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVirtualMemLimit)(windows_core::Interface::as_raw(self), sztmaxvirtualmemmb).ok()
    }
}
#[repr(C)]
pub struct IGCHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGCStartupLimits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Collect: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_GC_STATS) -> windows_core::HRESULT,
    pub GetThreadStats: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT,
    pub SetVirtualMemLimit: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetGCStartupLimitsEx)(windows_core::Interface::as_raw(self), segmentsize, maxgen0size).ok()
    }
}
#[repr(C)]
pub struct IGCHost2_Vtbl {
    pub base__: IGCHost_Vtbl,
    pub SetGCStartupLimitsEx: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGCHostControl, IGCHostControl_Vtbl, 0x5513d564_8374_4cb9_aed9_0083f4160a1d);
impl core::ops::Deref for IGCHostControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGCHostControl, windows_core::IUnknown);
impl IGCHostControl {
    pub unsafe fn RequestVirtualMemLimit(&self, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestVirtualMemLimit)(windows_core::Interface::as_raw(self), sztmaxvirtualmemmb, psztnewmaxvirtualmemmb).ok()
    }
}
#[repr(C)]
pub struct IGCHostControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestVirtualMemLimit: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGCThreadControl, IGCThreadControl_Vtbl, 0xf31d1788_c397_4725_87a5_6af3472c2791);
impl core::ops::Deref for IGCThreadControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGCThreadControl, windows_core::IUnknown);
impl IGCThreadControl {
    pub unsafe fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadIsBlockingForSuspension)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionStarting(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspensionStarting)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspensionEnding)(windows_core::Interface::as_raw(self), generation).ok()
    }
}
#[repr(C)]
pub struct IGCThreadControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadIsBlockingForSuspension: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionStarting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionEnding: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostAssemblyManager, IHostAssemblyManager_Vtbl, 0x613dabd7_62b2_493e_9e65_c1e32a1e0c5e);
impl core::ops::Deref for IHostAssemblyManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostAssemblyManager, windows_core::IUnknown);
impl IHostAssemblyManager {
    pub unsafe fn GetNonHostStoreAssemblies(&self) -> windows_core::Result<ICLRAssemblyReferenceList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNonHostStoreAssemblies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAssemblyStore(&self) -> windows_core::Result<IHostAssemblyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAssemblyStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IHostAssemblyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNonHostStoreAssemblies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAssemblyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostAssemblyStore, IHostAssemblyStore_Vtbl, 0x7b102a88_3f7f_496d_8fa2_c35374e01af3);
impl core::ops::Deref for IHostAssemblyStore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostAssemblyStore, windows_core::IUnknown);
impl IHostAssemblyStore {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProvideAssembly(&self, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut Option<super::Com::IStream>, ppstmpdb: *mut Option<super::Com::IStream>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProvideAssembly)(windows_core::Interface::as_raw(self), pbindinfo, passemblyid, pcontext, core::mem::transmute(ppstmassemblyimage), core::mem::transmute(ppstmpdb)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProvideModule(&self, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut Option<super::Com::IStream>, ppstmpdb: *mut Option<super::Com::IStream>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProvideModule)(windows_core::Interface::as_raw(self), pbindinfo, pdwmoduleid, core::mem::transmute(ppstmmoduleimage), core::mem::transmute(ppstmpdb)).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IHostAutoEvent, IHostAutoEvent_Vtbl, 0x50b0cfce_4063_4278_9673_e5cb4ed0bdb8);
impl core::ops::Deref for IHostAutoEvent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostAutoEvent, windows_core::IUnknown);
impl IHostAutoEvent {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn Set(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IHostAutoEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostControl, IHostControl_Vtbl, 0x02ca073c_7079_4860_880a_c2f7a449c991);
impl core::ops::Deref for IHostControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostControl, windows_core::IUnknown);
impl IHostControl {
    pub unsafe fn GetHostManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHostManager)(windows_core::Interface::as_raw(self), riid, ppobject).ok()
    }
    pub unsafe fn SetAppDomainManager<P0>(&self, dwappdomainid: u32, punkappdomainmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetAppDomainManager)(windows_core::Interface::as_raw(self), dwappdomainid, punkappdomainmanager.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IHostControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHostManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppDomainManager: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostCrst, IHostCrst_Vtbl, 0x6df710a6_26a4_4a65_8cd5_7237b8bda8dc);
impl core::ops::Deref for IHostCrst {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostCrst, windows_core::IUnknown);
impl IHostCrst {
    pub unsafe fn Enter(&self, option: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn Leave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Leave)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TryEnter(&self, option: u32) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TryEnter)(windows_core::Interface::as_raw(self), option, &mut result__).map(|| result__)
    }
    pub unsafe fn SetSpinCount(&self, dwspincount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSpinCount)(windows_core::Interface::as_raw(self), dwspincount).ok()
    }
}
#[repr(C)]
pub struct IHostCrst_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryEnter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetSpinCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostGCManager, IHostGCManager_Vtbl, 0x5d4ec34e_f248_457b_b603_255faaba0d21);
impl core::ops::Deref for IHostGCManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostGCManager, windows_core::IUnknown);
impl IHostGCManager {
    pub unsafe fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadIsBlockingForSuspension)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionStarting(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspensionStarting)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspensionEnding)(windows_core::Interface::as_raw(self), generation).ok()
    }
}
#[repr(C)]
pub struct IHostGCManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadIsBlockingForSuspension: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionStarting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuspensionEnding: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostIoCompletionManager, IHostIoCompletionManager_Vtbl, 0x8bde9d80_ec06_41d6_83e6_22580effcc20);
impl core::ops::Deref for IHostIoCompletionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostIoCompletionManager, windows_core::IUnknown);
impl IHostIoCompletionManager {
    pub unsafe fn CreateIoCompletionPort(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateIoCompletionPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CloseIoCompletionPort<P0>(&self, hport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).CloseIoCompletionPort)(windows_core::Interface::as_raw(self), hport.param().abi()).ok()
    }
    pub unsafe fn SetMaxThreads(&self, dwmaxiocompletionthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxThreads)(windows_core::Interface::as_raw(self), dwmaxiocompletionthreads).ok()
    }
    pub unsafe fn GetMaxThreads(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAvailableThreads(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAvailableThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHostOverlappedSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHostOverlappedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCLRIoCompletionManager<P0>(&self, pmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRIoCompletionManager>,
    {
        (windows_core::Interface::vtable(self).SetCLRIoCompletionManager)(windows_core::Interface::as_raw(self), pmanager.param().abi()).ok()
    }
    pub unsafe fn InitializeHostOverlapped(&self, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeHostOverlapped)(windows_core::Interface::as_raw(self), pvoverlapped).ok()
    }
    pub unsafe fn Bind<P0, P1>(&self, hport: P0, hhandle: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), hport.param().abi(), hhandle.param().abi()).ok()
    }
    pub unsafe fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinThreads)(windows_core::Interface::as_raw(self), dwminiocompletionthreads).ok()
    }
    pub unsafe fn GetMinThreads(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IHostMalloc, IHostMalloc_Vtbl, 0x1831991c_cc53_4a31_b218_04e910446479);
impl core::ops::Deref for IHostMalloc {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostMalloc, windows_core::IUnknown);
impl IHostMalloc {
    pub unsafe fn Alloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cbsize, ecriticallevel, ppmem).ok()
    }
    pub unsafe fn DebugAlloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DebugAlloc)(windows_core::Interface::as_raw(self), cbsize, ecriticallevel, pszfilename, ilineno, ppmem).ok()
    }
    pub unsafe fn Free(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pmem).ok()
    }
}
#[repr(C)]
pub struct IHostMalloc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize, EMemoryCriticalLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize, EMemoryCriticalLevel, *const u8, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostManualEvent, IHostManualEvent_Vtbl, 0x1bf4ec38_affe_4fb9_85a6_525268f15b54);
impl core::ops::Deref for IHostManualEvent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostManualEvent, windows_core::IUnknown);
impl IHostManualEvent {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Set(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IHostManualEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostMemoryManager, IHostMemoryManager_Vtbl, 0x7bc698d1_f9e3_4460_9cde_d04248e9fa25);
impl core::ops::Deref for IHostMemoryManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostMemoryManager, windows_core::IUnknown);
impl IHostMemoryManager {
    pub unsafe fn CreateMalloc(&self, dwmalloctype: u32) -> windows_core::Result<IHostMalloc> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMalloc)(windows_core::Interface::as_raw(self), dwmalloctype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VirtualAlloc(&self, paddress: *const core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VirtualAlloc)(windows_core::Interface::as_raw(self), paddress, dwsize, flallocationtype, flprotect, ecriticallevel, ppmem).ok()
    }
    pub unsafe fn VirtualFree(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VirtualFree)(windows_core::Interface::as_raw(self), lpaddress, dwsize, dwfreetype).ok()
    }
    pub unsafe fn VirtualQuery(&self, lpaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwlength: usize, presult: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VirtualQuery)(windows_core::Interface::as_raw(self), lpaddress, lpbuffer, dwlength, presult).ok()
    }
    pub unsafe fn VirtualProtect(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VirtualProtect)(windows_core::Interface::as_raw(self), lpaddress, dwsize, flnewprotect, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMemoryLoad(&self, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMemoryLoad)(windows_core::Interface::as_raw(self), pmemoryload, pavailablebytes).ok()
    }
    pub unsafe fn RegisterMemoryNotificationCallback<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRMemoryNotificationCallback>,
    {
        (windows_core::Interface::vtable(self).RegisterMemoryNotificationCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
    pub unsafe fn NeedsVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NeedsVirtualAddressSpace)(windows_core::Interface::as_raw(self), startaddress, size).ok()
    }
    pub unsafe fn AcquiredVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AcquiredVirtualAddressSpace)(windows_core::Interface::as_raw(self), startaddress, size).ok()
    }
    pub unsafe fn ReleasedVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleasedVirtualAddressSpace)(windows_core::Interface::as_raw(self), startaddress).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IHostPolicyManager, IHostPolicyManager_Vtbl, 0x7ae49844_b1e3_4683_ba7c_1e8212ea3b79);
impl core::ops::Deref for IHostPolicyManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostPolicyManager, windows_core::IUnknown);
impl IHostPolicyManager {
    pub unsafe fn OnDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDefaultAction)(windows_core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn OnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTimeout)(windows_core::Interface::as_raw(self), operation, action).ok()
    }
    pub unsafe fn OnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnFailure)(windows_core::Interface::as_raw(self), failure, action).ok()
    }
}
#[repr(C)]
pub struct IHostPolicyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub OnTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, EClrOperation, EPolicyAction) -> windows_core::HRESULT,
    pub OnFailure: unsafe extern "system" fn(*mut core::ffi::c_void, EClrFailure, EPolicyAction) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostSecurityContext, IHostSecurityContext_Vtbl, 0x7e573ce4_0343_4423_98d7_6318348a1d3c);
impl core::ops::Deref for IHostSecurityContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostSecurityContext, windows_core::IUnknown);
impl IHostSecurityContext {
    pub unsafe fn Capture(&self) -> windows_core::Result<IHostSecurityContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IHostSecurityContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostSecurityManager, IHostSecurityManager_Vtbl, 0x75ad2468_a349_4d02_a764_76a68aee0c4f);
impl core::ops::Deref for IHostSecurityManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostSecurityManager, windows_core::IUnknown);
impl IHostSecurityManager {
    pub unsafe fn ImpersonateLoggedOnUser<P0>(&self, htoken: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).ImpersonateLoggedOnUser)(windows_core::Interface::as_raw(self), htoken.param().abi()).ok()
    }
    pub unsafe fn RevertToSelf(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RevertToSelf)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OpenThreadToken<P0>(&self, dwdesiredaccess: u32, bopenasself: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenThreadToken)(windows_core::Interface::as_raw(self), dwdesiredaccess, bopenasself.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetThreadToken<P0>(&self, htoken: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetThreadToken)(windows_core::Interface::as_raw(self), htoken.param().abi()).ok()
    }
    pub unsafe fn GetSecurityContext(&self, econtexttype: EContextType) -> windows_core::Result<IHostSecurityContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurityContext)(windows_core::Interface::as_raw(self), econtexttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurityContext<P0>(&self, econtexttype: EContextType, psecuritycontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IHostSecurityContext>,
    {
        (windows_core::Interface::vtable(self).SetSecurityContext)(windows_core::Interface::as_raw(self), econtexttype, psecuritycontext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IHostSecurityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ImpersonateLoggedOnUser: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenThreadToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetThreadToken: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void, EContextType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void, EContextType, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostSemaphore, IHostSemaphore_Vtbl, 0x855efd47_cc09_463a_a97d_16acab882661);
impl core::ops::Deref for IHostSemaphore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostSemaphore, windows_core::IUnknown);
impl IHostSemaphore {
    pub unsafe fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn ReleaseSemaphore(&self, lreleasecount: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReleaseSemaphore)(windows_core::Interface::as_raw(self), lreleasecount, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IHostSemaphore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ReleaseSemaphore: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostSyncManager, IHostSyncManager_Vtbl, 0x234330c7_5f10_4f20_9615_5122dab7a0ac);
impl core::ops::Deref for IHostSyncManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostSyncManager, windows_core::IUnknown);
impl IHostSyncManager {
    pub unsafe fn SetCLRSyncManager<P0>(&self, pmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRSyncManager>,
    {
        (windows_core::Interface::vtable(self).SetCLRSyncManager)(windows_core::Interface::as_raw(self), pmanager.param().abi()).ok()
    }
    pub unsafe fn CreateCrst(&self) -> windows_core::Result<IHostCrst> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCrst)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCrstWithSpinCount(&self, dwspincount: u32) -> windows_core::Result<IHostCrst> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCrstWithSpinCount)(windows_core::Interface::as_raw(self), dwspincount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAutoEvent(&self) -> windows_core::Result<IHostAutoEvent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAutoEvent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateManualEvent<P0>(&self, binitialstate: P0) -> windows_core::Result<IHostManualEvent>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateManualEvent)(windows_core::Interface::as_raw(self), binitialstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateMonitorEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMonitorEvent)(windows_core::Interface::as_raw(self), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRWLockWriterEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRWLockWriterEvent)(windows_core::Interface::as_raw(self), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRWLockReaderEvent<P0>(&self, binitialstate: P0, cookie: usize) -> windows_core::Result<IHostManualEvent>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRWLockReaderEvent)(windows_core::Interface::as_raw(self), binitialstate.param().abi(), cookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSemaphoreA(&self, dwinitial: u32, dwmax: u32) -> windows_core::Result<IHostSemaphore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSemaphoreA)(windows_core::Interface::as_raw(self), dwinitial, dwmax, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IHostSyncManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCLRSyncManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCrst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCrstWithSpinCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAutoEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateManualEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMonitorEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRWLockWriterEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRWLockReaderEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSemaphoreA: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostTask, IHostTask_Vtbl, 0xc2275828_c4b1_4b55_82c9_92135f74df1a);
impl core::ops::Deref for IHostTask {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostTask, windows_core::IUnknown);
impl IHostTask {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Alert(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Alert)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Join(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Join)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn SetPriority(&self, newpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), newpriority).ok()
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCLRTask<P0>(&self, pclrtask: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRTask>,
    {
        (windows_core::Interface::vtable(self).SetCLRTask)(windows_core::Interface::as_raw(self), pclrtask.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IHostTask_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Alert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Join: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCLRTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostTaskManager, IHostTaskManager_Vtbl, 0x997ff24c_43b7_4352_8667_0dc04fafd354);
impl core::ops::Deref for IHostTaskManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostTaskManager, windows_core::IUnknown);
impl IHostTaskManager {
    pub unsafe fn GetCurrentTask(&self) -> windows_core::Result<IHostTask> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentTask)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn CreateTask(&self, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const core::ffi::c_void) -> windows_core::Result<IHostTask> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTask)(windows_core::Interface::as_raw(self), dwstacksize, pstartaddress, pparameter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Sleep(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Sleep)(windows_core::Interface::as_raw(self), dwmilliseconds, option).ok()
    }
    pub unsafe fn SwitchToTask(&self, option: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SwitchToTask)(windows_core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUILocale)(windows_core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLocale(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocale)(windows_core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn CallNeedsHostHook(&self, target: usize) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallNeedsHostHook)(windows_core::Interface::as_raw(self), target, &mut result__).map(|| result__)
    }
    pub unsafe fn LeaveRuntime(&self, target: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LeaveRuntime)(windows_core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn EnterRuntime(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnterRuntime)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReverseLeaveRuntime(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReverseLeaveRuntime)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReverseEnterRuntime(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReverseEnterRuntime)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginDelayAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginDelayAbort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndDelayAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndDelayAbort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginThreadAffinity(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginThreadAffinity)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndThreadAffinity(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndThreadAffinity)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStackGuarantee(&self, guarantee: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStackGuarantee)(windows_core::Interface::as_raw(self), guarantee).ok()
    }
    pub unsafe fn GetStackGuarantee(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStackGuarantee)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCLRTaskManager<P0>(&self, ppmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICLRTaskManager>,
    {
        (windows_core::Interface::vtable(self).SetCLRTaskManager)(windows_core::Interface::as_raw(self), ppmanager.param().abi()).ok()
    }
}
#[repr(C)]
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
    pub CallNeedsHostHook: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
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
windows_core::imp::define_interface!(IHostThreadpoolManager, IHostThreadpoolManager_Vtbl, 0x983d50e2_cb15_466b_80fc_845dc6e8c5fd);
impl core::ops::Deref for IHostThreadpoolManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostThreadpoolManager, windows_core::IUnknown);
impl IHostThreadpoolManager {
    #[cfg(feature = "Win32_System_Threading")]
    pub unsafe fn QueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueueUserWorkItem)(windows_core::Interface::as_raw(self), function, context, flags).ok()
    }
    pub unsafe fn SetMaxThreads(&self, dwmaxworkerthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxThreads)(windows_core::Interface::as_raw(self), dwmaxworkerthreads).ok()
    }
    pub unsafe fn GetMaxThreads(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAvailableThreads(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAvailableThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinThreads)(windows_core::Interface::as_raw(self), dwminiocompletionthreads).ok()
    }
    pub unsafe fn GetMinThreads(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinThreads)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IManagedObject, IManagedObject_Vtbl, 0xc3fcc19e_a970_11d2_8b5a_00a0c9b7c9c4);
impl core::ops::Deref for IManagedObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IManagedObject, windows_core::IUnknown);
impl IManagedObject {
    pub unsafe fn GetSerializedBuffer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSerializedBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetObjectIdentity(&self, pbstrguid: *mut windows_core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectIdentity)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrguid), appdomainid, pccw).ok()
    }
}
#[repr(C)]
pub struct IManagedObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSerializedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetObjectIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectHandle, IObjectHandle_Vtbl, 0xc460e2b4_e199_412a_8456_84dc3e4838c3);
impl core::ops::Deref for IObjectHandle {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectHandle, windows_core::IUnknown);
impl IObjectHandle {
    pub unsafe fn Unwrap(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Unwrap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IObjectHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unwrap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeName, ITypeName_Vtbl, 0xb81ff171_20f3_11d2_8dcc_00a0c9b00522);
impl core::ops::Deref for ITypeName {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeName, windows_core::IUnknown);
impl ITypeName {
    pub unsafe fn GetNameCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNames(&self, count: u32, rgbsznames: *mut windows_core::BSTR) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), count, core::mem::transmute(rgbsznames), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeArgumentCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeArgumentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeArguments(&self, count: u32, rgparguments: *mut Option<ITypeName>) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeArguments)(windows_core::Interface::as_raw(self), count, core::mem::transmute(rgparguments), &mut result__).map(|| result__)
    }
    pub unsafe fn GetModifierLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModifierLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetModifiers(&self, count: u32, rgmodifiers: *mut u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModifiers)(windows_core::Interface::as_raw(self), count, rgmodifiers, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAssemblyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAssemblyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITypeName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
    pub GetTypeArgumentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTypeArguments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetModifierLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetAssemblyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeNameBuilder, ITypeNameBuilder_Vtbl, 0xb81ff171_20f3_11d2_8dcc_00a0c9b00523);
impl core::ops::Deref for ITypeNameBuilder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeNameBuilder, windows_core::IUnknown);
impl ITypeNameBuilder {
    pub unsafe fn OpenGenericArguments(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenGenericArguments)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseGenericArguments(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseGenericArguments)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OpenGenericArgument(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenGenericArgument)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseGenericArgument(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseGenericArgument)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddName<P0>(&self, szname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddName)(windows_core::Interface::as_raw(self), szname.param().abi()).ok()
    }
    pub unsafe fn AddPointer(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPointer)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddByRef(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddByRef)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddSzArray(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddSzArray)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddArray(&self, rank: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddArray)(windows_core::Interface::as_raw(self), rank).ok()
    }
    pub unsafe fn AddAssemblySpec<P0>(&self, szassemblyspec: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddAssemblySpec)(windows_core::Interface::as_raw(self), szassemblyspec.param().abi()).ok()
    }
    pub unsafe fn ToString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ToString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
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
    pub ToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeNameFactory, ITypeNameFactory_Vtbl, 0xb81ff171_20f3_11d2_8dcc_00a0c9b00521);
impl core::ops::Deref for ITypeNameFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeNameFactory, windows_core::IUnknown);
impl ITypeNameFactory {
    pub unsafe fn ParseTypeName<P0>(&self, szname: P0, perror: *mut u32) -> windows_core::Result<ITypeName>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParseTypeName)(windows_core::Interface::as_raw(self), szname.param().abi(), perror, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTypeNameBuilder(&self) -> windows_core::Result<ITypeNameBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeNameBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITypeNameFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ParseTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypeNameBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const APPDOMAIN_FORCE_TRIVIAL_WAIT_OPERATIONS: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(8i32);
pub const APPDOMAIN_SECURITY_DEFAULT: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(0i32);
pub const APPDOMAIN_SECURITY_FORBID_CROSSAD_REVERSE_PINVOKE: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(2i32);
pub const APPDOMAIN_SECURITY_SANDBOXED: APPDOMAIN_SECURITY_FLAGS = APPDOMAIN_SECURITY_FLAGS(1i32);
pub const BucketParamLength: u32 = 255u32;
pub const BucketParamsCount: u32 = 10u32;
pub const CLR_ASSEMBLY_BUILD_VERSION: u32 = 0u32;
pub const CLR_ASSEMBLY_IDENTITY_FLAGS_DEFAULT: ECLRAssemblyIdentityFlags = ECLRAssemblyIdentityFlags(0i32);
pub const CLR_ASSEMBLY_MAJOR_VERSION: u32 = 4u32;
pub const CLR_ASSEMBLY_MINOR_VERSION: u32 = 0u32;
pub const CLR_BUILD_VERSION: u32 = 22220u32;
pub const CLR_DEBUGGING_MANAGED_EVENT_DEBUGGER_LAUNCH: CLR_DEBUGGING_PROCESS_FLAGS = CLR_DEBUGGING_PROCESS_FLAGS(2i32);
pub const CLR_DEBUGGING_MANAGED_EVENT_PENDING: CLR_DEBUGGING_PROCESS_FLAGS = CLR_DEBUGGING_PROCESS_FLAGS(1i32);
pub const CLR_MAJOR_VERSION: u32 = 4u32;
pub const CLR_MINOR_VERSION: u32 = 0u32;
pub const CLSID_CLRDebugging: windows_core::GUID = windows_core::GUID::from_u128(0xbacc578d_fbdd_48a4_969f_02d932b74634);
pub const CLSID_CLRDebuggingLegacy: windows_core::GUID = windows_core::GUID::from_u128(0xdf8395b5_a4ba_450b_a77c_a9a47762c520);
pub const CLSID_CLRMetaHost: windows_core::GUID = windows_core::GUID::from_u128(0x9280188d_0e8e_4867_b30c_7fa83884e8de);
pub const CLSID_CLRMetaHostPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x2ebcd49a_1b47_4a61_b13a_4a03701e594b);
pub const CLSID_CLRProfiling: windows_core::GUID = windows_core::GUID::from_u128(0xbd097ed8_733e_43fe_8ed7_a95ff9a8448c);
pub const CLSID_CLRStrongName: windows_core::GUID = windows_core::GUID::from_u128(0xb79b0acd_f5cd_409b_b5a5_a16244610b92);
pub const CLSID_RESOLUTION_DEFAULT: CLSID_RESOLUTION_FLAGS = CLSID_RESOLUTION_FLAGS(0i32);
pub const CLSID_RESOLUTION_REGISTERED: CLSID_RESOLUTION_FLAGS = CLSID_RESOLUTION_FLAGS(1i32);
pub const COR_GC_COUNTS: COR_GC_STAT_TYPES = COR_GC_STAT_TYPES(1i32);
pub const COR_GC_MEMORYUSAGE: COR_GC_STAT_TYPES = COR_GC_STAT_TYPES(2i32);
pub const COR_GC_THREAD_HAS_PROMOTED_BYTES: COR_GC_THREAD_STATS_TYPES = COR_GC_THREAD_STATS_TYPES(1i32);
pub const DEPRECATED_CLR_API_MESG: windows_core::PCSTR = windows_core::s!("This API has been deprecated. Refer to https://go.microsoft.com/fwlink/?LinkId=143720 for more details.");
pub const DUMP_FLAVOR_CriticalCLRState: ECustomDumpFlavor = ECustomDumpFlavor(1i32);
pub const DUMP_FLAVOR_Default: ECustomDumpFlavor = ECustomDumpFlavor(0i32);
pub const DUMP_FLAVOR_Mini: ECustomDumpFlavor = ECustomDumpFlavor(0i32);
pub const DUMP_FLAVOR_NonHeapCLRState: ECustomDumpFlavor = ECustomDumpFlavor(2i32);
pub const DUMP_ITEM_None: ECustomDumpItemKind = ECustomDumpItemKind(0i32);
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
pub const HOST_APPLICATION_BINDING_POLICY: EHostApplicationPolicy = EHostApplicationPolicy(1i32);
pub const HOST_BINDING_POLICY_MODIFY_CHAIN: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(1i32);
pub const HOST_BINDING_POLICY_MODIFY_DEFAULT: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(0i32);
pub const HOST_BINDING_POLICY_MODIFY_MAX: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(3i32);
pub const HOST_BINDING_POLICY_MODIFY_REMOVE: EHostBindingPolicyModifyFlags = EHostBindingPolicyModifyFlags(2i32);
pub const HOST_TYPE_APPLAUNCH: HOST_TYPE = HOST_TYPE(1i32);
pub const HOST_TYPE_CORFLAG: HOST_TYPE = HOST_TYPE(2i32);
pub const HOST_TYPE_DEFAULT: HOST_TYPE = HOST_TYPE(0i32);
pub const InvalidBucketParamIndex: BucketParameterIndex = BucketParameterIndex(9i32);
pub const LIBID_mscoree: windows_core::GUID = windows_core::GUID::from_u128(0x5477469e_83b1_11d2_8b49_00a0c9b7c9c4);
pub const MALLOC_EXECUTABLE: MALLOC_TYPE = MALLOC_TYPE(2i32);
pub const MALLOC_THREADSAFE: MALLOC_TYPE = MALLOC_TYPE(1i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_FALSE: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(2i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_MASK: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(3i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_TRUE: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(1i32);
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_UNSET: METAHOST_CONFIG_FLAGS = METAHOST_CONFIG_FLAGS(0i32);
pub const METAHOST_POLICY_APPLY_UPGRADE_POLICY: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(8i32);
pub const METAHOST_POLICY_EMULATE_EXE_LAUNCH: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(16i32);
pub const METAHOST_POLICY_ENSURE_SKU_SUPPORTED: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(128i32);
pub const METAHOST_POLICY_HIGHCOMPAT: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(0i32);
pub const METAHOST_POLICY_IGNORE_ERROR_MODE: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(4096i32);
pub const METAHOST_POLICY_SHOW_ERROR_DIALOG: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(32i32);
pub const METAHOST_POLICY_USE_PROCESS_IMAGE_PATH: METAHOST_POLICY_FLAGS = METAHOST_POLICY_FLAGS(64i32);
pub const MaxClrEvent: EClrEvent = EClrEvent(4i32);
pub const MaxClrFailure: EClrFailure = EClrFailure(7i32);
pub const MaxClrOperation: EClrOperation = EClrOperation(7i32);
pub const MaxPolicyAction: EPolicyAction = EPolicyAction(10i32);
pub const OPR_AppDomainRudeUnload: EClrOperation = EClrOperation(4i32);
pub const OPR_AppDomainUnload: EClrOperation = EClrOperation(3i32);
pub const OPR_FinalizerRun: EClrOperation = EClrOperation(6i32);
pub const OPR_ProcessExit: EClrOperation = EClrOperation(5i32);
pub const OPR_ThreadAbort: EClrOperation = EClrOperation(0i32);
pub const OPR_ThreadRudeAbortInCriticalRegion: EClrOperation = EClrOperation(2i32);
pub const OPR_ThreadRudeAbortInNonCriticalRegion: EClrOperation = EClrOperation(1i32);
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
pub const RUNTIME_INFO_IGNORE_ERROR_MODE: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(4096i32);
pub const RUNTIME_INFO_REQUEST_AMD64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(4i32);
pub const RUNTIME_INFO_REQUEST_ARM64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(8192i32);
pub const RUNTIME_INFO_REQUEST_IA64: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(2i32);
pub const RUNTIME_INFO_REQUEST_X86: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(8i32);
pub const RUNTIME_INFO_UPGRADE_VERSION: RUNTIME_INFO_FLAGS = RUNTIME_INFO_FLAGS(1i32);
pub const SO_ClrEngine: StackOverflowType = StackOverflowType(1i32);
pub const SO_Managed: StackOverflowType = StackOverflowType(0i32);
pub const SO_Other: StackOverflowType = StackOverflowType(2i32);
pub const STARTUP_ALWAYSFLOW_IMPERSONATION: STARTUP_FLAGS = STARTUP_FLAGS(262144i32);
pub const STARTUP_ARM: STARTUP_FLAGS = STARTUP_FLAGS(4194304i32);
pub const STARTUP_CONCURRENT_GC: STARTUP_FLAGS = STARTUP_FLAGS(1i32);
pub const STARTUP_DISABLE_COMMITTHREADSTACK: STARTUP_FLAGS = STARTUP_FLAGS(131072i32);
pub const STARTUP_ETW: STARTUP_FLAGS = STARTUP_FLAGS(1048576i32);
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
pub const WAIT_ALERTABLE: WAIT_OPTION = WAIT_OPTION(2i32);
pub const WAIT_MSGPUMP: WAIT_OPTION = WAIT_OPTION(1i32);
pub const WAIT_NOTINDEADLOCK: WAIT_OPTION = WAIT_OPTION(4i32);
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPDOMAIN_SECURITY_FLAGS(pub i32);
impl windows_core::TypeKind for APPDOMAIN_SECURITY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPDOMAIN_SECURITY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPDOMAIN_SECURITY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BucketParameterIndex(pub i32);
impl windows_core::TypeKind for BucketParameterIndex {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BucketParameterIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BucketParameterIndex").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLR_DEBUGGING_PROCESS_FLAGS(pub i32);
impl windows_core::TypeKind for CLR_DEBUGGING_PROCESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLR_DEBUGGING_PROCESS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLR_DEBUGGING_PROCESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLSID_RESOLUTION_FLAGS(pub i32);
impl windows_core::TypeKind for CLSID_RESOLUTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLSID_RESOLUTION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLSID_RESOLUTION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_GC_STAT_TYPES(pub i32);
impl windows_core::TypeKind for COR_GC_STAT_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_GC_STAT_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_GC_STAT_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_GC_THREAD_STATS_TYPES(pub i32);
impl windows_core::TypeKind for COR_GC_THREAD_STATS_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_GC_THREAD_STATS_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_GC_THREAD_STATS_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EApiCategories(pub i32);
impl windows_core::TypeKind for EApiCategories {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EApiCategories {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EApiCategories").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EBindPolicyLevels(pub i32);
impl windows_core::TypeKind for EBindPolicyLevels {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EBindPolicyLevels {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EBindPolicyLevels").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ECLRAssemblyIdentityFlags(pub i32);
impl windows_core::TypeKind for ECLRAssemblyIdentityFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ECLRAssemblyIdentityFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ECLRAssemblyIdentityFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EClrEvent(pub i32);
impl windows_core::TypeKind for EClrEvent {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EClrEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EClrEvent").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EClrFailure(pub i32);
impl windows_core::TypeKind for EClrFailure {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EClrFailure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EClrFailure").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EClrOperation(pub i32);
impl windows_core::TypeKind for EClrOperation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EClrOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EClrOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EClrUnhandledException(pub i32);
impl windows_core::TypeKind for EClrUnhandledException {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EClrUnhandledException {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EClrUnhandledException").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EContextType(pub i32);
impl windows_core::TypeKind for EContextType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EContextType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EContextType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ECustomDumpFlavor(pub i32);
impl windows_core::TypeKind for ECustomDumpFlavor {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ECustomDumpFlavor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ECustomDumpFlavor").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ECustomDumpItemKind(pub i32);
impl windows_core::TypeKind for ECustomDumpItemKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ECustomDumpItemKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ECustomDumpItemKind").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EHostApplicationPolicy(pub i32);
impl windows_core::TypeKind for EHostApplicationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EHostApplicationPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EHostApplicationPolicy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EHostBindingPolicyModifyFlags(pub i32);
impl windows_core::TypeKind for EHostBindingPolicyModifyFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EHostBindingPolicyModifyFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EHostBindingPolicyModifyFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EInitializeNewDomainFlags(pub i32);
impl windows_core::TypeKind for EInitializeNewDomainFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EInitializeNewDomainFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EInitializeNewDomainFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EMemoryAvailable(pub i32);
impl windows_core::TypeKind for EMemoryAvailable {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EMemoryAvailable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EMemoryAvailable").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EMemoryCriticalLevel(pub i32);
impl windows_core::TypeKind for EMemoryCriticalLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EMemoryCriticalLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EMemoryCriticalLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EPolicyAction(pub i32);
impl windows_core::TypeKind for EPolicyAction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EPolicyAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EPolicyAction").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESymbolReadingPolicy(pub i32);
impl windows_core::TypeKind for ESymbolReadingPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESymbolReadingPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESymbolReadingPolicy").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ETaskType(pub i32);
impl windows_core::TypeKind for ETaskType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ETaskType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ETaskType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HOST_TYPE(pub i32);
impl windows_core::TypeKind for HOST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HOST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HOST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MALLOC_TYPE(pub i32);
impl windows_core::TypeKind for MALLOC_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MALLOC_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MALLOC_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct METAHOST_CONFIG_FLAGS(pub i32);
impl windows_core::TypeKind for METAHOST_CONFIG_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for METAHOST_CONFIG_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("METAHOST_CONFIG_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct METAHOST_POLICY_FLAGS(pub i32);
impl windows_core::TypeKind for METAHOST_POLICY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for METAHOST_POLICY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("METAHOST_POLICY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RUNTIME_INFO_FLAGS(pub i32);
impl windows_core::TypeKind for RUNTIME_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RUNTIME_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RUNTIME_INFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STARTUP_FLAGS(pub i32);
impl windows_core::TypeKind for STARTUP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STARTUP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STARTUP_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StackOverflowType(pub i32);
impl windows_core::TypeKind for StackOverflowType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StackOverflowType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StackOverflowType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WAIT_OPTION(pub i32);
impl windows_core::TypeKind for WAIT_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WAIT_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WAIT_OPTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssemblyBindInfo {
    pub dwAppDomainId: u32,
    pub lpReferencedIdentity: windows_core::PCWSTR,
    pub lpPostPolicyIdentity: windows_core::PCWSTR,
    pub ePolicyLevel: u32,
}
impl windows_core::TypeKind for AssemblyBindInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for AssemblyBindInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BucketParameters {
    pub fInited: super::super::Foundation::BOOL,
    pub pszEventTypeName: [u16; 255],
    pub pszParams: [u16; 2550],
}
impl windows_core::TypeKind for BucketParameters {
    type TypeKind = windows_core::CopyType;
}
impl Default for BucketParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLRRuntimeHost: windows_core::GUID = windows_core::GUID::from_u128(0x90f1a06e_7712_4762_86b5_7a5eba6bdb02);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLR_DEBUGGING_VERSION {
    pub wStructVersion: u16,
    pub wMajor: u16,
    pub wMinor: u16,
    pub wBuild: u16,
    pub wRevision: u16,
}
impl windows_core::TypeKind for CLR_DEBUGGING_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLR_DEBUGGING_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for COR_GC_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_GC_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_GC_THREAD_STATS {
    pub PerThreadAllocation: u64,
    pub Flags: u32,
}
impl windows_core::TypeKind for COR_GC_THREAD_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_GC_THREAD_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ComCallUnmarshal: windows_core::GUID = windows_core::GUID::from_u128(0x3f281000_e95a_11d2_886b_00c04f869f04);
pub const ComCallUnmarshalV4: windows_core::GUID = windows_core::GUID::from_u128(0x45fb4600_e6e8_4928_b25e_50476ff79425);
pub const CorRuntimeHost: windows_core::GUID = windows_core::GUID::from_u128(0xcb2f6723_ab3a_11d2_9c40_00c04fa30a3e);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CustomDumpItem {
    pub itemKind: ECustomDumpItemKind,
    pub Anonymous: CustomDumpItem_0,
}
impl windows_core::TypeKind for CustomDumpItem {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for CustomDumpItem_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CustomDumpItem_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MDAInfo {
    pub lpMDACaption: windows_core::PCWSTR,
    pub lpMDAMessage: windows_core::PCWSTR,
    pub lpStackTrace: windows_core::PCWSTR,
}
impl windows_core::TypeKind for MDAInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for MDAInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleBindInfo {
    pub dwAppDomainId: u32,
    pub lpAssemblyIdentity: windows_core::PCWSTR,
    pub lpModuleName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for ModuleBindInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for ModuleBindInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StackOverflowInfo {
    pub soType: StackOverflowType,
    pub pExceptionInfo: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for StackOverflowInfo {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl Default for StackOverflowInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TypeNameFactory: windows_core::GUID = windows_core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b00525);
pub type CLRCreateInstanceFnPtr = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type CallbackThreadSetFnPtr = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type CallbackThreadUnsetFnPtr = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type CreateInterfaceFnPtr = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FExecuteInAppDomainCallback = Option<unsafe extern "system" fn(cookie: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FLockClrVersionCallback = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type PTLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(__midl____midl_itf_mscoree_0000_00040005: *mut core::ffi::c_void)>;
pub type RuntimeLoadedCallbackFnPtr = Option<unsafe extern "system" fn(pruntimeinfo: Option<ICLRRuntimeInfo>, pfncallbackthreadset: CallbackThreadSetFnPtr, pfncallbackthreadunset: CallbackThreadUnsetFnPtr)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
