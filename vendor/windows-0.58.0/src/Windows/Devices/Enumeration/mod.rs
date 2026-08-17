#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
windows_core::imp::define_interface!(IDeviceAccessChangedEventArgs, IDeviceAccessChangedEventArgs_Vtbl, 0xdeda0bcc_4f9d_4f58_9dba_a9bc800408d5);
impl windows_core::RuntimeType for IDeviceAccessChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccessChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccessStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceAccessChangedEventArgs2, IDeviceAccessChangedEventArgs2_Vtbl, 0x82523262_934b_4b30_a178_adc39f2f2be3);
impl windows_core::RuntimeType for IDeviceAccessChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccessChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceAccessChangedEventArgs3, IDeviceAccessChangedEventArgs3_Vtbl, 0x7580a878_7fd9_5cd7_8560_3c644b9b10db);
impl windows_core::RuntimeType for IDeviceAccessChangedEventArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccessChangedEventArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserPromptRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceAccessInformation, IDeviceAccessInformation_Vtbl, 0x0baa9a73_6de5_4915_8ddd_9a0554a6f545);
impl windows_core::RuntimeType for IDeviceAccessInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccessInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccessChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAccessChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CurrentStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceAccessStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceAccessInformation2, IDeviceAccessInformation2_Vtbl, 0xe2b9dff6_e88f_5d0a_9c1e_d788808df47b);
impl windows_core::RuntimeType for IDeviceAccessInformation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccessInformation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserPromptRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceAccessInformationStatics, IDeviceAccessInformationStatics_Vtbl, 0x574bd3d3_5f30_45cd_8a94_724fe5973084);
impl windows_core::RuntimeType for IDeviceAccessInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceAccessInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromDeviceClassId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromDeviceClass: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceClass, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceConnectionChangeTriggerDetails, IDeviceConnectionChangeTriggerDetails_Vtbl, 0xb8578c0c_bbc1_484b_bffa_7b31dcc200b2);
impl windows_core::RuntimeType for IDeviceConnectionChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceConnectionChangeTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceDisconnectButtonClickedEventArgs, IDeviceDisconnectButtonClickedEventArgs_Vtbl, 0x8e44b56d_f902_4a00_b536_f37992e6a2a7);
impl windows_core::RuntimeType for IDeviceDisconnectButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceDisconnectButtonClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceEnumerationSettings, IDeviceEnumerationSettings_Vtbl, 0xf7710f66_9ff3_41c8_85eb_87f81148a30f);
impl core::ops::Deref for IDeviceEnumerationSettings {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDeviceEnumerationSettings, windows_core::IUnknown, windows_core::IInspectable);
impl IDeviceEnumerationSettings {}
impl windows_core::RuntimeType for IDeviceEnumerationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceEnumerationSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDeviceInformation, IDeviceInformation_Vtbl, 0xaba0fb95_4398_489d_8e44_e6130927011f);
impl windows_core::RuntimeType for IDeviceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EnclosureLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetThumbnailAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetThumbnailAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetGlyphThumbnailAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetGlyphThumbnailAsync: usize,
}
windows_core::imp::define_interface!(IDeviceInformation2, IDeviceInformation2_Vtbl, 0xf156a638_7997_48d9_a10c_269d46533f48);
impl windows_core::RuntimeType for IDeviceInformation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceInformationKind) -> windows_core::HRESULT,
    pub Pairing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceInformationCustomPairing, IDeviceInformationCustomPairing_Vtbl, 0x85138c02_4ee6_4914_8370_107a39144c0e);
impl windows_core::RuntimeType for IDeviceInformationCustomPairing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationCustomPairing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, DevicePairingKinds, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, DevicePairingKinds, DevicePairingProtectionLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, DevicePairingKinds, DevicePairingProtectionLevel, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PairingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePairingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceInformationCustomPairing2, IDeviceInformationCustomPairing2_Vtbl, 0x0ebda662_e696_5fa9_8f72_80cfebcd851d);
impl windows_core::RuntimeType for IDeviceInformationCustomPairing2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationCustomPairing2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddPairingSetMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PairingSetMembersRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePairingSetMembersRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceInformationPairing, IDeviceInformationPairing_Vtbl, 0x2c4769f5_f684_40d5_8469_e8dbaab70485);
impl windows_core::RuntimeType for IDeviceInformationPairing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationPairing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPaired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanPair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, DevicePairingProtectionLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceInformationPairing2, IDeviceInformationPairing2_Vtbl, 0xf68612fd_0aee_4328_85cc_1c742bb1790d);
impl windows_core::RuntimeType for IDeviceInformationPairing2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationPairing2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DevicePairingProtectionLevel) -> windows_core::HRESULT,
    pub Custom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, DevicePairingProtectionLevel, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnpairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceInformationPairingStatics, IDeviceInformationPairingStatics_Vtbl, 0xe915c408_36d4_49a1_bf13_514173799b6b);
