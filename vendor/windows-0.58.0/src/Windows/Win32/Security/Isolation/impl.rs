pub trait IIsolatedAppLauncher_Impl: Sized {
    fn Launch(&self, appusermodelid: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IIsolatedAppLauncher {}
impl IIsolatedAppLauncher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIsolatedAppLauncher_Vtbl
    where
        Identity: IIsolatedAppLauncher_Impl,
    {
        unsafe extern "system" fn Launch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appusermodelid: windows_core::PCWSTR, arguments: windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> windows_core::HRESULT
        where
            Identity: IIsolatedAppLauncher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsolatedAppLauncher_Impl::Launch(this, core::mem::transmute(&appusermodelid), core::mem::transmute(&arguments), core::mem::transmute_copy(&telemetryparameters)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Launch: Launch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsolatedAppLauncher as windows_core::Interface>::IID
    }
}
pub trait IIsolatedProcessLauncher_Impl: Sized {
    fn LaunchProcess(&self, process: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, workingdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ShareDirectory(&self, hostpath: &windows_core::PCWSTR, containerpath: &windows_core::PCWSTR, readonly: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetContainerGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn AllowSetForegroundAccess(&self, pid: u32) -> windows_core::Result<()>;
    fn IsContainerRunning(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IIsolatedProcessLauncher {}
impl IIsolatedProcessLauncher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIsolatedProcessLauncher_Vtbl
    where
        Identity: IIsolatedProcessLauncher_Impl,
    {
        unsafe extern "system" fn LaunchProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: windows_core::PCWSTR, arguments: windows_core::PCWSTR, workingdirectory: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IIsolatedProcessLauncher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsolatedProcessLauncher_Impl::LaunchProcess(this, core::mem::transmute(&process), core::mem::transmute(&arguments), core::mem::transmute(&workingdirectory)).into()
        }
        unsafe extern "system" fn ShareDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostpath: windows_core::PCWSTR, containerpath: windows_core::PCWSTR, readonly: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IIsolatedProcessLauncher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsolatedProcessLauncher_Impl::ShareDirectory(this, core::mem::transmute(&hostpath), core::mem::transmute(&containerpath), core::mem::transmute_copy(&readonly)).into()
        }
        unsafe extern "system" fn GetContainerGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IIsolatedProcessLauncher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIsolatedProcessLauncher_Impl::GetContainerGuid(this) {
                Ok(ok__) => {
                    guid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSetForegroundAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: u32) -> windows_core::HRESULT
        where
            Identity: IIsolatedProcessLauncher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsolatedProcessLauncher_Impl::AllowSetForegroundAccess(this, core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn IsContainerRunning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, running: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IIsolatedProcessLauncher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIsolatedProcessLauncher_Impl::IsContainerRunning(this) {
                Ok(ok__) => {
                    running.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IIsolatedProcessLauncher2_Impl: Sized + IIsolatedProcessLauncher_Impl {
    fn LaunchProcess2(&self, process: &windows_core::PCWSTR, arguments: &windows_core::PCWSTR, workingdirectory: &windows_core::PCWSTR, correlationguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IIsolatedProcessLauncher2 {}
impl IIsolatedProcessLauncher2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIsolatedProcessLauncher2_Vtbl
    where
        Identity: IIsolatedProcessLauncher2_Impl,
    {
        unsafe extern "system" fn LaunchProcess2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: windows_core::PCWSTR, arguments: windows_core::PCWSTR, workingdirectory: windows_core::PCWSTR, correlationguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IIsolatedProcessLauncher2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsolatedProcessLauncher2_Impl::LaunchProcess2(this, core::mem::transmute(&process), core::mem::transmute(&arguments), core::mem::transmute(&workingdirectory), core::mem::transmute_copy(&correlationguid)).into()
        }
        Self { base__: IIsolatedProcessLauncher_Vtbl::new::<Identity, OFFSET>(), LaunchProcess2: LaunchProcess2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsolatedProcessLauncher2 as windows_core::Interface>::IID || iid == &<IIsolatedProcessLauncher as windows_core::Interface>::IID
    }
}
