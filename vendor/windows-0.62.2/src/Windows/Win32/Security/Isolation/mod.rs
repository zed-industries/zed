#[inline]
pub unsafe fn CreateAppContainerProfile<P0, P1, P2>(pszappcontainername: P0, pszdisplayname: P1, pszdescription: P2, pcapabilities: Option<&[super::SID_AND_ATTRIBUTES]>) -> windows_core::Result<super::PSID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : windows_core::PCWSTR, pszdisplayname : windows_core::PCWSTR, pszdescription : windows_core::PCWSTR, pcapabilities : *const super:: SID_AND_ATTRIBUTES, dwcapabilitycount : u32, ppsidappcontainersid : *mut super:: PSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateAppContainerProfile(pszappcontainername.param().abi(), pszdisplayname.param().abi(), pszdescription.param().abi(), core::mem::transmute(pcapabilities.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcapabilities.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DeleteAppContainerProfile<P0>(pszappcontainername: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeleteAppContainerProfile(pszappcontainername : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeleteAppContainerProfile(pszappcontainername.param().abi()).ok() }
}
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<P0>(pszappcontainername: P0) -> windows_core::Result<super::PSID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : windows_core::PCWSTR, ppsidappcontainersid : *mut super:: PSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DeriveAppContainerSidFromAppContainerName(pszappcontainername.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<P1>(psidappcontainersid: super::PSID, pszrestrictedappcontainername: P1) -> windows_core::Result<super::PSID>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super:: PSID, pszrestrictedappcontainername : windows_core::PCWSTR, ppsidrestrictedappcontainersid : *mut super:: PSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid, pszrestrictedappcontainername.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetAppContainerFolderPath<P0>(pszappcontainersid: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppContainerFolderPath(pszappcontainersid : windows_core::PCWSTR, ppszpath : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetAppContainerFolderPath(pszappcontainersid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath(token: Option<super::super::Foundation::HANDLE>, appcontainersid: Option<super::PSID>, objectpath: Option<&mut [u16]>, returnlength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::super::Foundation:: HANDLE, appcontainersid : super:: PSID, objectpathlength : u32, objectpath : windows_core::PWSTR, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetAppContainerNamedObjectPath(token.unwrap_or(core::mem::zeroed()) as _, appcontainersid.unwrap_or(core::mem::zeroed()) as _, objectpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(objectpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), returnlength as _).ok() }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : u32, phappcontainerkey : *mut super::super::System::Registry:: HKEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetAppContainerRegistryLocation(desiredaccess, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IsCrossIsolatedEnvironmentClipboardContent() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsCrossIsolatedEnvironmentClipboardContent(iscrossisolatedenvironmentclipboardcontent : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IsCrossIsolatedEnvironmentClipboardContent(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("api-ms-win-security-isolatedcontainer-l1-1-0.dll" "system" fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IsProcessInIsolatedContainer(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IsProcessInIsolatedWindowsEnvironment(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const core::ffi::c_void) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("api-ms-win-security-isolatedcontainer-l1-1-1.dll" "system" fn IsProcessInWDAGContainer(reserved : *const core::ffi::c_void, isprocessinwdagcontainer : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IsProcessInWDAGContainer(reserved, &mut result__).map(|| result__)
    }
}
windows_core::imp::define_interface!(IIsolatedAppLauncher, IIsolatedAppLauncher_Vtbl, 0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
windows_core::imp::interface_hierarchy!(IIsolatedAppLauncher, windows_core::IUnknown);
impl IIsolatedAppLauncher {
    pub unsafe fn Launch<P0, P1>(&self, appusermodelid: P0, arguments: P1, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Launch)(windows_core::Interface::as_raw(self), appusermodelid.param().abi(), arguments.param().abi(), telemetryparameters).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Launch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const IsolatedAppLauncherTelemetryParameters) -> windows_core::HRESULT,
}
pub trait IIsolatedAppLauncher_Impl: windows_core::IUnknownImpl {
    fn Launch(&self, appusermodelid: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> windows_core::Result<()>;
}
impl IIsolatedAppLauncher_Vtbl {
    pub const fn new<Identity: IIsolatedAppLauncher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Launch<Identity: IIsolatedAppLauncher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appusermodelid: windows_core::PCWSTR, arguments: windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsolatedAppLauncher_Impl::Launch(this, core::mem::transmute(&appusermodelid), core::mem::transmute(&arguments), core::mem::transmute_copy(&telemetryparameters)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Launch: Launch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsolatedAppLauncher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IIsolatedAppLauncher {}
windows_core::imp::define_interface!(IIsolatedProcessLauncher, IIsolatedProcessLauncher_Vtbl, 0x1aa24232_9a91_4201_88cb_122f9d6522e0);
windows_core::imp::interface_hierarchy!(IIsolatedProcessLauncher, windows_core::IUnknown);
impl IIsolatedProcessLauncher {
    pub unsafe fn LaunchProcess<P0, P1, P2>(&self, process: P0, arguments: P1, workingdirectory: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LaunchProcess)(windows_core::Interface::as_raw(self), process.param().abi(), arguments.param().abi(), workingdirectory.param().abi()).ok() }
    }
    pub unsafe fn ShareDirectory<P0, P1>(&self, hostpath: P0, containerpath: P1, readonly: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShareDirectory)(windows_core::Interface::as_raw(self), hostpath.param().abi(), containerpath.param().abi(), readonly.into()).ok() }
    }
    pub unsafe fn GetContainerGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AllowSetForegroundAccess(&self, pid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllowSetForegroundAccess)(windows_core::Interface::as_raw(self), pid).ok() }
    }
    pub unsafe fn IsContainerRunning(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsContainerRunning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedProcessLauncher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LaunchProcess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ShareDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetContainerGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AllowSetForegroundAccess: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsContainerRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IIsolatedProcessLauncher_Impl: windows_core::IUnknownImpl {
    fn LaunchProcess(&self, process: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, workingdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShareDirectory(&self, hostpath: &windows_core::PCWSTR, containerpath: &windows_core::PCWSTR, readonly: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetContainerGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn AllowSetForegroundAccess(&self, pid: u32) -> windows_core::Result<()>;
    fn IsContainerRunning(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IIsolatedProcessLauncher_Vtbl {
    pub const fn new<Identity: IIsolatedProcessLauncher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LaunchProcess<Identity: IIsolatedProcessLauncher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: windows_core::PCWSTR, arguments: windows_core::PCWSTR, workingdirectory: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsolatedProcessLauncher_Impl::LaunchProcess(this, core::mem::transmute(&process), core::mem::transmute(&arguments), core::mem::transmute(&workingdirectory)).into()
            }
        }
        unsafe extern "system" fn ShareDirectory<Identity: IIsolatedProcessLauncher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostpath: windows_core::PCWSTR, containerpath: windows_core::PCWSTR, readonly: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsolatedProcessLauncher_Impl::ShareDirectory(this, core::mem::transmute(&hostpath), core::mem::transmute(&containerpath), core::mem::transmute_copy(&readonly)).into()
            }
        }
        unsafe extern "system" fn GetContainerGuid<Identity: IIsolatedProcessLauncher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsolatedProcessLauncher_Impl::GetContainerGuid(this) {
                    Ok(ok__) => {
                        guid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllowSetForegroundAccess<Identity: IIsolatedProcessLauncher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsolatedProcessLauncher_Impl::AllowSetForegroundAccess(this, core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn IsContainerRunning<Identity: IIsolatedProcessLauncher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, running: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsolatedProcessLauncher_Impl::IsContainerRunning(this) {
                    Ok(ok__) => {
                        running.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LaunchProcess: LaunchProcess::<Identity, OFFSET>,
            ShareDirectory: ShareDirectory::<Identity, OFFSET>,
            GetContainerGuid: GetContainerGuid::<Identity, OFFSET>,
            AllowSetForegroundAccess: AllowSetForegroundAccess::<Identity, OFFSET>,
            IsContainerRunning: IsContainerRunning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsolatedProcessLauncher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IIsolatedProcessLauncher {}
windows_core::imp::define_interface!(IIsolatedProcessLauncher2, IIsolatedProcessLauncher2_Vtbl, 0x780e4416_5e72_4123_808e_66dc6479feef);
impl core::ops::Deref for IIsolatedProcessLauncher2 {
    type Target = IIsolatedProcessLauncher;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IIsolatedProcessLauncher2, windows_core::IUnknown, IIsolatedProcessLauncher);
impl IIsolatedProcessLauncher2 {
    pub unsafe fn LaunchProcess2<P0, P1, P2>(&self, process: P0, arguments: P1, workingdirectory: P2, correlationguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LaunchProcess2)(windows_core::Interface::as_raw(self), process.param().abi(), arguments.param().abi(), workingdirectory.param().abi(), correlationguid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedProcessLauncher2_Vtbl {
    pub base__: IIsolatedProcessLauncher_Vtbl,
    pub LaunchProcess2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IIsolatedProcessLauncher2_Impl: IIsolatedProcessLauncher_Impl {
    fn LaunchProcess2(&self, process: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, workingdirectory: &windows_core::PCWSTR, correlationguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IIsolatedProcessLauncher2_Vtbl {
    pub const fn new<Identity: IIsolatedProcessLauncher2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LaunchProcess2<Identity: IIsolatedProcessLauncher2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: windows_core::PCWSTR, arguments: windows_core::PCWSTR, workingdirectory: windows_core::PCWSTR, correlationguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsolatedProcessLauncher2_Impl::LaunchProcess2(this, core::mem::transmute(&process), core::mem::transmute(&arguments), core::mem::transmute(&workingdirectory), core::mem::transmute_copy(&correlationguid)).into()
            }
        }
        Self { base__: IIsolatedProcessLauncher_Vtbl::new::<Identity, OFFSET>(), LaunchProcess2: LaunchProcess2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsolatedProcessLauncher2 as windows_core::Interface>::IID || iid == &<IIsolatedProcessLauncher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IIsolatedProcessLauncher2 {}
pub const IsolatedAppLauncher: windows_core::GUID = windows_core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: windows_core::BOOL,
    pub CorrelationGUID: windows_core::GUID,
}
pub const WDAG_CLIPBOARD_TAG: windows_core::PCWSTR = windows_core::w!("CrossIsolatedEnvironmentContent");
