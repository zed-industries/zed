#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(ICallAnswerEventArgs, ICallAnswerEventArgs_Vtbl, 0xfd789617_2dd7_4c8c_b2bd_95d17a5bb733);
impl windows_core::RuntimeType for ICallAnswerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICallAnswerEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcceptedMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VoipPhoneCallMedia) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallRejectEventArgs, ICallRejectEventArgs_Vtbl, 0xda47fad7_13d4_4d92_a1c2_b77811ee37ec);
impl windows_core::RuntimeType for ICallRejectEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICallRejectEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RejectReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VoipPhoneCallRejectReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallStateChangeEventArgs, ICallStateChangeEventArgs_Vtbl, 0xeab2349e_66f5_47f9_9fb5_459c5198c720);
impl windows_core::RuntimeType for ICallStateChangeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICallStateChangeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VoipPhoneCallState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockScreenCallEndCallDeferral, ILockScreenCallEndCallDeferral_Vtbl, 0x2dd7ed0d_98ed_4041_9632_50ff812b773f);
impl windows_core::RuntimeType for ILockScreenCallEndCallDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILockScreenCallEndCallDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockScreenCallEndRequestedEventArgs, ILockScreenCallEndRequestedEventArgs_Vtbl, 0x8190a363_6f27_46e9_aeb6_c0ae83e47dc7);
impl windows_core::RuntimeType for ILockScreenCallEndRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILockScreenCallEndRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILockScreenCallUI, ILockScreenCallUI_Vtbl, 0xc596fd8d_73c9_4a14_b021_ec1c50a3b727);
impl windows_core::RuntimeType for ILockScreenCallUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILockScreenCallUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Dismiss: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEndRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CallTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCallTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMuteChangeEventArgs, IMuteChangeEventArgs_Vtbl, 0x8585e159_0c41_432c_814d_c5f1fdf530be);
impl windows_core::RuntimeType for IMuteChangeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMuteChangeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Muted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCall, IPhoneCall_Vtbl, 0xc14ed0f8_c17d_59d2_9628_66e545b6cd21);
impl windows_core::RuntimeType for IPhoneCall {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCall_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AudioDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsMutedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsMutedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CallId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallStatus) -> windows_core::HRESULT,
    pub AudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallAudioDevice) -> windows_core::HRESULT,
    pub GetPhoneCallInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPhoneCallInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub EndAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendDtmfKey: unsafe extern "system" fn(*mut core::ffi::c_void, DtmfKey, DtmfToneAudioPlayback, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub SendDtmfKeyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, DtmfKey, DtmfToneAudioPlayback, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub AcceptIncomingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub HoldAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeFromHold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub ResumeFromHoldAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Mute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub MuteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub UnmuteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RejectIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub RejectIncomingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangeAudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallAudioDevice, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub ChangeAudioDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallAudioDevice, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallBlockingStatics, IPhoneCallBlockingStatics_Vtbl, 0x19646f84_2b79_26f1_a46f_694be043f313);
impl windows_core::RuntimeType for IPhoneCallBlockingStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallBlockingStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BlockUnknownNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetBlockUnknownNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub BlockPrivateNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetBlockPrivateNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetCallBlockingListAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetCallBlockingListAsync: usize,
}
windows_core::imp::define_interface!(IPhoneCallHistoryEntry, IPhoneCallHistoryEntry_Vtbl, 0xfab0e129_32a4_4b85_83d1_f90d8c23a857);
impl windows_core::RuntimeType for IPhoneCallHistoryEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCallerIdBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCallerIdBlocked: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsEmergency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEmergency: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsMissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsMissed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRinging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRinging: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsSuppressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSuppressed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsVoicemail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsVoicemail: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Media: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallHistoryEntryMedia) -> windows_core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallHistoryEntryMedia) -> windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallHistoryEntryOtherAppReadAccess) -> windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallHistoryEntryOtherAppReadAccess) -> windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSourceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SourceIdKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallHistorySourceIdKind) -> windows_core::HRESULT,
    pub SetSourceIdKind: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallHistorySourceIdKind) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallHistoryEntryAddress, IPhoneCallHistoryEntryAddress_Vtbl, 0x30f159da_3955_4042_84e6_66eebf82e67f);
impl windows_core::RuntimeType for IPhoneCallHistoryEntryAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryAddress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RawAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRawAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RawAddressKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallHistoryEntryRawAddressKind) -> windows_core::HRESULT,
    pub SetRawAddressKind: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallHistoryEntryRawAddressKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallHistoryEntryAddressFactory, IPhoneCallHistoryEntryAddressFactory_Vtbl, 0xfb0fadba_c7f0_4bb6_9f6b_ba5d73209aca);
impl windows_core::RuntimeType for IPhoneCallHistoryEntryAddressFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryAddressFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, PhoneCallHistoryEntryRawAddressKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallHistoryEntryQueryOptions, IPhoneCallHistoryEntryQueryOptions_Vtbl, 0x9c5fe15c_8bed_40ca_b06e_c4ca8eae5c87);
impl windows_core::RuntimeType for IPhoneCallHistoryEntryQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryQueryOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallHistoryEntryQueryDesiredMedia) -> windows_core::HRESULT,
    pub SetDesiredMedia: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallHistoryEntryQueryDesiredMedia) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceIds: usize,
}
windows_core::imp::define_interface!(IPhoneCallHistoryEntryReader, IPhoneCallHistoryEntryReader_Vtbl, 0x61ece4be_8d86_479f_8404_a9846920fee6);
impl windows_core::RuntimeType for IPhoneCallHistoryEntryReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
windows_core::imp::define_interface!(IPhoneCallHistoryManagerForUser, IPhoneCallHistoryManagerForUser_Vtbl, 0xd925c523_f55f_4353_9db4_0205a5265a55);
impl windows_core::RuntimeType for IPhoneCallHistoryManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallHistoryStoreAccessType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IPhoneCallHistoryManagerStatics, IPhoneCallHistoryManagerStatics_Vtbl, 0xf5a6da39_b31f_4f45_ac8e_1b08893c1b50);
impl windows_core::RuntimeType for IPhoneCallHistoryManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallHistoryStoreAccessType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallHistoryManagerStatics2, IPhoneCallHistoryManagerStatics2_Vtbl, 0xefd474f0_a2db_4188_9e92_bc3cfa6813cf);
impl windows_core::RuntimeType for IPhoneCallHistoryManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IPhoneCallHistoryStore, IPhoneCallHistoryStore_Vtbl, 0x2f907db8_b40e_422b_8545_cb1910a61c52);
impl windows_core::RuntimeType for IPhoneCallHistoryStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallHistoryStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetEntryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEntryReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEntryReaderWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveEntryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteEntryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteEntriesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteEntriesAsync: usize,
    pub MarkEntryAsSeenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkEntriesAsSeenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkEntriesAsSeenAsync: usize,
    pub GetUnseenCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MarkAllAsSeenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSourcesUnseenCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSourcesUnseenCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkSourcesAsSeenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkSourcesAsSeenAsync: usize,
}
windows_core::imp::define_interface!(IPhoneCallInfo, IPhoneCallInfo_Vtbl, 0x22b42577_3e4d_5dc6_89c2_469fe5ffc189);
impl windows_core::RuntimeType for IPhoneCallInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsHoldSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallDirection) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallManagerStatics, IPhoneCallManagerStatics_Vtbl, 0x60edac78_78a6_4872_a3ef_98325ec8b843);
impl windows_core::RuntimeType for IPhoneCallManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowPhoneCallUI: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallManagerStatics2, IPhoneCallManagerStatics2_Vtbl, 0xc7e3c8bc_2370_431c_98fd_43be5f03086d);
impl windows_core::RuntimeType for IPhoneCallManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CallStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCallStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsCallActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCallIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShowPhoneCallSettingsUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallStatics, IPhoneCallStatics_Vtbl, 0x2218eeab_f60b_53e7_ba13_5aeafbc22957);
impl windows_core::RuntimeType for IPhoneCallStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallStore, IPhoneCallStore_Vtbl, 0x5f610748_18a6_4173_86d1_28be9dc62dba);
impl windows_core::RuntimeType for IPhoneCallStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEmergencyPhoneNumberAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultLineAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestLineWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallVideoCapabilities, IPhoneCallVideoCapabilities_Vtbl, 0x02382786_b16a_4fdb_be3b_c4240e13ad0d);
impl windows_core::RuntimeType for IPhoneCallVideoCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallVideoCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsVideoCallingCapable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallVideoCapabilitiesManagerStatics, IPhoneCallVideoCapabilitiesManagerStatics_Vtbl, 0xf3c64b56_f00b_4a1c_a0c6_ee1910749ce7);
impl windows_core::RuntimeType for IPhoneCallVideoCapabilitiesManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCapabilitiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneCallsResult, IPhoneCallsResult_Vtbl, 0x1bfad365_57cf_57dd_986d_b057c91eac33);
impl windows_core::RuntimeType for IPhoneCallsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OperationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneLineOperationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllActivePhoneCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllActivePhoneCalls: usize,
}
windows_core::imp::define_interface!(IPhoneDialOptions, IPhoneDialOptions_Vtbl, 0xb639c4b8_f06f_36cb_a863_823742b5f2d4);
impl windows_core::RuntimeType for IPhoneDialOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneDialOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Number: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPhone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPhone: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContactPhone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContactPhone: usize,
    pub Media: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallMedia) -> windows_core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallMedia) -> windows_core::HRESULT,
    pub AudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneAudioRoutingEndpoint) -> windows_core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneAudioRoutingEndpoint) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLine, IPhoneLine_Vtbl, 0x27c66f30_6a69_34ca_a2ba_65302530c311);
