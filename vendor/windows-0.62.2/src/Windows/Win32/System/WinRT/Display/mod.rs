windows_core::imp::define_interface!(IDisplayDeviceInterop, IDisplayDeviceInterop_Vtbl, 0x64338358_366a_471b_bd56_dd8ef48e439b);
windows_core::imp::interface_hierarchy!(IDisplayDeviceInterop, windows_core::IUnknown);
impl IDisplayDeviceInterop {
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn CreateSharedHandle<P0>(&self, pobject: P0, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSharedHandle)(windows_core::Interface::as_raw(self), pobject.param().abi(), psecurityattributes, access, core::mem::transmute_copy(name), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OpenSharedHandle(&self, nthandle: super::super::super::Foundation::HANDLE, riid: windows_core::GUID) -> windows_core::Result<*mut core::ffi::c_void> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenSharedHandle)(windows_core::Interface::as_raw(self), nthandle, core::mem::transmute(riid), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security")]
    pub CreateSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Security::SECURITY_ATTRIBUTES, u32, *mut core::ffi::c_void, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    CreateSharedHandle: usize,
    pub OpenSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HANDLE, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Security")]
pub trait IDisplayDeviceInterop_Impl: windows_core::IUnknownImpl {
    fn CreateSharedHandle(&self, pobject: windows_core::Ref<windows_core::IInspectable>, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(&self, nthandle: super::super::super::Foundation::HANDLE, riid: &windows_core::GUID) -> windows_core::Result<*mut core::ffi::c_void>;
}
#[cfg(feature = "Win32_Security")]
impl IDisplayDeviceInterop_Vtbl {
    pub const fn new<Identity: IDisplayDeviceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSharedHandle<Identity: IDisplayDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: *mut core::ffi::c_void, phandle: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDisplayDeviceInterop_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&psecurityattributes), core::mem::transmute_copy(&access), core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Identity: IDisplayDeviceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDisplayDeviceInterop_Impl::OpenSharedHandle(this, core::mem::transmute_copy(&nthandle), core::mem::transmute(&riid)) {
                    Ok(ok__) => {
                        ppvobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for IDisplayDeviceInterop {}
windows_core::imp::define_interface!(IDisplayPathInterop, IDisplayPathInterop_Vtbl, 0xa6ba4205_e59e_4e71_b25b_4e436d21ee3d);
windows_core::imp::interface_hierarchy!(IDisplayPathInterop, windows_core::IUnknown);
impl IDisplayPathInterop {
    pub unsafe fn CreateSourcePresentationHandle(&self) -> windows_core::Result<super::super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSourcePresentationHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSourceId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateSourcePresentationHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetSourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDisplayPathInterop_Impl: windows_core::IUnknownImpl {
    fn CreateSourcePresentationHandle(&self) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn GetSourceId(&self) -> windows_core::Result<u32>;
}
impl IDisplayPathInterop_Vtbl {
    pub const fn new<Identity: IDisplayPathInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSourcePresentationHandle<Identity: IDisplayPathInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDisplayPathInterop_Impl::CreateSourcePresentationHandle(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceId<Identity: IDisplayPathInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDisplayPathInterop_Impl::GetSourceId(this) {
                    Ok(ok__) => {
                        psourceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IDisplayPathInterop {}
