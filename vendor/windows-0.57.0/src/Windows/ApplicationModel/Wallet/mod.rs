#[cfg(feature = "ApplicationModel_Wallet_System")]
pub mod System;
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletBarcode, IWalletBarcode_Vtbl, 0x4f857b29_de80_4ea4_a1cd_81cd084dac27);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletBarcode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletBarcode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Symbology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WalletBarcodeSymbology) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Symbology: usize,
    #[cfg(feature = "deprecated")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Value: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub GetImageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    GetImageAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletBarcodeFactory, IWalletBarcodeFactory_Vtbl, 0x30117161_ed9c_469e_bbfd_306c95ea7108);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletBarcodeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletBarcodeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateWalletBarcode: unsafe extern "system" fn(*mut core::ffi::c_void, WalletBarcodeSymbology, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWalletBarcode: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub CreateCustomWalletBarcode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    CreateCustomWalletBarcode: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItem, IWalletItem_Vtbl, 0x20b54be8_118d_4ec4_996c_b963e7bd3e74);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub IsAcknowledged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsAcknowledged: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsAcknowledged: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsAcknowledged: usize,
    #[cfg(feature = "deprecated")]
    pub IssuerDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IssuerDisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub SetIssuerDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIssuerDisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub LastUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LastUpdated: usize,
    #[cfg(feature = "deprecated")]
    pub SetLastUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLastUpdated: usize,
    #[cfg(feature = "deprecated")]
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WalletItemKind) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Kind: usize,
    #[cfg(feature = "deprecated")]
    pub Barcode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Barcode: usize,
    #[cfg(feature = "deprecated")]
    pub SetBarcode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBarcode: usize,
    #[cfg(feature = "deprecated")]
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExpirationDate: usize,
    #[cfg(feature = "deprecated")]
    pub SetExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetExpirationDate: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub Logo159x159: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    Logo159x159: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetLogo159x159: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetLogo159x159: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub Logo336x336: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    Logo336x336: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetLogo336x336: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetLogo336x336: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub Logo99x99: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    Logo99x99: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetLogo99x99: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetLogo99x99: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayMessage: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayMessage: usize,
    #[cfg(feature = "deprecated")]
    pub IsDisplayMessageLaunchable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsDisplayMessageLaunchable: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsDisplayMessageLaunchable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsDisplayMessageLaunchable: usize,
    #[cfg(feature = "deprecated")]
    pub LogoText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LogoText: usize,
    #[cfg(feature = "deprecated")]
    pub SetLogoText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLogoText: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub HeaderColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    HeaderColor: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub SetHeaderColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    SetHeaderColor: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub BodyColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    BodyColor: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub SetBodyColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    SetBodyColor: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub HeaderFontColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    HeaderFontColor: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub SetHeaderFontColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    SetHeaderFontColor: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub BodyFontColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    BodyFontColor: usize,
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub SetBodyFontColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI", feature = "deprecated")))]
    SetBodyFontColor: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub HeaderBackgroundImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    HeaderBackgroundImage: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetHeaderBackgroundImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetHeaderBackgroundImage: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub BodyBackgroundImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    BodyBackgroundImage: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetBodyBackgroundImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetBodyBackgroundImage: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub LogoImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    LogoImage: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetLogoImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetLogoImage: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub PromotionalImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    PromotionalImage: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetPromotionalImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetPromotionalImage: usize,
    #[cfg(feature = "deprecated")]
    pub RelevantDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RelevantDate: usize,
    #[cfg(feature = "deprecated")]
    pub SetRelevantDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetRelevantDate: usize,
    #[cfg(feature = "deprecated")]
    pub RelevantDateDisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RelevantDateDisplayMessage: usize,
    #[cfg(feature = "deprecated")]
    pub SetRelevantDateDisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetRelevantDateDisplayMessage: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub TransactionHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    TransactionHistory: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RelevantLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RelevantLocations: usize,
    #[cfg(feature = "deprecated")]
    pub IsMoreTransactionHistoryLaunchable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsMoreTransactionHistoryLaunchable: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsMoreTransactionHistoryLaunchable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsMoreTransactionHistoryLaunchable: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub DisplayProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    DisplayProperties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Verbs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Verbs: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItemCustomProperty, IWalletItemCustomProperty_Vtbl, 0xb94b40f3_fa00_40fd_98dc_9de46697f1e7);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItemCustomProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItemCustomProperty_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetName: usize,
    #[cfg(feature = "deprecated")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Value: usize,
    #[cfg(feature = "deprecated")]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetValue: usize,
    #[cfg(feature = "deprecated")]
    pub AutoDetectLinks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoDetectLinks: usize,
    #[cfg(feature = "deprecated")]
    pub SetAutoDetectLinks: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAutoDetectLinks: usize,
    #[cfg(feature = "deprecated")]
    pub DetailViewPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WalletDetailViewPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DetailViewPosition: usize,
    #[cfg(feature = "deprecated")]
    pub SetDetailViewPosition: unsafe extern "system" fn(*mut core::ffi::c_void, WalletDetailViewPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDetailViewPosition: usize,
    #[cfg(feature = "deprecated")]
    pub SummaryViewPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WalletSummaryViewPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SummaryViewPosition: usize,
    #[cfg(feature = "deprecated")]
    pub SetSummaryViewPosition: unsafe extern "system" fn(*mut core::ffi::c_void, WalletSummaryViewPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSummaryViewPosition: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItemCustomPropertyFactory, IWalletItemCustomPropertyFactory_Vtbl, 0xd0046a44_61a1_41aa_b259_a5610ab5d575);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItemCustomPropertyFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItemCustomPropertyFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateWalletItemCustomProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWalletItemCustomProperty: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItemFactory, IWalletItemFactory_Vtbl, 0x53e27470_4f0b_4a3e_99e5_0bbb1eab38d4);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateWalletItem: unsafe extern "system" fn(*mut core::ffi::c_void, WalletItemKind, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWalletItem: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItemStore, IWalletItemStore_Vtbl, 0x7160484b_6d49_48f8_91a9_40a1d0f13ef4);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItemStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItemStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AddAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ClearAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClearAsync: usize,
    #[cfg(feature = "deprecated")]
    pub GetWalletItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetWalletItemAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetItemsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetItemsWithKindAsync: unsafe extern "system" fn(*mut core::ffi::c_void, WalletItemKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetItemsWithKindAsync: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub ImportItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    ImportItemAsync: usize,
    #[cfg(feature = "deprecated")]
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeleteAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ShowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ShowItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowItemAsync: usize,
    #[cfg(feature = "deprecated")]
    pub UpdateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UpdateAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletItemStore2, IWalletItemStore2_Vtbl, 0x65e682f0_7009_4a15_bd54_4fff379bffe2);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletItemStore2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletItemStore2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ItemsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveItemsChanged: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletManagerStatics, IWalletManagerStatics_Vtbl, 0x5111d6b8_c9a4_4c64_b4dd_e1e548001c0d);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestStoreAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletRelevantLocation, IWalletRelevantLocation_Vtbl, 0x9fd8782a_e3f9_4de1_bab3_bb192e46b3f3);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletRelevantLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletRelevantLocation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "deprecated"))]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Geolocation::BasicGeoposition) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "deprecated")))]
    Position: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "deprecated"))]
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Devices::Geolocation::BasicGeoposition) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "deprecated")))]
    SetPosition: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayMessage: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayMessage: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletTransaction, IWalletTransaction_Vtbl, 0x40e1e940_2606_4519_81cb_bff1c60d1f79);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletTransaction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDescription: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayAmount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayAmount: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayAmount: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayAmount: usize,
    #[cfg(feature = "deprecated")]
    pub IgnoreTimeOfDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IgnoreTimeOfDay: usize,
    #[cfg(feature = "deprecated")]
    pub SetIgnoreTimeOfDay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIgnoreTimeOfDay: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayLocation: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayLocation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayLocation: usize,
    #[cfg(feature = "deprecated")]
    pub TransactionDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransactionDate: usize,
    #[cfg(feature = "deprecated")]
    pub SetTransactionDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTransactionDate: usize,
    #[cfg(feature = "deprecated")]
    pub IsLaunchable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsLaunchable: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsLaunchable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsLaunchable: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletVerb, IWalletVerb_Vtbl, 0x17b826d6_e3c1_4c74_8a94_217aadbc4884);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletVerb {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletVerb_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetName: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IWalletVerbFactory, IWalletVerbFactory_Vtbl, 0x76012771_be58_4d5e_83ed_58b1669c7ad9);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IWalletVerbFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IWalletVerbFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateWalletVerb: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWalletVerb: usize,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletBarcode(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletBarcode, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletBarcode {
    #[cfg(feature = "deprecated")]
    pub fn Symbology(&self) -> windows_core::Result<WalletBarcodeSymbology> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Symbology)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn GetImageAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetImageAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateWalletBarcode(symbology: WalletBarcodeSymbology, value: &windows_core::HSTRING) -> windows_core::Result<WalletBarcode> {
        Self::IWalletBarcodeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWalletBarcode)(windows_core::Interface::as_raw(this), symbology, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn CreateCustomWalletBarcode<P0>(streamtobarcodeimage: P0) -> windows_core::Result<WalletBarcode>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IWalletBarcodeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCustomWalletBarcode)(windows_core::Interface::as_raw(this), streamtobarcodeimage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IWalletBarcodeFactory<R, F: FnOnce(&IWalletBarcodeFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletBarcode, IWalletBarcodeFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletBarcode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletBarcode>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletBarcode {
    type Vtable = IWalletBarcode_Vtbl;
    const IID: windows_core::GUID = <IWalletBarcode as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletBarcode {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletBarcode";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletBarcode {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletBarcode {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletItem(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletItem, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletItem {
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsAcknowledged(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAcknowledged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsAcknowledged(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAcknowledged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn IssuerDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IssuerDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIssuerDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIssuerDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn LastUpdated(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastUpdated)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetLastUpdated<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLastUpdated)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Kind(&self) -> windows_core::Result<WalletItemKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Barcode(&self) -> windows_core::Result<WalletBarcode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Barcode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetBarcode<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WalletBarcode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBarcode)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExpirationDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetExpirationDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn Logo159x159(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Logo159x159)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetLogo159x159<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLogo159x159)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn Logo336x336(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Logo336x336)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetLogo336x336<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLogo336x336)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn Logo99x99(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Logo99x99)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetLogo99x99<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLogo99x99)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DisplayMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsDisplayMessageLaunchable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisplayMessageLaunchable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsDisplayMessageLaunchable(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsDisplayMessageLaunchable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn LogoText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogoText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetLogoText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLogoText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn HeaderColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn SetHeaderColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHeaderColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn BodyColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BodyColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn SetBodyColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBodyColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn HeaderFontColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderFontColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn SetHeaderFontColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHeaderFontColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn BodyFontColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BodyFontColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "UI", feature = "deprecated"))]
    pub fn SetBodyFontColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBodyFontColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn HeaderBackgroundImage(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderBackgroundImage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetHeaderBackgroundImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHeaderBackgroundImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn BodyBackgroundImage(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BodyBackgroundImage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetBodyBackgroundImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBodyBackgroundImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn LogoImage(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogoImage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetLogoImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLogoImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn PromotionalImage(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PromotionalImage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetPromotionalImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPromotionalImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn RelevantDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelevantDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetRelevantDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelevantDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn RelevantDateDisplayMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelevantDateDisplayMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetRelevantDateDisplayMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelevantDateDisplayMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn TransactionHistory(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, WalletTransaction>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransactionHistory)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RelevantLocations(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, WalletRelevantLocation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelevantLocations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsMoreTransactionHistoryLaunchable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMoreTransactionHistoryLaunchable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsMoreTransactionHistoryLaunchable(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsMoreTransactionHistoryLaunchable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn DisplayProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, WalletItemCustomProperty>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Verbs(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, WalletVerb>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Verbs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateWalletItem(kind: WalletItemKind, displayname: &windows_core::HSTRING) -> windows_core::Result<WalletItem> {
        Self::IWalletItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWalletItem)(windows_core::Interface::as_raw(this), kind, core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IWalletItemFactory<R, F: FnOnce(&IWalletItemFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletItem, IWalletItemFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletItem>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletItem {
    type Vtable = IWalletItem_Vtbl;
    const IID: windows_core::GUID = <IWalletItem as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletItem {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletItem";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletItem {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletItem {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletItemCustomProperty(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletItemCustomProperty, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletItemCustomProperty {
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn AutoDetectLinks(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoDetectLinks)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAutoDetectLinks(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoDetectLinks)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DetailViewPosition(&self) -> windows_core::Result<WalletDetailViewPosition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetailViewPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDetailViewPosition(&self, value: WalletDetailViewPosition) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDetailViewPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn SummaryViewPosition(&self) -> windows_core::Result<WalletSummaryViewPosition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SummaryViewPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetSummaryViewPosition(&self, value: WalletSummaryViewPosition) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSummaryViewPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateWalletItemCustomProperty(name: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<WalletItemCustomProperty> {
        Self::IWalletItemCustomPropertyFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWalletItemCustomProperty)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IWalletItemCustomPropertyFactory<R, F: FnOnce(&IWalletItemCustomPropertyFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletItemCustomProperty, IWalletItemCustomPropertyFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletItemCustomProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletItemCustomProperty>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletItemCustomProperty {
    type Vtable = IWalletItemCustomProperty_Vtbl;
    const IID: windows_core::GUID = <IWalletItemCustomProperty as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletItemCustomProperty {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletItemCustomProperty";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletItemCustomProperty {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletItemCustomProperty {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletItemStore(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletItemStore, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletItemStore {
    #[cfg(feature = "deprecated")]
    pub fn AddAsync<P0>(&self, id: &windows_core::HSTRING, item: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ClearAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetWalletItemAsync(&self, id: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<WalletItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetWalletItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetItemsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetItemsWithKindAsync(&self, kind: WalletItemKind) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WalletItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsWithKindAsync)(windows_core::Interface::as_raw(this), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn ImportItemAsync<P0>(&self, stream: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<WalletItem>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportItemAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn DeleteAsync(&self, id: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowItemAsync(&self, id: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn UpdateAsync<P0>(&self, item: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<WalletItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateAsync)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletItemStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletItemStore>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletItemStore {
    type Vtable = IWalletItemStore_Vtbl;
    const IID: windows_core::GUID = <IWalletItemStore as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletItemStore {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletItemStore";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletItemStore {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletItemStore {}
#[cfg(feature = "deprecated")]
pub struct WalletManager;
#[cfg(feature = "deprecated")]
impl WalletManager {
    #[cfg(feature = "deprecated")]
    pub fn RequestStoreAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<WalletItemStore>> {
        Self::IWalletManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IWalletManagerStatics<R, F: FnOnce(&IWalletManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletManager, IWalletManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletManager {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletManager";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletRelevantLocation(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletRelevantLocation, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletRelevantLocation {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletRelevantLocation, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "deprecated"))]
    pub fn Position(&self) -> windows_core::Result<super::super::Devices::Geolocation::BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "deprecated"))]
    pub fn SetPosition(&self, value: super::super::Devices::Geolocation::BasicGeoposition) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DisplayMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletRelevantLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletRelevantLocation>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletRelevantLocation {
    type Vtable = IWalletRelevantLocation_Vtbl;
    const IID: windows_core::GUID = <IWalletRelevantLocation as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletRelevantLocation {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletRelevantLocation";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletRelevantLocation {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletRelevantLocation {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletTransaction(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletTransaction, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletTransaction {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletTransaction, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "deprecated")]
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DisplayAmount(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayAmount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayAmount(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayAmount)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn IgnoreTimeOfDay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnoreTimeOfDay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIgnoreTimeOfDay(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIgnoreTimeOfDay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn DisplayLocation(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayLocation(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayLocation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn TransactionDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransactionDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetTransactionDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransactionDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsLaunchable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLaunchable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetIsLaunchable(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsLaunchable)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletTransaction>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletTransaction {
    type Vtable = IWalletTransaction_Vtbl;
    const IID: windows_core::GUID = <IWalletTransaction as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletTransaction";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletTransaction {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletTransaction {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WalletVerb(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(WalletVerb, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl WalletVerb {
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateWalletVerb(name: &windows_core::HSTRING) -> windows_core::Result<WalletVerb> {
        Self::IWalletVerbFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWalletVerb)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IWalletVerbFactory<R, F: FnOnce(&IWalletVerbFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WalletVerb, IWalletVerbFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletVerb {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWalletVerb>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for WalletVerb {
    type Vtable = IWalletVerb_Vtbl;
    const IID: windows_core::GUID = <IWalletVerb as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for WalletVerb {
    const NAME: &'static str = "Windows.ApplicationModel.Wallet.WalletVerb";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for WalletVerb {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for WalletVerb {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WalletActionKind(pub i32);
#[cfg(feature = "deprecated")]
impl WalletActionKind {
    pub const OpenItem: Self = Self(0i32);
    pub const Transaction: Self = Self(1i32);
    pub const MoreTransactions: Self = Self(2i32);
    pub const Message: Self = Self(3i32);
    pub const Verb: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for WalletActionKind {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for WalletActionKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WalletActionKind").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletActionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletActionKind;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WalletBarcodeSymbology(pub i32);
#[cfg(feature = "deprecated")]
impl WalletBarcodeSymbology {
    pub const Invalid: Self = Self(0i32);
    pub const Upca: Self = Self(1i32);
    pub const Upce: Self = Self(2i32);
    pub const Ean13: Self = Self(3i32);
    pub const Ean8: Self = Self(4i32);
    pub const Itf: Self = Self(5i32);
    pub const Code39: Self = Self(6i32);
    pub const Code128: Self = Self(7i32);
    pub const Qr: Self = Self(8i32);
    pub const Pdf417: Self = Self(9i32);
    pub const Aztec: Self = Self(10i32);
    pub const Custom: Self = Self(100000i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for WalletBarcodeSymbology {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for WalletBarcodeSymbology {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WalletBarcodeSymbology").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletBarcodeSymbology {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletBarcodeSymbology;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WalletDetailViewPosition(pub i32);
#[cfg(feature = "deprecated")]
impl WalletDetailViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const HeaderField1: Self = Self(1i32);
    pub const HeaderField2: Self = Self(2i32);
    pub const PrimaryField1: Self = Self(3i32);
    pub const PrimaryField2: Self = Self(4i32);
    pub const SecondaryField1: Self = Self(5i32);
    pub const SecondaryField2: Self = Self(6i32);
    pub const SecondaryField3: Self = Self(7i32);
    pub const SecondaryField4: Self = Self(8i32);
    pub const SecondaryField5: Self = Self(9i32);
    pub const CenterField1: Self = Self(10i32);
    pub const FooterField1: Self = Self(11i32);
    pub const FooterField2: Self = Self(12i32);
    pub const FooterField3: Self = Self(13i32);
    pub const FooterField4: Self = Self(14i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for WalletDetailViewPosition {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for WalletDetailViewPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WalletDetailViewPosition").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletDetailViewPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletDetailViewPosition;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WalletItemKind(pub i32);
#[cfg(feature = "deprecated")]
impl WalletItemKind {
    pub const Invalid: Self = Self(0i32);
    pub const Deal: Self = Self(1i32);
    pub const General: Self = Self(2i32);
    pub const PaymentInstrument: Self = Self(3i32);
    pub const Ticket: Self = Self(4i32);
    pub const BoardingPass: Self = Self(5i32);
    pub const MembershipCard: Self = Self(6i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for WalletItemKind {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for WalletItemKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WalletItemKind").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletItemKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletItemKind;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WalletSummaryViewPosition(pub i32);
#[cfg(feature = "deprecated")]
impl WalletSummaryViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const Field1: Self = Self(1i32);
    pub const Field2: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for WalletSummaryViewPosition {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for WalletSummaryViewPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WalletSummaryViewPosition").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for WalletSummaryViewPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Wallet.WalletSummaryViewPosition;i4)");
}
