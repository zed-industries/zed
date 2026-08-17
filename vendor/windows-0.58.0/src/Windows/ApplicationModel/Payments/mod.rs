#[cfg(feature = "ApplicationModel_Payments_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(IPaymentAddress, IPaymentAddress_Vtbl, 0x5f2264e9_6f3a_4166_a018_0a0b06bb32b5);
impl windows_core::RuntimeType for IPaymentAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentAddress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Country: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddressLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddressLines: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAddressLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAddressLines: usize,
    pub Region: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub City: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DependentLocality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDependentLocality: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SortingCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSortingCode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LanguageCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLanguageCode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Organization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOrganization: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Recipient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRecipient: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IPaymentCanMakePaymentResult, IPaymentCanMakePaymentResult_Vtbl, 0x7696fe55_d5d3_4d3d_b345_45591759c510);
impl windows_core::RuntimeType for IPaymentCanMakePaymentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentCanMakePaymentResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PaymentCanMakePaymentResultStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentCanMakePaymentResultFactory, IPaymentCanMakePaymentResultFactory_Vtbl, 0xbbdcaa3e_7d49_4f69_aa53_2a0f8164b7c9);
impl windows_core::RuntimeType for IPaymentCanMakePaymentResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentCanMakePaymentResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, PaymentCanMakePaymentResultStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentCurrencyAmount, IPaymentCurrencyAmount_Vtbl, 0xe3a3e9e0_b41f_4987_bdcb_071331f2daa4);
impl windows_core::RuntimeType for IPaymentCurrencyAmount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentCurrencyAmount_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Currency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCurrency: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CurrencySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCurrencySystem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentCurrencyAmountFactory, IPaymentCurrencyAmountFactory_Vtbl, 0x3257d338_140c_4575_8535_f773178c09a7);
impl windows_core::RuntimeType for IPaymentCurrencyAmountFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentCurrencyAmountFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithCurrencySystem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentDetails, IPaymentDetails_Vtbl, 0x53bb2d7d_e0eb_4053_8eae_ce7c48e02945);
impl windows_core::RuntimeType for IPaymentDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Total: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTotal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DisplayItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDisplayItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShippingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShippingOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShippingOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Modifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Modifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetModifiers: usize,
}
windows_core::imp::define_interface!(IPaymentDetailsFactory, IPaymentDetailsFactory_Vtbl, 0xcfe8afee_c0ea_4ca1_8bc7_6de67b1f3763);
impl windows_core::RuntimeType for IPaymentDetailsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentDetailsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithDisplayItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithDisplayItems: usize,
}
windows_core::imp::define_interface!(IPaymentDetailsModifier, IPaymentDetailsModifier_Vtbl, 0xbe1c7d65_4323_41d7_b305_dfcb765f69de);
impl windows_core::RuntimeType for IPaymentDetailsModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentDetailsModifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub JsonData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub Total: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AdditionalDisplayItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AdditionalDisplayItems: usize,
}
windows_core::imp::define_interface!(IPaymentDetailsModifierFactory, IPaymentDetailsModifierFactory_Vtbl, 0x79005286_54de_429c_9e4f_5dce6e10ebce);
impl windows_core::RuntimeType for IPaymentDetailsModifierFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentDetailsModifierFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAdditionalDisplayItemsAndJsonData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAdditionalDisplayItemsAndJsonData: usize,
}
windows_core::imp::define_interface!(IPaymentItem, IPaymentItem_Vtbl, 0x685ac88b_79b2_4b76_9e03_a876223dfe72);
impl windows_core::RuntimeType for IPaymentItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pending: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPending: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentItemFactory, IPaymentItemFactory_Vtbl, 0xc6ab7ad8_2503_4d1d_a778_02b2e5927b2c);
impl windows_core::RuntimeType for IPaymentItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentMediator, IPaymentMediator_Vtbl, 0xfb0ee829_ec0c_449a_83da_7ae3073365a2);
impl windows_core::RuntimeType for IPaymentMediator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentMediator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedMethodIdsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedMethodIdsAsync: usize,
    pub SubmitPaymentRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubmitPaymentRequestWithChangeHandlerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentMediator2, IPaymentMediator2_Vtbl, 0xceef98f1_e407_4128_8e73_d93d5f822786);
