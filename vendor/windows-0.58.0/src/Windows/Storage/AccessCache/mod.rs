windows_core::imp::define_interface!(IItemRemovedEventArgs, IItemRemovedEventArgs_Vtbl, 0x59677e5c_55be_4c66_ba66_5eaea79d2631);
impl windows_core::RuntimeType for IItemRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IItemRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemovedEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<AccessListEntry>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageApplicationPermissionsStatics, IStorageApplicationPermissionsStatics_Vtbl, 0x4391dfaa_d033_48f9_8060_3ec847d2e3f1);
impl windows_core::RuntimeType for IStorageApplicationPermissionsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageApplicationPermissionsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FutureAccessList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MostRecentlyUsedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageApplicationPermissionsStatics2, IStorageApplicationPermissionsStatics2_Vtbl, 0x072716ec_aa05_4294_9a11_1a3d04519ad0);
impl windows_core::RuntimeType for IStorageApplicationPermissionsStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageApplicationPermissionsStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetFutureAccessListForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFutureAccessListForUser: usize,
    #[cfg(feature = "System")]
    pub GetMostRecentlyUsedListForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetMostRecentlyUsedListForUser: usize,
}
windows_core::imp::define_interface!(IStorageItemAccessList, IStorageItemAccessList_Vtbl, 0x2caff6ad_de90_47f5_b2c3_dd36c9fdd453);
impl core::ops::Deref for IStorageItemAccessList {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageItemAccessList, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageItemAccessList {
    pub fn AddOverloadDefaultMetadata<P0>(&self, file: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P0>(&self, token: &windows_core::HSTRING, file: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi()).ok() }
    }
    pub fn AddOrReplace<P0>(&self, token: &windows_core::HSTRING, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata)).ok() }
    }
    pub fn GetItemAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFileAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFolderAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFileWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFolderWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Remove(&self, token: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token)).ok() }
    }
    pub fn ContainsItem(&self, token: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsItem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).map(|| result__)
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CheckAccess<P0>(&self, file: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckAccess)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> windows_core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entries)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaximumItemsAllowed(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaximumItemsAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IStorageItemAccessList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageItemAccessList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddOverloadDefaultMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddOrReplaceOverloadDefaultMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddOrReplace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFolderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, AccessCacheOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, AccessCacheOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFolderWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, AccessCacheOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContainsItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Entries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Entries: usize,
    pub MaximumItemsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemMostRecentlyUsedList, IStorageItemMostRecentlyUsedList_Vtbl, 0x016239d5_510d_411e_8cf1_c3d1effa4c33);
