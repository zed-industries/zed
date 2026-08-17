pub trait IIOControlCode_Impl: Sized {
    fn AccessMode(&self) -> windows_core::Result<IOControlAccessMode>;
    fn BufferingMethod(&self) -> windows_core::Result<IOControlBufferingMethod>;
    fn Function(&self) -> windows_core::Result<u16>;
    fn DeviceType(&self) -> windows_core::Result<u16>;
    fn ControlCode(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IIOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IIOControlCode";
}
impl IIOControlCode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIOControlCode_Vtbl
    where
        Identity: IIOControlCode_Impl,
    {
        unsafe extern "system" fn AccessMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut IOControlAccessMode) -> windows_core::HRESULT
        where
            Identity: IIOControlCode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIOControlCode_Impl::AccessMode(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferingMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> windows_core::HRESULT
        where
            Identity: IIOControlCode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIOControlCode_Impl::BufferingMethod(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Function<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT
        where
            Identity: IIOControlCode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIOControlCode_Impl::Function(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT
        where
            Identity: IIOControlCode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIOControlCode_Impl::DeviceType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IIOControlCode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIOControlCode_Impl::ControlCode(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIOControlCode, OFFSET>(),
            AccessMode: AccessMode::<Identity, OFFSET>,
            BufferingMethod: BufferingMethod::<Identity, OFFSET>,
            Function: Function::<Identity, OFFSET>,
            DeviceType: DeviceType::<Identity, OFFSET>,
            ControlCode: ControlCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIOControlCode as windows_core::Interface>::IID
    }
}
