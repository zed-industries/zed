windows_core::imp::define_interface!(IMiracastReceiver, IMiracastReceiver_Vtbl, 0x7a315258_e444_51b4_aff7_b88daa1229e0);
impl windows_core::RuntimeType for IMiracastReceiver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAllAndApplySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAllAndApplySettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateSession: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateSessionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateSessionAsync: usize,
    pub ClearKnownTransmitters: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveKnownTransmitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverApplySettingsResult, IMiracastReceiverApplySettingsResult_Vtbl, 0xd0aa6272_09cd_58e1_a4f2_5d5143d312f9);
impl windows_core::RuntimeType for IMiracastReceiverApplySettingsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverApplySettingsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MiracastReceiverApplySettingsStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverConnection, IMiracastReceiverConnection_Vtbl, 0x704b2f36_d2e5_551f_a854_f822b7917d28);
impl windows_core::RuntimeType for IMiracastReceiverConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, MiracastReceiverDisconnectReason) -> windows_core::HRESULT,
    pub DisconnectWithMessage: unsafe extern "system" fn(*mut core::ffi::c_void, MiracastReceiverDisconnectReason, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Transmitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InputDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CursorImageChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StreamControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverConnectionCreatedEventArgs, IMiracastReceiverConnectionCreatedEventArgs_Vtbl, 0x7d8dfa39_307a_5c0f_94bd_d0c69d169982);
impl windows_core::RuntimeType for IMiracastReceiverConnectionCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverConnectionCreatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverCursorImageChannel, IMiracastReceiverCursorImageChannel_Vtbl, 0xd9ac332d_723a_5a9d_b90a_81153efa2a0f);
impl windows_core::RuntimeType for IMiracastReceiverCursorImageChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverCursorImageChannel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::PointInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Position: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImageStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImageStream: usize,
    pub ImageStreamChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveImageStreamChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverCursorImageChannelSettings, IMiracastReceiverCursorImageChannelSettings_Vtbl, 0xccdbedff_bd00_5b9c_8e4c_00cacf86b634);
impl windows_core::RuntimeType for IMiracastReceiverCursorImageChannelSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverCursorImageChannelSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub SetMaxImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetMaxImageSize: usize,
}
windows_core::imp::define_interface!(IMiracastReceiverDisconnectedEventArgs, IMiracastReceiverDisconnectedEventArgs_Vtbl, 0xd9a15e5e_5fee_57e6_b4b0_04727db93229);
impl windows_core::RuntimeType for IMiracastReceiverDisconnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverDisconnectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverGameControllerDevice, IMiracastReceiverGameControllerDevice_Vtbl, 0x2d7171e8_bed4_5118_a058_e2477eb5888d);
impl windows_core::RuntimeType for IMiracastReceiverGameControllerDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverGameControllerDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransmitInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MiracastReceiverGameControllerDeviceUsageMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, MiracastReceiverGameControllerDeviceUsageMode) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverInputDevices, IMiracastReceiverInputDevices_Vtbl, 0xda35bb02_28aa_5ee8_96f5_a42901c66f00);
impl windows_core::RuntimeType for IMiracastReceiverInputDevices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverInputDevices_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Keyboard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverKeyboardDevice, IMiracastReceiverKeyboardDevice_Vtbl, 0xbeb67272_06c0_54ff_ac96_217464ff2501);
impl windows_core::RuntimeType for IMiracastReceiverKeyboardDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverKeyboardDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransmitInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverMediaSourceCreatedEventArgs, IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl, 0x17cf519e_1246_531d_945a_6b158e39c3aa);
impl windows_core::RuntimeType for IMiracastReceiverMediaSourceCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    pub CursorImageChannelSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverSession, IMiracastReceiverSession_Vtbl, 0x1d2bcdb4_ef8b_5209_bfc9_c32116504803);
impl windows_core::RuntimeType for IMiracastReceiverSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConnectionCreated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MediaSourceCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMediaSourceCreated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AllowConnectionTakeover: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowConnectionTakeover: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxSimultaneousConnections: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverSessionStartResult, IMiracastReceiverSessionStartResult_Vtbl, 0xb7c573ee_40ca_51ff_95f2_c9de34f2e90e);
impl windows_core::RuntimeType for IMiracastReceiverSessionStartResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverSessionStartResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MiracastReceiverSessionStartStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverSettings, IMiracastReceiverSettings_Vtbl, 0x57cd2f24_c55a_5fbe_9464_eb05307705dd);
impl windows_core::RuntimeType for IMiracastReceiverSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetModelName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetModelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AuthorizationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MiracastReceiverAuthorizationMethod) -> windows_core::HRESULT,
    pub SetAuthorizationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, MiracastReceiverAuthorizationMethod) -> windows_core::HRESULT,
    pub RequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverStatus, IMiracastReceiverStatus_Vtbl, 0xc28a5591_23ab_519e_ad09_90bff6dcc87e);
