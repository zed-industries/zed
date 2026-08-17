#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
windows_core::imp::define_interface!(IAdvancedCapturedPhoto, IAdvancedCapturedPhoto_Vtbl, 0xf072728b_b292_4491_9d41_99807a550bbf);
impl windows_core::RuntimeType for IAdvancedCapturedPhoto {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedCapturedPhoto_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Devices::AdvancedPhotoMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    Mode: usize,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedCapturedPhoto2, IAdvancedCapturedPhoto2_Vtbl, 0x18cf6cd8_cffe_42d8_8104_017bb318f4a1);
impl windows_core::RuntimeType for IAdvancedCapturedPhoto2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedCapturedPhoto2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameBoundsRelativeToReferencePhoto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedPhotoCapture, IAdvancedPhotoCapture_Vtbl, 0x83ffaafa_6667_44dc_973c_a6bce596aa0f);
impl windows_core::RuntimeType for IAdvancedPhotoCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedPhotoCapture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CaptureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CaptureWithContextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalReferencePhotoCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveOptionalReferencePhotoCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AllPhotosCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAllPhotosCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastBackgroundService, IAppBroadcastBackgroundService_Vtbl, 0xbad1e72a_fa94_46f9_95fc_d71511cda70b);
impl windows_core::RuntimeType for IAppBroadcastBackgroundService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastBackgroundService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPlugInState: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastPlugInState) -> windows_core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastPlugInState) -> windows_core::HRESULT,
    pub SetSignInInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SignInInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetViewerCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TerminateBroadcast: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastTerminationReason, u32) -> windows_core::HRESULT,
    pub HeartbeatRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHeartbeatRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TitleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastBackgroundService2, IAppBroadcastBackgroundService2_Vtbl, 0xfc8ccbbf_5549_4b87_959f_23ca401fd473);
impl windows_core::RuntimeType for IAppBroadcastBackgroundService2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastBackgroundService2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetBroadcastTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BroadcastChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetBroadcastChannel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BroadcastTitleChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBroadcastTitleChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BroadcastLanguageChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBroadcastLanguageChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BroadcastChannelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBroadcastChannelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastBackgroundServiceSignInInfo, IAppBroadcastBackgroundServiceSignInInfo_Vtbl, 0x5e735275_88c8_4eca_89ba_4825985db880);
impl windows_core::RuntimeType for IAppBroadcastBackgroundServiceSignInInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceSignInInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SignInState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastSignInState) -> windows_core::HRESULT,
    pub SetOAuthRequestUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OAuthRequestUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOAuthCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OAuthCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SignInStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSignInStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastBackgroundServiceSignInInfo2, IAppBroadcastBackgroundServiceSignInInfo2_Vtbl, 0x9104285c_62cf_4a3c_a7ee_aeb507404645);
impl windows_core::RuntimeType for IAppBroadcastBackgroundServiceSignInInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUserNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastBackgroundServiceStreamInfo, IAppBroadcastBackgroundServiceStreamInfo_Vtbl, 0x31dc02bc_990a_4904_aa96_fe364381f136);
impl windows_core::RuntimeType for IAppBroadcastBackgroundServiceStreamInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceStreamInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StreamState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastStreamState) -> windows_core::HRESULT,
    pub SetDesiredVideoEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub DesiredVideoEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetBandwidthTestBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub BandwidthTestBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetAudioCodec: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AudioCodec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BroadcastStreamReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StreamStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStreamStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub VideoEncodingResolutionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVideoEncodingResolutionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub VideoEncodingBitrateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVideoEncodingBitrateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastBackgroundServiceStreamInfo2, IAppBroadcastBackgroundServiceStreamInfo2_Vtbl, 0xbd1e9f6d_94dc_4fce_9541_a9f129596334);
impl windows_core::RuntimeType for IAppBroadcastBackgroundServiceStreamInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportProblemWithStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastCameraCaptureStateChangedEventArgs, IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl, 0x1e334cd0_b882_4b88_8692_05999aceb70f);
impl windows_core::RuntimeType for IAppBroadcastCameraCaptureStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastCameraCaptureState) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastGlobalSettings, IAppBroadcastGlobalSettings_Vtbl, 0xb2cb27a5_70fc_4e17_80bd_6ba0fd3ff3a0);
impl windows_core::RuntimeType for IAppBroadcastGlobalSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastGlobalSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBroadcastEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetIsCameraCaptureEnabledByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCameraCaptureEnabledByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSelectedCameraId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SelectedCameraId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCameraOverlayLocation: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastCameraOverlayLocation) -> windows_core::HRESULT,
    pub CameraOverlayLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastCameraOverlayLocation) -> windows_core::HRESULT,
    pub SetCameraOverlaySize: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastCameraOverlaySize) -> windows_core::HRESULT,
    pub CameraOverlaySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastCameraOverlaySize) -> windows_core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastHeartbeatRequestedEventArgs, IAppBroadcastHeartbeatRequestedEventArgs_Vtbl, 0xcea54283_ee51_4dbf_9472_79a9ed4e2165);
impl windows_core::RuntimeType for IAppBroadcastHeartbeatRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastManagerStatics, IAppBroadcastManagerStatics_Vtbl, 0x364e018b_1e4e_411f_ab3e_92959844c156);
impl windows_core::RuntimeType for IAppBroadcastManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetGlobalSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyGlobalSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProviderSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyProviderSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastMicrophoneCaptureStateChangedEventArgs, IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl, 0xa86ad5e9_9440_4908_9d09_65b7e315d795);
impl windows_core::RuntimeType for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastMicrophoneCaptureState) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastPlugIn, IAppBroadcastPlugIn_Vtbl, 0x520c1e66_6513_4574_ac54_23b79729615b);
impl windows_core::RuntimeType for IAppBroadcastPlugIn {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPlugIn_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProviderSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastPlugInManager, IAppBroadcastPlugInManager_Vtbl, 0xe550d979_27a1_49a7_bbf4_d7a9e9d07668);
impl windows_core::RuntimeType for IAppBroadcastPlugInManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPlugInManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBroadcastProviderAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PlugInList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PlugInList: usize,
    pub DefaultPlugIn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultPlugIn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastPlugInManagerStatics, IAppBroadcastPlugInManagerStatics_Vtbl, 0xf2645c20_5c76_4cdc_9364_82fe9eb6534d);
impl windows_core::RuntimeType for IAppBroadcastPlugInManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPlugInManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IAppBroadcastPlugInStateChangedEventArgs, IAppBroadcastPlugInStateChangedEventArgs_Vtbl, 0x4881d0f2_abc5_4fc6_84b0_89370bb47212);
impl windows_core::RuntimeType for IAppBroadcastPlugInStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPlugInStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlugInState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastPlugInState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastPreview, IAppBroadcastPreview_Vtbl, 0x14b60f5a_6e4a_4b80_a14f_67ee77d153e7);
impl windows_core::RuntimeType for IAppBroadcastPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StopPreview: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreviewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastPreviewState) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreviewStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePreviewStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PreviewStreamReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastPreviewStateChangedEventArgs, IAppBroadcastPreviewStateChangedEventArgs_Vtbl, 0x5a57f2de_8dea_4e86_90ad_03fc26b9653c);
impl windows_core::RuntimeType for IAppBroadcastPreviewStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPreviewStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreviewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastPreviewState) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastPreviewStreamReader, IAppBroadcastPreviewStreamReader_Vtbl, 0x92228d50_db3f_40a8_8cd4_f4e371ddab37);
impl windows_core::RuntimeType for IAppBroadcastPreviewStreamReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPreviewStreamReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub VideoStride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapAlphaMode: usize,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastPreviewStreamVideoFrame, IAppBroadcastPreviewStreamVideoFrame_Vtbl, 0x010fbea1_94fe_4499_b8c0_8d244279fb12);
impl windows_core::RuntimeType for IAppBroadcastPreviewStreamVideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPreviewStreamVideoFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
windows_core::imp::define_interface!(IAppBroadcastPreviewStreamVideoHeader, IAppBroadcastPreviewStreamVideoHeader_Vtbl, 0x8bef6113_da84_4499_a7ab_87118cb4a157);
impl windows_core::RuntimeType for IAppBroadcastPreviewStreamVideoHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastPreviewStreamVideoHeader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub RelativeTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastProviderSettings, IAppBroadcastProviderSettings_Vtbl, 0xc30bdf62_9948_458f_ad50_aa06ec03da08);
impl windows_core::RuntimeType for IAppBroadcastProviderSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastProviderSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDefaultBroadcastTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DefaultBroadcastTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastVideoEncodingBitrateMode) -> windows_core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastVideoEncodingBitrateMode) -> windows_core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastVideoEncodingResolutionMode) -> windows_core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastVideoEncodingResolutionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastServices, IAppBroadcastServices_Vtbl, 0x8660b4d6_969b_4e3c_ac3a_8b042ee4ee63);
impl windows_core::RuntimeType for IAppBroadcastServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastServices_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CaptureTargetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastCaptureTargetType) -> windows_core::HRESULT,
    pub SetCaptureTargetType: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastCaptureTargetType) -> windows_core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetBroadcastTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CanCapture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EnterBroadcastModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExitBroadcastMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastExitBroadcastModeReason) -> windows_core::HRESULT,
    pub StartBroadcast: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseBroadcast: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeBroadcast: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPreview: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastSignInStateChangedEventArgs, IAppBroadcastSignInStateChangedEventArgs_Vtbl, 0x02b692a4_5919_4a9e_8d5e_c9bb0dd3377a);
impl windows_core::RuntimeType for IAppBroadcastSignInStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastSignInStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SignInState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastSignInState) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastSignInResult) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastState, IAppBroadcastState_Vtbl, 0xee08056d_8099_4ddd_922e_c56dac58abfb);
impl windows_core::RuntimeType for IAppBroadcastState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCaptureTargetRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShouldCaptureCamera: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShouldCaptureCamera: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RestartCameraCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EncodedVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub MicrophoneCaptureState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastMicrophoneCaptureState) -> windows_core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CameraCaptureState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastCameraCaptureState) -> windows_core::HRESULT,
    pub CameraCaptureError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub StreamState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastStreamState) -> windows_core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastPlugInState) -> windows_core::HRESULT,
    pub OAuthRequestUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OAuthCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub SetAuthenticationResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    SetAuthenticationResult: usize,
    pub SetSignInState: unsafe extern "system" fn(*mut core::ffi::c_void, AppBroadcastSignInState) -> windows_core::HRESULT,
    pub SignInState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastSignInState) -> windows_core::HRESULT,
    pub TerminationReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastTerminationReason) -> windows_core::HRESULT,
    pub TerminationReasonPlugInSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ViewerCountChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveViewerCountChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CameraCaptureStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCameraCaptureStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PlugInStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlugInStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StreamStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStreamStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CaptureTargetClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastStreamAudioFrame, IAppBroadcastStreamAudioFrame_Vtbl, 0xefab4ac8_21ba_453f_8bb7_5e938a2e9a74);
impl windows_core::RuntimeType for IAppBroadcastStreamAudioFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastStreamAudioFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioBuffer: usize,
}
windows_core::imp::define_interface!(IAppBroadcastStreamAudioHeader, IAppBroadcastStreamAudioHeader_Vtbl, 0xbf21a570_6b78_4216_9f07_5aff5256f1b7);
impl windows_core::RuntimeType for IAppBroadcastStreamAudioHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastStreamAudioHeader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub RelativeTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub HasDiscontinuity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastStreamReader, IAppBroadcastStreamReader_Vtbl, 0xb338bcf9_3364_4460_b5f1_3cc2796a8aa2);
impl windows_core::RuntimeType for IAppBroadcastStreamReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastStreamReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioChannels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AudioSampleRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioAacSequence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioAacSequence: usize,
    pub AudioBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TryGetNextAudioFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub VideoBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AudioFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub VideoFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastStreamStateChangedEventArgs, IAppBroadcastStreamStateChangedEventArgs_Vtbl, 0x5108a733_d008_4a89_93be_58aed961374e);
impl windows_core::RuntimeType for IAppBroadcastStreamStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastStreamStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StreamState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppBroadcastStreamState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastStreamVideoFrame, IAppBroadcastStreamVideoFrame_Vtbl, 0x0f97cf2b_c9e4_4e88_8194_d814cbd585d8);
impl windows_core::RuntimeType for IAppBroadcastStreamVideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastStreamVideoFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
windows_core::imp::define_interface!(IAppBroadcastStreamVideoHeader, IAppBroadcastStreamVideoHeader_Vtbl, 0x0b9ebece_7e32_432d_8ca2_36bf10b9f462);
impl windows_core::RuntimeType for IAppBroadcastStreamVideoHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastStreamVideoHeader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub RelativeTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub IsKeyFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasDiscontinuity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastTriggerDetails, IAppBroadcastTriggerDetails_Vtbl, 0xdeebab35_ec5e_4d8f_b1c0_5da6e8c75638);
impl windows_core::RuntimeType for IAppBroadcastTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BackgroundService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastViewerCountChangedEventArgs, IAppBroadcastViewerCountChangedEventArgs_Vtbl, 0xe6e11825_5401_4ade_8bd2_c14ecee6807d);
impl windows_core::RuntimeType for IAppBroadcastViewerCountChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastViewerCountChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCapture, IAppCapture_Vtbl, 0x9749d453_a29a_45ed_8f29_22d09942cff7);
impl windows_core::RuntimeType for IAppCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCapture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCapturingAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCapturingVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CapturingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCapturingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureAlternateShortcutKeys, IAppCaptureAlternateShortcutKeys_Vtbl, 0x19e8e0ef_236c_40f9_b38f_9b7dd65d1ccc);
impl windows_core::RuntimeType for IAppCaptureAlternateShortcutKeys {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureAlternateShortcutKeys_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKeyModifiers: usize,
}
windows_core::imp::define_interface!(IAppCaptureAlternateShortcutKeys2, IAppCaptureAlternateShortcutKeys2_Vtbl, 0xc3669090_dd17_47f0_95e5_ce42286cf338);
impl windows_core::RuntimeType for IAppCaptureAlternateShortcutKeys2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureAlternateShortcutKeys2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKeyModifiers: usize,
}
windows_core::imp::define_interface!(IAppCaptureAlternateShortcutKeys3, IAppCaptureAlternateShortcutKeys3_Vtbl, 0x7b81448c_418e_469c_a49a_45b597c826b6);
impl windows_core::RuntimeType for IAppCaptureAlternateShortcutKeys3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureAlternateShortcutKeys3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKeyModifiers: usize,
}
windows_core::imp::define_interface!(IAppCaptureDurationGeneratedEventArgs, IAppCaptureDurationGeneratedEventArgs_Vtbl, 0xc1f5563b_ffa1_44c9_975f_27fbeb553b35);
impl windows_core::RuntimeType for IAppCaptureDurationGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureDurationGeneratedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureFileGeneratedEventArgs, IAppCaptureFileGeneratedEventArgs_Vtbl, 0x4189fbf4_465e_45bf_907f_165b3fb23758);
impl windows_core::RuntimeType for IAppCaptureFileGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureFileGeneratedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
windows_core::imp::define_interface!(IAppCaptureManagerStatics, IAppCaptureManagerStatics_Vtbl, 0x7d9e3ea7_6282_4735_8d4e_aa45f90f6723);
impl windows_core::RuntimeType for IAppCaptureManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureMetadataWriter, IAppCaptureMetadataWriter_Vtbl, 0xe0ce4877_9aaf_46b4_ad31_6a60b441c780);
impl windows_core::RuntimeType for IAppCaptureMetadataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureMetadataWriter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddStringEvent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, AppCaptureMetadataPriority) -> windows_core::HRESULT,
    pub AddInt32Event: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32, AppCaptureMetadataPriority) -> windows_core::HRESULT,
    pub AddDoubleEvent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f64, AppCaptureMetadataPriority) -> windows_core::HRESULT,
    pub StartStringState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, AppCaptureMetadataPriority) -> windows_core::HRESULT,
    pub StartInt32State: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, i32, AppCaptureMetadataPriority) -> windows_core::HRESULT,
    pub StartDoubleState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, f64, AppCaptureMetadataPriority) -> windows_core::HRESULT,
    pub StopState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StopAllStates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemainingStorageBytesAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub MetadataPurged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMetadataPurged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureMicrophoneCaptureStateChangedEventArgs, IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl, 0x324d249e_45bc_4c35_bc35_e469fc7a69e0);
