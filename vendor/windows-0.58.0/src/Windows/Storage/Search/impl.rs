#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IIndexableContent_Impl: Sized {
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>;
    fn Stream(&self) -> windows_core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&self, value: Option<&super::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn StreamContentType(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetStreamContentType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IIndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IIndexableContent";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl IIndexableContent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIndexableContent_Vtbl
    where
        Identity: IIndexableContent_Impl,
    {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IIndexableContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIndexableContent_Impl::Id(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IIndexableContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIndexableContent_Impl::SetId(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIndexableContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIndexableContent_Impl::Properties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIndexableContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIndexableContent_Impl::Stream(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIndexableContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIndexableContent_Impl::SetStream(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn StreamContentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IIndexableContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIndexableContent_Impl::StreamContentType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamContentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IIndexableContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIndexableContent_Impl::SetStreamContentType(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIndexableContent, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
            SetStream: SetStream::<Identity, OFFSET>,
            StreamContentType: StreamContentType::<Identity, OFFSET>,
            SetStreamContentType: SetStreamContentType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIndexableContent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageFolderQueryOperations_Impl: Sized {
    fn GetIndexedStateAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&self) -> windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&self, query: CommonFileQuery) -> windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&self, queryoptions: Option<&QueryOptions>) -> windows_core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&self) -> windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&self, query: CommonFolderQuery) -> windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&self, queryoptions: Option<&QueryOptions>) -> windows_core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&self) -> windows_core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&self, queryoptions: Option<&QueryOptions>) -> windows_core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&self, queryoptions: Option<&QueryOptions>) -> windows_core::Result<bool>;
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> windows_core::Result<bool>;
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> windows_core::Result<bool>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
#[cfg(feature = "Foundation_Collections")]
impl IStorageFolderQueryOperations_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStorageFolderQueryOperations_Vtbl
    where
        Identity: IStorageFolderQueryOperations_Impl,
    {
        unsafe extern "system" fn GetIndexedStateAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::GetIndexedStateAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryOverloadDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateFileQueryOverloadDefault(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateFileQuery(this, query) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileQueryWithOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateFileQueryWithOptions(this, windows_core::from_raw_borrowed(&queryoptions)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryOverloadDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateFolderQueryOverloadDefault(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateFolderQuery(this, query) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderQueryWithOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateFolderQueryWithOptions(this, windows_core::from_raw_borrowed(&queryoptions)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateItemQuery(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItemQueryWithOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::CreateItemQueryWithOptions(this, windows_core::from_raw_borrowed(&queryoptions)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::GetFilesAsync(this, query, startindex, maxitemstoretrieve) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultStartAndCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::GetFilesAsyncOverloadDefaultStartAndCount(this, query) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::GetFoldersAsync(this, query, startindex, maxitemstoretrieve) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultStartAndCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::GetFoldersAsyncOverloadDefaultStartAndCount(this, query) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::GetItemsAsync(this, startindex, maxitemstoretrieve) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreQueryOptionsSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::AreQueryOptionsSupported(this, windows_core::from_raw_borrowed(&queryoptions)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFolderQuerySupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::IsCommonFolderQuerySupported(this, query) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommonFileQuerySupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IStorageFolderQueryOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageFolderQueryOperations_Impl::IsCommonFileQuerySupported(this, query) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageFolderQueryOperations, OFFSET>(),
            GetIndexedStateAsync: GetIndexedStateAsync::<Identity, OFFSET>,
            CreateFileQueryOverloadDefault: CreateFileQueryOverloadDefault::<Identity, OFFSET>,
            CreateFileQuery: CreateFileQuery::<Identity, OFFSET>,
            CreateFileQueryWithOptions: CreateFileQueryWithOptions::<Identity, OFFSET>,
            CreateFolderQueryOverloadDefault: CreateFolderQueryOverloadDefault::<Identity, OFFSET>,
            CreateFolderQuery: CreateFolderQuery::<Identity, OFFSET>,
            CreateFolderQueryWithOptions: CreateFolderQueryWithOptions::<Identity, OFFSET>,
            CreateItemQuery: CreateItemQuery::<Identity, OFFSET>,
            CreateItemQueryWithOptions: CreateItemQueryWithOptions::<Identity, OFFSET>,
            GetFilesAsync: GetFilesAsync::<Identity, OFFSET>,
            GetFilesAsyncOverloadDefaultStartAndCount: GetFilesAsyncOverloadDefaultStartAndCount::<Identity, OFFSET>,
            GetFoldersAsync: GetFoldersAsync::<Identity, OFFSET>,
            GetFoldersAsyncOverloadDefaultStartAndCount: GetFoldersAsyncOverloadDefaultStartAndCount::<Identity, OFFSET>,
            GetItemsAsync: GetItemsAsync::<Identity, OFFSET>,
            AreQueryOptionsSupported: AreQueryOptionsSupported::<Identity, OFFSET>,
            IsCommonFolderQuerySupported: IsCommonFolderQuerySupported::<Identity, OFFSET>,
            IsCommonFileQuerySupported: IsCommonFileQuerySupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageFolderQueryOperations as windows_core::Interface>::IID
    }
}
pub trait IStorageQueryResultBase_Impl: Sized {
    fn GetItemCountAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(&self) -> windows_core::Result<super::StorageFolder>;
    fn ContentsChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn OptionsChanged(&self, changedhandler: Option<&super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn FindStartIndexAsync(&self, value: Option<&windows_core::IInspectable>) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&self) -> windows_core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&self, newqueryoptions: Option<&QueryOptions>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStorageQueryResultBase {
    const NAME: &'static str = "Windows.Storage.Search.IStorageQueryResultBase";
}
impl IStorageQueryResultBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStorageQueryResultBase_Vtbl
    where
        Identity: IStorageQueryResultBase_Impl,
    {
        unsafe extern "system" fn GetItemCountAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageQueryResultBase_Impl::GetItemCountAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageQueryResultBase_Impl::Folder(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageQueryResultBase_Impl::ContentsChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageQueryResultBase_Impl::RemoveContentsChanged(this, core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn OptionsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changedhandler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageQueryResultBase_Impl::OptionsChanged(this, windows_core::from_raw_borrowed(&changedhandler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOptionsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageQueryResultBase_Impl::RemoveOptionsChanged(this, core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn FindStartIndexAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageQueryResultBase_Impl::FindStartIndexAsync(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStorageQueryResultBase_Impl::GetCurrentQueryOptions(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyNewQueryOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newqueryoptions: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStorageQueryResultBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStorageQueryResultBase_Impl::ApplyNewQueryOptions(this, windows_core::from_raw_borrowed(&newqueryoptions)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageQueryResultBase, OFFSET>(),
            GetItemCountAsync: GetItemCountAsync::<Identity, OFFSET>,
            Folder: Folder::<Identity, OFFSET>,
            ContentsChanged: ContentsChanged::<Identity, OFFSET>,
            RemoveContentsChanged: RemoveContentsChanged::<Identity, OFFSET>,
            OptionsChanged: OptionsChanged::<Identity, OFFSET>,
            RemoveOptionsChanged: RemoveOptionsChanged::<Identity, OFFSET>,
            FindStartIndexAsync: FindStartIndexAsync::<Identity, OFFSET>,
            GetCurrentQueryOptions: GetCurrentQueryOptions::<Identity, OFFSET>,
            ApplyNewQueryOptions: ApplyNewQueryOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageQueryResultBase as windows_core::Interface>::IID
    }
}
