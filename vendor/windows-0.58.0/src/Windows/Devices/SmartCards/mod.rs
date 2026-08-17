windows_core::imp::define_interface!(ICardAddedEventArgs, ICardAddedEventArgs_Vtbl, 0x18bbef98_f18b_4dd3_b118_dfb2c8e23cc6);
impl windows_core::RuntimeType for ICardAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICardAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICardRemovedEventArgs, ICardRemovedEventArgs_Vtbl, 0x15331aaf_22d7_4945_afc9_03b46f42a6cd);
impl windows_core::RuntimeType for ICardRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICardRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKnownSmartCardAppletIds, IKnownSmartCardAppletIds_Vtbl, 0x7b04d8d8_95b4_4c88_8cea_411e55511efc);
impl windows_core::RuntimeType for IKnownSmartCardAppletIds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownSmartCardAppletIds_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub PaymentSystemEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PaymentSystemEnvironment: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProximityPaymentSystemEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProximityPaymentSystemEnvironment: usize,
}
windows_core::imp::define_interface!(ISmartCard, ISmartCard_Vtbl, 0x1b718871_6434_43f4_b55a_6a29623870aa);
impl windows_core::RuntimeType for ISmartCard {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCard_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAnswerToResetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAnswerToResetAsync: usize,
}
windows_core::imp::define_interface!(ISmartCardAppletIdGroup, ISmartCardAppletIdGroup_Vtbl, 0x7db165e6_6264_56f4_5e03_c86385395eb1);
impl windows_core::RuntimeType for ISmartCardAppletIdGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAppletIdGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppletIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppletIds: usize,
    pub SmartCardEmulationCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardEmulationCategory) -> windows_core::HRESULT,
    pub SetSmartCardEmulationCategory: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardEmulationCategory) -> windows_core::HRESULT,
    pub SmartCardEmulationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardEmulationType) -> windows_core::HRESULT,
    pub SetSmartCardEmulationType: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardEmulationType) -> windows_core::HRESULT,
    pub AutomaticEnablement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutomaticEnablement: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardAppletIdGroup2, ISmartCardAppletIdGroup2_Vtbl, 0x6b0ef9dc_9956_4a62_8d4e_d37a68ebc3a6);
impl windows_core::RuntimeType for ISmartCardAppletIdGroup2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAppletIdGroup2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLogo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLogo: usize,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub SecureUserAuthenticationRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSecureUserAuthenticationRequired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardAppletIdGroupFactory, ISmartCardAppletIdGroupFactory_Vtbl, 0x9105eb4d_4a65_4e41_8061_cbe83f3695e5);
impl windows_core::RuntimeType for ISmartCardAppletIdGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, SmartCardEmulationCategory, SmartCardEmulationType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Create: usize,
}
windows_core::imp::define_interface!(ISmartCardAppletIdGroupRegistration, ISmartCardAppletIdGroupRegistration_Vtbl, 0xdf1208d1_31bb_5596_43b1_6d69a0257b3a);
impl windows_core::RuntimeType for ISmartCardAppletIdGroupRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupRegistration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardAppletIdGroupActivationPolicy) -> windows_core::HRESULT,
    pub AppletIdGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestActivationPolicyChangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardAppletIdGroupActivationPolicy, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAutomaticResponseApdusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAutomaticResponseApdusAsync: usize,
}
windows_core::imp::define_interface!(ISmartCardAppletIdGroupRegistration2, ISmartCardAppletIdGroupRegistration2_Vtbl, 0x5f5508d8_98a7_4f2e_91d9_6cfcceda407f);
impl windows_core::RuntimeType for ISmartCardAppletIdGroupRegistration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupRegistration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SmartCardReaderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
windows_core::imp::define_interface!(ISmartCardAppletIdGroupStatics, ISmartCardAppletIdGroupStatics_Vtbl, 0xab2899a9_e76c_45cf_bf1d_90eaa6205927);
impl windows_core::RuntimeType for ISmartCardAppletIdGroupStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAppletIdGroupStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxAppletIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardAutomaticResponseApdu, ISmartCardAutomaticResponseApdu_Vtbl, 0x52152bab_c63e_4531_a857_d756d99b986a);
impl windows_core::RuntimeType for ISmartCardAutomaticResponseApdu {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApdu_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApdu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApduBitMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApduBitMask: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetCommandApduBitMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetCommandApduBitMask: usize,
    pub ShouldMatchLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShouldMatchLength: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AppletId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetAppletId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseApdu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseApdu: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponseApdu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponseApdu: usize,
}
windows_core::imp::define_interface!(ISmartCardAutomaticResponseApdu2, ISmartCardAutomaticResponseApdu2_Vtbl, 0x44aebb14_559d_4531_4e51_89db6fa8a57a);
impl windows_core::RuntimeType for ISmartCardAutomaticResponseApdu2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApdu2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInputState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardAutomaticResponseApdu3, ISmartCardAutomaticResponseApdu3_Vtbl, 0xbf43da74_6576_4392_9367_fe3bc9e2d496);
impl windows_core::RuntimeType for ISmartCardAutomaticResponseApdu3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApdu3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWhenCryptogramGeneratorNotPrepared: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardAutomaticResponseApduFactory, ISmartCardAutomaticResponseApduFactory_Vtbl, 0xe97ea2fa_d02c_4c55_b02a_8cff7fa9f05b);
impl windows_core::RuntimeType for ISmartCardAutomaticResponseApduFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardAutomaticResponseApduFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
windows_core::imp::define_interface!(ISmartCardChallengeContext, ISmartCardChallengeContext_Vtbl, 0x192a5319_c9c4_4947_81cc_44794a61ef91);
impl windows_core::RuntimeType for ISmartCardChallengeContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardChallengeContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifyResponseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifyResponseAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProvisionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProvisionAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProvisionAsyncWithNewCardId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProvisionAsyncWithNewCardId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ChangeAdministrativeKeyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ChangeAdministrativeKeyAsync: usize,
}
windows_core::imp::define_interface!(ISmartCardConnect, ISmartCardConnect_Vtbl, 0x2fdf87e5_028d_491e_a058_3382c3986f40);
impl windows_core::RuntimeType for ISmartCardConnect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardConnect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardConnection, ISmartCardConnection_Vtbl, 0x7edb991a_a81a_47bc_a649_156be6b7f231);
impl windows_core::RuntimeType for ISmartCardConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub TransmitAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TransmitAsync: usize,
}
windows_core::imp::define_interface!(ISmartCardCryptogramGenerator, ISmartCardCryptogramGenerator_Vtbl, 0xe39f587b_edd3_4e49_b594_0ff5e4d0c76f);
impl windows_core::RuntimeType for ISmartCardCryptogramGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramGenerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCryptogramMaterialPackageConfirmationResponseFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCryptogramMaterialPackageConfirmationResponseFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedSmartCardCryptogramStorageKeyCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedSmartCardCryptogramStorageKeyCapabilities: usize,
    pub DeleteCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCryptogramMaterialStorageKeyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardUnlockPromptingBehavior, core::mem::MaybeUninit<windows_core::HSTRING>, SmartCardCryptogramStorageKeyAlgorithm, SmartCardCryptogramStorageKeyCapabilities, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")]
    pub RequestCryptogramMaterialStorageKeyInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardUnlockPromptingBehavior, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))]
    RequestCryptogramMaterialStorageKeyInfoAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportCryptogramMaterialPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardCryptogramMaterialPackageFormat, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportCryptogramMaterialPackageAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TryProvePossessionOfCryptogramMaterialPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardUnlockPromptingBehavior, SmartCardCryptogramMaterialPackageConfirmationResponseFormat, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryProvePossessionOfCryptogramMaterialPackageAsync: usize,
    pub RequestUnlockCryptogramMaterialForUseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardUnlockPromptingBehavior, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteCryptogramMaterialPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramGenerator2, ISmartCardCryptogramGenerator2_Vtbl, 0x7116aa34_5d6d_4b4a_96a3_efa47d2a7e25);