impl windows_core::RuntimeType for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureMicrophoneCaptureState) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureRecordOperation, IAppCaptureRecordOperation_Vtbl, 0xc66020a9_1538_495c_9bbb_2ba870ec5861);
impl windows_core::RuntimeType for IAppCaptureRecordOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureRecordOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StopRecording: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureRecordingState) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    pub IsFileTruncated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DurationGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDurationGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub FileGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFileGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureRecordingStateChangedEventArgs, IAppCaptureRecordingStateChangedEventArgs_Vtbl, 0x24fc8712_e305_490d_b415_6b1c9049736b);
impl windows_core::RuntimeType for IAppCaptureRecordingStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureRecordingStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureRecordingState) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureServices, IAppCaptureServices_Vtbl, 0x44fec0b5_34f5_4f18_ae8c_b9123abbfc0d);
impl windows_core::RuntimeType for IAppCaptureServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureServices_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Record: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanCapture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureSettings, IAppCaptureSettings_Vtbl, 0x14683a86_8807_48d3_883a_970ee4532a39);
impl windows_core::RuntimeType for IAppCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub SetAppCaptureDestinationFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetAppCaptureDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub AppCaptureDestinationFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    AppCaptureDestinationFolder: usize,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHistoricalBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HistoricalBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHistoricalBufferLengthUnit: unsafe extern "system" fn(*mut core::ffi::c_void, AppCaptureHistoricalBufferLengthUnit) -> windows_core::HRESULT,
    pub HistoricalBufferLengthUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureHistoricalBufferLengthUnit) -> windows_core::HRESULT,
    pub SetIsHistoricalCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetMaximumRecordLength: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MaximumRecordLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SetScreenshotDestinationFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetScreenshotDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub ScreenshotDestinationFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ScreenshotDestinationFolder: usize,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppCaptureVideoEncodingBitrateMode) -> windows_core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureVideoEncodingBitrateMode) -> windows_core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppCaptureVideoEncodingResolutionMode) -> windows_core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureVideoEncodingResolutionMode) -> windows_core::HRESULT,
    pub SetIsAppCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAppCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCpuConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMemoryConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureSettings2, IAppCaptureSettings2_Vtbl, 0xfcb8cee7_e26b_476f_9b1a_ec342d2a8fde);
impl windows_core::RuntimeType for IAppCaptureSettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureSettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsGpuConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AlternateShortcutKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureSettings3, IAppCaptureSettings3_Vtbl, 0xa93502fe_88c2_42d6_aaaa_40feffd75aec);
impl windows_core::RuntimeType for IAppCaptureSettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureSettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsMicrophoneCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureSettings4, IAppCaptureSettings4_Vtbl, 0x07c2774c_1a81_482f_a244_049d95f25b0b);
impl windows_core::RuntimeType for IAppCaptureSettings4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureSettings4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetVideoEncodingFrameRateMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppCaptureVideoEncodingFrameRateMode) -> windows_core::HRESULT,
    pub VideoEncodingFrameRateMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureVideoEncodingFrameRateMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureSettings5, IAppCaptureSettings5_Vtbl, 0x18894522_b0e8_4ba0_8f13_3eaa5fa4013b);
impl windows_core::RuntimeType for IAppCaptureSettings5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureSettings5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureState, IAppCaptureState_Vtbl, 0x73134372_d4eb_44ce_9538_465f506ac4ea);
impl windows_core::RuntimeType for IAppCaptureState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTargetRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MicrophoneCaptureState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppCaptureMicrophoneCaptureState) -> windows_core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CaptureTargetClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureStatics, IAppCaptureStatics_Vtbl, 0xf922dd6c_0a7e_4e74_8b20_9c1f902d08a1);
impl windows_core::RuntimeType for IAppCaptureStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppCaptureStatics2, IAppCaptureStatics2_Vtbl, 0xb2d881d4_836c_4da4_afd7_facc041e1cf3);
impl windows_core::RuntimeType for IAppCaptureStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppCaptureStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAllowedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraCaptureUI, ICameraCaptureUI_Vtbl, 0x48587540_6f93_4bb4_b8f3_e89e48948c91);
impl windows_core::RuntimeType for ICameraCaptureUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraCaptureUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PhotoSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VideoSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CaptureFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, CameraCaptureUIMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CaptureFileAsync: usize,
}
windows_core::imp::define_interface!(ICameraCaptureUIPhotoCaptureSettings, ICameraCaptureUIPhotoCaptureSettings_Vtbl, 0xb9f5be97_3472_46a8_8a9e_04ce42ccc97d);
impl windows_core::RuntimeType for ICameraCaptureUIPhotoCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraCaptureUIPhotoCaptureSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CameraCaptureUIPhotoFormat) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, CameraCaptureUIPhotoFormat) -> windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CameraCaptureUIMaxPhotoResolution) -> windows_core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(*mut core::ffi::c_void, CameraCaptureUIMaxPhotoResolution) -> windows_core::HRESULT,
    pub CroppedSizeInPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetCroppedSizeInPixels: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
    pub CroppedAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetCroppedAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
    pub AllowCropping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowCropping: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraCaptureUIVideoCaptureSettings, ICameraCaptureUIVideoCaptureSettings_Vtbl, 0x64e92d1f_a28d_425a_b84f_e568335ff24e);
impl windows_core::RuntimeType for ICameraCaptureUIVideoCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraCaptureUIVideoCaptureSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CameraCaptureUIVideoFormat) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, CameraCaptureUIVideoFormat) -> windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CameraCaptureUIMaxVideoResolution) -> windows_core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(*mut core::ffi::c_void, CameraCaptureUIMaxVideoResolution) -> windows_core::HRESULT,
    pub MaxDurationInSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMaxDurationInSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub AllowTrimming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowTrimming: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraOptionsUIStatics, ICameraOptionsUIStatics_Vtbl, 0x3b0d5e34_3906_4b7d_946c_7bde844499ae);
impl windows_core::RuntimeType for ICameraOptionsUIStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraOptionsUIStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICapturedFrame, ICapturedFrame_Vtbl, 0x1dd2de1f_571b_44d8_8e80_a08a1578766e);
impl windows_core::RuntimeType for ICapturedFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICapturedFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICapturedFrame2, ICapturedFrame2_Vtbl, 0x543fa6d1_bd78_4866_adda_24314bc65dea);
impl windows_core::RuntimeType for ICapturedFrame2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICapturedFrame2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ControlValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub BitmapProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    BitmapProperties: usize,
}
windows_core::imp::define_interface!(ICapturedFrameControlValues, ICapturedFrameControlValues_Vtbl, 0x90c65b7f_4e0d_4ca4_882d_7a144fed0a90);
impl windows_core::RuntimeType for ICapturedFrameControlValues {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICapturedFrameControlValues_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Exposure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExposureCompensation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsoSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Focus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub SceneMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    SceneMode: usize,
    pub Flashed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlashPowerPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WhiteBalance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ZoomFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICapturedFrameControlValues2, ICapturedFrameControlValues2_Vtbl, 0x500b2b88_06d2_4aa7_a7db_d37af73321d8);
impl windows_core::RuntimeType for ICapturedFrameControlValues2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICapturedFrameControlValues2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
    pub IsoDigitalGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsoAnalogGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub SensorFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SensorFrameRate: usize,
    pub WhiteBalanceGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICapturedFrameWithSoftwareBitmap, ICapturedFrameWithSoftwareBitmap_Vtbl, 0xb58e8b6e_8503_49b5_9e86_897d26a3ff3d);
impl windows_core::RuntimeType for ICapturedFrameWithSoftwareBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICapturedFrameWithSoftwareBitmap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
windows_core::imp::define_interface!(ICapturedPhoto, ICapturedPhoto_Vtbl, 0xb0ce7e5a_cfcc_4d6c_8ad1_0869208aca16);
impl windows_core::RuntimeType for ICapturedPhoto {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICapturedPhoto_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameBarServices, IGameBarServices_Vtbl, 0x2dbead57_50a6_499e_8c6c_d330a7311796);
impl windows_core::RuntimeType for IGameBarServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameBarServices_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TargetCapturePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameBarTargetCapturePolicy) -> windows_core::HRESULT,
    pub EnableCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppBroadcastServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppCaptureServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommandReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCommandReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameBarServicesCommandEventArgs, IGameBarServicesCommandEventArgs_Vtbl, 0xa74226b2_f176_4fcf_8fbb_cf698b2eb8e0);
impl windows_core::RuntimeType for IGameBarServicesCommandEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameBarServicesCommandEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameBarCommand) -> windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameBarCommandOrigin) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameBarServicesManager, IGameBarServicesManager_Vtbl, 0x3a4b9cfa_7f8b_4c60_9dbb_0bcd262dffc6);
impl windows_core::RuntimeType for IGameBarServicesManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameBarServicesManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GameBarServicesCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGameBarServicesCreated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameBarServicesManagerGameBarServicesCreatedEventArgs, IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl, 0xededbd9c_143e_49a3_a5ea_0b1995c8d46e);
impl windows_core::RuntimeType for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GameBarServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameBarServicesManagerStatics, IGameBarServicesManagerStatics_Vtbl, 0x34c1b616_ff25_4792_98f2_d3753f15ac13);
impl windows_core::RuntimeType for IGameBarServicesManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameBarServicesManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameBarServicesTargetInfo, IGameBarServicesTargetInfo_Vtbl, 0xb4202f92_1611_4e05_b6ef_dfd737ae33b0);
impl windows_core::RuntimeType for IGameBarServicesTargetInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameBarServicesTargetInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TitleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameBarServicesDisplayMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILowLagMediaRecording, ILowLagMediaRecording_Vtbl, 0x41c8baf7_ff3f_49f0_a477_f195e3ce5108);
impl windows_core::RuntimeType for ILowLagMediaRecording {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLagMediaRecording_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILowLagMediaRecording2, ILowLagMediaRecording2_Vtbl, 0x6369c758_5644_41e2_97af_8ef56a25e225);
impl windows_core::RuntimeType for ILowLagMediaRecording2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLagMediaRecording2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices")]
    pub PauseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Devices::MediaCapturePauseBehavior, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseAsync: usize,
    pub ResumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILowLagMediaRecording3, ILowLagMediaRecording3_Vtbl, 0x5c33ab12_48f7_47da_b41e_90880a5fe0ec);
impl windows_core::RuntimeType for ILowLagMediaRecording3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLagMediaRecording3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices")]
    pub PauseWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Devices::MediaCapturePauseBehavior, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseWithResultAsync: usize,
    pub StopWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILowLagPhotoCapture, ILowLagPhotoCapture_Vtbl, 0xa37251b7_6b44_473d_8f24_f703d6c0ec44);
impl windows_core::RuntimeType for ILowLagPhotoCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLagPhotoCapture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CaptureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILowLagPhotoSequenceCapture, ILowLagPhotoSequenceCapture_Vtbl, 0x7cc346bb_b9a9_4c91_8ffa_287e9c668669);
impl windows_core::RuntimeType for ILowLagPhotoSequenceCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLagPhotoSequenceCapture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinishAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhotoCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePhotoCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCapture, IMediaCapture_Vtbl, 0xc61afbb4_fb10_4a34_ac18_ca80d9c8e7ee);
impl windows_core::RuntimeType for IMediaCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InitializeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeWithSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub StartRecordToStorageFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    StartRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub StartRecordToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    StartRecordToStreamAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub StartRecordToCustomSinkAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    StartRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkIdAsync: usize,
    pub StopRecordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub CapturePhotoToStorageFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    CapturePhotoToStorageFileAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub CapturePhotoToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    CapturePhotoToStreamAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AddEffectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, MediaStreamType, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AddEffectAsync: usize,
    pub ClearEffectsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, MediaStreamType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEncoderProperty: unsafe extern "system" fn(*mut core::ffi::c_void, MediaStreamType, windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEncoderProperty: unsafe extern "system" fn(*mut core::ffi::c_void, MediaStreamType, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Failed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFailed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RecordLimitationExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRecordLimitationExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MediaCaptureSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
    pub SetPreviewMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetPreviewMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPreviewRotation: unsafe extern "system" fn(*mut core::ffi::c_void, VideoRotation) -> windows_core::HRESULT,
    pub GetPreviewRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VideoRotation) -> windows_core::HRESULT,
    pub SetRecordRotation: unsafe extern "system" fn(*mut core::ffi::c_void, VideoRotation) -> windows_core::HRESULT,
    pub GetRecordRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VideoRotation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCapture2, IMediaCapture2_Vtbl, 0x9cc68260_7da1_4043_b652_21b8878daff9);