impl windows_core::RuntimeType for IDeviceInformationPairingStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationPairingStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryRegisterForAllInboundPairingRequests: unsafe extern "system" fn(*mut core::ffi::c_void, DevicePairingKinds, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceInformationPairingStatics2, IDeviceInformationPairingStatics2_Vtbl, 0x04de5372_b7b7_476b_a74f_c5836a704d98);
impl windows_core::RuntimeType for IDeviceInformationPairingStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationPairingStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryRegisterForAllInboundPairingRequestsWithProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, DevicePairingKinds, DevicePairingProtectionLevel, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceInformationStatics, IDeviceInformationStatics_Vtbl, 0xc17f100e_3a46_4a78_8013_769dc9b97390);
impl windows_core::RuntimeType for IDeviceInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncAdditionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncDeviceClass: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceClass, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncDeviceClass: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilter: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilterAndAdditionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilterAndAdditionalProperties: usize,
    pub CreateWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWatcherDeviceClass: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceClass, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWatcherAqsFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherAqsFilterAndAdditionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherAqsFilterAndAdditionalProperties: usize,
}
windows_core::imp::define_interface!(IDeviceInformationStatics2, IDeviceInformationStatics2_Vtbl, 0x493b4f34_a84f_45fd_9167_15d1cb1bd1f9);
impl windows_core::RuntimeType for IDeviceInformationStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAqsFilterFromDeviceClass: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceClass, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncWithKindAndAdditionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, DeviceInformationKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncWithKindAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, DeviceInformationKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncWithKindAqsFilterAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, DeviceInformationKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithKindAqsFilterAndAdditionalProperties: usize,
}
windows_core::imp::define_interface!(IDeviceInformationStatics3, IDeviceInformationStatics3_Vtbl, 0x25f06279_9364_5a6c_8a54_5d4a6d3d922a);
impl windows_core::RuntimeType for IDeviceInformationStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, DeviceInformationKind, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, DeviceInformationKind, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, DeviceInformationKind, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings: usize,
}
windows_core::imp::define_interface!(IDeviceInformationUpdate, IDeviceInformationUpdate_Vtbl, 0x8f315305_d972_44b7_a37e_9e822c78213b);
impl windows_core::RuntimeType for IDeviceInformationUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IDeviceInformationUpdate2, IDeviceInformationUpdate2_Vtbl, 0x5d9d148c_a873_485e_baa6_aa620788e3cc);
impl windows_core::RuntimeType for IDeviceInformationUpdate2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceInformationUpdate2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceInformationKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePairingRequestedEventArgs, IDevicePairingRequestedEventArgs_Vtbl, 0xf717fc56_de6b_487f_8376_0180aca69963);
impl windows_core::RuntimeType for IDevicePairingRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePairingRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PairingKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DevicePairingKinds) -> windows_core::HRESULT,
    pub Pin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptWithPin: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePairingRequestedEventArgs2, IDevicePairingRequestedEventArgs2_Vtbl, 0xc83752d9_e4d3_4db0_a360_a105e437dbdc);
impl windows_core::RuntimeType for IDevicePairingRequestedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePairingRequestedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub AcceptWithPasswordCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    AcceptWithPasswordCredential: usize,
}
windows_core::imp::define_interface!(IDevicePairingRequestedEventArgs3, IDevicePairingRequestedEventArgs3_Vtbl, 0x195e5a38_43dc_562f_babe_efc8b110088b);
impl windows_core::RuntimeType for IDevicePairingRequestedEventArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePairingRequestedEventArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcceptWithAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePairingResult, IDevicePairingResult_Vtbl, 0x072b02bf_dd95_4025_9b37_de51adba37b7);
impl windows_core::RuntimeType for IDevicePairingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePairingResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DevicePairingResultStatus) -> windows_core::HRESULT,
    pub ProtectionLevelUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DevicePairingProtectionLevel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePairingSetMembersRequestedEventArgs, IDevicePairingSetMembersRequestedEventArgs_Vtbl, 0x7fb42cff_ecac_5012_8d7d_a1894680a349);
impl windows_core::RuntimeType for IDevicePairingSetMembersRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePairingSetMembersRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DevicePairingAddPairingSetMemberStatus) -> windows_core::HRESULT,
    pub ParentDeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PairingSetMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PairingSetMembers: usize,
}
windows_core::imp::define_interface!(IDevicePairingSettings, IDevicePairingSettings_Vtbl, 0x482cb27c_83bb_420e_be51_6602b222de54);
impl core::ops::Deref for IDevicePairingSettings {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDevicePairingSettings, windows_core::IUnknown, windows_core::IInspectable);
impl IDevicePairingSettings {}
impl windows_core::RuntimeType for IDevicePairingSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePairingSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDevicePicker, IDevicePicker_Vtbl, 0x84997aa2_034a_4440_8813_7d0bd479bf5a);
impl windows_core::RuntimeType for IDevicePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Appearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestedProperties: usize,
    pub DeviceSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDeviceSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DisconnectButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisconnectButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DevicePickerDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDevicePickerDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowWithPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowWithPlacement: usize,
    pub PickSingleDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub PickSingleDeviceAsyncWithPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PickSingleDeviceAsyncWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, DevicePickerDisplayStatusOptions) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePickerAppearance, IDevicePickerAppearance_Vtbl, 0xe69a12c6_e627_4ed8_9b6c_460af445e56d);
