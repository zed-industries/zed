windows_core::imp::define_interface!(IBackgroundDownloader, IBackgroundDownloader_Vtbl, 0xc1c79333_6649_4b1d_a826_a4b3dd234d0b);
impl windows_core::RuntimeType for IBackgroundDownloader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundDownloader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub CreateDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateDownload: usize,
    #[cfg(feature = "Storage")]
    pub CreateDownloadFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateDownloadFromFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateDownloadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateDownloadAsync: usize,
}
windows_core::imp::define_interface!(IBackgroundDownloader2, IBackgroundDownloader2_Vtbl, 0xa94a5847_348d_4a35_890e_8a1ef3798479);
impl windows_core::RuntimeType for IBackgroundDownloader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundDownloader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureTileNotification: usize,
}
windows_core::imp::define_interface!(IBackgroundDownloader3, IBackgroundDownloader3_Vtbl, 0xd11a8c48_86e8_48e2_b615_6976aabf861d);
impl windows_core::RuntimeType for IBackgroundDownloader3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundDownloader3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CompletionGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundDownloaderFactory, IBackgroundDownloaderFactory_Vtbl, 0x26836c24_d89e_46f4_a29a_4f4d4f144155);
impl windows_core::RuntimeType for IBackgroundDownloaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundDownloaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundDownloaderStaticMethods, IBackgroundDownloaderStaticMethods_Vtbl, 0x52a65a35_c64e_426c_9919_540d0d21a650);
impl windows_core::RuntimeType for IBackgroundDownloaderStaticMethods {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundDownloaderStaticMethods_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentDownloadsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentDownloadsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCurrentDownloadsForGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCurrentDownloadsForGroupAsync: usize,
}
windows_core::imp::define_interface!(IBackgroundDownloaderStaticMethods2, IBackgroundDownloaderStaticMethods2_Vtbl, 0x2faa1327_1ad4_4ca5_b2cd_08dbf0746afe);
impl windows_core::RuntimeType for IBackgroundDownloaderStaticMethods2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundDownloaderStaticMethods2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentDownloadsForTransferGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentDownloadsForTransferGroupAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IBackgroundDownloaderUserConsent, IBackgroundDownloaderUserConsent_Vtbl, 0x5d14e906_9266_4808_bd71_5925f2a3130a);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IBackgroundDownloaderUserConsent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundDownloaderUserConsent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RequestUnconstrainedDownloadsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RequestUnconstrainedDownloadsAsync: usize,
}
windows_core::imp::define_interface!(IBackgroundTransferBase, IBackgroundTransferBase_Vtbl, 0x2a9da250_c769_458c_afe8_feb8d4d3b2ef);
impl core::ops::Deref for IBackgroundTransferBase {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTransferBase, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTransferBase {
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, credential: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), credential.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, credential: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), credential.param().abi()).ok() }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMethod(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMethod)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CostPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCostPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for IBackgroundTransferBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    pub Method: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Group: usize,
    #[cfg(feature = "deprecated")]
    pub SetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGroup: usize,
    pub CostPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundTransferCostPolicy) -> windows_core::HRESULT,
    pub SetCostPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, BackgroundTransferCostPolicy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTransferCompletionGroup, IBackgroundTransferCompletionGroup_Vtbl, 0x2d930225_986b_574d_7950_0add47f5d706);
