#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
pub trait IPdfRendererNative_Impl: Sized {
    fn RenderPageToSurface(&self, pdfpage: Option<&windows_core::IUnknown>, psurface: Option<&super::super::super::Graphics::Dxgi::IDXGISurface>, offset: &super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::Result<()>;
    fn RenderPageToDeviceContext(&self, pdfpage: Option<&windows_core::IUnknown>, pd2ddevicecontext: Option<&super::super::super::Graphics::Direct2D::ID2D1DeviceContext>, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl windows_core::RuntimeName for IPdfRendererNative {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl IPdfRendererNative_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPdfRendererNative_Vtbl
    where
        Identity: IPdfRendererNative_Impl,
    {
        unsafe extern "system" fn RenderPageToSurface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdfpage: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::HRESULT
        where
            Identity: IPdfRendererNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPdfRendererNative_Impl::RenderPageToSurface(this, windows_core::from_raw_borrowed(&pdfpage), windows_core::from_raw_borrowed(&psurface), core::mem::transmute(&offset), core::mem::transmute_copy(&prenderparams)).into()
        }
        unsafe extern "system" fn RenderPageToDeviceContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdfpage: *mut core::ffi::c_void, pd2ddevicecontext: *mut core::ffi::c_void, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::HRESULT
        where
            Identity: IPdfRendererNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPdfRendererNative_Impl::RenderPageToDeviceContext(this, windows_core::from_raw_borrowed(&pdfpage), windows_core::from_raw_borrowed(&pd2ddevicecontext), core::mem::transmute_copy(&prenderparams)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RenderPageToSurface: RenderPageToSurface::<Identity, OFFSET>,
            RenderPageToDeviceContext: RenderPageToDeviceContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPdfRendererNative as windows_core::Interface>::IID
    }
}
