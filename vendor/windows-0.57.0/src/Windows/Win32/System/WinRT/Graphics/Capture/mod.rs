windows_core::imp::define_interface!(IGraphicsCaptureItemInterop, IGraphicsCaptureItemInterop_Vtbl, 0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
impl core::ops::Deref for IGraphicsCaptureItemInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGraphicsCaptureItemInterop, windows_core::IUnknown);
impl IGraphicsCaptureItemInterop {
    pub unsafe fn CreateForWindow<P0, T>(&self, window: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), window.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateForMonitor<P0, T>(&self, monitor: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Gdi::HMONITOR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateForMonitor)(windows_core::Interface::as_raw(self), monitor.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IGraphicsCaptureItemInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateForMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Graphics::Gdi::HMONITOR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateForMonitor: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
