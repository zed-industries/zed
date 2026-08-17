windows_core::imp::define_interface!(IAudioFrameNative, IAudioFrameNative_Vtbl, 0x20be1e2e_930f_4746_9335_3c332f255093);
impl core::ops::Deref for IAudioFrameNative {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioFrameNative, windows_core::IUnknown, windows_core::IInspectable);
impl IAudioFrameNative {
    pub unsafe fn GetData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAudioFrameNative_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioFrameNativeFactory, IAudioFrameNativeFactory_Vtbl, 0x7bd67cf8_bf7d_43e6_af8d_b170ee0c0110);
impl core::ops::Deref for IAudioFrameNativeFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioFrameNativeFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IAudioFrameNativeFactory {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn CreateFromMFSample<P0, P1, T>(&self, data: P0, forcereadonly: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Media::MediaFoundation::IMFSample>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateFromMFSample)(windows_core::Interface::as_raw(self), data.param().abi(), forcereadonly.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAudioFrameNativeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub CreateFromMFSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::BOOL, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    CreateFromMFSample: usize,
}
windows_core::imp::define_interface!(IVideoFrameNative, IVideoFrameNative_Vtbl, 0x26ba702b_314a_4620_aaf6_7a51aa58fa18);
impl core::ops::Deref for IVideoFrameNative {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVideoFrameNative, windows_core::IUnknown, windows_core::IInspectable);
impl IVideoFrameNative {
    pub unsafe fn GetData<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDevice<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVideoFrameNative_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoFrameNativeFactory, IVideoFrameNativeFactory_Vtbl, 0x69e3693e_8e1e_4e63_ac4c_7fdc21d9731d);
impl core::ops::Deref for IVideoFrameNativeFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVideoFrameNativeFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IVideoFrameNativeFactory {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn CreateFromMFSample<P0, P1, P2, T>(&self, data: P0, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: P1, mindisplayaperture: Option<*const super::super::super::Media::MediaFoundation::MFVideoArea>, device: P2) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Media::MediaFoundation::IMFSample>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateFromMFSample)(windows_core::Interface::as_raw(self), data.param().abi(), subtype, width, height, forcereadonly.param().abi(), core::mem::transmute(mindisplayaperture.unwrap_or(std::ptr::null())), device.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVideoFrameNativeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub CreateFromMFSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32, u32, super::super::super::Foundation::BOOL, *const super::super::super::Media::MediaFoundation::MFVideoArea, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    CreateFromMFSample: usize,
}
pub const CLSID_AudioFrameNativeFactory: windows_core::GUID = windows_core::GUID::from_u128(0x16a0a3b9_9f65_4102_9367_2cda3a4f372a);
pub const CLSID_VideoFrameNativeFactory: windows_core::GUID = windows_core::GUID::from_u128(0xd194386a_04e3_4814_8100_b2b0ae6d78c7);
#[cfg(feature = "implement")]
core::include!("impl.rs");
