windows_core::imp::define_interface!(IRadialControllerConfigurationInterop, IRadialControllerConfigurationInterop_Vtbl, 0x787cdaac_3186_476d_87e4_b9374a7b9970);
windows_core::imp::interface_hierarchy!(IRadialControllerConfigurationInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IRadialControllerConfigurationInterop {
    pub unsafe fn GetForWindow<T>(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRadialControllerConfigurationInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IRadialControllerConfigurationInterop_Vtbl {
    pub const fn new<Identity: IRadialControllerConfigurationInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IRadialControllerConfigurationInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRadialControllerConfigurationInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRadialControllerConfigurationInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadialControllerConfigurationInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRadialControllerConfigurationInterop {}
windows_core::imp::define_interface!(IRadialControllerIndependentInputSourceInterop, IRadialControllerIndependentInputSourceInterop_Vtbl, 0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
windows_core::imp::interface_hierarchy!(IRadialControllerIndependentInputSourceInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IRadialControllerIndependentInputSourceInterop {
    pub unsafe fn CreateForWindow<T>(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), hwnd, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRadialControllerIndependentInputSourceInterop_Impl: windows_core::IUnknownImpl {
    fn CreateForWindow(&self, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub const fn new<Identity: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateForWindow<Identity: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRadialControllerIndependentInputSourceInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRadialControllerIndependentInputSourceInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadialControllerIndependentInputSourceInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRadialControllerIndependentInputSourceInterop {}
windows_core::imp::define_interface!(IRadialControllerInterop, IRadialControllerInterop_Vtbl, 0x1b0535c9_57ad_45c1_9d79_ad5c34360513);
windows_core::imp::interface_hierarchy!(IRadialControllerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IRadialControllerInterop {
    pub unsafe fn CreateForWindow<T>(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), hwnd, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRadialControllerInterop_Impl: windows_core::IUnknownImpl {
    fn CreateForWindow(&self, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IRadialControllerInterop_Vtbl {
    pub const fn new<Identity: IRadialControllerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateForWindow<Identity: IRadialControllerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRadialControllerInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRadialControllerInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadialControllerInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRadialControllerInterop {}