impl windows_core::RuntimeType for IPhoneLine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLineChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub NetworkState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneNetworkState) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Voicemail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetworkName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CellularDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Transport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneLineTransport) -> windows_core::HRESULT,
    pub CanDial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportsTile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub VideoCallingCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LineConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsImmediateDialNumberAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dial: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DialWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLine2, IPhoneLine2_Vtbl, 0x0167f56a_5344_5d64_8af3_a31a950e916a);
impl windows_core::RuntimeType for IPhoneLine2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLine2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub EnableTextReply: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EnableTextReply: usize,
    pub TransportDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLine3, IPhoneLine3_Vtbl, 0xe2e33cf7_2406_57f3_826a_e5a5f40d6fb5);
impl windows_core::RuntimeType for IPhoneLine3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLine3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DialWithResult: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DialWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllActivePhoneCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllActivePhoneCallsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineCellularDetails, IPhoneLineCellularDetails_Vtbl, 0x192601d5_147c_4769_b673_98a5ec8426cb);
impl windows_core::RuntimeType for IPhoneLineCellularDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineCellularDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SimState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneSimState) -> windows_core::HRESULT,
    pub SimSlotIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsModemOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RegistrationRejectCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetNetworkOperatorDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneLineNetworkOperatorDisplayTextLocation, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineConfiguration, IPhoneLineConfiguration_Vtbl, 0xfe265862_f64f_4312_b2a8_4e257721aa95);
impl windows_core::RuntimeType for IPhoneLineConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsVideoCallingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
windows_core::imp::define_interface!(IPhoneLineDialResult, IPhoneLineDialResult_Vtbl, 0xe825a30a_5c7f_546f_b918_3ad2fe70fb34);
impl windows_core::RuntimeType for IPhoneLineDialResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineDialResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DialCallStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallOperationStatus) -> windows_core::HRESULT,
    pub DialedCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineStatics, IPhoneLineStatics_Vtbl, 0xf38b5f23_ceb0_404f_bcf2_ba9f697d8adf);
impl windows_core::RuntimeType for IPhoneLineStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineTransportDevice, IPhoneLineTransportDevice_Vtbl, 0xefa8f889_cffa_59f4_97e4_74705b7dc490);
impl windows_core::RuntimeType for IPhoneLineTransportDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineTransportDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Transport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneLineTransport) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RequestAccessAsync: usize,
    pub RegisterApp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub RegisterAppForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    RegisterAppForUser: usize,
    pub UnregisterApp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub UnregisterAppForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    UnregisterAppForUser: usize,
    pub IsRegistered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineTransportDevice2, IPhoneLineTransportDevice2_Vtbl, 0x64c885f2_ecf4_5761_8c04_3c248ce61690);
impl windows_core::RuntimeType for IPhoneLineTransportDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineTransportDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioRoutingStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TransportDeviceAudioRoutingStatus) -> windows_core::HRESULT,
    pub AudioRoutingStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioRoutingStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InBandRingingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub InBandRingingEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInBandRingingEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineTransportDeviceStatics, IPhoneLineTransportDeviceStatics_Vtbl, 0x0f3121ac_d609_51a1_96f3_fb00d1819252);
impl windows_core::RuntimeType for IPhoneLineTransportDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineTransportDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorForPhoneLineTransport: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneLineTransport, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineWatcher, IPhoneLineWatcher_Vtbl, 0x8a45cd0a_6323_44e0_a6f6_9f21f64dc90a);
impl windows_core::RuntimeType for IPhoneLineWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LineAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLineAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub LineRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLineRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub LineUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLineUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneLineWatcherStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineWatcherEventArgs, IPhoneLineWatcherEventArgs_Vtbl, 0xd07c753e_9e12_4a37_82b7_ad535dad6a67);
impl windows_core::RuntimeType for IPhoneLineWatcherEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineWatcherEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneVoicemail, IPhoneVoicemail_Vtbl, 0xc9ce77f6_6e9f_3a8b_b727_6e0cf6998224);
impl windows_core::RuntimeType for IPhoneVoicemail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneVoicemail_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Number: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneVoicemailType) -> windows_core::HRESULT,
    pub DialVoicemailAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipCallCoordinator, IVoipCallCoordinator_Vtbl, 0x4f118bcf_e8ef_4434_9c5f_a8d893fafe79);
