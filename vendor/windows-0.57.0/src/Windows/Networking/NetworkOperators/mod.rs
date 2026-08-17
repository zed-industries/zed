windows_core::imp::define_interface!(IESim, IESim_Vtbl, 0x6f6e6e26_f123_437d_8ced_dc1d2bc0c3a9);
impl windows_core::RuntimeType for IESim {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESim_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AvailableMemoryInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Eid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FirmwareVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MobileBroadbandModemDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimState) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProfiles: usize,
    pub DeleteProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DownloadProfileMetadataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProfileChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProfileChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESim2, IESim2_Vtbl, 0xbd4fd0a0_c68f_56eb_b99b_8f34b8100299);
impl windows_core::RuntimeType for IESim2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESim2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Discover: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscoverWithServerAddressAndMatchingId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscoverAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscoverWithServerAddressAndMatchingIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESim3, IESim3_Vtbl, 0xfe1edf45_01b8_5d31_b8d3_d9cbebb2b831);
impl windows_core::RuntimeType for IESim3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESim3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SlotIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimAddedEventArgs, IESimAddedEventArgs_Vtbl, 0x38bd0a58_4d5a_4d08_8da7_e73eff369ddd);
impl windows_core::RuntimeType for IESimAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimAddedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ESim: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimDiscoverEvent, IESimDiscoverEvent_Vtbl, 0xe59ac3e3_39bc_5f6f_9321_0d4a182d261b);
impl windows_core::RuntimeType for IESimDiscoverEvent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimDiscoverEvent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MatchingId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RspServerAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimDiscoverResult, IESimDiscoverResult_Vtbl, 0x56b4bb5e_ab2f_5ac6_b359_dd5a8e237926);
impl windows_core::RuntimeType for IESimDiscoverResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimDiscoverResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Events: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Events: usize,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimDiscoverResultKind) -> windows_core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimDownloadProfileMetadataResult, IESimDownloadProfileMetadataResult_Vtbl, 0xc4234d9e_5ad6_426d_8d00_4434f449afec);
impl windows_core::RuntimeType for IESimDownloadProfileMetadataResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimDownloadProfileMetadataResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimManagerStatics, IESimManagerStatics_Vtbl, 0x0bfa2c0c_df88_4631_bf04_c12e281b3962);
impl windows_core::RuntimeType for IESimManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryCreateESimWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServiceInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimOperationResult, IESimOperationResult_Vtbl, 0xa67b63b1_309b_4e77_9e7e_cd93f1ddc7b9);
impl windows_core::RuntimeType for IESimOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimOperationStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimPolicy, IESimPolicy_Vtbl, 0x41e1b99d_cf7e_4315_882b_6f1e74b0d38f);
impl windows_core::RuntimeType for IESimPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimPolicy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShouldEnableManagingUi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimProfile, IESimProfile_Vtbl, 0xee1e7880_06a9_4027_b4f8_ddb23d7810e0);
impl windows_core::RuntimeType for IESimProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimProfile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimProfileClass) -> windows_core::HRESULT,
    pub Nickname: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimProfileState) -> windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNicknameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimProfileMetadata, IESimProfileMetadata_Vtbl, 0xed25831f_90db_498d_a7b4_ebce807d3c23);
impl windows_core::RuntimeType for IESimProfileMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimProfileMetadata_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsConfirmationCodeRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimProfileMetadataState) -> windows_core::HRESULT,
    pub DenyInstallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfirmInstallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfirmInstallWithConfirmationCodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostponeInstallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimProfilePolicy, IESimProfilePolicy_Vtbl, 0xe6dd0f1d_9c5c_46c5_a289_a948999bf062);
impl windows_core::RuntimeType for IESimProfilePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimProfilePolicy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsManagedByEnterprise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimRemovedEventArgs, IESimRemovedEventArgs_Vtbl, 0xdec5277b_2fd9_4ed9_8376_d9b5e41278a3);
impl windows_core::RuntimeType for IESimRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ESim: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimServiceInfo, IESimServiceInfo_Vtbl, 0xf16aabcf_7f59_4a51_8494_bd89d5ff50ee);
impl windows_core::RuntimeType for IESimServiceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimServiceInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AuthenticationPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimAuthenticationPreference) -> windows_core::HRESULT,
    pub IsESimUiEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimUpdatedEventArgs, IESimUpdatedEventArgs_Vtbl, 0x4c125cec_508d_4b88_83cb_68bef8168d12);
impl windows_core::RuntimeType for IESimUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ESim: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IESimWatcher, IESimWatcher_Vtbl, 0xc1f84ceb_a28d_4fbf_9771_6e31b81ccf22);
impl windows_core::RuntimeType for IESimWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IESimWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ESimWatcherStatus) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFdnAccessManagerStatics, IFdnAccessManagerStatics_Vtbl, 0xf2aa4395_f1e6_4319_aa3e_477ca64b2bdf);
impl windows_core::RuntimeType for IFdnAccessManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFdnAccessManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestUnlockAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHotspotAuthenticationContext, IHotspotAuthenticationContext_Vtbl, 0xe756c791_1003_4de5_83c7_de61d88831d0);
impl windows_core::RuntimeType for IHotspotAuthenticationContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHotspotAuthenticationContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WirelessNetworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    pub RedirectMessageUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub RedirectMessageXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    RedirectMessageXml: usize,
    pub AuthenticationUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IssueCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, bool) -> windows_core::HRESULT,
    pub AbortAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SkipAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TriggerAttentionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHotspotAuthenticationContext2, IHotspotAuthenticationContext2_Vtbl, 0xe756c791_1004_4de5_83c7_de61d88831d0);
impl windows_core::RuntimeType for IHotspotAuthenticationContext2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHotspotAuthenticationContext2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IssueCredentialsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHotspotAuthenticationContextStatics, IHotspotAuthenticationContextStatics_Vtbl, 0xe756c791_1002_4de5_83c7_de61d88831d0);
impl windows_core::RuntimeType for IHotspotAuthenticationContextStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHotspotAuthenticationContextStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetAuthenticationContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHotspotAuthenticationEventDetails, IHotspotAuthenticationEventDetails_Vtbl, 0xe756c791_1001_4de5_83c7_de61d88831d0);
impl windows_core::RuntimeType for IHotspotAuthenticationEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHotspotAuthenticationEventDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EventToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHotspotCredentialsAuthenticationResult, IHotspotCredentialsAuthenticationResult_Vtbl, 0xe756c791_1005_4de5_83c7_de61d88831d0);
impl windows_core::RuntimeType for IHotspotCredentialsAuthenticationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHotspotCredentialsAuthenticationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasNetworkErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ResponseCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HotspotAuthenticationResponseCode) -> windows_core::HRESULT,
    pub LogoffUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub AuthenticationReplyXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    AuthenticationReplyXml: usize,
}
windows_core::imp::define_interface!(IKnownCSimFilePathsStatics, IKnownCSimFilePathsStatics_Vtbl, 0xb458aeed_49f1_4c22_b073_96d511bf9c35);
impl windows_core::RuntimeType for IKnownCSimFilePathsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownCSimFilePathsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
windows_core::imp::define_interface!(IKnownRuimFilePathsStatics, IKnownRuimFilePathsStatics_Vtbl, 0x3883c8b9_ff24_4571_a867_09f960426e14);
impl windows_core::RuntimeType for IKnownRuimFilePathsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownRuimFilePathsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
windows_core::imp::define_interface!(IKnownSimFilePathsStatics, IKnownSimFilePathsStatics_Vtbl, 0x80cd1a63_37a5_43d3_80a3_ccd23e8fecee);
impl windows_core::RuntimeType for IKnownSimFilePathsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownSimFilePathsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOns: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
windows_core::imp::define_interface!(IKnownUSimFilePathsStatics, IKnownUSimFilePathsStatics_Vtbl, 0x7c34e581_1f1b_43f4_9530_8b092d32d71f);
impl windows_core::RuntimeType for IKnownUSimFilePathsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownUSimFilePathsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOpl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOpl: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFPnn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFPnn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandAccount, IMobileBroadbandAccount_Vtbl, 0x36c24ccd_cee2_43e0_a603_ee86a36d6570);
impl windows_core::RuntimeType for IMobileBroadbandAccount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAccount_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ServiceProviderGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ServiceProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CurrentNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandAccount2, IMobileBroadbandAccount2_Vtbl, 0x38f52f1c_1136_4257_959f_b658a352b6d4);
impl windows_core::RuntimeType for IMobileBroadbandAccount2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAccount2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub GetConnectionProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking_Connectivity")))]
    GetConnectionProfiles: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandAccount3, IMobileBroadbandAccount3_Vtbl, 0x092a1e21_9379_4b9b_ad31_d5fee2f748c6);
impl windows_core::RuntimeType for IMobileBroadbandAccount3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAccount3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountExperienceUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandAccountEventArgs, IMobileBroadbandAccountEventArgs_Vtbl, 0x3853c880_77de_4c04_bead_a123b08c9f59);
impl windows_core::RuntimeType for IMobileBroadbandAccountEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAccountEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandAccountStatics, IMobileBroadbandAccountStatics_Vtbl, 0xaa7f4d24_afc1_4fc8_ae9a_a9175310faad);
impl windows_core::RuntimeType for IMobileBroadbandAccountStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAccountStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableNetworkAccountIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableNetworkAccountIds: usize,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandAccountUpdatedEventArgs, IMobileBroadbandAccountUpdatedEventArgs_Vtbl, 0x7bc31d88_a6bd_49e1_80ab_6b91354a57d4);
impl windows_core::RuntimeType for IMobileBroadbandAccountUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAccountUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HasDeviceInformationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNetworkChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandAccountWatcher, IMobileBroadbandAccountWatcher_Vtbl, 0x6bf3335e_23b5_449f_928d_5e0d3e04471d);
impl windows_core::RuntimeType for IMobileBroadbandAccountWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAccountWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAccountAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AccountUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAccountUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AccountRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAccountRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandAccountWatcherStatus) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandAntennaSar, IMobileBroadbandAntennaSar_Vtbl, 0xb9af4b7e_cbf9_4109_90be_5c06bfd513b6);
impl windows_core::RuntimeType for IMobileBroadbandAntennaSar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAntennaSar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AntennaIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SarBackoffIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandAntennaSarFactory, IMobileBroadbandAntennaSarFactory_Vtbl, 0xa91e1716_c04d_4a21_8698_1459dc672c6e);
impl windows_core::RuntimeType for IMobileBroadbandAntennaSarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandAntennaSarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandCellCdma, IMobileBroadbandCellCdma_Vtbl, 0x0601b3b4_411a_4f2e_8287_76f5650c60cd);
impl windows_core::RuntimeType for IMobileBroadbandCellCdma {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellCdma_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BaseStationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BaseStationPNCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BaseStationLatitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BaseStationLongitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BaseStationLastBroadcastGpsTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PilotSignalStrengthInDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandCellGsm, IMobileBroadbandCellGsm_Vtbl, 0xcc917f06_7ee0_47b8_9e1f_c3b48df9df5b);
impl windows_core::RuntimeType for IMobileBroadbandCellGsm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellGsm_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BaseStationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CellId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChannelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocationAreaCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReceivedSignalStrengthInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandCellLte, IMobileBroadbandCellLte_Vtbl, 0x9197c87b_2b78_456d_8b53_aaa25d0af741);
impl windows_core::RuntimeType for IMobileBroadbandCellLte {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellLte_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CellId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChannelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhysicalCellId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrackingAreaCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandCellNR, IMobileBroadbandCellNR_Vtbl, 0xa13f0deb_66fc_4b4b_83a9_a487a3a5a0a6);
impl windows_core::RuntimeType for IMobileBroadbandCellNR {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellNR_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CellId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChannelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhysicalCellId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimingAdvanceInNanoseconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrackingAreaCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandCellTdscdma, IMobileBroadbandCellTdscdma_Vtbl, 0x0eda1655_db0e_4182_8cda_cc419a7bde08);
impl windows_core::RuntimeType for IMobileBroadbandCellTdscdma {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellTdscdma_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CellId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CellParameterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChannelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocationAreaCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathLossInDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandCellUmts, IMobileBroadbandCellUmts_Vtbl, 0x77b4b5ae_49c8_4f15_b285_4c26a7f67215);
impl windows_core::RuntimeType for IMobileBroadbandCellUmts {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellUmts_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CellId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChannelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocationAreaCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathLossInDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrimaryScramblingCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandCellsInfo, IMobileBroadbandCellsInfo_Vtbl, 0x89a9562a_e472_4da5_929c_de61711dd261);
impl windows_core::RuntimeType for IMobileBroadbandCellsInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellsInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsCdma: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsGsm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsLte: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsTdscdma: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsUmts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsUmts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsCdma: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsGsm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsLte: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsTdscdma: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsUmts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsUmts: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandCellsInfo2, IMobileBroadbandCellsInfo2_Vtbl, 0x66205912_b89f_4e12_bbb6_d5cf09a820ca);
impl windows_core::RuntimeType for IMobileBroadbandCellsInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCellsInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsNR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsNR: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsNR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsNR: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandCurrentSlotIndexChangedEventArgs, IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl, 0xf718b184_c370_5fd4_a670_1846cb9bce47);
impl windows_core::RuntimeType for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentSlotIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceInformation, IMobileBroadbandDeviceInformation_Vtbl, 0xe6d08168_e381_4c6e_9be8_fe156969a446);
impl windows_core::RuntimeType for IMobileBroadbandDeviceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkDeviceStatus) -> windows_core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FirmwareInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub CellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Sms::CellularClass) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    CellularClass: usize,
    pub DataClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataClasses) -> windows_core::HRESULT,
    pub CustomDataClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MobileEquipmentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TelephoneNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TelephoneNumbers: usize,
    pub SubscriberId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandDeviceType) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CurrentRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandRadioState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceInformation2, IMobileBroadbandDeviceInformation2_Vtbl, 0x2e467af1_f932_4737_a722_03ba72370cb8);
