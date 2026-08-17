windows_core::imp::define_interface!(IAdaptiveMediaSource, IAdaptiveMediaSource_Vtbl, 0x4c7332ef_d39f_4396_b4d9_043957a7c964);
impl windows_core::RuntimeType for IAdaptiveMediaSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsLive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DesiredLiveOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDesiredLiveOffset: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub InitialBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetInitialBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CurrentDownloadBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentPlaybackBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableBitrates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableBitrates: usize,
    pub DesiredMinBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDesiredMinBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DesiredMaxBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDesiredMaxBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AudioOnlyPlayback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub InboundBitsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub InboundBitsPerSecondWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetInboundBitsPerSecondWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub DownloadBitrateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDownloadBitrateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PlaybackBitrateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePlaybackBitrateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DownloadRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDownloadRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DownloadCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDownloadCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DownloadFailed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDownloadFailed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSource2, IAdaptiveMediaSource2_Vtbl, 0x17890342_6760_4bb9_a58a_f7aa98b08c0e);
impl windows_core::RuntimeType for IAdaptiveMediaSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AdvancedSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSource3, IAdaptiveMediaSource3_Vtbl, 0xba7023fd_c334_461b_a36e_c99f54f7174a);
impl windows_core::RuntimeType for IAdaptiveMediaSource3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSource3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MinLiveOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxSeekableWindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DesiredSeekableWindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDesiredSeekableWindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCorrelatedTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceAdvancedSettings, IAdaptiveMediaSourceAdvancedSettings_Vtbl, 0x55db1680_1aeb_47dc_aa08_9a11610ba45a);
impl windows_core::RuntimeType for IAdaptiveMediaSourceAdvancedSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceAdvancedSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllSegmentsIndependent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllSegmentsIndependent: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DesiredBitrateHeadroomRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDesiredBitrateHeadroomRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BitrateDowngradeTriggerRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBitrateDowngradeTriggerRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceCorrelatedTimes, IAdaptiveMediaSourceCorrelatedTimes_Vtbl, 0x05108787_e032_48e1_ab8d_002b0b3051df);
impl windows_core::RuntimeType for IAdaptiveMediaSourceCorrelatedTimes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceCorrelatedTimes_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PresentationTimeStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProgramDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceCreationResult, IAdaptiveMediaSourceCreationResult_Vtbl, 0x4686b6b2_800f_4e31_9093_76d4782013e7);
impl windows_core::RuntimeType for IAdaptiveMediaSourceCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceCreationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdaptiveMediaSourceCreationStatus) -> windows_core::HRESULT,
    pub MediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceCreationResult2, IAdaptiveMediaSourceCreationResult2_Vtbl, 0x1c3243bf_1c44_404b_a201_df45ac7898e8);