impl windows_core::RuntimeType for IMiracastReceiverStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ListeningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MiracastReceiverListeningStatus) -> windows_core::HRESULT,
    pub WiFiStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MiracastReceiverWiFiStatus) -> windows_core::HRESULT,
    pub IsConnectionTakeoverSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub KnownTransmitters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KnownTransmitters: usize,
}
windows_core::imp::define_interface!(IMiracastReceiverStreamControl, IMiracastReceiverStreamControl_Vtbl, 0x38ea2d8b_2769_5ad7_8a8a_254b9df7ba82);
impl windows_core::RuntimeType for IMiracastReceiverStreamControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverStreamControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetVideoStreamSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVideoStreamSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestVideoStreamSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestVideoStreamSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MuteAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetMuteAudio: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastReceiverVideoStreamSettings, IMiracastReceiverVideoStreamSettings_Vtbl, 0x169b5e1b_149d_52d0_b126_6f89744e4f50);
impl windows_core::RuntimeType for IMiracastReceiverVideoStreamSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastReceiverVideoStreamSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Size: usize,
    #[cfg(feature = "Graphics")]
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetSize: usize,
    pub Bitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMiracastTransmitter, IMiracastTransmitter_Vtbl, 0x342d79fd_2e64_5508_8a30_833d1eac70d0);