impl windows_core::RuntimeType for IMobileBroadbandDeviceInformation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PinManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceInformation3, IMobileBroadbandDeviceInformation3_Vtbl, 0xe08bb4bd_5d30_4b5a_92cc_d54df881d49e);
impl windows_core::RuntimeType for IMobileBroadbandDeviceInformation3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SimSpn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SimPnn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SimGid1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceInformation4, IMobileBroadbandDeviceInformation4_Vtbl, 0x263f3152_7b9d_582c_b17c_f80a60b50031);
impl windows_core::RuntimeType for IMobileBroadbandDeviceInformation4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SlotManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceService, IMobileBroadbandDeviceService_Vtbl, 0x22be1a52_bd80_40ac_8e1f_2e07836a3dbd);
impl windows_core::RuntimeType for IMobileBroadbandDeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCommands: usize,
    pub OpenDataSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenCommandSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceServiceCommandResult, IMobileBroadbandDeviceServiceCommandResult_Vtbl, 0xb0f46abb_94d6_44b9_a538_f0810b645389);
impl windows_core::RuntimeType for IMobileBroadbandDeviceServiceCommandResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceCommandResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseData: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceServiceCommandSession, IMobileBroadbandDeviceServiceCommandSession_Vtbl, 0xfc098a45_913b_4914_b6c3_ae6304593e75);
impl windows_core::RuntimeType for IMobileBroadbandDeviceServiceCommandSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceCommandSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SendQueryCommandAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendQueryCommandAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SendSetCommandAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendSetCommandAsync: usize,
    pub CloseSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceServiceDataReceivedEventArgs, IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl, 0xb6aa13de_1380_40e3_8618_73cbca48138c);
impl windows_core::RuntimeType for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceServiceDataSession, IMobileBroadbandDeviceServiceDataSession_Vtbl, 0xdad62333_8bcf_4289_8a37_045c2169486a);
impl windows_core::RuntimeType for IMobileBroadbandDeviceServiceDataSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceDataSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub WriteDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteDataAsync: usize,
    pub CloseSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceServiceInformation, IMobileBroadbandDeviceServiceInformation_Vtbl, 0x53d69b5b_c4ed_45f0_803a_d9417a6d9846);
impl windows_core::RuntimeType for IMobileBroadbandDeviceServiceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsDataReadSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDataWriteSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceServiceTriggerDetails, IMobileBroadbandDeviceServiceTriggerDetails_Vtbl, 0x4a055b70_b9ae_4458_9241_a6a5fbf18a0c);
impl windows_core::RuntimeType for IMobileBroadbandDeviceServiceTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeviceServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandDeviceServiceTriggerDetails2, IMobileBroadbandDeviceServiceTriggerDetails2_Vtbl, 0xd83d5f16_336a_553f_94bb_0cd1a2ff0c81);
impl windows_core::RuntimeType for IMobileBroadbandDeviceServiceTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EventId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModem, IMobileBroadbandModem_Vtbl, 0xd0356912_e9f9_4f67_a03d_43189a316bf1);
impl windows_core::RuntimeType for IMobileBroadbandModem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxDeviceServiceCommandSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxDeviceServiceDataSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceServices: usize,
    pub GetDeviceService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsResetSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ResetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModem2, IMobileBroadbandModem2_Vtbl, 0x12862b28_b9eb_4ee2_bbe3_711f53eea373);
impl windows_core::RuntimeType for IMobileBroadbandModem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetIsPassthroughEnabledAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsPassthroughEnabledAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModem3, IMobileBroadbandModem3_Vtbl, 0xe9fec6ea_2f34_4582_9102_c314d2a87eec);
impl windows_core::RuntimeType for IMobileBroadbandModem3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModem3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetPcoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsInEmergencyCallMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInEmergencyCallModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsInEmergencyCallModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModem4, IMobileBroadbandModem4_Vtbl, 0x4a0398c2_91be_412b_b569_586e9f0030d1);
impl windows_core::RuntimeType for IMobileBroadbandModem4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModem4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsPassthroughEnabledWithSlotIndexAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIsPassthroughEnabledWithSlotIndexAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsPassthroughEnabledWithSlotIndex: unsafe extern "system" fn(*mut core::ffi::c_void, bool, i32, *mut MobileBroadbandModemStatus) -> windows_core::HRESULT,
    pub GetIsPassthroughEnabledWithSlotIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModemConfiguration, IMobileBroadbandModemConfiguration_Vtbl, 0xfce035a3_d6cd_4320_b982_be9d3ec7890f);
impl windows_core::RuntimeType for IMobileBroadbandModemConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModemConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uicc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HomeProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HomeProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModemConfiguration2, IMobileBroadbandModemConfiguration2_Vtbl, 0x320ff5c5_e460_42ae_aa51_69621e7a4477);
impl windows_core::RuntimeType for IMobileBroadbandModemConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModemConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SarManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModemIsolation, IMobileBroadbandModemIsolation_Vtbl, 0xb5618fec_e661_4330_9bb4_3480212ec354);
impl windows_core::RuntimeType for IMobileBroadbandModemIsolation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModemIsolation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddAllowedHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAllowedHostRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModemIsolationFactory, IMobileBroadbandModemIsolationFactory_Vtbl, 0x21d7ec58_c2b1_4c2f_a030_72820a24ecd9);
impl windows_core::RuntimeType for IMobileBroadbandModemIsolationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModemIsolationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandModemStatics, IMobileBroadbandModemStatics_Vtbl, 0xf99ed637_d6f1_4a78_8cbc_6421a65063c8);
impl windows_core::RuntimeType for IMobileBroadbandModemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandModemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandNetwork, IMobileBroadbandNetwork_Vtbl, 0xcb63928c_0309_4cb6_a8c1_6a5a3c8e1ff6);
impl windows_core::RuntimeType for IMobileBroadbandNetwork {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandNetwork_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    pub NetworkRegistrationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkRegistrationState) -> windows_core::HRESULT,
    pub RegistrationNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PacketAttachNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ActivationNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RegisteredDataClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataClasses) -> windows_core::HRESULT,
    pub RegisteredProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RegisteredProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ShowConnectionUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandNetwork2, IMobileBroadbandNetwork2_Vtbl, 0x5a55db22_62f7_4bdd_ba1d_477441960ba0);
impl windows_core::RuntimeType for IMobileBroadbandNetwork2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandNetwork2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetVoiceCallSupportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegistrationUiccApps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegistrationUiccApps: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandNetwork3, IMobileBroadbandNetwork3_Vtbl, 0x33670a8a_c7ef_444c_ab6c_df7ef7a390fe);
impl windows_core::RuntimeType for IMobileBroadbandNetwork3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandNetwork3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCellsInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandNetworkRegistrationStateChange, IMobileBroadbandNetworkRegistrationStateChange_Vtbl, 0xbeaf94e1_960f_49b4_a08d_7d85e968c7ec);
impl windows_core::RuntimeType for IMobileBroadbandNetworkRegistrationStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandNetworkRegistrationStateChange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Network: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails, IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl, 0x89135cff_28b8_46aa_b137_1c4b0f21edfe);
impl windows_core::RuntimeType for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NetworkRegistrationStateChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NetworkRegistrationStateChanges: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandPco, IMobileBroadbandPco_Vtbl, 0xd4e4fcbe_e3a3_43c5_a87b_6c86d229d7fa);
impl windows_core::RuntimeType for IMobileBroadbandPco {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandPco_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandPcoDataChangeTriggerDetails, IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl, 0x263f5114_64e0_4493_909b_2d14a01962b1);
impl windows_core::RuntimeType for IMobileBroadbandPcoDataChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdatedData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandPin, IMobileBroadbandPin_Vtbl, 0xe661d709_e779_45bf_8281_75323df9e321);
impl windows_core::RuntimeType for IMobileBroadbandPin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandPin_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandPinType) -> windows_core::HRESULT,
    pub LockState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandPinLockState) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandPinFormat) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MinLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnblockAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandPinLockStateChange, IMobileBroadbandPinLockStateChange_Vtbl, 0xbe16673e_1f04_4f95_8b90_e7f559dde7e5);
impl windows_core::RuntimeType for IMobileBroadbandPinLockStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandPinLockStateChange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PinType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandPinType) -> windows_core::HRESULT,
    pub PinLockState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandPinLockState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandPinLockStateChangeTriggerDetails, IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl, 0xd338c091_3e91_4d38_9036_aee83a6e79ad);
impl windows_core::RuntimeType for IMobileBroadbandPinLockStateChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PinLockStateChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PinLockStateChanges: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandPinManager, IMobileBroadbandPinManager_Vtbl, 0x83567edd_6e1f_4b9b_a413_2b1f50cc36df);
impl windows_core::RuntimeType for IMobileBroadbandPinManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandPinManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPins: usize,
    pub GetPin: unsafe extern "system" fn(*mut core::ffi::c_void, MobileBroadbandPinType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandPinOperationResult, IMobileBroadbandPinOperationResult_Vtbl, 0x11dddc32_31e7_49f5_b663_123d3bef0362);
impl windows_core::RuntimeType for IMobileBroadbandPinOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandPinOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSuccessful: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandRadioStateChange, IMobileBroadbandRadioStateChange_Vtbl, 0xb054a561_9833_4aed_9717_4348b21a24b3);
impl windows_core::RuntimeType for IMobileBroadbandRadioStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandRadioStateChange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RadioState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandRadioState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandRadioStateChangeTriggerDetails, IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl, 0x71301ace_093c_42c6_b0db_ad1f75a65445);
impl windows_core::RuntimeType for IMobileBroadbandRadioStateChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RadioStateChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RadioStateChanges: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandSarManager, IMobileBroadbandSarManager_Vtbl, 0xe5b26833_967e_40c9_a485_19c0dd209e22);
impl windows_core::RuntimeType for IMobileBroadbandSarManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandSarManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBackoffEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsWiFiHardwareIntegrated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSarControlledByHardware: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Antennas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Antennas: usize,
    pub HysteresisTimerPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub TransmissionStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTransmissionStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnableBackoffAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableBackoffAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetConfigurationAsync: usize,
    pub RevertSarToHardwareControlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTransmissionStateChangedHysteresisAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIsTransmittingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTransmissionStateMonitoring: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopTransmissionStateMonitoring: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandSlotInfo, IMobileBroadbandSlotInfo_Vtbl, 0xbd350b32_882e_542a_b17d_0bb1b49bae9e);
impl windows_core::RuntimeType for IMobileBroadbandSlotInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandSlotInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandSlotState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandSlotInfo2, IMobileBroadbandSlotInfo2_Vtbl, 0x393cb039_ca44_524c_822d_83a3620f0efc);
impl windows_core::RuntimeType for IMobileBroadbandSlotInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandSlotInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IccId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandSlotInfoChangedEventArgs, IMobileBroadbandSlotInfoChangedEventArgs_Vtbl, 0x3158839f_950c_54ce_a48d_ba4529b48f0f);
impl windows_core::RuntimeType for IMobileBroadbandSlotInfoChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SlotInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandSlotManager, IMobileBroadbandSlotManager_Vtbl, 0xeba07cd6_2019_5f81_a294_cc364a11d0b2);
impl windows_core::RuntimeType for IMobileBroadbandSlotManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandSlotManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SlotInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SlotInfos: usize,
    pub CurrentSlotIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCurrentSlot: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut MobileBroadbandModemStatus) -> windows_core::HRESULT,
    pub SetCurrentSlotAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SlotInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSlotInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CurrentSlotIndexChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCurrentSlotIndexChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandTransmissionStateChangedEventArgs, IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl, 0x612e3875_040a_4f99_a4f9_61d7c32da129);
impl windows_core::RuntimeType for IMobileBroadbandTransmissionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTransmitting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandUicc, IMobileBroadbandUicc_Vtbl, 0xe634f691_525a_4ce2_8fce_aa4162579154);
impl windows_core::RuntimeType for IMobileBroadbandUicc {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandUicc_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SimIccId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetUiccAppsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandUiccApp, IMobileBroadbandUiccApp_Vtbl, 0x4d170556_98a1_43dd_b2ec_50c90cf248df);
impl windows_core::RuntimeType for IMobileBroadbandUiccApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandUiccApp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiccAppKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecordDetailsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecordDetailsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadRecordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadRecordAsync: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandUiccAppReadRecordResult, IMobileBroadbandUiccAppReadRecordResult_Vtbl, 0x64c95285_358e_47c5_8249_695f383b2bdb);
impl windows_core::RuntimeType for IMobileBroadbandUiccAppReadRecordResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandUiccAppReadRecordResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandUiccAppOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
windows_core::imp::define_interface!(IMobileBroadbandUiccAppRecordDetailsResult, IMobileBroadbandUiccAppRecordDetailsResult_Vtbl, 0xd919682f_be14_4934_981d_2f57b9ed83e6);
impl windows_core::RuntimeType for IMobileBroadbandUiccAppRecordDetailsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandUiccAppOperationStatus) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiccAppRecordKind) -> windows_core::HRESULT,
    pub RecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RecordSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReadAccessCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiccAccessCondition) -> windows_core::HRESULT,
    pub WriteAccessCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiccAccessCondition) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMobileBroadbandUiccAppsResult, IMobileBroadbandUiccAppsResult_Vtbl, 0x744930eb_8157_4a41_8494_6bf54c9b1d2b);