impl windows_core::RuntimeType for IBackgroundTransferCompletionGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferCompletionGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub Trigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    Trigger: usize,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTransferCompletionGroupTriggerDetails, IBackgroundTransferCompletionGroupTriggerDetails_Vtbl, 0x7b6be286_6e47_5136_7fcb_fa4389f46f5b);
impl windows_core::RuntimeType for IBackgroundTransferCompletionGroupTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Downloads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Downloads: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Uploads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Uploads: usize,
}
windows_core::imp::define_interface!(IBackgroundTransferContentPart, IBackgroundTransferContentPart_Vtbl, 0xe8e15657_d7d1_4ed8_838e_674ac217ace6);
impl windows_core::RuntimeType for IBackgroundTransferContentPart {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferContentPart_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SetFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetFile: usize,
}
windows_core::imp::define_interface!(IBackgroundTransferContentPartFactory, IBackgroundTransferContentPartFactory_Vtbl, 0x90ef98a9_7a01_4a0b_9f80_a0b0bb370f8d);
impl core::ops::Deref for IBackgroundTransferContentPartFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTransferContentPartFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTransferContentPartFactory {
    pub fn CreateWithName(&self, name: &windows_core::HSTRING) -> windows_core::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWithNameAndFileName(&self, name: &windows_core::HSTRING, filename: &windows_core::HSTRING) -> windows_core::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithNameAndFileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(filename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IBackgroundTransferContentPartFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferContentPartFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithNameAndFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTransferErrorStaticMethods, IBackgroundTransferErrorStaticMethods_Vtbl, 0xaad33b04_1192_4bf4_8b68_39c5add244e2);
impl windows_core::RuntimeType for IBackgroundTransferErrorStaticMethods {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferErrorStaticMethods_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Web::WebErrorStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
windows_core::imp::define_interface!(IBackgroundTransferGroup, IBackgroundTransferGroup_Vtbl, 0xd8c3e3e4_6459_4540_85eb_aaa1c8903677);
impl windows_core::RuntimeType for IBackgroundTransferGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransferBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundTransferBehavior) -> windows_core::HRESULT,
    pub SetTransferBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, BackgroundTransferBehavior) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTransferGroupStatics, IBackgroundTransferGroupStatics_Vtbl, 0x02ec50b2_7d18_495b_aa22_32a97d45d3e2);
impl windows_core::RuntimeType for IBackgroundTransferGroupStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferGroupStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTransferOperation, IBackgroundTransferOperation_Vtbl, 0xded06846_90ca_44fb_8fb1_124154c0d539);
impl core::ops::Deref for IBackgroundTransferOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTransferOperation, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTransferOperation {
    pub fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Guid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestedUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CostPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCostPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResultStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResponseInformation(&self) -> windows_core::Result<ResponseInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResponseInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IBackgroundTransferOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RequestedUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Method: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Group: usize,
    pub CostPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundTransferCostPolicy) -> windows_core::HRESULT,
    pub SetCostPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, BackgroundTransferCostPolicy) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetResultStreamAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetResultStreamAt: usize,
    pub GetResponseInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTransferOperationPriority, IBackgroundTransferOperationPriority_Vtbl, 0x04854327_5254_4b3a_915e_0aa49275c0f9);
impl core::ops::Deref for IBackgroundTransferOperationPriority {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTransferOperationPriority, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTransferOperationPriority {
    pub fn Priority(&self) -> windows_core::Result<BackgroundTransferPriority> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Priority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPriority)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for IBackgroundTransferOperationPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferOperationPriority_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundTransferPriority) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, BackgroundTransferPriority) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTransferRangesDownloadedEventArgs, IBackgroundTransferRangesDownloadedEventArgs_Vtbl, 0x3ebc7453_bf48_4a88_9248_b0c165184f5c);
