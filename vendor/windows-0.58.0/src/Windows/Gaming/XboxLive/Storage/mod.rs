windows_core::imp::define_interface!(IGameSaveBlobGetResult, IGameSaveBlobGetResult_Vtbl, 0x917281e0_7201_4953_aa2c_4008f03aef45);
impl windows_core::RuntimeType for IGameSaveBlobGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveBlobGetResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameSaveErrorStatus) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Value: usize,
}
windows_core::imp::define_interface!(IGameSaveBlobInfo, IGameSaveBlobInfo_Vtbl, 0xadd38034_baf0_4645_b6d0_46edaffb3c2b);
impl windows_core::RuntimeType for IGameSaveBlobInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveBlobInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameSaveBlobInfoGetResult, IGameSaveBlobInfoGetResult_Vtbl, 0xc7578582_3697_42bf_989c_665d923b5231);
impl windows_core::RuntimeType for IGameSaveBlobInfoGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveBlobInfoGetResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameSaveErrorStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
windows_core::imp::define_interface!(IGameSaveBlobInfoQuery, IGameSaveBlobInfoQuery_Vtbl, 0x9fdd74b2_eeee_447b_a9d2_7f96c0f83208);
impl windows_core::RuntimeType for IGameSaveBlobInfoQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveBlobInfoQuery_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetBlobInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlobInfoWithIndexAndMaxAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameSaveContainer, IGameSaveContainer_Vtbl, 0xc3c08f89_563f_4ecd_9c6f_33fd0e323d10);
impl windows_core::RuntimeType for IGameSaveContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveContainer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Provider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SubmitUpdatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SubmitUpdatesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ReadAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SubmitPropertySetUpdatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SubmitPropertySetUpdatesAsync: usize,
    pub CreateBlobInfoQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameSaveContainerInfo, IGameSaveContainerInfo_Vtbl, 0xb7e27300_155d_4bb4_b2ba_930306f391b5);