impl windows_core::RuntimeType for IMobileBroadbandUiccAppsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMobileBroadbandUiccAppsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MobileBroadbandUiccAppOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UiccApps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UiccApps: usize,
}
windows_core::imp::define_interface!(INetworkOperatorDataUsageTriggerDetails, INetworkOperatorDataUsageTriggerDetails_Vtbl, 0x50e3126d_a465_4eeb_9317_28a167630cea);
impl windows_core::RuntimeType for INetworkOperatorDataUsageTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorDataUsageTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotificationKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkOperatorDataUsageNotificationKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorNotificationEventDetails, INetworkOperatorNotificationEventDetails_Vtbl, 0xbc68a9d1_82e1_4488_9f2c_1276c2468fac);
impl windows_core::RuntimeType for INetworkOperatorNotificationEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorNotificationEventDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotificationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkOperatorEventMessageType) -> windows_core::HRESULT,
    pub NetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EncodingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RuleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub SmsMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    SmsMessage: usize,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringAccessPointConfiguration, INetworkOperatorTetheringAccessPointConfiguration_Vtbl, 0x0bcc0284_412e_403d_acc6_b757e34774a4);
impl windows_core::RuntimeType for INetworkOperatorTetheringAccessPointConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringAccessPointConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Ssid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Passphrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPassphrase: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringAccessPointConfiguration2, INetworkOperatorTetheringAccessPointConfiguration2_Vtbl, 0xb1809142_7238_59a0_928b_74ab46fd64b6);
impl windows_core::RuntimeType for INetworkOperatorTetheringAccessPointConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBandSupported: unsafe extern "system" fn(*mut core::ffi::c_void, TetheringWiFiBand, *mut bool) -> windows_core::HRESULT,
    pub IsBandSupportedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, TetheringWiFiBand, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Band: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TetheringWiFiBand) -> windows_core::HRESULT,
    pub SetBand: unsafe extern "system" fn(*mut core::ffi::c_void, TetheringWiFiBand) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringClient, INetworkOperatorTetheringClient_Vtbl, 0x709d254c_595f_4847_bb30_646935542918);
impl windows_core::RuntimeType for INetworkOperatorTetheringClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringClient_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MacAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HostNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HostNames: usize,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringClientManager, INetworkOperatorTetheringClientManager_Vtbl, 0x91b14016_8dca_4225_bbed_eef8b8d718d7);
impl windows_core::RuntimeType for INetworkOperatorTetheringClientManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringClientManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTetheringClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTetheringClients: usize,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringEntitlementCheck, INetworkOperatorTetheringEntitlementCheck_Vtbl, 0x0108916d_9e9a_4af6_8da3_60493b19c204);
impl windows_core::RuntimeType for INetworkOperatorTetheringEntitlementCheck {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringEntitlementCheck_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AuthorizeTethering: unsafe extern "system" fn(*mut core::ffi::c_void, bool, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringManager, INetworkOperatorTetheringManager_Vtbl, 0xd45a8da0_0e86_4d98_8ba4_dd70d4b764d3);
impl windows_core::RuntimeType for INetworkOperatorTetheringManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxClientCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ClientCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TetheringOperationalState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TetheringOperationalState) -> windows_core::HRESULT,
    pub GetCurrentAccessPointConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureAccessPointAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTetheringAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopTetheringAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringManagerStatics, INetworkOperatorTetheringManagerStatics_Vtbl, 0x3ebcbacc_f8c3_405c_9964_70a1eeabe194);
impl windows_core::RuntimeType for INetworkOperatorTetheringManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetTetheringCapability: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut TetheringCapability) -> windows_core::HRESULT,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringManagerStatics2, INetworkOperatorTetheringManagerStatics2_Vtbl, 0x5b235412_35f0_49e7_9b08_16d278fbaa42);
impl windows_core::RuntimeType for INetworkOperatorTetheringManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub GetTetheringCapabilityFromConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut TetheringCapability) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    GetTetheringCapabilityFromConnectionProfile: usize,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfile: usize,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringManagerStatics3, INetworkOperatorTetheringManagerStatics3_Vtbl, 0x8fdaadb6_4af9_4f21_9b58_d53e9f24231e);
impl windows_core::RuntimeType for INetworkOperatorTetheringManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfileWithTargetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfileWithTargetAdapter: usize,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringManagerStatics4, INetworkOperatorTetheringManagerStatics4_Vtbl, 0xb3b9f9d0_ebff_46a4_a847_d663d8b0977e);
impl windows_core::RuntimeType for INetworkOperatorTetheringManagerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsNoConnectionsTimeoutEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EnableNoConnectionsTimeout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableNoConnectionsTimeoutAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableNoConnectionsTimeout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableNoConnectionsTimeoutAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorTetheringOperationResult, INetworkOperatorTetheringOperationResult_Vtbl, 0xebd203a1_01ba_476d_b4b3_bf3d12c8f80c);
impl windows_core::RuntimeType for INetworkOperatorTetheringOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorTetheringOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TetheringOperationStatus) -> windows_core::HRESULT,
    pub AdditionalErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProvisionFromXmlDocumentResults, IProvisionFromXmlDocumentResults_Vtbl, 0x217700e0_8203_11df_adb9_f4ce462d9137);
impl windows_core::RuntimeType for IProvisionFromXmlDocumentResults {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProvisionFromXmlDocumentResults_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllElementsProvisioned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ProvisionResultsXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProvisionedProfile, IProvisionedProfile_Vtbl, 0x217700e0_8202_11df_adb9_f4ce462d9137);
impl windows_core::RuntimeType for IProvisionedProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProvisionedProfile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub UpdateCost: unsafe extern "system" fn(*mut core::ffi::c_void, super::Connectivity::NetworkCostType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    UpdateCost: usize,
    pub UpdateUsage: unsafe extern "system" fn(*mut core::ffi::c_void, ProfileUsage) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProvisioningAgent, IProvisioningAgent_Vtbl, 0x217700e0_8201_11df_adb9_f4ce462d9137);
