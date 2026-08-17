windows_core::imp::define_interface!(IWICImageEncoder, IWICImageEncoder_Vtbl, 0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
windows_core::imp::interface_hierarchy!(IWICImageEncoder, windows_core::IUnknown);
impl IWICImageEncoder {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrame<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapFrameEncode>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteFrame)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrameThumbnail<P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapFrameEncode>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteFrameThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pframeencode.param().abi(), pimageparameters).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteThumbnail<P0, P1>(&self, pimage: P0, pencoder: P1, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Direct2D::ID2D1Image>,
        P1: windows_core::Param<super::IWICBitmapEncoder>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteThumbnail)(windows_core::Interface::as_raw(self), pimage.param().abi(), pencoder.param().abi(), pimageparameters).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICImageEncoder_Impl: windows_core::IUnknownImpl {
    fn WriteFrame(&self, pimage: windows_core::Ref<super::super::Direct2D::ID2D1Image>, pframeencode: windows_core::Ref<super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>;
    fn WriteFrameThumbnail(&self, pimage: windows_core::Ref<super::super::Direct2D::ID2D1Image>, pframeencode: windows_core::Ref<super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>;
    fn WriteThumbnail(&self, pimage: windows_core::Ref<super::super::Direct2D::ID2D1Image>, pencoder: windows_core::Ref<super::IWICBitmapEncoder>, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICImageEncoder_Vtbl {
    pub const fn new<Identity: IWICImageEncoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteFrame<Identity: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pframeencode: *mut core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICImageEncoder_Impl::WriteFrame(this, core::mem::transmute_copy(&pimage), core::mem::transmute_copy(&pframeencode), core::mem::transmute_copy(&pimageparameters)).into()
            }
        }
        unsafe extern "system" fn WriteFrameThumbnail<Identity: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pframeencode: *mut core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICImageEncoder_Impl::WriteFrameThumbnail(this, core::mem::transmute_copy(&pimage), core::mem::transmute_copy(&pframeencode), core::mem::transmute_copy(&pimageparameters)).into()
            }
        }
        unsafe extern "system" fn WriteThumbnail<Identity: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pencoder: *mut core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICImageEncoder_Impl::WriteThumbnail(this, core::mem::transmute_copy(&pimage), core::mem::transmute_copy(&pencoder), core::mem::transmute_copy(&pimageparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WriteFrame: WriteFrame::<Identity, OFFSET>,
            WriteFrameThumbnail: WriteFrameThumbnail::<Identity, OFFSET>,
            WriteThumbnail: WriteThumbnail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImageEncoder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for IWICImageEncoder {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageEncoder)(windows_core::Interface::as_raw(self), pd2ddevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory2_Vtbl {
    pub base__: super::IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory2_Impl: super::IWICImagingFactory_Impl {
    fn CreateImageEncoder(&self, pd2ddevice: windows_core::Ref<super::super::Direct2D::ID2D1Device>) -> windows_core::Result<IWICImageEncoder>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory2_Vtbl {
    pub const fn new<Identity: IWICImagingFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateImageEncoder<Identity: IWICImagingFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd2ddevice: *mut core::ffi::c_void, ppwicimageencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICImagingFactory2_Impl::CreateImageEncoder(this, core::mem::transmute_copy(&pd2ddevice)) {
                    Ok(ok__) => {
                        ppwicimageencoder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IWICImagingFactory_Vtbl::new::<Identity, OFFSET>(), CreateImageEncoder: CreateImageEncoder::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImagingFactory2 as windows_core::Interface>::IID || iid == &<super::IWICImagingFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IWICImagingFactory2 {}