impl windows_core::RuntimeType for IPaymentMediator2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentMediator2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanMakePaymentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentMerchantInfo, IPaymentMerchantInfo_Vtbl, 0x63445050_0e94_4ed6_aacb_e6012bd327a7);
impl windows_core::RuntimeType for IPaymentMerchantInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentMerchantInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentMerchantInfoFactory, IPaymentMerchantInfoFactory_Vtbl, 0x9e89ced3_ccb7_4167_a8ec_e10ae96dbcd1);
impl windows_core::RuntimeType for IPaymentMerchantInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentMerchantInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentMethodData, IPaymentMethodData_Vtbl, 0xd1d3caf4_de98_4129_b1b7_c3ad86237bf4);
impl windows_core::RuntimeType for IPaymentMethodData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentMethodData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedMethodIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedMethodIds: usize,
    pub JsonData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentMethodDataFactory, IPaymentMethodDataFactory_Vtbl, 0x8addd27f_9baa_4a82_8342_a8210992a36b);
impl windows_core::RuntimeType for IPaymentMethodDataFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentMethodDataFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithJsonData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithJsonData: usize,
}
windows_core::imp::define_interface!(IPaymentOptions, IPaymentOptions_Vtbl, 0xaaa30854_1f2b_4365_8251_01b58915a5bc);
impl windows_core::RuntimeType for IPaymentOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestPayerEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PaymentOptionPresence) -> windows_core::HRESULT,
    pub SetRequestPayerEmail: unsafe extern "system" fn(*mut core::ffi::c_void, PaymentOptionPresence) -> windows_core::HRESULT,
    pub RequestPayerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PaymentOptionPresence) -> windows_core::HRESULT,
    pub SetRequestPayerName: unsafe extern "system" fn(*mut core::ffi::c_void, PaymentOptionPresence) -> windows_core::HRESULT,
    pub RequestPayerPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PaymentOptionPresence) -> windows_core::HRESULT,
    pub SetRequestPayerPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, PaymentOptionPresence) -> windows_core::HRESULT,
    pub RequestShipping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRequestShipping: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShippingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PaymentShippingType) -> windows_core::HRESULT,
    pub SetShippingType: unsafe extern "system" fn(*mut core::ffi::c_void, PaymentShippingType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentRequest, IPaymentRequest_Vtbl, 0xb74942e1_ed7b_47eb_bc08_78cc5d6896b6);
