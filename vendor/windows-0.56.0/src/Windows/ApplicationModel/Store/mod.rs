#[cfg(feature = "ApplicationModel_Store_LicenseManagement")]
pub mod LicenseManagement;
#[cfg(feature = "ApplicationModel_Store_Preview")]
pub mod Preview;
windows_core::imp::define_interface!(ICurrentApp, ICurrentApp_Vtbl, 0xd52dc065_da3f_4685_995e_9b482eb5e603);
impl windows_core::RuntimeType for ICurrentApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentApp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LicenseInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LinkUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestProductPurchaseAsync: usize,
    pub LoadListingInformationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppReceiptAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProductReceiptAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrentApp2Statics, ICurrentApp2Statics_Vtbl, 0xdf4e6e2d_3171_4ad3_8614_2c61244373cb);
impl windows_core::RuntimeType for ICurrentApp2Statics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentApp2Statics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCustomerPurchaseIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCustomerCollectionsIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrentAppSimulator, ICurrentAppSimulator_Vtbl, 0xf17f9db1_74cd_4787_9787_19866e9a5559);
impl windows_core::RuntimeType for ICurrentAppSimulator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentAppSimulator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LicenseInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LinkUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RequestAppPurchaseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub RequestProductPurchaseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestProductPurchaseAsync: usize,
    pub LoadListingInformationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppReceiptAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProductReceiptAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub ReloadSimulatorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ReloadSimulatorAsync: usize,
}
windows_core::imp::define_interface!(ICurrentAppSimulatorStaticsWithFiltering, ICurrentAppSimulatorStaticsWithFiltering_Vtbl, 0x617e70e2_f86f_4b54_9666_dde285092c68);
impl windows_core::RuntimeType for ICurrentAppSimulatorStaticsWithFiltering {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentAppSimulatorStaticsWithFiltering_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByKeywordsAsync: usize,
}
windows_core::imp::define_interface!(ICurrentAppSimulatorWithCampaignId, ICurrentAppSimulatorWithCampaignId_Vtbl, 0x84678a43_df00_4672_a43f_b25b1441cfcf);
impl windows_core::RuntimeType for ICurrentAppSimulatorWithCampaignId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentAppSimulatorWithCampaignId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrentAppSimulatorWithConsumables, ICurrentAppSimulatorWithConsumables_Vtbl, 0x4e51f0ab_20e7_4412_9b85_59bb78388667);
impl windows_core::RuntimeType for ICurrentAppSimulatorWithConsumables {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentAppSimulatorWithConsumables_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnfulfilledConsumablesAsync: usize,
}
windows_core::imp::define_interface!(ICurrentAppStaticsWithFiltering, ICurrentAppStaticsWithFiltering_Vtbl, 0xd36d6542_9085_438e_97ba_a25c976be2fd);
impl windows_core::RuntimeType for ICurrentAppStaticsWithFiltering {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentAppStaticsWithFiltering_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByProductIdsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByProductIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadListingInformationByKeywordsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadListingInformationByKeywordsAsync: usize,
    pub ReportProductFulfillment: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrentAppWithCampaignId, ICurrentAppWithCampaignId_Vtbl, 0x312f4cd0_36c1_44a6_b32b_432d608e4dd6);
impl windows_core::RuntimeType for ICurrentAppWithCampaignId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentAppWithCampaignId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAppPurchaseCampaignIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICurrentAppWithConsumables, ICurrentAppWithConsumables_Vtbl, 0x844e0071_9e4f_4f79_995a_5f91172e6cef);
impl windows_core::RuntimeType for ICurrentAppWithConsumables {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICurrentAppWithConsumables_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportConsumableFulfillmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestProductPurchaseWithResultsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestProductPurchaseWithDisplayPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnfulfilledConsumablesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnfulfilledConsumablesAsync: usize,
}
windows_core::imp::define_interface!(ILicenseInformation, ILicenseInformation_Vtbl, 0x8eb7dc30_f170_4ed5_8e21_1516da3fd367);
impl windows_core::RuntimeType for ILicenseInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILicenseInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProductLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProductLicenses: usize,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTrial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub LicenseChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLicenseChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IListingInformation, IListingInformation_Vtbl, 0x588b4abf_bc74_4383_b78c_99606323dece);
impl windows_core::RuntimeType for IListingInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IListingInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentMarket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProductListings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProductListings: usize,
    pub FormattedPrice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AgeRating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IListingInformation2, IListingInformation2_Vtbl, 0xc0fd2c1d_b30e_4384_84ea_72fefa82223e);