impl windows_core::RuntimeType for IBackgroundTransferRangesDownloadedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTransferRangesDownloadedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WasDownloadRestarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddedRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddedRanges: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundUploader, IBackgroundUploader_Vtbl, 0xc595c9ae_cead_465b_8801_c55ac90a01ce);
impl windows_core::RuntimeType for IBackgroundUploader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundUploader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub CreateUpload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateUpload: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateUploadFromStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateUploadFromStreamAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithFormDataAndAutoBoundaryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithFormDataAndAutoBoundaryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithSubTypeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithSubTypeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUploadWithSubTypeAndBoundaryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUploadWithSubTypeAndBoundaryAsync: usize,
}
windows_core::imp::define_interface!(IBackgroundUploader2, IBackgroundUploader2_Vtbl, 0x8e0612ce_0c34_4463_807f_198a1b8bd4ad);
impl windows_core::RuntimeType for IBackgroundUploader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundUploader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SuccessTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetSuccessTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetSuccessTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub FailureTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    FailureTileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub SetFailureTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    SetFailureTileNotification: usize,
}
windows_core::imp::define_interface!(IBackgroundUploader3, IBackgroundUploader3_Vtbl, 0xb95e9439_5bf0_4b3a_8c47_2c6199a854b9);
impl windows_core::RuntimeType for IBackgroundUploader3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundUploader3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CompletionGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundUploaderFactory, IBackgroundUploaderFactory_Vtbl, 0x736203c7_10e7_48a0_ac3c_1ac71095ec57);
impl windows_core::RuntimeType for IBackgroundUploaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundUploaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithCompletionGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundUploaderStaticMethods, IBackgroundUploaderStaticMethods_Vtbl, 0xf2875cfb_9b05_4741_9121_740a83e247df);
impl windows_core::RuntimeType for IBackgroundUploaderStaticMethods {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundUploaderStaticMethods_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentUploadsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentUploadsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetCurrentUploadsForGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetCurrentUploadsForGroupAsync: usize,
}
windows_core::imp::define_interface!(IBackgroundUploaderStaticMethods2, IBackgroundUploaderStaticMethods2_Vtbl, 0xe919ac62_ea08_42f0_a2ac_07e467549080);
impl windows_core::RuntimeType for IBackgroundUploaderStaticMethods2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundUploaderStaticMethods2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentUploadsForTransferGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentUploadsForTransferGroupAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IBackgroundUploaderUserConsent, IBackgroundUploaderUserConsent_Vtbl, 0x3bb384cb_0760_461d_907f_5138f84d44c1);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IBackgroundUploaderUserConsent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundUploaderUserConsent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RequestUnconstrainedUploadsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RequestUnconstrainedUploadsAsync: usize,
}
windows_core::imp::define_interface!(IContentPrefetcher, IContentPrefetcher_Vtbl, 0xa8d6f754_7dc1_4cd9_8810_2a6aa9417e11);
impl windows_core::RuntimeType for IContentPrefetcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentPrefetcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContentUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContentUris: usize,
    pub SetIndirectContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IndirectContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContentPrefetcherTime, IContentPrefetcherTime_Vtbl, 0xe361fd08_132a_4fde_a7cc_fcb0e66523af);