impl windows_core::RuntimeType for IPaymentRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MerchantInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MethodData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MethodData: usize,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentRequest2, IPaymentRequest2_Vtbl, 0xb63ccfb5_5998_493e_a04c_67048a50f141);
impl windows_core::RuntimeType for IPaymentRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentRequestChangedArgs, IPaymentRequestChangedArgs_Vtbl, 0xc6145e44_cd8b_4be4_b555_27c99194c0c5);
impl windows_core::RuntimeType for IPaymentRequestChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequestChangedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PaymentRequestChangeKind) -> windows_core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectedShippingOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Acknowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentRequestChangedResult, IPaymentRequestChangedResult_Vtbl, 0xdf699e5c_16c4_47ad_9401_8440ec0757db);
impl windows_core::RuntimeType for IPaymentRequestChangedResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequestChangedResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeAcceptedByMerchant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetChangeAcceptedByMerchant: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UpdatedPaymentDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUpdatedPaymentDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentRequestChangedResultFactory, IPaymentRequestChangedResultFactory_Vtbl, 0x08740f56_1d33_4431_814b_67ea24bf21db);
impl windows_core::RuntimeType for IPaymentRequestChangedResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequestChangedResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithPaymentDetails: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentRequestFactory, IPaymentRequestFactory_Vtbl, 0x3e8a79dc_6b74_42d3_b103_f0de35fb1848);
impl windows_core::RuntimeType for IPaymentRequestFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequestFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoAndOptions: usize,
}
windows_core::imp::define_interface!(IPaymentRequestFactory2, IPaymentRequestFactory2_Vtbl, 0xe6ce1325_a506_4372_b7ef_1a031d5662d1);
impl windows_core::RuntimeType for IPaymentRequestFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequestFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithMerchantInfoOptionsAndId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithMerchantInfoOptionsAndId: usize,
}
windows_core::imp::define_interface!(IPaymentRequestSubmitResult, IPaymentRequestSubmitResult_Vtbl, 0x7b9c3912_30f2_4e90_b249_8ce7d78ffe56);
impl windows_core::RuntimeType for IPaymentRequestSubmitResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentRequestSubmitResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PaymentRequestStatus) -> windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentResponse, IPaymentResponse_Vtbl, 0xe1389457_8bd2_4888_9fa8_97985545108e);
impl windows_core::RuntimeType for IPaymentResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PaymentToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShippingOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShippingAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PayerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CompleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, PaymentRequestCompletionStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentShippingOption, IPaymentShippingOption_Vtbl, 0x13372ada_9753_4574_8966_93145a76c7f9);
impl windows_core::RuntimeType for IPaymentShippingOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentShippingOption_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentShippingOptionFactory, IPaymentShippingOptionFactory_Vtbl, 0x5de5f917_b2d7_446b_9d73_6123fbca3bc6);
impl windows_core::RuntimeType for IPaymentShippingOptionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentShippingOptionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithSelected: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithSelectedAndTag: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, bool, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentToken, IPaymentToken_Vtbl, 0xbbcac013_ccd0_41f2_b2a1_0a2e4b5dce25);
impl windows_core::RuntimeType for IPaymentToken {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentToken_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PaymentMethodId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub JsonDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPaymentTokenFactory, IPaymentTokenFactory_Vtbl, 0x988cd7aa_4753_4904_8373_dd7b08b995c1);
impl windows_core::RuntimeType for IPaymentTokenFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPaymentTokenFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithJsonDetails: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentAddress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentAddress, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentAddress {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentAddress, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Country(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Country)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCountry(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCountry)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddressLines(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddressLines)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAddressLines<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAddressLines)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Region(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Region)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRegion(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRegion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn City(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).City)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCity(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DependentLocality(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DependentLocality)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDependentLocality(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDependentLocality)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PostalCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PostalCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPostalCode(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPostalCode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SortingCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SortingCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSortingCode(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSortingCode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn LanguageCode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LanguageCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguageCode(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguageCode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Organization(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Organization)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOrganization(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrganization)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Recipient(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recipient)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRecipient(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRecipient)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPhoneNumber(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPhoneNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PaymentAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentAddress>();
}
unsafe impl windows_core::Interface for PaymentAddress {
    type Vtable = IPaymentAddress_Vtbl;
    const IID: windows_core::GUID = <IPaymentAddress as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentAddress";
}
unsafe impl Send for PaymentAddress {}
unsafe impl Sync for PaymentAddress {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentCanMakePaymentResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentCanMakePaymentResult, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentCanMakePaymentResult {
    pub fn Status(&self) -> windows_core::Result<PaymentCanMakePaymentResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(value: PaymentCanMakePaymentResultStatus) -> windows_core::Result<PaymentCanMakePaymentResult> {
        Self::IPaymentCanMakePaymentResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentCanMakePaymentResultFactory<R, F: FnOnce(&IPaymentCanMakePaymentResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentCanMakePaymentResult, IPaymentCanMakePaymentResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentCanMakePaymentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentCanMakePaymentResult>();
}
unsafe impl windows_core::Interface for PaymentCanMakePaymentResult {
    type Vtable = IPaymentCanMakePaymentResult_Vtbl;
    const IID: windows_core::GUID = <IPaymentCanMakePaymentResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentCanMakePaymentResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult";
}
unsafe impl Send for PaymentCanMakePaymentResult {}
unsafe impl Sync for PaymentCanMakePaymentResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentCurrencyAmount(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentCurrencyAmount, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentCurrencyAmount {
    pub fn Currency(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Currency)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCurrency(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCurrency)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CurrencySystem(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrencySystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCurrencySystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCurrencySystem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(value: &windows_core::HSTRING, currency: &windows_core::HSTRING) -> windows_core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), core::mem::transmute_copy(currency), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithCurrencySystem(value: &windows_core::HSTRING, currency: &windows_core::HSTRING, currencysystem: &windows_core::HSTRING) -> windows_core::Result<PaymentCurrencyAmount> {
        Self::IPaymentCurrencyAmountFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithCurrencySystem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), core::mem::transmute_copy(currency), core::mem::transmute_copy(currencysystem), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentCurrencyAmountFactory<R, F: FnOnce(&IPaymentCurrencyAmountFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentCurrencyAmount, IPaymentCurrencyAmountFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentCurrencyAmount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentCurrencyAmount>();
}
unsafe impl windows_core::Interface for PaymentCurrencyAmount {
    type Vtable = IPaymentCurrencyAmount_Vtbl;
    const IID: windows_core::GUID = <IPaymentCurrencyAmount as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentCurrencyAmount {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentCurrencyAmount";
}
unsafe impl Send for PaymentCurrencyAmount {}
unsafe impl Sync for PaymentCurrencyAmount {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentDetails {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentDetails, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Total(&self) -> windows_core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Total)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTotal<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PaymentItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTotal)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayItems(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayItems)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDisplayItems<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<PaymentItem>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayItems)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShippingOptions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShippingOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetShippingOptions<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<PaymentShippingOption>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShippingOptions)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Modifiers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Modifiers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetModifiers<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<PaymentDetailsModifier>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetModifiers)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(total: P0) -> windows_core::Result<PaymentDetails>
    where
        P0: windows_core::Param<PaymentItem>,
    {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), total.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithDisplayItems<P0, P1>(total: P0, displayitems: P1) -> windows_core::Result<PaymentDetails>
    where
        P0: windows_core::Param<PaymentItem>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<PaymentItem>>,
    {
        Self::IPaymentDetailsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithDisplayItems)(windows_core::Interface::as_raw(this), total.param().abi(), displayitems.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentDetailsFactory<R, F: FnOnce(&IPaymentDetailsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentDetails, IPaymentDetailsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentDetails>();
}
unsafe impl windows_core::Interface for PaymentDetails {
    type Vtable = IPaymentDetails_Vtbl;
    const IID: windows_core::GUID = <IPaymentDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetails";
}
unsafe impl Send for PaymentDetails {}
unsafe impl Sync for PaymentDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentDetailsModifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentDetailsModifier, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentDetailsModifier {
    pub fn JsonData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JsonData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedMethodIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Total(&self) -> windows_core::Result<PaymentItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Total)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AdditionalDisplayItems(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdditionalDisplayItems)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0, P1>(supportedmethodids: P0, total: P1) -> windows_core::Result<PaymentDetailsModifier>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<PaymentItem>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), supportedmethodids.param().abi(), total.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItems<P0, P1, P2>(supportedmethodids: P0, total: P1, additionaldisplayitems: P2) -> windows_core::Result<PaymentDetailsModifier>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<PaymentItem>,
        P2: windows_core::Param<super::super::Foundation::Collections::IIterable<PaymentItem>>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAdditionalDisplayItems)(windows_core::Interface::as_raw(this), supportedmethodids.param().abi(), total.param().abi(), additionaldisplayitems.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAdditionalDisplayItemsAndJsonData<P0, P1, P2>(supportedmethodids: P0, total: P1, additionaldisplayitems: P2, jsondata: &windows_core::HSTRING) -> windows_core::Result<PaymentDetailsModifier>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<PaymentItem>,
        P2: windows_core::Param<super::super::Foundation::Collections::IIterable<PaymentItem>>,
    {
        Self::IPaymentDetailsModifierFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithAdditionalDisplayItemsAndJsonData)(windows_core::Interface::as_raw(this), supportedmethodids.param().abi(), total.param().abi(), additionaldisplayitems.param().abi(), core::mem::transmute_copy(jsondata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentDetailsModifierFactory<R, F: FnOnce(&IPaymentDetailsModifierFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentDetailsModifier, IPaymentDetailsModifierFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentDetailsModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentDetailsModifier>();
}
unsafe impl windows_core::Interface for PaymentDetailsModifier {
    type Vtable = IPaymentDetailsModifier_Vtbl;
    const IID: windows_core::GUID = <IPaymentDetailsModifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentDetailsModifier {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentDetailsModifier";
}
unsafe impl Send for PaymentDetailsModifier {}
unsafe impl Sync for PaymentDetailsModifier {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentItem, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentItem {
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Amount(&self) -> windows_core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Amount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAmount<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PaymentCurrencyAmount>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAmount)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Pending(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pending)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPending(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPending)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(label: &windows_core::HSTRING, amount: P0) -> windows_core::Result<PaymentItem>
    where
        P0: windows_core::Param<PaymentCurrencyAmount>,
    {
        Self::IPaymentItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(label), amount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentItemFactory<R, F: FnOnce(&IPaymentItemFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentItem, IPaymentItemFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentItem>();
}
unsafe impl windows_core::Interface for PaymentItem {
    type Vtable = IPaymentItem_Vtbl;
    const IID: windows_core::GUID = <IPaymentItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentItem {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentItem";
}
unsafe impl Send for PaymentItem {}
unsafe impl Sync for PaymentItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentMediator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentMediator, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentMediator {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentMediator, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedMethodIdsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedMethodIdsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubmitPaymentRequestAsync<P0>(&self, paymentrequest: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>
    where
        P0: windows_core::Param<PaymentRequest>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubmitPaymentRequestAsync)(windows_core::Interface::as_raw(this), paymentrequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubmitPaymentRequestWithChangeHandlerAsync<P0, P1>(&self, paymentrequest: P0, changehandler: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentRequestSubmitResult>>
    where
        P0: windows_core::Param<PaymentRequest>,
        P1: windows_core::Param<PaymentRequestChangedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubmitPaymentRequestWithChangeHandlerAsync)(windows_core::Interface::as_raw(this), paymentrequest.param().abi(), changehandler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanMakePaymentAsync<P0>(&self, paymentrequest: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PaymentCanMakePaymentResult>>
    where
        P0: windows_core::Param<PaymentRequest>,
    {
        let this = &windows_core::Interface::cast::<IPaymentMediator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanMakePaymentAsync)(windows_core::Interface::as_raw(this), paymentrequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PaymentMediator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentMediator>();
}
unsafe impl windows_core::Interface for PaymentMediator {
    type Vtable = IPaymentMediator_Vtbl;
    const IID: windows_core::GUID = <IPaymentMediator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentMediator {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMediator";
}
unsafe impl Send for PaymentMediator {}
unsafe impl Sync for PaymentMediator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentMerchantInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentMerchantInfo, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentMerchantInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentMerchantInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PackageFullName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFullName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(uri: P0) -> windows_core::Result<PaymentMerchantInfo>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IPaymentMerchantInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentMerchantInfoFactory<R, F: FnOnce(&IPaymentMerchantInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentMerchantInfo, IPaymentMerchantInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentMerchantInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentMerchantInfo>();
}
unsafe impl windows_core::Interface for PaymentMerchantInfo {
    type Vtable = IPaymentMerchantInfo_Vtbl;
    const IID: windows_core::GUID = <IPaymentMerchantInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentMerchantInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMerchantInfo";
}
unsafe impl Send for PaymentMerchantInfo {}
unsafe impl Sync for PaymentMerchantInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentMethodData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentMethodData, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentMethodData {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedMethodIds(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedMethodIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn JsonData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JsonData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(supportedmethodids: P0) -> windows_core::Result<PaymentMethodData>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), supportedmethodids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithJsonData<P0>(supportedmethodids: P0, jsondata: &windows_core::HSTRING) -> windows_core::Result<PaymentMethodData>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPaymentMethodDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithJsonData)(windows_core::Interface::as_raw(this), supportedmethodids.param().abi(), core::mem::transmute_copy(jsondata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentMethodDataFactory<R, F: FnOnce(&IPaymentMethodDataFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentMethodData, IPaymentMethodDataFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentMethodData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentMethodData>();
}
unsafe impl windows_core::Interface for PaymentMethodData {
    type Vtable = IPaymentMethodData_Vtbl;
    const IID: windows_core::GUID = <IPaymentMethodData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentMethodData {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentMethodData";
}
unsafe impl Send for PaymentMethodData {}
unsafe impl Sync for PaymentMethodData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentOptions, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestPayerEmail(&self) -> windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPayerEmail)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequestPayerEmail(&self, value: PaymentOptionPresence) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequestPayerEmail)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerName(&self) -> windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPayerName)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequestPayerName(&self, value: PaymentOptionPresence) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequestPayerName)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestPayerPhoneNumber(&self) -> windows_core::Result<PaymentOptionPresence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPayerPhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequestPayerPhoneNumber(&self, value: PaymentOptionPresence) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequestPayerPhoneNumber)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestShipping(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestShipping)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequestShipping(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequestShipping)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShippingType(&self) -> windows_core::Result<PaymentShippingType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShippingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShippingType(&self, value: PaymentShippingType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShippingType)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PaymentOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentOptions>();
}
unsafe impl windows_core::Interface for PaymentOptions {
    type Vtable = IPaymentOptions_Vtbl;
    const IID: windows_core::GUID = <IPaymentOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentOptions";
}
unsafe impl Send for PaymentOptions {}
unsafe impl Sync for PaymentOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentRequest, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentRequest {
    pub fn MerchantInfo(&self) -> windows_core::Result<PaymentMerchantInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MerchantInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Details(&self) -> windows_core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Details)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MethodData(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PaymentMethodData>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MethodData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Options(&self) -> windows_core::Result<PaymentOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPaymentRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0, P1>(details: P0, methoddata: P1) -> windows_core::Result<PaymentRequest>
    where
        P0: windows_core::Param<PaymentDetails>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), details.param().abi(), methoddata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfo<P0, P1, P2>(details: P0, methoddata: P1, merchantinfo: P2) -> windows_core::Result<PaymentRequest>
    where
        P0: windows_core::Param<PaymentDetails>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
        P2: windows_core::Param<PaymentMerchantInfo>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMerchantInfo)(windows_core::Interface::as_raw(this), details.param().abi(), methoddata.param().abi(), merchantinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoAndOptions<P0, P1, P2, P3>(details: P0, methoddata: P1, merchantinfo: P2, options: P3) -> windows_core::Result<PaymentRequest>
    where
        P0: windows_core::Param<PaymentDetails>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
        P2: windows_core::Param<PaymentMerchantInfo>,
        P3: windows_core::Param<PaymentOptions>,
    {
        Self::IPaymentRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMerchantInfoAndOptions)(windows_core::Interface::as_raw(this), details.param().abi(), methoddata.param().abi(), merchantinfo.param().abi(), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithMerchantInfoOptionsAndId<P0, P1, P2, P3>(details: P0, methoddata: P1, merchantinfo: P2, options: P3, id: &windows_core::HSTRING) -> windows_core::Result<PaymentRequest>
    where
        P0: windows_core::Param<PaymentDetails>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<PaymentMethodData>>,
        P2: windows_core::Param<PaymentMerchantInfo>,
        P3: windows_core::Param<PaymentOptions>,
    {
        Self::IPaymentRequestFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithMerchantInfoOptionsAndId)(windows_core::Interface::as_raw(this), details.param().abi(), methoddata.param().abi(), merchantinfo.param().abi(), options.param().abi(), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentRequestFactory<R, F: FnOnce(&IPaymentRequestFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentRequest, IPaymentRequestFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPaymentRequestFactory2<R, F: FnOnce(&IPaymentRequestFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentRequest, IPaymentRequestFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentRequest>();
}
unsafe impl windows_core::Interface for PaymentRequest {
    type Vtable = IPaymentRequest_Vtbl;
    const IID: windows_core::GUID = <IPaymentRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequest";
}
unsafe impl Send for PaymentRequest {}
unsafe impl Sync for PaymentRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentRequestChangedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentRequestChangedArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentRequestChangedArgs {
    pub fn ChangeKind(&self) -> windows_core::Result<PaymentRequestChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShippingAddress(&self) -> windows_core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShippingAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectedShippingOption(&self) -> windows_core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedShippingOption)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Acknowledge<P0>(&self, changeresult: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PaymentRequestChangedResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Acknowledge)(windows_core::Interface::as_raw(this), changeresult.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for PaymentRequestChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentRequestChangedArgs>();
}
unsafe impl windows_core::Interface for PaymentRequestChangedArgs {
    type Vtable = IPaymentRequestChangedArgs_Vtbl;
    const IID: windows_core::GUID = <IPaymentRequestChangedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentRequestChangedArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedArgs";
}
unsafe impl Send for PaymentRequestChangedArgs {}
unsafe impl Sync for PaymentRequestChangedArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentRequestChangedResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentRequestChangedResult, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentRequestChangedResult {
    pub fn ChangeAcceptedByMerchant(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeAcceptedByMerchant)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetChangeAcceptedByMerchant(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetChangeAcceptedByMerchant)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdatedPaymentDetails(&self) -> windows_core::Result<PaymentDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdatedPaymentDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUpdatedPaymentDetails<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PaymentDetails>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUpdatedPaymentDetails)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create(changeacceptedbymerchant: bool) -> windows_core::Result<PaymentRequestChangedResult> {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), changeacceptedbymerchant, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithPaymentDetails<P0>(changeacceptedbymerchant: bool, updatedpaymentdetails: P0) -> windows_core::Result<PaymentRequestChangedResult>
    where
        P0: windows_core::Param<PaymentDetails>,
    {
        Self::IPaymentRequestChangedResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithPaymentDetails)(windows_core::Interface::as_raw(this), changeacceptedbymerchant, updatedpaymentdetails.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentRequestChangedResultFactory<R, F: FnOnce(&IPaymentRequestChangedResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentRequestChangedResult, IPaymentRequestChangedResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentRequestChangedResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentRequestChangedResult>();
}
unsafe impl windows_core::Interface for PaymentRequestChangedResult {
    type Vtable = IPaymentRequestChangedResult_Vtbl;
    const IID: windows_core::GUID = <IPaymentRequestChangedResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentRequestChangedResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestChangedResult";
}
unsafe impl Send for PaymentRequestChangedResult {}
unsafe impl Sync for PaymentRequestChangedResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentRequestSubmitResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentRequestSubmitResult, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentRequestSubmitResult {
    pub fn Status(&self) -> windows_core::Result<PaymentRequestStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Response(&self) -> windows_core::Result<PaymentResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Response)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PaymentRequestSubmitResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentRequestSubmitResult>();
}
unsafe impl windows_core::Interface for PaymentRequestSubmitResult {
    type Vtable = IPaymentRequestSubmitResult_Vtbl;
    const IID: windows_core::GUID = <IPaymentRequestSubmitResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentRequestSubmitResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentRequestSubmitResult";
}
unsafe impl Send for PaymentRequestSubmitResult {}
unsafe impl Sync for PaymentRequestSubmitResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentResponse, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentResponse {
    pub fn PaymentToken(&self) -> windows_core::Result<PaymentToken> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaymentToken)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShippingOption(&self) -> windows_core::Result<PaymentShippingOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShippingOption)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShippingAddress(&self) -> windows_core::Result<PaymentAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShippingAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PayerEmail(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayerEmail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PayerName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayerName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PayerPhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayerPhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CompleteAsync(&self, status: PaymentRequestCompletionStatus) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompleteAsync)(windows_core::Interface::as_raw(this), status, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PaymentResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentResponse>();
}
unsafe impl windows_core::Interface for PaymentResponse {
    type Vtable = IPaymentResponse_Vtbl;
    const IID: windows_core::GUID = <IPaymentResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentResponse {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentResponse";
}
unsafe impl Send for PaymentResponse {}
unsafe impl Sync for PaymentResponse {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentShippingOption(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentShippingOption, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentShippingOption {
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Amount(&self) -> windows_core::Result<PaymentCurrencyAmount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Amount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAmount<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PaymentCurrencyAmount>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAmount)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsSelected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSelected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsSelected)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(label: &windows_core::HSTRING, amount: P0) -> windows_core::Result<PaymentShippingOption>
    where
        P0: windows_core::Param<PaymentCurrencyAmount>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(label), amount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithSelected<P0>(label: &windows_core::HSTRING, amount: P0, selected: bool) -> windows_core::Result<PaymentShippingOption>
    where
        P0: windows_core::Param<PaymentCurrencyAmount>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithSelected)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(label), amount.param().abi(), selected, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithSelectedAndTag<P0>(label: &windows_core::HSTRING, amount: P0, selected: bool, tag: &windows_core::HSTRING) -> windows_core::Result<PaymentShippingOption>
    where
        P0: windows_core::Param<PaymentCurrencyAmount>,
    {
        Self::IPaymentShippingOptionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithSelectedAndTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(label), amount.param().abi(), selected, core::mem::transmute_copy(tag), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentShippingOptionFactory<R, F: FnOnce(&IPaymentShippingOptionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentShippingOption, IPaymentShippingOptionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentShippingOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentShippingOption>();
}
unsafe impl windows_core::Interface for PaymentShippingOption {
    type Vtable = IPaymentShippingOption_Vtbl;
    const IID: windows_core::GUID = <IPaymentShippingOption as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentShippingOption {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentShippingOption";
}
unsafe impl Send for PaymentShippingOption {}
unsafe impl Sync for PaymentShippingOption {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PaymentToken(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentToken, windows_core::IUnknown, windows_core::IInspectable);
impl PaymentToken {
    pub fn PaymentMethodId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaymentMethodId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn JsonDetails(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).JsonDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(paymentmethodid: &windows_core::HSTRING) -> windows_core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(paymentmethodid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithJsonDetails(paymentmethodid: &windows_core::HSTRING, jsondetails: &windows_core::HSTRING) -> windows_core::Result<PaymentToken> {
        Self::IPaymentTokenFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithJsonDetails)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(paymentmethodid), core::mem::transmute_copy(jsondetails), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPaymentTokenFactory<R, F: FnOnce(&IPaymentTokenFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentToken, IPaymentTokenFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentToken {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPaymentToken>();
}
unsafe impl windows_core::Interface for PaymentToken {
    type Vtable = IPaymentToken_Vtbl;
    const IID: windows_core::GUID = <IPaymentToken as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentToken {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.PaymentToken";
}
unsafe impl Send for PaymentToken {}
unsafe impl Sync for PaymentToken {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaymentCanMakePaymentResultStatus(pub i32);
impl PaymentCanMakePaymentResultStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Yes: Self = Self(1i32);
    pub const No: Self = Self(2i32);
    pub const NotAllowed: Self = Self(3i32);
    pub const UserNotSignedIn: Self = Self(4i32);
    pub const SpecifiedPaymentMethodIdsNotSupported: Self = Self(5i32);
    pub const NoQualifyingCardOnFile: Self = Self(6i32);
}
impl windows_core::TypeKind for PaymentCanMakePaymentResultStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaymentCanMakePaymentResultStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaymentCanMakePaymentResultStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PaymentCanMakePaymentResultStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentCanMakePaymentResultStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaymentOptionPresence(pub i32);
impl PaymentOptionPresence {
    pub const None: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl windows_core::TypeKind for PaymentOptionPresence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaymentOptionPresence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaymentOptionPresence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PaymentOptionPresence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentOptionPresence;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaymentRequestChangeKind(pub i32);
impl PaymentRequestChangeKind {
    pub const ShippingOption: Self = Self(0i32);
    pub const ShippingAddress: Self = Self(1i32);
}
impl windows_core::TypeKind for PaymentRequestChangeKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaymentRequestChangeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaymentRequestChangeKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PaymentRequestChangeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestChangeKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaymentRequestCompletionStatus(pub i32);
impl PaymentRequestCompletionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl windows_core::TypeKind for PaymentRequestCompletionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaymentRequestCompletionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaymentRequestCompletionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PaymentRequestCompletionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestCompletionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaymentRequestStatus(pub i32);
impl PaymentRequestStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl windows_core::TypeKind for PaymentRequestStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaymentRequestStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaymentRequestStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PaymentRequestStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentRequestStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaymentShippingType(pub i32);
impl PaymentShippingType {
    pub const Shipping: Self = Self(0i32);
    pub const Delivery: Self = Self(1i32);
    pub const Pickup: Self = Self(2i32);
}
impl windows_core::TypeKind for PaymentShippingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaymentShippingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaymentShippingType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PaymentShippingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Payments.PaymentShippingType;i4)");
}
windows_core::imp::define_interface!(PaymentRequestChangedHandler, PaymentRequestChangedHandler_Vtbl, 0x5078b9e1_f398_4f2c_a27e_94d371cf6c7d);
impl PaymentRequestChangedHandler {
    pub fn new<F: FnMut(Option<&PaymentRequest>, Option<&PaymentRequestChangedArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = PaymentRequestChangedHandlerBox::<F> { vtable: &PaymentRequestChangedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, paymentrequest: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PaymentRequest>,
        P1: windows_core::Param<PaymentRequestChangedArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), paymentrequest.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
struct PaymentRequestChangedHandlerBox<F: FnMut(Option<&PaymentRequest>, Option<&PaymentRequestChangedArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const PaymentRequestChangedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&PaymentRequest>, Option<&PaymentRequestChangedArgs>) -> windows_core::Result<()> + Send + 'static> PaymentRequestChangedHandlerBox<F> {
    const VTABLE: PaymentRequestChangedHandler_Vtbl = PaymentRequestChangedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <PaymentRequestChangedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, paymentrequest: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&paymentrequest), windows_core::from_raw_borrowed(&args)).into()
    }
}
impl windows_core::RuntimeType for PaymentRequestChangedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct PaymentRequestChangedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
