windows_core::imp::define_interface!(ISoftwareBitmapNative, ISoftwareBitmapNative_Vtbl, 0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
impl core::ops::Deref for ISoftwareBitmapNative {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISoftwareBitmapNative, windows_core::IUnknown, windows_core::IInspectable);
impl ISoftwareBitmapNative {
    pub unsafe fn GetData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISoftwareBitmapNative_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISoftwareBitmapNativeFactory, ISoftwareBitmapNativeFactory_Vtbl, 0xc3c181ec_2914_4791_af02_02d224a10b43);
impl core::ops::Deref for ISoftwareBitmapNativeFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISoftwareBitmapNativeFactory, windows_core::IUnknown, windows_core::IInspectable);
impl ISoftwareBitmapNativeFactory {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateFromWICBitmap<P0, P1, T>(&self, data: P0, forcereadonly: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::IWICBitmap>,
        P1: windows_core::Param<super::super::super::super::Foundation::BOOL>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateFromWICBitmap)(windows_core::Interface::as_raw(self), data.param().abi(), forcereadonly.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn CreateFromMF2DBuffer2<P0, P1, T>(&self, data: P0, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: P1, mindisplayaperture: Option<*const super::super::super::super::Media::MediaFoundation::MFVideoArea>) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>,
        P1: windows_core::Param<super::super::super::super::Foundation::BOOL>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateFromMF2DBuffer2)(windows_core::Interface::as_raw(self), data.param().abi(), subtype, width, height, forcereadonly.param().abi(), core::mem::transmute(mindisplayaperture.unwrap_or(std::ptr::null())), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISoftwareBitmapNativeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub CreateFromWICBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::super::Foundation::BOOL, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    CreateFromWICBitmap: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub CreateFromMF2DBuffer2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, u32, super::super::super::super::Foundation::BOOL, *const super::super::super::super::Media::MediaFoundation::MFVideoArea, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    CreateFromMF2DBuffer2: usize,
}
pub const CLSID_SoftwareBitmapNativeFactory: windows_core::GUID = windows_core::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
#[cfg(feature = "implement")]
core::include!("impl.rs");