impl windows_core::RuntimeType for IAdaptiveMediaSourceCreationResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceCreationResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDiagnosticAvailableEventArgs, IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl, 0x3af64f06_6d9c_494a_b7a9_b3a5dee6ad68);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DiagnosticType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdaptiveMediaSourceDiagnosticType) -> windows_core::HRESULT,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SegmentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDiagnosticAvailableEventArgs2, IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl, 0x8c6dd857_16a5_4d9f_810e_00bd901b3ef9);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDiagnosticAvailableEventArgs3, IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl, 0xc3650cd5_daeb_4103_84da_68769ad513ff);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDiagnostics, IAdaptiveMediaSourceDiagnostics_Vtbl, 0x9b24ee68_962e_448c_aebf_b29b56098e23);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDiagnostics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnostics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DiagnosticAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDiagnosticAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadBitrateChangedEventArgs, IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl, 0x670c0a44_e04e_4eff_816a_17399f78f4ba);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OldValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2, IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl, 0xf3f1f444_96ae_4de0_b540_2b3246e6968c);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadCompletedEventArgs, IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl, 0x19240dc3_5b37_4a1a_8970_d621cb6ca83b);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdaptiveMediaSourceResourceType) -> windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadCompletedEventArgs2, IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl, 0x704744c4_964a_40e4_af95_9177dd6dfa00);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Statistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadCompletedEventArgs3, IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl, 0x0f8a8bd1_93b2_47c6_badc_8be2c8f7f6e8);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadFailedEventArgs, IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl, 0x37739048_f4ab_40a4_b135_c6dfd8bd7ff1);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdaptiveMediaSourceResourceType) -> windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadFailedEventArgs2, IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl, 0x70919568_967c_4986_90c5_c6fc4b31e2d8);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Statistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadFailedEventArgs3, IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl, 0xd0354549_1132_4a10_915a_c2211b5b9409);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadRequestedDeferral, IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl, 0x05c68f64_fa20_4dbd_9821_4bf4c9bf77ab);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadRequestedEventArgs, IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl, 0xc83fdffd_44a9_47a2_bf96_03398b4bfaaf);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdaptiveMediaSourceResourceType) -> windows_core::HRESULT,
    pub ResourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadRequestedEventArgs2, IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl, 0xb37d8bfe_aa44_4d82_825b_611de3bcfecb);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadRequestedEventArgs3, IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl, 0x333c50fd_4f62_4481_ab44_1e47b0574225);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadResult, IAdaptiveMediaSourceDownloadResult_Vtbl, 0xf4afdc73_bcee_4a6a_9f0a_fec41e2339b0);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetInputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBuffer: usize,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadResult2, IAdaptiveMediaSourceDownloadResult2_Vtbl, 0x15552cb7_7b80_4ac4_8660_a4b97f7c70f0);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceByteRangeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResourceByteRangeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceByteRangeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResourceByteRangeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceDownloadStatistics, IAdaptiveMediaSourceDownloadStatistics_Vtbl, 0xa306cefb_e96a_4dff_a9b8_1ae08c01ae98);
impl windows_core::RuntimeType for IAdaptiveMediaSourceDownloadStatistics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadStatistics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentBytesReceivedCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub TimeToHeadersReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimeToFirstByteReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimeToLastByteReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs, IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl, 0x23a29f6d_7dda_4a51_87a9_6fa8c5b292be);
impl windows_core::RuntimeType for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OldValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AudioOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveMediaSourceStatics, IAdaptiveMediaSourceStatics_Vtbl, 0x50a6bd5d_66ef_4cd3_9579_9e660507dc3f);
impl windows_core::RuntimeType for IAdaptiveMediaSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveMediaSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsContentTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub CreateFromUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub CreateFromUriWithDownloaderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    CreateFromUriWithDownloaderAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamAsync: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Http"))]
    pub CreateFromStreamWithDownloaderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "Web_Http")))]
    CreateFromStreamWithDownloaderAsync: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSource, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Media_Core")]
