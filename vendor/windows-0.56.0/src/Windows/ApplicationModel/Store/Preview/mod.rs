#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
windows_core::imp::define_interface!(IDeliveryOptimizationSettings, IDeliveryOptimizationSettings_Vtbl, 0x1810fda0_e853_565e_b874_7a8a7b9a0e0f);
impl windows_core::RuntimeType for IDeliveryOptimizationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeliveryOptimizationSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DownloadMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeliveryOptimizationDownloadMode) -> windows_core::HRESULT,
    pub DownloadModeSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeliveryOptimizationDownloadModeSource) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeliveryOptimizationSettingsStatics, IDeliveryOptimizationSettingsStatics_Vtbl, 0x5c817caf_aed5_5999_b4c9_8c60898bc4f3);
impl windows_core::RuntimeType for IDeliveryOptimizationSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeliveryOptimizationSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStoreConfigurationStatics, IStoreConfigurationStatics_Vtbl, 0x728f7fc0_8628_42ec_84a2_07780eb44d8b);
impl windows_core::RuntimeType for IStoreConfigurationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStoreConfigurationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSystemConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, super::super::super::Foundation::DateTime, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetMobileOperatorConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, u32, u32) -> windows_core::HRESULT,
    pub SetStoreWebAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsStoreWebAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub HardwareManufacturerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FilterUnsupportedSystemFeaturesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FilterUnsupportedSystemFeaturesAsync: usize,
}
windows_core::imp::define_interface!(IStoreConfigurationStatics2, IStoreConfigurationStatics2_Vtbl, 0x657c4595_c8b7_4fe9_9f4c_4d71027d347e);
impl windows_core::RuntimeType for IStoreConfigurationStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStoreConfigurationStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PurchasePromptingPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPurchasePromptingPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStoreConfigurationStatics3, IStoreConfigurationStatics3_Vtbl, 0x6d45f57c_f144_4cb5_9d3f_4eb05e30b6d3);
impl windows_core::RuntimeType for IStoreConfigurationStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStoreConfigurationStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasStoreWebAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub HasStoreWebAccountForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    HasStoreWebAccountForUser: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetStoreLogDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, StoreLogOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetStoreLogDataAsync: usize,
    #[cfg(feature = "System")]
    pub SetStoreWebAccountIdForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetStoreWebAccountIdForUser: usize,
    #[cfg(feature = "System")]
    pub IsStoreWebAccountIdForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsStoreWebAccountIdForUser: usize,
    #[cfg(feature = "System")]
    pub GetPurchasePromptingPolicyForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetPurchasePromptingPolicyForUser: usize,
    #[cfg(feature = "System")]
    pub SetPurchasePromptingPolicyForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetPurchasePromptingPolicyForUser: usize,
}
windows_core::imp::define_interface!(IStoreConfigurationStatics4, IStoreConfigurationStatics4_Vtbl, 0x20ff56d2_4ee3_4cf0_9b12_552c03310f75);
impl windows_core::RuntimeType for IStoreConfigurationStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStoreConfigurationStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStoreWebAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetStoreWebAccountIdForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetStoreWebAccountIdForUser: usize,
    pub SetEnterpriseStoreWebAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub SetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetEnterpriseStoreWebAccountIdForUser: usize,
    pub GetEnterpriseStoreWebAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetEnterpriseStoreWebAccountIdForUser: usize,
    pub ShouldRestrictToEnterpriseStoreOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub ShouldRestrictToEnterpriseStoreOnlyForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ShouldRestrictToEnterpriseStoreOnlyForUser: usize,
}
windows_core::imp::define_interface!(IStoreConfigurationStatics5, IStoreConfigurationStatics5_Vtbl, 0xf7613191_8fa9_49db_822b_0160e7e4e5c5);
impl windows_core::RuntimeType for IStoreConfigurationStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStoreConfigurationStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPinToDesktopSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPinToTaskbarSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPinToStartSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PinToDesktop: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub PinToDesktopForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    PinToDesktopForUser: usize,
}
windows_core::imp::define_interface!(IStoreHardwareManufacturerInfo, IStoreHardwareManufacturerInfo_Vtbl, 0xf292dc08_c654_43ac_a21f_34801c9d3388);
impl windows_core::RuntimeType for IStoreHardwareManufacturerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStoreHardwareManufacturerInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HardwareManufacturerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StoreContentModifierId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorePreview, IStorePreview_Vtbl, 0x8a157241_840e_49a9_bc01_5d5b01fbc8e9);
impl windows_core::RuntimeType for IStorePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorePreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestProductPurchaseByProductIdAndSkuIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadAddOnProductInfosAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadAddOnProductInfosAsync: usize,
}
windows_core::imp::define_interface!(IStorePreviewProductInfo, IStorePreviewProductInfo_Vtbl, 0x1937dbb3_6c01_4c9d_85cd_5babaac2b351);
impl windows_core::RuntimeType for IStorePreviewProductInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorePreviewProductInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProductType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SkuInfoList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SkuInfoList: usize,
}
windows_core::imp::define_interface!(IStorePreviewPurchaseResults, IStorePreviewPurchaseResults_Vtbl, 0xb0daaed1_d6c5_4e53_a043_fba0d8e61231);
impl windows_core::RuntimeType for IStorePreviewPurchaseResults {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorePreviewPurchaseResults_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductPurchaseStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorePreviewProductPurchaseStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorePreviewSkuInfo, IStorePreviewSkuInfo_Vtbl, 0x81fd76e2_0b26_48d9_98ce_27461c669d6c);
impl windows_core::RuntimeType for IStorePreviewSkuInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorePreviewSkuInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SkuId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SkuType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FormattedListPrice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExtendedData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAuthenticationCoreManagerHelper, IWebAuthenticationCoreManagerHelper_Vtbl, 0x06a50525_e715_4123_9276_9d6f865ba55f);
impl windows_core::RuntimeType for IWebAuthenticationCoreManagerHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerHelper_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Authentication_Web_Core", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAsync: usize,
    #[cfg(all(feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAndWebAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAndWebAccountAsync: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeliveryOptimizationSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeliveryOptimizationSettings, windows_core::IUnknown, windows_core::IInspectable);