impl windows_core::RuntimeType for IVoipCallCoordinator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipCallCoordinator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReserveCallResourcesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MuteStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMuteStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RequestNewIncomingCall: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, VoipPhoneCallMedia, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestNewOutgoingCall: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, VoipPhoneCallMedia, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyMuted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyUnmuted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestOutgoingUpgradeToVideoCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestIncomingUpgradeToVideoCall: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminateCellularCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub CancelUpgrade: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipCallCoordinator2, IVoipCallCoordinator2_Vtbl, 0xbeb4a9f3_c704_4234_89ce_e88cc0d28fbe);
impl windows_core::RuntimeType for IVoipCallCoordinator2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipCallCoordinator2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetupNewAcceptedCall: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, VoipPhoneCallMedia, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipCallCoordinator3, IVoipCallCoordinator3_Vtbl, 0x338d0cbf_9b55_4021_87ca_e64b9bd666c7);
impl windows_core::RuntimeType for IVoipCallCoordinator3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipCallCoordinator3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestNewAppInitiatedCall: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, VoipPhoneCallMedia, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestNewIncomingCallWithContactRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, VoipPhoneCallMedia, super::super::Foundation::TimeSpan, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipCallCoordinator4, IVoipCallCoordinator4_Vtbl, 0x83737239_9311_468f_bb49_47e0dfb5d93e);
impl windows_core::RuntimeType for IVoipCallCoordinator4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipCallCoordinator4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReserveOneProcessCallResourcesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipCallCoordinatorStatics, IVoipCallCoordinatorStatics_Vtbl, 0x7f5d1f2b_e04a_4d10_b31a_a55c922cc2fb);
impl windows_core::RuntimeType for IVoipCallCoordinatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipCallCoordinatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipPhoneCall, IVoipPhoneCall_Vtbl, 0x6cf1f19a_7794_4a5a_8c68_ae87947a6990);
impl windows_core::RuntimeType for IVoipPhoneCall {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipPhoneCall_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EndRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEndRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub HoldRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHoldRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ResumeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveResumeRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AnswerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAnswerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RejectRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRejectRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NotifyCallHeld: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyCallActive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyCallEnded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContactName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub CallMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VoipPhoneCallMedia) -> windows_core::HRESULT,
    pub SetCallMedia: unsafe extern "system" fn(*mut core::ffi::c_void, VoipPhoneCallMedia) -> windows_core::HRESULT,
    pub NotifyCallReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipPhoneCall2, IVoipPhoneCall2_Vtbl, 0x741b46e1_245f_41f3_9399_3141d25b52e3);
