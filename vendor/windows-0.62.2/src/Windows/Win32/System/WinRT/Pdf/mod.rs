#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn PdfCreateRenderer<P0>(pdevice: P0) -> windows_core::Result<IPdfRendererNative>
where
    P0: windows_core::Param<super::super::super::Graphics::Dxgi::IDXGIDevice>,
{
    windows_core::link!("windows.data.pdf.dll" "system" fn PdfCreateRenderer(pdevice : * mut core::ffi::c_void, pprenderer : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PdfCreateRenderer(pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IPdfRendererNative, IPdfRendererNative_Vtbl, 0x7d9dcd91_d277_4947_8527_07a0daeda94a);
windows_core::imp::interface_hierarchy!(IPdfRendererNative, windows_core::IUnknown);
impl IPdfRendererNative {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn RenderPageToSurface<P0, P1>(&self, pdfpage: P0, psurface: P1, offset: super::super::super::Foundation::POINT, prenderparams: Option<*const PDF_RENDER_PARAMS>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::super::Graphics::Dxgi::IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenderPageToSurface)(windows_core::Interface::as_raw(self), pdfpage.param().abi(), psurface.param().abi(), core::mem::transmute(offset), prenderparams.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn RenderPageToDeviceContext<P0, P1>(&self, pdfpage: P0, pd2ddevicecontext: P1, prenderparams: Option<*const PDF_RENDER_PARAMS>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::super::Graphics::Direct2D::ID2D1DeviceContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenderPageToDeviceContext)(windows_core::Interface::as_raw(self), pdfpage.param().abi(), pd2ddevicecontext.param().abi(), prenderparams.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfRendererNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub RenderPageToSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::POINT, *const PDF_RENDER_PARAMS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi")))]
    RenderPageToSurface: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub RenderPageToDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const PDF_RENDER_PARAMS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    RenderPageToDeviceContext: usize,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
pub trait IPdfRendererNative_Impl: windows_core::IUnknownImpl {
    fn RenderPageToSurface(&self, pdfpage: windows_core::Ref<windows_core::IUnknown>, psurface: windows_core::Ref<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: &super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::Result<()>;
    fn RenderPageToDeviceContext(&self, pdfpage: windows_core::Ref<windows_core::IUnknown>, pd2ddevicecontext: windows_core::Ref<super::super::super::Graphics::Direct2D::ID2D1DeviceContext>, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl IPdfRendererNative_Vtbl {
    pub const fn new<Identity: IPdfRendererNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RenderPageToSurface<Identity: IPdfRendererNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdfpage: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPdfRendererNative_Impl::RenderPageToSurface(this, core::mem::transmute_copy(&pdfpage), core::mem::transmute_copy(&psurface), core::mem::transmute(&offset), core::mem::transmute_copy(&prenderparams)).into()
            }
        }
        unsafe extern "system" fn RenderPageToDeviceContext<Identity: IPdfRendererNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdfpage: *mut core::ffi::c_void, pd2ddevicecontext: *mut core::ffi::c_void, prenderparams: *const PDF_RENDER_PARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPdfRendererNative_Impl::RenderPageToDeviceContext(this, core::mem::transmute_copy(&pdfpage), core::mem::transmute_copy(&pd2ddevicecontext), core::mem::transmute_copy(&prenderparams)).into()
            }
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
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl windows_core::RuntimeName for IPdfRendererNative {}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::super::Graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::super::Graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: bool,
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = Option<unsafe extern "system" fn(param0: windows_core::Ref<super::super::super::Graphics::Dxgi::IDXGIDevice>, param1: windows_core::OutRef<IPdfRendererNative>) -> windows_core::HRESULT>;