impl windows_core::RuntimeType for IStorageItemMostRecentlyUsedList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageItemMostRecentlyUsedList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ItemRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveItemRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemMostRecentlyUsedList2, IStorageItemMostRecentlyUsedList2_Vtbl, 0xda481ea0_ed8d_4731_a1db_e44ee2204093);
impl windows_core::RuntimeType for IStorageItemMostRecentlyUsedList2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageItemMostRecentlyUsedList2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddWithMetadataAndVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, RecentStorageItemVisibility, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddOrReplaceWithMetadataAndVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, RecentStorageItemVisibility) -> windows_core::HRESULT,
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AccessListEntryView(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(AccessListEntryView, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(AccessListEntryView, super::super::Foundation::Collections::IIterable::<AccessListEntry>, super::super::Foundation::Collections::IVectorView::<AccessListEntry>);
#[cfg(feature = "Foundation_Collections")]
impl AccessListEntryView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<AccessListEntry>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<AccessListEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<AccessListEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [AccessListEntry]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for AccessListEntryView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::Collections::IVectorView<AccessListEntry>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for AccessListEntryView {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<AccessListEntry>;
    const IID: windows_core::GUID = <super::super::Foundation::Collections::IVectorView<AccessListEntry> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for AccessListEntryView {
    const NAME: &'static str = "Windows.Storage.AccessCache.AccessListEntryView";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ItemRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ItemRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ItemRemovedEventArgs {
    pub fn RemovedEntry(&self) -> windows_core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovedEntry)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ItemRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IItemRemovedEventArgs>();
}
unsafe impl windows_core::Interface for ItemRemovedEventArgs {
    type Vtable = IItemRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IItemRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ItemRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.AccessCache.ItemRemovedEventArgs";
}
pub struct StorageApplicationPermissions;
impl StorageApplicationPermissions {
    pub fn FutureAccessList() -> windows_core::Result<StorageItemAccessList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FutureAccessList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MostRecentlyUsedList() -> windows_core::Result<StorageItemMostRecentlyUsedList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MostRecentlyUsedList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetFutureAccessListForUser<P0>(user: P0) -> windows_core::Result<StorageItemAccessList>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFutureAccessListForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetMostRecentlyUsedListForUser<P0>(user: P0) -> windows_core::Result<StorageItemMostRecentlyUsedList>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMostRecentlyUsedListForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStorageApplicationPermissionsStatics<R, F: FnOnce(&IStorageApplicationPermissionsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStorageApplicationPermissionsStatics2<R, F: FnOnce(&IStorageApplicationPermissionsStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StorageApplicationPermissions {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageApplicationPermissions";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageItemAccessList(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageItemAccessList, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageItemAccessList, IStorageItemAccessList);
impl StorageItemAccessList {
    pub fn AddOverloadDefaultMetadata<P0>(&self, file: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P0>(&self, token: &windows_core::HSTRING, file: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi()).ok() }
    }
    pub fn AddOrReplace<P0>(&self, token: &windows_core::HSTRING, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata)).ok() }
    }
    pub fn GetItemAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFileAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFolderAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFileWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFolderWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Remove(&self, token: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token)).ok() }
    }
    pub fn ContainsItem(&self, token: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsItem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).map(|| result__)
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CheckAccess<P0>(&self, file: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckAccess)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> windows_core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entries)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaximumItemsAllowed(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaximumItemsAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for StorageItemAccessList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageItemAccessList>();
}
unsafe impl windows_core::Interface for StorageItemAccessList {
    type Vtable = IStorageItemAccessList_Vtbl;
    const IID: windows_core::GUID = <IStorageItemAccessList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemAccessList";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageItemMostRecentlyUsedList(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageItemMostRecentlyUsedList, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageItemMostRecentlyUsedList, IStorageItemAccessList);
impl StorageItemMostRecentlyUsedList {
    pub fn AddOverloadDefaultMetadata<P0>(&self, file: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P0>(&self, token: &windows_core::HSTRING, file: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi()).ok() }
    }
    pub fn AddOrReplace<P0>(&self, token: &windows_core::HSTRING, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata)).ok() }
    }
    pub fn GetItemAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFileAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFolderAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFileWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFolderWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Remove(&self, token: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token)).ok() }
    }
    pub fn ContainsItem(&self, token: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsItem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).map(|| result__)
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CheckAccess<P0>(&self, file: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckAccess)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> windows_core::Result<AccessListEntryView> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entries)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaximumItemsAllowed(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaximumItemsAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ItemRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveItemRemoved(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveItemRemoved)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn AddWithMetadataAndVisibility<P0>(&self, file: P0, metadata: &windows_core::HSTRING, visibility: RecentStorageItemVisibility) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddWithMetadataAndVisibility)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), visibility, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOrReplaceWithMetadataAndVisibility<P0>(&self, token: &windows_core::HSTRING, file: P0, metadata: &windows_core::HSTRING, visibility: RecentStorageItemVisibility) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceWithMetadataAndVisibility)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata), visibility).ok() }
    }
}
impl windows_core::RuntimeType for StorageItemMostRecentlyUsedList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageItemMostRecentlyUsedList>();
}
unsafe impl windows_core::Interface for StorageItemMostRecentlyUsedList {
    type Vtable = IStorageItemMostRecentlyUsedList_Vtbl;
    const IID: windows_core::GUID = <IStorageItemMostRecentlyUsedList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageItemMostRecentlyUsedList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AccessCacheOptions(pub u32);
impl AccessCacheOptions {
    pub const None: Self = Self(0u32);
    pub const DisallowUserInput: Self = Self(1u32);
    pub const FastLocationsOnly: Self = Self(2u32);
    pub const UseReadOnlyCachedCopy: Self = Self(4u32);
    pub const SuppressAccessTimeUpdate: Self = Self(8u32);
}
impl windows_core::TypeKind for AccessCacheOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AccessCacheOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AccessCacheOptions").field(&self.0).finish()
    }
}
impl AccessCacheOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AccessCacheOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AccessCacheOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AccessCacheOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AccessCacheOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AccessCacheOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for AccessCacheOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.AccessCacheOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: Self = Self(0i32);
    pub const AppAndSystem: Self = Self(1i32);
}
impl windows_core::TypeKind for RecentStorageItemVisibility {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RecentStorageItemVisibility {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RecentStorageItemVisibility").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RecentStorageItemVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.RecentStorageItemVisibility;i4)");
}
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessListEntry {
    pub Token: windows_core::HSTRING,
    pub Metadata: windows_core::HSTRING,
}
impl windows_core::TypeKind for AccessListEntry {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for AccessListEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Storage.AccessCache.AccessListEntry;string;string)");
}
impl Default for AccessListEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