impl windows_core::RuntimeType for ISmartCardCryptogramGenerator2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramGenerator2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ValidateRequestApduAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardUnlockPromptingBehavior, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ValidateRequestApduAsync: usize,
    pub GetAllCryptogramStorageKeyCharacteristicsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllCryptogramMaterialPackageCharacteristicsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllCryptogramMaterialCharacteristicsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardUnlockPromptingBehavior, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramGeneratorStatics, ISmartCardCryptogramGeneratorStatics_Vtbl, 0x09933910_cb9c_4015_967d_5234f3b02900);
impl windows_core::RuntimeType for ISmartCardCryptogramGeneratorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramGeneratorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSmartCardCryptogramGeneratorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramGeneratorStatics2, ISmartCardCryptogramGeneratorStatics2_Vtbl, 0x09bdf5e5_b4bd_4e23_a588_74469204c128);
impl windows_core::RuntimeType for ISmartCardCryptogramGeneratorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramGeneratorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl, 0x2798e029_d687_4c92_86c6_399e9a0ecb09);
impl windows_core::RuntimeType for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramGeneratorOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
windows_core::imp::define_interface!(ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl, 0x4e6a8a5c_9773_46c4_a32f_b1e543159e04);
impl windows_core::RuntimeType for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramGeneratorOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
windows_core::imp::define_interface!(ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl, 0x8c7ce857_a7e7_489d_b9d6_368061515012);
impl windows_core::RuntimeType for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramGeneratorOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Characteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Characteristics: usize,
}
windows_core::imp::define_interface!(ISmartCardCryptogramMaterialCharacteristics, ISmartCardCryptogramMaterialCharacteristics_Vtbl, 0xfc9ac5cc_c1d7_4153_923b_a2d43c6c8d49);
impl windows_core::RuntimeType for ISmartCardCryptogramMaterialCharacteristics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramMaterialCharacteristics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaterialName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedProofOfPossessionAlgorithms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedProofOfPossessionAlgorithms: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedValidations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedValidations: usize,
    pub MaterialType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramMaterialType) -> windows_core::HRESULT,
    pub ProtectionMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramMaterialProtectionMethod) -> windows_core::HRESULT,
    pub ProtectionVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaterialLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramMaterialPackageCharacteristics, ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl, 0xffb58e1f_0692_4c47_93cf_34d91f9dcd00);
impl windows_core::RuntimeType for ISmartCardCryptogramMaterialPackageCharacteristics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StorageKeyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DateImported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PackageFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramMaterialPackageFormat) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramMaterialPossessionProof, ISmartCardCryptogramMaterialPossessionProof_Vtbl, 0xe5b9ab8c_a141_4135_9add_b0d2e3aa1fc9);
impl windows_core::RuntimeType for ISmartCardCryptogramMaterialPossessionProof {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramMaterialPossessionProof_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramGeneratorOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Proof: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Proof: usize,
}
windows_core::imp::define_interface!(ISmartCardCryptogramPlacementStep, ISmartCardCryptogramPlacementStep_Vtbl, 0x947b03eb_8342_4792_a2e5_925636378a53);
impl windows_core::RuntimeType for ISmartCardCryptogramPlacementStep {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramPlacementStep_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Algorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramAlgorithm) -> windows_core::HRESULT,
    pub SetAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardCryptogramAlgorithm) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceData: usize,
    pub CryptogramMaterialPackageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCryptogramMaterialPackageName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CryptogramMaterialName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCryptogramMaterialName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TemplateOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTemplateOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CryptogramOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCryptogramOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CryptogramLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCryptogramLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CryptogramPlacementOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramPlacementOptions) -> windows_core::HRESULT,
    pub SetCryptogramPlacementOptions: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardCryptogramPlacementOptions) -> windows_core::HRESULT,
    pub ChainedOutputStep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetChainedOutputStep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramStorageKeyCharacteristics, ISmartCardCryptogramStorageKeyCharacteristics_Vtbl, 0x8552546e_4457_4825_b464_635471a39f5c);
impl windows_core::RuntimeType for ISmartCardCryptogramStorageKeyCharacteristics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StorageKeyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DateCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Algorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramStorageKeyAlgorithm) -> windows_core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramStorageKeyCapabilities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramStorageKeyInfo, ISmartCardCryptogramStorageKeyInfo_Vtbl, 0x77b0f00d_b097_4f61_a26a_9561639c9c3a);
impl windows_core::RuntimeType for ISmartCardCryptogramStorageKeyInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramStorageKeyInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramGeneratorOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")]
    pub PublicKeyBlobType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))]
    PublicKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublicKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublicKey: usize,
    pub AttestationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptographicKeyAttestationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Attestation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Attestation: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationCertificateChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationCertificateChain: usize,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardCryptogramStorageKeyCapabilities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardCryptogramStorageKeyInfo2, ISmartCardCryptogramStorageKeyInfo2_Vtbl, 0x000440f9_f7fd_417d_89e1_fbb0382adc4d);
impl windows_core::RuntimeType for ISmartCardCryptogramStorageKeyInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardCryptogramStorageKeyInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OperationalRequirements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulator, ISmartCardEmulator_Vtbl, 0xdfb906b2_875e_47e5_8077_e8bff1b1c6fb);
impl windows_core::RuntimeType for ISmartCardEmulator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnablementPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardEmulatorEnablementPolicy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulator2, ISmartCardEmulator2_Vtbl, 0xfe3fc0b8_8529_411a_807b_48edc2a0ab44);
impl windows_core::RuntimeType for ISmartCardEmulator2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulator2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ApduReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveApduReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ConnectionDeactivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConnectionDeactivated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsHostCardEmulationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulatorApduReceivedEventArgs, ISmartCardEmulatorApduReceivedEventArgs_Vtbl, 0xd55d1576_69d2_5333_5b5f_f8c0d6e9f09f);
impl windows_core::RuntimeType for ISmartCardEmulatorApduReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorApduReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CommandApdu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CommandApdu: usize,
    pub ConnectionProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub TryRespondAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryRespondAsync: usize,
    pub AutomaticResponseStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardAutomaticResponseStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulatorApduReceivedEventArgs2, ISmartCardEmulatorApduReceivedEventArgs2_Vtbl, 0x8bf93df0_22e1_4238_8610_94ce4a965425);
impl windows_core::RuntimeType for ISmartCardEmulatorApduReceivedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub TryRespondWithStateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryRespondWithStateAsync: usize,
}
windows_core::imp::define_interface!(ISmartCardEmulatorApduReceivedEventArgsWithCryptograms, ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_Vtbl, 0xd550bac7_b7bf_4e29_9294_0c4ac3c941bd);
impl windows_core::RuntimeType for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub TryRespondWithCryptogramsAndStateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    TryRespondWithCryptogramsAndStateAsync: usize,
}
windows_core::imp::define_interface!(ISmartCardEmulatorConnectionDeactivatedEventArgs, ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl, 0x2186d8d3_c5eb_5262_43df_62a0a1b55557);
impl windows_core::RuntimeType for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardEmulatorConnectionDeactivatedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulatorConnectionProperties, ISmartCardEmulatorConnectionProperties_Vtbl, 0x4e2ca5ee_f969_507d_6cf9_34e2d18df311);
impl windows_core::RuntimeType for ISmartCardEmulatorConnectionProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorConnectionProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardEmulatorConnectionSource) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulatorStatics, ISmartCardEmulatorStatics_Vtbl, 0x7a9bfc4b_c4d3_494f_b8a2_6215d81e85b2);
impl windows_core::RuntimeType for ISmartCardEmulatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulatorStatics2, ISmartCardEmulatorStatics2_Vtbl, 0x69ae9f8a_b775_488b_8436_6c1e28ed731f);
impl windows_core::RuntimeType for ISmartCardEmulatorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppletIdGroupRegistrationsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppletIdGroupRegistrationsAsync: usize,
    pub RegisterAppletIdGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterAppletIdGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxAppletIdGroupRegistrations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardEmulatorStatics3, ISmartCardEmulatorStatics3_Vtbl, 0x59ea142a_9f09_43f5_8565_cfa8148e4cb2);
