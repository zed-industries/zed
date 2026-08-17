#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICImageEncoder_Impl: Sized {
    fn WriteFrame(&self, pimage: Option<&super::super::Direct2D::ID2D1Image>, pframeencode: Option<&super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>;
    fn WriteFrameThumbnail(&self, pimage: Option<&super::super::Direct2D::ID2D1Image>, pframeencode: Option<&super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>;
    fn WriteThumbnail(&self, pimage: Option<&super::super::Direct2D::ID2D1Image>, pencoder: Option<&super::IWICBitmapEncoder>, pimageparameters: *const super::WICImageParameters) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for IWICImageEncoder {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl IWICImageEncoder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>() -> IWICImageEncoder_Vtbl {
        unsafe extern "system" fn WriteFrame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pframeencode: *mut core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWICImageEncoder_Impl::WriteFrame(this, windows_core::from_raw_borrowed(&pimage), windows_core::from_raw_borrowed(&pframeencode), core::mem::transmute_copy(&pimageparameters)).into()
        }
        unsafe extern "system" fn WriteFrameThumbnail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pframeencode: *mut core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWICImageEncoder_Impl::WriteFrameThumbnail(this, windows_core::from_raw_borrowed(&pimage), windows_core::from_raw_borrowed(&pframeencode), core::mem::transmute_copy(&pimageparameters)).into()
        }
        unsafe extern "system" fn WriteThumbnail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimage: *mut core::ffi::c_void, pencoder: *mut core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IWICImageEncoder_Impl::WriteThumbnail(this, windows_core::from_raw_borrowed(&pimage), windows_core::from_raw_borrowed(&pencoder), core::mem::transmute_copy(&pimageparameters)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WriteFrame: WriteFrame::<Identity, Impl, OFFSET>,
            WriteFrameThumbnail: WriteFrameThumbnail::<Identity, Impl, OFFSET>,
            WriteThumbnail: WriteThumbnail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImageEncoder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory2_Impl: Sized + super::IWICImagingFactory_Impl {
    fn CreateImageEncoder(&self, pd2ddevice: Option<&super::super::Direct2D::ID2D1Device>) -> windows_core::Result<IWICImageEncoder>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IWICImagingFactory2 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWICImagingFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory2_Impl, const OFFSET: isize>() -> IWICImagingFactory2_Vtbl {
        unsafe extern "system" fn CreateImageEncoder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWICImagingFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pd2ddevice: *mut core::ffi::c_void, ppwicimageencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWICImagingFactory2_Impl::CreateImageEncoder(this, windows_core::from_raw_borrowed(&pd2ddevice)) {
                Ok(ok__) => {
                    core::ptr::write(ppwicimageencoder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::IWICImagingFactory_Vtbl::new::<Identity, Impl, OFFSET>(), CreateImageEncoder: CreateImageEncoder::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICImagingFactory2 as windows_core::Interface>::IID || iid == &<super::IWICImagingFactory as windows_core::Interface>::IID
    }
}