impl windows_core::RuntimeType for IListingInformation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IListingInformation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SaleEndDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductLicense, IProductLicense_Vtbl, 0x363308c7_2bcf_4c0e_8f2f_e808aaa8f99d);
impl windows_core::RuntimeType for IProductLicense {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductLicense_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductLicenseWithFulfillment, IProductLicenseWithFulfillment_Vtbl, 0xfc535c8a_f667_40f3_ba3c_045a63abb3ac);
impl windows_core::RuntimeType for IProductLicenseWithFulfillment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductLicenseWithFulfillment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsConsumable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductListing, IProductListing_Vtbl, 0x45a7d6ad_c750_4d9c_947c_b00dcbf9e9c2);
impl windows_core::RuntimeType for IProductListing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductListing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FormattedPrice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductListing2, IProductListing2_Vtbl, 0xf89e290f_73fe_494d_a939_08a9b2495abe);
impl windows_core::RuntimeType for IProductListing2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductListing2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormattedBasePrice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SaleEndDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub IsOnSale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductListingWithConsumables, IProductListingWithConsumables_Vtbl, 0xeb9e9790_8f6b_481f_93a7_5c3a63068149);
impl windows_core::RuntimeType for IProductListingWithConsumables {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductListingWithConsumables_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProductType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductListingWithMetadata, IProductListingWithMetadata_Vtbl, 0x124da567_23f8_423e_9532_189943c40ace);
impl windows_core::RuntimeType for IProductListingWithMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductListingWithMetadata_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Keywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Keywords: usize,
    pub ProductType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProductType) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ImageUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductPurchaseDisplayProperties, IProductPurchaseDisplayProperties_Vtbl, 0xd70b7420_bc92_401b_a809_c9b2e5dbbdaf);