impl windows_core::RuntimeType for IMediaCapture2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapture2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareLowLagRecordToStorageFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareLowLagRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareLowLagRecordToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareLowLagRecordToStreamAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareLowLagRecordToCustomSinkAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareLowLagRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkIdAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareLowLagPhotoCaptureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareLowLagPhotoCaptureAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareLowLagPhotoSequenceCaptureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareLowLagPhotoSequenceCaptureAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SetEncodingPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, MediaStreamType, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SetEncodingPropertiesAsync: usize,
}
windows_core::imp::define_interface!(IMediaCapture3, IMediaCapture3_Vtbl, 0xd4136f30_1564_466e_bc0a_af94e02ab016);
impl windows_core::RuntimeType for IMediaCapture3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapture3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub PrepareVariablePhotoSequenceCaptureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture_Core", feature = "Media_MediaProperties")))]
    PrepareVariablePhotoSequenceCaptureAsync: usize,
    pub FocusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFocusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PhotoConfirmationCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePhotoConfirmationCaptured: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCapture4, IMediaCapture4_Vtbl, 0xbacd6fd6_fb08_4947_aea2_ce14eff0ce13);
impl windows_core::RuntimeType for IMediaCapture4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapture4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Effects")]
    pub AddAudioEffectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    AddAudioEffectAsync: usize,
    #[cfg(feature = "Media_Effects")]
    pub AddVideoEffectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MediaStreamType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    AddVideoEffectAsync: usize,
    #[cfg(feature = "Media_Devices")]
    pub PauseRecordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Devices::MediaCapturePauseBehavior, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseRecordAsync: usize,
    pub ResumeRecordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CameraStreamStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCameraStreamStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub CameraStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Devices::CameraStreamState) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    CameraStreamState: usize,
    pub GetPreviewFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviewFrameCopyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThermalStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveThermalStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ThermalStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCaptureThermalStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub PrepareAdvancedPhotoCaptureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PrepareAdvancedPhotoCaptureAsync: usize,
}
windows_core::imp::define_interface!(IMediaCapture5, IMediaCapture5_Vtbl, 0xda787c22_3a9b_4720_a71e_97900a316e5a);
impl windows_core::RuntimeType for IMediaCapture5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapture5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoveEffectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub PauseRecordWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Devices::MediaCapturePauseBehavior, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    PauseRecordWithResultAsync: usize,
    pub StopRecordWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSources: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFrameReaderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFrameReaderAsync: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFrameReaderWithSubtypeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFrameReaderWithSubtypeAsync: usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAndSizeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Graphics::Imaging::BitmapSize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAndSizeAsync: usize,
}
windows_core::imp::define_interface!(IMediaCapture6, IMediaCapture6_Vtbl, 0x228948bd_4b20_4bb1_9fd6_a583212a1012);
impl windows_core::RuntimeType for IMediaCapture6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapture6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub CreateMultiSourceFrameReaderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    CreateMultiSourceFrameReaderAsync: usize,
}
windows_core::imp::define_interface!(IMediaCapture7, IMediaCapture7_Vtbl, 0x9169f102_8888_541a_95bc_24e4d462542a);
impl windows_core::RuntimeType for IMediaCapture7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapture7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateRelativePanelWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, StreamingCaptureMode, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateRelativePanelWatcher: usize,
}
windows_core::imp::define_interface!(IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs, IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl, 0x9d2f920d_a588_43c6_89d6_5ad322af006a);
impl windows_core::RuntimeType for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCaptureDeviceExclusiveControlStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureFailedEventArgs, IMediaCaptureFailedEventArgs_Vtbl, 0x80fde3f4_54c4_42c0_8d19_cea1a87ca18b);
impl windows_core::RuntimeType for IMediaCaptureFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Code: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureFocusChangedEventArgs, IMediaCaptureFocusChangedEventArgs_Vtbl, 0x81e1bc7f_2277_493e_abee_d3f44ff98c04);
impl windows_core::RuntimeType for IMediaCaptureFocusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureFocusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Devices::MediaCaptureFocusState) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
}
windows_core::imp::define_interface!(IMediaCaptureInitializationSettings, IMediaCaptureInitializationSettings_Vtbl, 0x9782ba70_ea65_4900_9356_8ca887726884);
impl windows_core::RuntimeType for IMediaCaptureInitializationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAudioDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AudioDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetVideoDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetStreamingCaptureMode: unsafe extern "system" fn(*mut core::ffi::c_void, StreamingCaptureMode) -> windows_core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StreamingCaptureMode) -> windows_core::HRESULT,
    pub SetPhotoCaptureSource: unsafe extern "system" fn(*mut core::ffi::c_void, PhotoCaptureSource) -> windows_core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhotoCaptureSource) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureInitializationSettings2, IMediaCaptureInitializationSettings2_Vtbl, 0x404e0626_c9dc_43e9_aee4_e6bf1b57b44c);
impl windows_core::RuntimeType for IMediaCaptureInitializationSettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetMediaCategory: unsafe extern "system" fn(*mut core::ffi::c_void, MediaCategory) -> windows_core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCategory) -> windows_core::HRESULT,
    pub SetAudioProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, super::AudioProcessing) -> windows_core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AudioProcessing) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureInitializationSettings3, IMediaCaptureInitializationSettings3_Vtbl, 0x4160519d_be48_4730_8104_0cf6e9e97948);
impl windows_core::RuntimeType for IMediaCaptureInitializationSettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub SetAudioSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetAudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub SetVideoSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetVideoSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
}
windows_core::imp::define_interface!(IMediaCaptureInitializationSettings4, IMediaCaptureInitializationSettings4_Vtbl, 0xf502a537_4cb7_4d28_95ed_4f9f012e0518);
impl windows_core::RuntimeType for IMediaCaptureInitializationSettings4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVideoProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreviewMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPreviewMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecordMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhotoMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPhotoMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureInitializationSettings5, IMediaCaptureInitializationSettings5_Vtbl, 0xd5a2e3b8_2626_4e94_b7b3_5308a0f64b1a);
impl windows_core::RuntimeType for IMediaCaptureInitializationSettings5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SourceGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SourceGroup: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SetSourceGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SetSourceGroup: usize,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCaptureSharingMode) -> windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, MediaCaptureSharingMode) -> windows_core::HRESULT,
    pub MemoryPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCaptureMemoryPreference) -> windows_core::HRESULT,
    pub SetMemoryPreference: unsafe extern "system" fn(*mut core::ffi::c_void, MediaCaptureMemoryPreference) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureInitializationSettings6, IMediaCaptureInitializationSettings6_Vtbl, 0xb2e26b47_3db1_4d33_ab63_0ffa09056585);
impl windows_core::RuntimeType for IMediaCaptureInitializationSettings6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlwaysPlaySystemShutterSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAlwaysPlaySystemShutterSound: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureInitializationSettings7, IMediaCaptureInitializationSettings7_Vtbl, 0x41546967_f58a_5d82_9ef4_ed572fb5e34e);
impl windows_core::RuntimeType for IMediaCaptureInitializationSettings7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub DeviceUriPasswordCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    DeviceUriPasswordCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetDeviceUriPasswordCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetDeviceUriPasswordCredential: usize,
    pub DeviceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDeviceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCapturePauseResult, IMediaCapturePauseResult_Vtbl, 0xaec47ca3_4477_4b04_a06f_2c1c5182fe9d);
impl windows_core::RuntimeType for IMediaCapturePauseResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCapturePauseResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LastFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureRelativePanelWatcher, IMediaCaptureRelativePanelWatcher_Vtbl, 0x7d896566_04be_5b89_b30e_bd34a9f12db0);
impl windows_core::RuntimeType for IMediaCaptureRelativePanelWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureRelativePanelWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub RelativePanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Enumeration::Panel) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RelativePanel: usize,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureSettings, IMediaCaptureSettings_Vtbl, 0x1d83aafe_6d45_4477_8dc4_ac5bc01c4091);
impl windows_core::RuntimeType for IMediaCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StreamingCaptureMode) -> windows_core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhotoCaptureSource) -> windows_core::HRESULT,
    pub VideoDeviceCharacteristic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VideoDeviceCharacteristic) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureSettings2, IMediaCaptureSettings2_Vtbl, 0x6f9e7cfb_fa9f_4b13_9cbe_5ab94f1f3493);
impl windows_core::RuntimeType for IMediaCaptureSettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureSettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConcurrentRecordAndPhotoSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ConcurrentRecordAndPhotoSequenceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CameraSoundRequiredForRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Horizontal35mmEquivalentFocalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PitchOffsetDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Vertical35mmEquivalentFocalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCategory) -> windows_core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AudioProcessing) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureSettings3, IMediaCaptureSettings3_Vtbl, 0x303c67c2_8058_4b1b_b877_8c2ef3528440);
impl windows_core::RuntimeType for IMediaCaptureSettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureSettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
windows_core::imp::define_interface!(IMediaCaptureStatics, IMediaCaptureStatics_Vtbl, 0xacef81ff_99ed_4645_965e_1925cfc63834);
impl windows_core::RuntimeType for IMediaCaptureStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsVideoProfileSupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllVideoProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllVideoProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConcurrentProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConcurrentProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindKnownVideoProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, KnownVideoProfile, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindKnownVideoProfiles: usize,
}
windows_core::imp::define_interface!(IMediaCaptureStopResult, IMediaCaptureStopResult_Vtbl, 0xf9db6a2a_a092_4ad1_97d4_f201f9d082db);
impl windows_core::RuntimeType for IMediaCaptureStopResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureStopResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LastFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureVideoPreview, IMediaCaptureVideoPreview_Vtbl, 0x27727073_549e_447f_a20a_4f03c479d8c0);
impl windows_core::RuntimeType for IMediaCaptureVideoPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureVideoPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartPreviewAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub StartPreviewToCustomSinkAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    StartPreviewToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkIdAsync: usize,
    pub StopPreviewAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCaptureVideoProfile, IMediaCaptureVideoProfile_Vtbl, 0x21a073bf_a3ee_4ecf_9ef6_50b0bc4e1305);
impl windows_core::RuntimeType for IMediaCaptureVideoProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureVideoProfile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPreviewMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPreviewMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedRecordMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedRecordMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPhotoMediaDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPhotoMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConcurrency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConcurrency: usize,
}
windows_core::imp::define_interface!(IMediaCaptureVideoProfile2, IMediaCaptureVideoProfile2_Vtbl, 0x97ddc95f_94ce_468f_9316_fc5bc2638f6b);
impl windows_core::RuntimeType for IMediaCaptureVideoProfile2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureVideoProfile2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSourceInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSourceInfos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IMediaCaptureVideoProfileMediaDescription, IMediaCaptureVideoProfileMediaDescription_Vtbl, 0x8012afef_b691_49ff_83f2_c1e76eaaea1b);
impl windows_core::RuntimeType for IMediaCaptureVideoProfileMediaDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureVideoProfileMediaDescription_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsVariablePhotoSequenceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsVariablePhotoSequenceSupported: usize,
    #[cfg(feature = "deprecated")]
    pub IsHdrVideoSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsHdrVideoSupported: usize,
}
windows_core::imp::define_interface!(IMediaCaptureVideoProfileMediaDescription2, IMediaCaptureVideoProfileMediaDescription2_Vtbl, 0xc6a6ef13_322d_413a_b85a_68a88e02f4e9);
impl windows_core::RuntimeType for IMediaCaptureVideoProfileMediaDescription2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCaptureVideoProfileMediaDescription2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Subtype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IOptionalReferencePhotoCapturedEventArgs, IOptionalReferencePhotoCapturedEventArgs_Vtbl, 0x470f88b3_1e6d_4051_9c8b_f1d85af047b7);
impl windows_core::RuntimeType for IOptionalReferencePhotoCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOptionalReferencePhotoCapturedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoCapturedEventArgs, IPhotoCapturedEventArgs_Vtbl, 0x373bfbc1_984e_4ff0_bf85_1c00aabc5a45);
impl windows_core::RuntimeType for IPhotoCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhotoCapturedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CaptureTimeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoConfirmationCapturedEventArgs, IPhotoConfirmationCapturedEventArgs_Vtbl, 0xab473672_c28a_4827_8f8d_3636d3beb51e);
impl windows_core::RuntimeType for IPhotoConfirmationCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhotoConfirmationCapturedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CaptureTimeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScreenCapture, IScreenCapture_Vtbl, 0x89179ef7_cd12_4e0e_a6d4_5b3de98b2e9b);
impl windows_core::RuntimeType for IScreenCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScreenCapture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
    pub IsAudioSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SourceSuspensionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourceSuspensionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScreenCaptureStatics, IScreenCaptureStatics_Vtbl, 0xc898c3b0_c8a5_11e2_8b8b_0800200c9a66);
