windows_core::imp::define_interface!(ICoreFrameworkInputViewInterop, ICoreFrameworkInputViewInterop_Vtbl, 0x0e3da342_b11c_484b_9c1c_be0d61c2f6c5);
impl core::ops::Deref for ICoreFrameworkInputViewInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreFrameworkInputViewInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreFrameworkInputViewInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICoreFrameworkInputViewInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
