pub trait IMediaProtectionServiceRequest_Impl: Sized {
    fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID>;
    fn Type(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IMediaProtectionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionServiceRequest";
}
impl IMediaProtectionServiceRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaProtectionServiceRequest_Vtbl
    where
        Identity: IMediaProtectionServiceRequest_Impl,
    {
        unsafe extern "system" fn ProtectionSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMediaProtectionServiceRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaProtectionServiceRequest_Impl::ProtectionSystem(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMediaProtectionServiceRequest_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaProtectionServiceRequest_Impl::Type(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaProtectionServiceRequest, OFFSET>(),
            ProtectionSystem: ProtectionSystem::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaProtectionServiceRequest as windows_core::Interface>::IID
    }
}
