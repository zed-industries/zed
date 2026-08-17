windows_core::imp::define_interface!(IGraphicsCaptureItemInterop, IGraphicsCaptureItemInterop_Vtbl, 0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
windows_core::imp::interface_hierarchy!(IGraphicsCaptureItemInterop, windows_core::IUnknown);
impl IGraphicsCaptureItemInterop {
    pub unsafe fn CreateForWindow<T>(&self, window: super::super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateForMonitor<T>(&self, monitor: super::super::super::super::Graphics::Gdi::HMONITOR) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateForMonitor)(windows_core::Interface::as_raw(self), monitor, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateForMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Graphics::Gdi::HMONITOR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateForMonitor: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IGraphicsCaptureItemInterop_Impl: windows_core::IUnknownImpl {
    fn CreateForWindow(&self, window: super::super::super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateForMonitor(&self, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IGraphicsCaptureItemInterop_Vtbl {
    pub const fn new<Identity: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateForWindow<Identity: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGraphicsCaptureItemInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn CreateForMonitor<Identity: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGraphicsCaptureItemInterop_Impl::CreateForMonitor(this, core::mem::transmute_copy(&monitor), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
            CreateForMonitor: CreateForMonitor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IGraphicsCaptureItemInterop {}
