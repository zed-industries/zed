#[cfg(feature = "Win32_Security")]
pub trait IDisplayDeviceInterop_Impl: Sized {
    fn CreateSharedHandle(&self, pobject: Option<&windows_core::IInspectable>, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(&self, nthandle: super::super::super::Foundation::HANDLE, riid: &windows_core::GUID) -> windows_core::Result<*mut core::ffi::c_void>;
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for IDisplayDeviceInterop {}
#[cfg(feature = "Win32_Security")]
impl IDisplayDeviceInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDisplayDeviceInterop_Vtbl
    where
        Identity: IDisplayDeviceInterop_Impl,
    {
        unsafe extern "system" fn CreateSharedHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: core::mem::MaybeUninit<windows_core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDisplayDeviceInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDisplayDeviceInterop_Impl::CreateSharedHandle(this, windows_core::from_raw_borrowed(&pobject), core::mem::transmute_copy(&psecurityattributes), core::mem::transmute_copy(&access), core::mem::transmute(&name)) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDisplayDeviceInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDisplayDeviceInterop_Impl::OpenSharedHandle(this, core::mem::transmute_copy(&nthandle), core::mem::transmute(&riid)) {
                Ok(ok__) => {
                    ppvobj.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSharedHandle: CreateSharedHandle::<Identity, OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDisplayDeviceInterop as windows_core::Interface>::IID
    }
}
pub trait IDisplayPathInterop_Impl: Sized {
    fn CreateSourcePresentationHandle(&self) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn GetSourceId(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IDisplayPathInterop {}
impl IDisplayPathInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDisplayPathInterop_Vtbl
    where
        Identity: IDisplayPathInterop_Impl,
    {
        unsafe extern "system" fn CreateSourcePresentationHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDisplayPathInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDisplayPathInterop_Impl::CreateSourcePresentationHandle(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDisplayPathInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDisplayPathInterop_Impl::GetSourceId(this) {
                Ok(ok__) => {
                    psourceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSourcePresentationHandle: CreateSourcePresentationHandle::<Identity, OFFSET>,
            GetSourceId: GetSourceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDisplayPathInterop as windows_core::Interface>::IID
    }
}
