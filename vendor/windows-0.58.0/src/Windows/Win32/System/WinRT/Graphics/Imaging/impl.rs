pub trait ISoftwareBitmapNative_Impl: Sized {
    fn GetData(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISoftwareBitmapNative {}
impl ISoftwareBitmapNative_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISoftwareBitmapNative_Vtbl
    where
        Identity: ISoftwareBitmapNative_Impl,
    {
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISoftwareBitmapNative_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISoftwareBitmapNative_Impl::GetData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISoftwareBitmapNative, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISoftwareBitmapNative as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
pub trait ISoftwareBitmapNativeFactory_Impl: Sized {
    fn CreateFromWICBitmap(&self, data: Option<&super::super::super::super::Graphics::Imaging::IWICBitmap>, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateFromMF2DBuffer2(&self, data: Option<&super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for ISoftwareBitmapNativeFactory {}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl ISoftwareBitmapNativeFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISoftwareBitmapNativeFactory_Vtbl
    where
        Identity: ISoftwareBitmapNativeFactory_Impl,
    {
        unsafe extern "system" fn CreateFromWICBitmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISoftwareBitmapNativeFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISoftwareBitmapNativeFactory_Impl::CreateFromWICBitmap(this, windows_core::from_raw_borrowed(&data), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateFromMF2DBuffer2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISoftwareBitmapNativeFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISoftwareBitmapNativeFactory_Impl::CreateFromMF2DBuffer2(this, windows_core::from_raw_borrowed(&data), core::mem::transmute_copy(&subtype), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&mindisplayaperture), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISoftwareBitmapNativeFactory, OFFSET>(),
            CreateFromWICBitmap: CreateFromWICBitmap::<Identity, OFFSET>,
            CreateFromMF2DBuffer2: CreateFromMF2DBuffer2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISoftwareBitmapNativeFactory as windows_core::Interface>::IID
    }
}
