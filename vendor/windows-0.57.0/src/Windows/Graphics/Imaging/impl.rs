#[cfg(feature = "Storage_Streams")]
pub trait IBitmapFrame_Impl: Sized {
    fn GetThumbnailAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn BitmapProperties(&self) -> windows_core::Result<BitmapPropertiesView>;
    fn BitmapPixelFormat(&self) -> windows_core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> windows_core::Result<BitmapAlphaMode>;
    fn DpiX(&self) -> windows_core::Result<f64>;
    fn DpiY(&self) -> windows_core::Result<f64>;
    fn PixelWidth(&self) -> windows_core::Result<u32>;
    fn PixelHeight(&self) -> windows_core::Result<u32>;
    fn OrientedPixelWidth(&self) -> windows_core::Result<u32>;
    fn OrientedPixelHeight(&self) -> windows_core::Result<u32>;
    fn GetPixelDataAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
    fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Option<&BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IBitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrame";
}
#[cfg(feature = "Storage_Streams")]
impl IBitmapFrame_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>() -> IBitmapFrame_Vtbl {
        unsafe extern "system" fn GetThumbnailAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::GetThumbnailAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::BitmapProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapPixelFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BitmapPixelFormat) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::BitmapPixelFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitmapAlphaMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut BitmapAlphaMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::BitmapAlphaMode(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiX<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::DpiX(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DpiY<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::DpiY(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelWidth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::PixelWidth(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelHeight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::PixelHeight(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelWidth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::OrientedPixelWidth(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrientedPixelHeight<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::OrientedPixelHeight(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::GetPixelDataAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelDataTransformedAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrame_Impl::GetPixelDataTransformedAsync(this, pixelformat, alphamode, windows_core::from_raw_borrowed(&transform), exiforientationmode, colormanagementmode) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapFrame, OFFSET>(),
            GetThumbnailAsync: GetThumbnailAsync::<Identity, Impl, OFFSET>,
            BitmapProperties: BitmapProperties::<Identity, Impl, OFFSET>,
            BitmapPixelFormat: BitmapPixelFormat::<Identity, Impl, OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Identity, Impl, OFFSET>,
            DpiX: DpiX::<Identity, Impl, OFFSET>,
            DpiY: DpiY::<Identity, Impl, OFFSET>,
            PixelWidth: PixelWidth::<Identity, Impl, OFFSET>,
            PixelHeight: PixelHeight::<Identity, Impl, OFFSET>,
            OrientedPixelWidth: OrientedPixelWidth::<Identity, Impl, OFFSET>,
            OrientedPixelHeight: OrientedPixelHeight::<Identity, Impl, OFFSET>,
            GetPixelDataAsync: GetPixelDataAsync::<Identity, Impl, OFFSET>,
            GetPixelDataTransformedAsync: GetPixelDataTransformedAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapFrame as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait IBitmapFrameWithSoftwareBitmap_Impl: Sized + IBitmapFrame_Impl {
    fn GetSoftwareBitmapAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Option<&BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IBitmapFrameWithSoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapFrameWithSoftwareBitmap";
}
#[cfg(feature = "Storage_Streams")]
impl IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>() -> IBitmapFrameWithSoftwareBitmap_Vtbl {
        unsafe extern "system" fn GetSoftwareBitmapAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrameWithSoftwareBitmap_Impl::GetSoftwareBitmapAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapConvertedAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrameWithSoftwareBitmap_Impl::GetSoftwareBitmapConvertedAsync(this, pixelformat, alphamode) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSoftwareBitmapTransformedAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapFrameWithSoftwareBitmap_Impl::GetSoftwareBitmapTransformedAsync(this, pixelformat, alphamode, windows_core::from_raw_borrowed(&transform), exiforientationmode, colormanagementmode) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapFrameWithSoftwareBitmap, OFFSET>(),
            GetSoftwareBitmapAsync: GetSoftwareBitmapAsync::<Identity, Impl, OFFSET>,
            GetSoftwareBitmapConvertedAsync: GetSoftwareBitmapConvertedAsync::<Identity, Impl, OFFSET>,
            GetSoftwareBitmapTransformedAsync: GetSoftwareBitmapTransformedAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapFrameWithSoftwareBitmap as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IBitmapPropertiesView_Impl: Sized {
    fn GetPropertiesAsync(&self, propertiestoretrieve: Option<&super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>) -> windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IBitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.IBitmapPropertiesView";
}
#[cfg(feature = "Foundation_Collections")]
impl IBitmapPropertiesView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapPropertiesView_Impl, const OFFSET: isize>() -> IBitmapPropertiesView_Vtbl {
        unsafe extern "system" fn GetPropertiesAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBitmapPropertiesView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiestoretrieve: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBitmapPropertiesView_Impl::GetPropertiesAsync(this, windows_core::from_raw_borrowed(&propertiestoretrieve)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapPropertiesView, OFFSET>(),
            GetPropertiesAsync: GetPropertiesAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapPropertiesView as windows_core::Interface>::IID
    }
}
