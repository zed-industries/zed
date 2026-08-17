windows_core::imp::define_interface!(IRadialControllerConfigurationInterop, IRadialControllerConfigurationInterop_Vtbl, 0x787cdaac_3186_476d_87e4_b9374a7b9970);
impl core::ops::Deref for IRadialControllerConfigurationInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRadialControllerConfigurationInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IRadialControllerConfigurationInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, hwnd: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRadialControllerConfigurationInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerIndependentInputSourceInterop, IRadialControllerIndependentInputSourceInterop_Vtbl, 0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
impl core::ops::Deref for IRadialControllerIndependentInputSourceInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRadialControllerIndependentInputSourceInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IRadialControllerIndependentInputSourceInterop {
    pub unsafe fn CreateForWindow<P0, T>(&self, hwnd: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerInterop, IRadialControllerInterop_Vtbl, 0x1b0535c9_57ad_45c1_9d79_ad5c34360513);
impl core::ops::Deref for IRadialControllerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRadialControllerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IRadialControllerInterop {
    pub unsafe fn CreateForWindow<P0, T>(&self, hwnd: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRadialControllerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
