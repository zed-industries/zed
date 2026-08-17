#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IGraphicsCaptureItemInterop_Impl: Sized {
    fn CreateForWindow(&self, window: super::super::super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateForMonitor(&self, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IGraphicsCaptureItemInterop {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IGraphicsCaptureItemInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGraphicsCaptureItemInterop_Vtbl
    where
        Identity: IGraphicsCaptureItemInterop_Impl,
    {
        unsafe extern "system" fn CreateForWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGraphicsCaptureItemInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGraphicsCaptureItemInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn CreateForMonitor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const windows_core::GUID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGraphicsCaptureItemInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGraphicsCaptureItemInterop_Impl::CreateForMonitor(this, core::mem::transmute_copy(&monitor), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
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
