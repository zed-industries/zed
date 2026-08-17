#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemInformation_Impl: Sized {
    fn MusicProperties(&self) -> windows_core::Result<super::FileProperties::MusicProperties>;
    fn VideoProperties(&self) -> windows_core::Result<super::FileProperties::VideoProperties>;
    fn ImageProperties(&self) -> windows_core::Result<super::FileProperties::ImageProperties>;
    fn DocumentProperties(&self) -> windows_core::Result<super::FileProperties::DocumentProperties>;
    fn BasicProperties(&self) -> windows_core::Result<super::FileProperties::BasicProperties>;
    fn Thumbnail(&self) -> windows_core::Result<super::FileProperties::StorageItemThumbnail>;
    fn ThumbnailUpdated(&self, changedhandler: Option<&super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThumbnailUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PropertiesUpdated(&self, changedhandler: Option<&super::super::Foundation::TypedEventHandler<IStorageItemInformation, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesUpdated(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IStorageItemInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.IStorageItemInformation";
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>() -> IStorageItemInformation_Vtbl {
        unsafe extern "system" fn MusicProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::MusicProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::VideoProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::ImageProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::DocumentProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BasicProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::BasicProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thumbnail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::Thumbnail(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThumbnailUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changedhandler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::ThumbnailUpdated(this, windows_core::from_raw_borrowed(&changedhandler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveThumbnailUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorageItemInformation_Impl::RemoveThumbnailUpdated(this, core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn PropertiesUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changedhandler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStorageItemInformation_Impl::PropertiesUpdated(this, windows_core::from_raw_borrowed(&changedhandler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertiesUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStorageItemInformation_Impl::RemovePropertiesUpdated(this, core::mem::transmute(&eventcookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemInformation, OFFSET>(),
            MusicProperties: MusicProperties::<Identity, Impl, OFFSET>,
            VideoProperties: VideoProperties::<Identity, Impl, OFFSET>,
            ImageProperties: ImageProperties::<Identity, Impl, OFFSET>,
            DocumentProperties: DocumentProperties::<Identity, Impl, OFFSET>,
            BasicProperties: BasicProperties::<Identity, Impl, OFFSET>,
            Thumbnail: Thumbnail::<Identity, Impl, OFFSET>,
            ThumbnailUpdated: ThumbnailUpdated::<Identity, Impl, OFFSET>,
            RemoveThumbnailUpdated: RemoveThumbnailUpdated::<Identity, Impl, OFFSET>,
            PropertiesUpdated: PropertiesUpdated::<Identity, Impl, OFFSET>,
            RemovePropertiesUpdated: RemovePropertiesUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageItemInformation as windows_core::Interface>::IID
    }
}