impl windows_core::RuntimeType for IScreenCaptureStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScreenCaptureStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISourceSuspensionChangedEventArgs, ISourceSuspensionChangedEventArgs_Vtbl, 0x2ece7b5e_d49b_4394_bc32_f97d6cedec1c);
impl windows_core::RuntimeType for ISourceSuspensionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISourceSuspensionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAudioSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoStreamConfiguration, IVideoStreamConfiguration_Vtbl, 0xd8770a6f_4390_4b5e_ad3e_0f8af0963490);
impl windows_core::RuntimeType for IVideoStreamConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoStreamConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub InputProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    InputProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub OutputProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    OutputProperties: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdvancedCapturedPhoto(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdvancedCapturedPhoto, windows_core::IUnknown, windows_core::IInspectable);
impl AdvancedCapturedPhoto {
    pub fn Frame(&self) -> windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn Mode(&self) -> windows_core::Result<super::Devices::AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Context(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FrameBoundsRelativeToReferencePhoto(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>> {
        let this = &windows_core::Interface::cast::<IAdvancedCapturedPhoto2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameBoundsRelativeToReferencePhoto)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdvancedCapturedPhoto {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdvancedCapturedPhoto>();
}
unsafe impl windows_core::Interface for AdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_Vtbl;
    const IID: windows_core::GUID = <IAdvancedCapturedPhoto as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedCapturedPhoto";
}
unsafe impl Send for AdvancedCapturedPhoto {}
unsafe impl Sync for AdvancedCapturedPhoto {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdvancedPhotoCapture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdvancedPhotoCapture, windows_core::IUnknown, windows_core::IInspectable);
impl AdvancedPhotoCapture {
    pub fn CaptureAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CaptureWithContextAsync<P0>(&self, context: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureWithContextAsync)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OptionalReferencePhotoCaptured<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalReferencePhotoCaptured)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOptionalReferencePhotoCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOptionalReferencePhotoCaptured)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AllPhotosCaptured<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllPhotosCaptured)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAllPhotosCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAllPhotosCaptured)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FinishAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinishAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdvancedPhotoCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdvancedPhotoCapture>();
}
unsafe impl windows_core::Interface for AdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_Vtbl;
    const IID: windows_core::GUID = <IAdvancedPhotoCapture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedPhotoCapture";
}
unsafe impl Send for AdvancedPhotoCapture {}
unsafe impl Sync for AdvancedPhotoCapture {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastBackgroundService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastBackgroundService, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastBackgroundService {
    pub fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlugInState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlugInState(&self) -> windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlugInState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignInInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppBroadcastBackgroundServiceSignInInfo>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignInInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SignInInfo(&self) -> windows_core::Result<AppBroadcastBackgroundServiceSignInInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignInInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStreamInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppBroadcastBackgroundServiceStreamInfo>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStreamInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StreamInfo(&self) -> windows_core::Result<AppBroadcastBackgroundServiceStreamInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BroadcastTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetViewerCount(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewerCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ViewerCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewerCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).TerminateBroadcast)(windows_core::Interface::as_raw(this), reason, providerspecificreason).ok() }
    }
    pub fn HeartbeatRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeartbeatRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeartbeatRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeartbeatRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TitleId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBroadcastTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBroadcastTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BroadcastLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBroadcastLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBroadcastLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BroadcastChannel(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastChannel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBroadcastChannel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBroadcastChannel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BroadcastTitleChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastTitleChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBroadcastTitleChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveBroadcastTitleChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BroadcastLanguageChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastLanguageChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBroadcastLanguageChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveBroadcastLanguageChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BroadcastChannelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastChannelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBroadcastChannelChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveBroadcastChannelChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppBroadcastBackgroundService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastBackgroundService>();
}
unsafe impl windows_core::Interface for AppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastBackgroundService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundService";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastBackgroundServiceSignInInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastBackgroundServiceSignInInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastBackgroundServiceSignInInfo {
    pub fn SignInState(&self) -> windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignInState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOAuthRequestUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOAuthRequestUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OAuthRequestUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OAuthRequestUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOAuthCallbackUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOAuthCallbackUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OAuthCallbackUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OAuthCallbackUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUserName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUserName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignInStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignInStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSignInStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSignInStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UserNameChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserNameChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserNameChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserNameChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppBroadcastBackgroundServiceSignInInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastBackgroundServiceSignInInfo>();
}
unsafe impl windows_core::Interface for AppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastBackgroundServiceSignInInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastBackgroundServiceStreamInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastBackgroundServiceStreamInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastBackgroundServiceStreamInfo {
    pub fn StreamState(&self) -> windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredVideoEncodingBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredVideoEncodingBitrate(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredVideoEncodingBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBandwidthTestBitrate(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBandwidthTestBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BandwidthTestBitrate(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthTestBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioCodec(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudioCodec)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AudioCodec(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioCodec)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BroadcastStreamReader(&self) -> windows_core::Result<AppBroadcastStreamReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastStreamReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StreamStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStreamStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStreamStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn VideoEncodingResolutionChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoEncodingResolutionChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVideoEncodingResolutionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVideoEncodingResolutionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn VideoEncodingBitrateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoEncodingBitrateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVideoEncodingBitrateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVideoEncodingBitrateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReportProblemWithStream(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppBroadcastBackgroundServiceStreamInfo2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportProblemWithStream)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AppBroadcastBackgroundServiceStreamInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastBackgroundServiceStreamInfo>();
}
unsafe impl windows_core::Interface for AppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastBackgroundServiceStreamInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastCameraCaptureStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastCameraCaptureStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastCameraCaptureStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastCameraCaptureStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastCameraCaptureStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs";
}
unsafe impl Send for AppBroadcastCameraCaptureStateChangedEventArgs {}
unsafe impl Sync for AppBroadcastCameraCaptureStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastGlobalSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastGlobalSettings, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastGlobalSettings {
    pub fn IsBroadcastEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBroadcastEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledByPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGpuConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasHardwareEncoder)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAudioCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAudioCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEchoCancellationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEchoCancellationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSystemAudioGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemAudioGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMicrophoneGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCameraCaptureEnabledByDefault)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCameraCaptureEnabledByDefault(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCameraCaptureEnabledByDefault)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSelectedCameraId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedCameraId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SelectedCameraId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedCameraId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCameraOverlayLocation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CameraOverlayLocation(&self) -> windows_core::Result<AppBroadcastCameraOverlayLocation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraOverlayLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCameraOverlaySize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CameraOverlaySize(&self) -> windows_core::Result<AppBroadcastCameraOverlaySize> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraOverlaySize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCursorImageCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastGlobalSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastGlobalSettings>();
}
unsafe impl windows_core::Interface for AppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastGlobalSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastGlobalSettings";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastHeartbeatRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastHeartbeatRequestedEventArgs {
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastHeartbeatRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastHeartbeatRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastHeartbeatRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs";
}
pub struct AppBroadcastManager;
impl AppBroadcastManager {
    pub fn GetGlobalSettings() -> windows_core::Result<AppBroadcastGlobalSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGlobalSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ApplyGlobalSettings<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppBroadcastGlobalSettings>,
    {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ApplyGlobalSettings)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    pub fn GetProviderSettings() -> windows_core::Result<AppBroadcastProviderSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProviderSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ApplyProviderSettings<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppBroadcastProviderSettings>,
    {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ApplyProviderSettings)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastManagerStatics<R, F: FnOnce(&IAppBroadcastManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBroadcastManager, IAppBroadcastManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AppBroadcastManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastMicrophoneCaptureStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastMicrophoneCaptureStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastMicrophoneCaptureStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
unsafe impl Send for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
unsafe impl Sync for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPlugIn(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPlugIn, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPlugIn {
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderSettings(&self) -> windows_core::Result<AppBroadcastProviderSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Logo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastPlugIn {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPlugIn>();
}
unsafe impl windows_core::Interface for AppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPlugIn as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugIn";
}
unsafe impl Send for AppBroadcastPlugIn {}
unsafe impl Sync for AppBroadcastPlugIn {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPlugInManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPlugInManager, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPlugInManager {
    pub fn IsBroadcastProviderAvailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBroadcastProviderAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PlugInList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlugInList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultPlugIn(&self) -> windows_core::Result<AppBroadcastPlugIn> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultPlugIn)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultPlugIn<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppBroadcastPlugIn>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultPlugIn)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<AppBroadcastPlugInManager>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastPlugInManagerStatics<R, F: FnOnce(&IAppBroadcastPlugInManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBroadcastPlugInManager, IAppBroadcastPlugInManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBroadcastPlugInManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPlugInManager>();
}
unsafe impl windows_core::Interface for AppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPlugInManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInManager";
}
unsafe impl Send for AppBroadcastPlugInManager {}
unsafe impl Sync for AppBroadcastPlugInManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPlugInStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPlugInStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPlugInStateChangedEventArgs {
    pub fn PlugInState(&self) -> windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlugInState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastPlugInStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPlugInStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPlugInStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs";
}
unsafe impl Send for AppBroadcastPlugInStateChangedEventArgs {}
unsafe impl Sync for AppBroadcastPlugInStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPreview, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPreview {
    pub fn StopPreview(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopPreview)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PreviewState(&self) -> windows_core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviewStateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewStateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePreviewStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePreviewStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PreviewStreamReader(&self) -> windows_core::Result<AppBroadcastPreviewStreamReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewStreamReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPreview>();
}
unsafe impl windows_core::Interface for AppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreview";
}
unsafe impl Send for AppBroadcastPreview {}
unsafe impl Sync for AppBroadcastPreview {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPreviewStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPreviewStateChangedEventArgs {
    pub fn PreviewState(&self) -> windows_core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastPreviewStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPreviewStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPreviewStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs";
}
unsafe impl Send for AppBroadcastPreviewStateChangedEventArgs {}
unsafe impl Sync for AppBroadcastPreviewStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPreviewStreamReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStreamReader, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPreviewStreamReader {
    pub fn VideoWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VideoHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VideoStride(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoStride)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapPixelFormat(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoBitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapAlphaMode(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoBitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> windows_core::Result<AppBroadcastPreviewStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetNextVideoFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoFrameArrived<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFrameArrived)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVideoFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVideoFrameArrived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppBroadcastPreviewStreamReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPreviewStreamReader>();
}
unsafe impl windows_core::Interface for AppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPreviewStreamReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamReader";
}
unsafe impl Send for AppBroadcastPreviewStreamReader {}
unsafe impl Sync for AppBroadcastPreviewStreamReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPreviewStreamVideoFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStreamVideoFrame, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPreviewStreamVideoFrame {
    pub fn VideoHeader(&self) -> windows_core::Result<AppBroadcastPreviewStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoHeader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastPreviewStreamVideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPreviewStreamVideoFrame>();
}
unsafe impl windows_core::Interface for AppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPreviewStreamVideoFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame";
}
unsafe impl Send for AppBroadcastPreviewStreamVideoFrame {}
unsafe impl Sync for AppBroadcastPreviewStreamVideoFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastPreviewStreamVideoHeader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStreamVideoHeader, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastPreviewStreamVideoHeader {
    pub fn AbsoluteTimestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RelativeTimestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FrameId(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastPreviewStreamVideoHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastPreviewStreamVideoHeader>();
}
unsafe impl windows_core::Interface for AppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastPreviewStreamVideoHeader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader";
}
unsafe impl Send for AppBroadcastPreviewStreamVideoHeader {}
unsafe impl Sync for AppBroadcastPreviewStreamVideoHeader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastProviderSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastProviderSettings, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastProviderSettings {
    pub fn SetDefaultBroadcastTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultBroadcastTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DefaultBroadcastTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultBroadcastTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudioEncodingBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioEncodingBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomVideoEncodingBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomVideoEncodingHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomVideoEncodingHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomVideoEncodingWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomVideoEncodingWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVideoEncodingBitrateMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> windows_core::Result<AppBroadcastVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoEncodingBitrateMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVideoEncodingResolutionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> windows_core::Result<AppBroadcastVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoEncodingResolutionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastProviderSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastProviderSettings>();
}
unsafe impl windows_core::Interface for AppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastProviderSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastProviderSettings";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastServices(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastServices, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastServices {
    pub fn CaptureTargetType(&self) -> windows_core::Result<AppBroadcastCaptureTargetType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureTargetType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCaptureTargetType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BroadcastTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastTitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBroadcastTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBroadcastTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BroadcastLanguage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BroadcastLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBroadcastLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBroadcastLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanCapture(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCapture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnterBroadcastModeAsync<P0>(&self, plugin: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<AppBroadcastPlugIn>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterBroadcastModeAsync)(windows_core::Interface::as_raw(this), plugin.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ExitBroadcastMode)(windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn StartBroadcast(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartBroadcast)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PauseBroadcast(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PauseBroadcast)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ResumeBroadcast(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ResumeBroadcast)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartPreview(&self, desiredsize: super::super::Foundation::Size) -> windows_core::Result<AppBroadcastPreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPreview)(windows_core::Interface::as_raw(this), desiredsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<AppBroadcastState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastServices>();
}
unsafe impl windows_core::Interface for AppBroadcastServices {
    type Vtable = IAppBroadcastServices_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastServices as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastServices";
}
unsafe impl Send for AppBroadcastServices {}
unsafe impl Sync for AppBroadcastServices {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastSignInStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastSignInStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastSignInStateChangedEventArgs {
    pub fn SignInState(&self) -> windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignInState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Result(&self) -> windows_core::Result<AppBroadcastSignInResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastSignInStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastSignInStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastSignInStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastState, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastState {
    pub fn IsCaptureTargetRunning(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCaptureTargetRunning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ViewerCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewerCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldCaptureMicrophone)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShouldCaptureMicrophone)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RestartMicrophoneCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ShouldCaptureCamera(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldCaptureCamera)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShouldCaptureCamera(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShouldCaptureCamera)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartCameraCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RestartCameraCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EncodedVideoSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodedVideoSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MicrophoneCaptureState(&self) -> windows_core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneCaptureState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneCaptureError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CameraCaptureState(&self) -> windows_core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraCaptureState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CameraCaptureError(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraCaptureError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StreamState(&self) -> windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PlugInState(&self) -> windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlugInState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OAuthRequestUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OAuthRequestUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OAuthCallbackUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OAuthCallbackUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn SetAuthenticationResult<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Authentication::Web::WebAuthenticationResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuthenticationResult)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetSignInState(&self, value: AppBroadcastSignInState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignInState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignInState(&self) -> windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignInState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TerminationReason(&self) -> windows_core::Result<AppBroadcastTerminationReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TerminationReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TerminationReasonPlugInSpecific(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TerminationReasonPlugInSpecific)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ViewerCountChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewerCountChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveViewerCountChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveViewerCountChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MicrophoneCaptureStateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneCaptureStateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMicrophoneCaptureStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CameraCaptureStateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraCaptureStateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCameraCaptureStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCameraCaptureStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PlugInStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlugInStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlugInStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlugInStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StreamStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStreamStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStreamStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CaptureTargetClosed<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastState, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureTargetClosed)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCaptureTargetClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCaptureTargetClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppBroadcastState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastState>();
}
unsafe impl windows_core::Interface for AppBroadcastState {
    type Vtable = IAppBroadcastState_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastState";
}
unsafe impl Send for AppBroadcastState {}
unsafe impl Sync for AppBroadcastState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastStreamAudioFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastStreamAudioFrame, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastStreamAudioFrame {
    pub fn AudioHeader(&self) -> windows_core::Result<AppBroadcastStreamAudioHeader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioHeader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioBuffer(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastStreamAudioFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastStreamAudioFrame>();
}
unsafe impl windows_core::Interface for AppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastStreamAudioFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioFrame";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastStreamAudioHeader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastStreamAudioHeader, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastStreamAudioHeader {
    pub fn AbsoluteTimestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RelativeTimestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasDiscontinuity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FrameId(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastStreamAudioHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastStreamAudioHeader>();
}
unsafe impl windows_core::Interface for AppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastStreamAudioHeader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioHeader";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastStreamReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastStreamReader, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastStreamReader {
    pub fn AudioChannels(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioChannels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AudioSampleRate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioSampleRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioAacSequence(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioAacSequence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AudioBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryGetNextAudioFrame(&self) -> windows_core::Result<AppBroadcastStreamAudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetNextAudioFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VideoHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VideoBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> windows_core::Result<AppBroadcastStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetNextVideoFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AudioFrameArrived<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioFrameArrived)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioFrameArrived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn VideoFrameArrived<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFrameArrived)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVideoFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVideoFrameArrived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppBroadcastStreamReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastStreamReader>();
}
unsafe impl windows_core::Interface for AppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastStreamReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamReader";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastStreamStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastStreamStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastStreamStateChangedEventArgs {
    pub fn StreamState(&self) -> windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastStreamStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastStreamStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastStreamStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastStreamVideoFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastStreamVideoFrame, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastStreamVideoFrame {
    pub fn VideoHeader(&self) -> windows_core::Result<AppBroadcastStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoHeader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastStreamVideoFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastStreamVideoFrame>();
}
unsafe impl windows_core::Interface for AppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastStreamVideoFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoFrame";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastStreamVideoHeader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastStreamVideoHeader, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastStreamVideoHeader {
    pub fn AbsoluteTimestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RelativeTimestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativeTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsKeyFrame(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsKeyFrame)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasDiscontinuity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FrameId(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastStreamVideoHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastStreamVideoHeader>();
}
unsafe impl windows_core::Interface for AppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastStreamVideoHeader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoHeader";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastTriggerDetails {
    pub fn BackgroundService(&self) -> windows_core::Result<AppBroadcastBackgroundService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundService)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastTriggerDetails>();
}
unsafe impl windows_core::Interface for AppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppBroadcastViewerCountChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastViewerCountChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastViewerCountChangedEventArgs {
    pub fn ViewerCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewerCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastViewerCountChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastViewerCountChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastViewerCountChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs";
}
unsafe impl Send for AppBroadcastViewerCountChangedEventArgs {}
unsafe impl Sync for AppBroadcastViewerCountChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCapture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCapture, windows_core::IUnknown, windows_core::IInspectable);
impl AppCapture {
    pub fn IsCapturingAudio(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCapturingAudio)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCapturingVideo(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCapturingVideo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CapturingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppCapture, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCapturingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCapturingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<AppCapture> {
        Self::IAppCaptureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetAllowedAsync(allowed: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppCaptureStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAllowedAsync)(windows_core::Interface::as_raw(this), allowed, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppCaptureStatics<R, F: FnOnce(&IAppCaptureStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppCapture, IAppCaptureStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppCaptureStatics2<R, F: FnOnce(&IAppCaptureStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppCapture, IAppCaptureStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCapture>();
}
unsafe impl windows_core::Interface for AppCapture {
    type Vtable = IAppCapture_Vtbl;
    const IID: windows_core::GUID = <IAppCapture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCapture {
    const NAME: &'static str = "Windows.Media.Capture.AppCapture";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureAlternateShortcutKeys(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureAlternateShortcutKeys, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureAlternateShortcutKeys {
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetToggleGameBarKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleGameBarKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetToggleGameBarKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleGameBarKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSaveHistoricalVideoKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveHistoricalVideoKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSaveHistoricalVideoKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveHistoricalVideoKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetToggleRecordingKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleRecordingKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetToggleRecordingKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleRecordingKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTakeScreenshotKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TakeScreenshotKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTakeScreenshotKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TakeScreenshotKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetToggleRecordingIndicatorKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleRecordingIndicatorKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetToggleRecordingIndicatorKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleRecordingIndicatorKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetToggleMicrophoneCaptureKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleMicrophoneCaptureKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetToggleMicrophoneCaptureKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleMicrophoneCaptureKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetToggleCameraCaptureKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleCameraCaptureKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetToggleCameraCaptureKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleCameraCaptureKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKey(&self, value: super::super::System::VirtualKey) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetToggleBroadcastKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleBroadcastKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetToggleBroadcastKeyModifiers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &windows_core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToggleBroadcastKeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppCaptureAlternateShortcutKeys {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureAlternateShortcutKeys>();
}
unsafe impl windows_core::Interface for AppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureAlternateShortcutKeys as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureAlternateShortcutKeys";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureDurationGeneratedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureDurationGeneratedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureDurationGeneratedEventArgs {
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppCaptureDurationGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureDurationGeneratedEventArgs>();
}
unsafe impl windows_core::Interface for AppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureDurationGeneratedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs";
}
unsafe impl Send for AppCaptureDurationGeneratedEventArgs {}
unsafe impl Sync for AppCaptureDurationGeneratedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureFileGeneratedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureFileGeneratedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppCaptureFileGeneratedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureFileGeneratedEventArgs>();
}
unsafe impl windows_core::Interface for AppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureFileGeneratedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureFileGeneratedEventArgs";
}
unsafe impl Send for AppCaptureFileGeneratedEventArgs {}
unsafe impl Sync for AppCaptureFileGeneratedEventArgs {}
pub struct AppCaptureManager;
impl AppCaptureManager {
    pub fn GetCurrentSettings() -> windows_core::Result<AppCaptureSettings> {
        Self::IAppCaptureManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ApplySettings<P0>(appcapturesettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppCaptureSettings>,
    {
        Self::IAppCaptureManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ApplySettings)(windows_core::Interface::as_raw(this), appcapturesettings.param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAppCaptureManagerStatics<R, F: FnOnce(&IAppCaptureManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppCaptureManager, IAppCaptureManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AppCaptureManager {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureMetadataWriter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureMetadataWriter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AppCaptureMetadataWriter, super::super::Foundation::IClosable);
impl AppCaptureMetadataWriter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppCaptureMetadataWriter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AddStringEvent(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING, priority: AppCaptureMetadataPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStringEvent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), priority).ok() }
    }
    pub fn AddInt32Event(&self, name: &windows_core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddInt32Event)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn AddDoubleEvent(&self, name: &windows_core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddDoubleEvent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn StartStringState(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING, priority: AppCaptureMetadataPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartStringState)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(value), priority).ok() }
    }
    pub fn StartInt32State(&self, name: &windows_core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartInt32State)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn StartDoubleState(&self, name: &windows_core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartDoubleState)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn StopState(&self, name: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopState)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name)).ok() }
    }
    pub fn StopAllStates(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopAllStates)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemainingStorageBytesAvailable(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingStorageBytesAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MetadataPurged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MetadataPurged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMetadataPurged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMetadataPurged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AppCaptureMetadataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureMetadataWriter>();
}
unsafe impl windows_core::Interface for AppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureMetadataWriter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMetadataWriter";
}
unsafe impl Send for AppCaptureMetadataWriter {}
unsafe impl Sync for AppCaptureMetadataWriter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureMicrophoneCaptureStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureMicrophoneCaptureStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureMicrophoneCaptureStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs";
}
unsafe impl Send for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
unsafe impl Sync for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureRecordOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureRecordOperation, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureRecordOperation {
    pub fn StopRecording(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopRecording)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn State(&self) -> windows_core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsFileTruncated(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFileTruncated)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DurationGenerated<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DurationGenerated)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDurationGenerated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDurationGenerated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FileGenerated<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileGenerated)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFileGenerated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFileGenerated)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppCaptureRecordOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureRecordOperation>();
}
unsafe impl windows_core::Interface for AppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureRecordOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordOperation";
}
unsafe impl Send for AppCaptureRecordOperation {}
unsafe impl Sync for AppCaptureRecordOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureRecordingStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureRecordingStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureRecordingStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppCaptureRecordingStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureRecordingStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureRecordingStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs";
}
unsafe impl Send for AppCaptureRecordingStateChangedEventArgs {}
unsafe impl Sync for AppCaptureRecordingStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureServices(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureServices, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureServices {
    pub fn Record(&self) -> windows_core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Record)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecordTimeSpan(&self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordTimeSpan)(windows_core::Interface::as_raw(this), starttime, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanCapture(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCapture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<AppCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppCaptureServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureServices>();
}
unsafe impl windows_core::Interface for AppCaptureServices {
    type Vtable = IAppCaptureServices_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureServices as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureServices";
}
unsafe impl Send for AppCaptureServices {}
unsafe impl Sync for AppCaptureServices {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureSettings, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureSettings {
    #[cfg(feature = "Storage")]
    pub fn SetAppCaptureDestinationFolder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::StorageFolder>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppCaptureDestinationFolder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn AppCaptureDestinationFolder(&self) -> windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppCaptureDestinationFolder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudioEncodingBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioEncodingBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAudioCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAudioCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomVideoEncodingBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomVideoEncodingHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomVideoEncodingHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomVideoEncodingWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomVideoEncodingWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHistoricalBufferLength(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHistoricalBufferLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HistoricalBufferLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HistoricalBufferLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHistoricalBufferLengthUnit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HistoricalBufferLengthUnit(&self) -> windows_core::Result<AppCaptureHistoricalBufferLengthUnit> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HistoricalBufferLengthUnit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsHistoricalCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHistoricalCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsHistoricalCaptureOnBatteryAllowed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnBatteryAllowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHistoricalCaptureOnBatteryAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsHistoricalCaptureOnWirelessDisplayAllowed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHistoricalCaptureOnWirelessDisplayAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaximumRecordLength(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaximumRecordLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaximumRecordLength(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaximumRecordLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SetScreenshotDestinationFolder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::StorageFolder>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScreenshotDestinationFolder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn ScreenshotDestinationFolder(&self) -> windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenshotDestinationFolder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVideoEncodingBitrateMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> windows_core::Result<AppCaptureVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoEncodingBitrateMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVideoEncodingResolutionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> windows_core::Result<AppCaptureVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoEncodingResolutionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAppCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAppCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAppCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCpuConstrained(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCpuConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisabledByPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMemoryConstrained(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMemoryConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasHardwareEncoder)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGpuConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AlternateShortcutKeys(&self) -> windows_core::Result<AppCaptureAlternateShortcutKeys> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlternateShortcutKeys)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSystemAudioGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemAudioGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMicrophoneGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVideoEncodingFrameRateMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingFrameRateMode(&self) -> windows_core::Result<AppCaptureVideoEncodingFrameRateMode> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoEncodingFrameRateMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEchoCancellationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEchoCancellationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCursorImageCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureSettings>();
}
unsafe impl windows_core::Interface for AppCaptureSettings {
    type Vtable = IAppCaptureSettings_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureSettings";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppCaptureState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppCaptureState, windows_core::IUnknown, windows_core::IInspectable);
impl AppCaptureState {
    pub fn IsTargetRunning(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTargetRunning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHistoricalCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldCaptureMicrophone)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShouldCaptureMicrophone)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RestartMicrophoneCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MicrophoneCaptureState(&self) -> windows_core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneCaptureState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneCaptureError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MicrophoneCaptureStateChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MicrophoneCaptureStateChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMicrophoneCaptureStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CaptureTargetClosed<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppCaptureState, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureTargetClosed)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCaptureTargetClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCaptureTargetClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AppCaptureState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppCaptureState>();
}
unsafe impl windows_core::Interface for AppCaptureState {
    type Vtable = IAppCaptureState_Vtbl;
    const IID: windows_core::GUID = <IAppCaptureState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureState";
}
unsafe impl Send for AppCaptureState {}
unsafe impl Sync for AppCaptureState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CameraCaptureUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraCaptureUI, windows_core::IUnknown, windows_core::IInspectable);
impl CameraCaptureUI {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CameraCaptureUI, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PhotoSettings(&self) -> windows_core::Result<CameraCaptureUIPhotoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoSettings(&self) -> windows_core::Result<CameraCaptureUIVideoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureFileAsync)(windows_core::Interface::as_raw(this), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CameraCaptureUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraCaptureUI>();
}
unsafe impl windows_core::Interface for CameraCaptureUI {
    type Vtable = ICameraCaptureUI_Vtbl;
    const IID: windows_core::GUID = <ICameraCaptureUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUI";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CameraCaptureUIPhotoCaptureSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraCaptureUIPhotoCaptureSettings, windows_core::IUnknown, windows_core::IInspectable);
impl CameraCaptureUIPhotoCaptureSettings {
    pub fn Format(&self) -> windows_core::Result<CameraCaptureUIPhotoFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> windows_core::Result<CameraCaptureUIMaxPhotoResolution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxResolution)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CroppedSizeInPixels(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CroppedSizeInPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCroppedSizeInPixels(&self, value: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCroppedSizeInPixels)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CroppedAspectRatio(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CroppedAspectRatio)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCroppedAspectRatio(&self, value: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCroppedAspectRatio)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowCropping(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowCropping)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowCropping(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowCropping)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CameraCaptureUIPhotoCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraCaptureUIPhotoCaptureSettings>();
}
unsafe impl windows_core::Interface for CameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_Vtbl;
    const IID: windows_core::GUID = <ICameraCaptureUIPhotoCaptureSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings";
}
unsafe impl Send for CameraCaptureUIPhotoCaptureSettings {}
unsafe impl Sync for CameraCaptureUIPhotoCaptureSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CameraCaptureUIVideoCaptureSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraCaptureUIVideoCaptureSettings, windows_core::IUnknown, windows_core::IInspectable);
impl CameraCaptureUIVideoCaptureSettings {
    pub fn Format(&self) -> windows_core::Result<CameraCaptureUIVideoFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> windows_core::Result<CameraCaptureUIMaxVideoResolution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxResolution)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxResolution)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxDurationInSeconds(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxDurationInSeconds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxDurationInSeconds(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxDurationInSeconds)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowTrimming(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowTrimming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowTrimming(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowTrimming)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CameraCaptureUIVideoCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraCaptureUIVideoCaptureSettings>();
}
unsafe impl windows_core::Interface for CameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_Vtbl;
    const IID: windows_core::GUID = <ICameraCaptureUIVideoCaptureSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings";
}
unsafe impl Send for CameraCaptureUIVideoCaptureSettings {}
unsafe impl Sync for CameraCaptureUIVideoCaptureSettings {}
pub struct CameraOptionsUI;
impl CameraOptionsUI {
    pub fn Show<P0>(mediacapture: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaCapture>,
    {
        Self::ICameraOptionsUIStatics(|this| unsafe { (windows_core::Interface::vtable(this).Show)(windows_core::Interface::as_raw(this), mediacapture.param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ICameraOptionsUIStatics<R, F: FnOnce(&ICameraOptionsUIStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CameraOptionsUI, ICameraOptionsUIStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CameraOptionsUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraOptionsUI";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CapturedFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CapturedFrame, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(CapturedFrame, super::super::Foundation::IClosable, super::super::Storage::Streams::IContentTypeProvider, super::super::Storage::Streams::IInputStream, super::super::Storage::Streams::IOutputStream, super::super::Storage::Streams::IRandomAccessStream, super::super::Storage::Streams::IRandomAccessStreamWithContentType);
impl CapturedFrame {
    pub fn Width(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ControlValues(&self) -> windows_core::Result<CapturedFrameControlValues> {
        let this = &windows_core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlValues)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn BitmapProperties(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapPropertySet> {
        let this = &windows_core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> windows_core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = &windows_core::Interface::cast::<ICapturedFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoftwareBitmap)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
impl windows_core::RuntimeType for CapturedFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICapturedFrame>();
}
unsafe impl windows_core::Interface for CapturedFrame {
    type Vtable = ICapturedFrame_Vtbl;
    const IID: windows_core::GUID = <ICapturedFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrame";
}
unsafe impl Send for CapturedFrame {}
unsafe impl Sync for CapturedFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CapturedFrameControlValues(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CapturedFrameControlValues, windows_core::IUnknown, windows_core::IInspectable);
impl CapturedFrameControlValues {
    pub fn Exposure(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Exposure)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExposureCompensation(&self) -> windows_core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExposureCompensation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsoSpeed(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsoSpeed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Focus(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn SceneMode(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SceneMode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Flashed(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Flashed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlashPowerPercent(&self) -> windows_core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlashPowerPercent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WhiteBalance(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WhiteBalance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ZoomFactor(&self) -> windows_core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZoomFactor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn FocusState(&self) -> windows_core::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>> {
        let this = &windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsoDigitalGain(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsoDigitalGain)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsoAnalogGain(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsoAnalogGain)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SensorFrameRate(&self) -> windows_core::Result<super::MediaProperties::MediaRatio> {
        let this = &windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SensorFrameRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WhiteBalanceGain(&self) -> windows_core::Result<super::super::Foundation::IReference<WhiteBalanceGain>> {
        let this = &windows_core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WhiteBalanceGain)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CapturedFrameControlValues {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICapturedFrameControlValues>();
}
unsafe impl windows_core::Interface for CapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_Vtbl;
    const IID: windows_core::GUID = <ICapturedFrameControlValues as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrameControlValues";
}
unsafe impl Send for CapturedFrameControlValues {}
unsafe impl Sync for CapturedFrameControlValues {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CapturedPhoto(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CapturedPhoto, windows_core::IUnknown, windows_core::IInspectable);
impl CapturedPhoto {
    pub fn Frame(&self) -> windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Thumbnail(&self) -> windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CapturedPhoto {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICapturedPhoto>();
}
unsafe impl windows_core::Interface for CapturedPhoto {
    type Vtable = ICapturedPhoto_Vtbl;
    const IID: windows_core::GUID = <ICapturedPhoto as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.CapturedPhoto";
}
unsafe impl Send for CapturedPhoto {}
unsafe impl Sync for CapturedPhoto {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameBarServices(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameBarServices, windows_core::IUnknown, windows_core::IInspectable);
impl GameBarServices {
    pub fn TargetCapturePolicy(&self) -> windows_core::Result<GameBarTargetCapturePolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetCapturePolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnableCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EnableCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DisableCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DisableCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TargetInfo(&self) -> windows_core::Result<GameBarServicesTargetInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppBroadcastServices(&self) -> windows_core::Result<AppBroadcastServices> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppBroadcastServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppCaptureServices(&self) -> windows_core::Result<AppCaptureServices> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppCaptureServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CommandReceived<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandReceived)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCommandReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCommandReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for GameBarServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameBarServices>();
}
unsafe impl windows_core::Interface for GameBarServices {
    type Vtable = IGameBarServices_Vtbl;
    const IID: windows_core::GUID = <IGameBarServices as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServices";
}
unsafe impl Send for GameBarServices {}
unsafe impl Sync for GameBarServices {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameBarServicesCommandEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameBarServicesCommandEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GameBarServicesCommandEventArgs {
    pub fn Command(&self) -> windows_core::Result<GameBarCommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Command)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Origin(&self) -> windows_core::Result<GameBarCommandOrigin> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Origin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GameBarServicesCommandEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameBarServicesCommandEventArgs>();
}
unsafe impl windows_core::Interface for GameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGameBarServicesCommandEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesCommandEventArgs";
}
unsafe impl Send for GameBarServicesCommandEventArgs {}
unsafe impl Sync for GameBarServicesCommandEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameBarServicesManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameBarServicesManager, windows_core::IUnknown, windows_core::IInspectable);
impl GameBarServicesManager {
    pub fn GameBarServicesCreated<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameBarServicesCreated)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGameBarServicesCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGameBarServicesCreated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<GameBarServicesManager> {
        Self::IGameBarServicesManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGameBarServicesManagerStatics<R, F: FnOnce(&IGameBarServicesManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameBarServicesManager, IGameBarServicesManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GameBarServicesManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameBarServicesManager>();
}
unsafe impl windows_core::Interface for GameBarServicesManager {
    type Vtable = IGameBarServicesManager_Vtbl;
    const IID: windows_core::GUID = <IGameBarServicesManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManager";
}
unsafe impl Send for GameBarServicesManager {}
unsafe impl Sync for GameBarServicesManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameBarServicesManagerGameBarServicesCreatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GameBarServicesManagerGameBarServicesCreatedEventArgs {
    pub fn GameBarServices(&self) -> windows_core::Result<GameBarServices> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameBarServices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameBarServicesManagerGameBarServicesCreatedEventArgs>();
}
unsafe impl windows_core::Interface for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGameBarServicesManagerGameBarServicesCreatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs";
}
unsafe impl Send for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
unsafe impl Sync for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameBarServicesTargetInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameBarServicesTargetInfo, windows_core::IUnknown, windows_core::IInspectable);
impl GameBarServicesTargetInfo {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TitleId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayMode(&self) -> windows_core::Result<GameBarServicesDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GameBarServicesTargetInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameBarServicesTargetInfo>();
}
unsafe impl windows_core::Interface for GameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_Vtbl;
    const IID: windows_core::GUID = <IGameBarServicesTargetInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesTargetInfo";
}
unsafe impl Send for GameBarServicesTargetInfo {}
unsafe impl Sync for GameBarServicesTargetInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LowLagMediaRecording(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LowLagMediaRecording, windows_core::IUnknown, windows_core::IInspectable);
impl LowLagMediaRecording {
    pub fn StartAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FinishAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinishAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseAsync)(windows_core::Interface::as_raw(this), behavior, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResumeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResumeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &windows_core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseWithResultAsync)(windows_core::Interface::as_raw(this), behavior, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopWithResultAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &windows_core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopWithResultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LowLagMediaRecording {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILowLagMediaRecording>();
}
unsafe impl windows_core::Interface for LowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_Vtbl;
    const IID: windows_core::GUID = <ILowLagMediaRecording as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.LowLagMediaRecording";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LowLagPhotoCapture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LowLagPhotoCapture, windows_core::IUnknown, windows_core::IInspectable);
impl LowLagPhotoCapture {
    pub fn CaptureAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FinishAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinishAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LowLagPhotoCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILowLagPhotoCapture>();
}
unsafe impl windows_core::Interface for LowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_Vtbl;
    const IID: windows_core::GUID = <ILowLagPhotoCapture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoCapture";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LowLagPhotoSequenceCapture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LowLagPhotoSequenceCapture, windows_core::IUnknown, windows_core::IInspectable);
impl LowLagPhotoSequenceCapture {
    pub fn StartAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FinishAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinishAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhotoCaptured<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoCaptured)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePhotoCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePhotoCaptured)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for LowLagPhotoSequenceCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILowLagPhotoSequenceCapture>();
}
unsafe impl windows_core::Interface for LowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_Vtbl;
    const IID: windows_core::GUID = <ILowLagPhotoSequenceCapture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoSequenceCapture";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCapture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCapture, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaCapture, super::super::Foundation::IClosable);
impl MediaCapture {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaCapture, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InitializeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitializeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InitializeWithSettingsAsync<P0>(&self, mediacaptureinitializationsettings: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<MediaCaptureInitializationSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitializeWithSettingsAsync)(windows_core::Interface::as_raw(this), mediacaptureinitializationsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn StartRecordToStorageFileAsync<P0, P1>(&self, encodingprofile: P0, file: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartRecordToStorageFileAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn StartRecordToStreamAsync<P0, P1>(&self, encodingprofile: P0, stream: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartRecordToStreamAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn StartRecordToCustomSinkAsync<P0, P1>(&self, encodingprofile: P0, custommediasink: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::IMediaExtension>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartRecordToCustomSinkAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), custommediasink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkIdAsync<P0, P1>(&self, encodingprofile: P0, customsinkactivationid: &windows_core::HSTRING, customsinksettings: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartRecordToCustomSinkIdAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), core::mem::transmute_copy(customsinkactivationid), customsinksettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopRecordAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopRecordAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CapturePhotoToStorageFileAsync<P0, P1>(&self, r#type: P0, file: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::ImageEncodingProperties>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePhotoToStorageFileAsync)(windows_core::Interface::as_raw(this), r#type.param().abi(), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn CapturePhotoToStreamAsync<P0, P1>(&self, r#type: P0, stream: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::ImageEncodingProperties>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePhotoToStreamAsync)(windows_core::Interface::as_raw(this), r#type.param().abi(), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AddEffectAsync<P0>(&self, mediastreamtype: MediaStreamType, effectactivationid: &windows_core::HSTRING, effectsettings: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddEffectAsync)(windows_core::Interface::as_raw(this), mediastreamtype, core::mem::transmute_copy(effectactivationid), effectsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearEffectsAsync)(windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEncoderProperty<P0>(&self, mediastreamtype: MediaStreamType, propertyid: windows_core::GUID, propertyvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncoderProperty)(windows_core::Interface::as_raw(this), mediastreamtype, propertyid, propertyvalue.param().abi()).ok() }
    }
    pub fn GetEncoderProperty(&self, mediastreamtype: MediaStreamType, propertyid: windows_core::GUID) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEncoderProperty)(windows_core::Interface::as_raw(this), mediastreamtype, propertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Failed<P0>(&self, erroreventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<MediaCaptureFailedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Failed)(windows_core::Interface::as_raw(this), erroreventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFailed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFailed)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn RecordLimitationExceeded<P0>(&self, recordlimitationexceededeventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<RecordLimitationExceededEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordLimitationExceeded)(windows_core::Interface::as_raw(this), recordlimitationexceededeventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRecordLimitationExceeded(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRecordLimitationExceeded)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn MediaCaptureSettings(&self) -> windows_core::Result<MediaCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaCaptureSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> windows_core::Result<super::Devices::AudioDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDeviceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> windows_core::Result<super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPreviewMirroring(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPreviewMirroring)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPreviewMirroring(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPreviewMirroring)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreviewRotation(&self, value: VideoRotation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPreviewRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPreviewRotation(&self) -> windows_core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPreviewRotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRecordRotation(&self, value: VideoRotation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRecordRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetRecordRotation(&self) -> windows_core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecordRotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareLowLagRecordToStorageFileAsync<P0, P1>(&self, encodingprofile: P0, file: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareLowLagRecordToStorageFileAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareLowLagRecordToStreamAsync<P0, P1>(&self, encodingprofile: P0, stream: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareLowLagRecordToStreamAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareLowLagRecordToCustomSinkAsync<P0, P1>(&self, encodingprofile: P0, custommediasink: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::IMediaExtension>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), custommediasink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkIdAsync<P0, P1>(&self, encodingprofile: P0, customsinkactivationid: &windows_core::HSTRING, customsinksettings: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkIdAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), core::mem::transmute_copy(customsinkactivationid), customsinksettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareLowLagPhotoCaptureAsync<P0>(&self, r#type: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>
    where
        P0: windows_core::Param<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareLowLagPhotoCaptureAsync)(windows_core::Interface::as_raw(this), r#type.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareLowLagPhotoSequenceCaptureAsync<P0>(&self, r#type: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>
    where
        P0: windows_core::Param<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareLowLagPhotoSequenceCaptureAsync)(windows_core::Interface::as_raw(this), r#type.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SetEncodingPropertiesAsync<P0, P1>(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: P0, encoderproperties: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::IMediaEncodingProperties>,
        P1: windows_core::Param<super::MediaProperties::MediaPropertySet>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetEncodingPropertiesAsync)(windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.param().abi(), encoderproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub fn PrepareVariablePhotoSequenceCaptureAsync<P0>(&self, r#type: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>
    where
        P0: windows_core::Param<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareVariablePhotoSequenceCaptureAsync)(windows_core::Interface::as_raw(this), r#type.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FocusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFocusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PhotoConfirmationCaptured<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoConfirmationCaptured)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePhotoConfirmationCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePhotoConfirmationCaptured)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn AddAudioEffectAsync<P0>(&self, definition: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddAudioEffectAsync)(windows_core::Interface::as_raw(this), definition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn AddVideoEffectAsync<P0>(&self, definition: P0, mediastreamtype: MediaStreamType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>
    where
        P0: windows_core::Param<super::Effects::IVideoEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddVideoEffectAsync)(windows_core::Interface::as_raw(this), definition.param().abi(), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseRecordAsync)(windows_core::Interface::as_raw(this), behavior, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResumeRecordAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResumeRecordAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CameraStreamStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaCapture, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraStreamStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCameraStreamStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCameraStreamStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn CameraStreamState(&self) -> windows_core::Result<super::Devices::CameraStreamState> {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraStreamState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPreviewFrameAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPreviewFrameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPreviewFrameCopyAsync<P0>(&self, destination: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>
    where
        P0: windows_core::Param<super::VideoFrame>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPreviewFrameCopyAsync)(windows_core::Interface::as_raw(this), destination.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThermalStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaCapture, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThermalStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveThermalStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveThermalStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ThermalStatus(&self) -> windows_core::Result<MediaCaptureThermalStatus> {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThermalStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PrepareAdvancedPhotoCaptureAsync<P0>(&self, encodingproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>
    where
        P0: windows_core::Param<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrepareAdvancedPhotoCaptureAsync)(windows_core::Interface::as_raw(this), encodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveEffectAsync<P0>(&self, effect: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::IMediaExtension>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveEffectAsync)(windows_core::Interface::as_raw(this), effect.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PauseRecordWithResultAsync)(windows_core::Interface::as_raw(this), behavior, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopRecordWithResultAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopRecordWithResultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSources(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, Frames::MediaFrameSource>> {
        let this = &windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameSources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn CreateFrameReaderAsync<P0>(&self, inputsource: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>
    where
        P0: windows_core::Param<Frames::MediaFrameSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameReaderAsync)(windows_core::Interface::as_raw(this), inputsource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn CreateFrameReaderWithSubtypeAsync<P0>(&self, inputsource: P0, outputsubtype: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>
    where
        P0: windows_core::Param<Frames::MediaFrameSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameReaderWithSubtypeAsync)(windows_core::Interface::as_raw(this), inputsource.param().abi(), core::mem::transmute_copy(outputsubtype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAndSizeAsync<P0>(&self, inputsource: P0, outputsubtype: &windows_core::HSTRING, outputsize: super::super::Graphics::Imaging::BitmapSize) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>
    where
        P0: windows_core::Param<Frames::MediaFrameSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameReaderWithSubtypeAndSizeAsync)(windows_core::Interface::as_raw(this), inputsource.param().abi(), core::mem::transmute_copy(outputsubtype), outputsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CaptureDeviceExclusiveControlStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureDeviceExclusiveControlStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCaptureDeviceExclusiveControlStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCaptureDeviceExclusiveControlStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn CreateMultiSourceFrameReaderAsync<P0>(&self, inputsources: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMultiSourceFrameReaderAsync)(windows_core::Interface::as_raw(this), inputsources.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateRelativePanelWatcher<P0>(&self, capturemode: StreamingCaptureMode, displayregion: P0) -> windows_core::Result<MediaCaptureRelativePanelWatcher>
    where
        P0: windows_core::Param<super::super::UI::WindowManagement::DisplayRegion>,
    {
        let this = &windows_core::Interface::cast::<IMediaCapture7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRelativePanelWatcher)(windows_core::Interface::as_raw(this), capturemode, displayregion.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsVideoProfileSupported(videodeviceid: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoProfileSupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(videodeviceid), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllVideoProfiles(videodeviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllVideoProfiles)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(videodeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConcurrentProfiles(videodeviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindConcurrentProfiles)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(videodeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindKnownVideoProfiles(videodeviceid: &windows_core::HSTRING, name: KnownVideoProfile) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindKnownVideoProfiles)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(videodeviceid), name, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn StartPreviewAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPreviewAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn StartPreviewToCustomSinkAsync<P0, P1>(&self, encodingprofile: P0, custommediasink: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::IMediaExtension>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPreviewToCustomSinkAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), custommediasink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkIdAsync<P0, P1>(&self, encodingprofile: P0, customsinkactivationid: &windows_core::HSTRING, customsinksettings: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
        P1: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartPreviewToCustomSinkIdAsync)(windows_core::Interface::as_raw(this), encodingprofile.param().abi(), core::mem::transmute_copy(customsinkactivationid), customsinksettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopPreviewAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopPreviewAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IMediaCaptureStatics<R, F: FnOnce(&IMediaCaptureStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaCapture, IMediaCaptureStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MediaCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCapture>();
}
unsafe impl windows_core::Interface for MediaCapture {
    type Vtable = IMediaCapture_Vtbl;
    const IID: windows_core::GUID = <IMediaCapture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapture";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureDeviceExclusiveControlStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<MediaCaptureDeviceExclusiveControlStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
unsafe impl Send for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
unsafe impl Sync for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCaptureFailedEventArgs {
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Code(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaCaptureFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureFailedEventArgs>();
}
unsafe impl windows_core::Interface for MediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFailedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureFocusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureFocusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCaptureFocusChangedEventArgs {
    #[cfg(feature = "Media_Devices")]
    pub fn FocusState(&self) -> windows_core::Result<super::Devices::MediaCaptureFocusState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaCaptureFocusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureFocusChangedEventArgs>();
}
unsafe impl windows_core::Interface for MediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureFocusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFocusChangedEventArgs";
}
unsafe impl Send for MediaCaptureFocusChangedEventArgs {}
unsafe impl Sync for MediaCaptureFocusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureInitializationSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureInitializationSettings, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCaptureInitializationSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaCaptureInitializationSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetAudioDeviceId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudioDeviceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AudioDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVideoDeviceId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVideoDeviceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn VideoDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStreamingCaptureMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StreamingCaptureMode(&self) -> windows_core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamingCaptureMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPhotoCaptureSource)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhotoCaptureSource(&self) -> windows_core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoCaptureSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMediaCategory(&self, value: MediaCategory) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMediaCategory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaCategory(&self) -> windows_core::Result<MediaCategory> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioProcessing(&self, value: super::AudioProcessing) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioProcessing(&self) -> windows_core::Result<super::AudioProcessing> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetAudioSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::IMediaSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> windows_core::Result<super::Core::IMediaSource> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetVideoSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::IMediaSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVideoSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> windows_core::Result<super::Core::IMediaSource> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoProfile(&self) -> windows_core::Result<MediaCaptureVideoProfile> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoProfile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVideoProfile<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaCaptureVideoProfile>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVideoProfile)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PreviewMediaDescription(&self) -> windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviewMediaDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPreviewMediaDescription<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaCaptureVideoProfileMediaDescription>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPreviewMediaDescription)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RecordMediaDescription(&self) -> windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordMediaDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRecordMediaDescription<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaCaptureVideoProfileMediaDescription>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRecordMediaDescription)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PhotoMediaDescription(&self) -> windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoMediaDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPhotoMediaDescription<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaCaptureVideoProfileMediaDescription>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPhotoMediaDescription)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SourceGroup(&self) -> windows_core::Result<Frames::MediaFrameSourceGroup> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SetSourceGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Frames::MediaFrameSourceGroup>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSourceGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SharingMode(&self) -> windows_core::Result<MediaCaptureSharingMode> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSharingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MemoryPreference(&self) -> windows_core::Result<MediaCaptureMemoryPreference> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MemoryPreference)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMemoryPreference)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysPlaySystemShutterSound(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlwaysPlaySystemShutterSound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlwaysPlaySystemShutterSound)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn DeviceUriPasswordCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceUriPasswordCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetDeviceUriPasswordCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDeviceUriPasswordCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DeviceUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDeviceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDeviceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for MediaCaptureInitializationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureInitializationSettings>();
}
unsafe impl windows_core::Interface for MediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureInitializationSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureInitializationSettings";
}
unsafe impl Send for MediaCaptureInitializationSettings {}
unsafe impl Sync for MediaCaptureInitializationSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCapturePauseResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCapturePauseResult, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaCapturePauseResult, super::super::Foundation::IClosable);
impl MediaCapturePauseResult {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LastFrame(&self) -> windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecordDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaCapturePauseResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCapturePauseResult>();
}
unsafe impl windows_core::Interface for MediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_Vtbl;
    const IID: windows_core::GUID = <IMediaCapturePauseResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapturePauseResult";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureRelativePanelWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureRelativePanelWatcher, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaCaptureRelativePanelWatcher, super::super::Foundation::IClosable);
impl MediaCaptureRelativePanelWatcher {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RelativePanel(&self) -> windows_core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelativePanel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, windows_core::IInspectable>>,
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
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for MediaCaptureRelativePanelWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureRelativePanelWatcher>();
}
unsafe impl windows_core::Interface for MediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureRelativePanelWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureRelativePanelWatcher";
}
unsafe impl Send for MediaCaptureRelativePanelWatcher {}
unsafe impl Sync for MediaCaptureRelativePanelWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureSettings, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCaptureSettings {
    pub fn AudioDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StreamingCaptureMode(&self) -> windows_core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamingCaptureMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhotoCaptureSource(&self) -> windows_core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoCaptureSource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VideoDeviceCharacteristic(&self) -> windows_core::Result<VideoDeviceCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceCharacteristic)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConcurrentRecordAndPhotoSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConcurrentRecordAndPhotoSequenceSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CameraSoundRequiredForRegion(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraSoundRequiredForRegion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Horizontal35mmEquivalentFocalLength(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Horizontal35mmEquivalentFocalLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PitchOffsetDegrees(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PitchOffsetDegrees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Vertical35mmEquivalentFocalLength(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Vertical35mmEquivalentFocalLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MediaCategory(&self) -> windows_core::Result<MediaCategory> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AudioProcessing(&self) -> windows_core::Result<super::AudioProcessing> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> windows_core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = &windows_core::Interface::cast::<IMediaCaptureSettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direct3D11Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureSettings>();
}
unsafe impl windows_core::Interface for MediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureSettings";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureStopResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureStopResult, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaCaptureStopResult, super::super::Foundation::IClosable);
impl MediaCaptureStopResult {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LastFrame(&self) -> windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecordDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecordDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaCaptureStopResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureStopResult>();
}
unsafe impl windows_core::Interface for MediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureStopResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureStopResult";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureVideoProfile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureVideoProfile, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCaptureVideoProfile {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VideoDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPreviewMediaDescription(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPreviewMediaDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedRecordMediaDescription(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedRecordMediaDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPhotoMediaDescription(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPhotoMediaDescription)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConcurrency(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConcurrency)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSourceInfos(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>> {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameSourceInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::GUID, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaCaptureVideoProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureVideoProfile>();
}
unsafe impl windows_core::Interface for MediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureVideoProfile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfile";
}
unsafe impl Send for MediaCaptureVideoProfile {}
unsafe impl Sync for MediaCaptureVideoProfile {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaCaptureVideoProfileMediaDescription(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCaptureVideoProfileMediaDescription, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCaptureVideoProfileMediaDescription {
    pub fn Width(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FrameRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsVariablePhotoSequenceSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVariablePhotoSequenceSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsHdrVideoSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHdrVideoSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Subtype(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subtype)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::GUID, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaCaptureVideoProfileMediaDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCaptureVideoProfileMediaDescription>();
}
unsafe impl windows_core::Interface for MediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_Vtbl;
    const IID: windows_core::GUID = <IMediaCaptureVideoProfileMediaDescription as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription";
}
unsafe impl Send for MediaCaptureVideoProfileMediaDescription {}
unsafe impl Sync for MediaCaptureVideoProfileMediaDescription {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OptionalReferencePhotoCapturedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OptionalReferencePhotoCapturedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl OptionalReferencePhotoCapturedEventArgs {
    pub fn Frame(&self) -> windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Context(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OptionalReferencePhotoCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOptionalReferencePhotoCapturedEventArgs>();
}
unsafe impl windows_core::Interface for OptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IOptionalReferencePhotoCapturedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs";
}
unsafe impl Send for OptionalReferencePhotoCapturedEventArgs {}
unsafe impl Sync for OptionalReferencePhotoCapturedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhotoCapturedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhotoCapturedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PhotoCapturedEventArgs {
    pub fn Frame(&self) -> windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Thumbnail(&self) -> windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CaptureTimeOffset(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureTimeOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhotoCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhotoCapturedEventArgs>();
}
unsafe impl windows_core::Interface for PhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPhotoCapturedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoCapturedEventArgs";
}
unsafe impl Send for PhotoCapturedEventArgs {}
unsafe impl Sync for PhotoCapturedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhotoConfirmationCapturedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhotoConfirmationCapturedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PhotoConfirmationCapturedEventArgs {
    pub fn Frame(&self) -> windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CaptureTimeOffset(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureTimeOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhotoConfirmationCapturedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhotoConfirmationCapturedEventArgs>();
}
unsafe impl windows_core::Interface for PhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPhotoConfirmationCapturedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoConfirmationCapturedEventArgs";
}
unsafe impl Send for PhotoConfirmationCapturedEventArgs {}
unsafe impl Sync for PhotoConfirmationCapturedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ScreenCapture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScreenCapture, windows_core::IUnknown, windows_core::IInspectable);
impl ScreenCapture {
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> windows_core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> windows_core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsAudioSuspended(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAudioSuspended)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoSuspended)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourceSuspensionChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceSuspensionChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceSuspensionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceSuspensionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<ScreenCapture> {
        Self::IScreenCaptureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IScreenCaptureStatics<R, F: FnOnce(&IScreenCaptureStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScreenCapture, IScreenCaptureStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScreenCapture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScreenCapture>();
}
unsafe impl windows_core::Interface for ScreenCapture {
    type Vtable = IScreenCapture_Vtbl;
    const IID: windows_core::GUID = <IScreenCapture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.ScreenCapture";
}
unsafe impl Send for ScreenCapture {}
unsafe impl Sync for ScreenCapture {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SourceSuspensionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SourceSuspensionChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SourceSuspensionChangedEventArgs {
    pub fn IsAudioSuspended(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAudioSuspended)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoSuspended)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SourceSuspensionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISourceSuspensionChangedEventArgs>();
}
unsafe impl windows_core::Interface for SourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISourceSuspensionChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.SourceSuspensionChangedEventArgs";
}
unsafe impl Send for SourceSuspensionChangedEventArgs {}
unsafe impl Sync for SourceSuspensionChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoStreamConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoStreamConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl VideoStreamConfiguration {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn InputProperties(&self) -> windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn OutputProperties(&self) -> windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VideoStreamConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoStreamConfiguration>();
}
unsafe impl windows_core::Interface for VideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_Vtbl;
    const IID: windows_core::GUID = <IVideoStreamConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.VideoStreamConfiguration";
}
unsafe impl Send for VideoStreamConfiguration {}
unsafe impl Sync for VideoStreamConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl windows_core::TypeKind for AppBroadcastCameraCaptureState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastCameraCaptureState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraCaptureState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastCameraCaptureState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraCaptureState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: Self = Self(0i32);
    pub const TopCenter: Self = Self(1i32);
    pub const TopRight: Self = Self(2i32);
    pub const MiddleLeft: Self = Self(3i32);
    pub const MiddleCenter: Self = Self(4i32);
    pub const MiddleRight: Self = Self(5i32);
    pub const BottomLeft: Self = Self(6i32);
    pub const BottomCenter: Self = Self(7i32);
    pub const BottomRight: Self = Self(8i32);
}
impl windows_core::TypeKind for AppBroadcastCameraOverlayLocation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastCameraOverlayLocation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlayLocation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastCameraOverlayLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlayLocation;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl windows_core::TypeKind for AppBroadcastCameraOverlaySize {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastCameraOverlaySize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlaySize").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastCameraOverlaySize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlaySize;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
}
impl windows_core::TypeKind for AppBroadcastCaptureTargetType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastCaptureTargetType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastCaptureTargetType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastCaptureTargetType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCaptureTargetType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
    pub const AuthorizationFail: Self = Self(2i32);
    pub const ForegroundAppActivated: Self = Self(3i32);
}
impl windows_core::TypeKind for AppBroadcastExitBroadcastModeReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastExitBroadcastModeReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastExitBroadcastModeReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastExitBroadcastModeReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastExitBroadcastModeReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl windows_core::TypeKind for AppBroadcastMicrophoneCaptureState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastMicrophoneCaptureState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastMicrophoneCaptureState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastMicrophoneCaptureState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastMicrophoneCaptureState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastPlugInState(pub i32);
impl AppBroadcastPlugInState {
    pub const Unknown: Self = Self(0i32);
    pub const Initialized: Self = Self(1i32);
    pub const MicrosoftSignInRequired: Self = Self(2i32);
    pub const OAuthSignInRequired: Self = Self(3i32);
    pub const ProviderSignInRequired: Self = Self(4i32);
    pub const InBandwidthTest: Self = Self(5i32);
    pub const ReadyToBroadcast: Self = Self(6i32);
}
impl windows_core::TypeKind for AppBroadcastPlugInState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastPlugInState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastPlugInState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPlugInState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl windows_core::TypeKind for AppBroadcastPreviewState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastPreviewState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastPreviewState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPreviewState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: Self = Self(0i32);
    pub const AuthenticationFailed: Self = Self(1i32);
    pub const Unauthorized: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl windows_core::TypeKind for AppBroadcastSignInResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastSignInResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastSignInResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: Self = Self(0i32);
    pub const MicrosoftSignInInProgress: Self = Self(1i32);
    pub const MicrosoftSignInComplete: Self = Self(2i32);
    pub const OAuthSignInInProgress: Self = Self(3i32);
    pub const OAuthSignInComplete: Self = Self(4i32);
}
impl windows_core::TypeKind for AppBroadcastSignInState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastSignInState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastSignInState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: Self = Self(0i32);
    pub const StreamReady: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Paused: Self = Self(3i32);
    pub const Terminated: Self = Self(4i32);
}
impl windows_core::TypeKind for AppBroadcastStreamState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastStreamState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastStreamState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastStreamState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastTerminationReason(pub i32);
impl AppBroadcastTerminationReason {
    pub const NormalTermination: Self = Self(0i32);
    pub const LostConnectionToService: Self = Self(1i32);
    pub const NoNetworkConnectivity: Self = Self(2i32);
    pub const ServiceAbort: Self = Self(3i32);
    pub const ServiceError: Self = Self(4i32);
    pub const ServiceUnavailable: Self = Self(5i32);
    pub const InternalError: Self = Self(6i32);
    pub const UnsupportedFormat: Self = Self(7i32);
    pub const BackgroundTaskTerminated: Self = Self(8i32);
    pub const BackgroundTaskUnresponsive: Self = Self(9i32);
}
impl windows_core::TypeKind for AppBroadcastTerminationReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastTerminationReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastTerminationReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastTerminationReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastTerminationReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl windows_core::TypeKind for AppBroadcastVideoEncodingBitrateMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastVideoEncodingBitrateMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingBitrateMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl windows_core::TypeKind for AppBroadcastVideoEncodingResolutionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppBroadcastVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppBroadcastVideoEncodingResolutionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingResolutionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: Self = Self(0i32);
    pub const Seconds: Self = Self(1i32);
}
impl windows_core::TypeKind for AppCaptureHistoricalBufferLengthUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppCaptureHistoricalBufferLengthUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppCaptureHistoricalBufferLengthUnit").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppCaptureHistoricalBufferLengthUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureHistoricalBufferLengthUnit;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
}
impl windows_core::TypeKind for AppCaptureMetadataPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppCaptureMetadataPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppCaptureMetadataPriority").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppCaptureMetadataPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMetadataPriority;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl windows_core::TypeKind for AppCaptureMicrophoneCaptureState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppCaptureMicrophoneCaptureState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppCaptureMicrophoneCaptureState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppCaptureMicrophoneCaptureState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMicrophoneCaptureState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl windows_core::TypeKind for AppCaptureRecordingState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppCaptureRecordingState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppCaptureRecordingState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppCaptureRecordingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureRecordingState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl windows_core::TypeKind for AppCaptureVideoEncodingBitrateMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppCaptureVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppCaptureVideoEncodingBitrateMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingBitrateMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for AppCaptureVideoEncodingFrameRateMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppCaptureVideoEncodingFrameRateMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingFrameRateMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppCaptureVideoEncodingFrameRateMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingFrameRateMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl windows_core::TypeKind for AppCaptureVideoEncodingResolutionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppCaptureVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppCaptureVideoEncodingResolutionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingResolutionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const VerySmallQvga: Self = Self(1i32);
    pub const SmallVga: Self = Self(2i32);
    pub const MediumXga: Self = Self(3i32);
    pub const Large3M: Self = Self(4i32);
    pub const VeryLarge5M: Self = Self(5i32);
}
impl windows_core::TypeKind for CameraCaptureUIMaxPhotoResolution {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CameraCaptureUIMaxPhotoResolution {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxPhotoResolution").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CameraCaptureUIMaxPhotoResolution {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxPhotoResolution;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const LowDefinition: Self = Self(1i32);
    pub const StandardDefinition: Self = Self(2i32);
    pub const HighDefinition: Self = Self(3i32);
}
impl windows_core::TypeKind for CameraCaptureUIMaxVideoResolution {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CameraCaptureUIMaxVideoResolution {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxVideoResolution").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CameraCaptureUIMaxVideoResolution {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxVideoResolution;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl windows_core::TypeKind for CameraCaptureUIMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CameraCaptureUIMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CameraCaptureUIMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl windows_core::TypeKind for CameraCaptureUIPhotoFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CameraCaptureUIPhotoFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CameraCaptureUIPhotoFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CameraCaptureUIPhotoFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIPhotoFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl windows_core::TypeKind for CameraCaptureUIVideoFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CameraCaptureUIVideoFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CameraCaptureUIVideoFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CameraCaptureUIVideoFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIVideoFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: Self = Self(0i32);
    pub const MoreSettings: Self = Self(1i32);
}
impl windows_core::TypeKind for ForegroundActivationArgument {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ForegroundActivationArgument {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ForegroundActivationArgument").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ForegroundActivationArgument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.ForegroundActivationArgument;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: Self = Self(0i32);
    pub const RecordHistoricalBuffer: Self = Self(1i32);
    pub const ToggleStartStopRecord: Self = Self(2i32);
    pub const StartRecord: Self = Self(3i32);
    pub const StopRecord: Self = Self(4i32);
    pub const TakeScreenshot: Self = Self(5i32);
    pub const StartBroadcast: Self = Self(6i32);
    pub const StopBroadcast: Self = Self(7i32);
    pub const PauseBroadcast: Self = Self(8i32);
    pub const ResumeBroadcast: Self = Self(9i32);
    pub const ToggleStartStopBroadcast: Self = Self(10i32);
    pub const ToggleMicrophoneCapture: Self = Self(11i32);
    pub const ToggleCameraCapture: Self = Self(12i32);
    pub const ToggleRecordingIndicator: Self = Self(13i32);
}
impl windows_core::TypeKind for GameBarCommand {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameBarCommand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameBarCommand").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameBarCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommand;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: Self = Self(0i32);
    pub const Cortana: Self = Self(1i32);
    pub const AppCommand: Self = Self(2i32);
}
impl windows_core::TypeKind for GameBarCommandOrigin {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameBarCommandOrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameBarCommandOrigin").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameBarCommandOrigin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommandOrigin;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: Self = Self(0i32);
    pub const FullScreenExclusive: Self = Self(1i32);
}
impl windows_core::TypeKind for GameBarServicesDisplayMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameBarServicesDisplayMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameBarServicesDisplayMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameBarServicesDisplayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarServicesDisplayMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: Self = Self(0i32);
    pub const EnabledByUser: Self = Self(1i32);
    pub const NotEnabled: Self = Self(2i32);
    pub const ProhibitedBySystem: Self = Self(3i32);
    pub const ProhibitedByPublisher: Self = Self(4i32);
}
impl windows_core::TypeKind for GameBarTargetCapturePolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameBarTargetCapturePolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameBarTargetCapturePolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameBarTargetCapturePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarTargetCapturePolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KnownVideoProfile(pub i32);
impl KnownVideoProfile {
    pub const VideoRecording: Self = Self(0i32);
    pub const HighQualityPhoto: Self = Self(1i32);
    pub const BalancedVideoAndPhoto: Self = Self(2i32);
    pub const VideoConferencing: Self = Self(3i32);
    pub const PhotoSequence: Self = Self(4i32);
    pub const HighFrameRate: Self = Self(5i32);
    pub const VariablePhotoSequence: Self = Self(6i32);
    pub const HdrWithWcgVideo: Self = Self(7i32);
    pub const HdrWithWcgPhoto: Self = Self(8i32);
    pub const VideoHdr8: Self = Self(9i32);
    pub const CompressedCamera: Self = Self(10i32);
}
impl windows_core::TypeKind for KnownVideoProfile {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KnownVideoProfile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KnownVideoProfile").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for KnownVideoProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.KnownVideoProfile;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCaptureDeviceExclusiveControlReleaseMode(pub i32);
impl MediaCaptureDeviceExclusiveControlReleaseMode {
    pub const OnDispose: Self = Self(0i32);
    pub const OnAllStreamsStopped: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaCaptureDeviceExclusiveControlReleaseMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCaptureDeviceExclusiveControlReleaseMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlReleaseMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlReleaseMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlReleaseMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: Self = Self(0i32);
    pub const SharedReadOnlyAvailable: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaCaptureDeviceExclusiveControlStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCaptureDeviceExclusiveControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaCaptureMemoryPreference {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCaptureMemoryPreference {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCaptureMemoryPreference").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCaptureMemoryPreference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureMemoryPreference;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaCaptureSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCaptureSharingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCaptureSharingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCaptureSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureSharingMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: Self = Self(0i32);
    pub const Overheated: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaCaptureThermalStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCaptureThermalStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCaptureThermalStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCaptureThermalStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureThermalStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const GameChat: Self = Self(3i32);
    pub const Speech: Self = Self(4i32);
    pub const FarFieldSpeech: Self = Self(5i32);
    pub const UniformSpeech: Self = Self(6i32);
    pub const VoiceTyping: Self = Self(7i32);
}
impl windows_core::TypeKind for MediaCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCategory").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCategory;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: Self = Self(0i32);
    pub const VideoRecord: Self = Self(1i32);
    pub const Audio: Self = Self(2i32);
    pub const Photo: Self = Self(3i32);
    pub const Metadata: Self = Self(4i32);
}
impl windows_core::TypeKind for MediaStreamType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaStreamType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaStreamType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaStreamType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaStreamType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: Self = Self(0i32);
    pub const VideoPreview: Self = Self(1i32);
    pub const Photo: Self = Self(2i32);
}
impl windows_core::TypeKind for PhotoCaptureSource {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhotoCaptureSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhotoCaptureSource").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhotoCaptureSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PhotoCaptureSource;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: Self = Self(0i32);
    pub const FiftyHertz: Self = Self(1i32);
    pub const SixtyHertz: Self = Self(2i32);
    pub const Auto: Self = Self(3i32);
}
impl windows_core::TypeKind for PowerlineFrequency {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PowerlineFrequency {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PowerlineFrequency").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PowerlineFrequency {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PowerlineFrequency;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl windows_core::TypeKind for StreamingCaptureMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StreamingCaptureMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StreamingCaptureMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StreamingCaptureMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.StreamingCaptureMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: Self = Self(0i32);
    pub const PreviewRecordStreamsIdentical: Self = Self(1i32);
    pub const PreviewPhotoStreamsIdentical: Self = Self(2i32);
    pub const RecordPhotoStreamsIdentical: Self = Self(3i32);
    pub const AllStreamsIdentical: Self = Self(4i32);
}
impl windows_core::TypeKind for VideoDeviceCharacteristic {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VideoDeviceCharacteristic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VideoDeviceCharacteristic").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VideoDeviceCharacteristic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoDeviceCharacteristic;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl windows_core::TypeKind for VideoRotation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VideoRotation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VideoRotation").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VideoRotation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoRotation;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WhiteBalanceGain {
    pub R: f64,
    pub G: f64,
    pub B: f64,
}
impl windows_core::TypeKind for WhiteBalanceGain {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WhiteBalanceGain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Capture.WhiteBalanceGain;f8;f8;f8)");
}
impl Default for WhiteBalanceGain {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(MediaCaptureFailedEventHandler, MediaCaptureFailedEventHandler_Vtbl, 0x2014effb_5cd8_4f08_a314_0d360da59f14);
impl MediaCaptureFailedEventHandler {
    pub fn new<F: FnMut(Option<&MediaCapture>, Option<&MediaCaptureFailedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = MediaCaptureFailedEventHandlerBox::<F> { vtable: &MediaCaptureFailedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, erroreventargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaCapture>,
        P1: windows_core::Param<MediaCaptureFailedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), erroreventargs.param().abi()).ok() }
    }
}
#[repr(C)]
struct MediaCaptureFailedEventHandlerBox<F: FnMut(Option<&MediaCapture>, Option<&MediaCaptureFailedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const MediaCaptureFailedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&MediaCapture>, Option<&MediaCaptureFailedEventArgs>) -> windows_core::Result<()> + Send + 'static> MediaCaptureFailedEventHandlerBox<F> {
    const VTABLE: MediaCaptureFailedEventHandler_Vtbl = MediaCaptureFailedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <MediaCaptureFailedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, erroreventargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&erroreventargs)).into()
    }
}
impl windows_core::RuntimeType for MediaCaptureFailedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct MediaCaptureFailedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(RecordLimitationExceededEventHandler, RecordLimitationExceededEventHandler_Vtbl, 0x3fae8f2e_4fe1_4ffd_aaba_e1f1337d4e53);
impl RecordLimitationExceededEventHandler {
    pub fn new<F: FnMut(Option<&MediaCapture>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = RecordLimitationExceededEventHandlerBox::<F> { vtable: &RecordLimitationExceededEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaCapture>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
struct RecordLimitationExceededEventHandlerBox<F: FnMut(Option<&MediaCapture>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const RecordLimitationExceededEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&MediaCapture>) -> windows_core::Result<()> + Send + 'static> RecordLimitationExceededEventHandlerBox<F> {
    const VTABLE: RecordLimitationExceededEventHandler_Vtbl = RecordLimitationExceededEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <RecordLimitationExceededEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender)).into()
    }
}
impl windows_core::RuntimeType for RecordLimitationExceededEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct RecordLimitationExceededEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
