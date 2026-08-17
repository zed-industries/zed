pub const CLSID_SoftwareBitmapNativeFactory: windows_core::GUID = windows_core::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
windows_core::imp::define_interface!(ISoftwareBitmapNative, ISoftwareBitmapNative_Vtbl, 0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
windows_core::imp::interface_hierarchy!(ISoftwareBitmapNative, windows_core::IUnknown, windows_core::IInspectable);
impl ISoftwareBitmapNative {
    pub unsafe fn GetData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNative_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISoftwareBitmapNative_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ISoftwareBitmapNative_Vtbl {
    pub const fn new<Identity: ISoftwareBitmapNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: ISoftwareBitmapNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISoftwareBitmapNative_Impl::GetData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISoftwareBitmapNative, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISoftwareBitmapNative as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISoftwareBitmapNative {}
windows_core::imp::define_interface!(ISoftwareBitmapNativeFactory, ISoftwareBitmapNativeFactory_Vtbl, 0xc3c181ec_2914_4791_af02_02d224a10b43);
windows_core::imp::interface_hierarchy!(ISoftwareBitmapNativeFactory, windows_core::IUnknown, windows_core::IInspectable);
impl ISoftwareBitmapNativeFactory {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateFromWICBitmap<P0, T>(&self, data: P0, forcereadonly: bool) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::IWICBitmap>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateFromWICBitmap)(windows_core::Interface::as_raw(self), data.param().abi(), forcereadonly.into(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn CreateFromMF2DBuffer2<P0, T>(&self, data: P0, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: bool, mindisplayaperture: Option<*const super::super::super::super::Media::MediaFoundation::MFVideoArea>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateFromMF2DBuffer2)(windows_core::Interface::as_raw(self), data.param().abi(), subtype, width, height, forcereadonly.into(), mindisplayaperture.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateFromWICBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateFromWICBitmap: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub CreateFromMF2DBuffer2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, u32, windows_core::BOOL, *const super::super::super::super::Media::MediaFoundation::MFVideoArea, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    CreateFromMF2DBuffer2: usize,
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
pub trait ISoftwareBitmapNativeFactory_Impl: windows_core::IUnknownImpl {
    fn CreateFromWICBitmap(&self, data: windows_core::Ref<super::super::super::super::Graphics::Imaging::IWICBitmap>, forcereadonly: windows_core::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateFromMF2DBuffer2(&self, data: windows_core::Ref<super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: windows_core::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl ISoftwareBitmapNativeFactory_Vtbl {
    pub const fn new<Identity: ISoftwareBitmapNativeFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromWICBitmap<Identity: ISoftwareBitmapNativeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, forcereadonly: windows_core::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISoftwareBitmapNativeFactory_Impl::CreateFromWICBitmap(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateFromMF2DBuffer2<Identity: ISoftwareBitmapNativeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: windows_core::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISoftwareBitmapNativeFactory_Impl::CreateFromMF2DBuffer2(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&subtype), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&mindisplayaperture), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
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
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for ISoftwareBitmapNativeFactory {}