impl windows_core::RuntimeType for ISmartCardEmulatorStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardEmulatorStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardPinPolicy, ISmartCardPinPolicy_Vtbl, 0x183ce184_4db6_4841_ac9e_2ac1f39b7304);
impl windows_core::RuntimeType for ISmartCardPinPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardPinPolicy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MinLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UppercaseLetters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
    pub SetUppercaseLetters: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
    pub LowercaseLetters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
    pub SetLowercaseLetters: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
    pub Digits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
    pub SetDigits: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
    pub SpecialCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
    pub SetSpecialCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardPinCharacterPolicyOption) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardPinResetDeferral, ISmartCardPinResetDeferral_Vtbl, 0x18c94aac_7805_4004_85e4_bbefac8f6884);
impl windows_core::RuntimeType for ISmartCardPinResetDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardPinResetDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardPinResetRequest, ISmartCardPinResetRequest_Vtbl, 0x12fe3c4d_5fb9_4e8e_9ff6_61f475124fef);
impl windows_core::RuntimeType for ISmartCardPinResetRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardPinResetRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Challenge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Challenge: usize,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetResponse: usize,
}
windows_core::imp::define_interface!(ISmartCardProvisioning, ISmartCardProvisioning_Vtbl, 0x19eeedbd_1fab_477c_b712_1a2c5af1fd6e);
impl windows_core::RuntimeType for ISmartCardProvisioning {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardProvisioning_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChallengeContextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestPinChangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestPinResetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardProvisioning2, ISmartCardProvisioning2_Vtbl, 0x10fd28eb_3f79_4b66_9b7c_11c149b7d0bc);
impl windows_core::RuntimeType for ISmartCardProvisioning2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardProvisioning2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAuthorityKeyContainerNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardProvisioningStatics, ISmartCardProvisioningStatics_Vtbl, 0x13882848_0d13_4e70_9735_51daeca5254f);
impl windows_core::RuntimeType for ISmartCardProvisioningStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardProvisioningStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromSmartCardAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RequestVirtualSmartCardCreationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestVirtualSmartCardCreationAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RequestVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestVirtualSmartCardCreationAsyncWithCardId: usize,
    pub RequestVirtualSmartCardDeletionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardProvisioningStatics2, ISmartCardProvisioningStatics2_Vtbl, 0x3447c6a8_c9a0_4bd6_b50d_251f4e8d3a62);
impl windows_core::RuntimeType for ISmartCardProvisioningStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardProvisioningStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub RequestAttestedVirtualSmartCardCreationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestAttestedVirtualSmartCardCreationAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RequestAttestedVirtualSmartCardCreationAsyncWithCardId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestAttestedVirtualSmartCardCreationAsyncWithCardId: usize,
}
windows_core::imp::define_interface!(ISmartCardReader, ISmartCardReader_Vtbl, 0x1074b4e0_54c2_4df0_817a_14c14378f06c);
impl windows_core::RuntimeType for ISmartCardReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardReaderKind) -> windows_core::HRESULT,
    pub GetStatusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllCardsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllCardsAsync: usize,
    pub CardAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCardAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CardRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCardRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardReaderStatics, ISmartCardReaderStatics_Vtbl, 0x103c04e1_a1ca_48f2_a281_5b6f669af107);