impl windows_core::RuntimeType for IGameSaveContainerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveContainerInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub NeedsSync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameSaveContainerInfoGetResult, IGameSaveContainerInfoGetResult_Vtbl, 0xffc50d74_c581_4f9d_9e39_30a10c1e4c50);
impl windows_core::RuntimeType for IGameSaveContainerInfoGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveContainerInfoGetResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameSaveErrorStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
windows_core::imp::define_interface!(IGameSaveContainerInfoQuery, IGameSaveContainerInfoQuery_Vtbl, 0x3c94e863_6f80_4327_9327_ffc11afd42b3);
impl windows_core::RuntimeType for IGameSaveContainerInfoQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveContainerInfoQuery_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetContainerInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContainerInfoWithIndexAndMaxAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameSaveOperationResult, IGameSaveOperationResult_Vtbl, 0xcf0f1a05_24a0_4582_9a55_b1bbbb9388d8);
impl windows_core::RuntimeType for IGameSaveOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameSaveErrorStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameSaveProvider, IGameSaveProvider_Vtbl, 0x90a60394_80fe_4211_97f8_a5de14dd95d2);
impl windows_core::RuntimeType for IGameSaveProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub CreateContainer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteContainerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateContainerInfoQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateContainerInfoQueryWithName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemainingBytesInQuotaAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ContainersChangedSinceLastSync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContainersChangedSinceLastSync: usize,
}
windows_core::imp::define_interface!(IGameSaveProviderGetResult, IGameSaveProviderGetResult_Vtbl, 0x3ab90816_d393_4d65_ac16_41c3e67ab945);
impl windows_core::RuntimeType for IGameSaveProviderGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveProviderGetResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameSaveErrorStatus) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameSaveProviderStatics, IGameSaveProviderStatics_Vtbl, 0xd01d3ed0_7b03_449d_8cbd_3402842a1048);
impl windows_core::RuntimeType for IGameSaveProviderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameSaveProviderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUserAsync: usize,
    #[cfg(feature = "System")]
    pub GetSyncOnDemandForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetSyncOnDemandForUserAsync: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveBlobGetResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveBlobGetResult, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveBlobGetResult {
    pub fn Status(&self) -> windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Value(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameSaveBlobGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveBlobGetResult>();
}
unsafe impl windows_core::Interface for GameSaveBlobGetResult {
    type Vtable = IGameSaveBlobGetResult_Vtbl;
    const IID: windows_core::GUID = <IGameSaveBlobGetResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveBlobGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult";
}
unsafe impl Send for GameSaveBlobGetResult {}
unsafe impl Sync for GameSaveBlobGetResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveBlobInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveBlobInfo, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveBlobInfo {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GameSaveBlobInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveBlobInfo>();
}
unsafe impl windows_core::Interface for GameSaveBlobInfo {
    type Vtable = IGameSaveBlobInfo_Vtbl;
    const IID: windows_core::GUID = <IGameSaveBlobInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveBlobInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo";
}
unsafe impl Send for GameSaveBlobInfo {}
unsafe impl Sync for GameSaveBlobInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveBlobInfoGetResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveBlobInfoGetResult, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveBlobInfoGetResult {
    pub fn Status(&self) -> windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveBlobInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameSaveBlobInfoGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveBlobInfoGetResult>();
}
unsafe impl windows_core::Interface for GameSaveBlobInfoGetResult {
    type Vtable = IGameSaveBlobInfoGetResult_Vtbl;
    const IID: windows_core::GUID = <IGameSaveBlobInfoGetResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveBlobInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult";
}
unsafe impl Send for GameSaveBlobInfoGetResult {}
unsafe impl Sync for GameSaveBlobInfoGetResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveBlobInfoQuery(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveBlobInfoQuery, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveBlobInfoQuery {
    pub fn GetBlobInfoAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBlobInfoAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetBlobInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBlobInfoWithIndexAndMaxAsync)(windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemCountAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameSaveBlobInfoQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveBlobInfoQuery>();
}
unsafe impl windows_core::Interface for GameSaveBlobInfoQuery {
    type Vtable = IGameSaveBlobInfoQuery_Vtbl;
    const IID: windows_core::GUID = <IGameSaveBlobInfoQuery as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveBlobInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery";
}
unsafe impl Send for GameSaveBlobInfoQuery {}
unsafe impl Sync for GameSaveBlobInfoQuery {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveContainer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveContainer, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveContainer {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Provider(&self) -> windows_core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Provider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SubmitUpdatesAsync<P0, P1>(&self, blobstowrite: P0, blobstodelete: P1, displayname: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, super::super::super::Storage::Streams::IBuffer>>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubmitUpdatesAsync)(windows_core::Interface::as_raw(this), blobstowrite.param().abi(), blobstodelete.param().abi(), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0>(&self, blobstoread: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, super::super::super::Storage::Streams::IBuffer>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), blobstoread.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsync<P0>(&self, blobstoread: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveBlobGetResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsync)(windows_core::Interface::as_raw(this), blobstoread.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SubmitPropertySetUpdatesAsync<P0, P1>(&self, blobstowrite: P0, blobstodelete: P1, displayname: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IPropertySet>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubmitPropertySetUpdatesAsync)(windows_core::Interface::as_raw(this), blobstowrite.param().abi(), blobstodelete.param().abi(), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateBlobInfoQuery(&self, blobnameprefix: &windows_core::HSTRING) -> windows_core::Result<GameSaveBlobInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBlobInfoQuery)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(blobnameprefix), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameSaveContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveContainer>();
}
unsafe impl windows_core::Interface for GameSaveContainer {
    type Vtable = IGameSaveContainer_Vtbl;
    const IID: windows_core::GUID = <IGameSaveContainer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveContainer {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainer";
}
unsafe impl Send for GameSaveContainer {}
unsafe impl Sync for GameSaveContainer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveContainerInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveContainerInfo, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveContainerInfo {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TotalSize(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TotalSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastModifiedTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastModifiedTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NeedsSync(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeedsSync)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GameSaveContainerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveContainerInfo>();
}
unsafe impl windows_core::Interface for GameSaveContainerInfo {
    type Vtable = IGameSaveContainerInfo_Vtbl;
    const IID: windows_core::GUID = <IGameSaveContainerInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveContainerInfo {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo";
}
unsafe impl Send for GameSaveContainerInfo {}
unsafe impl Sync for GameSaveContainerInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveContainerInfoGetResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveContainerInfoGetResult, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveContainerInfoGetResult {
    pub fn Status(&self) -> windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GameSaveContainerInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameSaveContainerInfoGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveContainerInfoGetResult>();
}
unsafe impl windows_core::Interface for GameSaveContainerInfoGetResult {
    type Vtable = IGameSaveContainerInfoGetResult_Vtbl;
    const IID: windows_core::GUID = <IGameSaveContainerInfoGetResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveContainerInfoGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult";
}
unsafe impl Send for GameSaveContainerInfoGetResult {}
unsafe impl Sync for GameSaveContainerInfoGetResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveContainerInfoQuery(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveContainerInfoQuery, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveContainerInfoQuery {
    pub fn GetContainerInfoAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContainerInfoAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetContainerInfoWithIndexAndMaxAsync(&self, startindex: u32, maxnumberofitems: u32) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveContainerInfoGetResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContainerInfoWithIndexAndMaxAsync)(windows_core::Interface::as_raw(this), startindex, maxnumberofitems, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetItemCountAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameSaveContainerInfoQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveContainerInfoQuery>();
}
unsafe impl windows_core::Interface for GameSaveContainerInfoQuery {
    type Vtable = IGameSaveContainerInfoQuery_Vtbl;
    const IID: windows_core::GUID = <IGameSaveContainerInfoQuery as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveContainerInfoQuery {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery";
}
unsafe impl Send for GameSaveContainerInfoQuery {}
unsafe impl Sync for GameSaveContainerInfoQuery {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveOperationResult {
    pub fn Status(&self) -> windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GameSaveOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveOperationResult>();
}
unsafe impl windows_core::Interface for GameSaveOperationResult {
    type Vtable = IGameSaveOperationResult_Vtbl;
    const IID: windows_core::GUID = <IGameSaveOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveOperationResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveOperationResult";
}
unsafe impl Send for GameSaveOperationResult {}
unsafe impl Sync for GameSaveOperationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveProvider, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveProvider {
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateContainer(&self, name: &windows_core::HSTRING) -> windows_core::Result<GameSaveContainer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateContainer)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteContainerAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteContainerAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateContainerInfoQuery(&self) -> windows_core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateContainerInfoQuery)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateContainerInfoQueryWithName(&self, containernameprefix: &windows_core::HSTRING) -> windows_core::Result<GameSaveContainerInfoQuery> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateContainerInfoQueryWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(containernameprefix), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRemainingBytesInQuotaAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRemainingBytesInQuotaAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContainersChangedSinceLastSync(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainersChangedSinceLastSync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn GetForUserAsync<P0>(user: P0, serviceconfigid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(serviceconfigid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetSyncOnDemandForUserAsync<P0>(user: P0, serviceconfigid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<GameSaveProviderGetResult>>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IGameSaveProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSyncOnDemandForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(serviceconfigid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGameSaveProviderStatics<R, F: FnOnce(&IGameSaveProviderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameSaveProvider, IGameSaveProviderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GameSaveProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveProvider>();
}
unsafe impl windows_core::Interface for GameSaveProvider {
    type Vtable = IGameSaveProvider_Vtbl;
    const IID: windows_core::GUID = <IGameSaveProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveProvider {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProvider";
}
unsafe impl Send for GameSaveProvider {}
unsafe impl Sync for GameSaveProvider {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameSaveProviderGetResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameSaveProviderGetResult, windows_core::IUnknown, windows_core::IInspectable);
impl GameSaveProviderGetResult {
    pub fn Status(&self) -> windows_core::Result<GameSaveErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<GameSaveProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameSaveProviderGetResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameSaveProviderGetResult>();
}
unsafe impl windows_core::Interface for GameSaveProviderGetResult {
    type Vtable = IGameSaveProviderGetResult_Vtbl;
    const IID: windows_core::GUID = <IGameSaveProviderGetResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameSaveProviderGetResult {
    const NAME: &'static str = "Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult";
}
unsafe impl Send for GameSaveProviderGetResult {}
unsafe impl Sync for GameSaveProviderGetResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameSaveErrorStatus(pub i32);
impl GameSaveErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Abort: Self = Self(-2147467260i32);
    pub const InvalidContainerName: Self = Self(-2138898431i32);
    pub const NoAccess: Self = Self(-2138898430i32);
    pub const OutOfLocalStorage: Self = Self(-2138898429i32);
    pub const UserCanceled: Self = Self(-2138898428i32);
    pub const UpdateTooBig: Self = Self(-2138898427i32);
    pub const QuotaExceeded: Self = Self(-2138898426i32);
    pub const ProvidedBufferTooSmall: Self = Self(-2138898425i32);
    pub const BlobNotFound: Self = Self(-2138898424i32);
    pub const NoXboxLiveInfo: Self = Self(-2138898423i32);
    pub const ContainerNotInSync: Self = Self(-2138898422i32);
    pub const ContainerSyncFailed: Self = Self(-2138898421i32);
    pub const UserHasNoXboxLiveInfo: Self = Self(-2138898420i32);
    pub const ObjectExpired: Self = Self(-2138898419i32);
}
impl windows_core::TypeKind for GameSaveErrorStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameSaveErrorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameSaveErrorStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameSaveErrorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.XboxLive.Storage.GameSaveErrorStatus;i4)");
}