impl windows_core::RuntimeType for IContentPrefetcherTime {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentPrefetcherTime_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LastSuccessfulPrefetchTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDownloadOperation, IDownloadOperation_Vtbl, 0xbd87ebb0_5714_4e09_ba68_bef73903b0d7);
impl windows_core::RuntimeType for IDownloadOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDownloadOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub ResultFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ResultFile: usize,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundDownloadProgress) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttachAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDownloadOperation2, IDownloadOperation2_Vtbl, 0xa3cced40_8f9c_4353_9cd4_290dee387c38);
impl windows_core::RuntimeType for IDownloadOperation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDownloadOperation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDownloadOperation3, IDownloadOperation3_Vtbl, 0x5027351c_7d5e_4adc_b8d3_df5c6031b9cc);
impl windows_core::RuntimeType for IDownloadOperation3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDownloadOperation3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRandomAccessRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRandomAccessRequired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetResultRandomAccessStreamReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetResultRandomAccessStreamReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDownloadedRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDownloadedRanges: usize,
    pub RangesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRangesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SetRequestedUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))]
    pub RecoverableWebErrorStatuses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web")))]
    RecoverableWebErrorStatuses: usize,
    #[cfg(feature = "Web")]
    pub CurrentWebErrorStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web"))]
    CurrentWebErrorStatus: usize,
}
windows_core::imp::define_interface!(IDownloadOperation4, IDownloadOperation4_Vtbl, 0x0cdaaef4_8cef_404a_966d_f058400bed80);
impl windows_core::RuntimeType for IDownloadOperation4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDownloadOperation4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDownloadOperation5, IDownloadOperation5_Vtbl, 0xa699a86f_5590_463a_b8d6_1e491a2760a5);
impl windows_core::RuntimeType for IDownloadOperation5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDownloadOperation5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResponseInformation, IResponseInformation_Vtbl, 0xf8bb9a12_f713_4792_8b68_d9d297f91d2e);
impl windows_core::RuntimeType for IResponseInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IResponseInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsResumable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ActualUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IUnconstrainedTransferRequestResult, IUnconstrainedTransferRequestResult_Vtbl, 0x4c24b81f_d944_4112_a98e_6a69522b7ebb);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IUnconstrainedTransferRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IUnconstrainedTransferRequestResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub IsUnconstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsUnconstrained: usize,
}
windows_core::imp::define_interface!(IUploadOperation, IUploadOperation_Vtbl, 0x3e5624e0_7389_434c_8b35_427fd36bbdae);
impl windows_core::RuntimeType for IUploadOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUploadOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub SourceFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SourceFile: usize,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundUploadProgress) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttachAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUploadOperation2, IUploadOperation2_Vtbl, 0x556189f2_2774_4df6_9fa5_209f2bfb12f7);
impl windows_core::RuntimeType for IUploadOperation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUploadOperation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUploadOperation3, IUploadOperation3_Vtbl, 0x42c92ca3_de39_4546_bc62_3774b4294de3);
impl windows_core::RuntimeType for IUploadOperation3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUploadOperation3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MakeCurrentInTransferGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUploadOperation4, IUploadOperation4_Vtbl, 0x50edef31_fac5_41ee_b030_dc77caee9faa);
impl windows_core::RuntimeType for IUploadOperation4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUploadOperation4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundDownloader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundDownloader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BackgroundDownloader, IBackgroundTransferBase);
impl BackgroundDownloader {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundDownloader, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage")]
    pub fn CreateDownload<P0, P1>(&self, uri: P0, resultfile: P1) -> windows_core::Result<DownloadOperation>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDownload)(windows_core::Interface::as_raw(this), uri.param().abi(), resultfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CreateDownloadFromFile<P0, P1, P2>(&self, uri: P0, resultfile: P1, requestbodyfile: P2) -> windows_core::Result<DownloadOperation>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
        P2: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDownloadFromFile)(windows_core::Interface::as_raw(this), uri.param().abi(), resultfile.param().abi(), requestbodyfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateDownloadAsync<P0, P1, P2>(&self, uri: P0, resultfile: P1, requestbodystream: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DownloadOperation>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
        P2: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDownloadAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), resultfile.param().abi(), requestbodystream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransferGroup(&self) -> windows_core::Result<BackgroundTransferGroup> {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransferGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BackgroundTransferGroup>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransferGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessToastNotification(&self) -> windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuccessToastNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessToastNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::ToastNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSuccessToastNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureToastNotification(&self) -> windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FailureToastNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureToastNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::ToastNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFailureToastNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessTileNotification(&self) -> windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuccessTileNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessTileNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::TileNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSuccessTileNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureTileNotification(&self) -> windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FailureTileNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureTileNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::TileNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFailureTileNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CompletionGroup(&self) -> windows_core::Result<BackgroundTransferCompletionGroup> {
        let this = &windows_core::Interface::cast::<IBackgroundDownloader3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletionGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWithCompletionGroup<P0>(completiongroup: P0) -> windows_core::Result<BackgroundDownloader>
    where
        P0: windows_core::Param<BackgroundTransferCompletionGroup>,
    {
        Self::IBackgroundDownloaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithCompletionGroup)(windows_core::Interface::as_raw(this), completiongroup.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentDownloadsAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentDownloadsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCurrentDownloadsForGroupAsync(group: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentDownloadsForGroupAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(group), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentDownloadsForTransferGroupAsync<P0>(group: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>
    where
        P0: windows_core::Param<BackgroundTransferGroup>,
    {
        Self::IBackgroundDownloaderStaticMethods2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentDownloadsForTransferGroupAsync)(windows_core::Interface::as_raw(this), group.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RequestUnconstrainedDownloadsAsync<P0>(operations: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<DownloadOperation>>,
    {
        Self::IBackgroundDownloaderUserConsent(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestUnconstrainedDownloadsAsync)(windows_core::Interface::as_raw(this), operations.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, credential: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), credential.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, credential: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), credential.param().abi()).ok() }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMethod(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMethod)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CostPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCostPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderFactory<R, F: FnOnce(&IBackgroundDownloaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundDownloader, IBackgroundDownloaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderStaticMethods<R, F: FnOnce(&IBackgroundDownloaderStaticMethods) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderStaticMethods2<R, F: FnOnce(&IBackgroundDownloaderStaticMethods2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundDownloaderUserConsent<R, F: FnOnce(&IBackgroundDownloaderUserConsent) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundDownloader, IBackgroundDownloaderUserConsent> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BackgroundDownloader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundDownloader>();
}
unsafe impl windows_core::Interface for BackgroundDownloader {
    type Vtable = IBackgroundDownloader_Vtbl;
    const IID: windows_core::GUID = <IBackgroundDownloader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundDownloader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundDownloader";
}
unsafe impl Send for BackgroundDownloader {}
unsafe impl Sync for BackgroundDownloader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTransferCompletionGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTransferCompletionGroup, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTransferCompletionGroup {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTransferCompletionGroup, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Trigger(&self) -> windows_core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Trigger)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Enable(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Enable)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BackgroundTransferCompletionGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTransferCompletionGroup>();
}
unsafe impl windows_core::Interface for BackgroundTransferCompletionGroup {
    type Vtable = IBackgroundTransferCompletionGroup_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTransferCompletionGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTransferCompletionGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup";
}
unsafe impl Send for BackgroundTransferCompletionGroup {}
unsafe impl Sync for BackgroundTransferCompletionGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTransferCompletionGroupTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTransferCompletionGroupTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTransferCompletionGroupTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Downloads(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Downloads)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Uploads(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uploads)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BackgroundTransferCompletionGroupTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTransferCompletionGroupTriggerDetails>();
}
unsafe impl windows_core::Interface for BackgroundTransferCompletionGroupTriggerDetails {
    type Vtable = IBackgroundTransferCompletionGroupTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTransferCompletionGroupTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTransferCompletionGroupTriggerDetails {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails";
}
unsafe impl Send for BackgroundTransferCompletionGroupTriggerDetails {}
unsafe impl Sync for BackgroundTransferCompletionGroupTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTransferContentPart(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTransferContentPart, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTransferContentPart {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTransferContentPart, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn SetFile<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFile)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CreateWithName(name: &windows_core::HSTRING) -> windows_core::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithNameAndFileName(name: &windows_core::HSTRING, filename: &windows_core::HSTRING) -> windows_core::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithNameAndFileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(filename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferContentPartFactory<R, F: FnOnce(&IBackgroundTransferContentPartFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTransferContentPart, IBackgroundTransferContentPartFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BackgroundTransferContentPart {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTransferContentPart>();
}
unsafe impl windows_core::Interface for BackgroundTransferContentPart {
    type Vtable = IBackgroundTransferContentPart_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTransferContentPart as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTransferContentPart {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart";
}
unsafe impl Send for BackgroundTransferContentPart {}
unsafe impl Sync for BackgroundTransferContentPart {}
pub struct BackgroundTransferError;
impl BackgroundTransferError {
    #[cfg(feature = "Web")]
    pub fn GetStatus(hresult: i32) -> windows_core::Result<super::super::Web::WebErrorStatus> {
        Self::IBackgroundTransferErrorStaticMethods(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), hresult, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferErrorStaticMethods<R, F: FnOnce(&IBackgroundTransferErrorStaticMethods) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTransferError, IBackgroundTransferErrorStaticMethods> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BackgroundTransferError {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferError";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTransferGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTransferGroup, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTransferGroup {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransferBehavior(&self) -> windows_core::Result<BackgroundTransferBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferBehavior)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransferBehavior(&self, value: BackgroundTransferBehavior) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransferBehavior)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateGroup(name: &windows_core::HSTRING) -> windows_core::Result<BackgroundTransferGroup> {
        Self::IBackgroundTransferGroupStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferGroupStatics<R, F: FnOnce(&IBackgroundTransferGroupStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTransferGroup, IBackgroundTransferGroupStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BackgroundTransferGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTransferGroup>();
}
unsafe impl windows_core::Interface for BackgroundTransferGroup {
    type Vtable = IBackgroundTransferGroup_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTransferGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTransferGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferGroup";
}
unsafe impl Send for BackgroundTransferGroup {}
unsafe impl Sync for BackgroundTransferGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTransferRangesDownloadedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTransferRangesDownloadedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTransferRangesDownloadedEventArgs {
    pub fn WasDownloadRestarted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WasDownloadRestarted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddedRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddedRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BackgroundTransferRangesDownloadedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTransferRangesDownloadedEventArgs>();
}
unsafe impl windows_core::Interface for BackgroundTransferRangesDownloadedEventArgs {
    type Vtable = IBackgroundTransferRangesDownloadedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTransferRangesDownloadedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTransferRangesDownloadedEventArgs {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs";
}
unsafe impl Send for BackgroundTransferRangesDownloadedEventArgs {}
unsafe impl Sync for BackgroundTransferRangesDownloadedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundUploader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundUploader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BackgroundUploader, IBackgroundTransferBase);
impl BackgroundUploader {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundUploader, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, credential: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), credential.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, credential: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), credential.param().abi()).ok() }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMethod(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMethod)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CostPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCostPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn CreateUpload<P0, P1>(&self, uri: P0, sourcefile: P1) -> windows_core::Result<UploadOperation>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUpload)(windows_core::Interface::as_raw(this), uri.param().abi(), sourcefile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateUploadFromStreamAsync<P0, P1>(&self, uri: P0, sourcestream: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUploadFromStreamAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), sourcestream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUploadWithFormDataAndAutoBoundaryAsync<P0, P1>(&self, uri: P0, parts: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUploadWithFormDataAndAutoBoundaryAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), parts.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUploadWithSubTypeAsync<P0, P1>(&self, uri: P0, parts: P1, subtype: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUploadWithSubTypeAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), parts.param().abi(), core::mem::transmute_copy(subtype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUploadWithSubTypeAndBoundaryAsync<P0, P1>(&self, uri: P0, parts: P1, subtype: &windows_core::HSTRING, boundary: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUploadWithSubTypeAndBoundaryAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), parts.param().abi(), core::mem::transmute_copy(subtype), core::mem::transmute_copy(boundary), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransferGroup(&self) -> windows_core::Result<BackgroundTransferGroup> {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransferGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BackgroundTransferGroup>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransferGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessToastNotification(&self) -> windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuccessToastNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessToastNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::ToastNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSuccessToastNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureToastNotification(&self) -> windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FailureToastNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureToastNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::ToastNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFailureToastNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessTileNotification(&self) -> windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuccessTileNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessTileNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::TileNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSuccessTileNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureTileNotification(&self) -> windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FailureTileNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureTileNotification<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Notifications::TileNotification>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFailureTileNotification)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CompletionGroup(&self) -> windows_core::Result<BackgroundTransferCompletionGroup> {
        let this = &windows_core::Interface::cast::<IBackgroundUploader3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletionGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWithCompletionGroup<P0>(completiongroup: P0) -> windows_core::Result<BackgroundUploader>
    where
        P0: windows_core::Param<BackgroundTransferCompletionGroup>,
    {
        Self::IBackgroundUploaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithCompletionGroup)(windows_core::Interface::as_raw(this), completiongroup.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentUploadsAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentUploadsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCurrentUploadsForGroupAsync(group: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentUploadsForGroupAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(group), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentUploadsForTransferGroupAsync<P0>(group: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>
    where
        P0: windows_core::Param<BackgroundTransferGroup>,
    {
        Self::IBackgroundUploaderStaticMethods2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentUploadsForTransferGroupAsync)(windows_core::Interface::as_raw(this), group.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RequestUnconstrainedUploadsAsync<P0>(operations: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<UploadOperation>>,
    {
        Self::IBackgroundUploaderUserConsent(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestUnconstrainedUploadsAsync)(windows_core::Interface::as_raw(this), operations.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderFactory<R, F: FnOnce(&IBackgroundUploaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundUploader, IBackgroundUploaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderStaticMethods<R, F: FnOnce(&IBackgroundUploaderStaticMethods) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderStaticMethods2<R, F: FnOnce(&IBackgroundUploaderStaticMethods2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundUploaderUserConsent<R, F: FnOnce(&IBackgroundUploaderUserConsent) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundUploader, IBackgroundUploaderUserConsent> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BackgroundUploader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundUploader>();
}
unsafe impl windows_core::Interface for BackgroundUploader {
    type Vtable = IBackgroundUploader_Vtbl;
    const IID: windows_core::GUID = <IBackgroundUploader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundUploader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundUploader";
}
unsafe impl Send for BackgroundUploader {}
unsafe impl Sync for BackgroundUploader {}
pub struct ContentPrefetcher;
impl ContentPrefetcher {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContentUris() -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetIndirectContentUri<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IContentPrefetcher(|this| unsafe { (windows_core::Interface::vtable(this).SetIndirectContentUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    pub fn IndirectContentUri() -> windows_core::Result<super::super::Foundation::Uri> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndirectContentUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LastSuccessfulPrefetchTime() -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        Self::IContentPrefetcherTime(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastSuccessfulPrefetchTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IContentPrefetcher<R, F: FnOnce(&IContentPrefetcher) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentPrefetcher, IContentPrefetcher> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContentPrefetcherTime<R, F: FnOnce(&IContentPrefetcherTime) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentPrefetcher, IContentPrefetcherTime> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ContentPrefetcher {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ContentPrefetcher";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DownloadOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DownloadOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DownloadOperation, IBackgroundTransferOperation, IBackgroundTransferOperationPriority);
impl DownloadOperation {
    pub fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Guid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestedUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CostPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCostPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResultStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResponseInformation(&self) -> windows_core::Result<ResponseInformation> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResponseInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Priority(&self) -> windows_core::Result<BackgroundTransferPriority> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Priority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPriority)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn ResultFile(&self) -> windows_core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<BackgroundDownloadProgress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttachAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttachAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Resume)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TransferGroup(&self) -> windows_core::Result<BackgroundTransferGroup> {
        let this = &windows_core::Interface::cast::<IDownloadOperation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsRandomAccessRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRandomAccessRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRandomAccessRequired(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRandomAccessRequired)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultRandomAccessStreamReference(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResultRandomAccessStreamReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDownloadedRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDownloadedRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RangesDownloaded<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RangesDownloaded)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRangesDownloaded(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveRangesDownloaded)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn SetRequestedUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestedUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))]
    pub fn RecoverableWebErrorStatuses(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>> {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecoverableWebErrorStatuses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web")]
    pub fn CurrentWebErrorStatus(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>> {
        let this = &windows_core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentWebErrorStatus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MakeCurrentInTransferGroup(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDownloadOperation4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).MakeCurrentInTransferGroup)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    pub fn RemoveRequestHeader(&self, headername: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername)).ok() }
    }
}
impl windows_core::RuntimeType for DownloadOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDownloadOperation>();
}
unsafe impl windows_core::Interface for DownloadOperation {
    type Vtable = IDownloadOperation_Vtbl;
    const IID: windows_core::GUID = <IDownloadOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DownloadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.DownloadOperation";
}
unsafe impl Send for DownloadOperation {}
unsafe impl Sync for DownloadOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ResponseInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResponseInformation, windows_core::IUnknown, windows_core::IInspectable);
impl ResponseInformation {
    pub fn IsResumable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsResumable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ResponseInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResponseInformation>();
}
unsafe impl windows_core::Interface for ResponseInformation {
    type Vtable = IResponseInformation_Vtbl;
    const IID: windows_core::GUID = <IResponseInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResponseInformation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ResponseInformation";
}
unsafe impl Send for ResponseInformation {}
unsafe impl Sync for ResponseInformation {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UnconstrainedTransferRequestResult(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(UnconstrainedTransferRequestResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl UnconstrainedTransferRequestResult {
    #[cfg(feature = "deprecated")]
    pub fn IsUnconstrained(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUnconstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for UnconstrainedTransferRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUnconstrainedTransferRequestResult>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for UnconstrainedTransferRequestResult {
    type Vtable = IUnconstrainedTransferRequestResult_Vtbl;
    const IID: windows_core::GUID = <IUnconstrainedTransferRequestResult as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for UnconstrainedTransferRequestResult {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for UnconstrainedTransferRequestResult {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for UnconstrainedTransferRequestResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UploadOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UploadOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UploadOperation, IBackgroundTransferOperation, IBackgroundTransferOperationPriority);
impl UploadOperation {
    pub fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Guid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestedUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Method(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CostPolicy(&self) -> windows_core::Result<BackgroundTransferCostPolicy> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CostPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCostPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResultStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResponseInformation(&self) -> windows_core::Result<ResponseInformation> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResponseInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Priority(&self) -> windows_core::Result<BackgroundTransferPriority> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Priority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPriority)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> windows_core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<BackgroundUploadProgress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttachAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttachAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransferGroup(&self) -> windows_core::Result<BackgroundTransferGroup> {
        let this = &windows_core::Interface::cast::<IUploadOperation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MakeCurrentInTransferGroup(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUploadOperation3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).MakeCurrentInTransferGroup)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    pub fn RemoveRequestHeader(&self, headername: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername)).ok() }
    }
}
impl windows_core::RuntimeType for UploadOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUploadOperation>();
}
unsafe impl windows_core::Interface for UploadOperation {
    type Vtable = IUploadOperation_Vtbl;
    const IID: windows_core::GUID = <IUploadOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UploadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UploadOperation";
}
unsafe impl Send for UploadOperation {}
unsafe impl Sync for UploadOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundTransferBehavior(pub i32);
impl BackgroundTransferBehavior {
    pub const Parallel: Self = Self(0i32);
    pub const Serialized: Self = Self(1i32);
}
impl windows_core::TypeKind for BackgroundTransferBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundTransferBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundTransferBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundTransferBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundTransferCostPolicy(pub i32);
impl BackgroundTransferCostPolicy {
    pub const Default: Self = Self(0i32);
    pub const UnrestrictedOnly: Self = Self(1i32);
    pub const Always: Self = Self(2i32);
}
impl windows_core::TypeKind for BackgroundTransferCostPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundTransferCostPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundTransferCostPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundTransferCostPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferCostPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundTransferPriority(pub i32);
impl BackgroundTransferPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl windows_core::TypeKind for BackgroundTransferPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundTransferPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundTransferPriority").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundTransferPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferPriority;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundTransferStatus(pub i32);
impl BackgroundTransferStatus {
    pub const Idle: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const PausedByApplication: Self = Self(2i32);
    pub const PausedCostedNetwork: Self = Self(3i32);
    pub const PausedNoNetwork: Self = Self(4i32);
    pub const Completed: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const PausedRecoverableWebErrorStatus: Self = Self(8i32);
    pub const PausedSystemPolicy: Self = Self(32i32);
}
impl windows_core::TypeKind for BackgroundTransferStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundTransferStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundTransferStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundTransferStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundDownloadProgress {
    pub BytesReceived: u64,
    pub TotalBytesToReceive: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl windows_core::TypeKind for BackgroundDownloadProgress {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BackgroundDownloadProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundDownloadProgress;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
}
impl Default for BackgroundDownloadProgress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundTransferFileRange {
    pub Offset: u64,
    pub Length: u64,
}
impl windows_core::TypeKind for BackgroundTransferFileRange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BackgroundTransferFileRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange;u8;u8)");
}
impl Default for BackgroundTransferFileRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundUploadProgress {
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub TotalBytesToReceive: u64,
    pub TotalBytesToSend: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl windows_core::TypeKind for BackgroundUploadProgress {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BackgroundUploadProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundUploadProgress;u8;u8;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
}
impl Default for BackgroundUploadProgress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