impl DeliveryOptimizationSettings {
    pub fn DownloadMode(&self) -> windows_core::Result<DeliveryOptimizationDownloadMode> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DownloadModeSource(&self) -> windows_core::Result<DeliveryOptimizationDownloadModeSource> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadModeSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentSettings() -> windows_core::Result<DeliveryOptimizationSettings> {
        Self::IDeliveryOptimizationSettingsStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeliveryOptimizationSettingsStatics<R, F: FnOnce(&IDeliveryOptimizationSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeliveryOptimizationSettings, IDeliveryOptimizationSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeliveryOptimizationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeliveryOptimizationSettings>();
}
unsafe impl windows_core::Interface for DeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_Vtbl;
    const IID: windows_core::GUID = <IDeliveryOptimizationSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeliveryOptimizationSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings";
}
unsafe impl Send for DeliveryOptimizationSettings {}
unsafe impl Sync for DeliveryOptimizationSettings {}
pub struct StoreConfiguration;
impl StoreConfiguration {
    pub fn SetSystemConfiguration(cataloghardwaremanufacturerid: &windows_core::HSTRING, catalogstorecontentmodifierid: &windows_core::HSTRING, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetSystemConfiguration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(cataloghardwaremanufacturerid), core::mem::transmute_copy(catalogstorecontentmodifierid), systemconfigurationexpiration, core::mem::transmute_copy(cataloghardwaredescriptor)).ok() })
    }
    pub fn SetMobileOperatorConfiguration(mobileoperatorid: &windows_core::HSTRING, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> windows_core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetMobileOperatorConfiguration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mobileoperatorid), appdownloadlimitinmegabytes, updatedownloadlimitinmegabytes).ok() })
    }
    pub fn SetStoreWebAccountId(webaccountid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetStoreWebAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountid)).ok() })
    }
    pub fn IsStoreWebAccountId(webaccountid: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStoreWebAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountid), &mut result__).map(|| result__)
        })
    }
    pub fn HardwareManufacturerInfo() -> windows_core::Result<StoreHardwareManufacturerInfo> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareManufacturerInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FilterUnsupportedSystemFeaturesAsync<P0>(systemfeatures: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>,
    {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).FilterUnsupportedSystemFeaturesAsync)(windows_core::Interface::as_raw(this), systemfeatures.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PurchasePromptingPolicy() -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics2(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PurchasePromptingPolicy)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetPurchasePromptingPolicy<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        Self::IStoreConfigurationStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetPurchasePromptingPolicy)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    pub fn HasStoreWebAccount() -> windows_core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).HasStoreWebAccount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn HasStoreWebAccountForUser<P0>(user: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).HasStoreWebAccountForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetStoreLogDataAsync(options: StoreLogOptions) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStoreLogDataAsync)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn SetStoreWebAccountIdForUser<P0>(user: P0, webaccountid: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe { (windows_core::Interface::vtable(this).SetStoreWebAccountIdForUser)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(webaccountid)).ok() })
    }
    #[cfg(feature = "System")]
    pub fn IsStoreWebAccountIdForUser<P0>(user: P0, webaccountid: &windows_core::HSTRING) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStoreWebAccountIdForUser)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(webaccountid), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetPurchasePromptingPolicyForUser<P0>(user: P0) -> windows_core::Result<super::super::super::Foundation::IReference<u32>>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPurchasePromptingPolicyForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn SetPurchasePromptingPolicyForUser<P0, P1>(user: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::User>,
        P1: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe { (windows_core::Interface::vtable(this).SetPurchasePromptingPolicyForUser)(windows_core::Interface::as_raw(this), user.param().abi(), value.param().abi()).ok() })
    }
    pub fn GetStoreWebAccountId() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStoreWebAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetStoreWebAccountIdForUser<P0>(user: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStoreWebAccountIdForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetEnterpriseStoreWebAccountId(webaccountid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (windows_core::Interface::vtable(this).SetEnterpriseStoreWebAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountid)).ok() })
    }
    #[cfg(feature = "System")]
    pub fn SetEnterpriseStoreWebAccountIdForUser<P0>(user: P0, webaccountid: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe { (windows_core::Interface::vtable(this).SetEnterpriseStoreWebAccountIdForUser)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(webaccountid)).ok() })
    }
    pub fn GetEnterpriseStoreWebAccountId() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEnterpriseStoreWebAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetEnterpriseStoreWebAccountIdForUser<P0>(user: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEnterpriseStoreWebAccountIdForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShouldRestrictToEnterpriseStoreOnly() -> windows_core::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldRestrictToEnterpriseStoreOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn ShouldRestrictToEnterpriseStoreOnlyForUser<P0>(user: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldRestrictToEnterpriseStoreOnlyForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn IsPinToDesktopSupported() -> windows_core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPinToDesktopSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsPinToTaskbarSupported() -> windows_core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPinToTaskbarSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsPinToStartSupported() -> windows_core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPinToStartSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PinToDesktop(apppackagefamilyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (windows_core::Interface::vtable(this).PinToDesktop)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(apppackagefamilyname)).ok() })
    }
    #[cfg(feature = "System")]
    pub fn PinToDesktopForUser<P0>(user: P0, apppackagefamilyname: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics5(|this| unsafe { (windows_core::Interface::vtable(this).PinToDesktopForUser)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(apppackagefamilyname)).ok() })
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics<R, F: FnOnce(&IStoreConfigurationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics2<R, F: FnOnce(&IStoreConfigurationStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics3<R, F: FnOnce(&IStoreConfigurationStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics4<R, F: FnOnce(&IStoreConfigurationStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics5<R, F: FnOnce(&IStoreConfigurationStatics5) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics5> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StoreConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreConfiguration";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StoreHardwareManufacturerInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StoreHardwareManufacturerInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StoreHardwareManufacturerInfo {
    pub fn HardwareManufacturerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareManufacturerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StoreContentModifierId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreContentModifierId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ModelName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ManufacturerName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ManufacturerName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StoreHardwareManufacturerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStoreHardwareManufacturerInfo>();
}
unsafe impl windows_core::Interface for StoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_Vtbl;
    const IID: windows_core::GUID = <IStoreHardwareManufacturerInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StoreHardwareManufacturerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo";
}
unsafe impl Send for StoreHardwareManufacturerInfo {}
unsafe impl Sync for StoreHardwareManufacturerInfo {}
pub struct StorePreview;
impl StorePreview {
    pub fn RequestProductPurchaseByProductIdAndSkuIdAsync(productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestProductPurchaseByProductIdAndSkuIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadAddOnProductInfosAsync() -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAddOnProductInfosAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStorePreview<R, F: FnOnce(&IStorePreview) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorePreview, IStorePreview> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StorePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreview";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StorePreviewProductInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorePreviewProductInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorePreviewProductInfo {
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProductType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SkuInfoList(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SkuInfoList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorePreviewProductInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorePreviewProductInfo>();
}
unsafe impl windows_core::Interface for StorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_Vtbl;
    const IID: windows_core::GUID = <IStorePreviewProductInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorePreviewProductInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo";
}
unsafe impl Send for StorePreviewProductInfo {}
unsafe impl Sync for StorePreviewProductInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StorePreviewPurchaseResults(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorePreviewPurchaseResults, windows_core::IUnknown, windows_core::IInspectable);
impl StorePreviewPurchaseResults {
    pub fn ProductPurchaseStatus(&self) -> windows_core::Result<StorePreviewProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductPurchaseStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for StorePreviewPurchaseResults {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorePreviewPurchaseResults>();
}
unsafe impl windows_core::Interface for StorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_Vtbl;
    const IID: windows_core::GUID = <IStorePreviewPurchaseResults as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorePreviewPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults";
}
unsafe impl Send for StorePreviewPurchaseResults {}
unsafe impl Sync for StorePreviewPurchaseResults {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StorePreviewSkuInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorePreviewSkuInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorePreviewSkuInfo {
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SkuId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SkuId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SkuType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SkuType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CustomDeveloperData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomDeveloperData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrencyCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrencyCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormattedListPrice(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedListPrice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorePreviewSkuInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorePreviewSkuInfo>();
}
unsafe impl windows_core::Interface for StorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_Vtbl;
    const IID: windows_core::GUID = <IStorePreviewSkuInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorePreviewSkuInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo";
}
unsafe impl Send for StorePreviewSkuInfo {}
unsafe impl Sync for StorePreviewSkuInfo {}
pub struct WebAuthenticationCoreManagerHelper;
impl WebAuthenticationCoreManagerHelper {
    #[cfg(all(feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAsync<P0, P1>(request: P0, uielement: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>
    where
        P0: windows_core::Param<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>,
        P1: windows_core::Param<super::super::super::UI::Xaml::UIElement>,
    {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestTokenWithUIElementHostingAsync)(windows_core::Interface::as_raw(this), request.param().abi(), uielement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAndWebAccountAsync<P0, P1, P2>(request: P0, webaccount: P1, uielement: P2) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>
    where
        P0: windows_core::Param<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>,
        P1: windows_core::Param<super::super::super::Security::Credentials::WebAccount>,
        P2: windows_core::Param<super::super::super::UI::Xaml::UIElement>,
    {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestTokenWithUIElementHostingAndWebAccountAsync)(windows_core::Interface::as_raw(this), request.param().abi(), webaccount.param().abi(), uielement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerHelper<R, F: FnOnce(&IWebAuthenticationCoreManagerHelper) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAuthenticationCoreManagerHelper, IWebAuthenticationCoreManagerHelper> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebAuthenticationCoreManagerHelper {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: Self = Self(0i32);
    pub const HttpOnly: Self = Self(1i32);
    pub const Lan: Self = Self(2i32);
    pub const Group: Self = Self(3i32);
    pub const Internet: Self = Self(4i32);
    pub const Bypass: Self = Self(5i32);
}
impl windows_core::TypeKind for DeliveryOptimizationDownloadMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeliveryOptimizationDownloadMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeliveryOptimizationDownloadMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: Self = Self(0i32);
    pub const Policy: Self = Self(1i32);
}
impl windows_core::TypeKind for DeliveryOptimizationDownloadModeSource {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeliveryOptimizationDownloadModeSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadModeSource").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeliveryOptimizationDownloadModeSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadModeSource;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StoreLogOptions(pub u32);
impl StoreLogOptions {
    pub const None: Self = Self(0u32);
    pub const TryElevate: Self = Self(1u32);
}
impl windows_core::TypeKind for StoreLogOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StoreLogOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StoreLogOptions").field(&self.0).finish()
    }
}
impl StoreLogOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for StoreLogOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for StoreLogOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for StoreLogOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for StoreLogOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for StoreLogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for StoreLogOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreLogOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl windows_core::TypeKind for StorePreviewProductPurchaseStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorePreviewProductPurchaseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorePreviewProductPurchaseStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorePreviewProductPurchaseStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StorePreviewProductPurchaseStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: Self = Self(0i32);
    pub const ArchitectureX64: Self = Self(1i32);
    pub const ArchitectureArm: Self = Self(2i32);
    pub const DirectX9: Self = Self(3i32);
    pub const DirectX10: Self = Self(4i32);
    pub const DirectX11: Self = Self(5i32);
    pub const D3D12HardwareFL11: Self = Self(6i32);
    pub const D3D12HardwareFL12: Self = Self(7i32);
    pub const Memory300MB: Self = Self(8i32);
    pub const Memory750MB: Self = Self(9i32);
    pub const Memory1GB: Self = Self(10i32);
    pub const Memory2GB: Self = Self(11i32);
    pub const CameraFront: Self = Self(12i32);
    pub const CameraRear: Self = Self(13i32);
    pub const Gyroscope: Self = Self(14i32);
    pub const Hover: Self = Self(15i32);
    pub const Magnetometer: Self = Self(16i32);
    pub const Nfc: Self = Self(17i32);
    pub const Resolution720P: Self = Self(18i32);
    pub const ResolutionWvga: Self = Self(19i32);
    pub const ResolutionWvgaOr720P: Self = Self(20i32);
    pub const ResolutionWxga: Self = Self(21i32);
    pub const ResolutionWvgaOrWxga: Self = Self(22i32);
    pub const ResolutionWxgaOr720P: Self = Self(23i32);
    pub const Memory4GB: Self = Self(24i32);
    pub const Memory6GB: Self = Self(25i32);
    pub const Memory8GB: Self = Self(26i32);
    pub const Memory12GB: Self = Self(27i32);
    pub const Memory16GB: Self = Self(28i32);
    pub const Memory20GB: Self = Self(29i32);
    pub const VideoMemory2GB: Self = Self(30i32);
    pub const VideoMemory4GB: Self = Self(31i32);
    pub const VideoMemory6GB: Self = Self(32i32);
    pub const VideoMemory1GB: Self = Self(33i32);
    pub const ArchitectureArm64: Self = Self(34i32);
}
impl windows_core::TypeKind for StoreSystemFeature {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StoreSystemFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StoreSystemFeature").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StoreSystemFeature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreSystemFeature;i4)");
}
