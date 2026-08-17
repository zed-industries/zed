#[cfg(feature = "Foundation_Collections")]
pub trait IMediaExtension_Impl: Sized {
    fn SetProperties(&self, configuration: Option<&super::Foundation::Collections::IPropertySet>) -> windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IMediaExtension {
    const NAME: &'static str = "Windows.Media.IMediaExtension";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaExtension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaExtension_Vtbl
    where
        Identity: IMediaExtension_Impl,
    {
        unsafe extern "system" fn SetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaExtension_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaExtension_Impl::SetProperties(this, windows_core::from_raw_borrowed(&configuration)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaExtension, OFFSET>(), SetProperties: SetProperties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaExtension as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaFrame_Impl: Sized + super::Foundation::IClosable_Impl {
    fn Type(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn IsReadOnly(&self) -> windows_core::Result<bool>;
    fn SetRelativeTime(&self, value: Option<&super::Foundation::IReference<super::Foundation::TimeSpan>>) -> windows_core::Result<()>;
    fn RelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetSystemRelativeTime(&self, value: Option<&super::Foundation::IReference<super::Foundation::TimeSpan>>) -> windows_core::Result<()>;
    fn SystemRelativeTime(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: Option<&super::Foundation::IReference<super::Foundation::TimeSpan>>) -> windows_core::Result<()>;
    fn Duration(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>>;
    fn SetIsDiscontinuous(&self, value: bool) -> windows_core::Result<()>;
    fn IsDiscontinuous(&self) -> windows_core::Result<bool>;
    fn ExtendedProperties(&self) -> windows_core::Result<super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IMediaFrame {
    const NAME: &'static str = "Windows.Media.IMediaFrame";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaFrame_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaFrame_Vtbl
    where
        Identity: IMediaFrame_Impl,
    {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaFrame_Impl::Type(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaFrame_Impl::IsReadOnly(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelativeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaFrame_Impl::SetRelativeTime(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RelativeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaFrame_Impl::RelativeTime(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemRelativeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaFrame_Impl::SetSystemRelativeTime(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SystemRelativeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaFrame_Impl::SystemRelativeTime(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaFrame_Impl::SetDuration(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Duration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaFrame_Impl::Duration(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDiscontinuous<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMediaFrame_Impl::SetIsDiscontinuous(this, value).into()
        }
        unsafe extern "system" fn IsDiscontinuous<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaFrame_Impl::IsDiscontinuous(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaFrame_Impl::ExtendedProperties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaFrame, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, OFFSET>,
            SetRelativeTime: SetRelativeTime::<Identity, OFFSET>,
            RelativeTime: RelativeTime::<Identity, OFFSET>,
            SetSystemRelativeTime: SetSystemRelativeTime::<Identity, OFFSET>,
            SystemRelativeTime: SystemRelativeTime::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetIsDiscontinuous: SetIsDiscontinuous::<Identity, OFFSET>,
            IsDiscontinuous: IsDiscontinuous::<Identity, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaFrame as windows_core::Interface>::IID
    }
}
pub trait IMediaMarker_Impl: Sized {
    fn Time(&self) -> windows_core::Result<super::Foundation::TimeSpan>;
    fn MediaMarkerType(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Text(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IMediaMarker {
    const NAME: &'static str = "Windows.Media.IMediaMarker";
}
impl IMediaMarker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaMarker_Vtbl
    where
        Identity: IMediaMarker_Impl,
    {
        unsafe extern "system" fn Time<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> windows_core::HRESULT
        where
            Identity: IMediaMarker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaMarker_Impl::Time(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaMarkerType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IMediaMarker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaMarker_Impl::MediaMarkerType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IMediaMarker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaMarker_Impl::Text(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaMarker, OFFSET>(),
            Time: Time::<Identity, OFFSET>,
            MediaMarkerType: MediaMarkerType::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaMarker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaMarkers_Impl: Sized {
    fn Markers(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IMediaMarkers {
    const NAME: &'static str = "Windows.Media.IMediaMarkers";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaMarkers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMediaMarkers_Vtbl
    where
        Identity: IMediaMarkers_Impl,
    {
        unsafe extern "system" fn Markers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMediaMarkers_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMediaMarkers_Impl::Markers(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaMarkers, OFFSET>(), Markers: Markers::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaMarkers as windows_core::Interface>::IID
    }
}
