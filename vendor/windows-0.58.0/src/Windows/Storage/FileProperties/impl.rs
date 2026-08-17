#[cfg(feature = "Foundation_Collections")]
pub trait IStorageItemExtraProperties_Impl: Sized {
    fn RetrievePropertiesAsync(&self, propertiestoretrieve: Option<&super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>>;
    fn SavePropertiesAsync(&self, propertiestosave: Option<&super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>) -> windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn SavePropertiesAsyncOverloadDefault(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IStorageItemExtraProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.IStorageItemExtraProperties";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageItemExtraProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStorageItemExtraProperties_Vtbl
    where
        Identity: IStorageItemExtraProperties_Impl,
    {
        unsafe extern "system" fn RetrievePropertiesAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiestoretrieve: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageItemExtraProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageItemExtraProperties_Impl::RetrievePropertiesAsync(this, windows_core::from_raw_borrowed(&propertiestoretrieve)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiestosave: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageItemExtraProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageItemExtraProperties_Impl::SavePropertiesAsync(this, windows_core::from_raw_borrowed(&propertiestosave)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavePropertiesAsyncOverloadDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageItemExtraProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageItemExtraProperties_Impl::SavePropertiesAsyncOverloadDefault(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemExtraProperties, OFFSET>(),
            RetrievePropertiesAsync: RetrievePropertiesAsync::<Identity, OFFSET>,
            SavePropertiesAsync: SavePropertiesAsync::<Identity, OFFSET>,
            SavePropertiesAsyncOverloadDefault: SavePropertiesAsyncOverloadDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageItemExtraProperties as windows_core::Interface>::IID
    }
}