impl windows_core::RuntimeType for ISmartCardReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorWithKind: unsafe extern "system" fn(*mut core::ffi::c_void, SmartCardReaderKind, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardTriggerDetails, ISmartCardTriggerDetails_Vtbl, 0x5f9bf11e_39ef_4f2b_b44f_0a9155b177bc);
impl windows_core::RuntimeType for ISmartCardTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TriggerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SmartCardTriggerType) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SourceAppletId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceAppletId: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TriggerData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TriggerData: usize,
}
windows_core::imp::define_interface!(ISmartCardTriggerDetails2, ISmartCardTriggerDetails2_Vtbl, 0x2945c569_8975_4a51_9e1a_5f8a76ee51af);
impl windows_core::RuntimeType for ISmartCardTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Emulator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryLaunchCurrentAppAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryLaunchCurrentAppWithBehaviorAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, SmartCardLaunchBehavior, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISmartCardTriggerDetails3, ISmartCardTriggerDetails3_Vtbl, 0xb3e2c27d_18c6_4ba8_8376_ef03d4912666);
impl windows_core::RuntimeType for ISmartCardTriggerDetails3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardTriggerDetails3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SmartCard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CardAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CardAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CardAddedEventArgs {
    pub fn SmartCard(&self) -> windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartCard)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CardAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICardAddedEventArgs>();
}
unsafe impl windows_core::Interface for CardAddedEventArgs {
    type Vtable = ICardAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICardAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CardAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardAddedEventArgs";
}
unsafe impl Send for CardAddedEventArgs {}
unsafe impl Sync for CardAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CardRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CardRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CardRemovedEventArgs {
    pub fn SmartCard(&self) -> windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartCard)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CardRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICardRemovedEventArgs>();
}
unsafe impl windows_core::Interface for CardRemovedEventArgs {
    type Vtable = ICardRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICardRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CardRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.CardRemovedEventArgs";
}
unsafe impl Send for CardRemovedEventArgs {}
unsafe impl Sync for CardRemovedEventArgs {}
pub struct KnownSmartCardAppletIds;
impl KnownSmartCardAppletIds {
    #[cfg(feature = "Storage_Streams")]
    pub fn PaymentSystemEnvironment() -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaymentSystemEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProximityPaymentSystemEnvironment() -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IKnownSmartCardAppletIds(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProximityPaymentSystemEnvironment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownSmartCardAppletIds<R, F: FnOnce(&IKnownSmartCardAppletIds) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownSmartCardAppletIds, IKnownSmartCardAppletIds> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownSmartCardAppletIds {
    const NAME: &'static str = "Windows.Devices.SmartCards.KnownSmartCardAppletIds";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCard(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCard, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCard {
    pub fn Reader(&self) -> windows_core::Result<SmartCardReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetStatusAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatusAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetAnswerToResetAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAnswerToResetAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardConnection>> {
        let this = &windows_core::Interface::cast::<ISmartCardConnect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCard {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCard>();
}
unsafe impl windows_core::Interface for SmartCard {
    type Vtable = ISmartCard_Vtbl;
    const IID: windows_core::GUID = <ISmartCard as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCard {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCard";
}
unsafe impl Send for SmartCard {}
unsafe impl Sync for SmartCard {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardAppletIdGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardAppletIdGroup, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardAppletIdGroup {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardAppletIdGroup, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn AppletIds(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppletIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SmartCardEmulationCategory(&self) -> windows_core::Result<SmartCardEmulationCategory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartCardEmulationCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSmartCardEmulationCategory(&self, value: SmartCardEmulationCategory) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSmartCardEmulationCategory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SmartCardEmulationType(&self) -> windows_core::Result<SmartCardEmulationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartCardEmulationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSmartCardEmulationType(&self, value: SmartCardEmulationType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSmartCardEmulationType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticEnablement(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomaticEnablement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutomaticEnablement(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutomaticEnablement)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Logo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLogo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLogo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SecureUserAuthenticationRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecureUserAuthenticationRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSecureUserAuthenticationRequired(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroup2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSecureUserAuthenticationRequired)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Create<P0>(displayname: &windows_core::HSTRING, appletids: P0, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType) -> windows_core::Result<SmartCardAppletIdGroup>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>,
    {
        Self::ISmartCardAppletIdGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displayname), appletids.param().abi(), emulationcategory, emulationtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MaxAppletIds() -> windows_core::Result<u16> {
        Self::ISmartCardAppletIdGroupStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAppletIds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardAppletIdGroupFactory<R, F: FnOnce(&ISmartCardAppletIdGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardAppletIdGroupStatics<R, F: FnOnce(&ISmartCardAppletIdGroupStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardAppletIdGroup, ISmartCardAppletIdGroupStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmartCardAppletIdGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardAppletIdGroup>();
}
unsafe impl windows_core::Interface for SmartCardAppletIdGroup {
    type Vtable = ISmartCardAppletIdGroup_Vtbl;
    const IID: windows_core::GUID = <ISmartCardAppletIdGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardAppletIdGroup {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroup";
}
unsafe impl Send for SmartCardAppletIdGroup {}
unsafe impl Sync for SmartCardAppletIdGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardAppletIdGroupRegistration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardAppletIdGroupRegistration, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardAppletIdGroupRegistration {
    pub fn ActivationPolicy(&self) -> windows_core::Result<SmartCardAppletIdGroupActivationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivationPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppletIdGroup(&self) -> windows_core::Result<SmartCardAppletIdGroup> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppletIdGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestActivationPolicyChangeAsync(&self, policy: SmartCardAppletIdGroupActivationPolicy) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestActivationPolicyChangeAsync)(windows_core::Interface::as_raw(this), policy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAutomaticResponseApdusAsync<P0>(&self, apdus: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<SmartCardAutomaticResponseApdu>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAutomaticResponseApdusAsync)(windows_core::Interface::as_raw(this), apdus.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SmartCardReaderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartCardReaderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPropertiesAsync<P0>(&self, props: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardAppletIdGroupRegistration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPropertiesAsync)(windows_core::Interface::as_raw(this), props.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardAppletIdGroupRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardAppletIdGroupRegistration>();
}
unsafe impl windows_core::Interface for SmartCardAppletIdGroupRegistration {
    type Vtable = ISmartCardAppletIdGroupRegistration_Vtbl;
    const IID: windows_core::GUID = <ISmartCardAppletIdGroupRegistration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardAppletIdGroupRegistration {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAppletIdGroupRegistration";
}
unsafe impl Send for SmartCardAppletIdGroupRegistration {}
unsafe impl Sync for SmartCardAppletIdGroupRegistration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardAutomaticResponseApdu(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardAutomaticResponseApdu, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardAutomaticResponseApdu {
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApdu(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandApdu)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetCommandApdu<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommandApdu)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApduBitMask(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandApduBitMask)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetCommandApduBitMask<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommandApduBitMask)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ShouldMatchLength(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldMatchLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShouldMatchLength(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShouldMatchLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AppletId(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppletId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetAppletId<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppletId)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ResponseApdu(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseApdu)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetResponseApdu<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResponseApdu)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn InputState(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInputState<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInputState)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OutputState(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutputState<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutputState)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowWhenCryptogramGeneratorNotPrepared(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWhenCryptogramGeneratorNotPrepared)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowWhenCryptogramGeneratorNotPrepared(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISmartCardAutomaticResponseApdu3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowWhenCryptogramGeneratorNotPrepared)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0, P1>(commandapdu: P0, responseapdu: P1) -> windows_core::Result<SmartCardAutomaticResponseApdu>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISmartCardAutomaticResponseApduFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), commandapdu.param().abi(), responseapdu.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardAutomaticResponseApduFactory<R, F: FnOnce(&ISmartCardAutomaticResponseApduFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardAutomaticResponseApdu, ISmartCardAutomaticResponseApduFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmartCardAutomaticResponseApdu {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardAutomaticResponseApdu>();
}
unsafe impl windows_core::Interface for SmartCardAutomaticResponseApdu {
    type Vtable = ISmartCardAutomaticResponseApdu_Vtbl;
    const IID: windows_core::GUID = <ISmartCardAutomaticResponseApdu as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardAutomaticResponseApdu {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardAutomaticResponseApdu";
}
unsafe impl Send for SmartCardAutomaticResponseApdu {}
unsafe impl Sync for SmartCardAutomaticResponseApdu {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardChallengeContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardChallengeContext, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmartCardChallengeContext, super::super::Foundation::IClosable);
impl SmartCardChallengeContext {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Challenge(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Challenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifyResponseAsync<P0>(&self, response: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerifyResponseAsync)(windows_core::Interface::as_raw(this), response.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProvisionAsync<P0>(&self, response: P0, formatcard: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProvisionAsync)(windows_core::Interface::as_raw(this), response.param().abi(), formatcard, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProvisionAsyncWithNewCardId<P0>(&self, response: P0, formatcard: bool, newcardid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProvisionAsyncWithNewCardId)(windows_core::Interface::as_raw(this), response.param().abi(), formatcard, newcardid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ChangeAdministrativeKeyAsync<P0, P1>(&self, response: P0, newadministrativekey: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeAdministrativeKeyAsync)(windows_core::Interface::as_raw(this), response.param().abi(), newadministrativekey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardChallengeContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardChallengeContext>();
}
unsafe impl windows_core::Interface for SmartCardChallengeContext {
    type Vtable = ISmartCardChallengeContext_Vtbl;
    const IID: windows_core::GUID = <ISmartCardChallengeContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardChallengeContext {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardChallengeContext";
}
unsafe impl Send for SmartCardChallengeContext {}
unsafe impl Sync for SmartCardChallengeContext {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardConnection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmartCardConnection, super::super::Foundation::IClosable);
impl SmartCardConnection {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TransmitAsync<P0>(&self, command: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitAsync)(windows_core::Interface::as_raw(this), command.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardConnection>();
}
unsafe impl windows_core::Interface for SmartCardConnection {
    type Vtable = ISmartCardConnection_Vtbl;
    const IID: windows_core::GUID = <ISmartCardConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardConnection {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardConnection";
}
unsafe impl Send for SmartCardConnection {}
unsafe impl Sync for SmartCardConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramGenerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramGenerator, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramGenerator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialTypes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCryptogramMaterialTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramAlgorithms(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCryptogramAlgorithms)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialPackageFormats(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCryptogramMaterialPackageFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCryptogramMaterialPackageConfirmationResponseFormats(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCryptogramMaterialPackageConfirmationResponseFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedSmartCardCryptogramStorageKeyCapabilities(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedSmartCardCryptogramStorageKeyCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteCryptogramMaterialStorageKeyAsync(&self, storagekeyname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteCryptogramMaterialStorageKeyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(storagekeyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateCryptogramMaterialStorageKeyAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &windows_core::HSTRING, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCryptogramMaterialStorageKeyAsync)(windows_core::Interface::as_raw(this), promptingbehavior, core::mem::transmute_copy(storagekeyname), algorithm, capabilities, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Core")]
    pub fn RequestCryptogramMaterialStorageKeyInfoAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &windows_core::HSTRING, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCryptogramMaterialStorageKeyInfoAsync)(windows_core::Interface::as_raw(this), promptingbehavior, core::mem::transmute_copy(storagekeyname), format, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportCryptogramMaterialPackageAsync<P0>(&self, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: &windows_core::HSTRING, materialpackagename: &windows_core::HSTRING, cryptogrammaterialpackage: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportCryptogramMaterialPackageAsync)(windows_core::Interface::as_raw(this), format, core::mem::transmute_copy(storagekeyname), core::mem::transmute_copy(materialpackagename), cryptogrammaterialpackage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryProvePossessionOfCryptogramMaterialPackageAsync<P0>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: &windows_core::HSTRING, materialname: &windows_core::HSTRING, challenge: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryProvePossessionOfCryptogramMaterialPackageAsync)(windows_core::Interface::as_raw(this), promptingbehavior, responseformat, core::mem::transmute_copy(materialpackagename), core::mem::transmute_copy(materialname), challenge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestUnlockCryptogramMaterialForUseAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestUnlockCryptogramMaterialForUseAsync)(windows_core::Interface::as_raw(this), promptingbehavior, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteCryptogramMaterialPackageAsync(&self, materialpackagename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteCryptogramMaterialPackageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(materialpackagename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ValidateRequestApduAsync<P0, P1>(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: P0, cryptogramplacementsteps: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateRequestApduAsync)(windows_core::Interface::as_raw(this), promptingbehavior, apdutovalidate.param().abi(), cryptogramplacementsteps.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAllCryptogramStorageKeyCharacteristicsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllCryptogramStorageKeyCharacteristicsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAllCryptogramMaterialPackageCharacteristicsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllCryptogramMaterialPackageCharacteristicsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync(&self, storagekeyname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(storagekeyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAllCryptogramMaterialCharacteristicsAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>> {
        let this = &windows_core::Interface::cast::<ISmartCardCryptogramGenerator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllCryptogramMaterialCharacteristicsAsync)(windows_core::Interface::as_raw(this), promptingbehavior, core::mem::transmute_copy(materialpackagename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSmartCardCryptogramGeneratorAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGenerator>> {
        Self::ISmartCardCryptogramGeneratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSmartCardCryptogramGeneratorAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::ISmartCardCryptogramGeneratorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardCryptogramGeneratorStatics<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardCryptogramGeneratorStatics2<R, F: FnOnce(&ISmartCardCryptogramGeneratorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramGenerator, ISmartCardCryptogramGeneratorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramGenerator>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramGenerator {
    type Vtable = ISmartCardCryptogramGenerator_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramGenerator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramGenerator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGenerator";
}
unsafe impl Send for SmartCardCryptogramGenerator {}
unsafe impl Sync for SmartCardCryptogramGenerator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OperationStatus(&self) -> windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult";
}
unsafe impl Send for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
unsafe impl Sync for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OperationStatus(&self) -> windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult";
}
unsafe impl Send for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
unsafe impl Sync for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OperationStatus(&self) -> windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Characteristics(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    type Vtable = ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult";
}
unsafe impl Send for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
unsafe impl Sync for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramMaterialCharacteristics(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramMaterialCharacteristics, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramMaterialCharacteristics {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramMaterialCharacteristics, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaterialName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedAlgorithms(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedAlgorithms)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedProofOfPossessionAlgorithms(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedProofOfPossessionAlgorithms)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedValidations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedValidations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaterialType(&self) -> windows_core::Result<SmartCardCryptogramMaterialType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtectionMethod(&self) -> windows_core::Result<SmartCardCryptogramMaterialProtectionMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtectionVersion(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaterialLength(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramMaterialCharacteristics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramMaterialCharacteristics>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramMaterialCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialCharacteristics_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramMaterialCharacteristics as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramMaterialCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialCharacteristics";
}
unsafe impl Send for SmartCardCryptogramMaterialCharacteristics {}
unsafe impl Sync for SmartCardCryptogramMaterialCharacteristics {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramMaterialPackageCharacteristics(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramMaterialPackageCharacteristics, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramMaterialPackageCharacteristics {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramMaterialPackageCharacteristics, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PackageName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StorageKeyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StorageKeyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DateImported(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateImported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PackageFormat(&self) -> windows_core::Result<SmartCardCryptogramMaterialPackageFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramMaterialPackageCharacteristics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramMaterialPackageCharacteristics>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramMaterialPackageCharacteristics {
    type Vtable = ISmartCardCryptogramMaterialPackageCharacteristics_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramMaterialPackageCharacteristics as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramMaterialPackageCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageCharacteristics";
}
unsafe impl Send for SmartCardCryptogramMaterialPackageCharacteristics {}
unsafe impl Sync for SmartCardCryptogramMaterialPackageCharacteristics {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramMaterialPossessionProof(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramMaterialPossessionProof, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramMaterialPossessionProof {
    pub fn OperationStatus(&self) -> windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Proof(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Proof)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramMaterialPossessionProof {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramMaterialPossessionProof>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramMaterialPossessionProof {
    type Vtable = ISmartCardCryptogramMaterialPossessionProof_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramMaterialPossessionProof as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramMaterialPossessionProof {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramMaterialPossessionProof";
}
unsafe impl Send for SmartCardCryptogramMaterialPossessionProof {}
unsafe impl Sync for SmartCardCryptogramMaterialPossessionProof {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramPlacementStep(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramPlacementStep, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramPlacementStep {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramPlacementStep, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Algorithm(&self) -> windows_core::Result<SmartCardCryptogramAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Algorithm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlgorithm(&self, value: SmartCardCryptogramAlgorithm) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlgorithm)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSourceData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSourceData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CryptogramMaterialPackageName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CryptogramMaterialPackageName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCryptogramMaterialPackageName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCryptogramMaterialPackageName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CryptogramMaterialName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CryptogramMaterialName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCryptogramMaterialName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCryptogramMaterialName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TemplateOffset(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TemplateOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTemplateOffset(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTemplateOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramOffset(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CryptogramOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCryptogramOffset(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCryptogramOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramLength(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CryptogramLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCryptogramLength(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCryptogramLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CryptogramPlacementOptions(&self) -> windows_core::Result<SmartCardCryptogramPlacementOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CryptogramPlacementOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCryptogramPlacementOptions(&self, value: SmartCardCryptogramPlacementOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCryptogramPlacementOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChainedOutputStep(&self) -> windows_core::Result<SmartCardCryptogramPlacementStep> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChainedOutputStep)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChainedOutputStep<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SmartCardCryptogramPlacementStep>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetChainedOutputStep)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramPlacementStep {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramPlacementStep>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramPlacementStep {
    type Vtable = ISmartCardCryptogramPlacementStep_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramPlacementStep as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramPlacementStep {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramPlacementStep";
}
unsafe impl Send for SmartCardCryptogramPlacementStep {}
unsafe impl Sync for SmartCardCryptogramPlacementStep {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramStorageKeyCharacteristics(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramStorageKeyCharacteristics, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramStorageKeyCharacteristics {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardCryptogramStorageKeyCharacteristics, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StorageKeyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StorageKeyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Algorithm(&self) -> windows_core::Result<SmartCardCryptogramStorageKeyAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Algorithm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramStorageKeyCharacteristics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramStorageKeyCharacteristics>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramStorageKeyCharacteristics {
    type Vtable = ISmartCardCryptogramStorageKeyCharacteristics_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramStorageKeyCharacteristics as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramStorageKeyCharacteristics {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCharacteristics";
}
unsafe impl Send for SmartCardCryptogramStorageKeyCharacteristics {}
unsafe impl Sync for SmartCardCryptogramStorageKeyCharacteristics {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardCryptogramStorageKeyInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardCryptogramStorageKeyInfo, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardCryptogramStorageKeyInfo {
    pub fn OperationStatus(&self) -> windows_core::Result<SmartCardCryptogramGeneratorOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Security_Cryptography_Core")]
    pub fn PublicKeyBlobType(&self) -> windows_core::Result<super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublicKeyBlobType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PublicKey(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublicKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttestationStatus(&self) -> windows_core::Result<SmartCardCryptographicKeyAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttestationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Attestation(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attestation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AttestationCertificateChain(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttestationCertificateChain)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Capabilities(&self) -> windows_core::Result<SmartCardCryptogramStorageKeyCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OperationalRequirements(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ISmartCardCryptogramStorageKeyInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperationalRequirements)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramStorageKeyInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardCryptogramStorageKeyInfo>();
}
unsafe impl windows_core::Interface for SmartCardCryptogramStorageKeyInfo {
    type Vtable = ISmartCardCryptogramStorageKeyInfo_Vtbl;
    const IID: windows_core::GUID = <ISmartCardCryptogramStorageKeyInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardCryptogramStorageKeyInfo {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyInfo";
}
unsafe impl Send for SmartCardCryptogramStorageKeyInfo {}
unsafe impl Sync for SmartCardCryptogramStorageKeyInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardEmulator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardEmulator, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardEmulator {
    pub fn EnablementPolicy(&self) -> windows_core::Result<SmartCardEmulatorEnablementPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnablementPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ApduReceived<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApduReceived)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveApduReceived(&self, value: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveApduReceived)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ConnectionDeactivated<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionDeactivated)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionDeactivated(&self, value: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionDeactivated)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsHostCardEmulationSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISmartCardEmulator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHostCardEmulationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardEmulator>> {
        Self::ISmartCardEmulatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppletIdGroupRegistrationsAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppletIdGroupRegistrationsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RegisterAppletIdGroupAsync<P0>(appletidgroup: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>>
    where
        P0: windows_core::Param<SmartCardAppletIdGroup>,
    {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterAppletIdGroupAsync)(windows_core::Interface::as_raw(this), appletidgroup.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UnregisterAppletIdGroupAsync<P0>(registration: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<SmartCardAppletIdGroupRegistration>,
    {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnregisterAppletIdGroupAsync)(windows_core::Interface::as_raw(this), registration.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MaxAppletIdGroupRegistrations() -> windows_core::Result<u16> {
        Self::ISmartCardEmulatorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAppletIdGroupRegistrations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::ISmartCardEmulatorStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardEmulatorStatics<R, F: FnOnce(&ISmartCardEmulatorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardEmulatorStatics2<R, F: FnOnce(&ISmartCardEmulatorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardEmulatorStatics3<R, F: FnOnce(&ISmartCardEmulatorStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardEmulator, ISmartCardEmulatorStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmartCardEmulator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardEmulator>();
}
unsafe impl windows_core::Interface for SmartCardEmulator {
    type Vtable = ISmartCardEmulator_Vtbl;
    const IID: windows_core::GUID = <ISmartCardEmulator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardEmulator {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulator";
}
unsafe impl Send for SmartCardEmulator {}
unsafe impl Sync for SmartCardEmulator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardEmulatorApduReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardEmulatorApduReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardEmulatorApduReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn CommandApdu(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandApdu)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionProperties(&self) -> windows_core::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryRespondAsync<P0>(&self, responseapdu: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRespondAsync)(windows_core::Interface::as_raw(this), responseapdu.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutomaticResponseStatus(&self) -> windows_core::Result<SmartCardAutomaticResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomaticResponseStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryRespondWithStateAsync<P0, P1>(&self, responseapdu: P0, nextstate: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRespondWithStateAsync)(windows_core::Interface::as_raw(this), responseapdu.param().abi(), nextstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryRespondWithCryptogramsAsync<P0, P1>(&self, responsetemplate: P0, cryptogramplacementsteps: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRespondWithCryptogramsAsync)(windows_core::Interface::as_raw(this), responsetemplate.param().abi(), cryptogramplacementsteps.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryRespondWithCryptogramsAndStateAsync<P0, P1, P2>(&self, responsetemplate: P0, cryptogramplacementsteps: P1, nextstate: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>,
        P2: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<ISmartCardEmulatorApduReceivedEventArgsWithCryptograms>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRespondWithCryptogramsAndStateAsync)(windows_core::Interface::as_raw(this), responsetemplate.param().abi(), cryptogramplacementsteps.param().abi(), nextstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardEmulatorApduReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardEmulatorApduReceivedEventArgs>();
}
unsafe impl windows_core::Interface for SmartCardEmulatorApduReceivedEventArgs {
    type Vtable = ISmartCardEmulatorApduReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISmartCardEmulatorApduReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardEmulatorApduReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorApduReceivedEventArgs";
}
unsafe impl Send for SmartCardEmulatorApduReceivedEventArgs {}
unsafe impl Sync for SmartCardEmulatorApduReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardEmulatorConnectionDeactivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardEmulatorConnectionDeactivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardEmulatorConnectionDeactivatedEventArgs {
    pub fn ConnectionProperties(&self) -> windows_core::Result<SmartCardEmulatorConnectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reason(&self) -> windows_core::Result<SmartCardEmulatorConnectionDeactivatedReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardEmulatorConnectionDeactivatedEventArgs>();
}
unsafe impl windows_core::Interface for SmartCardEmulatorConnectionDeactivatedEventArgs {
    type Vtable = ISmartCardEmulatorConnectionDeactivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISmartCardEmulatorConnectionDeactivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardEmulatorConnectionDeactivatedEventArgs {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedEventArgs";
}
unsafe impl Send for SmartCardEmulatorConnectionDeactivatedEventArgs {}
unsafe impl Sync for SmartCardEmulatorConnectionDeactivatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardEmulatorConnectionProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardEmulatorConnectionProperties, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardEmulatorConnectionProperties {
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Source(&self) -> windows_core::Result<SmartCardEmulatorConnectionSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SmartCardEmulatorConnectionProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardEmulatorConnectionProperties>();
}
unsafe impl windows_core::Interface for SmartCardEmulatorConnectionProperties {
    type Vtable = ISmartCardEmulatorConnectionProperties_Vtbl;
    const IID: windows_core::GUID = <ISmartCardEmulatorConnectionProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardEmulatorConnectionProperties {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardEmulatorConnectionProperties";
}
unsafe impl Send for SmartCardEmulatorConnectionProperties {}
unsafe impl Sync for SmartCardEmulatorConnectionProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardPinPolicy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardPinPolicy, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardPinPolicy {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardPinPolicy, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MinLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinLength(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMinLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxLength(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UppercaseLetters(&self) -> windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UppercaseLetters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUppercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUppercaseLetters)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowercaseLetters(&self) -> windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LowercaseLetters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLowercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLowercaseLetters)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Digits(&self) -> windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Digits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDigits(&self, value: SmartCardPinCharacterPolicyOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDigits)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpecialCharacters(&self) -> windows_core::Result<SmartCardPinCharacterPolicyOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpecialCharacters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpecialCharacters(&self, value: SmartCardPinCharacterPolicyOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpecialCharacters)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SmartCardPinPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardPinPolicy>();
}
unsafe impl windows_core::Interface for SmartCardPinPolicy {
    type Vtable = ISmartCardPinPolicy_Vtbl;
    const IID: windows_core::GUID = <ISmartCardPinPolicy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardPinPolicy {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinPolicy";
}
unsafe impl Send for SmartCardPinPolicy {}
unsafe impl Sync for SmartCardPinPolicy {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardPinResetDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardPinResetDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardPinResetDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SmartCardPinResetDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardPinResetDeferral>();
}
unsafe impl windows_core::Interface for SmartCardPinResetDeferral {
    type Vtable = ISmartCardPinResetDeferral_Vtbl;
    const IID: windows_core::GUID = <ISmartCardPinResetDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardPinResetDeferral {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetDeferral";
}
unsafe impl Send for SmartCardPinResetDeferral {}
unsafe impl Sync for SmartCardPinResetDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardPinResetRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardPinResetRequest, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardPinResetRequest {
    #[cfg(feature = "Storage_Streams")]
    pub fn Challenge(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Challenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<SmartCardPinResetDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetResponse<P0>(&self, response: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResponse)(windows_core::Interface::as_raw(this), response.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for SmartCardPinResetRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardPinResetRequest>();
}
unsafe impl windows_core::Interface for SmartCardPinResetRequest {
    type Vtable = ISmartCardPinResetRequest_Vtbl;
    const IID: windows_core::GUID = <ISmartCardPinResetRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardPinResetRequest {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardPinResetRequest";
}
unsafe impl Send for SmartCardPinResetRequest {}
unsafe impl Sync for SmartCardPinResetRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardProvisioning(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardProvisioning, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardProvisioning {
    pub fn SmartCard(&self) -> windows_core::Result<SmartCard> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartCard)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIdAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIdAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNameAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetChallengeContextAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardChallengeContext>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChallengeContextAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestPinChangeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPinChangeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestPinResetAsync<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<SmartCardPinResetHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPinResetAsync)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAuthorityKeyContainerNameAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ISmartCardProvisioning2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAuthorityKeyContainerNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromSmartCardAsync<P0>(card: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: windows_core::Param<SmartCard>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromSmartCardAsync)(windows_core::Interface::as_raw(this), card.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestVirtualSmartCardCreationAsync<P0, P1>(friendlyname: &windows_core::HSTRING, administrativekey: P0, pinpolicy: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestVirtualSmartCardCreationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(friendlyname), administrativekey.param().abi(), pinpolicy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestVirtualSmartCardCreationAsyncWithCardId<P0, P1>(friendlyname: &windows_core::HSTRING, administrativekey: P0, pinpolicy: P1, cardid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestVirtualSmartCardCreationAsyncWithCardId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(friendlyname), administrativekey.param().abi(), pinpolicy.param().abi(), cardid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestVirtualSmartCardDeletionAsync<P0>(card: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<SmartCard>,
    {
        Self::ISmartCardProvisioningStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestVirtualSmartCardDeletionAsync)(windows_core::Interface::as_raw(this), card.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestAttestedVirtualSmartCardCreationAsync<P0, P1>(friendlyname: &windows_core::HSTRING, administrativekey: P0, pinpolicy: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAttestedVirtualSmartCardCreationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(friendlyname), administrativekey.param().abi(), pinpolicy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestAttestedVirtualSmartCardCreationAsyncWithCardId<P0, P1>(friendlyname: &windows_core::HSTRING, administrativekey: P0, pinpolicy: P1, cardid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<SmartCardPinPolicy>,
    {
        Self::ISmartCardProvisioningStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAttestedVirtualSmartCardCreationAsyncWithCardId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(friendlyname), administrativekey.param().abi(), pinpolicy.param().abi(), cardid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardProvisioningStatics<R, F: FnOnce(&ISmartCardProvisioningStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISmartCardProvisioningStatics2<R, F: FnOnce(&ISmartCardProvisioningStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardProvisioning, ISmartCardProvisioningStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmartCardProvisioning {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardProvisioning>();
}
unsafe impl windows_core::Interface for SmartCardProvisioning {
    type Vtable = ISmartCardProvisioning_Vtbl;
    const IID: windows_core::GUID = <ISmartCardProvisioning as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardProvisioning {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardProvisioning";
}
unsafe impl Send for SmartCardProvisioning {}
unsafe impl Sync for SmartCardProvisioning {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardReader, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardReader {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<SmartCardReaderKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetStatusAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardReaderStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatusAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllCardsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCard>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllCardsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CardAdded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardAdded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCardAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCardAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CardRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CardRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCardRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCardRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorWithKind(kind: SmartCardReaderKind) -> windows_core::Result<windows_core::HSTRING> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorWithKind)(windows_core::Interface::as_raw(this), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SmartCardReader>> {
        Self::ISmartCardReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardReaderStatics<R, F: FnOnce(&ISmartCardReaderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardReader, ISmartCardReaderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmartCardReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardReader>();
}
unsafe impl windows_core::Interface for SmartCardReader {
    type Vtable = ISmartCardReader_Vtbl;
    const IID: windows_core::GUID = <ISmartCardReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardReader {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardReader";
}
unsafe impl Send for SmartCardReader {}
unsafe impl Sync for SmartCardReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SmartCardTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SmartCardTriggerDetails {
    pub fn TriggerType(&self) -> windows_core::Result<SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SourceAppletId(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceAppletId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TriggerData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Emulator(&self) -> windows_core::Result<SmartCardEmulator> {
        let this = &windows_core::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emulator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryLaunchCurrentAppAsync(&self, arguments: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryLaunchCurrentAppAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(arguments), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryLaunchCurrentAppWithBehaviorAsync(&self, arguments: &windows_core::HSTRING, behavior: SmartCardLaunchBehavior) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<ISmartCardTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryLaunchCurrentAppWithBehaviorAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(arguments), behavior, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SmartCard(&self) -> windows_core::Result<SmartCard> {
        let this = &windows_core::Interface::cast::<ISmartCardTriggerDetails3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartCard)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SmartCardTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardTriggerDetails>();
}
unsafe impl windows_core::Interface for SmartCardTriggerDetails {
    type Vtable = ISmartCardTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <ISmartCardTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardTriggerDetails {
    const NAME: &'static str = "Windows.Devices.SmartCards.SmartCardTriggerDetails";
}
unsafe impl Send for SmartCardTriggerDetails {}
unsafe impl Sync for SmartCardTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardActivationPolicyChangeResult(pub i32);
impl SmartCardActivationPolicyChangeResult {
    pub const Denied: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardActivationPolicyChangeResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardActivationPolicyChangeResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardActivationPolicyChangeResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardActivationPolicyChangeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardActivationPolicyChangeResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardAppletIdGroupActivationPolicy(pub i32);
impl SmartCardAppletIdGroupActivationPolicy {
    pub const Disabled: Self = Self(0i32);
    pub const ForegroundOverride: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl windows_core::TypeKind for SmartCardAppletIdGroupActivationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardAppletIdGroupActivationPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardAppletIdGroupActivationPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardAppletIdGroupActivationPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAppletIdGroupActivationPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardAutomaticResponseStatus(pub i32);
impl SmartCardAutomaticResponseStatus {
    pub const None: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const UnknownError: Self = Self(2i32);
}
impl windows_core::TypeKind for SmartCardAutomaticResponseStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardAutomaticResponseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardAutomaticResponseStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardAutomaticResponseStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardAutomaticResponseStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramAlgorithm(pub i32);
impl SmartCardCryptogramAlgorithm {
    pub const None: Self = Self(0i32);
    pub const CbcMac: Self = Self(1i32);
    pub const Cvc3Umd: Self = Self(2i32);
    pub const DecimalizedMsd: Self = Self(3i32);
    pub const Cvc3MD: Self = Self(4i32);
    pub const Sha1: Self = Self(5i32);
    pub const SignedDynamicApplicationData: Self = Self(6i32);
    pub const RsaPkcs1: Self = Self(7i32);
    pub const Sha256Hmac: Self = Self(8i32);
}
impl windows_core::TypeKind for SmartCardCryptogramAlgorithm {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramAlgorithm").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramGeneratorOperationStatus(pub i32);
impl SmartCardCryptogramGeneratorOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const AuthorizationFailed: Self = Self(1i32);
    pub const AuthorizationCanceled: Self = Self(2i32);
    pub const AuthorizationRequired: Self = Self(3i32);
    pub const CryptogramMaterialPackageStorageKeyExists: Self = Self(4i32);
    pub const NoCryptogramMaterialPackageStorageKey: Self = Self(5i32);
    pub const NoCryptogramMaterialPackage: Self = Self(6i32);
    pub const UnsupportedCryptogramMaterialPackage: Self = Self(7i32);
    pub const UnknownCryptogramMaterialName: Self = Self(8i32);
    pub const InvalidCryptogramMaterialUsage: Self = Self(9i32);
    pub const ApduResponseNotSent: Self = Self(10i32);
    pub const OtherError: Self = Self(11i32);
    pub const ValidationFailed: Self = Self(12i32);
    pub const NotSupported: Self = Self(13i32);
}
impl windows_core::TypeKind for SmartCardCryptogramGeneratorOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramGeneratorOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramGeneratorOperationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramGeneratorOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramGeneratorOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramMaterialPackageConfirmationResponseFormat(pub i32);
impl SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    pub const None: Self = Self(0i32);
    pub const VisaHmac: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageConfirmationResponseFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageConfirmationResponseFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramMaterialPackageFormat(pub i32);
impl SmartCardCryptogramMaterialPackageFormat {
    pub const None: Self = Self(0i32);
    pub const JweRsaPki: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardCryptogramMaterialPackageFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramMaterialPackageFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialPackageFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramMaterialPackageFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialPackageFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramMaterialProtectionMethod(pub i32);
impl SmartCardCryptogramMaterialProtectionMethod {
    pub const None: Self = Self(0i32);
    pub const WhiteBoxing: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardCryptogramMaterialProtectionMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramMaterialProtectionMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialProtectionMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramMaterialProtectionMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialProtectionMethod;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramMaterialType(pub i32);
impl SmartCardCryptogramMaterialType {
    pub const None: Self = Self(0i32);
    pub const StaticDataAuthentication: Self = Self(1i32);
    pub const TripleDes112: Self = Self(2i32);
    pub const Aes: Self = Self(3i32);
    pub const RsaPkcs1: Self = Self(4i32);
}
impl windows_core::TypeKind for SmartCardCryptogramMaterialType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramMaterialType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramMaterialType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramMaterialType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramMaterialType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramPlacementOptions(pub u32);
impl SmartCardCryptogramPlacementOptions {
    pub const None: Self = Self(0u32);
    pub const UnitsAreInNibbles: Self = Self(1u32);
    pub const ChainOutput: Self = Self(2u32);
}
impl windows_core::TypeKind for SmartCardCryptogramPlacementOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramPlacementOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramPlacementOptions").field(&self.0).finish()
    }
}
impl SmartCardCryptogramPlacementOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SmartCardCryptogramPlacementOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SmartCardCryptogramPlacementOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SmartCardCryptogramPlacementOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramPlacementOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramPlacementOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(pub i32);
impl SmartCardCryptogramStorageKeyAlgorithm {
    pub const None: Self = Self(0i32);
    pub const Rsa2048: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardCryptogramStorageKeyAlgorithm {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramStorageKeyAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyAlgorithm").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramStorageKeyAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptogramStorageKeyCapabilities(pub u32);
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const None: Self = Self(0u32);
    pub const HardwareProtection: Self = Self(1u32);
    pub const UnlockPrompt: Self = Self(2u32);
}
impl windows_core::TypeKind for SmartCardCryptogramStorageKeyCapabilities {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptogramStorageKeyCapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptogramStorageKeyCapabilities").field(&self.0).finish()
    }
}
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SmartCardCryptogramStorageKeyCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SmartCardCryptogramStorageKeyCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for SmartCardCryptogramStorageKeyCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptogramStorageKeyCapabilities;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardCryptographicKeyAttestationStatus(pub i32);
impl SmartCardCryptographicKeyAttestationStatus {
    pub const NoAttestation: Self = Self(0i32);
    pub const SoftwareKeyWithoutTpm: Self = Self(1i32);
    pub const SoftwareKeyWithTpm: Self = Self(2i32);
    pub const TpmKeyUnknownAttestationStatus: Self = Self(3i32);
    pub const TpmKeyWithoutAttestationCapability: Self = Self(4i32);
    pub const TpmKeyWithTemporaryAttestationFailure: Self = Self(5i32);
    pub const TpmKeyWithLongTermAttestationFailure: Self = Self(6i32);
    pub const TpmKeyWithAttestation: Self = Self(7i32);
}
impl windows_core::TypeKind for SmartCardCryptographicKeyAttestationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardCryptographicKeyAttestationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardCryptographicKeyAttestationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardCryptographicKeyAttestationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardCryptographicKeyAttestationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardEmulationCategory(pub i32);
impl SmartCardEmulationCategory {
    pub const Other: Self = Self(0i32);
    pub const Payment: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardEmulationCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardEmulationCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardEmulationCategory").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardEmulationCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationCategory;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardEmulationType(pub i32);
impl SmartCardEmulationType {
    pub const Host: Self = Self(0i32);
    pub const Uicc: Self = Self(1i32);
    pub const EmbeddedSE: Self = Self(2i32);
}
impl windows_core::TypeKind for SmartCardEmulationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardEmulationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardEmulationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardEmulationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulationType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardEmulatorConnectionDeactivatedReason(pub i32);
impl SmartCardEmulatorConnectionDeactivatedReason {
    pub const ConnectionLost: Self = Self(0i32);
    pub const ConnectionRedirected: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardEmulatorConnectionDeactivatedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardEmulatorConnectionDeactivatedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionDeactivatedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardEmulatorConnectionDeactivatedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionDeactivatedReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardEmulatorConnectionSource(pub i32);
impl SmartCardEmulatorConnectionSource {
    pub const Unknown: Self = Self(0i32);
    pub const NfcReader: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardEmulatorConnectionSource {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardEmulatorConnectionSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorConnectionSource").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardEmulatorConnectionSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorConnectionSource;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardEmulatorEnablementPolicy(pub i32);
impl SmartCardEmulatorEnablementPolicy {
    pub const Never: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const ScreenOn: Self = Self(2i32);
    pub const ScreenUnlocked: Self = Self(3i32);
}
impl windows_core::TypeKind for SmartCardEmulatorEnablementPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardEmulatorEnablementPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardEmulatorEnablementPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardEmulatorEnablementPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardEmulatorEnablementPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardLaunchBehavior(pub i32);
impl SmartCardLaunchBehavior {
    pub const Default: Self = Self(0i32);
    pub const AboveLock: Self = Self(1i32);
}
impl windows_core::TypeKind for SmartCardLaunchBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardLaunchBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardLaunchBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardLaunchBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardLaunchBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardPinCharacterPolicyOption(pub i32);
impl SmartCardPinCharacterPolicyOption {
    pub const Allow: Self = Self(0i32);
    pub const RequireAtLeastOne: Self = Self(1i32);
    pub const Disallow: Self = Self(2i32);
}
impl windows_core::TypeKind for SmartCardPinCharacterPolicyOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardPinCharacterPolicyOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardPinCharacterPolicyOption").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardPinCharacterPolicyOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardPinCharacterPolicyOption;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardReaderKind(pub i32);
impl SmartCardReaderKind {
    pub const Any: Self = Self(0i32);
    pub const Generic: Self = Self(1i32);
    pub const Tpm: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Uicc: Self = Self(4i32);
    pub const EmbeddedSE: Self = Self(5i32);
}
impl windows_core::TypeKind for SmartCardReaderKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardReaderKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardReaderKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardReaderKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardReaderStatus(pub i32);
impl SmartCardReaderStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Exclusive: Self = Self(2i32);
}
impl windows_core::TypeKind for SmartCardReaderStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardReaderStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardReaderStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardReaderStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardReaderStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardStatus(pub i32);
impl SmartCardStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Shared: Self = Self(2i32);
    pub const Exclusive: Self = Self(3i32);
    pub const Unresponsive: Self = Self(4i32);
}
impl windows_core::TypeKind for SmartCardStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardTriggerType(pub i32);
impl SmartCardTriggerType {
    pub const EmulatorTransaction: Self = Self(0i32);
    pub const EmulatorNearFieldEntry: Self = Self(1i32);
    pub const EmulatorNearFieldExit: Self = Self(2i32);
    pub const EmulatorHostApplicationActivated: Self = Self(3i32);
    pub const EmulatorAppletIdGroupRegistrationChanged: Self = Self(4i32);
    pub const ReaderCardAdded: Self = Self(5i32);
}
impl windows_core::TypeKind for SmartCardTriggerType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardTriggerType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardTriggerType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardTriggerType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardTriggerType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmartCardUnlockPromptingBehavior(pub i32);
impl SmartCardUnlockPromptingBehavior {
    pub const AllowUnlockPrompt: Self = Self(0i32);
    pub const RequireUnlockPrompt: Self = Self(1i32);
    pub const PreventUnlockPrompt: Self = Self(2i32);
}
impl windows_core::TypeKind for SmartCardUnlockPromptingBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmartCardUnlockPromptingBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmartCardUnlockPromptingBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SmartCardUnlockPromptingBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.SmartCards.SmartCardUnlockPromptingBehavior;i4)");
}
windows_core::imp::define_interface!(SmartCardPinResetHandler, SmartCardPinResetHandler_Vtbl, 0x138d5e40_f3bc_4a5c_b41d_4b4ef684e237);
impl SmartCardPinResetHandler {
    pub fn new<F: FnMut(Option<&SmartCardProvisioning>, Option<&SmartCardPinResetRequest>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = SmartCardPinResetHandlerBox::<F> { vtable: &SmartCardPinResetHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, request: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SmartCardProvisioning>,
        P1: windows_core::Param<SmartCardPinResetRequest>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), request.param().abi()).ok() }
    }
}
#[repr(C)]
struct SmartCardPinResetHandlerBox<F: FnMut(Option<&SmartCardProvisioning>, Option<&SmartCardPinResetRequest>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const SmartCardPinResetHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&SmartCardProvisioning>, Option<&SmartCardPinResetRequest>) -> windows_core::Result<()> + Send + 'static> SmartCardPinResetHandlerBox<F> {
    const VTABLE: SmartCardPinResetHandler_Vtbl = SmartCardPinResetHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <SmartCardPinResetHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, request: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&request)).into()
    }
}
impl windows_core::RuntimeType for SmartCardPinResetHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct SmartCardPinResetHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
