#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn PdfCreateRenderer<P0>(pdevice: P0) -> windows_core::Result<IPdfRendererNative>
where
    P0: windows_core::Param<super::super::super::Graphics::Dxgi::IDXGIDevice>,
{
    windows_targets::link!("windows.data.pdf.dll" "system" fn PdfCreateRenderer(pdevice : * mut core::ffi::c_void, pprenderer : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PdfCreateRenderer(pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(IPdfRendererNative, IPdfRendererNative_Vtbl, 0x7d9dcd91_d277_4947_8527_07a0daeda94a);
impl core::ops::Deref for IPdfRendererNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPdfRendererNative, windows_core::IUnknown);
impl IPdfRendererNative {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub unsafe fn RenderPageToSurface<P0, P1>(&self, pdfpage: P0, psurface: P1, offset: super::super::super::Foundation::POINT, prenderparams: Option<*const PDF_RENDER_PARAMS>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::super::Graphics::Dxgi::IDXGISurface>,
    {
        (windows_core::Interface::vtable(self).RenderPageToSurface)(windows_core::Interface::as_raw(self), pdfpage.param().abi(), psurface.param().abi(), core::mem::transmute(offset), core::mem::transmute(prenderparams.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn RenderPageToDeviceContext<P0, P1>(&self, pdfpage: P0, pd2ddevicecontext: P1, prenderparams: Option<*const PDF_RENDER_PARAMS>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::super::Graphics::Direct2D::ID2D1DeviceContext>,
    {
        (windows_core::Interface::vtable(self).RenderPageToDeviceContext)(windows_core::Interface::as_raw(self), pdfpage.param().abi(), pd2ddevicecontext.param().abi(), core::mem::transmute(prenderparams.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::super::Graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::super::Graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for PDF_RENDER_PARAMS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for PDF_RENDER_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = Option<unsafe extern "system" fn(param0: Option<super::super::super::Graphics::Dxgi::IDXGIDevice>, param1: *mut Option<IPdfRendererNative>) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