impl windows_core::RuntimeType for IMiracastTransmitter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMiracastTransmitter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AuthorizationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MiracastTransmitterAuthorizationStatus) -> windows_core::HRESULT,
    pub SetAuthorizationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, MiracastTransmitterAuthorizationStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnections: usize,
    pub MacAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LastConnectionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiver(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiver, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiver {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MiracastReceiver, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetDefaultSettings(&self) -> windows_core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentSettings(&self) -> windows_core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentSettingsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettingsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisconnectAllAndApplySettings<P0>(&self, settings: P0) -> windows_core::Result<MiracastReceiverApplySettingsResult>
    where
        P0: windows_core::Param<MiracastReceiverSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisconnectAllAndApplySettings)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisconnectAllAndApplySettingsAsync<P0>(&self, settings: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>>
    where
        P0: windows_core::Param<MiracastReceiverSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisconnectAllAndApplySettingsAsync)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetStatus(&self) -> windows_core::Result<MiracastReceiverStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetStatusAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatusAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiver, windows_core::IInspectable>>,
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
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateSession<P0>(&self, view: P0) -> windows_core::Result<MiracastReceiverSession>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Core::CoreApplicationView>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSession)(windows_core::Interface::as_raw(this), view.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateSessionAsync<P0>(&self, view: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Core::CoreApplicationView>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSessionAsync)(windows_core::Interface::as_raw(this), view.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearKnownTransmitters(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearKnownTransmitters)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveKnownTransmitter<P0>(&self, transmitter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MiracastTransmitter>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKnownTransmitter)(windows_core::Interface::as_raw(this), transmitter.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiver>();
}
unsafe impl windows_core::Interface for MiracastReceiver {
    type Vtable = IMiracastReceiver_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiver as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiver {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiver";
}
unsafe impl Send for MiracastReceiver {}
unsafe impl Sync for MiracastReceiver {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverApplySettingsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverApplySettingsResult, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverApplySettingsResult {
    pub fn Status(&self) -> windows_core::Result<MiracastReceiverApplySettingsStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MiracastReceiverApplySettingsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverApplySettingsResult>();
}
unsafe impl windows_core::Interface for MiracastReceiverApplySettingsResult {
    type Vtable = IMiracastReceiverApplySettingsResult_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverApplySettingsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverApplySettingsResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverApplySettingsResult";
}
unsafe impl Send for MiracastReceiverApplySettingsResult {}
unsafe impl Sync for MiracastReceiverApplySettingsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverConnection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MiracastReceiverConnection, super::super::Foundation::IClosable);
impl MiracastReceiverConnection {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Disconnect(&self, reason: MiracastReceiverDisconnectReason) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Disconnect)(windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DisconnectWithMessage(&self, reason: MiracastReceiverDisconnectReason, message: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DisconnectWithMessage)(windows_core::Interface::as_raw(this), reason, core::mem::transmute_copy(message)).ok() }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PauseAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Resume(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Resume)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ResumeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResumeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Transmitter(&self) -> windows_core::Result<MiracastTransmitter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transmitter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InputDevices(&self) -> windows_core::Result<MiracastReceiverInputDevices> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputDevices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CursorImageChannel(&self) -> windows_core::Result<MiracastReceiverCursorImageChannel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CursorImageChannel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StreamControl(&self) -> windows_core::Result<MiracastReceiverStreamControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MiracastReceiverConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverConnection>();
}
unsafe impl windows_core::Interface for MiracastReceiverConnection {
    type Vtable = IMiracastReceiverConnection_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverConnection {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnection";
}
unsafe impl Send for MiracastReceiverConnection {}
unsafe impl Sync for MiracastReceiverConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverConnectionCreatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverConnectionCreatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverConnectionCreatedEventArgs {
    pub fn Connection(&self) -> windows_core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Pin(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pin)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for MiracastReceiverConnectionCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverConnectionCreatedEventArgs>();
}
unsafe impl windows_core::Interface for MiracastReceiverConnectionCreatedEventArgs {
    type Vtable = IMiracastReceiverConnectionCreatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverConnectionCreatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverConnectionCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs";
}
unsafe impl Send for MiracastReceiverConnectionCreatedEventArgs {}
unsafe impl Sync for MiracastReceiverConnectionCreatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverCursorImageChannel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverCursorImageChannel, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverCursorImageChannel {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> windows_core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxImageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn Position(&self) -> windows_core::Result<super::super::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImageStream(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImageStreamChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageStreamChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveImageStreamChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveImageStreamChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PositionChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePositionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiverCursorImageChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverCursorImageChannel>();
}
unsafe impl windows_core::Interface for MiracastReceiverCursorImageChannel {
    type Vtable = IMiracastReceiverCursorImageChannel_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverCursorImageChannel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverCursorImageChannel {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannel";
}
unsafe impl Send for MiracastReceiverCursorImageChannel {}
unsafe impl Sync for MiracastReceiverCursorImageChannel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverCursorImageChannelSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverCursorImageChannelSettings, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverCursorImageChannelSettings {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> windows_core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxImageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn SetMaxImageSize(&self, value: super::super::Graphics::SizeInt32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxImageSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiverCursorImageChannelSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverCursorImageChannelSettings>();
}
unsafe impl windows_core::Interface for MiracastReceiverCursorImageChannelSettings {
    type Vtable = IMiracastReceiverCursorImageChannelSettings_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverCursorImageChannelSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverCursorImageChannelSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings";
}
unsafe impl Send for MiracastReceiverCursorImageChannelSettings {}
unsafe impl Sync for MiracastReceiverCursorImageChannelSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverDisconnectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverDisconnectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverDisconnectedEventArgs {
    pub fn Connection(&self) -> windows_core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MiracastReceiverDisconnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverDisconnectedEventArgs>();
}
unsafe impl windows_core::Interface for MiracastReceiverDisconnectedEventArgs {
    type Vtable = IMiracastReceiverDisconnectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverDisconnectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverDisconnectedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs";
}
unsafe impl Send for MiracastReceiverDisconnectedEventArgs {}
unsafe impl Sync for MiracastReceiverDisconnectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverGameControllerDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverGameControllerDevice, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverGameControllerDevice {
    pub fn TransmitInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransmitInput(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransmitInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRequestedByTransmitter(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequestedByTransmitter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTransmittingInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTransmittingInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Mode(&self) -> windows_core::Result<MiracastReceiverGameControllerDeviceUsageMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: MiracastReceiverGameControllerDeviceUsageMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiverGameControllerDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverGameControllerDevice>();
}
unsafe impl windows_core::Interface for MiracastReceiverGameControllerDevice {
    type Vtable = IMiracastReceiverGameControllerDevice_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverGameControllerDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverGameControllerDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverGameControllerDevice";
}
unsafe impl Send for MiracastReceiverGameControllerDevice {}
unsafe impl Sync for MiracastReceiverGameControllerDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverInputDevices(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverInputDevices, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverInputDevices {
    pub fn Keyboard(&self) -> windows_core::Result<MiracastReceiverKeyboardDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Keyboard)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GameController(&self) -> windows_core::Result<MiracastReceiverGameControllerDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MiracastReceiverInputDevices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverInputDevices>();
}
unsafe impl windows_core::Interface for MiracastReceiverInputDevices {
    type Vtable = IMiracastReceiverInputDevices_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverInputDevices as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverInputDevices {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverInputDevices";
}
unsafe impl Send for MiracastReceiverInputDevices {}
unsafe impl Sync for MiracastReceiverInputDevices {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverKeyboardDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverKeyboardDevice, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverKeyboardDevice {
    pub fn TransmitInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransmitInput(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransmitInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRequestedByTransmitter(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequestedByTransmitter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTransmittingInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTransmittingInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiverKeyboardDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverKeyboardDevice>();
}
unsafe impl windows_core::Interface for MiracastReceiverKeyboardDevice {
    type Vtable = IMiracastReceiverKeyboardDevice_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverKeyboardDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverKeyboardDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverKeyboardDevice";
}
unsafe impl Send for MiracastReceiverKeyboardDevice {}
unsafe impl Sync for MiracastReceiverKeyboardDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverMediaSourceCreatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverMediaSourceCreatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverMediaSourceCreatedEventArgs {
    pub fn Connection(&self) -> windows_core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> windows_core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CursorImageChannelSettings(&self) -> windows_core::Result<MiracastReceiverCursorImageChannelSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CursorImageChannelSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for MiracastReceiverMediaSourceCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverMediaSourceCreatedEventArgs>();
}
unsafe impl windows_core::Interface for MiracastReceiverMediaSourceCreatedEventArgs {
    type Vtable = IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverMediaSourceCreatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverMediaSourceCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs";
}
unsafe impl Send for MiracastReceiverMediaSourceCreatedEventArgs {}
unsafe impl Sync for MiracastReceiverMediaSourceCreatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MiracastReceiverSession, super::super::Foundation::IClosable);
impl MiracastReceiverSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ConnectionCreated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionCreated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionCreated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MediaSourceCreated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSourceCreated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMediaSourceCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMediaSourceCreated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Disconnected<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Disconnected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AllowConnectionTakeover(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowConnectionTakeover)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowConnectionTakeover(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowConnectionTakeover)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxSimultaneousConnections(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSimultaneousConnections)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxSimultaneousConnections(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxSimultaneousConnections)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<MiracastReceiverSessionStartResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MiracastReceiverSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverSession>();
}
unsafe impl windows_core::Interface for MiracastReceiverSession {
    type Vtable = IMiracastReceiverSession_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverSession {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSession";
}
unsafe impl Send for MiracastReceiverSession {}
unsafe impl Sync for MiracastReceiverSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverSessionStartResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverSessionStartResult, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverSessionStartResult {
    pub fn Status(&self) -> windows_core::Result<MiracastReceiverSessionStartStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MiracastReceiverSessionStartResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverSessionStartResult>();
}
unsafe impl windows_core::Interface for MiracastReceiverSessionStartResult {
    type Vtable = IMiracastReceiverSessionStartResult_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverSessionStartResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverSessionStartResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSessionStartResult";
}
unsafe impl Send for MiracastReceiverSessionStartResult {}
unsafe impl Sync for MiracastReceiverSessionStartResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverSettings, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverSettings {
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ModelName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetModelName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetModelName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ModelNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetModelNumber(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetModelNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AuthorizationMethod(&self) -> windows_core::Result<MiracastReceiverAuthorizationMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthorizationMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuthorizationMethod(&self, value: MiracastReceiverAuthorizationMethod) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuthorizationMethod)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequireAuthorizationFromKnownTransmitters(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequireAuthorizationFromKnownTransmitters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequireAuthorizationFromKnownTransmitters(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequireAuthorizationFromKnownTransmitters)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiverSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverSettings>();
}
unsafe impl windows_core::Interface for MiracastReceiverSettings {
    type Vtable = IMiracastReceiverSettings_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSettings";
}
unsafe impl Send for MiracastReceiverSettings {}
unsafe impl Sync for MiracastReceiverSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverStatus, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverStatus {
    pub fn ListeningStatus(&self) -> windows_core::Result<MiracastReceiverListeningStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListeningStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WiFiStatus(&self) -> windows_core::Result<MiracastReceiverWiFiStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WiFiStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConnectionTakeoverSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnectionTakeoverSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxSimultaneousConnections(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSimultaneousConnections)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn KnownTransmitters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownTransmitters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MiracastReceiverStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverStatus>();
}
unsafe impl windows_core::Interface for MiracastReceiverStatus {
    type Vtable = IMiracastReceiverStatus_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverStatus {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStatus";
}
unsafe impl Send for MiracastReceiverStatus {}
unsafe impl Sync for MiracastReceiverStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverStreamControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverStreamControl, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverStreamControl {
    pub fn GetVideoStreamSettings(&self) -> windows_core::Result<MiracastReceiverVideoStreamSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVideoStreamSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetVideoStreamSettingsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVideoStreamSettingsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SuggestVideoStreamSettings<P0>(&self, settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MiracastReceiverVideoStreamSettings>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SuggestVideoStreamSettings)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() }
    }
    pub fn SuggestVideoStreamSettingsAsync<P0>(&self, settings: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<MiracastReceiverVideoStreamSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestVideoStreamSettingsAsync)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MuteAudio(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MuteAudio)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMuteAudio(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMuteAudio)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiverStreamControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverStreamControl>();
}
unsafe impl windows_core::Interface for MiracastReceiverStreamControl {
    type Vtable = IMiracastReceiverStreamControl_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverStreamControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverStreamControl {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStreamControl";
}
unsafe impl Send for MiracastReceiverStreamControl {}
unsafe impl Sync for MiracastReceiverStreamControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastReceiverVideoStreamSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastReceiverVideoStreamSettings, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastReceiverVideoStreamSettings {
    #[cfg(feature = "Graphics")]
    pub fn Size(&self) -> windows_core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics")]
    pub fn SetSize(&self, value: super::super::Graphics::SizeInt32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bitrate(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBitrate(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MiracastReceiverVideoStreamSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastReceiverVideoStreamSettings>();
}
unsafe impl windows_core::Interface for MiracastReceiverVideoStreamSettings {
    type Vtable = IMiracastReceiverVideoStreamSettings_Vtbl;
    const IID: windows_core::GUID = <IMiracastReceiverVideoStreamSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastReceiverVideoStreamSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverVideoStreamSettings";
}
unsafe impl Send for MiracastReceiverVideoStreamSettings {}
unsafe impl Sync for MiracastReceiverVideoStreamSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MiracastTransmitter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MiracastTransmitter, windows_core::IUnknown, windows_core::IInspectable);
impl MiracastTransmitter {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AuthorizationStatus(&self) -> windows_core::Result<MiracastTransmitterAuthorizationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthorizationStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuthorizationStatus(&self, value: MiracastTransmitterAuthorizationStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuthorizationStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MacAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MacAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastConnectionTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastConnectionTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MiracastTransmitter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMiracastTransmitter>();
}
unsafe impl windows_core::Interface for MiracastTransmitter {
    type Vtable = IMiracastTransmitter_Vtbl;
    const IID: windows_core::GUID = <IMiracastTransmitter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MiracastTransmitter {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastTransmitter";
}
unsafe impl Send for MiracastTransmitter {}
unsafe impl Sync for MiracastTransmitter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastReceiverApplySettingsStatus(pub i32);
impl MiracastReceiverApplySettingsStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const FriendlyNameTooLong: Self = Self(4i32);
    pub const ModelNameTooLong: Self = Self(5i32);
    pub const ModelNumberTooLong: Self = Self(6i32);
    pub const InvalidSettings: Self = Self(7i32);
}
impl windows_core::TypeKind for MiracastReceiverApplySettingsStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastReceiverApplySettingsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastReceiverApplySettingsStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastReceiverApplySettingsStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverApplySettingsStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastReceiverAuthorizationMethod(pub i32);
impl MiracastReceiverAuthorizationMethod {
    pub const None: Self = Self(0i32);
    pub const ConfirmConnection: Self = Self(1i32);
    pub const PinDisplayIfRequested: Self = Self(2i32);
    pub const PinDisplayRequired: Self = Self(3i32);
}
impl windows_core::TypeKind for MiracastReceiverAuthorizationMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastReceiverAuthorizationMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastReceiverAuthorizationMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastReceiverAuthorizationMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverAuthorizationMethod;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastReceiverDisconnectReason(pub i32);
impl MiracastReceiverDisconnectReason {
    pub const Finished: Self = Self(0i32);
    pub const AppSpecificError: Self = Self(1i32);
    pub const ConnectionNotAccepted: Self = Self(2i32);
    pub const DisconnectedByUser: Self = Self(3i32);
    pub const FailedToStartStreaming: Self = Self(4i32);
    pub const MediaDecodingError: Self = Self(5i32);
    pub const MediaStreamingError: Self = Self(6i32);
    pub const MediaDecryptionError: Self = Self(7i32);
}
impl windows_core::TypeKind for MiracastReceiverDisconnectReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastReceiverDisconnectReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastReceiverDisconnectReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastReceiverDisconnectReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverDisconnectReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastReceiverGameControllerDeviceUsageMode(pub i32);
impl MiracastReceiverGameControllerDeviceUsageMode {
    pub const AsGameController: Self = Self(0i32);
    pub const AsMouseAndKeyboard: Self = Self(1i32);
}
impl windows_core::TypeKind for MiracastReceiverGameControllerDeviceUsageMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastReceiverGameControllerDeviceUsageMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastReceiverGameControllerDeviceUsageMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastReceiverGameControllerDeviceUsageMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverGameControllerDeviceUsageMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastReceiverListeningStatus(pub i32);
impl MiracastReceiverListeningStatus {
    pub const NotListening: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const ConnectionPending: Self = Self(2i32);
    pub const Connected: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
    pub const TemporarilyDisabled: Self = Self(5i32);
}
impl windows_core::TypeKind for MiracastReceiverListeningStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastReceiverListeningStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastReceiverListeningStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastReceiverListeningStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverListeningStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastReceiverSessionStartStatus(pub i32);
impl MiracastReceiverSessionStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl windows_core::TypeKind for MiracastReceiverSessionStartStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastReceiverSessionStartStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastReceiverSessionStartStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastReceiverSessionStartStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverSessionStartStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastReceiverWiFiStatus(pub i32);
impl MiracastReceiverWiFiStatus {
    pub const MiracastSupportUndetermined: Self = Self(0i32);
    pub const MiracastNotSupported: Self = Self(1i32);
    pub const MiracastSupportNotOptimized: Self = Self(2i32);
    pub const MiracastSupported: Self = Self(3i32);
}
impl windows_core::TypeKind for MiracastReceiverWiFiStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastReceiverWiFiStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastReceiverWiFiStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastReceiverWiFiStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverWiFiStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MiracastTransmitterAuthorizationStatus(pub i32);
impl MiracastTransmitterAuthorizationStatus {
    pub const Undecided: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const AlwaysPrompt: Self = Self(2i32);
    pub const Blocked: Self = Self(3i32);
}
impl windows_core::TypeKind for MiracastTransmitterAuthorizationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MiracastTransmitterAuthorizationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MiracastTransmitterAuthorizationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MiracastTransmitterAuthorizationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastTransmitterAuthorizationStatus;i4)");
}