impl windows_core::RuntimeType for IDevicePickerAppearance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePickerAppearance_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub AccentColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    AccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetAccentColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedAccentColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedAccentColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedAccentColor: usize,
}
windows_core::imp::define_interface!(IDevicePickerFilter, IDevicePickerFilter_Vtbl, 0x91db92a2_57cb_48f1_9b59_a59b7a1f02a2);
impl windows_core::RuntimeType for IDevicePickerFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePickerFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceClasses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceSelectors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceSelectors: usize,
}
windows_core::imp::define_interface!(IDeviceSelectedEventArgs, IDeviceSelectedEventArgs_Vtbl, 0x269edade_1d2f_4940_8402_4156b81d3c77);
impl windows_core::RuntimeType for IDeviceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceSelectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceUnpairingResult, IDeviceUnpairingResult_Vtbl, 0x66f44ad3_79d9_444b_92cf_a92ef72571c7);
impl windows_core::RuntimeType for IDeviceUnpairingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceUnpairingResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceUnpairingResultStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceWatcher, IDeviceWatcher_Vtbl, 0xc9eab97d_8f6b_4f96_a9f4_abc814e22271);
impl windows_core::RuntimeType for IDeviceWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceWatcherStatus) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceWatcher2, IDeviceWatcher2_Vtbl, 0xff08456e_ed14_49e9_9a69_8117c54ae971);
impl windows_core::RuntimeType for IDeviceWatcher2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceWatcher2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub GetBackgroundTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections")))]
    GetBackgroundTrigger: usize,
}
windows_core::imp::define_interface!(IDeviceWatcherEvent, IDeviceWatcherEvent_Vtbl, 0x74aa9c0b_1dbd_47fd_b635_3cc556d0ff8b);
impl windows_core::RuntimeType for IDeviceWatcherEvent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceWatcherEvent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeviceWatcherEventKind) -> windows_core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceInformationUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceWatcherTriggerDetails, IDeviceWatcherTriggerDetails_Vtbl, 0x38808119_4cb7_4e57_a56d_776d07cbfef9);
impl windows_core::RuntimeType for IDeviceWatcherTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceWatcherTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceWatcherEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceWatcherEvents: usize,
}
windows_core::imp::define_interface!(IEnclosureLocation, IEnclosureLocation_Vtbl, 0x42340a27_5810_459c_aabb_c65e1f813ecf);
impl windows_core::RuntimeType for IEnclosureLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEnclosureLocation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InDock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub InLid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Panel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Panel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnclosureLocation2, IEnclosureLocation2_Vtbl, 0x2885995b_e07d_485d_8a9e_bdf29aef4f66);
impl windows_core::RuntimeType for IEnclosureLocation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEnclosureLocation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RotationAngleInDegreesClockwise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceAccessChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceAccessChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceAccessChangedEventArgs {
    pub fn Status(&self) -> windows_core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDeviceAccessChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserPromptRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccessChangedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserPromptRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DeviceAccessChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceAccessChangedEventArgs>();
}
unsafe impl windows_core::Interface for DeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDeviceAccessChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessChangedEventArgs";
}
unsafe impl Send for DeviceAccessChangedEventArgs {}
unsafe impl Sync for DeviceAccessChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceAccessInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceAccessInformation, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceAccessInformation {
    pub fn AccessChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAccessChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAccessChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CurrentStatus(&self) -> windows_core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UserPromptRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeviceAccessInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserPromptRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateFromId(deviceid: &windows_core::HSTRING) -> windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromDeviceClassId(deviceclassid: windows_core::GUID) -> windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromDeviceClassId)(windows_core::Interface::as_raw(this), deviceclassid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromDeviceClass(deviceclass: DeviceClass) -> windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromDeviceClass)(windows_core::Interface::as_raw(this), deviceclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeviceAccessInformationStatics<R, F: FnOnce(&IDeviceAccessInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceAccessInformation, IDeviceAccessInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeviceAccessInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceAccessInformation>();
}
unsafe impl windows_core::Interface for DeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_Vtbl;
    const IID: windows_core::GUID = <IDeviceAccessInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceAccessInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessInformation";
}
unsafe impl Send for DeviceAccessInformation {}
unsafe impl Sync for DeviceAccessInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceConnectionChangeTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceConnectionChangeTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceConnectionChangeTriggerDetails {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceConnectionChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceConnectionChangeTriggerDetails>();
}
unsafe impl windows_core::Interface for DeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IDeviceConnectionChangeTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceConnectionChangeTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails";
}
unsafe impl Send for DeviceConnectionChangeTriggerDetails {}
unsafe impl Sync for DeviceConnectionChangeTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceDisconnectButtonClickedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceDisconnectButtonClickedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceDisconnectButtonClickedEventArgs {
    pub fn Device(&self) -> windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceDisconnectButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceDisconnectButtonClickedEventArgs>();
}
unsafe impl windows_core::Interface for DeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDeviceDisconnectButtonClickedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs";
}
unsafe impl Send for DeviceDisconnectButtonClickedEventArgs {}
unsafe impl Sync for DeviceDisconnectButtonClickedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceInformation, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceInformation {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDefault(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDefault)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnclosureLocation(&self) -> windows_core::Result<EnclosureLocation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnclosureLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Update<P0>(&self, updateinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DeviceInformationUpdate>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), updateinfo.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetThumbnailAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetGlyphThumbnailAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGlyphThumbnailAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<DeviceInformationKind> {
        let this = &windows_core::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pairing(&self) -> windows_core::Result<DeviceInformationPairing> {
        let this = &windows_core::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pairing)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsyncAdditionalProperties<P0>(deviceid: &windows_core::HSTRING, additionalproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIdAsyncAdditionalProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), additionalproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncDeviceClass(deviceclass: DeviceClass) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsyncDeviceClass)(windows_core::Interface::as_raw(this), deviceclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncAqsFilter(aqsfilter: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsyncAqsFilter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncAqsFilterAndAdditionalProperties<P0>(aqsfilter: &windows_core::HSTRING, additionalproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsyncAqsFilterAndAdditionalProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), additionalproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcher() -> windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcherDeviceClass(deviceclass: DeviceClass) -> windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherDeviceClass)(windows_core::Interface::as_raw(this), deviceclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWatcherAqsFilter(aqsfilter: &windows_core::HSTRING) -> windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherAqsFilter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherAqsFilterAndAdditionalProperties<P0>(aqsfilter: &windows_core::HSTRING, additionalproperties: P0) -> windows_core::Result<DeviceWatcher>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherAqsFilterAndAdditionalProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), additionalproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAqsFilterFromDeviceClass(deviceclass: DeviceClass) -> windows_core::Result<windows_core::HSTRING> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAqsFilterFromDeviceClass)(windows_core::Interface::as_raw(this), deviceclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsyncWithKindAndAdditionalProperties<P0>(deviceid: &windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIdAsyncWithKindAndAdditionalProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), additionalproperties.param().abi(), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties<P0>(aqsfilter: &windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsyncWithKindAqsFilterAndAdditionalProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), additionalproperties.param().abi(), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithKindAqsFilterAndAdditionalProperties<P0>(aqsfilter: &windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind) -> windows_core::Result<DeviceWatcher>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherWithKindAqsFilterAndAdditionalProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), additionalproperties.param().abi(), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings<P0, P1>(deviceid: &windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind, settings: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<IDeviceEnumerationSettings>,
    {
        Self::IDeviceInformationStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIdAsyncWithAdditionalPropertiesKindAndSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), additionalproperties.param().abi(), kind, settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings<P0, P1>(aqsfilter: &windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind, settings: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<IDeviceEnumerationSettings>,
    {
        Self::IDeviceInformationStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsyncWithAqsFilterAdditionalPropertiesKindAndSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), additionalproperties.param().abi(), kind, settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings<P0, P1>(aqsfilter: &windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind, settings: P1) -> windows_core::Result<DeviceWatcher>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<IDeviceEnumerationSettings>,
    {
        Self::IDeviceInformationStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcherWithAqsFilterAdditionalPropertiesKindAndSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(aqsfilter), additionalproperties.param().abi(), kind, settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeviceInformationStatics<R, F: FnOnce(&IDeviceInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceInformation, IDeviceInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDeviceInformationStatics2<R, F: FnOnce(&IDeviceInformationStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceInformation, IDeviceInformationStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDeviceInformationStatics3<R, F: FnOnce(&IDeviceInformationStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceInformation, IDeviceInformationStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeviceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceInformation>();
}
unsafe impl windows_core::Interface for DeviceInformation {
    type Vtable = IDeviceInformation_Vtbl;
    const IID: windows_core::GUID = <IDeviceInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformation";
}
unsafe impl Send for DeviceInformation {}
unsafe impl Sync for DeviceInformation {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceInformationCollection(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(DeviceInformationCollection, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(DeviceInformationCollection, super::super::Foundation::Collections::IIterable::<DeviceInformation>, super::super::Foundation::Collections::IVectorView::<DeviceInformation>);
#[cfg(feature = "Foundation_Collections")]
impl DeviceInformationCollection {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<DeviceInformation>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<DeviceInformation>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<DeviceInformation> {
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
        P0: windows_core::Param<DeviceInformation>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<DeviceInformation>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for DeviceInformationCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::Collections::IVectorView<DeviceInformation>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for DeviceInformationCollection {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<DeviceInformation>;
    const IID: windows_core::GUID = <super::super::Foundation::Collections::IVectorView<DeviceInformation> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for DeviceInformationCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for DeviceInformationCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceInformationCustomPairing(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceInformationCustomPairing, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceInformationCustomPairing {
    pub fn PairAsync(&self, pairingkindssupported: DevicePairingKinds) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairAsync)(windows_core::Interface::as_raw(this), pairingkindssupported, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PairWithProtectionLevelAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairWithProtectionLevelAsync)(windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PairWithProtectionLevelAndSettingsAsync<P0>(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>
    where
        P0: windows_core::Param<IDevicePairingSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairWithProtectionLevelAndSettingsAsync)(windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, devicepairingsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PairingRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairingRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePairingRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePairingRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AddPairingSetMember<P0>(&self, device: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DeviceInformation>,
    {
        let this = &windows_core::Interface::cast::<IDeviceInformationCustomPairing2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddPairingSetMember)(windows_core::Interface::as_raw(this), device.param().abi()).ok() }
    }
    pub fn PairingSetMembersRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingSetMembersRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IDeviceInformationCustomPairing2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairingSetMembersRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePairingSetMembersRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDeviceInformationCustomPairing2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePairingSetMembersRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for DeviceInformationCustomPairing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceInformationCustomPairing>();
}
unsafe impl windows_core::Interface for DeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_Vtbl;
    const IID: windows_core::GUID = <IDeviceInformationCustomPairing as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceInformationCustomPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCustomPairing";
}
unsafe impl Send for DeviceInformationCustomPairing {}
unsafe impl Sync for DeviceInformationCustomPairing {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceInformationPairing(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceInformationPairing, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceInformationPairing {
    pub fn IsPaired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPaired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanPair(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPair)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PairAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PairWithProtectionLevelAsync(&self, minprotectionlevel: DevicePairingProtectionLevel) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairWithProtectionLevelAsync)(windows_core::Interface::as_raw(this), minprotectionlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProtectionLevel(&self) -> windows_core::Result<DevicePairingProtectionLevel> {
        let this = &windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Custom(&self) -> windows_core::Result<DeviceInformationCustomPairing> {
        let this = &windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Custom)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PairWithProtectionLevelAndSettingsAsync<P0>(&self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>
    where
        P0: windows_core::Param<IDevicePairingSettings>,
    {
        let this = &windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairWithProtectionLevelAndSettingsAsync)(windows_core::Interface::as_raw(this), minprotectionlevel, devicepairingsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnpairAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>> {
        let this = &windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnpairAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryRegisterForAllInboundPairingRequests(pairingkindssupported: DevicePairingKinds) -> windows_core::Result<bool> {
        Self::IDeviceInformationPairingStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRegisterForAllInboundPairingRequests)(windows_core::Interface::as_raw(this), pairingkindssupported, &mut result__).map(|| result__)
        })
    }
    pub fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> windows_core::Result<bool> {
        Self::IDeviceInformationPairingStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRegisterForAllInboundPairingRequestsWithProtectionLevel)(windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceInformationPairingStatics<R, F: FnOnce(&IDeviceInformationPairingStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDeviceInformationPairingStatics2<R, F: FnOnce(&IDeviceInformationPairingStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeviceInformationPairing {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceInformationPairing>();
}
unsafe impl windows_core::Interface for DeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_Vtbl;
    const IID: windows_core::GUID = <IDeviceInformationPairing as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceInformationPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationPairing";
}
unsafe impl Send for DeviceInformationPairing {}
unsafe impl Sync for DeviceInformationPairing {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceInformationUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceInformationUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceInformationUpdate {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<DeviceInformationKind> {
        let this = &windows_core::Interface::cast::<IDeviceInformationUpdate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DeviceInformationUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceInformationUpdate>();
}
unsafe impl windows_core::Interface for DeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_Vtbl;
    const IID: windows_core::GUID = <IDeviceInformationUpdate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceInformationUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationUpdate";
}
unsafe impl Send for DeviceInformationUpdate {}
unsafe impl Sync for DeviceInformationUpdate {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePairingRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePairingRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePairingRequestedEventArgs {
    pub fn DeviceInformation(&self) -> windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PairingKind(&self) -> windows_core::Result<DevicePairingKinds> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairingKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pin(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pin)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Accept(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Accept)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptWithPin(&self, pin: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcceptWithPin)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pin)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn AcceptWithPasswordCredential<P0>(&self, passwordcredential: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IDevicePairingRequestedEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AcceptWithPasswordCredential)(windows_core::Interface::as_raw(this), passwordcredential.param().abi()).ok() }
    }
    pub fn AcceptWithAddress(&self, address: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDevicePairingRequestedEventArgs3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AcceptWithAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(address)).ok() }
    }
}
impl windows_core::RuntimeType for DevicePairingRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePairingRequestedEventArgs>();
}
unsafe impl windows_core::Interface for DevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDevicePairingRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePairingRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingRequestedEventArgs";
}
unsafe impl Send for DevicePairingRequestedEventArgs {}
unsafe impl Sync for DevicePairingRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePairingResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePairingResult, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePairingResult {
    pub fn Status(&self) -> windows_core::Result<DevicePairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtectionLevelUsed(&self) -> windows_core::Result<DevicePairingProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionLevelUsed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DevicePairingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePairingResult>();
}
unsafe impl windows_core::Interface for DevicePairingResult {
    type Vtable = IDevicePairingResult_Vtbl;
    const IID: windows_core::GUID = <IDevicePairingResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingResult";
}
unsafe impl Send for DevicePairingResult {}
unsafe impl Sync for DevicePairingResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePairingSetMembersRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePairingSetMembersRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePairingSetMembersRequestedEventArgs {
    pub fn Status(&self) -> windows_core::Result<DevicePairingAddPairingSetMemberStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ParentDeviceInformation(&self) -> windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentDeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PairingSetMembers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PairingSetMembers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DevicePairingSetMembersRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePairingSetMembersRequestedEventArgs>();
}
unsafe impl windows_core::Interface for DevicePairingSetMembersRequestedEventArgs {
    type Vtable = IDevicePairingSetMembersRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDevicePairingSetMembersRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePairingSetMembersRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingSetMembersRequestedEventArgs";
}
unsafe impl Send for DevicePairingSetMembersRequestedEventArgs {}
unsafe impl Sync for DevicePairingSetMembersRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePicker, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePicker {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DevicePicker, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Filter(&self) -> windows_core::Result<DevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Filter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Appearance(&self) -> windows_core::Result<DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appearance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceSelected<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceSelected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeviceSelected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDeviceSelected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DisconnectButtonClicked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisconnectButtonClicked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisconnectButtonClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisconnectButtonClicked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DevicePickerDismissed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DevicePicker, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DevicePickerDismissed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDevicePickerDismissed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDevicePickerDismissed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Show(&self, selection: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Show)(windows_core::Interface::as_raw(this), selection).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowWithPlacement(&self, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowWithPlacement)(windows_core::Interface::as_raw(this), selection, placement).ok() }
    }
    pub fn PickSingleDeviceAsync(&self, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleDeviceAsync)(windows_core::Interface::as_raw(this), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn PickSingleDeviceAsyncWithPlacement(&self, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleDeviceAsyncWithPlacement)(windows_core::Interface::as_raw(this), selection, placement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Hide(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Hide)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetDisplayStatus<P0>(&self, device: P0, status: &windows_core::HSTRING, options: DevicePickerDisplayStatusOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DeviceInformation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayStatus)(windows_core::Interface::as_raw(this), device.param().abi(), core::mem::transmute_copy(status), options).ok() }
    }
}
impl windows_core::RuntimeType for DevicePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePicker>();
}
unsafe impl windows_core::Interface for DevicePicker {
    type Vtable = IDevicePicker_Vtbl;
    const IID: windows_core::GUID = <IDevicePicker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePicker {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePicker";
}
unsafe impl Send for DevicePicker {}
unsafe impl Sync for DevicePicker {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePickerAppearance(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePickerAppearance, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePickerAppearance {
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn ForegroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetForegroundColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForegroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn AccentColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccentColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetAccentColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAccentColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SelectedForegroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetSelectedForegroundColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedForegroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SelectedBackgroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedBackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetSelectedBackgroundColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SelectedAccentColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedAccentColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetSelectedAccentColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedAccentColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for DevicePickerAppearance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePickerAppearance>();
}
unsafe impl windows_core::Interface for DevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_Vtbl;
    const IID: windows_core::GUID = <IDevicePickerAppearance as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePickerAppearance {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerAppearance";
}
unsafe impl Send for DevicePickerAppearance {}
unsafe impl Sync for DevicePickerAppearance {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePickerFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePickerFilter, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePickerFilter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDeviceClasses(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<DeviceClass>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedDeviceClasses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDeviceSelectors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedDeviceSelectors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DevicePickerFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePickerFilter>();
}
unsafe impl windows_core::Interface for DevicePickerFilter {
    type Vtable = IDevicePickerFilter_Vtbl;
    const IID: windows_core::GUID = <IDevicePickerFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePickerFilter {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerFilter";
}
unsafe impl Send for DevicePickerFilter {}
unsafe impl Sync for DevicePickerFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceSelectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceSelectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceSelectedEventArgs {
    pub fn SelectedDevice(&self) -> windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceSelectedEventArgs>();
}
unsafe impl windows_core::Interface for DeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDeviceSelectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceSelectedEventArgs";
}
unsafe impl Send for DeviceSelectedEventArgs {}
unsafe impl Sync for DeviceSelectedEventArgs {}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceThumbnail(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(DeviceThumbnail, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(DeviceThumbnail, super::super::Foundation::IClosable, super::super::Storage::Streams::IContentTypeProvider, super::super::Storage::Streams::IInputStream, super::super::Storage::Streams::IOutputStream, super::super::Storage::Streams::IRandomAccessStream, super::super::Storage::Streams::IRandomAccessStreamWithContentType);
#[cfg(feature = "Storage_Streams")]
impl DeviceThumbnail {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn FlushAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for DeviceThumbnail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Storage::Streams::IRandomAccessStreamWithContentType>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for DeviceThumbnail {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamWithContentType_Vtbl;
    const IID: windows_core::GUID = <super::super::Storage::Streams::IRandomAccessStreamWithContentType as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for DeviceThumbnail {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceThumbnail";
}
#[cfg(feature = "Storage_Streams")]
unsafe impl Send for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
unsafe impl Sync for DeviceThumbnail {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceUnpairingResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceUnpairingResult, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceUnpairingResult {
    pub fn Status(&self) -> windows_core::Result<DeviceUnpairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DeviceUnpairingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceUnpairingResult>();
}
unsafe impl windows_core::Interface for DeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_Vtbl;
    const IID: windows_core::GUID = <IDeviceUnpairingResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceUnpairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceUnpairingResult";
}
unsafe impl Send for DeviceUnpairingResult {}
unsafe impl Sync for DeviceUnpairingResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceWatcher {
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Added)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Updated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Removed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnumerationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DeviceWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub fn GetBackgroundTrigger<P0>(&self, requestedeventkinds: P0) -> windows_core::Result<super::super::ApplicationModel::Background::DeviceWatcherTrigger>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind>>,
    {
        let this = &windows_core::Interface::cast::<IDeviceWatcher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBackgroundTrigger)(windows_core::Interface::as_raw(this), requestedeventkinds.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceWatcher>();
}
unsafe impl windows_core::Interface for DeviceWatcher {
    type Vtable = IDeviceWatcher_Vtbl;
    const IID: windows_core::GUID = <IDeviceWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcher";
}
unsafe impl Send for DeviceWatcher {}
unsafe impl Sync for DeviceWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceWatcherEvent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceWatcherEvent, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceWatcherEvent {
    pub fn Kind(&self) -> windows_core::Result<DeviceWatcherEventKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceInformation(&self) -> windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceInformationUpdate(&self) -> windows_core::Result<DeviceInformationUpdate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformationUpdate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceWatcherEvent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceWatcherEvent>();
}
unsafe impl windows_core::Interface for DeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_Vtbl;
    const IID: windows_core::GUID = <IDeviceWatcherEvent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceWatcherEvent {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherEvent";
}
unsafe impl Send for DeviceWatcherEvent {}
unsafe impl Sync for DeviceWatcherEvent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DeviceWatcherTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceWatcherTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl DeviceWatcherTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceWatcherEvents(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceWatcherEvents)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceWatcherTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceWatcherTriggerDetails>();
}
unsafe impl windows_core::Interface for DeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IDeviceWatcherTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherTriggerDetails";
}
unsafe impl Send for DeviceWatcherTriggerDetails {}
unsafe impl Sync for DeviceWatcherTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EnclosureLocation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EnclosureLocation, windows_core::IUnknown, windows_core::IInspectable);
impl EnclosureLocation {
    pub fn InDock(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InDock)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InLid(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InLid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Panel(&self) -> windows_core::Result<Panel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Panel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RotationAngleInDegreesClockwise(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IEnclosureLocation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAngleInDegreesClockwise)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for EnclosureLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEnclosureLocation>();
}
unsafe impl windows_core::Interface for EnclosureLocation {
    type Vtable = IEnclosureLocation_Vtbl;
    const IID: windows_core::GUID = <IEnclosureLocation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EnclosureLocation {
    const NAME: &'static str = "Windows.Devices.Enumeration.EnclosureLocation";
}
unsafe impl Send for EnclosureLocation {}
unsafe impl Sync for EnclosureLocation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceAccessStatus(pub i32);
impl DeviceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl windows_core::TypeKind for DeviceAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceAccessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceAccessStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceClass(pub i32);
impl DeviceClass {
    pub const All: Self = Self(0i32);
    pub const AudioCapture: Self = Self(1i32);
    pub const AudioRender: Self = Self(2i32);
    pub const PortableStorageDevice: Self = Self(3i32);
    pub const VideoCapture: Self = Self(4i32);
    pub const ImageScanner: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
}
impl windows_core::TypeKind for DeviceClass {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceClass").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceClass;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceInformationKind(pub i32);
impl DeviceInformationKind {
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
    pub const AssociationEndpointProtocol: Self = Self(9i32);
}
impl windows_core::TypeKind for DeviceInformationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceInformationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceInformationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceInformationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceInformationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DevicePairingAddPairingSetMemberStatus(pub i32);
impl DevicePairingAddPairingSetMemberStatus {
    pub const AddedToSet: Self = Self(0i32);
    pub const CouldNotBeAddedToSet: Self = Self(1i32);
    pub const SetDiscoveryNotAttemptedByProtocol: Self = Self(2i32);
    pub const SetDiscoveryCompletedByProtocol: Self = Self(3i32);
    pub const SetDiscoveryPartiallyCompletedByProtocol: Self = Self(4i32);
    pub const Failed: Self = Self(5i32);
}
impl windows_core::TypeKind for DevicePairingAddPairingSetMemberStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DevicePairingAddPairingSetMemberStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DevicePairingAddPairingSetMemberStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DevicePairingAddPairingSetMemberStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingAddPairingSetMemberStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DevicePairingKinds(pub u32);
impl DevicePairingKinds {
    pub const None: Self = Self(0u32);
    pub const ConfirmOnly: Self = Self(1u32);
    pub const DisplayPin: Self = Self(2u32);
    pub const ProvidePin: Self = Self(4u32);
    pub const ConfirmPinMatch: Self = Self(8u32);
    pub const ProvidePasswordCredential: Self = Self(16u32);
    pub const ProvideAddress: Self = Self(32u32);
}
impl windows_core::TypeKind for DevicePairingKinds {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DevicePairingKinds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DevicePairingKinds").field(&self.0).finish()
    }
}
impl DevicePairingKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DevicePairingKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DevicePairingKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DevicePairingKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DevicePairingKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DevicePairingKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DevicePairingKinds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingKinds;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DevicePairingProtectionLevel(pub i32);
impl DevicePairingProtectionLevel {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Encryption: Self = Self(2i32);
    pub const EncryptionAndAuthentication: Self = Self(3i32);
}
impl windows_core::TypeKind for DevicePairingProtectionLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DevicePairingProtectionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DevicePairingProtectionLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DevicePairingProtectionLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingProtectionLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DevicePairingResultStatus(pub i32);
impl DevicePairingResultStatus {
    pub const Paired: Self = Self(0i32);
    pub const NotReadyToPair: Self = Self(1i32);
    pub const NotPaired: Self = Self(2i32);
    pub const AlreadyPaired: Self = Self(3i32);
    pub const ConnectionRejected: Self = Self(4i32);
    pub const TooManyConnections: Self = Self(5i32);
    pub const HardwareFailure: Self = Self(6i32);
    pub const AuthenticationTimeout: Self = Self(7i32);
    pub const AuthenticationNotAllowed: Self = Self(8i32);
    pub const AuthenticationFailure: Self = Self(9i32);
    pub const NoSupportedProfiles: Self = Self(10i32);
    pub const ProtectionLevelCouldNotBeMet: Self = Self(11i32);
    pub const AccessDenied: Self = Self(12i32);
    pub const InvalidCeremonyData: Self = Self(13i32);
    pub const PairingCanceled: Self = Self(14i32);
    pub const OperationAlreadyInProgress: Self = Self(15i32);
    pub const RequiredHandlerNotRegistered: Self = Self(16i32);
    pub const RejectedByHandler: Self = Self(17i32);
    pub const RemoteDeviceHasAssociation: Self = Self(18i32);
    pub const Failed: Self = Self(19i32);
}
impl windows_core::TypeKind for DevicePairingResultStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DevicePairingResultStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DevicePairingResultStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DevicePairingResultStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingResultStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DevicePickerDisplayStatusOptions(pub u32);
impl DevicePickerDisplayStatusOptions {
    pub const None: Self = Self(0u32);
    pub const ShowProgress: Self = Self(1u32);
    pub const ShowDisconnectButton: Self = Self(2u32);
    pub const ShowRetryButton: Self = Self(4u32);
}
impl windows_core::TypeKind for DevicePickerDisplayStatusOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DevicePickerDisplayStatusOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DevicePickerDisplayStatusOptions").field(&self.0).finish()
    }
}
impl DevicePickerDisplayStatusOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DevicePickerDisplayStatusOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DevicePickerDisplayStatusOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DevicePickerDisplayStatusOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePickerDisplayStatusOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceUnpairingResultStatus(pub i32);
impl DeviceUnpairingResultStatus {
    pub const Unpaired: Self = Self(0i32);
    pub const AlreadyUnpaired: Self = Self(1i32);
    pub const OperationAlreadyInProgress: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl windows_core::TypeKind for DeviceUnpairingResultStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceUnpairingResultStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceUnpairingResultStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceUnpairingResultStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceUnpairingResultStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceWatcherEventKind(pub i32);
impl DeviceWatcherEventKind {
    pub const Add: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Remove: Self = Self(2i32);
}
impl windows_core::TypeKind for DeviceWatcherEventKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceWatcherEventKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceWatcherEventKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceWatcherEventKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherEventKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceWatcherStatus(pub i32);
impl DeviceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl windows_core::TypeKind for DeviceWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Panel(pub i32);
impl Panel {
    pub const Unknown: Self = Self(0i32);
    pub const Front: Self = Self(1i32);
    pub const Back: Self = Self(2i32);
    pub const Top: Self = Self(3i32);
    pub const Bottom: Self = Self(4i32);
    pub const Left: Self = Self(5i32);
    pub const Right: Self = Self(6i32);
}
impl windows_core::TypeKind for Panel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Panel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Panel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Panel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Panel;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
