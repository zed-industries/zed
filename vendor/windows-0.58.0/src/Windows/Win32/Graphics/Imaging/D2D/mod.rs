windows_core::imp::define_interface!(IWICImageEncoder, IWICImageEncoder_Vtbl, 0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
impl core::ops::Deref for IWICImageEncoder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICImageEncoder, windows_core::IUnknown);
impl IWICImageEncoder {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrame<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapFrameEncode>,
    {
        (windows_core::Interface::vtable(self).WriteFrame)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrameThumbnail<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapFrameEncode>,
    {
        (windows_core::Interface::vtable(self).WriteFrameThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteThumbnail<P0, P1>(&self, pimage: P0, pencoder: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapEncoder>,
    {
        (windows_core::Interface::vtable(self).WriteThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pencoder.param().abi(), pimageparameters).ok()
    }
}
#[repr(C)]
pub struct IWICImageEncoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrame: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrameThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrameThumbnail: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::WICImageParameters) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteThumbnail: usize,
}
windows_core::imp::define_interface!(IWICImagingFactory2, IWICImagingFactory2_Vtbl, 0x7b816b45_1996_4476_b132_de9e247c8af0);
impl core::ops::Deref for IWICImagingFactory2 {
    type Target = super::IWICImagingFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICImagingFactory2, windows_core::IUnknown, super::IWICImagingFactory);
impl IWICImagingFactory2 {
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn CreateImageEncoder<P0>(&self, pd2ddevice: P0) -> windows_core::Result<IWICImageEncoder>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Device>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImageEncoder)(windows_core::Interface::as_raw(self), pd2ddevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWICImagingFactory2_Vtbl {
    pub base__: super::IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