windows_core::imp::required_hierarchy!(AdaptiveMediaSource, super::super::super::Foundation::IClosable, super::super::Core::IMediaSource);
impl AdaptiveMediaSource {
    pub fn IsLive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DesiredLiveOffset(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredLiveOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredLiveOffset(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredLiveOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InitialBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitialBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInitialBitrate(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInitialBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentDownloadBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentDownloadBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentPlaybackBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentPlaybackBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableBitrates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailableBitrates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DesiredMinBitrate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredMinBitrate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDesiredMinBitrate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredMinBitrate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DesiredMaxBitrate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredMaxBitrate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDesiredMaxBitrate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredMaxBitrate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AudioOnlyPlayback(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioOnlyPlayback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InboundBitsPerSecond(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InboundBitsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InboundBitsPerSecondWindow(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InboundBitsPerSecondWindow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInboundBitsPerSecondWindow(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInboundBitsPerSecondWindow)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DownloadBitrateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadBitrateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDownloadBitrateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDownloadBitrateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PlaybackBitrateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackBitrateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackBitrateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePlaybackBitrateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DownloadRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDownloadRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDownloadRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DownloadCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDownloadCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDownloadCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DownloadFailed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadFailed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDownloadFailed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDownloadFailed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AdvancedSettings(&self) -> windows_core::Result<AdaptiveMediaSourceAdvancedSettings> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvancedSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinLiveOffset(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinLiveOffset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxSeekableWindowSize(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSeekableWindowSize)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DesiredSeekableWindowSize(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSeekableWindowSize)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDesiredSeekableWindowSize<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredSeekableWindowSize)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Diagnostics(&self) -> windows_core::Result<AdaptiveMediaSourceDiagnostics> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Diagnostics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCorrelatedTimes(&self) -> windows_core::Result<AdaptiveMediaSourceCorrelatedTimes> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCorrelatedTimes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsContentTypeSupported(contenttype: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsContentTypeSupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contenttype), &mut result__).map(|| result__)
        })
    }
    pub fn CreateFromUriAsync<P0>(uri: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUriAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Web_Http")]
    pub fn CreateFromUriWithDownloaderAsync<P0, P1>(uri: P0, httpclient: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::super::Web::Http::HttpClient>,
    {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUriWithDownloaderAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), httpclient.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamAsync<P0, P1>(stream: P0, uri: P1, contenttype: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStreamAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), uri.param().abi(), core::mem::transmute_copy(contenttype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Storage_Streams", feature = "Web_Http"))]
    pub fn CreateFromStreamWithDownloaderAsync<P0, P1, P2>(stream: P0, uri: P1, contenttype: &windows_core::HSTRING, httpclient: P2) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
        P2: windows_core::Param<super::super::super::Web::Http::HttpClient>,
    {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStreamWithDownloaderAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), uri.param().abi(), core::mem::transmute_copy(contenttype), httpclient.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAdaptiveMediaSourceStatics<R, F: FnOnce(&IAdaptiveMediaSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AdaptiveMediaSource, IAdaptiveMediaSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSource>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSource {
    type Vtable = IAdaptiveMediaSource_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSource {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSource";
}
unsafe impl Send for AdaptiveMediaSource {}
unsafe impl Sync for AdaptiveMediaSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceAdvancedSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceAdvancedSettings, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceAdvancedSettings {
    pub fn AllSegmentsIndependent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllSegmentsIndependent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllSegmentsIndependent(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllSegmentsIndependent)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredBitrateHeadroomRatio(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredBitrateHeadroomRatio)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDesiredBitrateHeadroomRatio<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<f64>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredBitrateHeadroomRatio)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BitrateDowngradeTriggerRatio(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitrateDowngradeTriggerRatio)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBitrateDowngradeTriggerRatio<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<f64>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBitrateDowngradeTriggerRatio)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceAdvancedSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceAdvancedSettings>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceAdvancedSettings {
    type Vtable = IAdaptiveMediaSourceAdvancedSettings_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceAdvancedSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceAdvancedSettings {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings";
}
unsafe impl Send for AdaptiveMediaSourceAdvancedSettings {}
unsafe impl Sync for AdaptiveMediaSourceAdvancedSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceCorrelatedTimes(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceCorrelatedTimes, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceCorrelatedTimes {
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PresentationTimeStamp(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationTimeStamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProgramDateTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProgramDateTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceCorrelatedTimes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceCorrelatedTimes>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceCorrelatedTimes {
    type Vtable = IAdaptiveMediaSourceCorrelatedTimes_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceCorrelatedTimes as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceCorrelatedTimes {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes";
}
unsafe impl Send for AdaptiveMediaSourceCorrelatedTimes {}
unsafe impl Sync for AdaptiveMediaSourceCorrelatedTimes {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceCreationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceCreationResult, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceCreationResult {
    pub fn Status(&self) -> windows_core::Result<AdaptiveMediaSourceCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MediaSource(&self) -> windows_core::Result<AdaptiveMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> windows_core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HttpResponseMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceCreationResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceCreationResult>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceCreationResult {
    type Vtable = IAdaptiveMediaSourceCreationResult_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceCreationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceCreationResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult";
}
unsafe impl Send for AdaptiveMediaSourceCreationResult {}
unsafe impl Sync for AdaptiveMediaSourceCreationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDiagnosticAvailableEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    pub fn DiagnosticType(&self) -> windows_core::Result<AdaptiveMediaSourceDiagnosticType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiagnosticType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestId(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SegmentId(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SegmentId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceType(&self) -> windows_core::Result<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeOffset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeLength(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Bitrate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bitrate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResourceDuration(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceDuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDiagnosticAvailableEventArgs>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDiagnosticAvailableEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs";
}
unsafe impl Send for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
unsafe impl Sync for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDiagnostics(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDiagnostics, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDiagnostics {
    pub fn DiagnosticAvailable<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiagnosticAvailable)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDiagnosticAvailable(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDiagnosticAvailable)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDiagnostics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDiagnostics>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDiagnostics {
    type Vtable = IAdaptiveMediaSourceDiagnostics_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDiagnostics as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDiagnostics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics";
}
unsafe impl Send for AdaptiveMediaSourceDiagnostics {}
unsafe impl Sync for AdaptiveMediaSourceDiagnostics {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDownloadBitrateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    pub fn OldValue(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NewValue(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Reason(&self) -> windows_core::Result<AdaptiveMediaSourceDownloadBitrateChangedReason> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDownloadBitrateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDownloadBitrateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs";
}
unsafe impl Send for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
unsafe impl Sync for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDownloadCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDownloadCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDownloadCompletedEventArgs {
    pub fn ResourceType(&self) -> windows_core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResourceUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeOffset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeLength(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> windows_core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HttpResponseMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Statistics(&self) -> windows_core::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Statistics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceDuration(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceDuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDownloadCompletedEventArgs>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDownloadCompletedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDownloadCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs";
}
unsafe impl Send for AdaptiveMediaSourceDownloadCompletedEventArgs {}
unsafe impl Sync for AdaptiveMediaSourceDownloadCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDownloadFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDownloadFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDownloadFailedEventArgs {
    pub fn ResourceType(&self) -> windows_core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResourceUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeOffset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeLength(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> windows_core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HttpResponseMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Statistics(&self) -> windows_core::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Statistics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceDuration(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceDuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDownloadFailedEventArgs>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDownloadFailedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDownloadFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDownloadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs";
}
unsafe impl Send for AdaptiveMediaSourceDownloadFailedEventArgs {}
unsafe impl Sync for AdaptiveMediaSourceDownloadFailedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDownloadRequestedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDownloadRequestedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDownloadRequestedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadRequestedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDownloadRequestedDeferral>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDownloadRequestedDeferral {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDownloadRequestedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDownloadRequestedDeferral {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral";
}
unsafe impl Send for AdaptiveMediaSourceDownloadRequestedDeferral {}
unsafe impl Sync for AdaptiveMediaSourceDownloadRequestedDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDownloadRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDownloadRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDownloadRequestedEventArgs {
    pub fn ResourceType(&self) -> windows_core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResourceUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeOffset(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeOffset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceByteRangeLength(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Result(&self) -> windows_core::Result<AdaptiveMediaSourceDownloadResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<AdaptiveMediaSourceDownloadRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceDuration(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceDuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResourceContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDownloadRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDownloadRequestedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDownloadRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs";
}
unsafe impl Send for AdaptiveMediaSourceDownloadRequestedEventArgs {}
unsafe impl Sync for AdaptiveMediaSourceDownloadRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDownloadResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDownloadResult, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDownloadResult {
    pub fn ResourceUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> windows_core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetInputStream<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInputStream)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Buffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBuffer<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBuffer)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ExtendedStatus(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExtendedStatus(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExtendedStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResourceByteRangeOffset(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeOffset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetResourceByteRangeOffset<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u64>>,
    {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResourceByteRangeOffset)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResourceByteRangeLength(&self) -> windows_core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceByteRangeLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetResourceByteRangeLength<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<u64>>,
    {
        let this = &windows_core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResourceByteRangeLength)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDownloadResult>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDownloadResult {
    type Vtable = IAdaptiveMediaSourceDownloadResult_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDownloadResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDownloadResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult";
}
unsafe impl Send for AdaptiveMediaSourceDownloadResult {}
unsafe impl Sync for AdaptiveMediaSourceDownloadResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourceDownloadStatistics(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourceDownloadStatistics, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourceDownloadStatistics {
    pub fn ContentBytesReceivedCount(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentBytesReceivedCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TimeToHeadersReceived(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeToHeadersReceived)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimeToFirstByteReceived(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeToFirstByteReceived)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimeToLastByteReceived(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeToLastByteReceived)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadStatistics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourceDownloadStatistics>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourceDownloadStatistics {
    type Vtable = IAdaptiveMediaSourceDownloadStatistics_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourceDownloadStatistics as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourceDownloadStatistics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics";
}
unsafe impl Send for AdaptiveMediaSourceDownloadStatistics {}
unsafe impl Sync for AdaptiveMediaSourceDownloadStatistics {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdaptiveMediaSourcePlaybackBitrateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveMediaSourcePlaybackBitrateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    pub fn OldValue(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NewValue(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AudioOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs>();
}
unsafe impl windows_core::Interface for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
}
unsafe impl Send for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
unsafe impl Sync for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdaptiveMediaSourceCreationStatus(pub i32);
impl AdaptiveMediaSourceCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const ManifestDownloadFailure: Self = Self(1i32);
    pub const ManifestParseFailure: Self = Self(2i32);
    pub const UnsupportedManifestContentType: Self = Self(3i32);
    pub const UnsupportedManifestVersion: Self = Self(4i32);
    pub const UnsupportedManifestProfile: Self = Self(5i32);
    pub const UnknownFailure: Self = Self(6i32);
}
impl windows_core::TypeKind for AdaptiveMediaSourceCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdaptiveMediaSourceCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdaptiveMediaSourceDiagnosticType(pub i32);
impl AdaptiveMediaSourceDiagnosticType {
    pub const ManifestUnchangedUponReload: Self = Self(0i32);
    pub const ManifestMismatchUponReload: Self = Self(1i32);
    pub const ManifestSignaledEndOfLiveEventUponReload: Self = Self(2i32);
    pub const MediaSegmentSkipped: Self = Self(3i32);
    pub const ResourceNotFound: Self = Self(4i32);
    pub const ResourceTimedOut: Self = Self(5i32);
    pub const ResourceParsingError: Self = Self(6i32);
    pub const BitrateDisabled: Self = Self(7i32);
    pub const FatalMediaSourceError: Self = Self(8i32);
}
impl windows_core::TypeKind for AdaptiveMediaSourceDiagnosticType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdaptiveMediaSourceDiagnosticType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDiagnosticType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDiagnosticType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(pub i32);
impl AdaptiveMediaSourceDownloadBitrateChangedReason {
    pub const SufficientInboundBitsPerSecond: Self = Self(0i32);
    pub const InsufficientInboundBitsPerSecond: Self = Self(1i32);
    pub const LowBufferLevel: Self = Self(2i32);
    pub const PositionChanged: Self = Self(3i32);
    pub const TrackSelectionChanged: Self = Self(4i32);
    pub const DesiredBitratesChanged: Self = Self(5i32);
    pub const ErrorInPreviousBitrate: Self = Self(6i32);
}
impl windows_core::TypeKind for AdaptiveMediaSourceDownloadBitrateChangedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadBitrateChangedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdaptiveMediaSourceResourceType(pub i32);
impl AdaptiveMediaSourceResourceType {
    pub const Manifest: Self = Self(0i32);
    pub const InitializationSegment: Self = Self(1i32);
    pub const MediaSegment: Self = Self(2i32);
    pub const Key: Self = Self(3i32);
    pub const InitializationVector: Self = Self(4i32);
    pub const MediaSegmentIndex: Self = Self(5i32);
}
impl windows_core::TypeKind for AdaptiveMediaSourceResourceType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdaptiveMediaSourceResourceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceResourceType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AdaptiveMediaSourceResourceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceResourceType;i4)");
}
