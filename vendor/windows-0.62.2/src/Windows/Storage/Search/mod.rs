#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CommonFileQuery(pub i32);
impl CommonFileQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const OrderByName: Self = Self(1i32);
    pub const OrderByTitle: Self = Self(2i32);
    pub const OrderByMusicProperties: Self = Self(3i32);
    pub const OrderBySearchRank: Self = Self(4i32);
    pub const OrderByDate: Self = Self(5i32);
}
impl windows_core::TypeKind for CommonFileQuery {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CommonFileQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFileQuery;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CommonFolderQuery(pub i32);
impl CommonFolderQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const GroupByYear: Self = Self(100i32);
    pub const GroupByMonth: Self = Self(101i32);
    pub const GroupByArtist: Self = Self(102i32);
    pub const GroupByAlbum: Self = Self(103i32);
    pub const GroupByAlbumArtist: Self = Self(104i32);
    pub const GroupByComposer: Self = Self(105i32);
    pub const GroupByGenre: Self = Self(106i32);
    pub const GroupByPublishedYear: Self = Self(107i32);
    pub const GroupByRating: Self = Self(108i32);
    pub const GroupByTag: Self = Self(109i32);
    pub const GroupByAuthor: Self = Self(110i32);
    pub const GroupByType: Self = Self(111i32);
}
impl windows_core::TypeKind for CommonFolderQuery {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CommonFolderQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFolderQuery;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentIndexer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContentIndexer, windows_core::IUnknown, windows_core::IInspectable);
impl ContentIndexer {
    pub fn AddAsync<P0>(&self, indexablecontent: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IIndexableContent>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddAsync)(windows_core::Interface::as_raw(this), indexablecontent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateAsync<P0>(&self, indexablecontent: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IIndexableContent>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateAsync)(windows_core::Interface::as_raw(this), indexablecontent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self, contentid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteMultipleAsync<P0>(&self, contentids: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteMultipleAsync)(windows_core::Interface::as_raw(this), contentids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAllAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAllAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetrievePropertiesAsync<P1>(&self, contentid: &windows_core::HSTRING, propertiestoretrieve: P1) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrievePropertiesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentid), propertiestoretrieve.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Revision(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Revision)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateQueryWithSortOrderAndLanguage<P1, P2>(&self, searchfilter: &windows_core::HSTRING, propertiestoretrieve: P1, sortorder: P2, searchfilterlanguage: &windows_core::HSTRING) -> windows_core::Result<ContentIndexerQuery>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P2: windows_core::Param<windows_collections::IIterable<SortEntry>>,
    {
        let this = &windows_core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateQueryWithSortOrderAndLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(searchfilter), propertiestoretrieve.param().abi(), sortorder.param().abi(), core::mem::transmute_copy(searchfilterlanguage), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateQueryWithSortOrder<P1, P2>(&self, searchfilter: &windows_core::HSTRING, propertiestoretrieve: P1, sortorder: P2) -> windows_core::Result<ContentIndexerQuery>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P2: windows_core::Param<windows_collections::IIterable<SortEntry>>,
    {
        let this = &windows_core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateQueryWithSortOrder)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(searchfilter), propertiestoretrieve.param().abi(), sortorder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateQuery<P1>(&self, searchfilter: &windows_core::HSTRING, propertiestoretrieve: P1) -> windows_core::Result<ContentIndexerQuery>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateQuery)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(searchfilter), propertiestoretrieve.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIndexerWithName(indexname: &windows_core::HSTRING) -> windows_core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndexerWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(indexname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetIndexer() -> windows_core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndexer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IContentIndexerStatics<R, F: FnOnce(&IContentIndexerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentIndexer, IContentIndexerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ContentIndexer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContentIndexer>();
}
unsafe impl windows_core::Interface for ContentIndexer {
    type Vtable = <IContentIndexer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentIndexer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContentIndexer {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexer";
}
unsafe impl Send for ContentIndexer {}
unsafe impl Sync for ContentIndexer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentIndexerQuery(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContentIndexerQuery, windows_core::IUnknown, windows_core::IInspectable);
impl ContentIndexerQuery {
    pub fn GetCountAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertiesRangeAsync(&self, startindex: u32, maxitems: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertiesRangeAsync)(windows_core::Interface::as_raw(this), startindex, maxitems, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRangeAsync(&self, startindex: u32, maxitems: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRangeAsync)(windows_core::Interface::as_raw(this), startindex, maxitems, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn QueryFolder(&self) -> windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryFolder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContentIndexerQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContentIndexerQuery>();
}
unsafe impl windows_core::Interface for ContentIndexerQuery {
    type Vtable = <IContentIndexerQuery as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContentIndexerQuery as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContentIndexerQuery {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexerQuery";
}
unsafe impl Send for ContentIndexerQuery {}
unsafe impl Sync for ContentIndexerQuery {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DateStackOption(pub i32);
impl DateStackOption {
    pub const None: Self = Self(0i32);
    pub const Year: Self = Self(1i32);
    pub const Month: Self = Self(2i32);
}
impl windows_core::TypeKind for DateStackOption {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DateStackOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.DateStackOption;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FolderDepth(pub i32);
impl FolderDepth {
    pub const Shallow: Self = Self(0i32);
    pub const Deep: Self = Self(1i32);
}
impl windows_core::TypeKind for FolderDepth {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FolderDepth {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.FolderDepth;i4)");
}
windows_core::imp::define_interface!(IContentIndexer, IContentIndexer_Vtbl, 0xb1767f8d_f698_4982_b05f_3a6e8cab01a2);
impl windows_core::RuntimeType for IContentIndexer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteMultipleAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetrievePropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContentIndexerQuery, IContentIndexerQuery_Vtbl, 0x70e3b0f8_4bfc_428a_8889_cc51da9a7b9d);
impl windows_core::RuntimeType for IContentIndexerQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQuery_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertiesRangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContentIndexerQueryOperations, IContentIndexerQueryOperations_Vtbl, 0x28823e10_4786_42f1_9730_792b3566b150);
impl windows_core::RuntimeType for IContentIndexerQueryOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQueryOperations_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateQueryWithSortOrderAndLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQueryWithSortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContentIndexerStatics, IContentIndexerStatics_Vtbl, 0x8c488375_b37e_4c60_9ba8_b760fda3e59d);
impl windows_core::RuntimeType for IContentIndexerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetIndexerWithName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndexer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIndexableContent, IIndexableContent_Vtbl, 0xccf1a05f_d4b5_483a_b06e_e0db1ec420e4);
impl windows_core::RuntimeType for IIndexableContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IIndexableContent, windows_core::IUnknown, windows_core::IInspectable);
impl IIndexableContent {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> windows_core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStream)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StreamContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetStreamContentType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStreamContentType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IIndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IIndexableContent";
}
#[cfg(feature = "Storage_Streams")]
pub trait IIndexableContent_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>;
    fn Stream(&self) -> windows_core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(&self, value: windows_core::Ref<super::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn StreamContentType(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetStreamContentType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage_Streams")]
impl IIndexableContent_Vtbl {
    pub const fn new<Identity: IIndexableContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IIndexableContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn SetId<Identity: IIndexableContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIndexableContent_Impl::SetId(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IIndexableContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn Stream<Identity: IIndexableContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn SetStream<Identity: IIndexableContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIndexableContent_Impl::SetStream(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StreamContentType<Identity: IIndexableContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn SetStreamContentType<Identity: IIndexableContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIndexableContent_Impl::SetStreamContentType(this, core::mem::transmute(&value)).into()
            }
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
#[repr(C)]
#[doc(hidden)]
pub struct IIndexableContent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    pub StreamContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStreamContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IQueryOptions, IQueryOptions_Vtbl, 0x1e5e46ee_0f45_4838_a8e9_d0479d446c30);
impl windows_core::RuntimeType for IQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FileTypeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FolderDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FolderDepth) -> windows_core::HRESULT,
    pub SetFolderDepth: unsafe extern "system" fn(*mut core::ffi::c_void, FolderDepth) -> windows_core::HRESULT,
    pub ApplicationSearchFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationSearchFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserSearchFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserSearchFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IndexerOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IndexerOption) -> windows_core::HRESULT,
    pub SetIndexerOption: unsafe extern "system" fn(*mut core::ffi::c_void, IndexerOption) -> windows_core::HRESULT,
    pub SortOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GroupPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DateStackOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DateStackOption) -> windows_core::HRESULT,
    pub SaveToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadFromString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub SetThumbnailPrefetch: unsafe extern "system" fn(*mut core::ffi::c_void, super::FileProperties::ThumbnailMode, u32, super::FileProperties::ThumbnailOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    SetThumbnailPrefetch: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub SetPropertyPrefetch: unsafe extern "system" fn(*mut core::ffi::c_void, super::FileProperties::PropertyPrefetchOptions, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    SetPropertyPrefetch: usize,
}
windows_core::imp::define_interface!(IQueryOptionsFactory, IQueryOptionsFactory_Vtbl, 0x032e1f8c_a9c1_4e71_8011_0dee9d4811a3);
impl windows_core::RuntimeType for IQueryOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCommonFileQuery: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFileQuery, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCommonFolderQuery: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFolderQuery, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IQueryOptionsWithProviderFilter, IQueryOptionsWithProviderFilter_Vtbl, 0x5b9d1026_15c4_44dd_b89a_47a59b7d7c4f);
impl windows_core::RuntimeType for IQueryOptionsWithProviderFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsWithProviderFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StorageProviderIdFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageFileQueryResult, IStorageFileQueryResult_Vtbl, 0x52fda447_2baa_412c_b29f_d4b1778efa1e);
impl windows_core::RuntimeType for IStorageFileQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetFilesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFilesAsyncDefaultStartAndCount: usize,
}
windows_core::imp::define_interface!(IStorageFileQueryResult2, IStorageFileQueryResult2_Vtbl, 0x4e5db9dd_7141_46c4_8be3_e9dc9e27275c);
impl windows_core::RuntimeType for IStorageFileQueryResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Data_Text", feature = "Storage_Streams"))]
    pub GetMatchingPropertiesWithRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Storage_Streams")))]
    GetMatchingPropertiesWithRanges: usize,
}
windows_core::imp::define_interface!(IStorageFolderQueryOperations, IStorageFolderQueryOperations_Vtbl, 0xcb43ccc9_446b_4a4f_be97_757771be5203);
impl windows_core::RuntimeType for IStorageFolderQueryOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageFolderQueryOperations, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageFolderQueryOperations {
    pub fn GetIndexedStateAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IndexedState>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndexedStateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFileQueryOverloadDefault(&self) -> windows_core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileQueryOverloadDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFileQuery(&self, query: CommonFileQuery) -> windows_core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileQuery)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFileQueryWithOptions<P0>(&self, queryoptions: P0) -> windows_core::Result<StorageFileQueryResult>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileQueryWithOptions)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderQueryOverloadDefault(&self) -> windows_core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderQuery(&self, query: CommonFolderQuery) -> windows_core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderQuery)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderQueryWithOptions<P0>(&self, queryoptions: P0) -> windows_core::Result<StorageFolderQueryResult>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderQueryWithOptions)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateItemQuery(&self) -> windows_core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateItemQuery)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateItemQueryWithOptions<P0>(&self, queryoptions: P0) -> windows_core::Result<StorageItemQueryResult>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateItemQueryWithOptions)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsync)(windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsync)(windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AreQueryOptionsSupported<P0>(&self, queryoptions: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreQueryOptionsSupported)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCommonFolderQuerySupported)(windows_core::Interface::as_raw(this), query, &mut result__).map(|| result__)
        }
    }
    pub fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCommonFileQuerySupported)(windows_core::Interface::as_raw(this), query, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
#[cfg(feature = "Storage_Streams")]
pub trait IStorageFolderQueryOperations_Impl: windows_core::IUnknownImpl {
    fn GetIndexedStateAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(&self) -> windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(&self, query: CommonFileQuery) -> windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(&self, queryOptions: windows_core::Ref<QueryOptions>) -> windows_core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(&self) -> windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(&self, query: CommonFolderQuery) -> windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(&self, queryOptions: windows_core::Ref<QueryOptions>) -> windows_core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(&self) -> windows_core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(&self, queryOptions: windows_core::Ref<QueryOptions>) -> windows_core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(&self, query: CommonFileQuery, startIndex: u32, maxItemsToRetrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startIndex: u32, maxItemsToRetrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(&self, startIndex: u32, maxItemsToRetrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(&self, queryOptions: windows_core::Ref<QueryOptions>) -> windows_core::Result<bool>;
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> windows_core::Result<bool>;
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> windows_core::Result<bool>;
}
#[cfg(feature = "Storage_Streams")]
impl IStorageFolderQueryOperations_Vtbl {
    pub const fn new<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIndexedStateAsync<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn CreateFileQueryOverloadDefault<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn CreateFileQuery<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn CreateFileQueryWithOptions<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderQueryOperations_Impl::CreateFileQueryWithOptions(this, core::mem::transmute_copy(&queryoptions)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFolderQueryOverloadDefault<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn CreateFolderQuery<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn CreateFolderQueryWithOptions<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderQueryOperations_Impl::CreateFolderQueryWithOptions(this, core::mem::transmute_copy(&queryoptions)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItemQuery<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn CreateItemQueryWithOptions<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderQueryOperations_Impl::CreateItemQueryWithOptions(this, core::mem::transmute_copy(&queryoptions)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilesAsync<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultStartAndCount<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetFoldersAsync<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultStartAndCount<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetItemsAsync<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn AreQueryOptionsSupported<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queryoptions: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderQueryOperations_Impl::AreQueryOptionsSupported(this, core::mem::transmute_copy(&queryoptions)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCommonFolderQuerySupported<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderQueryOperations_Impl::IsCommonFolderQuerySupported(this, query) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCommonFileQuerySupported<Identity: IStorageFolderQueryOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderQueryOperations_Impl::IsCommonFileQuerySupported(this, query) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryOperations_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetIndexedStateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileQueryOverloadDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileQuery: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFileQuery, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileQueryWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFolderQueryOverloadDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFolderQuery: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFolderQuery, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFolderQueryWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateItemQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateItemQueryWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetFilesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFileQuery, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetFilesAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFileQuery, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFilesAsyncOverloadDefaultStartAndCount: usize,
    pub GetFoldersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFolderQuery, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFoldersAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFolderQuery, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AreQueryOptionsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCommonFolderQuerySupported: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFolderQuery, *mut bool) -> windows_core::HRESULT,
    pub IsCommonFileQuerySupported: unsafe extern "system" fn(*mut core::ffi::c_void, CommonFileQuery, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageFolderQueryResult, IStorageFolderQueryResult_Vtbl, 0x6654c911_7d66_46fa_aecf_e4a4baa93ab8);
impl windows_core::RuntimeType for IStorageFolderQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFoldersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemQueryResult, IStorageItemQueryResult_Vtbl, 0xe8948079_9d58_47b8_b2b2_41b07f4795f9);
impl windows_core::RuntimeType for IStorageItemQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemQueryResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageLibraryChangeTrackerTriggerDetails, IStorageLibraryChangeTrackerTriggerDetails_Vtbl, 0x1dc7a369_b7a3_4df2_9d61_eba85a0343d2);
impl windows_core::RuntimeType for IStorageLibraryChangeTrackerTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Folder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageLibraryContentChangedTriggerDetails, IStorageLibraryContentChangedTriggerDetails_Vtbl, 0x2a371977_abbf_4e1d_8aa5_6385d8884799);
impl windows_core::RuntimeType for IStorageLibraryContentChangedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Folder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateModifiedSinceQuery: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageQueryResultBase, IStorageQueryResultBase_Vtbl, 0xc297d70d_7353_47ab_ba58_8c61425dc54b);
impl windows_core::RuntimeType for IStorageQueryResultBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStorageQueryResultBase, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageQueryResultBase {
    pub fn GetItemCountAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Folder(&self) -> windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContentsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionsChanged)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOptionsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOptionsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> windows_core::Result<windows_future::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindStartIndexAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> windows_core::Result<QueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentQueryOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ApplyNewQueryOptions)(windows_core::Interface::as_raw(this), newqueryoptions.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for IStorageQueryResultBase {
    const NAME: &'static str = "Windows.Storage.Search.IStorageQueryResultBase";
}
pub trait IStorageQueryResultBase_Impl: windows_core::IUnknownImpl {
    fn GetItemCountAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<u32>>;
    fn Folder(&self) -> windows_core::Result<super::StorageFolder>;
    fn ContentsChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveContentsChanged(&self, eventCookie: i64) -> windows_core::Result<()>;
    fn OptionsChanged(&self, changedHandler: windows_core::Ref<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveOptionsChanged(&self, eventCookie: i64) -> windows_core::Result<()>;
    fn FindStartIndexAsync(&self, value: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<windows_future::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(&self) -> windows_core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(&self, newQueryOptions: windows_core::Ref<QueryOptions>) -> windows_core::Result<()>;
}
impl IStorageQueryResultBase_Vtbl {
    pub const fn new<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemCountAsync<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn Folder<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn ContentsChanged<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageQueryResultBase_Impl::ContentsChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveContentsChanged<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageQueryResultBase_Impl::RemoveContentsChanged(this, eventcookie).into()
            }
        }
        unsafe extern "system" fn OptionsChanged<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changedhandler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageQueryResultBase_Impl::OptionsChanged(this, core::mem::transmute_copy(&changedhandler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveOptionsChanged<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageQueryResultBase_Impl::RemoveOptionsChanged(this, eventcookie).into()
            }
        }
        unsafe extern "system" fn FindStartIndexAsync<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageQueryResultBase_Impl::FindStartIndexAsync(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn ApplyNewQueryOptions<Identity: IStorageQueryResultBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newqueryoptions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageQueryResultBase_Impl::ApplyNewQueryOptions(this, core::mem::transmute_copy(&newqueryoptions)).into()
            }
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
#[repr(C)]
#[doc(hidden)]
pub struct IStorageQueryResultBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetItemCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Folder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveContentsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub OptionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveOptionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FindStartIndexAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentQueryOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyNewQueryOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IValueAndLanguage, IValueAndLanguage_Vtbl, 0xb9914881_a1ee_4bc4_92a5_466968e30436);
impl windows_core::RuntimeType for IValueAndLanguage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueAndLanguage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexableContent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IndexableContent, windows_core::IUnknown, windows_core::IInspectable, IIndexableContent);
impl IndexableContent {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IndexableContent, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> windows_core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStream)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StreamContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetStreamContentType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStreamContentType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for IndexableContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIndexableContent>();
}
unsafe impl windows_core::Interface for IndexableContent {
    type Vtable = <IIndexableContent as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IIndexableContent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IndexableContent";
}
unsafe impl Send for IndexableContent {}
unsafe impl Sync for IndexableContent {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IndexedState(pub i32);
impl IndexedState {
    pub const Unknown: Self = Self(0i32);
    pub const NotIndexed: Self = Self(1i32);
    pub const PartiallyIndexed: Self = Self(2i32);
    pub const FullyIndexed: Self = Self(3i32);
}
impl windows_core::TypeKind for IndexedState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IndexedState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexedState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IndexerOption(pub i32);
impl IndexerOption {
    pub const UseIndexerWhenAvailable: Self = Self(0i32);
    pub const OnlyUseIndexer: Self = Self(1i32);
    pub const DoNotUseIndexer: Self = Self(2i32);
    pub const OnlyUseIndexerAndOptimizeForIndexedProperties: Self = Self(3i32);
}
impl windows_core::TypeKind for IndexerOption {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for IndexerOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexerOption;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(QueryOptions, windows_core::IUnknown, windows_core::IInspectable);
impl QueryOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<QueryOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn FileTypeFilter(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileTypeFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FolderDepth(&self) -> windows_core::Result<FolderDepth> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderDepth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFolderDepth(&self, value: FolderDepth) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFolderDepth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplicationSearchFilter(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationSearchFilter)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetApplicationSearchFilter(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetApplicationSearchFilter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserSearchFilter(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserSearchFilter)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetUserSearchFilter(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUserSearchFilter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IndexerOption(&self) -> windows_core::Result<IndexerOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexerOption)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIndexerOption(&self, value: IndexerOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIndexerOption)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SortOrder(&self) -> windows_core::Result<windows_collections::IVector<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SortOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GroupPropertyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GroupPropertyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DateStackOption(&self) -> windows_core::Result<DateStackOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateStackOption)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SaveToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LoadFromString(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LoadFromString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn SetThumbnailPrefetch(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnailPrefetch)(windows_core::Interface::as_raw(this), mode, requestedsize, options).ok() }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn SetPropertyPrefetch<P1>(&self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPropertyPrefetch)(windows_core::Interface::as_raw(this), options, propertiestoretrieve.param().abi()).ok() }
    }
    pub fn CreateCommonFileQuery<P1>(query: CommonFileQuery, filetypefilter: P1) -> windows_core::Result<QueryOptions>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCommonFileQuery)(windows_core::Interface::as_raw(this), query, filetypefilter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCommonFolderQuery(query: CommonFolderQuery) -> windows_core::Result<QueryOptions> {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCommonFolderQuery)(windows_core::Interface::as_raw(this), query, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn StorageProviderIdFilter(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IQueryOptionsWithProviderFilter>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StorageProviderIdFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IQueryOptionsFactory<R, F: FnOnce(&IQueryOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<QueryOptions, IQueryOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for QueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IQueryOptions>();
}
unsafe impl windows_core::Interface for QueryOptions {
    type Vtable = <IQueryOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IQueryOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for QueryOptions {
    const NAME: &'static str = "Windows.Storage.Search.QueryOptions";
}
unsafe impl Send for QueryOptions {}
unsafe impl Sync for QueryOptions {}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SortEntry {
    pub PropertyName: windows_core::HSTRING,
    pub AscendingOrder: bool,
}
impl windows_core::TypeKind for SortEntry {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for SortEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Storage.Search.SortEntry;string;b1)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SortEntryVector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SortEntryVector, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IVector<SortEntry>);
windows_core::imp::required_hierarchy!(SortEntryVector, windows_collections::IIterable<SortEntry>);
impl SortEntryVector {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<SortEntry>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<SortEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<SortEntry> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf(&self, value: &SortEntry, index: &mut u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt(&self, index: u32, value: &SortEntry) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, core::mem::transmute_copy(value)).ok() }
    }
    pub fn InsertAt(&self, index: u32, value: &SortEntry) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append(&self, value: &SortEntry) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [SortEntry]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[SortEntry]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for SortEntryVector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<SortEntry>>();
}
unsafe impl windows_core::Interface for SortEntryVector {
    type Vtable = <windows_collections::IVector<SortEntry> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IVector<SortEntry> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SortEntryVector {
    const NAME: &'static str = "Windows.Storage.Search.SortEntryVector";
}
impl IntoIterator for SortEntryVector {
    type Item = SortEntry;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &SortEntryVector {
    type Item = SortEntry;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageFileQueryResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageFileQueryResult, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageFileQueryResult, IStorageQueryResultBase);
impl StorageFileQueryResult {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsync(&self, startindex: u32, maxnumberofitems: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsync)(windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Data_Text", feature = "Storage_Streams"))]
    pub fn GetMatchingPropertiesWithRanges<P0>(&self, file: P0) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_collections::IVectorView<super::super::Data::Text::TextSegment>>>
    where
        P0: windows_core::Param<super::StorageFile>,
    {
        let this = &windows_core::Interface::cast::<IStorageFileQueryResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMatchingPropertiesWithRanges)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemCountAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<u32>> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Folder(&self) -> windows_core::Result<super::StorageFolder> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContentsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionsChanged)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOptionsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOptionsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> windows_core::Result<windows_future::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindStartIndexAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> windows_core::Result<QueryOptions> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentQueryOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ApplyNewQueryOptions)(windows_core::Interface::as_raw(this), newqueryoptions.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageFileQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageFileQueryResult>();
}
unsafe impl windows_core::Interface for StorageFileQueryResult {
    type Vtable = <IStorageFileQueryResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageFileQueryResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageFileQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFileQueryResult";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageFolderQueryResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageFolderQueryResult, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageFolderQueryResult, IStorageQueryResultBase);
impl StorageFolderQueryResult {
    pub fn GetFoldersAsync(&self, startindex: u32, maxnumberofitems: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsync)(windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemCountAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<u32>> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Folder(&self) -> windows_core::Result<super::StorageFolder> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContentsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionsChanged)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOptionsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOptionsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> windows_core::Result<windows_future::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindStartIndexAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> windows_core::Result<QueryOptions> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentQueryOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ApplyNewQueryOptions)(windows_core::Interface::as_raw(this), newqueryoptions.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageFolderQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageFolderQueryResult>();
}
unsafe impl windows_core::Interface for StorageFolderQueryResult {
    type Vtable = <IStorageFolderQueryResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageFolderQueryResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageFolderQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFolderQueryResult";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageItemQueryResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageItemQueryResult, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageItemQueryResult, IStorageQueryResultBase);
impl StorageItemQueryResult {
    pub fn GetItemsAsync(&self, startindex: u32, maxnumberofitems: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemCountAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<u32>> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Folder(&self) -> windows_core::Result<super::StorageFolder> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContentsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn OptionsChanged<P0>(&self, changedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionsChanged)(windows_core::Interface::as_raw(this), changedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOptionsChanged(&self, eventcookie: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOptionsChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn FindStartIndexAsync<P0>(&self, value: P0) -> windows_core::Result<windows_future::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindStartIndexAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> windows_core::Result<QueryOptions> {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentQueryOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplyNewQueryOptions<P0>(&self, newqueryoptions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<QueryOptions>,
    {
        let this = &windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ApplyNewQueryOptions)(windows_core::Interface::as_raw(this), newqueryoptions.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageItemQueryResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageItemQueryResult>();
}
unsafe impl windows_core::Interface for StorageItemQueryResult {
    type Vtable = <IStorageItemQueryResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageItemQueryResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageItemQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageItemQueryResult";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageLibraryChangeTrackerTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageLibraryChangeTrackerTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl StorageLibraryChangeTrackerTriggerDetails {
    pub fn Folder(&self) -> windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChangeTracker(&self) -> windows_core::Result<super::StorageLibraryChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeTracker)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageLibraryChangeTrackerTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageLibraryChangeTrackerTriggerDetails>();
}
unsafe impl windows_core::Interface for StorageLibraryChangeTrackerTriggerDetails {
    type Vtable = <IStorageLibraryChangeTrackerTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageLibraryChangeTrackerTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageLibraryChangeTrackerTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryChangeTrackerTriggerDetails";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageLibraryContentChangedTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageLibraryContentChangedTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl StorageLibraryContentChangedTriggerDetails {
    pub fn Folder(&self) -> windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Folder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateModifiedSinceQuery(&self, lastquerytime: super::super::Foundation::DateTime) -> windows_core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateModifiedSinceQuery)(windows_core::Interface::as_raw(this), lastquerytime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageLibraryContentChangedTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageLibraryContentChangedTriggerDetails>();
}
unsafe impl windows_core::Interface for StorageLibraryContentChangedTriggerDetails {
    type Vtable = <IStorageLibraryContentChangedTriggerDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageLibraryContentChangedTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageLibraryContentChangedTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryContentChangedTriggerDetails";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValueAndLanguage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ValueAndLanguage, windows_core::IUnknown, windows_core::IInspectable);
impl ValueAndLanguage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ValueAndLanguage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for ValueAndLanguage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IValueAndLanguage>();
}
unsafe impl windows_core::Interface for ValueAndLanguage {
    type Vtable = <IValueAndLanguage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IValueAndLanguage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ValueAndLanguage {
    const NAME: &'static str = "Windows.Storage.Search.ValueAndLanguage";
}
unsafe impl Send for ValueAndLanguage {}
unsafe impl Sync for ValueAndLanguage {}
