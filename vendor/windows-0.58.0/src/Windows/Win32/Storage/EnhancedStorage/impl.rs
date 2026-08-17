pub trait IEnhancedStorageACT_Impl: Sized {
    fn Authorize(&self, hwndparent: u32, dwflags: u32) -> windows_core::Result<()>;
    fn Unauthorize(&self) -> windows_core::Result<()>;
    fn GetAuthorizationState(&self) -> windows_core::Result<ACT_AUTHORIZATION_STATE>;
    fn GetMatchingVolume(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetUniqueIdentity(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnhancedStorageACT {}
impl IEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnhancedStorageACT_Vtbl
    where
        Identity: IEnhancedStorageACT_Impl,
    {
        unsafe extern "system" fn Authorize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnhancedStorageACT_Impl::Authorize(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Unauthorize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnhancedStorageACT_Impl::Unauthorize(this).into()
        }
        unsafe extern "system" fn GetAuthorizationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageACT_Impl::GetAuthorizationState(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszvolume: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageACT_Impl::GetMatchingVolume(this) {
                Ok(ok__) => {
                    ppwszvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszidentity: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageACT_Impl::GetUniqueIdentity(this) {
                Ok(ok__) => {
                    ppwszidentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSilos<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppienhancedstoragesilos: *mut *mut Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnhancedStorageACT_Impl::GetSilos(this, core::mem::transmute_copy(&pppienhancedstoragesilos), core::mem::transmute_copy(&pcenhancedstoragesilos)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Authorize: Authorize::<Identity, OFFSET>,
            Unauthorize: Unauthorize::<Identity, OFFSET>,
            GetAuthorizationState: GetAuthorizationState::<Identity, OFFSET>,
            GetMatchingVolume: GetMatchingVolume::<Identity, OFFSET>,
            GetUniqueIdentity: GetUniqueIdentity::<Identity, OFFSET>,
            GetSilos: GetSilos::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageACT as windows_core::Interface>::IID
    }
}
pub trait IEnhancedStorageACT2_Impl: Sized + IEnhancedStorageACT_Impl {
    fn GetDeviceName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsDeviceRemovable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IEnhancedStorageACT2 {}
impl IEnhancedStorageACT2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnhancedStorageACT2_Vtbl
    where
        Identity: IEnhancedStorageACT2_Impl,
    {
        unsafe extern "system" fn GetDeviceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszdevicename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageACT2_Impl::GetDeviceName(this) {
                Ok(ok__) => {
                    ppwszdevicename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeviceRemovable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageACT2_Impl::IsDeviceRemovable(this) {
                Ok(ok__) => {
                    pisdeviceremovable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IEnhancedStorageACT_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceName: GetDeviceName::<Identity, OFFSET>,
            IsDeviceRemovable: IsDeviceRemovable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageACT2 as windows_core::Interface>::IID || iid == &<IEnhancedStorageACT as windows_core::Interface>::IID
    }
}
pub trait IEnhancedStorageACT3_Impl: Sized + IEnhancedStorageACT2_Impl {
    fn UnauthorizeEx(&self, dwflags: u32) -> windows_core::Result<()>;
    fn IsQueueFrozen(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetShellExtSupport(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IEnhancedStorageACT3 {}
impl IEnhancedStorageACT3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnhancedStorageACT3_Vtbl
    where
        Identity: IEnhancedStorageACT3_Impl,
    {
        unsafe extern "system" fn UnauthorizeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnhancedStorageACT3_Impl::UnauthorizeEx(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn IsQueueFrozen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageACT3_Impl::IsQueueFrozen(this) {
                Ok(ok__) => {
                    pisqueuefrozen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShellExtSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshellextsupport: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageACT3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageACT3_Impl::GetShellExtSupport(this) {
                Ok(ok__) => {
                    pshellextsupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IEnhancedStorageACT2_Vtbl::new::<Identity, OFFSET>(),
            UnauthorizeEx: UnauthorizeEx::<Identity, OFFSET>,
            IsQueueFrozen: IsQueueFrozen::<Identity, OFFSET>,
            GetShellExtSupport: GetShellExtSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageACT3 as windows_core::Interface>::IID || iid == &<IEnhancedStorageACT as windows_core::Interface>::IID || iid == &<IEnhancedStorageACT2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait IEnhancedStorageSilo_Impl: Sized {
    fn GetInfo(&self) -> windows_core::Result<SILO_INFO>;
    fn GetActions(&self, pppienhancedstoragesiloactions: *mut *mut Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> windows_core::Result<()>;
    fn SendCommand(&self, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> windows_core::Result<()>;
    fn GetPortableDevice(&self) -> windows_core::Result<super::super::Devices::PortableDevices::IPortableDevice>;
    fn GetDevicePath(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl windows_core::RuntimeName for IEnhancedStorageSilo {}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl IEnhancedStorageSilo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnhancedStorageSilo_Vtbl
    where
        Identity: IEnhancedStorageSilo_Impl,
    {
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSilo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageSilo_Impl::GetInfo(this) {
                Ok(ok__) => {
                    psiloinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSilo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnhancedStorageSilo_Impl::GetActions(this, core::mem::transmute_copy(&pppienhancedstoragesiloactions), core::mem::transmute_copy(&pcenhancedstoragesiloactions)).into()
        }
        unsafe extern "system" fn SendCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSilo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnhancedStorageSilo_Impl::SendCommand(this, core::mem::transmute_copy(&command), core::mem::transmute_copy(&pbcommandbuffer), core::mem::transmute_copy(&cbcommandbuffer), core::mem::transmute_copy(&pbresponsebuffer), core::mem::transmute_copy(&pcbresponsebuffer)).into()
        }
        unsafe extern "system" fn GetPortableDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiportabledevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSilo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageSilo_Impl::GetPortableDevice(this) {
                Ok(ok__) => {
                    ppiportabledevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszsilodevicepath: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSilo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageSilo_Impl::GetDevicePath(this) {
                Ok(ok__) => {
                    ppwszsilodevicepath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetActions: GetActions::<Identity, OFFSET>,
            SendCommand: SendCommand::<Identity, OFFSET>,
            GetPortableDevice: GetPortableDevice::<Identity, OFFSET>,
            GetDevicePath: GetDevicePath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageSilo as windows_core::Interface>::IID
    }
}
pub trait IEnhancedStorageSiloAction_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnhancedStorageSiloAction {}
impl IEnhancedStorageSiloAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnhancedStorageSiloAction_Vtbl
    where
        Identity: IEnhancedStorageSiloAction_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszactionname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSiloAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageSiloAction_Impl::GetName(this) {
                Ok(ok__) => {
                    ppwszactionname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszactiondescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSiloAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnhancedStorageSiloAction_Impl::GetDescription(this) {
                Ok(ok__) => {
                    ppwszactiondescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnhancedStorageSiloAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnhancedStorageSiloAction_Impl::Invoke(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageSiloAction as windows_core::Interface>::IID
    }
}
pub trait IEnumEnhancedStorageACT_Impl: Sized {
    fn GetACTs(&self, pppienhancedstorageacts: *mut *mut Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> windows_core::Result<()>;
    fn GetMatchingACT(&self, szvolume: &windows_core::PCWSTR) -> windows_core::Result<IEnhancedStorageACT>;
}
impl windows_core::RuntimeName for IEnumEnhancedStorageACT {}
impl IEnumEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumEnhancedStorageACT_Vtbl
    where
        Identity: IEnumEnhancedStorageACT_Impl,
    {
        unsafe extern "system" fn GetACTs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppienhancedstorageacts: *mut *mut Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumEnhancedStorageACT_Impl::GetACTs(this, core::mem::transmute_copy(&pppienhancedstorageacts), core::mem::transmute_copy(&pcenhancedstorageacts)).into()
        }
        unsafe extern "system" fn GetMatchingACT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szvolume: windows_core::PCWSTR, ppienhancedstorageact: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumEnhancedStorageACT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumEnhancedStorageACT_Impl::GetMatchingACT(this, core::mem::transmute(&szvolume)) {
                Ok(ok__) => {
                    ppienhancedstorageact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetACTs: GetACTs::<Identity, OFFSET>,
            GetMatchingACT: GetMatchingACT::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumEnhancedStorageACT as windows_core::Interface>::IID
    }
}
