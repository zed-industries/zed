#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IContactInformation_Impl: Sized {
    fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn FamilyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetFamilyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn GivenName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetGivenName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn HonorificPrefix(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetHonorificPrefix(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn HonorificSuffix(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetHonorificSuffix(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn GetDisplayPictureAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SetDisplayPictureAsync(&self, stream: Option<&super::super::Storage::Streams::IInputStream>) -> windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn DisplayPicture(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetPropertiesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>>;
    fn ToVcardAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ToVcardWithOptionsAsync(&self, format: VCardFormat) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IContactInformation {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IContactInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactInformation_Vtbl
    where
        Identity: IContactInformation_Impl,
    {
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::DisplayName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactInformation_Impl::SetDisplayName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn FamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::FamilyName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactInformation_Impl::SetFamilyName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GivenName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::GivenName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGivenName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactInformation_Impl::SetGivenName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn HonorificPrefix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::HonorificPrefix(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificPrefix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactInformation_Impl::SetHonorificPrefix(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn HonorificSuffix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::HonorificSuffix(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHonorificSuffix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactInformation_Impl::SetHonorificSuffix(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetDisplayPictureAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::GetDisplayPictureAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::SetDisplayPictureAsync(this, windows_core::from_raw_borrowed(&stream)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayPicture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::DisplayPicture(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::GetPropertiesAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::ToVcardAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToVcardWithOptionsAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: VCardFormat, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation_Impl::ToVcardWithOptionsAsync(this, format) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactInformation, OFFSET>(),
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            FamilyName: FamilyName::<Identity, OFFSET>,
            SetFamilyName: SetFamilyName::<Identity, OFFSET>,
            GivenName: GivenName::<Identity, OFFSET>,
            SetGivenName: SetGivenName::<Identity, OFFSET>,
            HonorificPrefix: HonorificPrefix::<Identity, OFFSET>,
            SetHonorificPrefix: SetHonorificPrefix::<Identity, OFFSET>,
            HonorificSuffix: HonorificSuffix::<Identity, OFFSET>,
            SetHonorificSuffix: SetHonorificSuffix::<Identity, OFFSET>,
            GetDisplayPictureAsync: GetDisplayPictureAsync::<Identity, OFFSET>,
            SetDisplayPictureAsync: SetDisplayPictureAsync::<Identity, OFFSET>,
            DisplayPicture: DisplayPicture::<Identity, OFFSET>,
            GetPropertiesAsync: GetPropertiesAsync::<Identity, OFFSET>,
            ToVcardAsync: ToVcardAsync::<Identity, OFFSET>,
            ToVcardWithOptionsAsync: ToVcardWithOptionsAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactInformation as windows_core::Interface>::IID
    }
}
pub trait IContactInformation2_Impl: Sized {
    fn DisplayPictureDate(&self) -> windows_core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureDate(&self, returnvalue: &super::super::Foundation::DateTime) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContactInformation2 {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.IContactInformation2";
}
impl IContactInformation2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactInformation2_Vtbl
    where
        Identity: IContactInformation2_Impl,
    {
        unsafe extern "system" fn DisplayPictureDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT
        where
            Identity: IContactInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactInformation2_Impl::DisplayPictureDate(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayPictureDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, returnvalue: super::super::Foundation::DateTime) -> windows_core::HRESULT
        where
            Identity: IContactInformation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactInformation2_Impl::SetDisplayPictureDate(this, core::mem::transmute(&returnvalue)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactInformation2, OFFSET>(),
            DisplayPictureDate: DisplayPictureDate::<Identity, OFFSET>,
            SetDisplayPictureDate: SetDisplayPictureDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactInformation2 as windows_core::Interface>::IID
    }
}
