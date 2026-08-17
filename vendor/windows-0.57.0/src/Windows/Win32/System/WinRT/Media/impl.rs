pub trait IAudioFrameNative_Impl: Sized {
    fn GetData(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioFrameNative {}
impl IAudioFrameNative_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioFrameNative_Impl, const OFFSET: isize>() -> IAudioFrameNative_Vtbl {
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioFrameNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioFrameNative_Impl::GetData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioFrameNative, OFFSET>(), GetData: GetData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioFrameNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IAudioFrameNativeFactory_Impl: Sized {
    fn CreateFromMFSample(&self, data: Option<&super::super::super::Media::MediaFoundation::IMFSample>, forcereadonly: super::super::super::Foundation::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::RuntimeName for IAudioFrameNativeFactory {}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IAudioFrameNativeFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioFrameNativeFactory_Impl, const OFFSET: isize>() -> IAudioFrameNativeFactory_Vtbl {
        unsafe extern "system" fn CreateFromMFSample<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioFrameNativeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, forcereadonly: super::super::super::Foundation::BOOL, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioFrameNativeFactory_Impl::CreateFromMFSample(this, windows_core::from_raw_borrowed(&data), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioFrameNativeFactory, OFFSET>(),
            CreateFromMFSample: CreateFromMFSample::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioFrameNativeFactory as windows_core::Interface>::IID
    }
}
pub trait IVideoFrameNative_Impl: Sized {
    fn GetData(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDevice(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVideoFrameNative {}
impl IVideoFrameNative_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVideoFrameNative_Impl, const OFFSET: isize>() -> IVideoFrameNative_Vtbl {
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVideoFrameNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVideoFrameNative_Impl::GetData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVideoFrameNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVideoFrameNative_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVideoFrameNative, OFFSET>(),
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoFrameNative as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IVideoFrameNativeFactory_Impl: Sized {
    fn CreateFromMFSample(&self, data: Option<&super::super::super::Media::MediaFoundation::IMFSample>, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: Option<&super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::RuntimeName for IVideoFrameNativeFactory {}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IVideoFrameNativeFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVideoFrameNativeFactory_Impl, const OFFSET: isize>() -> IVideoFrameNativeFactory_Vtbl {
        unsafe extern "system" fn CreateFromMFSample<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVideoFrameNativeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, subtype: *const windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVideoFrameNativeFactory_Impl::CreateFromMFSample(this, windows_core::from_raw_borrowed(&data), core::mem::transmute_copy(&subtype), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&forcereadonly), core::mem::transmute_copy(&mindisplayaperture), windows_core::from_raw_borrowed(&device), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVideoFrameNativeFactory, OFFSET>(),
            CreateFromMFSample: CreateFromMFSample::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVideoFrameNativeFactory as windows_core::Interface>::IID
    }
}
