pub trait IDefaultAudioDeviceChangedEventArgs_Impl: Sized {
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Role(&self) -> windows_core::Result<AudioDeviceRole>;
}
impl windows_core::RuntimeName for IDefaultAudioDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs";
}
impl IDefaultAudioDeviceChangedEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: isize>() -> IDefaultAudioDeviceChangedEventArgs_Vtbl {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDefaultAudioDeviceChangedEventArgs_Impl::Id(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Role<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut AudioDeviceRole) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDefaultAudioDeviceChangedEventArgs_Impl::Role(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDefaultAudioDeviceChangedEventArgs, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDefaultAudioDeviceChangedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
pub trait IMediaDeviceController_Impl: Sized {
    fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>;
    fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::MediaProperties::IMediaEncodingProperties>;
    fn SetMediaStreamPropertiesAsync(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: Option<&super::MediaProperties::IMediaEncodingProperties>) -> windows_core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IMediaDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceController";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl IMediaDeviceController_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: isize>() -> IMediaDeviceController_Vtbl {
        unsafe extern "system" fn GetAvailableMediaStreamProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMediaDeviceController_Impl::GetAvailableMediaStreamProperties(this, mediastreamtype) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaStreamProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMediaDeviceController_Impl::GetMediaStreamProperties(this, mediastreamtype) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaStreamPropertiesAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMediaDeviceController_Impl::SetMediaStreamPropertiesAsync(this, mediastreamtype, windows_core::from_raw_borrowed(&mediaencodingproperties)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaDeviceController, OFFSET>(),
            GetAvailableMediaStreamProperties: GetAvailableMediaStreamProperties::<Identity, Impl, OFFSET>,
            GetMediaStreamProperties: GetMediaStreamProperties::<Identity, Impl, OFFSET>,
            SetMediaStreamPropertiesAsync: SetMediaStreamPropertiesAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaDeviceController as windows_core::Interface>::IID
    }
}