impl windows_core::RuntimeType for IProvisioningAgent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProvisioningAgent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProvisionFromXmlDocumentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProvisionedProfile: unsafe extern "system" fn(*mut core::ffi::c_void, ProfileMediaType, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProvisioningAgentStaticMethods, IProvisioningAgentStaticMethods_Vtbl, 0x217700e0_8101_11df_adb9_f4ce462d9137);
impl windows_core::RuntimeType for IProvisioningAgentStaticMethods {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProvisioningAgentStaticMethods_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITetheringEntitlementCheckTriggerDetails, ITetheringEntitlementCheckTriggerDetails_Vtbl, 0x03c65e9d_5926_41f3_a94e_b50926fc421b);
impl windows_core::RuntimeType for ITetheringEntitlementCheckTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITetheringEntitlementCheckTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AllowTethering: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DenyTethering: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUssdMessage, IUssdMessage_Vtbl, 0x2f9acf82_2004_4d5d_bf81_2aba1b4be4a8);
impl windows_core::RuntimeType for IUssdMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUssdMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DataCodingScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetDataCodingScheme: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub GetPayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetPayload: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub PayloadAsText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPayloadAsText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUssdMessageFactory, IUssdMessageFactory_Vtbl, 0x2f9acf82_1003_4d5d_bf81_2aba1b4be4a8);
impl windows_core::RuntimeType for IUssdMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUssdMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUssdReply, IUssdReply_Vtbl, 0x2f9acf82_2005_4d5d_bf81_2aba1b4be4a8);
impl windows_core::RuntimeType for IUssdReply {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUssdReply_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UssdResultCode) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUssdSession, IUssdSession_Vtbl, 0x2f9acf82_2002_4d5d_bf81_2aba1b4be4a8);
impl windows_core::RuntimeType for IUssdSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUssdSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessageAndGetReplyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUssdSessionStatics, IUssdSessionStatics_Vtbl, 0x2f9acf82_1001_4d5d_bf81_2aba1b4be4a8);
impl windows_core::RuntimeType for IUssdSessionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUssdSessionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromNetworkInterfaceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESim(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESim, windows_core::IUnknown, windows_core::IInspectable);
impl ESim {
    pub fn AvailableMemoryInBytes(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailableMemoryInBytes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Eid(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Eid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirmwareVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirmwareVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MobileBroadbandModemDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MobileBroadbandModemDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Policy(&self) -> windows_core::Result<ESimPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Policy)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<ESimState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProfiles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ESimProfile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProfiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteProfileAsync(&self, profileid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteProfileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(profileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DownloadProfileMetadataAsync(&self, activationcode: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadProfileMetadataAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activationcode), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResetAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProfileChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ESim, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProfileChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProfileChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Discover(&self) -> windows_core::Result<ESimDiscoverResult> {
        let this = &windows_core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Discover)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DiscoverWithServerAddressAndMatchingId(&self, serveraddress: &windows_core::HSTRING, matchingid: &windows_core::HSTRING) -> windows_core::Result<ESimDiscoverResult> {
        let this = &windows_core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiscoverWithServerAddressAndMatchingId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(serveraddress), core::mem::transmute_copy(matchingid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DiscoverAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &windows_core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiscoverAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DiscoverWithServerAddressAndMatchingIdAsync(&self, serveraddress: &windows_core::HSTRING, matchingid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &windows_core::Interface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiscoverWithServerAddressAndMatchingIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(serveraddress), core::mem::transmute_copy(matchingid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SlotIndex(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = &windows_core::Interface::cast::<IESim3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlotIndex)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESim {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESim>();
}
unsafe impl windows_core::Interface for ESim {
    type Vtable = IESim_Vtbl;
    const IID: windows_core::GUID = <IESim as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESim {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESim";
}
unsafe impl Send for ESim {}
unsafe impl Sync for ESim {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimAddedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimAddedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ESimAddedEventArgs {
    pub fn ESim(&self) -> windows_core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ESim)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESimAddedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimAddedEventArgs>();
}
unsafe impl windows_core::Interface for ESimAddedEventArgs {
    type Vtable = IESimAddedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IESimAddedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimAddedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimAddedEventArgs";
}
unsafe impl Send for ESimAddedEventArgs {}
unsafe impl Sync for ESimAddedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimDiscoverEvent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimDiscoverEvent, windows_core::IUnknown, windows_core::IInspectable);
impl ESimDiscoverEvent {
    pub fn MatchingId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MatchingId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RspServerAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RspServerAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESimDiscoverEvent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimDiscoverEvent>();
}
unsafe impl windows_core::Interface for ESimDiscoverEvent {
    type Vtable = IESimDiscoverEvent_Vtbl;
    const IID: windows_core::GUID = <IESimDiscoverEvent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimDiscoverEvent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverEvent";
}
unsafe impl Send for ESimDiscoverEvent {}
unsafe impl Sync for ESimDiscoverEvent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimDiscoverResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimDiscoverResult, windows_core::IUnknown, windows_core::IInspectable);
impl ESimDiscoverResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Events(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Events)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ESimDiscoverResultKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProfileMetadata(&self) -> windows_core::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileMetadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Result(&self) -> windows_core::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESimDiscoverResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimDiscoverResult>();
}
unsafe impl windows_core::Interface for ESimDiscoverResult {
    type Vtable = IESimDiscoverResult_Vtbl;
    const IID: windows_core::GUID = <IESimDiscoverResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimDiscoverResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverResult";
}
unsafe impl Send for ESimDiscoverResult {}
unsafe impl Sync for ESimDiscoverResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimDownloadProfileMetadataResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimDownloadProfileMetadataResult, windows_core::IUnknown, windows_core::IInspectable);
impl ESimDownloadProfileMetadataResult {
    pub fn Result(&self) -> windows_core::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProfileMetadata(&self) -> windows_core::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileMetadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESimDownloadProfileMetadataResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimDownloadProfileMetadataResult>();
}
unsafe impl windows_core::Interface for ESimDownloadProfileMetadataResult {
    type Vtable = IESimDownloadProfileMetadataResult_Vtbl;
    const IID: windows_core::GUID = <IESimDownloadProfileMetadataResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimDownloadProfileMetadataResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDownloadProfileMetadataResult";
}
unsafe impl Send for ESimDownloadProfileMetadataResult {}
unsafe impl Sync for ESimDownloadProfileMetadataResult {}
pub struct ESimManager;
impl ESimManager {
    pub fn ServiceInfo() -> windows_core::Result<ESimServiceInfo> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryCreateESimWatcher() -> windows_core::Result<ESimWatcher> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateESimWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ServiceInfoChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceInfoChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveServiceInfoChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IESimManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveServiceInfoChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IESimManagerStatics<R, F: FnOnce(&IESimManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ESimManager, IESimManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ESimManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl ESimOperationResult {
    pub fn Status(&self) -> windows_core::Result<ESimOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ESimOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimOperationResult>();
}
unsafe impl windows_core::Interface for ESimOperationResult {
    type Vtable = IESimOperationResult_Vtbl;
    const IID: windows_core::GUID = <IESimOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimOperationResult";
}
unsafe impl Send for ESimOperationResult {}
unsafe impl Sync for ESimOperationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimPolicy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimPolicy, windows_core::IUnknown, windows_core::IInspectable);
impl ESimPolicy {
    pub fn ShouldEnableManagingUi(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldEnableManagingUi)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ESimPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimPolicy>();
}
unsafe impl windows_core::Interface for ESimPolicy {
    type Vtable = IESimPolicy_Vtbl;
    const IID: windows_core::GUID = <IESimPolicy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimPolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimPolicy";
}
unsafe impl Send for ESimPolicy {}
unsafe impl Sync for ESimPolicy {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimProfile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimProfile, windows_core::IUnknown, windows_core::IInspectable);
impl ESimProfile {
    pub fn Class(&self) -> windows_core::Result<ESimProfileClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Class)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Nickname(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Nickname)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Policy(&self) -> windows_core::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Policy)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderIcon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<ESimProfileState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnableAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNicknameAsync(&self, newnickname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetNicknameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(newnickname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESimProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimProfile>();
}
unsafe impl windows_core::Interface for ESimProfile {
    type Vtable = IESimProfile_Vtbl;
    const IID: windows_core::GUID = <IESimProfile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfile";
}
unsafe impl Send for ESimProfile {}
unsafe impl Sync for ESimProfile {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimProfileMetadata(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimProfileMetadata, windows_core::IUnknown, windows_core::IInspectable);
impl ESimProfileMetadata {
    pub fn IsConfirmationCodeRequired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConfirmationCodeRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Policy(&self) -> windows_core::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Policy)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderIcon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<ESimProfileMetadataState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DenyInstallAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DenyInstallAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfirmInstallAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConfirmInstallAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfirmInstallWithConfirmationCodeAsync(&self, confirmationcode: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConfirmInstallWithConfirmationCodeAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(confirmationcode), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PostponeInstallAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PostponeInstallAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ESimProfileMetadata, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ESimProfileMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimProfileMetadata>();
}
unsafe impl windows_core::Interface for ESimProfileMetadata {
    type Vtable = IESimProfileMetadata_Vtbl;
    const IID: windows_core::GUID = <IESimProfileMetadata as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimProfileMetadata {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfileMetadata";
}
unsafe impl Send for ESimProfileMetadata {}
unsafe impl Sync for ESimProfileMetadata {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimProfilePolicy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimProfilePolicy, windows_core::IUnknown, windows_core::IInspectable);
impl ESimProfilePolicy {
    pub fn CanDelete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDelete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanDisable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDisable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsManagedByEnterprise(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsManagedByEnterprise)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ESimProfilePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimProfilePolicy>();
}
unsafe impl windows_core::Interface for ESimProfilePolicy {
    type Vtable = IESimProfilePolicy_Vtbl;
    const IID: windows_core::GUID = <IESimProfilePolicy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimProfilePolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfilePolicy";
}
unsafe impl Send for ESimProfilePolicy {}
unsafe impl Sync for ESimProfilePolicy {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ESimRemovedEventArgs {
    pub fn ESim(&self) -> windows_core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ESim)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESimRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimRemovedEventArgs>();
}
unsafe impl windows_core::Interface for ESimRemovedEventArgs {
    type Vtable = IESimRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IESimRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimRemovedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimRemovedEventArgs";
}
unsafe impl Send for ESimRemovedEventArgs {}
unsafe impl Sync for ESimRemovedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimServiceInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimServiceInfo, windows_core::IUnknown, windows_core::IInspectable);
impl ESimServiceInfo {
    pub fn AuthenticationPreference(&self) -> windows_core::Result<ESimAuthenticationPreference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationPreference)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsESimUiEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsESimUiEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ESimServiceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimServiceInfo>();
}
unsafe impl windows_core::Interface for ESimServiceInfo {
    type Vtable = IESimServiceInfo_Vtbl;
    const IID: windows_core::GUID = <IESimServiceInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimServiceInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimServiceInfo";
}
unsafe impl Send for ESimServiceInfo {}
unsafe impl Sync for ESimServiceInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ESimUpdatedEventArgs {
    pub fn ESim(&self) -> windows_core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ESim)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ESimUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for ESimUpdatedEventArgs {
    type Vtable = IESimUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IESimUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimUpdatedEventArgs";
}
unsafe impl Send for ESimUpdatedEventArgs {}
unsafe impl Sync for ESimUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ESimWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ESimWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl ESimWatcher {
    pub fn Status(&self) -> windows_core::Result<ESimWatcherStatus> {
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
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>>,
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
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ESimWatcher, windows_core::IInspectable>>,
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
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>>,
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
    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ESimWatcher, windows_core::IInspectable>>,
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
    pub fn Updated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>>,
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
}
impl windows_core::RuntimeType for ESimWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IESimWatcher>();
}
unsafe impl windows_core::Interface for ESimWatcher {
    type Vtable = IESimWatcher_Vtbl;
    const IID: windows_core::GUID = <IESimWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ESimWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimWatcher";
}
unsafe impl Send for ESimWatcher {}
unsafe impl Sync for ESimWatcher {}
pub struct FdnAccessManager;
impl FdnAccessManager {
    pub fn RequestUnlockAsync(contactlistid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFdnAccessManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestUnlockAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contactlistid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IFdnAccessManagerStatics<R, F: FnOnce(&IFdnAccessManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FdnAccessManager, IFdnAccessManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for FdnAccessManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.FdnAccessManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HotspotAuthenticationContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HotspotAuthenticationContext, windows_core::IUnknown, windows_core::IInspectable);
impl HotspotAuthenticationContext {
    pub fn WirelessNetworkId(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).WirelessNetworkId)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> windows_core::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAdapter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RedirectMessageUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedirectMessageUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn RedirectMessageXml(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedirectMessageXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AuthenticationUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IssueCredentials(&self, username: &windows_core::HSTRING, password: &windows_core::HSTRING, extraparameters: &windows_core::HSTRING, markasmanualconnectonfailure: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).IssueCredentials)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(username), core::mem::transmute_copy(password), core::mem::transmute_copy(extraparameters), markasmanualconnectonfailure).ok() }
    }
    pub fn AbortAuthentication(&self, markasmanual: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AbortAuthentication)(windows_core::Interface::as_raw(this), markasmanual).ok() }
    }
    pub fn SkipAuthentication(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SkipAuthentication)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TriggerAttentionRequired(&self, packagerelativeapplicationid: &windows_core::HSTRING, applicationparameters: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).TriggerAttentionRequired)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagerelativeapplicationid), core::mem::transmute_copy(applicationparameters)).ok() }
    }
    pub fn IssueCredentialsAsync(&self, username: &windows_core::HSTRING, password: &windows_core::HSTRING, extraparameters: &windows_core::HSTRING, markasmanualconnectonfailure: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>> {
        let this = &windows_core::Interface::cast::<IHotspotAuthenticationContext2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IssueCredentialsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(username), core::mem::transmute_copy(password), core::mem::transmute_copy(extraparameters), markasmanualconnectonfailure, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAuthenticationContext(eventoken: &windows_core::HSTRING, context: &mut Option<HotspotAuthenticationContext>) -> windows_core::Result<bool> {
        Self::IHotspotAuthenticationContextStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAuthenticationContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(eventoken), context as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IHotspotAuthenticationContextStatics<R, F: FnOnce(&IHotspotAuthenticationContextStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HotspotAuthenticationContext, IHotspotAuthenticationContextStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HotspotAuthenticationContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHotspotAuthenticationContext>();
}
unsafe impl windows_core::Interface for HotspotAuthenticationContext {
    type Vtable = IHotspotAuthenticationContext_Vtbl;
    const IID: windows_core::GUID = <IHotspotAuthenticationContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HotspotAuthenticationContext {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationContext";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HotspotAuthenticationEventDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HotspotAuthenticationEventDetails, windows_core::IUnknown, windows_core::IInspectable);
impl HotspotAuthenticationEventDetails {
    pub fn EventToken(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventToken)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HotspotAuthenticationEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHotspotAuthenticationEventDetails>();
}
unsafe impl windows_core::Interface for HotspotAuthenticationEventDetails {
    type Vtable = IHotspotAuthenticationEventDetails_Vtbl;
    const IID: windows_core::GUID = <IHotspotAuthenticationEventDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HotspotAuthenticationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationEventDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HotspotCredentialsAuthenticationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HotspotCredentialsAuthenticationResult, windows_core::IUnknown, windows_core::IInspectable);
impl HotspotCredentialsAuthenticationResult {
    pub fn HasNetworkErrorOccurred(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNetworkErrorOccurred)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResponseCode(&self) -> windows_core::Result<HotspotAuthenticationResponseCode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LogoffUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogoffUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn AuthenticationReplyXml(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationReplyXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HotspotCredentialsAuthenticationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHotspotCredentialsAuthenticationResult>();
}
unsafe impl windows_core::Interface for HotspotCredentialsAuthenticationResult {
    type Vtable = IHotspotCredentialsAuthenticationResult_Vtbl;
    const IID: windows_core::GUID = <IHotspotCredentialsAuthenticationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HotspotCredentialsAuthenticationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotCredentialsAuthenticationResult";
}
pub struct KnownCSimFilePaths;
impl KnownCSimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EFSpn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownCSimFilePathsStatics<R, F: FnOnce(&IKnownCSimFilePathsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownCSimFilePaths, IKnownCSimFilePathsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownCSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownCSimFilePaths";
}
pub struct KnownRuimFilePaths;
impl KnownRuimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EFSpn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownRuimFilePathsStatics<R, F: FnOnce(&IKnownRuimFilePathsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownRuimFilePaths, IKnownRuimFilePathsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownRuimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownRuimFilePaths";
}
pub struct KnownSimFilePaths;
impl KnownSimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOns() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EFOns)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EFSpn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownSimFilePathsStatics<R, F: FnOnce(&IKnownSimFilePathsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownSimFilePaths, IKnownSimFilePathsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownSimFilePaths";
}
pub struct KnownUSimFilePaths;
impl KnownUSimFilePaths {
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EFSpn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOpl() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EFOpl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFPnn() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EFPnn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gid2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownUSimFilePathsStatics<R, F: FnOnce(&IKnownUSimFilePathsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownUSimFilePaths, IKnownUSimFilePathsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownUSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownUSimFilePaths";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandAccount(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandAccount, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandAccount {
    pub fn NetworkAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceProviderGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceProviderGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentNetwork(&self) -> windows_core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentNetwork)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentDeviceInformation(&self) -> windows_core::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentDeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub fn GetConnectionProfiles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandAccount2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionProfiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccountExperienceUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandAccount3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountExperienceUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableNetworkAccountIds() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailableNetworkAccountIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &windows_core::HSTRING) -> windows_core::Result<MobileBroadbandAccount> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandAccountStatics<R, F: FnOnce(&IMobileBroadbandAccountStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandAccount, IMobileBroadbandAccountStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandAccount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandAccount>();
}
unsafe impl windows_core::Interface for MobileBroadbandAccount {
    type Vtable = IMobileBroadbandAccount_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandAccount as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandAccount {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccount";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandAccountEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandAccountEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandAccountEventArgs {
    pub fn NetworkAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandAccountEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandAccountEventArgs>();
}
unsafe impl windows_core::Interface for MobileBroadbandAccountEventArgs {
    type Vtable = IMobileBroadbandAccountEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandAccountEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandAccountEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandAccountUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandAccountUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandAccountUpdatedEventArgs {
    pub fn NetworkAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasDeviceInformationChanged(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasDeviceInformationChanged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNetworkChanged(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNetworkChanged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandAccountUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandAccountUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for MobileBroadbandAccountUpdatedEventArgs {
    type Vtable = IMobileBroadbandAccountUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandAccountUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandAccountUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountUpdatedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandAccountWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandAccountWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandAccountWatcher {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandAccountWatcher, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AccountAdded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountAdded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAccountAdded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAccountAdded)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AccountUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAccountUpdated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAccountUpdated)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AccountRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAccountRemoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAccountRemoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnumerationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnumerationCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopped(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopped)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<MobileBroadbandAccountWatcherStatus> {
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
}
impl windows_core::RuntimeType for MobileBroadbandAccountWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandAccountWatcher>();
}
unsafe impl windows_core::Interface for MobileBroadbandAccountWatcher {
    type Vtable = IMobileBroadbandAccountWatcher_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandAccountWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandAccountWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcher";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandAntennaSar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandAntennaSar, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandAntennaSar {
    pub fn AntennaIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AntennaIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SarBackoffIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SarBackoffIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateWithIndex(antennaindex: i32, sarbackoffindex: i32) -> windows_core::Result<MobileBroadbandAntennaSar> {
        Self::IMobileBroadbandAntennaSarFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithIndex)(windows_core::Interface::as_raw(this), antennaindex, sarbackoffindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandAntennaSarFactory<R, F: FnOnce(&IMobileBroadbandAntennaSarFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandAntennaSar, IMobileBroadbandAntennaSarFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandAntennaSar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandAntennaSar>();
}
unsafe impl windows_core::Interface for MobileBroadbandAntennaSar {
    type Vtable = IMobileBroadbandAntennaSar_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandAntennaSar as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandAntennaSar {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAntennaSar";
}
unsafe impl Send for MobileBroadbandAntennaSar {}
unsafe impl Sync for MobileBroadbandAntennaSar {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCellCdma(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCellCdma, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCellCdma {
    pub fn BaseStationId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseStationId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BaseStationPNCode(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseStationPNCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BaseStationLatitude(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseStationLatitude)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BaseStationLongitude(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseStationLongitude)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BaseStationLastBroadcastGpsTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseStationLastBroadcastGpsTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NetworkId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PilotSignalStrengthInDB(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PilotSignalStrengthInDB)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SystemId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCellCdma {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCellCdma>();
}
unsafe impl windows_core::Interface for MobileBroadbandCellCdma {
    type Vtable = IMobileBroadbandCellCdma_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCellCdma as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCellCdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellCdma";
}
unsafe impl Send for MobileBroadbandCellCdma {}
unsafe impl Sync for MobileBroadbandCellCdma {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCellGsm(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCellGsm, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCellGsm {
    pub fn BaseStationId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseStationId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CellId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChannelNumber(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocationAreaCode(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationAreaCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReceivedSignalStrengthInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceivedSignalStrengthInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimingAdvanceInBitPeriods(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimingAdvanceInBitPeriods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCellGsm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCellGsm>();
}
unsafe impl windows_core::Interface for MobileBroadbandCellGsm {
    type Vtable = IMobileBroadbandCellGsm_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCellGsm as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCellGsm {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellGsm";
}
unsafe impl Send for MobileBroadbandCellGsm {}
unsafe impl Sync for MobileBroadbandCellGsm {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCellLte(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCellLte, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCellLte {
    pub fn CellId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChannelNumber(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhysicalCellId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalCellId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReferenceSignalReceivedPowerInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReferenceSignalReceivedQualityInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimingAdvanceInBitPeriods(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimingAdvanceInBitPeriods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrackingAreaCode(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrackingAreaCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCellLte {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCellLte>();
}
unsafe impl windows_core::Interface for MobileBroadbandCellLte {
    type Vtable = IMobileBroadbandCellLte_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCellLte as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCellLte {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellLte";
}
unsafe impl Send for MobileBroadbandCellLte {}
unsafe impl Sync for MobileBroadbandCellLte {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCellNR(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCellNR, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCellNR {
    pub fn CellId(&self) -> windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChannelNumber(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhysicalCellId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalCellId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReferenceSignalReceivedPowerInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReferenceSignalReceivedQualityInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimingAdvanceInNanoseconds(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimingAdvanceInNanoseconds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrackingAreaCode(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrackingAreaCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignalToNoiseRatioInDB(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalToNoiseRatioInDB)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCellNR {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCellNR>();
}
unsafe impl windows_core::Interface for MobileBroadbandCellNR {
    type Vtable = IMobileBroadbandCellNR_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCellNR as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCellNR {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellNR";
}
unsafe impl Send for MobileBroadbandCellNR {}
unsafe impl Sync for MobileBroadbandCellNR {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCellTdscdma(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCellTdscdma, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCellTdscdma {
    pub fn CellId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CellParameterId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellParameterId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChannelNumber(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocationAreaCode(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationAreaCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PathLossInDB(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PathLossInDB)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReceivedSignalCodePowerInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceivedSignalCodePowerInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimingAdvanceInBitPeriods(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimingAdvanceInBitPeriods)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCellTdscdma {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCellTdscdma>();
}
unsafe impl windows_core::Interface for MobileBroadbandCellTdscdma {
    type Vtable = IMobileBroadbandCellTdscdma_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCellTdscdma as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCellTdscdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellTdscdma";
}
unsafe impl Send for MobileBroadbandCellTdscdma {}
unsafe impl Sync for MobileBroadbandCellTdscdma {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCellUmts(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCellUmts, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCellUmts {
    pub fn CellId(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChannelNumber(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocationAreaCode(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocationAreaCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PathLossInDB(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PathLossInDB)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrimaryScramblingCode(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryScramblingCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReceivedSignalCodePowerInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceivedSignalCodePowerInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignalToNoiseRatioInDB(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalToNoiseRatioInDB)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCellUmts {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCellUmts>();
}
unsafe impl windows_core::Interface for MobileBroadbandCellUmts {
    type Vtable = IMobileBroadbandCellUmts_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCellUmts as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCellUmts {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellUmts";
}
unsafe impl Send for MobileBroadbandCellUmts {}
unsafe impl Sync for MobileBroadbandCellUmts {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCellsInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCellsInfo, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCellsInfo {
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsCdma(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringCellsCdma)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsGsm(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringCellsGsm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsLte(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringCellsLte)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsTdscdma(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringCellsTdscdma)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsUmts(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringCellsUmts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsCdma(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServingCellsCdma)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsGsm(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServingCellsGsm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsLte(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServingCellsLte)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsTdscdma(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServingCellsTdscdma)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsUmts(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServingCellsUmts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsNR(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeighboringCellsNR)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsNR(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServingCellsNR)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCellsInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCellsInfo>();
}
unsafe impl windows_core::Interface for MobileBroadbandCellsInfo {
    type Vtable = IMobileBroadbandCellsInfo_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCellsInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCellsInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellsInfo";
}
unsafe impl Send for MobileBroadbandCellsInfo {}
unsafe impl Sync for MobileBroadbandCellsInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandCurrentSlotIndexChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandCurrentSlotIndexChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandCurrentSlotIndexChangedEventArgs {
    pub fn CurrentSlotIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentSlotIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandCurrentSlotIndexChangedEventArgs>();
}
unsafe impl windows_core::Interface for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    type Vtable = IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandCurrentSlotIndexChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCurrentSlotIndexChangedEventArgs";
}
unsafe impl Send for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
unsafe impl Sync for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceInformation, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceInformation {
    pub fn NetworkDeviceStatus(&self) -> windows_core::Result<NetworkDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkDeviceStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Manufacturer(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Manufacturer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Model(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Model)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirmwareInformation(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirmwareInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Sms")]
    pub fn CellularClass(&self) -> windows_core::Result<super::super::Devices::Sms::CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DataClasses(&self) -> windows_core::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataClasses)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CustomDataClass(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomDataClass)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MobileEquipmentId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MobileEquipmentId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TelephoneNumbers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TelephoneNumbers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubscriberId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscriberId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceType(&self) -> windows_core::Result<MobileBroadbandDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentRadioState(&self) -> windows_core::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentRadioState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PinManager(&self) -> windows_core::Result<MobileBroadbandPinManager> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Revision(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Revision)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SerialNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SerialNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SimSpn(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimSpn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SimPnn(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimPnn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SimGid1(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimGid1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SlotManager(&self) -> windows_core::Result<MobileBroadbandSlotManager> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceInformation4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlotManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceInformation>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceInformation {
    type Vtable = IMobileBroadbandDeviceInformation_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceInformation";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceService, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceService {
    pub fn DeviceServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCommands(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCommands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenDataSession(&self) -> windows_core::Result<MobileBroadbandDeviceServiceDataSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenDataSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenCommandSession(&self) -> windows_core::Result<MobileBroadbandDeviceServiceCommandSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenCommandSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceService>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceService {
    type Vtable = IMobileBroadbandDeviceService_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceService {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceService";
}
unsafe impl Send for MobileBroadbandDeviceService {}
unsafe impl Sync for MobileBroadbandDeviceService {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceServiceCommandResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceCommandResult, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceServiceCommandResult {
    pub fn StatusCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ResponseData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceServiceCommandResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceServiceCommandResult>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceServiceCommandResult {
    type Vtable = IMobileBroadbandDeviceServiceCommandResult_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceServiceCommandResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceServiceCommandResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandResult";
}
unsafe impl Send for MobileBroadbandDeviceServiceCommandResult {}
unsafe impl Sync for MobileBroadbandDeviceServiceCommandResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceServiceCommandSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceCommandSession, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceServiceCommandSession {
    #[cfg(feature = "Storage_Streams")]
    pub fn SendQueryCommandAsync<P0>(&self, commandid: u32, data: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendQueryCommandAsync)(windows_core::Interface::as_raw(this), commandid, data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendSetCommandAsync<P0>(&self, commandid: u32, data: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendSetCommandAsync)(windows_core::Interface::as_raw(this), commandid, data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloseSession(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CloseSession)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceServiceCommandSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceServiceCommandSession>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceServiceCommandSession {
    type Vtable = IMobileBroadbandDeviceServiceCommandSession_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceServiceCommandSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceServiceCommandSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandSession";
}
unsafe impl Send for MobileBroadbandDeviceServiceCommandSession {}
unsafe impl Sync for MobileBroadbandDeviceServiceCommandSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceServiceDataReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceServiceDataReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceivedData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceServiceDataReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    type Vtable = IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceServiceDataReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataReceivedEventArgs";
}
unsafe impl Send for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
unsafe impl Sync for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceServiceDataSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceDataSession, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceServiceDataSession {
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteDataAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteDataAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloseSession(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CloseSession)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DataReceived<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataReceived)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDataReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDataReceived)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceServiceDataSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceServiceDataSession>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceServiceDataSession {
    type Vtable = IMobileBroadbandDeviceServiceDataSession_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceServiceDataSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceServiceDataSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataSession";
}
unsafe impl Send for MobileBroadbandDeviceServiceDataSession {}
unsafe impl Sync for MobileBroadbandDeviceServiceDataSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceServiceInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceInformation, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceServiceInformation {
    pub fn DeviceServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDataReadSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDataReadSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDataWriteSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDataWriteSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceServiceInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceServiceInformation>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceServiceInformation {
    type Vtable = IMobileBroadbandDeviceServiceInformation_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceServiceInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceServiceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceInformation";
}
unsafe impl Send for MobileBroadbandDeviceServiceInformation {}
unsafe impl Sync for MobileBroadbandDeviceServiceInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceServiceTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandDeviceServiceTriggerDetails {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceivedData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EventId(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandDeviceServiceTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceServiceTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandDeviceServiceTriggerDetails>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceServiceTriggerDetails {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandDeviceServiceTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceServiceTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceTriggerDetails";
}
unsafe impl Send for MobileBroadbandDeviceServiceTriggerDetails {}
unsafe impl Sync for MobileBroadbandDeviceServiceTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandModem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandModem, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandModem {
    pub fn CurrentAccount(&self) -> windows_core::Result<MobileBroadbandAccount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentAccount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceInformation(&self) -> windows_core::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxDeviceServiceCommandSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDeviceServiceCommandSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxDeviceServiceDataSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDeviceServiceDataSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceServices(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceService(&self, deviceserviceid: windows_core::GUID) -> windows_core::Result<MobileBroadbandDeviceService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceService)(windows_core::Interface::as_raw(this), deviceserviceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsResetSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsResetSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResetAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentConfigurationAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentConfigurationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentNetwork(&self) -> windows_core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentNetwork)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIsPassthroughEnabledAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsPassthroughEnabledAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsPassthroughEnabledAsync(&self, value: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetIsPassthroughEnabledAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetPcoAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPcoAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsInEmergencyCallMode(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInEmergencyCallMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInEmergencyCallModeChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandModem, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInEmergencyCallModeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsInEmergencyCallModeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsInEmergencyCallModeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetIsPassthroughEnabledWithSlotIndexAsync(&self, value: bool, slotindex: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetIsPassthroughEnabledWithSlotIndexAsync)(windows_core::Interface::as_raw(this), value, slotindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIsPassthroughEnabledWithSlotIndexAsync(&self, slotindex: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsPassthroughEnabledWithSlotIndexAsync)(windows_core::Interface::as_raw(this), slotindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsPassthroughEnabledWithSlotIndex(&self, value: bool, slotindex: i32) -> windows_core::Result<MobileBroadbandModemStatus> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetIsPassthroughEnabledWithSlotIndex)(windows_core::Interface::as_raw(this), value, slotindex, &mut result__).map(|| result__)
        }
    }
    pub fn GetIsPassthroughEnabledWithSlotIndex(&self, slotindex: i32) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsPassthroughEnabledWithSlotIndex)(windows_core::Interface::as_raw(this), slotindex, &mut result__).map(|| result__)
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromId(deviceid: &windows_core::HSTRING) -> windows_core::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefault() -> windows_core::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandModemStatics<R, F: FnOnce(&IMobileBroadbandModemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandModem, IMobileBroadbandModemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandModem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandModem>();
}
unsafe impl windows_core::Interface for MobileBroadbandModem {
    type Vtable = IMobileBroadbandModem_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandModem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandModem {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModem";
}
unsafe impl Send for MobileBroadbandModem {}
unsafe impl Sync for MobileBroadbandModem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandModemConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandModemConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandModemConfiguration {
    pub fn Uicc(&self) -> windows_core::Result<MobileBroadbandUicc> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uicc)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HomeProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HomeProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HomeProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HomeProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SarManager(&self) -> windows_core::Result<MobileBroadbandSarManager> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandModemConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SarManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandModemConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandModemConfiguration>();
}
unsafe impl windows_core::Interface for MobileBroadbandModemConfiguration {
    type Vtable = IMobileBroadbandModemConfiguration_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandModemConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandModemConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemConfiguration";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandModemIsolation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandModemIsolation, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandModemIsolation {
    pub fn AddAllowedHost<P0>(&self, host: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddAllowedHost)(windows_core::Interface::as_raw(this), host.param().abi()).ok() }
    }
    pub fn AddAllowedHostRange<P0, P1>(&self, first: P0, last: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HostName>,
        P1: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddAllowedHostRange)(windows_core::Interface::as_raw(this), first.param().abi(), last.param().abi()).ok() }
    }
    pub fn ApplyConfigurationAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyConfigurationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearConfigurationAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearConfigurationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(modemdeviceid: &windows_core::HSTRING, rulegroupid: &windows_core::HSTRING) -> windows_core::Result<MobileBroadbandModemIsolation> {
        Self::IMobileBroadbandModemIsolationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(modemdeviceid), core::mem::transmute_copy(rulegroupid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandModemIsolationFactory<R, F: FnOnce(&IMobileBroadbandModemIsolationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandModemIsolation, IMobileBroadbandModemIsolationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandModemIsolation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandModemIsolation>();
}
unsafe impl windows_core::Interface for MobileBroadbandModemIsolation {
    type Vtable = IMobileBroadbandModemIsolation_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandModemIsolation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandModemIsolation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemIsolation";
}
unsafe impl Send for MobileBroadbandModemIsolation {}
unsafe impl Sync for MobileBroadbandModemIsolation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandNetwork(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandNetwork, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandNetwork {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> windows_core::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAdapter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NetworkRegistrationState(&self) -> windows_core::Result<NetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkRegistrationState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RegistrationNetworkError(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegistrationNetworkError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PacketAttachNetworkError(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PacketAttachNetworkError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActivationNetworkError(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivationNetworkError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccessPointName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessPointName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisteredDataClass(&self) -> windows_core::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisteredDataClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RegisteredProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisteredProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisteredProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisteredProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowConnectionUI(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowConnectionUI)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetVoiceCallSupportAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVoiceCallSupportAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegistrationUiccApps(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegistrationUiccApps)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCellsInfoAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandNetwork3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCellsInfoAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandNetwork {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandNetwork>();
}
unsafe impl windows_core::Interface for MobileBroadbandNetwork {
    type Vtable = IMobileBroadbandNetwork_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandNetwork as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandNetwork {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetwork";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandNetworkRegistrationStateChange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandNetworkRegistrationStateChange, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandNetworkRegistrationStateChange {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Network(&self) -> windows_core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Network)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandNetworkRegistrationStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandNetworkRegistrationStateChange>();
}
unsafe impl windows_core::Interface for MobileBroadbandNetworkRegistrationStateChange {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChange_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandNetworkRegistrationStateChange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandNetworkRegistrationStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChange";
}
unsafe impl Send for MobileBroadbandNetworkRegistrationStateChange {}
unsafe impl Sync for MobileBroadbandNetworkRegistrationStateChange {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandNetworkRegistrationStateChangeTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandNetworkRegistrationStateChangeTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn NetworkRegistrationStateChanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkRegistrationStateChanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails>();
}
unsafe impl windows_core::Interface for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChangeTriggerDetails";
}
unsafe impl Send for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
unsafe impl Sync for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPco(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPco, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandPco {
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandPco {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandPco>();
}
unsafe impl windows_core::Interface for MobileBroadbandPco {
    type Vtable = IMobileBroadbandPco_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandPco as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPco {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPco";
}
unsafe impl Send for MobileBroadbandPco {}
unsafe impl Sync for MobileBroadbandPco {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPcoDataChangeTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPcoDataChangeTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandPcoDataChangeTriggerDetails {
    pub fn UpdatedData(&self) -> windows_core::Result<MobileBroadbandPco> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdatedData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandPcoDataChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandPcoDataChangeTriggerDetails>();
}
unsafe impl windows_core::Interface for MobileBroadbandPcoDataChangeTriggerDetails {
    type Vtable = IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandPcoDataChangeTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPcoDataChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPcoDataChangeTriggerDetails";
}
unsafe impl Send for MobileBroadbandPcoDataChangeTriggerDetails {}
unsafe impl Sync for MobileBroadbandPcoDataChangeTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPin(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPin, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandPin {
    pub fn Type(&self) -> windows_core::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LockState(&self) -> windows_core::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Format(&self) -> windows_core::Result<MobileBroadbandPinFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AttemptsRemaining(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttemptsRemaining)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnableAsync(&self, currentpin: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(currentpin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisableAsync(&self, currentpin: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(currentpin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnterAsync(&self, currentpin: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(currentpin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChangeAsync(&self, currentpin: &windows_core::HSTRING, newpin: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(currentpin), core::mem::transmute_copy(newpin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnblockAsync(&self, pinunblockkey: &windows_core::HSTRING, newpin: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnblockAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pinunblockkey), core::mem::transmute_copy(newpin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandPin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandPin>();
}
unsafe impl windows_core::Interface for MobileBroadbandPin {
    type Vtable = IMobileBroadbandPin_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandPin as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPin {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPin";
}
unsafe impl Send for MobileBroadbandPin {}
unsafe impl Sync for MobileBroadbandPin {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPinLockStateChange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPinLockStateChange, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandPinLockStateChange {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PinType(&self) -> windows_core::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PinLockState(&self) -> windows_core::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinLockState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinLockStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandPinLockStateChange>();
}
unsafe impl windows_core::Interface for MobileBroadbandPinLockStateChange {
    type Vtable = IMobileBroadbandPinLockStateChange_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandPinLockStateChange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPinLockStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChange";
}
unsafe impl Send for MobileBroadbandPinLockStateChange {}
unsafe impl Sync for MobileBroadbandPinLockStateChange {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPinLockStateChangeTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPinLockStateChangeTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandPinLockStateChangeTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn PinLockStateChanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinLockStateChanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinLockStateChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandPinLockStateChangeTriggerDetails>();
}
unsafe impl windows_core::Interface for MobileBroadbandPinLockStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandPinLockStateChangeTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPinLockStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChangeTriggerDetails";
}
unsafe impl Send for MobileBroadbandPinLockStateChangeTriggerDetails {}
unsafe impl Sync for MobileBroadbandPinLockStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPinManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPinManager, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandPinManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPins(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPins)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPin(&self, pintype: MobileBroadbandPinType) -> windows_core::Result<MobileBroadbandPin> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPin)(windows_core::Interface::as_raw(this), pintype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandPinManager>();
}
unsafe impl windows_core::Interface for MobileBroadbandPinManager {
    type Vtable = IMobileBroadbandPinManager_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandPinManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPinManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinManager";
}
unsafe impl Send for MobileBroadbandPinManager {}
unsafe impl Sync for MobileBroadbandPinManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPinOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPinOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandPinOperationResult {
    pub fn IsSuccessful(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSuccessful)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AttemptsRemaining(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttemptsRemaining)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandPinOperationResult>();
}
unsafe impl windows_core::Interface for MobileBroadbandPinOperationResult {
    type Vtable = IMobileBroadbandPinOperationResult_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandPinOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPinOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinOperationResult";
}
unsafe impl Send for MobileBroadbandPinOperationResult {}
unsafe impl Sync for MobileBroadbandPinOperationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandRadioStateChange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandRadioStateChange, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandRadioStateChange {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RadioState(&self) -> windows_core::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RadioState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandRadioStateChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandRadioStateChange>();
}
unsafe impl windows_core::Interface for MobileBroadbandRadioStateChange {
    type Vtable = IMobileBroadbandRadioStateChange_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandRadioStateChange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandRadioStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChange";
}
unsafe impl Send for MobileBroadbandRadioStateChange {}
unsafe impl Sync for MobileBroadbandRadioStateChange {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandRadioStateChangeTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandRadioStateChangeTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandRadioStateChangeTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn RadioStateChanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RadioStateChanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandRadioStateChangeTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandRadioStateChangeTriggerDetails>();
}
unsafe impl windows_core::Interface for MobileBroadbandRadioStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandRadioStateChangeTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandRadioStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChangeTriggerDetails";
}
unsafe impl Send for MobileBroadbandRadioStateChangeTriggerDetails {}
unsafe impl Sync for MobileBroadbandRadioStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandSarManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandSarManager, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandSarManager {
    pub fn IsBackoffEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBackoffEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsWiFiHardwareIntegrated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWiFiHardwareIntegrated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSarControlledByHardware(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSarControlledByHardware)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Antennas(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Antennas)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HysteresisTimerPeriod(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HysteresisTimerPeriod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransmissionStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmissionStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTransmissionStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTransmissionStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnableBackoffAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableBackoffAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisableBackoffAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableBackoffAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetConfigurationAsync<P0>(&self, antennas: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetConfigurationAsync)(windows_core::Interface::as_raw(this), antennas.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RevertSarToHardwareControlAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RevertSarToHardwareControlAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransmissionStateChangedHysteresisAsync(&self, timerperiod: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTransmissionStateChangedHysteresisAsync)(windows_core::Interface::as_raw(this), timerperiod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIsTransmittingAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsTransmittingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartTransmissionStateMonitoring(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartTransmissionStateMonitoring)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopTransmissionStateMonitoring(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopTransmissionStateMonitoring)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for MobileBroadbandSarManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandSarManager>();
}
unsafe impl windows_core::Interface for MobileBroadbandSarManager {
    type Vtable = IMobileBroadbandSarManager_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandSarManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandSarManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSarManager";
}
unsafe impl Send for MobileBroadbandSarManager {}
unsafe impl Sync for MobileBroadbandSarManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandSlotInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandSlotInfo, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandSlotInfo {
    pub fn Index(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Index)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<MobileBroadbandSlotState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMobileBroadbandSlotInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IccId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandSlotInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandSlotInfo>();
}
unsafe impl windows_core::Interface for MobileBroadbandSlotInfo {
    type Vtable = IMobileBroadbandSlotInfo_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandSlotInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandSlotInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfo";
}
unsafe impl Send for MobileBroadbandSlotInfo {}
unsafe impl Sync for MobileBroadbandSlotInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandSlotInfoChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandSlotInfoChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandSlotInfoChangedEventArgs {
    pub fn SlotInfo(&self) -> windows_core::Result<MobileBroadbandSlotInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlotInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandSlotInfoChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandSlotInfoChangedEventArgs>();
}
unsafe impl windows_core::Interface for MobileBroadbandSlotInfoChangedEventArgs {
    type Vtable = IMobileBroadbandSlotInfoChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandSlotInfoChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandSlotInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfoChangedEventArgs";
}
unsafe impl Send for MobileBroadbandSlotInfoChangedEventArgs {}
unsafe impl Sync for MobileBroadbandSlotInfoChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandSlotManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandSlotManager, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandSlotManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SlotInfos(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlotInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentSlotIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentSlotIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCurrentSlot(&self, slotindex: i32) -> windows_core::Result<MobileBroadbandModemStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetCurrentSlot)(windows_core::Interface::as_raw(this), slotindex, &mut result__).map(|| result__)
        }
    }
    pub fn SetCurrentSlotAsync(&self, slotindex: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetCurrentSlotAsync)(windows_core::Interface::as_raw(this), slotindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SlotInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlotInfoChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSlotInfoChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSlotInfoChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CurrentSlotIndexChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentSlotIndexChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCurrentSlotIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCurrentSlotIndexChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MobileBroadbandSlotManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandSlotManager>();
}
unsafe impl windows_core::Interface for MobileBroadbandSlotManager {
    type Vtable = IMobileBroadbandSlotManager_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandSlotManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandSlotManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotManager";
}
unsafe impl Send for MobileBroadbandSlotManager {}
unsafe impl Sync for MobileBroadbandSlotManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandTransmissionStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandTransmissionStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandTransmissionStateChangedEventArgs {
    pub fn IsTransmitting(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTransmitting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandTransmissionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandTransmissionStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for MobileBroadbandTransmissionStateChangedEventArgs {
    type Vtable = IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandTransmissionStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandTransmissionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandTransmissionStateChangedEventArgs";
}
unsafe impl Send for MobileBroadbandTransmissionStateChangedEventArgs {}
unsafe impl Sync for MobileBroadbandTransmissionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandUicc(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandUicc, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandUicc {
    pub fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimIccId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetUiccAppsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUiccAppsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandUicc {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandUicc>();
}
unsafe impl windows_core::Interface for MobileBroadbandUicc {
    type Vtable = IMobileBroadbandUicc_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandUicc as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandUicc {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUicc";
}
unsafe impl Send for MobileBroadbandUicc {}
unsafe impl Sync for MobileBroadbandUicc {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandUiccApp(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandUiccApp, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandUiccApp {
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<UiccAppKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecordDetailsAsync<P0>(&self, uiccfilepath: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<u32>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecordDetailsAsync)(windows_core::Interface::as_raw(this), uiccfilepath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadRecordAsync<P0>(&self, uiccfilepath: P0, recordindex: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<u32>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadRecordAsync)(windows_core::Interface::as_raw(this), uiccfilepath.param().abi(), recordindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandUiccApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandUiccApp>();
}
unsafe impl windows_core::Interface for MobileBroadbandUiccApp {
    type Vtable = IMobileBroadbandUiccApp_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandUiccApp as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandUiccApp {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccApp";
}
unsafe impl Send for MobileBroadbandUiccApp {}
unsafe impl Sync for MobileBroadbandUiccApp {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandUiccAppReadRecordResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandUiccAppReadRecordResult, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandUiccAppReadRecordResult {
    pub fn Status(&self) -> windows_core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandUiccAppReadRecordResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandUiccAppReadRecordResult>();
}
unsafe impl windows_core::Interface for MobileBroadbandUiccAppReadRecordResult {
    type Vtable = IMobileBroadbandUiccAppReadRecordResult_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandUiccAppReadRecordResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandUiccAppReadRecordResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppReadRecordResult";
}
unsafe impl Send for MobileBroadbandUiccAppReadRecordResult {}
unsafe impl Sync for MobileBroadbandUiccAppReadRecordResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandUiccAppRecordDetailsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandUiccAppRecordDetailsResult, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandUiccAppRecordDetailsResult {
    pub fn Status(&self) -> windows_core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<UiccAppRecordKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RecordCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RecordSize(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadAccessCondition(&self) -> windows_core::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAccessCondition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WriteAccessCondition(&self) -> windows_core::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAccessCondition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandUiccAppRecordDetailsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandUiccAppRecordDetailsResult>();
}
unsafe impl windows_core::Interface for MobileBroadbandUiccAppRecordDetailsResult {
    type Vtable = IMobileBroadbandUiccAppRecordDetailsResult_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandUiccAppRecordDetailsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandUiccAppRecordDetailsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppRecordDetailsResult";
}
unsafe impl Send for MobileBroadbandUiccAppRecordDetailsResult {}
unsafe impl Sync for MobileBroadbandUiccAppRecordDetailsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandUiccAppsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandUiccAppsResult, windows_core::IUnknown, windows_core::IInspectable);
impl MobileBroadbandUiccAppsResult {
    pub fn Status(&self) -> windows_core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UiccApps(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UiccApps)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MobileBroadbandUiccAppsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMobileBroadbandUiccAppsResult>();
}
unsafe impl windows_core::Interface for MobileBroadbandUiccAppsResult {
    type Vtable = IMobileBroadbandUiccAppsResult_Vtbl;
    const IID: windows_core::GUID = <IMobileBroadbandUiccAppsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandUiccAppsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppsResult";
}
unsafe impl Send for MobileBroadbandUiccAppsResult {}
unsafe impl Sync for MobileBroadbandUiccAppsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorDataUsageTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorDataUsageTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkOperatorDataUsageTriggerDetails {
    pub fn NotificationKind(&self) -> windows_core::Result<NetworkOperatorDataUsageNotificationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NetworkOperatorDataUsageTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorDataUsageTriggerDetails>();
}
unsafe impl windows_core::Interface for NetworkOperatorDataUsageTriggerDetails {
    type Vtable = INetworkOperatorDataUsageTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorDataUsageTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorDataUsageTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorDataUsageTriggerDetails";
}
unsafe impl Send for NetworkOperatorDataUsageTriggerDetails {}
unsafe impl Sync for NetworkOperatorDataUsageTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorNotificationEventDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorNotificationEventDetails, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkOperatorNotificationEventDetails {
    pub fn NotificationType(&self) -> windows_core::Result<NetworkOperatorEventMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NetworkAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EncodingType(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RuleId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RuleId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Sms")]
    pub fn SmsMessage(&self) -> windows_core::Result<super::super::Devices::Sms::ISmsMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmsMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AuthorizeTethering(&self, allow: bool, entitlementfailurereason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INetworkOperatorTetheringEntitlementCheck>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AuthorizeTethering)(windows_core::Interface::as_raw(this), allow, core::mem::transmute_copy(entitlementfailurereason)).ok() }
    }
}
impl windows_core::RuntimeType for NetworkOperatorNotificationEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorNotificationEventDetails>();
}
unsafe impl windows_core::Interface for NetworkOperatorNotificationEventDetails {
    type Vtable = INetworkOperatorNotificationEventDetails_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorNotificationEventDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorNotificationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorNotificationEventDetails";
}
unsafe impl Send for NetworkOperatorNotificationEventDetails {}
unsafe impl Sync for NetworkOperatorNotificationEventDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorTetheringAccessPointConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringAccessPointConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkOperatorTetheringAccessPointConfiguration {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorTetheringAccessPointConfiguration, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Ssid(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ssid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSsid(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSsid)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Passphrase(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Passphrase)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPassphrase(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPassphrase)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsBandSupported(&self, band: TetheringWiFiBand) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBandSupported)(windows_core::Interface::as_raw(this), band, &mut result__).map(|| result__)
        }
    }
    pub fn IsBandSupportedAsync(&self, band: TetheringWiFiBand) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBandSupportedAsync)(windows_core::Interface::as_raw(this), band, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Band(&self) -> windows_core::Result<TetheringWiFiBand> {
        let this = &windows_core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Band)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBand(&self, value: TetheringWiFiBand) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBand)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for NetworkOperatorTetheringAccessPointConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorTetheringAccessPointConfiguration>();
}
unsafe impl windows_core::Interface for NetworkOperatorTetheringAccessPointConfiguration {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorTetheringAccessPointConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorTetheringAccessPointConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringAccessPointConfiguration";
}
unsafe impl Send for NetworkOperatorTetheringAccessPointConfiguration {}
unsafe impl Sync for NetworkOperatorTetheringAccessPointConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorTetheringClient(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringClient, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkOperatorTetheringClient {
    pub fn MacAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MacAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HostNames(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NetworkOperatorTetheringClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorTetheringClient>();
}
unsafe impl windows_core::Interface for NetworkOperatorTetheringClient {
    type Vtable = INetworkOperatorTetheringClient_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorTetheringClient as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorTetheringClient {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringClient";
}
unsafe impl Send for NetworkOperatorTetheringClient {}
unsafe impl Sync for NetworkOperatorTetheringClient {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorTetheringManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringManager, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkOperatorTetheringManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTetheringClients(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>> {
        let this = &windows_core::Interface::cast::<INetworkOperatorTetheringClientManager>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTetheringClients)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxClientCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxClientCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ClientCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TetheringOperationalState(&self) -> windows_core::Result<TetheringOperationalState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TetheringOperationalState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentAccessPointConfiguration(&self) -> windows_core::Result<NetworkOperatorTetheringAccessPointConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentAccessPointConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfigureAccessPointAsync<P0>(&self, configuration: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<NetworkOperatorTetheringAccessPointConfiguration>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConfigureAccessPointAsync)(windows_core::Interface::as_raw(this), configuration.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartTetheringAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTetheringAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopTetheringAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopTetheringAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTetheringCapability(networkaccountid: &windows_core::HSTRING) -> windows_core::Result<TetheringCapability> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTetheringCapability)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkaccountid), &mut result__).map(|| result__)
        })
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &windows_core::HSTRING) -> windows_core::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn GetTetheringCapabilityFromConnectionProfile<P0>(profile: P0) -> windows_core::Result<TetheringCapability>
    where
        P0: windows_core::Param<super::Connectivity::ConnectionProfile>,
    {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTetheringCapabilityFromConnectionProfile)(windows_core::Interface::as_raw(this), profile.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfile<P0>(profile: P0) -> windows_core::Result<NetworkOperatorTetheringManager>
    where
        P0: windows_core::Param<super::Connectivity::ConnectionProfile>,
    {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromConnectionProfile)(windows_core::Interface::as_raw(this), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfileWithTargetAdapter<P0, P1>(profile: P0, adapter: P1) -> windows_core::Result<NetworkOperatorTetheringManager>
    where
        P0: windows_core::Param<super::Connectivity::ConnectionProfile>,
        P1: windows_core::Param<super::Connectivity::NetworkAdapter>,
    {
        Self::INetworkOperatorTetheringManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromConnectionProfileWithTargetAdapter)(windows_core::Interface::as_raw(this), profile.param().abi(), adapter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsNoConnectionsTimeoutEnabled() -> windows_core::Result<bool> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNoConnectionsTimeoutEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EnableNoConnectionsTimeout() -> windows_core::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (windows_core::Interface::vtable(this).EnableNoConnectionsTimeout)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn EnableNoConnectionsTimeoutAsync() -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnableNoConnectionsTimeoutAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DisableNoConnectionsTimeout() -> windows_core::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (windows_core::Interface::vtable(this).DisableNoConnectionsTimeout)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn DisableNoConnectionsTimeoutAsync() -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableNoConnectionsTimeoutAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics2<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics3<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics4<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NetworkOperatorTetheringManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorTetheringManager>();
}
unsafe impl windows_core::Interface for NetworkOperatorTetheringManager {
    type Vtable = INetworkOperatorTetheringManager_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorTetheringManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorTetheringManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorTetheringOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkOperatorTetheringOperationResult {
    pub fn Status(&self) -> windows_core::Result<TetheringOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdditionalErrorMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdditionalErrorMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NetworkOperatorTetheringOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorTetheringOperationResult>();
}
unsafe impl windows_core::Interface for NetworkOperatorTetheringOperationResult {
    type Vtable = INetworkOperatorTetheringOperationResult_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorTetheringOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorTetheringOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringOperationResult";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProvisionFromXmlDocumentResults(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProvisionFromXmlDocumentResults, windows_core::IUnknown, windows_core::IInspectable);
impl ProvisionFromXmlDocumentResults {
    pub fn AllElementsProvisioned(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllElementsProvisioned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProvisionResultsXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProvisionResultsXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProvisionFromXmlDocumentResults {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProvisionFromXmlDocumentResults>();
}
unsafe impl windows_core::Interface for ProvisionFromXmlDocumentResults {
    type Vtable = IProvisionFromXmlDocumentResults_Vtbl;
    const IID: windows_core::GUID = <IProvisionFromXmlDocumentResults as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProvisionFromXmlDocumentResults {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionFromXmlDocumentResults";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProvisionedProfile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProvisionedProfile, windows_core::IUnknown, windows_core::IInspectable);
impl ProvisionedProfile {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn UpdateCost(&self, value: super::Connectivity::NetworkCostType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateCost)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateUsage(&self, value: ProfileUsage) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateUsage)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ProvisionedProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProvisionedProfile>();
}
unsafe impl windows_core::Interface for ProvisionedProfile {
    type Vtable = IProvisionedProfile_Vtbl;
    const IID: windows_core::GUID = <IProvisionedProfile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProvisionedProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionedProfile";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProvisioningAgent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProvisioningAgent, windows_core::IUnknown, windows_core::IInspectable);
impl ProvisioningAgent {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProvisioningAgent, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProvisionFromXmlDocumentAsync(&self, provisioningxmldocument: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProvisionFromXmlDocumentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(provisioningxmldocument), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetProvisionedProfile(&self, mediatype: ProfileMediaType, profilename: &windows_core::HSTRING) -> windows_core::Result<ProvisionedProfile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProvisionedProfile)(windows_core::Interface::as_raw(this), mediatype, core::mem::transmute_copy(profilename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &windows_core::HSTRING) -> windows_core::Result<ProvisioningAgent> {
        Self::IProvisioningAgentStaticMethods(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IProvisioningAgentStaticMethods<R, F: FnOnce(&IProvisioningAgentStaticMethods) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProvisioningAgent, IProvisioningAgentStaticMethods> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProvisioningAgent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProvisioningAgent>();
}
unsafe impl windows_core::Interface for ProvisioningAgent {
    type Vtable = IProvisioningAgent_Vtbl;
    const IID: windows_core::GUID = <IProvisioningAgent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProvisioningAgent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisioningAgent";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TetheringEntitlementCheckTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TetheringEntitlementCheckTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl TetheringEntitlementCheckTriggerDetails {
    pub fn NetworkAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AllowTethering(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AllowTethering)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DenyTethering(&self, entitlementfailurereason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DenyTethering)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(entitlementfailurereason)).ok() }
    }
}
impl windows_core::RuntimeType for TetheringEntitlementCheckTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITetheringEntitlementCheckTriggerDetails>();
}
unsafe impl windows_core::Interface for TetheringEntitlementCheckTriggerDetails {
    type Vtable = ITetheringEntitlementCheckTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <ITetheringEntitlementCheckTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TetheringEntitlementCheckTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.TetheringEntitlementCheckTriggerDetails";
}
unsafe impl Send for TetheringEntitlementCheckTriggerDetails {}
unsafe impl Sync for TetheringEntitlementCheckTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UssdMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UssdMessage, windows_core::IUnknown, windows_core::IInspectable);
impl UssdMessage {
    pub fn DataCodingScheme(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataCodingScheme)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDataCodingScheme(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataCodingScheme)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPayload(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetPayload)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetPayload(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPayload)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn PayloadAsText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PayloadAsText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPayloadAsText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPayloadAsText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateMessage(messagetext: &windows_core::HSTRING) -> windows_core::Result<UssdMessage> {
        Self::IUssdMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(messagetext), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUssdMessageFactory<R, F: FnOnce(&IUssdMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UssdMessage, IUssdMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UssdMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUssdMessage>();
}
unsafe impl windows_core::Interface for UssdMessage {
    type Vtable = IUssdMessage_Vtbl;
    const IID: windows_core::GUID = <IUssdMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UssdMessage {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdMessage";
}
unsafe impl Send for UssdMessage {}
unsafe impl Sync for UssdMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UssdReply(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UssdReply, windows_core::IUnknown, windows_core::IInspectable);
impl UssdReply {
    pub fn ResultCode(&self) -> windows_core::Result<UssdResultCode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResultCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<UssdMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UssdReply {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUssdReply>();
}
unsafe impl windows_core::Interface for UssdReply {
    type Vtable = IUssdReply_Vtbl;
    const IID: windows_core::GUID = <IUssdReply as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UssdReply {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdReply";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UssdSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UssdSession, windows_core::IUnknown, windows_core::IInspectable);
impl UssdSession {
    pub fn SendMessageAndGetReplyAsync<P0>(&self, message: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UssdReply>>
    where
        P0: windows_core::Param<UssdMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendMessageAndGetReplyAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &windows_core::HSTRING) -> windows_core::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNetworkInterfaceId(networkinterfaceid: &windows_core::HSTRING) -> windows_core::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNetworkInterfaceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkinterfaceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUssdSessionStatics<R, F: FnOnce(&IUssdSessionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UssdSession, IUssdSessionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UssdSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUssdSession>();
}
unsafe impl windows_core::Interface for UssdSession {
    type Vtable = IUssdSession_Vtbl;
    const IID: windows_core::GUID = <IUssdSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UssdSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdSession";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DataClasses(pub u32);
impl DataClasses {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const NewRadioNonStandalone: Self = Self(64u32);
    pub const NewRadioStandalone: Self = Self(128u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl windows_core::TypeKind for DataClasses {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DataClasses {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DataClasses").field(&self.0).finish()
    }
}
impl DataClasses {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DataClasses {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DataClasses {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DataClasses {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DataClasses {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DataClasses {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DataClasses {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.DataClasses;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: Self = Self(0i32);
    pub const OnAction: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl windows_core::TypeKind for ESimAuthenticationPreference {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimAuthenticationPreference {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimAuthenticationPreference").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimAuthenticationPreference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimAuthenticationPreference;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: Self = Self(0i32);
    pub const Events: Self = Self(1i32);
    pub const ProfileMetadata: Self = Self(2i32);
}
impl windows_core::TypeKind for ESimDiscoverResultKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimDiscoverResultKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimDiscoverResultKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimDiscoverResultKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimDiscoverResultKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimOperationStatus(pub i32);
impl ESimOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAuthorized: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const PolicyViolation: Self = Self(3i32);
    pub const InsufficientSpaceOnCard: Self = Self(4i32);
    pub const ServerFailure: Self = Self(5i32);
    pub const ServerNotReachable: Self = Self(6i32);
    pub const TimeoutWaitingForUserConsent: Self = Self(7i32);
    pub const IncorrectConfirmationCode: Self = Self(8i32);
    pub const ConfirmationCodeMaxRetriesExceeded: Self = Self(9i32);
    pub const CardRemoved: Self = Self(10i32);
    pub const CardBusy: Self = Self(11i32);
    pub const Other: Self = Self(12i32);
    pub const CardGeneralFailure: Self = Self(13i32);
    pub const ConfirmationCodeMissing: Self = Self(14i32);
    pub const InvalidMatchingId: Self = Self(15i32);
    pub const NoEligibleProfileForThisDevice: Self = Self(16i32);
    pub const OperationAborted: Self = Self(17i32);
    pub const EidMismatch: Self = Self(18i32);
    pub const ProfileNotAvailableForNewBinding: Self = Self(19i32);
    pub const ProfileNotReleasedByOperator: Self = Self(20i32);
    pub const OperationProhibitedByProfileClass: Self = Self(21i32);
    pub const ProfileNotPresent: Self = Self(22i32);
    pub const NoCorrespondingRequest: Self = Self(23i32);
    pub const TimeoutWaitingForResponse: Self = Self(24i32);
    pub const IccidAlreadyExists: Self = Self(25i32);
    pub const ProfileProcessingError: Self = Self(26i32);
    pub const ServerNotTrusted: Self = Self(27i32);
    pub const ProfileDownloadMaxRetriesExceeded: Self = Self(28i32);
}
impl windows_core::TypeKind for ESimOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimOperationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: Self = Self(0i32);
    pub const Test: Self = Self(1i32);
    pub const Provisioning: Self = Self(2i32);
}
impl windows_core::TypeKind for ESimProfileClass {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimProfileClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimProfileClass").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimProfileClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileClass;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimProfileMetadataState(pub i32);
impl ESimProfileMetadataState {
    pub const Unknown: Self = Self(0i32);
    pub const WaitingForInstall: Self = Self(1i32);
    pub const Downloading: Self = Self(2i32);
    pub const Installing: Self = Self(3i32);
    pub const Expired: Self = Self(4i32);
    pub const RejectingDownload: Self = Self(5i32);
    pub const NoLongerAvailable: Self = Self(6i32);
    pub const DeniedByPolicy: Self = Self(7i32);
}
impl windows_core::TypeKind for ESimProfileMetadataState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimProfileMetadataState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimProfileMetadataState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimProfileMetadataState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileMetadataState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const Deleted: Self = Self(3i32);
}
impl windows_core::TypeKind for ESimProfileState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimProfileState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimProfileState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimProfileState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Removed: Self = Self(2i32);
    pub const Busy: Self = Self(3i32);
}
impl windows_core::TypeKind for ESimState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
}
impl windows_core::TypeKind for ESimWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ESimWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ESimWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ESimWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HotspotAuthenticationResponseCode(pub i32);
impl HotspotAuthenticationResponseCode {
    pub const NoError: Self = Self(0i32);
    pub const LoginSucceeded: Self = Self(50i32);
    pub const LoginFailed: Self = Self(100i32);
    pub const RadiusServerError: Self = Self(102i32);
    pub const NetworkAdministratorError: Self = Self(105i32);
    pub const LoginAborted: Self = Self(151i32);
    pub const AccessGatewayInternalError: Self = Self(255i32);
}
impl windows_core::TypeKind for HotspotAuthenticationResponseCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HotspotAuthenticationResponseCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationResponseCode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HotspotAuthenticationResponseCode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.HotspotAuthenticationResponseCode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandAccountWatcherStatus(pub i32);
impl MobileBroadbandAccountWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl windows_core::TypeKind for MobileBroadbandAccountWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandAccountWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandAccountWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandDeviceType(pub i32);
impl MobileBroadbandDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Embedded: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
    pub const Remote: Self = Self(3i32);
}
impl windows_core::TypeKind for MobileBroadbandDeviceType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandDeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandDeviceType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandModemStatus(pub i32);
impl MobileBroadbandModemStatus {
    pub const Success: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const Busy: Self = Self(2i32);
    pub const NoDeviceSupport: Self = Self(3i32);
}
impl windows_core::TypeKind for MobileBroadbandModemStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandModemStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandModemStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandModemStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandPinFormat(pub i32);
impl MobileBroadbandPinFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Numeric: Self = Self(1i32);
    pub const Alphanumeric: Self = Self(2i32);
}
impl windows_core::TypeKind for MobileBroadbandPinFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandPinFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: Self = Self(0i32);
    pub const Unlocked: Self = Self(1i32);
    pub const PinRequired: Self = Self(2i32);
    pub const PinUnblockKeyRequired: Self = Self(3i32);
}
impl windows_core::TypeKind for MobileBroadbandPinLockState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandPinLockState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinLockState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinLockState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandPinType(pub i32);
impl MobileBroadbandPinType {
    pub const None: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
    pub const Pin1: Self = Self(2i32);
    pub const Pin2: Self = Self(3i32);
    pub const SimPin: Self = Self(4i32);
    pub const FirstSimPin: Self = Self(5i32);
    pub const NetworkPin: Self = Self(6i32);
    pub const NetworkSubsetPin: Self = Self(7i32);
    pub const ServiceProviderPin: Self = Self(8i32);
    pub const CorporatePin: Self = Self(9i32);
    pub const SubsidyLock: Self = Self(10i32);
}
impl windows_core::TypeKind for MobileBroadbandPinType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandPinType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl windows_core::TypeKind for MobileBroadbandRadioState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandRadioState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandRadioState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandRadioState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandSlotState(pub i32);
impl MobileBroadbandSlotState {
    pub const Unmanaged: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const OffEmpty: Self = Self(2i32);
    pub const Off: Self = Self(3i32);
    pub const Empty: Self = Self(4i32);
    pub const NotReady: Self = Self(5i32);
    pub const Active: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const ActiveEsim: Self = Self(8i32);
    pub const ActiveEsimNoProfile: Self = Self(9i32);
}
impl windows_core::TypeKind for MobileBroadbandSlotState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandSlotState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandSlotState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandSlotState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MobileBroadbandUiccAppOperationStatus(pub i32);
impl MobileBroadbandUiccAppOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidUiccFilePath: Self = Self(1i32);
    pub const AccessConditionNotHeld: Self = Self(2i32);
    pub const UiccBusy: Self = Self(3i32);
}
impl windows_core::TypeKind for MobileBroadbandUiccAppOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MobileBroadbandUiccAppOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppOperationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MobileBroadbandUiccAppOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkDeviceStatus(pub i32);
impl NetworkDeviceStatus {
    pub const DeviceNotReady: Self = Self(0i32);
    pub const DeviceReady: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceHardwareFailure: Self = Self(4i32);
    pub const AccountNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl windows_core::TypeKind for NetworkDeviceStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkDeviceStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkDeviceStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkDeviceStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkDeviceStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: Self = Self(0i32);
}
impl windows_core::TypeKind for NetworkOperatorDataUsageNotificationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkOperatorDataUsageNotificationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkOperatorDataUsageNotificationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkOperatorDataUsageNotificationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorDataUsageNotificationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkOperatorEventMessageType(pub i32);
impl NetworkOperatorEventMessageType {
    pub const Gsm: Self = Self(0i32);
    pub const Cdma: Self = Self(1i32);
    pub const Ussd: Self = Self(2i32);
    pub const DataPlanThresholdReached: Self = Self(3i32);
    pub const DataPlanReset: Self = Self(4i32);
    pub const DataPlanDeleted: Self = Self(5i32);
    pub const ProfileConnected: Self = Self(6i32);
    pub const ProfileDisconnected: Self = Self(7i32);
    pub const RegisteredRoaming: Self = Self(8i32);
    pub const RegisteredHome: Self = Self(9i32);
    pub const TetheringEntitlementCheck: Self = Self(10i32);
    pub const TetheringOperationalStateChanged: Self = Self(11i32);
    pub const TetheringNumberOfClientsChanged: Self = Self(12i32);
}
impl windows_core::TypeKind for NetworkOperatorEventMessageType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkOperatorEventMessageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkOperatorEventMessageType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkOperatorEventMessageType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorEventMessageType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkRegistrationState(pub i32);
impl NetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl windows_core::TypeKind for NetworkRegistrationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkRegistrationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkRegistrationState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkRegistrationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkRegistrationState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: Self = Self(0i32);
    pub const Wwan: Self = Self(1i32);
}
impl windows_core::TypeKind for ProfileMediaType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ProfileMediaType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ProfileMediaType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ProfileMediaType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ProfileMediaType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TetheringCapability(pub i32);
impl TetheringCapability {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledByGroupPolicy: Self = Self(1i32);
    pub const DisabledByHardwareLimitation: Self = Self(2i32);
    pub const DisabledByOperator: Self = Self(3i32);
    pub const DisabledBySku: Self = Self(4i32);
    pub const DisabledByRequiredAppNotInstalled: Self = Self(5i32);
    pub const DisabledDueToUnknownCause: Self = Self(6i32);
    pub const DisabledBySystemCapability: Self = Self(7i32);
}
impl windows_core::TypeKind for TetheringCapability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TetheringCapability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TetheringCapability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TetheringCapability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringCapability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TetheringOperationStatus(pub i32);
impl TetheringOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const MobileBroadbandDeviceOff: Self = Self(2i32);
    pub const WiFiDeviceOff: Self = Self(3i32);
    pub const EntitlementCheckTimeout: Self = Self(4i32);
    pub const EntitlementCheckFailure: Self = Self(5i32);
    pub const OperationInProgress: Self = Self(6i32);
    pub const BluetoothDeviceOff: Self = Self(7i32);
    pub const NetworkLimitedConnectivity: Self = Self(8i32);
}
impl windows_core::TypeKind for TetheringOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TetheringOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TetheringOperationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TetheringOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const InTransition: Self = Self(3i32);
}
impl windows_core::TypeKind for TetheringOperationalState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TetheringOperationalState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TetheringOperationalState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TetheringOperationalState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationalState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: Self = Self(0i32);
    pub const TwoPointFourGigahertz: Self = Self(1i32);
    pub const FiveGigahertz: Self = Self(2i32);
}
impl windows_core::TypeKind for TetheringWiFiBand {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TetheringWiFiBand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TetheringWiFiBand").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TetheringWiFiBand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringWiFiBand;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UiccAccessCondition(pub i32);
impl UiccAccessCondition {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const Pin1: Self = Self(1i32);
    pub const Pin2: Self = Self(2i32);
    pub const Pin3: Self = Self(3i32);
    pub const Pin4: Self = Self(4i32);
    pub const Administrative5: Self = Self(5i32);
    pub const Administrative6: Self = Self(6i32);
    pub const NeverAllowed: Self = Self(7i32);
}
impl windows_core::TypeKind for UiccAccessCondition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UiccAccessCondition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UiccAccessCondition").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UiccAccessCondition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAccessCondition;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UiccAppKind(pub i32);
impl UiccAppKind {
    pub const Unknown: Self = Self(0i32);
    pub const MF: Self = Self(1i32);
    pub const MFSim: Self = Self(2i32);
    pub const MFRuim: Self = Self(3i32);
    pub const USim: Self = Self(4i32);
    pub const CSim: Self = Self(5i32);
    pub const ISim: Self = Self(6i32);
}
impl windows_core::TypeKind for UiccAppKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UiccAppKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UiccAppKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UiccAppKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: Self = Self(0i32);
    pub const Transparent: Self = Self(1i32);
    pub const RecordOriented: Self = Self(2i32);
}
impl windows_core::TypeKind for UiccAppRecordKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UiccAppRecordKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UiccAppRecordKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UiccAppRecordKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppRecordKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UssdResultCode(pub i32);
impl UssdResultCode {
    pub const NoActionRequired: Self = Self(0i32);
    pub const ActionRequired: Self = Self(1i32);
    pub const Terminated: Self = Self(2i32);
    pub const OtherLocalClient: Self = Self(3i32);
    pub const OperationNotSupported: Self = Self(4i32);
    pub const NetworkTimeout: Self = Self(5i32);
}
impl windows_core::TypeKind for UssdResultCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UssdResultCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UssdResultCode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UssdResultCode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UssdResultCode;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ESimProfileInstallProgress {
    pub TotalSizeInBytes: i32,
    pub InstalledSizeInBytes: i32,
}
impl windows_core::TypeKind for ESimProfileInstallProgress {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ESimProfileInstallProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ESimProfileInstallProgress;i4;i4)");
}
impl Default for ESimProfileInstallProgress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProfileUsage {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::DateTime,
}
impl windows_core::TypeKind for ProfileUsage {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProfileUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ProfileUsage;u4;struct(Windows.Foundation.DateTime;i8))");
}
impl Default for ProfileUsage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
