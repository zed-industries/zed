#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl windows_core::RuntimeType for AccessCacheOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.AccessCacheOptions;u4)");
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
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessListEntryView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccessListEntryView, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IVectorView<AccessListEntry>);
windows_core::imp::required_hierarchy!(AccessListEntryView, windows_collections::IIterable<AccessListEntry>);
impl AccessListEntryView {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<AccessListEntry>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<AccessListEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<AccessListEntry> {
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
    pub fn IndexOf(&self, value: &AccessListEntry, index: &mut u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), index, &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [AccessListEntry]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AccessListEntryView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVectorView<AccessListEntry>>();
}
unsafe impl windows_core::Interface for AccessListEntryView {
    type Vtable = <windows_collections::IVectorView<AccessListEntry> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IVectorView<AccessListEntry> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccessListEntryView {
    const NAME: &'static str = "Windows.Storage.AccessCache.AccessListEntryView";
}
impl IntoIterator for AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
windows_core::imp::define_interface!(IItemRemovedEventArgs, IItemRemovedEventArgs_Vtbl, 0x59677e5c_55be_4c66_ba66_5eaea79d2631);
impl windows_core::RuntimeType for IItemRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemovedEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<AccessListEntry>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageApplicationPermissionsStatics, IStorageApplicationPermissionsStatics_Vtbl, 0x4391dfaa_d033_48f9_8060_3ec847d2e3f1);
impl windows_core::RuntimeType for IStorageApplicationPermissionsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
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
#[doc(hidden)]
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
impl windows_core::RuntimeType for IStorageItemAccessList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
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
            (windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P1>(&self, token: &windows_core::HSTRING, file: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi()).ok() }
    }
    pub fn AddOrReplace<P1>(&self, token: &windows_core::HSTRING, file: P1, metadata: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata)).ok() }
    }
    pub fn GetItemAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFolderAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFolderWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
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
#[cfg(all(feature = "Storage_Search", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IStorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.IStorageItemAccessList";
}
#[cfg(all(feature = "Storage_Search", feature = "Storage_Streams"))]
pub trait IStorageItemAccessList_Impl: windows_core::IUnknownImpl {
    fn AddOverloadDefaultMetadata(&self, file: windows_core::Ref<super::IStorageItem>) -> windows_core::Result<windows_core::HSTRING>;
    fn Add(&self, file: windows_core::Ref<super::IStorageItem>, metadata: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>;
    fn AddOrReplaceOverloadDefaultMetadata(&self, token: &windows_core::HSTRING, file: windows_core::Ref<super::IStorageItem>) -> windows_core::Result<()>;
    fn AddOrReplace(&self, token: &windows_core::HSTRING, file: windows_core::Ref<super::IStorageItem>, metadata: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn GetItemAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>>;
    fn GetFileAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>>;
    fn GetFolderAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>>;
    fn GetItemWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>>;
    fn GetFileWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>>;
    fn GetFolderWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>>;
    fn Remove(&self, token: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ContainsItem(&self, token: &windows_core::HSTRING) -> windows_core::Result<bool>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn CheckAccess(&self, file: windows_core::Ref<super::IStorageItem>) -> windows_core::Result<bool>;
    fn Entries(&self) -> windows_core::Result<AccessListEntryView>;
    fn MaximumItemsAllowed(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Storage_Search", feature = "Storage_Streams"))]
impl IStorageItemAccessList_Vtbl {
    pub const fn new<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddOverloadDefaultMetadata<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::AddOverloadDefaultMetadata(this, core::mem::transmute_copy(&file)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, metadata: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::Add(this, core::mem::transmute_copy(&file), core::mem::transmute(&metadata)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddOrReplaceOverloadDefaultMetadata<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, file: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageItemAccessList_Impl::AddOrReplaceOverloadDefaultMetadata(this, core::mem::transmute(&token), core::mem::transmute_copy(&file)).into()
            }
        }
        unsafe extern "system" fn AddOrReplace<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, file: *mut core::ffi::c_void, metadata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageItemAccessList_Impl::AddOrReplace(this, core::mem::transmute(&token), core::mem::transmute_copy(&file), core::mem::transmute(&metadata)).into()
            }
        }
        unsafe extern "system" fn GetItemAsync<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::GetItemAsync(this, core::mem::transmute(&token)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileAsync<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::GetFileAsync(this, core::mem::transmute(&token)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFolderAsync<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::GetFolderAsync(this, core::mem::transmute(&token)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemWithOptionsAsync<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, options: AccessCacheOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::GetItemWithOptionsAsync(this, core::mem::transmute(&token), options) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileWithOptionsAsync<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, options: AccessCacheOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::GetFileWithOptionsAsync(this, core::mem::transmute(&token), options) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFolderWithOptionsAsync<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, options: AccessCacheOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::GetFolderWithOptionsAsync(this, core::mem::transmute(&token), options) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageItemAccessList_Impl::Remove(this, core::mem::transmute(&token)).into()
            }
        }
        unsafe extern "system" fn ContainsItem<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::ContainsItem(this, core::mem::transmute(&token)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorageItemAccessList_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn CheckAccess<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::CheckAccess(this, core::mem::transmute_copy(&file)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Entries<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::Entries(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaximumItemsAllowed<Identity: IStorageItemAccessList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemAccessList_Impl::MaximumItemsAllowed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemAccessList, OFFSET>(),
            AddOverloadDefaultMetadata: AddOverloadDefaultMetadata::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            AddOrReplaceOverloadDefaultMetadata: AddOrReplaceOverloadDefaultMetadata::<Identity, OFFSET>,
            AddOrReplace: AddOrReplace::<Identity, OFFSET>,
            GetItemAsync: GetItemAsync::<Identity, OFFSET>,
            GetFileAsync: GetFileAsync::<Identity, OFFSET>,
            GetFolderAsync: GetFolderAsync::<Identity, OFFSET>,
            GetItemWithOptionsAsync: GetItemWithOptionsAsync::<Identity, OFFSET>,
            GetFileWithOptionsAsync: GetFileWithOptionsAsync::<Identity, OFFSET>,
            GetFolderWithOptionsAsync: GetFolderWithOptionsAsync::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            ContainsItem: ContainsItem::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            CheckAccess: CheckAccess::<Identity, OFFSET>,
            Entries: Entries::<Identity, OFFSET>,
            MaximumItemsAllowed: MaximumItemsAllowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageItemAccessList as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemAccessList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddOverloadDefaultMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddOrReplaceOverloadDefaultMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddOrReplace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFileAsync: usize,
    #[cfg(feature = "Storage_Search")]
    pub GetFolderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetFolderAsync: usize,
    pub GetItemWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AccessCacheOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetFileWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AccessCacheOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFileWithOptionsAsync: usize,
    #[cfg(feature = "Storage_Search")]
    pub GetFolderWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AccessCacheOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetFolderWithOptionsAsync: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContainsItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Entries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaximumItemsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemMostRecentlyUsedList, IStorageItemMostRecentlyUsedList_Vtbl, 0x016239d5_510d_411e_8cf1_c3d1effa4c33);
impl windows_core::RuntimeType for IStorageItemMostRecentlyUsedList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ItemRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveItemRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemMostRecentlyUsedList2, IStorageItemMostRecentlyUsedList2_Vtbl, 0xda481ea0_ed8d_4731_a1db_e44ee2204093);
impl windows_core::RuntimeType for IStorageItemMostRecentlyUsedList2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddWithMetadataAndVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, RecentStorageItemVisibility, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddOrReplaceWithMetadataAndVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, RecentStorageItemVisibility) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ItemRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ItemRemovedEventArgs {
    pub fn RemovedEntry(&self) -> windows_core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovedEntry)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for ItemRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IItemRemovedEventArgs>();
}
unsafe impl windows_core::Interface for ItemRemovedEventArgs {
    type Vtable = <IItemRemovedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IItemRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ItemRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.AccessCache.ItemRemovedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: Self = Self(0i32);
    pub const AppAndSystem: Self = Self(1i32);
}
impl windows_core::TypeKind for RecentStorageItemVisibility {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RecentStorageItemVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.RecentStorageItemVisibility;i4)");
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
    fn IStorageApplicationPermissionsStatics<R, F: FnOnce(&IStorageApplicationPermissionsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IStorageApplicationPermissionsStatics2<R, F: FnOnce(&IStorageApplicationPermissionsStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StorageApplicationPermissions {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageApplicationPermissions";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageItemAccessList(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageItemAccessList, windows_core::IUnknown, windows_core::IInspectable, IStorageItemAccessList);
impl StorageItemAccessList {
    pub fn AddOverloadDefaultMetadata<P0>(&self, file: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P1>(&self, token: &windows_core::HSTRING, file: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi()).ok() }
    }
    pub fn AddOrReplace<P1>(&self, token: &windows_core::HSTRING, file: P1, metadata: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::IStorageItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata)).ok() }
    }
    pub fn GetItemAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFolderAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFolderWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
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
    type Vtable = <IStorageItemAccessList as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageItemAccessList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemAccessList";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
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
            (windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P1>(&self, token: &windows_core::HSTRING, file: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi()).ok() }
    }
    pub fn AddOrReplace<P1>(&self, token: &windows_core::HSTRING, file: P1, metadata: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata)).ok() }
    }
    pub fn GetItemAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFolderAsync(&self, token: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::IStorageItem>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFolderWithOptionsAsync(&self, token: &windows_core::HSTRING, options: AccessCacheOptions) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
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
    pub fn ItemRemoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveItemRemoved(&self, eventcookie: i64) -> windows_core::Result<()> {
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
            (windows_core::Interface::vtable(this).AddWithMetadataAndVisibility)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(metadata), visibility, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AddOrReplaceWithMetadataAndVisibility<P1>(&self, token: &windows_core::HSTRING, file: P1, metadata: &windows_core::HSTRING, visibility: RecentStorageItemVisibility) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOrReplaceWithMetadataAndVisibility)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), file.param().abi(), core::mem::transmute_copy(metadata), visibility).ok() }
    }
}
impl windows_core::RuntimeType for StorageItemMostRecentlyUsedList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageItemMostRecentlyUsedList>();
}
unsafe impl windows_core::Interface for StorageItemMostRecentlyUsedList {
    type Vtable = <IStorageItemMostRecentlyUsedList as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageItemMostRecentlyUsedList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageItemMostRecentlyUsedList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList";
}
