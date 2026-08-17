#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItemSystemStore, IWalletItemSystemStore_Vtbl, 0x522e2bff_96a2_4a17_8d19_fe1d9f837561);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItemSystemStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItemSystemStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetItemsAsync: usize,
    #[cfg(feature = "deprecated")]
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeleteAsync: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub ImportItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    ImportItemAsync: usize,
    #[cfg(feature = "deprecated")]
    pub GetAppStatusForItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut WalletItemAppAssociation) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetAppStatusForItem: usize,
    #[cfg(feature = "deprecated")]
    pub LaunchAppForItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LaunchAppForItemAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItemSystemStore2, IWalletItemSystemStore2_Vtbl, 0xf98d3a4e_be00_4fdd_9734_6c113c1ac1cb);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItemSystemStore2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItemSystemStore2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ItemsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveItemsChanged: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletManagerSystemStatics, IWalletManagerSystemStatics_Vtbl, 0xbee8eb89_2634_4b9a_8b23_ee8903c91fe0);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletManagerSystemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletManagerSystemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestStoreAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletItemSystemStore(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletItemSystemStore, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletItemSystemStore {
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetItemsAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<super::WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeleteAsync<P0>(&self, item: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn ImportItemAsync<P0>(&self, stream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::WalletItem>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportItemAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetAppStatusForItem<P0>(&self, item: P0) -> windows_core::Result<WalletItemAppAssociation>
    where
        P0: windows_core::Param<super::WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppStatusForItem)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LaunchAppForItemAsync<P0>(&self, item: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAppForItemAsync)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ItemsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<WalletItemSystemStore, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveItemsChanged(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWalletItemSystemStore2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveItemsChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletItemSystemStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletItemSystemStore>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletItemSystemStore {
    type Vtable = IWalletItemSystemStore_Vtbl;
    const IID: windows_core::GUID = <IWalletItemSystemStore as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletItemSystemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletItemSystemStore {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletItemSystemStore {}
#[cfg(feature = "deprecated")]
pub struct WalletManagerSystem;
#[cfg(feature = "deprecated")]
impl WalletManagerSystem {
    #[cfg(feature = "deprecated")]
    pub fn RequestStoreAsync() -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<WalletItemSystemStore>> {
        Self::IWalletManagerSystemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IWalletManagerSystemStatics<R, F: FnOnce(&IWalletManagerSystemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletManagerSystem, IWalletManagerSystemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletManagerSystem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WalletItemAppAssociation(pub i32);
#[cfg(feature = "deprecated")]
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for WalletItemAppAssociation {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for WalletItemAppAssociation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WalletItemAppAssociation").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletItemAppAssociation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation;i4)");
}