impl windows_core::RuntimeType for IProductPurchaseDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductPurchaseDisplayProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProductPurchaseDisplayPropertiesFactory, IProductPurchaseDisplayPropertiesFactory_Vtbl, 0x6f491df4_32d6_4b40_b474_b83038a4d9cf);
impl windows_core::RuntimeType for IProductPurchaseDisplayPropertiesFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProductPurchaseDisplayPropertiesFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateProductPurchaseDisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPurchaseResults, IPurchaseResults_Vtbl, 0xed50b37e_8656_4f65_b8c8_ac7e0cb1a1c2);
impl windows_core::RuntimeType for IPurchaseResults {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPurchaseResults_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProductPurchaseStatus) -> windows_core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ReceiptXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OfferId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnfulfilledConsumable, IUnfulfilledConsumable_Vtbl, 0x2df7fbbb_1cdd_4cb8_a014_7b9cf8986927);
impl windows_core::RuntimeType for IUnfulfilledConsumable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUnfulfilledConsumable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub OfferId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
pub struct CurrentApp;
impl CurrentApp {
    pub fn LicenseInformation() -> windows_core::Result<LicenseInformation> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LinkUri() -> windows_core::Result<super::super::Foundation::Uri> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LinkUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AppId() -> windows_core::Result<windows_core::GUID> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RequestAppPurchaseAsync(includereceipt: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAppPurchaseAsync)(windows_core::Interface::as_raw(this), includereceipt, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RequestProductPurchaseAsync(productid: &windows_core::HSTRING, includereceipt: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestProductPurchaseAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), includereceipt, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LoadListingInformationAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadListingInformationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAppReceiptAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppReceiptAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetProductReceiptAsync(productid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentApp(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProductReceiptAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCustomerPurchaseIdAsync(serviceticket: &windows_core::HSTRING, publisheruserid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentApp2Statics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCustomerPurchaseIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(serviceticket), core::mem::transmute_copy(publisheruserid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCustomerCollectionsIdAsync(serviceticket: &windows_core::HSTRING, publisheruserid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentApp2Statics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCustomerCollectionsIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(serviceticket), core::mem::transmute_copy(publisheruserid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByProductIdsAsync<P0>(productids: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadListingInformationByProductIdsAsync)(windows_core::Interface::as_raw(this), productids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByKeywordsAsync<P0>(keywords: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadListingInformationByKeywordsAsync)(windows_core::Interface::as_raw(this), keywords.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ReportProductFulfillment(productid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::ICurrentAppStaticsWithFiltering(|this| unsafe { (windows_core::Interface::vtable(this).ReportProductFulfillment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid)).ok() })
    }
    pub fn GetAppPurchaseCampaignIdAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentAppWithCampaignId(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppPurchaseCampaignIdAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ReportConsumableFulfillmentAsync(productid: &windows_core::HSTRING, transactionid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), transactionid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestProductPurchaseWithResultsAsync(productid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestProductPurchaseWithResultsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestProductPurchaseWithDisplayPropertiesAsync<P0>(productid: &windows_core::HSTRING, offerid: &windows_core::HSTRING, displayproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>
    where
        P0: windows_core::Param<ProductPurchaseDisplayProperties>,
    {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestProductPurchaseWithDisplayPropertiesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(offerid), displayproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnfulfilledConsumablesAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>> {
        Self::ICurrentAppWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUnfulfilledConsumablesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICurrentApp<R, F: FnOnce(&ICurrentApp) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentApp, ICurrentApp> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentApp2Statics<R, F: FnOnce(&ICurrentApp2Statics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentApp, ICurrentApp2Statics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppStaticsWithFiltering<R, F: FnOnce(&ICurrentAppStaticsWithFiltering) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentApp, ICurrentAppStaticsWithFiltering> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppWithCampaignId<R, F: FnOnce(&ICurrentAppWithCampaignId) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentApp, ICurrentAppWithCampaignId> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppWithConsumables<R, F: FnOnce(&ICurrentAppWithConsumables) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentApp, ICurrentAppWithConsumables> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CurrentApp {
    const NAME: &'static str = "Windows.ApplicationModel.Store.CurrentApp";
}
pub struct CurrentAppSimulator;
impl CurrentAppSimulator {
    pub fn LicenseInformation() -> windows_core::Result<LicenseInformation> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LinkUri() -> windows_core::Result<super::super::Foundation::Uri> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LinkUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AppId() -> windows_core::Result<windows_core::GUID> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RequestAppPurchaseAsync(includereceipt: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAppPurchaseAsync)(windows_core::Interface::as_raw(this), includereceipt, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RequestProductPurchaseAsync(productid: &windows_core::HSTRING, includereceipt: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestProductPurchaseAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), includereceipt, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LoadListingInformationAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadListingInformationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAppReceiptAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppReceiptAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetProductReceiptAsync(productid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProductReceiptAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage")]
    pub fn ReloadSimulatorAsync<P0>(simulatorsettingsfile: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::StorageFile>,
    {
        Self::ICurrentAppSimulator(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ReloadSimulatorAsync)(windows_core::Interface::as_raw(this), simulatorsettingsfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByProductIdsAsync<P0>(productids: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ICurrentAppSimulatorStaticsWithFiltering(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadListingInformationByProductIdsAsync)(windows_core::Interface::as_raw(this), productids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadListingInformationByKeywordsAsync<P0>(keywords: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ListingInformation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ICurrentAppSimulatorStaticsWithFiltering(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadListingInformationByKeywordsAsync)(windows_core::Interface::as_raw(this), keywords.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAppPurchaseCampaignIdAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::ICurrentAppSimulatorWithCampaignId(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppPurchaseCampaignIdAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ReportConsumableFulfillmentAsync(productid: &windows_core::HSTRING, transactionid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncOperation<FulfillmentResult>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportConsumableFulfillmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), transactionid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestProductPurchaseWithResultsAsync(productid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestProductPurchaseWithResultsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestProductPurchaseWithDisplayPropertiesAsync<P0>(productid: &windows_core::HSTRING, offerid: &windows_core::HSTRING, displayproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PurchaseResults>>
    where
        P0: windows_core::Param<ProductPurchaseDisplayProperties>,
    {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestProductPurchaseWithDisplayPropertiesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(offerid), displayproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnfulfilledConsumablesAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UnfulfilledConsumable>>> {
        Self::ICurrentAppSimulatorWithConsumables(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUnfulfilledConsumablesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulator<R, F: FnOnce(&ICurrentAppSimulator) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulator> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulatorStaticsWithFiltering<R, F: FnOnce(&ICurrentAppSimulatorStaticsWithFiltering) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorStaticsWithFiltering> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulatorWithCampaignId<R, F: FnOnce(&ICurrentAppSimulatorWithCampaignId) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorWithCampaignId> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICurrentAppSimulatorWithConsumables<R, F: FnOnce(&ICurrentAppSimulatorWithConsumables) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CurrentAppSimulator, ICurrentAppSimulatorWithConsumables> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CurrentAppSimulator {
    const NAME: &'static str = "Windows.ApplicationModel.Store.CurrentAppSimulator";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LicenseInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LicenseInformation, windows_core::IUnknown, windows_core::IInspectable);
impl LicenseInformation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProductLicenses(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ProductLicense>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductLicenses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTrial(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTrial)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExpirationDate(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LicenseChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<LicenseChangedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLicenseChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLicenseChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for LicenseInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILicenseInformation>();
}
unsafe impl windows_core::Interface for LicenseInformation {
    type Vtable = ILicenseInformation_Vtbl;
    const IID: windows_core::GUID = <ILicenseInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LicenseInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseInformation";
}
unsafe impl Send for LicenseInformation {}
unsafe impl Sync for LicenseInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ListingInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ListingInformation, windows_core::IUnknown, windows_core::IInspectable);
impl ListingInformation {
    pub fn CurrentMarket(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentMarket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn ProductListings(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ProductListing>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductListings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormattedPrice(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedPrice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AgeRating(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AgeRating)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FormattedBasePrice(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedBasePrice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaleEndDate(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SaleEndDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOnSale(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOnSale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrencyCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IListingInformation2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrencyCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ListingInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IListingInformation>();
}
unsafe impl windows_core::Interface for ListingInformation {
    type Vtable = IListingInformation_Vtbl;
    const IID: windows_core::GUID = <IListingInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ListingInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ListingInformation";
}
unsafe impl Send for ListingInformation {}
unsafe impl Sync for ListingInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProductLicense(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProductLicense, windows_core::IUnknown, windows_core::IInspectable);
impl ProductLicense {
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExpirationDate(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConsumable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IProductLicenseWithFulfillment>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConsumable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ProductLicense {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProductLicense>();
}
unsafe impl windows_core::Interface for ProductLicense {
    type Vtable = IProductLicense_Vtbl;
    const IID: windows_core::GUID = <IProductLicense as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProductLicense {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductLicense";
}
unsafe impl Send for ProductLicense {}
unsafe impl Sync for ProductLicense {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProductListing(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProductListing, windows_core::IUnknown, windows_core::IInspectable);
impl ProductListing {
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormattedPrice(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedPrice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FormattedBasePrice(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).FormattedBasePrice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaleEndDate(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SaleEndDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOnSale(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOnSale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrencyCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IProductListing2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrencyCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Keywords(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Keywords)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProductType(&self) -> windows_core::Result<ProductType> {
        let this = &windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImageUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IProductListingWithMetadata>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProductListing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProductListing>();
}
unsafe impl windows_core::Interface for ProductListing {
    type Vtable = IProductListing_Vtbl;
    const IID: windows_core::GUID = <IProductListing as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProductListing {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductListing";
}
unsafe impl Send for ProductListing {}
unsafe impl Sync for ProductListing {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProductPurchaseDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProductPurchaseDisplayProperties, windows_core::IUnknown, windows_core::IInspectable);
impl ProductPurchaseDisplayProperties {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProductPurchaseDisplayProperties, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Image(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Image)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CreateProductPurchaseDisplayProperties(name: &windows_core::HSTRING) -> windows_core::Result<ProductPurchaseDisplayProperties> {
        Self::IProductPurchaseDisplayPropertiesFactory(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateProductPurchaseDisplayProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IProductPurchaseDisplayPropertiesFactory<R, F: FnOnce(&IProductPurchaseDisplayPropertiesFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProductPurchaseDisplayProperties, IProductPurchaseDisplayPropertiesFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProductPurchaseDisplayProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProductPurchaseDisplayProperties>();
}
unsafe impl windows_core::Interface for ProductPurchaseDisplayProperties {
    type Vtable = IProductPurchaseDisplayProperties_Vtbl;
    const IID: windows_core::GUID = <IProductPurchaseDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProductPurchaseDisplayProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Store.ProductPurchaseDisplayProperties";
}
unsafe impl Send for ProductPurchaseDisplayProperties {}
unsafe impl Sync for ProductPurchaseDisplayProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PurchaseResults(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PurchaseResults, windows_core::IUnknown, windows_core::IInspectable);
impl PurchaseResults {
    pub fn Status(&self) -> windows_core::Result<ProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransactionId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).TransactionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReceiptXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceiptXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OfferId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).OfferId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PurchaseResults {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPurchaseResults>();
}
unsafe impl windows_core::Interface for PurchaseResults {
    type Vtable = IPurchaseResults_Vtbl;
    const IID: windows_core::GUID = <IPurchaseResults as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.PurchaseResults";
}
unsafe impl Send for PurchaseResults {}
unsafe impl Sync for PurchaseResults {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UnfulfilledConsumable(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UnfulfilledConsumable, windows_core::IUnknown, windows_core::IInspectable);
impl UnfulfilledConsumable {
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransactionId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).TransactionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OfferId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).OfferId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UnfulfilledConsumable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUnfulfilledConsumable>();
}
unsafe impl windows_core::Interface for UnfulfilledConsumable {
    type Vtable = IUnfulfilledConsumable_Vtbl;
    const IID: windows_core::GUID = <IUnfulfilledConsumable as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UnfulfilledConsumable {
    const NAME: &'static str = "Windows.ApplicationModel.Store.UnfulfilledConsumable";
}
unsafe impl Send for UnfulfilledConsumable {}
unsafe impl Sync for UnfulfilledConsumable {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FulfillmentResult(pub i32);
impl FulfillmentResult {
    pub const Succeeded: Self = Self(0i32);
    pub const NothingToFulfill: Self = Self(1i32);
    pub const PurchasePending: Self = Self(2i32);
    pub const PurchaseReverted: Self = Self(3i32);
    pub const ServerError: Self = Self(4i32);
}
impl windows_core::TypeKind for FulfillmentResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FulfillmentResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FulfillmentResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FulfillmentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.FulfillmentResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ProductPurchaseStatus(pub i32);
impl ProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl windows_core::TypeKind for ProductPurchaseStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ProductPurchaseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ProductPurchaseStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ProductPurchaseStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.ProductPurchaseStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ProductType(pub i32);
impl ProductType {
    pub const Unknown: Self = Self(0i32);
    pub const Durable: Self = Self(1i32);
    pub const Consumable: Self = Self(2i32);
}
impl windows_core::TypeKind for ProductType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ProductType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ProductType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ProductType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.ProductType;i4)");
}
windows_core::imp::define_interface!(LicenseChangedEventHandler, LicenseChangedEventHandler_Vtbl, 0xd4a50255_1369_4c36_832f_6f2d88e3659b);
impl LicenseChangedEventHandler {
    pub fn new<F: FnMut() -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = LicenseChangedEventHandlerBox::<F> { vtable: &LicenseChangedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct LicenseChangedEventHandlerBox<F: FnMut() -> windows_core::Result<()> + Send + 'static> {
    vtable: *const LicenseChangedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut() -> windows_core::Result<()> + Send + 'static> LicenseChangedEventHandlerBox<F> {
    const VTABLE: LicenseChangedEventHandler_Vtbl = LicenseChangedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <LicenseChangedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)().into()
    }
}
impl windows_core::RuntimeType for LicenseChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct LicenseChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