impl windows_core::RuntimeType for IVoipPhoneCall2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipPhoneCall2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryShowAppUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVoipPhoneCall3, IVoipPhoneCall3_Vtbl, 0x0d891522_e258_4aa9_907a_1aa413c25523);
impl windows_core::RuntimeType for IVoipPhoneCall3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVoipPhoneCall3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotifyCallAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, VoipPhoneCallMedia) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CallAnswerEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CallAnswerEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CallAnswerEventArgs {
    pub fn AcceptedMedia(&self) -> windows_core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptedMedia)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CallAnswerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICallAnswerEventArgs>();
}
unsafe impl windows_core::Interface for CallAnswerEventArgs {
    type Vtable = ICallAnswerEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICallAnswerEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CallAnswerEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallAnswerEventArgs";
}
unsafe impl Send for CallAnswerEventArgs {}
unsafe impl Sync for CallAnswerEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CallRejectEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CallRejectEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CallRejectEventArgs {
    pub fn RejectReason(&self) -> windows_core::Result<VoipPhoneCallRejectReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RejectReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CallRejectEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICallRejectEventArgs>();
}
unsafe impl windows_core::Interface for CallRejectEventArgs {
    type Vtable = ICallRejectEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICallRejectEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CallRejectEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallRejectEventArgs";
}
unsafe impl Send for CallRejectEventArgs {}
unsafe impl Sync for CallRejectEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CallStateChangeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CallStateChangeEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CallStateChangeEventArgs {
    pub fn State(&self) -> windows_core::Result<VoipPhoneCallState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CallStateChangeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICallStateChangeEventArgs>();
}
unsafe impl windows_core::Interface for CallStateChangeEventArgs {
    type Vtable = ICallStateChangeEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICallStateChangeEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CallStateChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.CallStateChangeEventArgs";
}
unsafe impl Send for CallStateChangeEventArgs {}
unsafe impl Sync for CallStateChangeEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LockScreenCallEndCallDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockScreenCallEndCallDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl LockScreenCallEndCallDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for LockScreenCallEndCallDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILockScreenCallEndCallDeferral>();
}
unsafe impl windows_core::Interface for LockScreenCallEndCallDeferral {
    type Vtable = ILockScreenCallEndCallDeferral_Vtbl;
    const IID: windows_core::GUID = <ILockScreenCallEndCallDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockScreenCallEndCallDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral";
}
unsafe impl Send for LockScreenCallEndCallDeferral {}
unsafe impl Sync for LockScreenCallEndCallDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LockScreenCallEndRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockScreenCallEndRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LockScreenCallEndRequestedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<LockScreenCallEndCallDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LockScreenCallEndRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILockScreenCallEndRequestedEventArgs>();
}
unsafe impl windows_core::Interface for LockScreenCallEndRequestedEventArgs {
    type Vtable = ILockScreenCallEndRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ILockScreenCallEndRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockScreenCallEndRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs";
}
unsafe impl Send for LockScreenCallEndRequestedEventArgs {}
unsafe impl Sync for LockScreenCallEndRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LockScreenCallUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LockScreenCallUI, windows_core::IUnknown, windows_core::IInspectable);
impl LockScreenCallUI {
    pub fn Dismiss(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Dismiss)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EndRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<LockScreenCallUI, LockScreenCallEndRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEndRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEndRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<LockScreenCallUI, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CallTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCallTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCallTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for LockScreenCallUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILockScreenCallUI>();
}
unsafe impl windows_core::Interface for LockScreenCallUI {
    type Vtable = ILockScreenCallUI_Vtbl;
    const IID: windows_core::GUID = <ILockScreenCallUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LockScreenCallUI {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.LockScreenCallUI";
}
unsafe impl Send for LockScreenCallUI {}
unsafe impl Sync for LockScreenCallUI {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MuteChangeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MuteChangeEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MuteChangeEventArgs {
    pub fn Muted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Muted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MuteChangeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMuteChangeEventArgs>();
}
unsafe impl windows_core::Interface for MuteChangeEventArgs {
    type Vtable = IMuteChangeEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMuteChangeEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MuteChangeEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.MuteChangeEventArgs";
}
unsafe impl Send for MuteChangeEventArgs {}
unsafe impl Sync for MuteChangeEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCall(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCall, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCall {
    pub fn StatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneCall, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AudioDeviceChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneCall, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDeviceChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioDeviceChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioDeviceChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsMutedChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneCall, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMutedChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsMutedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsMutedChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CallId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsMuted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMuted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<PhoneCallStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AudioDevice(&self) -> windows_core::Result<PhoneCallAudioDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDevice)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPhoneCallInfo(&self) -> windows_core::Result<PhoneCallInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPhoneCallInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPhoneCallInfoAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPhoneCallInfoAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn End(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).End)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendDtmfKey(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendDtmfKey)(windows_core::Interface::as_raw(this), key, dtmftoneaudioplayback, &mut result__).map(|| result__)
        }
    }
    pub fn SendDtmfKeyAsync(&self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendDtmfKeyAsync)(windows_core::Interface::as_raw(this), key, dtmftoneaudioplayback, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcceptIncoming(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptIncoming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AcceptIncomingAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptIncomingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Hold(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HoldAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResumeFromHold(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResumeFromHold)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResumeFromHoldAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResumeFromHoldAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Mute(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MuteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Unmute(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unmute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnmuteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnmuteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RejectIncoming(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RejectIncoming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RejectIncomingAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RejectIncomingAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChangeAudioDevice(&self, endpoint: PhoneCallAudioDevice) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeAudioDevice)(windows_core::Interface::as_raw(this), endpoint, &mut result__).map(|| result__)
        }
    }
    pub fn ChangeAudioDeviceAsync(&self, endpoint: PhoneCallAudioDevice) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallOperationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeAudioDeviceAsync)(windows_core::Interface::as_raw(this), endpoint, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFromId(callid: &windows_core::HSTRING) -> windows_core::Result<PhoneCall> {
        Self::IPhoneCallStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(callid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallStatics<R, F: FnOnce(&IPhoneCallStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCall, IPhoneCallStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PhoneCall {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCall>();
}
unsafe impl windows_core::Interface for PhoneCall {
    type Vtable = IPhoneCall_Vtbl;
    const IID: windows_core::GUID = <IPhoneCall as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCall";
}
unsafe impl Send for PhoneCall {}
unsafe impl Sync for PhoneCall {}
pub struct PhoneCallBlocking;
impl PhoneCallBlocking {
    pub fn BlockUnknownNumbers() -> windows_core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BlockUnknownNumbers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetBlockUnknownNumbers(value: bool) -> windows_core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetBlockUnknownNumbers)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn BlockPrivateNumbers() -> windows_core::Result<bool> {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BlockPrivateNumbers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetBlockPrivateNumbers(value: bool) -> windows_core::Result<()> {
        Self::IPhoneCallBlockingStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetBlockPrivateNumbers)(windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetCallBlockingListAsync<P0>(phonenumberlist: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::IPhoneCallBlockingStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetCallBlockingListAsync)(windows_core::Interface::as_raw(this), phonenumberlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallBlockingStatics<R, F: FnOnce(&IPhoneCallBlockingStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallBlocking, IPhoneCallBlockingStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PhoneCallBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallBlocking";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallHistoryEntry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntry, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallHistoryEntry {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallHistoryEntry, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Address(&self) -> windows_core::Result<PhoneCallHistoryEntryAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAddress<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PhoneCallHistoryEntryAddress>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAddress)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsCallerIdBlocked(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCallerIdBlocked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCallerIdBlocked(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCallerIdBlocked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEmergency(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEmergency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEmergency(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEmergency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsIncoming(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIncoming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsIncoming(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsIncoming)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMissed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMissed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsMissed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsMissed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRinging(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRinging)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRinging(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsRinging)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSeen(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSeen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsSeen)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSuppressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSuppressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSuppressed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsSuppressed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVoicemail(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVoicemail)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVoicemail(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsVoicemail)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Media(&self) -> windows_core::Result<PhoneCallHistoryEntryMedia> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Media)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMedia(&self, value: PhoneCallHistoryEntryMedia) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMedia)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppReadAccess(&self) -> windows_core::Result<PhoneCallHistoryEntryOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherAppReadAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOtherAppReadAccess)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSourceId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSourceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceIdKind(&self) -> windows_core::Result<PhoneCallHistorySourceIdKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceIdKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSourceIdKind(&self, value: PhoneCallHistorySourceIdKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSourceIdKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallHistoryEntry>();
}
unsafe impl windows_core::Interface for PhoneCallHistoryEntry {
    type Vtable = IPhoneCallHistoryEntry_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallHistoryEntry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallHistoryEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntry";
}
unsafe impl Send for PhoneCallHistoryEntry {}
unsafe impl Sync for PhoneCallHistoryEntry {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallHistoryEntryAddress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntryAddress, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallHistoryEntryAddress {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallHistoryEntryAddress, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ContactId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContactId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContactId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
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
    pub fn RawAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRawAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRawAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RawAddressKind(&self) -> windows_core::Result<PhoneCallHistoryEntryRawAddressKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawAddressKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRawAddressKind(&self, value: PhoneCallHistoryEntryRawAddressKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRawAddressKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(rawaddress: &windows_core::HSTRING, rawaddresskind: PhoneCallHistoryEntryRawAddressKind) -> windows_core::Result<PhoneCallHistoryEntryAddress> {
        Self::IPhoneCallHistoryEntryAddressFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(rawaddress), rawaddresskind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryEntryAddressFactory<R, F: FnOnce(&IPhoneCallHistoryEntryAddressFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallHistoryEntryAddress, IPhoneCallHistoryEntryAddressFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntryAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallHistoryEntryAddress>();
}
unsafe impl windows_core::Interface for PhoneCallHistoryEntryAddress {
    type Vtable = IPhoneCallHistoryEntryAddress_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallHistoryEntryAddress as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallHistoryEntryAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress";
}
unsafe impl Send for PhoneCallHistoryEntryAddress {}
unsafe impl Sync for PhoneCallHistoryEntryAddress {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallHistoryEntryQueryOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntryQueryOptions, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallHistoryEntryQueryOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallHistoryEntryQueryOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DesiredMedia(&self) -> windows_core::Result<PhoneCallHistoryEntryQueryDesiredMedia> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredMedia)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredMedia(&self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredMedia)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SourceIds(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntryQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallHistoryEntryQueryOptions>();
}
unsafe impl windows_core::Interface for PhoneCallHistoryEntryQueryOptions {
    type Vtable = IPhoneCallHistoryEntryQueryOptions_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallHistoryEntryQueryOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallHistoryEntryQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions";
}
unsafe impl Send for PhoneCallHistoryEntryQueryOptions {}
unsafe impl Sync for PhoneCallHistoryEntryQueryOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallHistoryEntryReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallHistoryEntryReader, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallHistoryEntryReader {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhoneCallHistoryEntry>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntryReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallHistoryEntryReader>();
}
unsafe impl windows_core::Interface for PhoneCallHistoryEntryReader {
    type Vtable = IPhoneCallHistoryEntryReader_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallHistoryEntryReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallHistoryEntryReader {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader";
}
unsafe impl Send for PhoneCallHistoryEntryReader {}
unsafe impl Sync for PhoneCallHistoryEntryReader {}
pub struct PhoneCallHistoryManager;
impl PhoneCallHistoryManager {
    pub fn RequestStoreAsync(accesstype: PhoneCallHistoryStoreAccessType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        Self::IPhoneCallHistoryManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), accesstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<PhoneCallHistoryManagerForUser>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IPhoneCallHistoryManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryManagerStatics<R, F: FnOnce(&IPhoneCallHistoryManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPhoneCallHistoryManagerStatics2<R, F: FnOnce(&IPhoneCallHistoryManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallHistoryManager, IPhoneCallHistoryManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PhoneCallHistoryManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallHistoryManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallHistoryManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallHistoryManagerForUser {
    pub fn RequestStoreAsync(&self, accesstype: PhoneCallHistoryStoreAccessType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryStore>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), accesstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallHistoryManagerForUser>();
}
unsafe impl windows_core::Interface for PhoneCallHistoryManagerForUser {
    type Vtable = IPhoneCallHistoryManagerForUser_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallHistoryManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallHistoryManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser";
}
unsafe impl Send for PhoneCallHistoryManagerForUser {}
unsafe impl Sync for PhoneCallHistoryManagerForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallHistoryStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallHistoryStore, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallHistoryStore {
    pub fn GetEntryAsync(&self, callhistoryentryid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallHistoryEntry>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEntryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(callhistoryentryid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetEntryReader(&self) -> windows_core::Result<PhoneCallHistoryEntryReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEntryReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetEntryReaderWithOptions<P0>(&self, queryoptions: P0) -> windows_core::Result<PhoneCallHistoryEntryReader>
    where
        P0: windows_core::Param<PhoneCallHistoryEntryQueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEntryReaderWithOptions)(windows_core::Interface::as_raw(this), queryoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveEntryAsync<P0>(&self, callhistoryentry: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<PhoneCallHistoryEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveEntryAsync)(windows_core::Interface::as_raw(this), callhistoryentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteEntryAsync<P0>(&self, callhistoryentry: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<PhoneCallHistoryEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteEntryAsync)(windows_core::Interface::as_raw(this), callhistoryentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeleteEntriesAsync<P0>(&self, callhistoryentries: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteEntriesAsync)(windows_core::Interface::as_raw(this), callhistoryentries.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MarkEntryAsSeenAsync<P0>(&self, callhistoryentry: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<PhoneCallHistoryEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkEntryAsSeenAsync)(windows_core::Interface::as_raw(this), callhistoryentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkEntriesAsSeenAsync<P0>(&self, callhistoryentries: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<PhoneCallHistoryEntry>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkEntriesAsSeenAsync)(windows_core::Interface::as_raw(this), callhistoryentries.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetUnseenCountAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUnseenCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MarkAllAsSeenAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkAllAsSeenAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSourcesUnseenCountAsync<P0>(&self, sourceids: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSourcesUnseenCountAsync)(windows_core::Interface::as_raw(this), sourceids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkSourcesAsSeenAsync<P0>(&self, sourceids: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkSourcesAsSeenAsync)(windows_core::Interface::as_raw(this), sourceids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallHistoryStore>();
}
unsafe impl windows_core::Interface for PhoneCallHistoryStore {
    type Vtable = IPhoneCallHistoryStore_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallHistoryStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallHistoryStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallHistoryStore";
}
unsafe impl Send for PhoneCallHistoryStore {}
unsafe impl Sync for PhoneCallHistoryStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallInfo, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallInfo {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHoldSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallDirection(&self) -> windows_core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneCallInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallInfo>();
}
unsafe impl windows_core::Interface for PhoneCallInfo {
    type Vtable = IPhoneCallInfo_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallInfo";
}
unsafe impl Send for PhoneCallInfo {}
unsafe impl Sync for PhoneCallInfo {}
pub struct PhoneCallManager;
impl PhoneCallManager {
    pub fn ShowPhoneCallUI(phonenumber: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IPhoneCallManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ShowPhoneCallUI)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(phonenumber), core::mem::transmute_copy(displayname)).ok() })
    }
    pub fn CallStateChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveCallStateChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveCallStateChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn IsCallActive() -> windows_core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCallActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsCallIncoming() -> windows_core::Result<bool> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCallIncoming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ShowPhoneCallSettingsUI() -> windows_core::Result<()> {
        Self::IPhoneCallManagerStatics2(|this| unsafe { (windows_core::Interface::vtable(this).ShowPhoneCallSettingsUI)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn RequestStoreAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallStore>> {
        Self::IPhoneCallManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallManagerStatics<R, F: FnOnce(&IPhoneCallManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPhoneCallManagerStatics2<R, F: FnOnce(&IPhoneCallManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallManager, IPhoneCallManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PhoneCallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallStore, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallStore {
    pub fn IsEmergencyPhoneNumberAsync(&self, number: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEmergencyPhoneNumberAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(number), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultLineAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultLineAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestLineWatcher(&self) -> windows_core::Result<PhoneLineWatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestLineWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallStore>();
}
unsafe impl windows_core::Interface for PhoneCallStore {
    type Vtable = IPhoneCallStore_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallStore {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallStore";
}
unsafe impl Send for PhoneCallStore {}
unsafe impl Sync for PhoneCallStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallVideoCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallVideoCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallVideoCapabilities {
    pub fn IsVideoCallingCapable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoCallingCapable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneCallVideoCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallVideoCapabilities>();
}
unsafe impl windows_core::Interface for PhoneCallVideoCapabilities {
    type Vtable = IPhoneCallVideoCapabilities_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallVideoCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallVideoCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities";
}
unsafe impl Send for PhoneCallVideoCapabilities {}
unsafe impl Sync for PhoneCallVideoCapabilities {}
pub struct PhoneCallVideoCapabilitiesManager;
impl PhoneCallVideoCapabilitiesManager {
    pub fn GetCapabilitiesAsync(phonenumber: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallVideoCapabilities>> {
        Self::IPhoneCallVideoCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCapabilitiesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(phonenumber), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneCallVideoCapabilitiesManagerStatics<R, F: FnOnce(&IPhoneCallVideoCapabilitiesManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneCallVideoCapabilitiesManager, IPhoneCallVideoCapabilitiesManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PhoneCallVideoCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallsResult, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallsResult {
    pub fn OperationStatus(&self) -> windows_core::Result<PhoneLineOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OperationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllActivePhoneCalls(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PhoneCall>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllActivePhoneCalls)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallsResult>();
}
unsafe impl windows_core::Interface for PhoneCallsResult {
    type Vtable = IPhoneCallsResult_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallsResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneCallsResult";
}
unsafe impl Send for PhoneCallsResult {}
unsafe impl Sync for PhoneCallsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneDialOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneDialOptions, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneDialOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneDialOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Number(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Number)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNumber(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
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
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContact<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Contacts::Contact>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContact)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPhone(&self) -> windows_core::Result<super::Contacts::ContactPhone> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactPhone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn SetContactPhone<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Contacts::ContactPhone>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContactPhone)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Media(&self) -> windows_core::Result<PhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Media)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMedia(&self, value: PhoneCallMedia) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMedia)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEndpoint(&self) -> windows_core::Result<PhoneAudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioEndpoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioEndpoint(&self, value: PhoneAudioRoutingEndpoint) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudioEndpoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PhoneDialOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneDialOptions>();
}
unsafe impl windows_core::Interface for PhoneDialOptions {
    type Vtable = IPhoneDialOptions_Vtbl;
    const IID: windows_core::GUID = <IPhoneDialOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneDialOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneDialOptions";
}
unsafe impl Send for PhoneDialOptions {}
unsafe impl Sync for PhoneDialOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLine(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLine, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLine {
    pub fn LineChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLine, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLineChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLineChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn DisplayColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NetworkState(&self) -> windows_core::Result<PhoneNetworkState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Voicemail(&self) -> windows_core::Result<PhoneVoicemail> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Voicemail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NetworkName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CellularDetails(&self) -> windows_core::Result<PhoneLineCellularDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CellularDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Transport(&self) -> windows_core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanDial(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDial)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportsTile(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsTile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VideoCallingCapabilities(&self) -> windows_core::Result<PhoneCallVideoCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoCallingCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LineConfiguration(&self) -> windows_core::Result<PhoneLineConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsImmediateDialNumberAsync(&self, number: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsImmediateDialNumberAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(number), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dial(&self, number: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Dial)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(number), core::mem::transmute_copy(displayname)).ok() }
    }
    pub fn DialWithOptions<P0>(&self, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PhoneDialOptions>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DialWithOptions)(windows_core::Interface::as_raw(this), options.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn EnableTextReply(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPhoneLine2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableTextReply)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransportDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPhoneLine2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DialWithResult(&self, number: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<PhoneLineDialResult> {
        let this = &windows_core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialWithResult)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(number), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DialWithResultAsync(&self, number: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneLineDialResult>> {
        let this = &windows_core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialWithResultAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(number), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAllActivePhoneCalls(&self) -> windows_core::Result<PhoneCallsResult> {
        let this = &windows_core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllActivePhoneCalls)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAllActivePhoneCallsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneCallsResult>> {
        let this = &windows_core::Interface::cast::<IPhoneLine3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllActivePhoneCallsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(lineid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PhoneLine>> {
        Self::IPhoneLineStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), lineid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneLineStatics<R, F: FnOnce(&IPhoneLineStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneLine, IPhoneLineStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PhoneLine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLine>();
}
unsafe impl windows_core::Interface for PhoneLine {
    type Vtable = IPhoneLine_Vtbl;
    const IID: windows_core::GUID = <IPhoneLine as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLine {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLine";
}
unsafe impl Send for PhoneLine {}
unsafe impl Sync for PhoneLine {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLineCellularDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineCellularDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineCellularDetails {
    pub fn SimState(&self) -> windows_core::Result<PhoneSimState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SimSlotIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimSlotIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsModemOn(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsModemOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RegistrationRejectCode(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegistrationRejectCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetNetworkOperatorDisplayText(&self, location: PhoneLineNetworkOperatorDisplayTextLocation) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNetworkOperatorDisplayText)(windows_core::Interface::as_raw(this), location, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneLineCellularDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineCellularDetails>();
}
unsafe impl windows_core::Interface for PhoneLineCellularDetails {
    type Vtable = IPhoneLineCellularDetails_Vtbl;
    const IID: windows_core::GUID = <IPhoneLineCellularDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineCellularDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineCellularDetails";
}
unsafe impl Send for PhoneLineCellularDetails {}
unsafe impl Sync for PhoneLineCellularDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLineConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineConfiguration {
    pub fn IsVideoCallingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoCallingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneLineConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineConfiguration>();
}
unsafe impl windows_core::Interface for PhoneLineConfiguration {
    type Vtable = IPhoneLineConfiguration_Vtbl;
    const IID: windows_core::GUID = <IPhoneLineConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineConfiguration";
}
unsafe impl Send for PhoneLineConfiguration {}
unsafe impl Sync for PhoneLineConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLineDialResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineDialResult, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineDialResult {
    pub fn DialCallStatus(&self) -> windows_core::Result<PhoneCallOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialCallStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DialedCall(&self) -> windows_core::Result<PhoneCall> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialedCall)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneLineDialResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineDialResult>();
}
unsafe impl windows_core::Interface for PhoneLineDialResult {
    type Vtable = IPhoneLineDialResult_Vtbl;
    const IID: windows_core::GUID = <IPhoneLineDialResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineDialResult {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineDialResult";
}
unsafe impl Send for PhoneLineDialResult {}
unsafe impl Sync for PhoneLineDialResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLineTransportDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineTransportDevice, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineTransportDevice {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Transport(&self) -> windows_core::Result<PhoneLineTransport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RequestAccessAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Devices::Enumeration::DeviceAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterApp(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterApp)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn RegisterAppForUser<P0>(&self, user: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterAppForUser)(windows_core::Interface::as_raw(this), user.param().abi()).ok() }
    }
    pub fn UnregisterApp(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UnregisterApp)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UnregisterAppForUser<P0>(&self, user: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UnregisterAppForUser)(windows_core::Interface::as_raw(this), user.param().abi()).ok() }
    }
    pub fn IsRegistered(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRegistered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Connect(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConnectAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AudioRoutingStatus(&self) -> windows_core::Result<TransportDeviceAudioRoutingStatus> {
        let this = &windows_core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioRoutingStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AudioRoutingStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioRoutingStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioRoutingStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioRoutingStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn InBandRingingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InBandRingingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InBandRingingEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLineTransportDevice, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InBandRingingEnabledChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInBandRingingEnabledChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPhoneLineTransportDevice2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveInBandRingingEnabledChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FromId(id: &windows_core::HSTRING) -> windows_core::Result<PhoneLineTransportDevice> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForPhoneLineTransport(transport: PhoneLineTransport) -> windows_core::Result<windows_core::HSTRING> {
        Self::IPhoneLineTransportDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForPhoneLineTransport)(windows_core::Interface::as_raw(this), transport, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneLineTransportDeviceStatics<R, F: FnOnce(&IPhoneLineTransportDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneLineTransportDevice, IPhoneLineTransportDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PhoneLineTransportDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineTransportDevice>();
}
unsafe impl windows_core::Interface for PhoneLineTransportDevice {
    type Vtable = IPhoneLineTransportDevice_Vtbl;
    const IID: windows_core::GUID = <IPhoneLineTransportDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineTransportDevice {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineTransportDevice";
}
unsafe impl Send for PhoneLineTransportDevice {}
unsafe impl Sync for PhoneLineTransportDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLineWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineWatcher {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LineAdded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineAdded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLineAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLineAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn LineRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLineRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLineRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn LineUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, PhoneLineWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLineUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLineUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, windows_core::IInspectable>>,
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
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PhoneLineWatcher, windows_core::IInspectable>>,
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
    pub fn Status(&self) -> windows_core::Result<PhoneLineWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneLineWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineWatcher>();
}
unsafe impl windows_core::Interface for PhoneLineWatcher {
    type Vtable = IPhoneLineWatcher_Vtbl;
    const IID: windows_core::GUID = <IPhoneLineWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineWatcher {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcher";
}
unsafe impl Send for PhoneLineWatcher {}
unsafe impl Sync for PhoneLineWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLineWatcherEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineWatcherEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineWatcherEventArgs {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneLineWatcherEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineWatcherEventArgs>();
}
unsafe impl windows_core::Interface for PhoneLineWatcherEventArgs {
    type Vtable = IPhoneLineWatcherEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPhoneLineWatcherEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineWatcherEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs";
}
unsafe impl Send for PhoneLineWatcherEventArgs {}
unsafe impl Sync for PhoneLineWatcherEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneVoicemail(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneVoicemail, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneVoicemail {
    pub fn Number(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Number)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MessageCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<PhoneVoicemailType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DialVoicemailAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialVoicemailAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneVoicemail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneVoicemail>();
}
unsafe impl windows_core::Interface for PhoneVoicemail {
    type Vtable = IPhoneVoicemail_Vtbl;
    const IID: windows_core::GUID = <IPhoneVoicemail as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneVoicemail {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.PhoneVoicemail";
}
unsafe impl Send for PhoneVoicemail {}
unsafe impl Sync for PhoneVoicemail {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoipCallCoordinator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoipCallCoordinator, windows_core::IUnknown, windows_core::IInspectable);
impl VoipCallCoordinator {
    pub fn ReserveCallResourcesAsync(&self, taskentrypoint: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReserveCallResourcesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(taskentrypoint), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MuteStateChanged<P0>(&self, mutechangehandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VoipCallCoordinator, MuteChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MuteStateChanged)(windows_core::Interface::as_raw(this), mutechangehandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMuteStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMuteStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestNewIncomingCall<P0, P1, P2>(&self, context: &windows_core::HSTRING, contactname: &windows_core::HSTRING, contactnumber: &windows_core::HSTRING, contactimage: P0, servicename: &windows_core::HSTRING, brandingimage: P1, calldetails: &windows_core::HSTRING, ringtone: P2, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan) -> windows_core::Result<VoipPhoneCall>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Uri>,
        P2: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestNewIncomingCall)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(context), core::mem::transmute_copy(contactname), core::mem::transmute_copy(contactnumber), contactimage.param().abi(), core::mem::transmute_copy(servicename), brandingimage.param().abi(), core::mem::transmute_copy(calldetails), ringtone.param().abi(), media, ringtimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestNewOutgoingCall(&self, context: &windows_core::HSTRING, contactname: &windows_core::HSTRING, servicename: &windows_core::HSTRING, media: VoipPhoneCallMedia) -> windows_core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestNewOutgoingCall)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(context), core::mem::transmute_copy(contactname), core::mem::transmute_copy(servicename), media, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NotifyMuted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyMuted)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyUnmuted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyUnmuted)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestOutgoingUpgradeToVideoCall(&self, callupgradeguid: windows_core::GUID, context: &windows_core::HSTRING, contactname: &windows_core::HSTRING, servicename: &windows_core::HSTRING) -> windows_core::Result<VoipPhoneCall> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestOutgoingUpgradeToVideoCall)(windows_core::Interface::as_raw(this), callupgradeguid, core::mem::transmute_copy(context), core::mem::transmute_copy(contactname), core::mem::transmute_copy(servicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestIncomingUpgradeToVideoCall<P0, P1, P2>(&self, context: &windows_core::HSTRING, contactname: &windows_core::HSTRING, contactnumber: &windows_core::HSTRING, contactimage: P0, servicename: &windows_core::HSTRING, brandingimage: P1, calldetails: &windows_core::HSTRING, ringtone: P2, ringtimeout: super::super::Foundation::TimeSpan) -> windows_core::Result<VoipPhoneCall>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Uri>,
        P2: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestIncomingUpgradeToVideoCall)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(context), core::mem::transmute_copy(contactname), core::mem::transmute_copy(contactnumber), contactimage.param().abi(), core::mem::transmute_copy(servicename), brandingimage.param().abi(), core::mem::transmute_copy(calldetails), ringtone.param().abi(), ringtimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TerminateCellularCall(&self, callupgradeguid: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).TerminateCellularCall)(windows_core::Interface::as_raw(this), callupgradeguid).ok() }
    }
    pub fn CancelUpgrade(&self, callupgradeguid: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CancelUpgrade)(windows_core::Interface::as_raw(this), callupgradeguid).ok() }
    }
    pub fn SetupNewAcceptedCall(&self, context: &windows_core::HSTRING, contactname: &windows_core::HSTRING, contactnumber: &windows_core::HSTRING, servicename: &windows_core::HSTRING, media: VoipPhoneCallMedia) -> windows_core::Result<VoipPhoneCall> {
        let this = &windows_core::Interface::cast::<IVoipCallCoordinator2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetupNewAcceptedCall)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(context), core::mem::transmute_copy(contactname), core::mem::transmute_copy(contactnumber), core::mem::transmute_copy(servicename), media, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestNewAppInitiatedCall(&self, context: &windows_core::HSTRING, contactname: &windows_core::HSTRING, contactnumber: &windows_core::HSTRING, servicename: &windows_core::HSTRING, media: VoipPhoneCallMedia) -> windows_core::Result<VoipPhoneCall> {
        let this = &windows_core::Interface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestNewAppInitiatedCall)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(context), core::mem::transmute_copy(contactname), core::mem::transmute_copy(contactnumber), core::mem::transmute_copy(servicename), media, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestNewIncomingCallWithContactRemoteId<P0, P1, P2>(&self, context: &windows_core::HSTRING, contactname: &windows_core::HSTRING, contactnumber: &windows_core::HSTRING, contactimage: P0, servicename: &windows_core::HSTRING, brandingimage: P1, calldetails: &windows_core::HSTRING, ringtone: P2, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, contactremoteid: &windows_core::HSTRING) -> windows_core::Result<VoipPhoneCall>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Uri>,
        P2: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IVoipCallCoordinator3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestNewIncomingCallWithContactRemoteId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(context), core::mem::transmute_copy(contactname), core::mem::transmute_copy(contactnumber), contactimage.param().abi(), core::mem::transmute_copy(servicename), brandingimage.param().abi(), core::mem::transmute_copy(calldetails), ringtone.param().abi(), media, ringtimeout, core::mem::transmute_copy(contactremoteid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReserveOneProcessCallResourcesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VoipPhoneCallResourceReservationStatus>> {
        let this = &windows_core::Interface::cast::<IVoipCallCoordinator4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReserveOneProcessCallResourcesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<VoipCallCoordinator> {
        Self::IVoipCallCoordinatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVoipCallCoordinatorStatics<R, F: FnOnce(&IVoipCallCoordinatorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VoipCallCoordinator, IVoipCallCoordinatorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VoipCallCoordinator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoipCallCoordinator>();
}
unsafe impl windows_core::Interface for VoipCallCoordinator {
    type Vtable = IVoipCallCoordinator_Vtbl;
    const IID: windows_core::GUID = <IVoipCallCoordinator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoipCallCoordinator {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipCallCoordinator";
}
unsafe impl Send for VoipCallCoordinator {}
unsafe impl Sync for VoipCallCoordinator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VoipPhoneCall(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VoipPhoneCall, windows_core::IUnknown, windows_core::IInspectable);
impl VoipPhoneCall {
    pub fn EndRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEndRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEndRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HoldRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHoldRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHoldRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ResumeRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallStateChangeEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResumeRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResumeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveResumeRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AnswerRequested<P0>(&self, accepthandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallAnswerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnswerRequested)(windows_core::Interface::as_raw(this), accepthandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAnswerRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAnswerRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RejectRequested<P0>(&self, rejecthandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VoipPhoneCall, CallRejectEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RejectRequested)(windows_core::Interface::as_raw(this), rejecthandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRejectRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRejectRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NotifyCallHeld(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyCallHeld)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyCallActive(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyCallActive)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyCallEnded(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyCallEnded)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ContactName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContactName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContactName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CallMedia(&self) -> windows_core::Result<VoipPhoneCallMedia> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallMedia)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCallMedia(&self, value: VoipPhoneCallMedia) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCallMedia)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NotifyCallReady(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyCallReady)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryShowAppUI(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVoipPhoneCall2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryShowAppUI)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyCallAccepted(&self, media: VoipPhoneCallMedia) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVoipPhoneCall3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).NotifyCallAccepted)(windows_core::Interface::as_raw(this), media).ok() }
    }
}
impl windows_core::RuntimeType for VoipPhoneCall {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVoipPhoneCall>();
}
unsafe impl windows_core::Interface for VoipPhoneCall {
    type Vtable = IVoipPhoneCall_Vtbl;
    const IID: windows_core::GUID = <IVoipPhoneCall as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VoipPhoneCall {
    const NAME: &'static str = "Windows.ApplicationModel.Calls.VoipPhoneCall";
}
unsafe impl Send for VoipPhoneCall {}
unsafe impl Sync for VoipPhoneCall {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: Self = Self(0i32);
    pub const Burst: Self = Self(1i32);
}
impl windows_core::TypeKind for CellularDtmfMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CellularDtmfMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CellularDtmfMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CellularDtmfMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.CellularDtmfMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DtmfKey(pub i32);
impl DtmfKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
}
impl windows_core::TypeKind for DtmfKey {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DtmfKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DtmfKey").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DtmfKey {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfKey;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: Self = Self(0i32);
    pub const DoNotPlay: Self = Self(1i32);
}
impl windows_core::TypeKind for DtmfToneAudioPlayback {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DtmfToneAudioPlayback {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DtmfToneAudioPlayback").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DtmfToneAudioPlayback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.DtmfToneAudioPlayback;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Bluetooth: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneAudioRoutingEndpoint {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneAudioRoutingEndpoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneAudioRoutingEndpoint").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneAudioRoutingEndpoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneAudioRoutingEndpoint;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: Self = Self(0i32);
    pub const LocalDevice: Self = Self(1i32);
    pub const RemoteDevice: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneCallAudioDevice {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallAudioDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallAudioDevice").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallAudioDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallAudioDevice;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Outgoing: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneCallDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallDirection;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl windows_core::TypeKind for PhoneCallHistoryEntryMedia {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallHistoryEntryMedia {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryMedia").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntryMedia {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryMedia;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
}
impl windows_core::TypeKind for PhoneCallHistoryEntryOtherAppReadAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallHistoryEntryOtherAppReadAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryOtherAppReadAccess").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntryOtherAppReadAccess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryOtherAppReadAccess;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const All: Self = Self(4294967295u32);
}
impl windows_core::TypeKind for PhoneCallHistoryEntryQueryDesiredMedia {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallHistoryEntryQueryDesiredMedia {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryQueryDesiredMedia").field(&self.0).finish()
    }
}
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PhoneCallHistoryEntryQueryDesiredMedia {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PhoneCallHistoryEntryQueryDesiredMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntryQueryDesiredMedia {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryDesiredMedia;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl windows_core::TypeKind for PhoneCallHistoryEntryRawAddressKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallHistoryEntryRawAddressKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryEntryRawAddressKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryEntryRawAddressKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryEntryRawAddressKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: Self = Self(0i32);
    pub const PackageFamilyName: Self = Self(1i32);
}
impl windows_core::TypeKind for PhoneCallHistorySourceIdKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallHistorySourceIdKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallHistorySourceIdKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallHistorySourceIdKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistorySourceIdKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: Self = Self(0i32);
    pub const AllEntriesLimitedReadWrite: Self = Self(1i32);
    pub const AllEntriesReadWrite: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneCallHistoryStoreAccessType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallHistoryStoreAccessType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallHistoryStoreAccessType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallHistoryStoreAccessType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallHistoryStoreAccessType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: Self = Self(0i32);
    pub const AudioAndVideo: Self = Self(1i32);
    pub const AudioAndRealTimeText: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneCallMedia {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallMedia {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallMedia").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallMedia {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallMedia;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl windows_core::TypeKind for PhoneCallOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallOperationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallStatus(pub i32);
impl PhoneCallStatus {
    pub const Lost: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Dialing: Self = Self(2i32);
    pub const Talking: Self = Self(3i32);
    pub const Held: Self = Self(4i32);
    pub const Ended: Self = Self(5i32);
}
impl windows_core::TypeKind for PhoneCallStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneCallStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Dialer: Self = Self(2i32);
    pub const InCallUI: Self = Self(3i32);
}
impl windows_core::TypeKind for PhoneLineNetworkOperatorDisplayTextLocation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneLineNetworkOperatorDisplayTextLocation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneLineNetworkOperatorDisplayTextLocation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneLineNetworkOperatorDisplayTextLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineNetworkOperatorDisplayTextLocation;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl windows_core::TypeKind for PhoneLineOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneLineOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneLineOperationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneLineOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: Self = Self(0i32);
    pub const VoipApp: Self = Self(1i32);
    pub const Bluetooth: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneLineTransport {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneLineTransport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneLineTransport").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneLineTransport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineTransport;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
}
impl windows_core::TypeKind for PhoneLineWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneLineWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneLineWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneLineWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneLineWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneNetworkState(pub i32);
impl PhoneNetworkState {
    pub const Unknown: Self = Self(0i32);
    pub const NoSignal: Self = Self(1i32);
    pub const Deregistered: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const Searching: Self = Self(4i32);
    pub const Home: Self = Self(5i32);
    pub const RoamingInternational: Self = Self(6i32);
    pub const RoamingDomestic: Self = Self(7i32);
}
impl windows_core::TypeKind for PhoneNetworkState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneNetworkState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneNetworkState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneNetworkState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneNetworkState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneSimState(pub i32);
impl PhoneSimState {
    pub const Unknown: Self = Self(0i32);
    pub const PinNotRequired: Self = Self(1i32);
    pub const PinUnlocked: Self = Self(2i32);
    pub const PinLocked: Self = Self(3i32);
    pub const PukLocked: Self = Self(4i32);
    pub const NotInserted: Self = Self(5i32);
    pub const Invalid: Self = Self(6i32);
    pub const Disabled: Self = Self(7i32);
}
impl windows_core::TypeKind for PhoneSimState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneSimState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneSimState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneSimState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneSimState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: Self = Self(0i32);
    pub const Traditional: Self = Self(1i32);
    pub const Visual: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneVoicemailType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneVoicemailType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneVoicemailType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneVoicemailType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.PhoneVoicemailType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CanRouteToLocalDevice: Self = Self(1i32);
    pub const CannotRouteToLocalDevice: Self = Self(2i32);
}
impl windows_core::TypeKind for TransportDeviceAudioRoutingStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TransportDeviceAudioRoutingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TransportDeviceAudioRoutingStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TransportDeviceAudioRoutingStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.TransportDeviceAudioRoutingStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
}
impl windows_core::TypeKind for VoipPhoneCallMedia {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VoipPhoneCallMedia {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VoipPhoneCallMedia").field(&self.0).finish()
    }
}
impl VoipPhoneCallMedia {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VoipPhoneCallMedia {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VoipPhoneCallMedia {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VoipPhoneCallMedia {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VoipPhoneCallMedia {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VoipPhoneCallMedia {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for VoipPhoneCallMedia {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallMedia;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const OtherIncomingCall: Self = Self(2i32);
    pub const EmergencyCallExists: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl windows_core::TypeKind for VoipPhoneCallRejectReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VoipPhoneCallRejectReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VoipPhoneCallRejectReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VoipPhoneCallRejectReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallRejectReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: Self = Self(0i32);
    pub const ResourcesNotAvailable: Self = Self(1i32);
}
impl windows_core::TypeKind for VoipPhoneCallResourceReservationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VoipPhoneCallResourceReservationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VoipPhoneCallResourceReservationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VoipPhoneCallResourceReservationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: Self = Self(0i32);
    pub const Held: Self = Self(1i32);
    pub const Active: Self = Self(2i32);
    pub const Incoming: Self = Self(3i32);
    pub const Outgoing: Self = Self(4i32);
}
impl windows_core::TypeKind for VoipPhoneCallState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VoipPhoneCallState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VoipPhoneCallState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VoipPhoneCallState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Calls.VoipPhoneCallState;i4)");
}
