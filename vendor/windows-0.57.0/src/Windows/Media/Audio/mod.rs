windows_core::imp::define_interface!(IAudioDeviceInputNode, IAudioDeviceInputNode_Vtbl, 0xb01b6be1_6f4e_49e2_ac01_559d62beb3a9);
impl windows_core::RuntimeType for IAudioDeviceInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceInputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
windows_core::imp::define_interface!(IAudioDeviceOutputNode, IAudioDeviceOutputNode_Vtbl, 0x362edbff_ff1c_4434_9e0f_bd2ef522ac82);
impl windows_core::RuntimeType for IAudioDeviceOutputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceOutputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
windows_core::imp::define_interface!(IAudioFileInputNode, IAudioFileInputNode_Vtbl, 0x905b67c8_6f65_4cd4_8890_4694843c276d);
impl windows_core::RuntimeType for IAudioFileInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioFileInputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoopCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLoopCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SourceFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SourceFile: usize,
    pub FileCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFileCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioFileOutputNode, IAudioFileOutputNode_Vtbl, 0x50e01980_5166_4093_80f8_ada00089e9cf);
impl windows_core::RuntimeType for IAudioFileOutputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioFileOutputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub FileEncodingProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FileEncodingProfile: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub FinalizeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    FinalizeAsync: usize,
}
windows_core::imp::define_interface!(IAudioFrameCompletedEventArgs, IAudioFrameCompletedEventArgs_Vtbl, 0xdc7c829e_0208_4504_a5a8_f0f268920a65);
impl windows_core::RuntimeType for IAudioFrameCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioFrameCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioFrameInputNode, IAudioFrameInputNode_Vtbl, 0x01b266c7_fd96_4ff5_a3c5_d27a9bf44237);
impl windows_core::RuntimeType for IAudioFrameInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioFrameInputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AddFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueuedSampleCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub AudioFrameCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioFrameCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub QuantumStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveQuantumStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioFrameOutputNode, IAudioFrameOutputNode_Vtbl, 0xb847371b_3299_45f5_88b3_c9d12a3f1cc8);
impl windows_core::RuntimeType for IAudioFrameOutputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioFrameOutputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioGraph, IAudioGraph_Vtbl, 0x1ad46eed_e48c_4e14_9660_2c4f83e9cdd8);
impl windows_core::RuntimeType for IAudioGraph {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraph_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFrameInputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormat: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateDeviceInputNodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateDeviceInputNodeAsync: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaCategory, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatOnDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaCategory, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatOnDeviceAsync: usize,
    pub CreateFrameOutputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameOutputNodeWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameOutputNodeWithFormat: usize,
    pub CreateDeviceOutputNodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CreateFileInputNodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateFileInputNodeAsync: usize,
    #[cfg(feature = "Storage")]
    pub CreateFileOutputNodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateFileOutputNodeAsync: usize,
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub CreateFileOutputNodeWithFileProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_MediaProperties", feature = "Storage")))]
    CreateFileOutputNodeWithFileProfileAsync: usize,
    pub CreateSubmixNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormat: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetAllNodes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuantumStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveQuantumStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub QuantumProcessed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveQuantumProcessed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub UnrecoverableErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUnrecoverableErrorOccurred: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CompletedQuantumCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub LatencyInSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    pub RenderDeviceAudioProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AudioProcessing) -> windows_core::HRESULT,
    pub SamplesPerQuantum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioGraph2, IAudioGraph2_Vtbl, 0x4e4c3bd5_4fc1_45f6_a947_3cd38f4fd839);
impl windows_core::RuntimeType for IAudioGraph2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraph2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormatAndEmitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormatAndEmitter: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaCategory, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: usize,
    #[cfg(feature = "Storage")]
    pub CreateFileInputNodeWithEmitterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateFileInputNodeWithEmitterAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormatAndEmitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormatAndEmitter: usize,
    pub CreateBatchUpdater: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioGraph3, IAudioGraph3_Vtbl, 0xddcd25ae_1185_42a7_831d_6a9b0fc86820);
impl windows_core::RuntimeType for IAudioGraph3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraph3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub CreateMediaSourceAudioInputNodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    CreateMediaSourceAudioInputNodeAsync: usize,
    #[cfg(feature = "Media_Core")]
    pub CreateMediaSourceAudioInputNodeWithEmitterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    CreateMediaSourceAudioInputNodeWithEmitterAsync: usize,
}
windows_core::imp::define_interface!(IAudioGraphConnection, IAudioGraphConnection_Vtbl, 0x763070ed_d04e_4fac_b233_600b42edd469);
impl windows_core::RuntimeType for IAudioGraphConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraphConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Gain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioGraphSettings, IAudioGraphSettings_Vtbl, 0x1d59647f_e6fe_4628_84f8_9d8bdba25785);
impl windows_core::RuntimeType for IAudioGraphSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraphSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPrimaryRenderDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPrimaryRenderDevice: usize,
    pub QuantumSizeSelectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut QuantumSizeSelectionMode) -> windows_core::HRESULT,
    pub SetQuantumSizeSelectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, QuantumSizeSelectionMode) -> windows_core::HRESULT,
    pub DesiredSamplesPerQuantum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDesiredSamplesPerQuantum: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub AudioRenderCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Render::AudioRenderCategory) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    AudioRenderCategory: usize,
    #[cfg(feature = "Media_Render")]
    pub SetAudioRenderCategory: unsafe extern "system" fn(*mut core::ffi::c_void, super::Render::AudioRenderCategory) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    SetAudioRenderCategory: usize,
    pub DesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AudioProcessing) -> windows_core::HRESULT,
    pub SetDesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, super::AudioProcessing) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioGraphSettings2, IAudioGraphSettings2_Vtbl, 0x72919787_4dab_46e3_b4c9_d8e1a2636062);
impl windows_core::RuntimeType for IAudioGraphSettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraphSettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetMaxPlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MaxPlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioGraphSettingsFactory, IAudioGraphSettingsFactory_Vtbl, 0xa5d91cc6_c2eb_4a61_a214_1d66d75f83da);
impl windows_core::RuntimeType for IAudioGraphSettingsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraphSettingsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Render")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::Render::AudioRenderCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    Create: usize,
}
windows_core::imp::define_interface!(IAudioGraphStatics, IAudioGraphStatics_Vtbl, 0x76ec3132_e159_4ab7_a82a_17beb4b31e94);
impl windows_core::RuntimeType for IAudioGraphStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraphStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioGraphUnrecoverableErrorOccurredEventArgs, IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl, 0xc3d9cbe0_3ff6_4fb3_b262_50d435c55423);
impl windows_core::RuntimeType for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioGraphUnrecoverableError) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioInputNode, IAudioInputNode_Vtbl, 0xd148005c_8428_4784_b7fd_a99d468c5d20);
impl core::ops::Deref for IAudioInputNode {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioInputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAudioInputNode, IAudioNode, super::super::Foundation::IClosable);
impl IAudioInputNode {
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(windows_core::Interface::as_raw(this), destination.param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IAudioInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioInputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub OutgoingConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutgoingConnections: usize,
    pub AddOutgoingConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddOutgoingConnectionWithGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RemoveOutgoingConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioInputNode2, IAudioInputNode2_Vtbl, 0x905156b7_ca68_4c6d_a8bc_e3ee17fe3fd2);
impl core::ops::Deref for IAudioInputNode2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioInputNode2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAudioInputNode2, IAudioInputNode, IAudioNode, super::super::Foundation::IClosable);
impl IAudioInputNode2 {
    pub fn Emitter(&self) -> windows_core::Result<AudioNodeEmitter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emitter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(windows_core::Interface::as_raw(this), destination.param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IAudioInputNode2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioInputNode2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Emitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNode, IAudioNode_Vtbl, 0x15389d7f_dbd8_4819_bf03_668e9357cd6d);
impl core::ops::Deref for IAudioNode {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAudioNode, super::super::Foundation::IClosable);
impl IAudioNode {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IAudioNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub EffectDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    EffectDefinitions: usize,
    pub SetOutgoingGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub OutgoingGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub ConsumeInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetConsumeInput: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Effects")]
    pub DisableEffectsByDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    DisableEffectsByDefinition: usize,
    #[cfg(feature = "Media_Effects")]
    pub EnableEffectsByDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    EnableEffectsByDefinition: usize,
}
windows_core::imp::define_interface!(IAudioNodeEmitter, IAudioNodeEmitter_Vtbl, 0x3676971d_880a_47b8_adf7_1323a9d965be);
impl windows_core::RuntimeType for IAudioNodeEmitter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Direction: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDirection: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDirection: usize,
    pub Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DecayModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Gain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DistanceScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDistanceScale: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DopplerScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDopplerScale: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
    pub IsDopplerDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitter2, IAudioNodeEmitter2_Vtbl, 0x4ab6eecb_ec29_47f8_818c_b6b660a5aeb1);
impl windows_core::RuntimeType for IAudioNodeEmitter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SpatialAudioModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialAudioModel) -> windows_core::HRESULT,
    pub SetSpatialAudioModel: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialAudioModel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitterConeProperties, IAudioNodeEmitterConeProperties_Vtbl, 0xe99b2cee_02ca_4375_9326_0c6ae4bcdfb5);
impl windows_core::RuntimeType for IAudioNodeEmitterConeProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitterConeProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InnerAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub OuterAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub OuterAngleGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitterDecayModel, IAudioNodeEmitterDecayModel_Vtbl, 0x1d1d5af7_0d53_4fa9_bd84_d5816a86f3ff);
impl windows_core::RuntimeType for IAudioNodeEmitterDecayModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitterDecayModel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioNodeEmitterDecayKind) -> windows_core::HRESULT,
    pub MinGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MaxGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub NaturalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitterDecayModelStatics, IAudioNodeEmitterDecayModelStatics_Vtbl, 0xc7787ca8_f178_462f_bc81_8dd5cbe5dae8);
impl windows_core::RuntimeType for IAudioNodeEmitterDecayModelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitterDecayModelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateNatural: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCustom: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitterFactory, IAudioNodeEmitterFactory_Vtbl, 0xfdc8489a_6ad6_4ce4_b7f7_a99370df7ee9);
impl windows_core::RuntimeType for IAudioNodeEmitterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAudioNodeEmitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, AudioNodeEmitterSettings, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitterNaturalDecayModelProperties, IAudioNodeEmitterNaturalDecayModelProperties_Vtbl, 0x48934bcf_cf2c_4efc_9331_75bd22df1f0c);
impl windows_core::RuntimeType for IAudioNodeEmitterNaturalDecayModelProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnityGainDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CutoffDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitterShape, IAudioNodeEmitterShape_Vtbl, 0xea0311c5_e73d_44bc_859c_45553bbc4828);
impl windows_core::RuntimeType for IAudioNodeEmitterShape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitterShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioNodeEmitterShapeKind) -> windows_core::HRESULT,
    pub ConeProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeEmitterShapeStatics, IAudioNodeEmitterShapeStatics_Vtbl, 0x57bb2771_ffa5_4b86_a779_e264aeb9145f);
impl windows_core::RuntimeType for IAudioNodeEmitterShapeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeEmitterShapeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCone: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOmnidirectional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioNodeListener, IAudioNodeListener_Vtbl, 0xd9722e16_0c0a_41da_b755_6c77835fb1eb);
impl windows_core::RuntimeType for IAudioNodeListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub SpeedOfSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetSpeedOfSound: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
}
windows_core::imp::define_interface!(IAudioNodeWithListener, IAudioNodeWithListener_Vtbl, 0x0e0f907c_79ff_4544_9eeb_01257b15105a);
impl core::ops::Deref for IAudioNodeWithListener {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioNodeWithListener, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAudioNodeWithListener, IAudioNode, super::super::Foundation::IClosable);
impl IAudioNodeWithListener {
    pub fn SetListener<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AudioNodeListener>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetListener)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Listener(&self) -> windows_core::Result<AudioNodeListener> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Listener)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IAudioNodeWithListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioNodeWithListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Listener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioPlaybackConnection, IAudioPlaybackConnection_Vtbl, 0x1a4c1dea_cafc_50e7_8718_ea3f81cbfa51);
impl windows_core::RuntimeType for IAudioPlaybackConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioPlaybackConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioPlaybackConnectionState) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioPlaybackConnectionOpenResult, IAudioPlaybackConnectionOpenResult_Vtbl, 0x4e656aef_39f9_5fc9_a519_a5bbfd9fe921);
impl windows_core::RuntimeType for IAudioPlaybackConnectionOpenResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioPlaybackConnectionOpenResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioPlaybackConnectionOpenResultStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioPlaybackConnectionStatics, IAudioPlaybackConnectionStatics_Vtbl, 0xe60963a2_69e6_5ffc_9e13_824a85213daf);
impl windows_core::RuntimeType for IAudioPlaybackConnectionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioPlaybackConnectionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TryCreateFromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioStateMonitor, IAudioStateMonitor_Vtbl, 0x1d13d136_0199_4cdc_b84e_e72c2b581ece);
impl windows_core::RuntimeType for IAudioStateMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioStateMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SoundLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SoundLevel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioStateMonitorStatics, IAudioStateMonitorStatics_Vtbl, 0x6374ea4c_1b3b_4001_94d9_dd225330fa40);
impl windows_core::RuntimeType for IAudioStateMonitorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioStateMonitorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForRenderMonitoring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategory: unsafe extern "system" fn(*mut core::ffi::c_void, super::Render::AudioRenderCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub CreateForRenderMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(*mut core::ffi::c_void, super::Render::AudioRenderCategory, super::Devices::AudioDeviceRole, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices", feature = "Media_Render")))]
    CreateForRenderMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Render::AudioRenderCategory, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategoryAndDeviceId: usize,
    pub CreateForCaptureMonitoring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategory: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaCategory, super::Devices::AudioDeviceRole, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices")))]
    CreateForCaptureMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaCategory, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategoryAndDeviceId: usize,
}
windows_core::imp::define_interface!(ICreateAudioDeviceInputNodeResult, ICreateAudioDeviceInputNodeResult_Vtbl, 0x16eec7a8_1ca7_40ef_91a4_d346e0aa1bba);
impl windows_core::RuntimeType for ICreateAudioDeviceInputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioDeviceInputNodeResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioDeviceNodeCreationStatus) -> windows_core::HRESULT,
    pub DeviceInputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioDeviceInputNodeResult2, ICreateAudioDeviceInputNodeResult2_Vtbl, 0x921c69ce_3f35_41c7_9622_79f608baedc2);
impl windows_core::RuntimeType for ICreateAudioDeviceInputNodeResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioDeviceInputNodeResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioDeviceOutputNodeResult, ICreateAudioDeviceOutputNodeResult_Vtbl, 0xf7776d27_1d9a_47f7_9cd4_2859cc1b7bff);
impl windows_core::RuntimeType for ICreateAudioDeviceOutputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioDeviceOutputNodeResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioDeviceNodeCreationStatus) -> windows_core::HRESULT,
    pub DeviceOutputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioDeviceOutputNodeResult2, ICreateAudioDeviceOutputNodeResult2_Vtbl, 0x4864269f_bdce_4ab1_bd38_fbae93aedaca);
impl windows_core::RuntimeType for ICreateAudioDeviceOutputNodeResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioDeviceOutputNodeResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioFileInputNodeResult, ICreateAudioFileInputNodeResult_Vtbl, 0xce83d61c_e297_4c50_9ce7_1c7a69d6bd09);
impl windows_core::RuntimeType for ICreateAudioFileInputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioFileInputNodeResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioFileNodeCreationStatus) -> windows_core::HRESULT,
    pub FileInputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioFileInputNodeResult2, ICreateAudioFileInputNodeResult2_Vtbl, 0xf9082020_3d80_4fe0_81c1_768fea7ca7e0);
impl windows_core::RuntimeType for ICreateAudioFileInputNodeResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioFileInputNodeResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioFileOutputNodeResult, ICreateAudioFileOutputNodeResult_Vtbl, 0x47d6ba7b_e909_453f_866e_5540cda734ff);
impl windows_core::RuntimeType for ICreateAudioFileOutputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioFileOutputNodeResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioFileNodeCreationStatus) -> windows_core::HRESULT,
    pub FileOutputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioFileOutputNodeResult2, ICreateAudioFileOutputNodeResult2_Vtbl, 0x9f01b50d_3318_47b3_a60a_1b492be7fc0d);
impl windows_core::RuntimeType for ICreateAudioFileOutputNodeResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioFileOutputNodeResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioGraphResult, ICreateAudioGraphResult_Vtbl, 0x5453ef7e_7bde_4b76_bb5d_48f79cfc8c0b);
impl windows_core::RuntimeType for ICreateAudioGraphResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioGraphResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioGraphCreationStatus) -> windows_core::HRESULT,
    pub Graph: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateAudioGraphResult2, ICreateAudioGraphResult2_Vtbl, 0x6d738dfc_88c6_4fcb_a534_85cedd4050a1);
impl windows_core::RuntimeType for ICreateAudioGraphResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateAudioGraphResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateMediaSourceAudioInputNodeResult, ICreateMediaSourceAudioInputNodeResult_Vtbl, 0x46a658a3_53c0_4d59_9e51_cc1d1044a4c4);
impl windows_core::RuntimeType for ICreateMediaSourceAudioInputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateMediaSourceAudioInputNodeResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaSourceAudioInputNodeCreationStatus) -> windows_core::HRESULT,
    pub Node: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateMediaSourceAudioInputNodeResult2, ICreateMediaSourceAudioInputNodeResult2_Vtbl, 0x63514ce8_6a1a_49e3_97ec_28fd5be114e5);
impl windows_core::RuntimeType for ICreateMediaSourceAudioInputNodeResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICreateMediaSourceAudioInputNodeResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEchoEffectDefinition, IEchoEffectDefinition_Vtbl, 0x0e4d3faa_36b8_4c91_b9da_11f44a8a6610);
impl windows_core::RuntimeType for IEchoEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEchoEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetWetDryMix: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Feedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEchoEffectDefinitionFactory, IEchoEffectDefinitionFactory_Vtbl, 0x0d4e2257_aaf2_4e86_a54c_fb79db8f6c12);
impl windows_core::RuntimeType for IEchoEffectDefinitionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEchoEffectDefinitionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEqualizerBand, IEqualizerBand_Vtbl, 0xc00a5a6a_262d_4b85_9bb7_43280b62ed0c);
impl windows_core::RuntimeType for IEqualizerBand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEqualizerBand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub FrequencyCenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFrequencyCenter: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Gain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEqualizerEffectDefinition, IEqualizerEffectDefinition_Vtbl, 0x023f6f1f_83fe_449a_a822_c696442d16b0);
impl windows_core::RuntimeType for IEqualizerEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEqualizerEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Bands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bands: usize,
}
windows_core::imp::define_interface!(IEqualizerEffectDefinitionFactory, IEqualizerEffectDefinitionFactory_Vtbl, 0xd2876fc4_d410_4eb5_9e69_c9aa1277eaf0);
impl windows_core::RuntimeType for IEqualizerEffectDefinitionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEqualizerEffectDefinitionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameInputNodeQuantumStartedEventArgs, IFrameInputNodeQuantumStartedEventArgs_Vtbl, 0x3d9bd498_a306_4f06_bd9f_e9efc8226304);
impl windows_core::RuntimeType for IFrameInputNodeQuantumStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFrameInputNodeQuantumStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequiredSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILimiterEffectDefinition, ILimiterEffectDefinition_Vtbl, 0x6b755d19_2603_47ba_bdeb_39055e3486dc);
impl windows_core::RuntimeType for ILimiterEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILimiterEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetRelease: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLoudness: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Loudness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILimiterEffectDefinitionFactory, ILimiterEffectDefinitionFactory_Vtbl, 0xecbae6f1_61ff_45ef_b8f5_48659a57c72d);
impl windows_core::RuntimeType for ILimiterEffectDefinitionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILimiterEffectDefinitionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaSourceAudioInputNode, IMediaSourceAudioInputNode_Vtbl, 0x99d8983b_a88a_4041_8e4f_ddbac0c91fd3);
impl windows_core::RuntimeType for IMediaSourceAudioInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaSourceAudioInputNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoopCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLoopCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    pub MediaSourceCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMediaSourceCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReverbEffectDefinition, IReverbEffectDefinition_Vtbl, 0x4606aa89_f563_4d0a_8f6e_f0cddff35d84);
impl windows_core::RuntimeType for IReverbEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReverbEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetWetDryMix: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetReflectionsDelay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReflectionsDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReverbDelay: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub ReverbDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetRearDelay: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub RearDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetPositionLeft: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub PositionLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetPositionRight: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub PositionRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetPositionMatrixLeft: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub PositionMatrixLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetPositionMatrixRight: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub PositionMatrixRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetEarlyDiffusion: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub EarlyDiffusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetLateDiffusion: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub LateDiffusion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetLowEQGain: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub LowEQGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetLowEQCutoff: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub LowEQCutoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetHighEQGain: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub HighEQGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetHighEQCutoff: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub HighEQCutoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetRoomFilterFreq: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RoomFilterFreq: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRoomFilterMain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RoomFilterMain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRoomFilterHF: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RoomFilterHF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetReflectionsGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ReflectionsGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetReverbGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ReverbGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDecayTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DecayTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDensity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Density: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRoomSize: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RoomSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDisableLateField: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DisableLateField: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReverbEffectDefinitionFactory, IReverbEffectDefinitionFactory_Vtbl, 0xa7d5cbfe_100b_4ff0_9da6_dc4e05a759f0);
impl windows_core::RuntimeType for IReverbEffectDefinitionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReverbEffectDefinitionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISetDefaultSpatialAudioFormatResult, ISetDefaultSpatialAudioFormatResult_Vtbl, 0x1c2aa511_1400_5e70_9ea9_ae151241e8ea);
impl windows_core::RuntimeType for ISetDefaultSpatialAudioFormatResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISetDefaultSpatialAudioFormatResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SetDefaultSpatialAudioFormatStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioDeviceConfiguration, ISpatialAudioDeviceConfiguration_Vtbl, 0xee830034_61cf_5749_9da4_10f0fe028199);
impl windows_core::RuntimeType for ISpatialAudioDeviceConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAudioDeviceConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsSpatialAudioSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSpatialAudioFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub ActiveSpatialAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DefaultSpatialAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDefaultSpatialAudioFormatAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConfigurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioDeviceConfigurationStatics, ISpatialAudioDeviceConfigurationStatics_Vtbl, 0x3ec37f7b_936d_4e04_9728_2827d9f758c4);
impl windows_core::RuntimeType for ISpatialAudioDeviceConfigurationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAudioDeviceConfigurationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioFormatConfiguration, ISpatialAudioFormatConfiguration_Vtbl, 0x32df09a8_50f0_5395_9923_7d44ca71ed6d);
impl windows_core::RuntimeType for ISpatialAudioFormatConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAudioFormatConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportLicenseChangedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportConfigurationChangedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MixedRealityExclusiveModePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MixedRealitySpatialAudioFormatPolicy) -> windows_core::HRESULT,
    pub SetMixedRealityExclusiveModePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, MixedRealitySpatialAudioFormatPolicy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioFormatConfigurationStatics, ISpatialAudioFormatConfigurationStatics_Vtbl, 0x2b5fef71_67c9_4e5f_a35b_41680711f8c7);
impl windows_core::RuntimeType for ISpatialAudioFormatConfigurationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAudioFormatConfigurationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioFormatSubtypeStatics, ISpatialAudioFormatSubtypeStatics_Vtbl, 0xb3de8a47_83ee_4266_a945_bedf507afeed);
impl windows_core::RuntimeType for ISpatialAudioFormatSubtypeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAudioFormatSubtypeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowsSonic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DolbyAtmosForHeadphones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DolbyAtmosForHomeTheater: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DolbyAtmosForSpeakers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DTSHeadphoneX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DTSXUltra: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioFormatSubtypeStatics2, ISpatialAudioFormatSubtypeStatics2_Vtbl, 0x4565e6cb_d95b_5621_b6af_0e8849c57c80);
impl windows_core::RuntimeType for ISpatialAudioFormatSubtypeStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialAudioFormatSubtypeStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DTSXForHomeTheater: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioDeviceInputNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioDeviceInputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioDeviceInputNode, IAudioInputNode, IAudioInputNode2, IAudioNode, super::super::Foundation::IClosable);
impl AudioDeviceInputNode {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(windows_core::Interface::as_raw(this), destination.param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn Emitter(&self) -> windows_core::Result<AudioNodeEmitter> {
        let this = &windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emitter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioDeviceInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioDeviceInputNode>();
}
unsafe impl windows_core::Interface for AudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_Vtbl;
    const IID: windows_core::GUID = <IAudioDeviceInputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioDeviceInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceInputNode";
}
unsafe impl Send for AudioDeviceInputNode {}
unsafe impl Sync for AudioDeviceInputNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioDeviceOutputNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioDeviceOutputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioDeviceOutputNode, IAudioNode, IAudioNodeWithListener, super::super::Foundation::IClosable);
impl AudioDeviceOutputNode {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn SetListener<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AudioNodeListener>,
    {
        let this = &windows_core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetListener)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Listener(&self) -> windows_core::Result<AudioNodeListener> {
        let this = &windows_core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Listener)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioDeviceOutputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioDeviceOutputNode>();
}
unsafe impl windows_core::Interface for AudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_Vtbl;
    const IID: windows_core::GUID = <IAudioDeviceOutputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioDeviceOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceOutputNode";
}
unsafe impl Send for AudioDeviceOutputNode {}
unsafe impl Sync for AudioDeviceOutputNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioFileInputNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioFileInputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioFileInputNode, IAudioInputNode, IAudioInputNode2, IAudioNode, super::super::Foundation::IClosable);
impl AudioFileInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackSpeedFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackSpeedFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEndTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEndTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LoopCount(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoopCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLoopCount<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLoopCount)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FileCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioFileInputNode, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFileCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFileCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(windows_core::Interface::as_raw(this), destination.param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn Emitter(&self) -> windows_core::Result<AudioNodeEmitter> {
        let this = &windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emitter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioFileInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioFileInputNode>();
}
unsafe impl windows_core::Interface for AudioFileInputNode {
    type Vtable = IAudioFileInputNode_Vtbl;
    const IID: windows_core::GUID = <IAudioFileInputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioFileInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileInputNode";
}
unsafe impl Send for AudioFileInputNode {}
unsafe impl Sync for AudioFileInputNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioFileOutputNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioFileOutputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioFileOutputNode, IAudioNode, super::super::Foundation::IClosable);
impl AudioFileOutputNode {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> windows_core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FileEncodingProfile(&self) -> windows_core::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileEncodingProfile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Transcoding")]
    pub fn FinalizeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinalizeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioFileOutputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioFileOutputNode>();
}
unsafe impl windows_core::Interface for AudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_Vtbl;
    const IID: windows_core::GUID = <IAudioFileOutputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioFileOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileOutputNode";
}
unsafe impl Send for AudioFileOutputNode {}
unsafe impl Sync for AudioFileOutputNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioFrameCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioFrameCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AudioFrameCompletedEventArgs {
    pub fn Frame(&self) -> windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Frame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AudioFrameCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioFrameCompletedEventArgs>();
}
unsafe impl windows_core::Interface for AudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAudioFrameCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioFrameCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameCompletedEventArgs";
}
unsafe impl Send for AudioFrameCompletedEventArgs {}
unsafe impl Sync for AudioFrameCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioFrameInputNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioFrameInputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioFrameInputNode, IAudioInputNode, IAudioInputNode2, IAudioNode, super::super::Foundation::IClosable);
impl AudioFrameInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackSpeedFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackSpeedFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddFrame<P0>(&self, frame: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AudioFrame>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddFrame)(windows_core::Interface::as_raw(this), frame.param().abi()).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DiscardQueuedFrames)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn QueuedSampleCount(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueuedSampleCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AudioFrameCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioFrameCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioFrameCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioFrameCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn QuantumStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuantumStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveQuantumStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveQuantumStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(windows_core::Interface::as_raw(this), destination.param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn Emitter(&self) -> windows_core::Result<AudioNodeEmitter> {
        let this = &windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emitter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioFrameInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioFrameInputNode>();
}
unsafe impl windows_core::Interface for AudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_Vtbl;
    const IID: windows_core::GUID = <IAudioFrameInputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioFrameInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameInputNode";
}
unsafe impl Send for AudioFrameInputNode {}
unsafe impl Sync for AudioFrameInputNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioFrameOutputNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioFrameOutputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioFrameOutputNode, IAudioNode, super::super::Foundation::IClosable);
impl AudioFrameOutputNode {
    pub fn GetFrame(&self) -> windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioFrameOutputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioFrameOutputNode>();
}
unsafe impl windows_core::Interface for AudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_Vtbl;
    const IID: windows_core::GUID = <IAudioFrameOutputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioFrameOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameOutputNode";
}
unsafe impl Send for AudioFrameOutputNode {}
unsafe impl Sync for AudioFrameOutputNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioGraph(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioGraph, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioGraph, super::super::Foundation::IClosable);
impl AudioGraph {
    pub fn CreateFrameInputNode(&self) -> windows_core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameInputNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormat<P0>(&self, encodingproperties: P0) -> windows_core::Result<AudioFrameInputNode>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameInputNodeWithFormat)(windows_core::Interface::as_raw(this), encodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateDeviceInputNodeAsync(&self, category: super::Capture::MediaCategory) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDeviceInputNodeAsync)(windows_core::Interface::as_raw(this), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAsync<P0>(&self, category: super::Capture::MediaCategory, encodingproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAsync)(windows_core::Interface::as_raw(this), category, encodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatOnDeviceAsync<P0, P1>(&self, category: super::Capture::MediaCategory, encodingproperties: P0, device: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
        P1: windows_core::Param<super::super::Devices::Enumeration::DeviceInformation>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDeviceInputNodeWithFormatOnDeviceAsync)(windows_core::Interface::as_raw(this), category, encodingproperties.param().abi(), device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFrameOutputNode(&self) -> windows_core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameOutputNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameOutputNodeWithFormat<P0>(&self, encodingproperties: P0) -> windows_core::Result<AudioFrameOutputNode>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameOutputNodeWithFormat)(windows_core::Interface::as_raw(this), encodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDeviceOutputNodeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDeviceOutputNodeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CreateFileInputNodeAsync<P0>(&self, file: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileInputNodeAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CreateFileOutputNodeAsync<P0>(&self, file: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileOutputNodeAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CreateFileOutputNodeWithFileProfileAsync<P0, P1>(&self, file: P0, fileencodingprofile: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
        P1: windows_core::Param<super::MediaProperties::MediaEncodingProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileOutputNodeWithFileProfileAsync)(windows_core::Interface::as_raw(this), file.param().abi(), fileencodingprofile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateSubmixNode(&self) -> windows_core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSubmixNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormat<P0>(&self, encodingproperties: P0) -> windows_core::Result<AudioSubmixNode>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSubmixNodeWithFormat)(windows_core::Interface::as_raw(this), encodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn ResetAllNodes(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ResetAllNodes)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn QuantumStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioGraph, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuantumStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveQuantumStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveQuantumStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn QuantumProcessed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioGraph, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuantumProcessed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveQuantumProcessed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveQuantumProcessed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UnrecoverableErrorOccurred<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnrecoverableErrorOccurred)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUnrecoverableErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUnrecoverableErrorOccurred)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CompletedQuantumCount(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletedQuantumCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LatencyInSamples(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LatencyInSamples)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryRenderDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RenderDeviceAudioProcessing(&self) -> windows_core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderDeviceAudioProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SamplesPerQuantum(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SamplesPerQuantum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormatAndEmitter<P0, P1>(&self, encodingproperties: P0, emitter: P1) -> windows_core::Result<AudioFrameInputNode>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
        P1: windows_core::Param<AudioNodeEmitter>,
    {
        let this = &windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFrameInputNodeWithFormatAndEmitter)(windows_core::Interface::as_raw(this), encodingproperties.param().abi(), emitter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync<P0, P1, P2>(&self, category: super::Capture::MediaCategory, encodingproperties: P0, device: P1, emitter: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
        P1: windows_core::Param<super::super::Devices::Enumeration::DeviceInformation>,
        P2: windows_core::Param<AudioNodeEmitter>,
    {
        let this = &windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync)(windows_core::Interface::as_raw(this), category, encodingproperties.param().abi(), device.param().abi(), emitter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn CreateFileInputNodeWithEmitterAsync<P0, P1>(&self, file: P0, emitter: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
        P1: windows_core::Param<AudioNodeEmitter>,
    {
        let this = &windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileInputNodeWithEmitterAsync)(windows_core::Interface::as_raw(this), file.param().abi(), emitter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormatAndEmitter<P0, P1>(&self, encodingproperties: P0, emitter: P1) -> windows_core::Result<AudioSubmixNode>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
        P1: windows_core::Param<AudioNodeEmitter>,
    {
        let this = &windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSubmixNodeWithFormatAndEmitter)(windows_core::Interface::as_raw(this), encodingproperties.param().abi(), emitter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateBatchUpdater(&self) -> windows_core::Result<AudioGraphBatchUpdater> {
        let this = &windows_core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBatchUpdater)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn CreateMediaSourceAudioInputNodeAsync<P0>(&self, mediasource: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>
    where
        P0: windows_core::Param<super::Core::MediaSource>,
    {
        let this = &windows_core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMediaSourceAudioInputNodeAsync)(windows_core::Interface::as_raw(this), mediasource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn CreateMediaSourceAudioInputNodeWithEmitterAsync<P0, P1>(&self, mediasource: P0, emitter: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>
    where
        P0: windows_core::Param<super::Core::MediaSource>,
        P1: windows_core::Param<AudioNodeEmitter>,
    {
        let this = &windows_core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMediaSourceAudioInputNodeWithEmitterAsync)(windows_core::Interface::as_raw(this), mediasource.param().abi(), emitter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAsync<P0>(settings: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>
    where
        P0: windows_core::Param<AudioGraphSettings>,
    {
        Self::IAudioGraphStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAudioGraphStatics<R, F: FnOnce(&IAudioGraphStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioGraph, IAudioGraphStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioGraph {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioGraph>();
}
unsafe impl windows_core::Interface for AudioGraph {
    type Vtable = IAudioGraph_Vtbl;
    const IID: windows_core::GUID = <IAudioGraph as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioGraph {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraph";
}
unsafe impl Send for AudioGraph {}
unsafe impl Sync for AudioGraph {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioGraphBatchUpdater(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioGraphBatchUpdater, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioGraphBatchUpdater, super::super::Foundation::IClosable);
impl AudioGraphBatchUpdater {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioGraphBatchUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Foundation::IClosable>();
}
unsafe impl windows_core::Interface for AudioGraphBatchUpdater {
    type Vtable = super::super::Foundation::IClosable_Vtbl;
    const IID: windows_core::GUID = <super::super::Foundation::IClosable as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioGraphBatchUpdater {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphBatchUpdater";
}
unsafe impl Send for AudioGraphBatchUpdater {}
unsafe impl Sync for AudioGraphBatchUpdater {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioGraphConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioGraphConnection, windows_core::IUnknown, windows_core::IInspectable);
impl AudioGraphConnection {
    pub fn Destination(&self) -> windows_core::Result<IAudioNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Destination)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AudioGraphConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioGraphConnection>();
}
unsafe impl windows_core::Interface for AudioGraphConnection {
    type Vtable = IAudioGraphConnection_Vtbl;
    const IID: windows_core::GUID = <IAudioGraphConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioGraphConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphConnection";
}
unsafe impl Send for AudioGraphConnection {}
unsafe impl Sync for AudioGraphConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioGraphSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioGraphSettings, windows_core::IUnknown, windows_core::IInspectable);
impl AudioGraphSettings {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncodingProperties)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryRenderDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPrimaryRenderDevice<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Devices::Enumeration::DeviceInformation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrimaryRenderDevice)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn QuantumSizeSelectionMode(&self) -> windows_core::Result<QuantumSizeSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuantumSizeSelectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQuantumSizeSelectionMode(&self, value: QuantumSizeSelectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuantumSizeSelectionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredSamplesPerQuantum(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSamplesPerQuantum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredSamplesPerQuantum(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredSamplesPerQuantum)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Render")]
    pub fn AudioRenderCategory(&self) -> windows_core::Result<super::Render::AudioRenderCategory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioRenderCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Render")]
    pub fn SetAudioRenderCategory(&self, value: super::Render::AudioRenderCategory) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudioRenderCategory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredRenderDeviceAudioProcessing(&self) -> windows_core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredRenderDeviceAudioProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredRenderDeviceAudioProcessing(&self, value: super::AudioProcessing) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredRenderDeviceAudioProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxPlaybackSpeedFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxPlaybackSpeedFactor(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPlaybackSpeedFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Render")]
    pub fn Create(audiorendercategory: super::Render::AudioRenderCategory) -> windows_core::Result<AudioGraphSettings> {
        Self::IAudioGraphSettingsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), audiorendercategory, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioGraphSettingsFactory<R, F: FnOnce(&IAudioGraphSettingsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioGraphSettings, IAudioGraphSettingsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioGraphSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioGraphSettings>();
}
unsafe impl windows_core::Interface for AudioGraphSettings {
    type Vtable = IAudioGraphSettings_Vtbl;
    const IID: windows_core::GUID = <IAudioGraphSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioGraphSettings {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphSettings";
}
unsafe impl Send for AudioGraphSettings {}
unsafe impl Sync for AudioGraphSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioGraphUnrecoverableErrorOccurredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AudioGraphUnrecoverableErrorOccurredEventArgs {
    pub fn Error(&self) -> windows_core::Result<AudioGraphUnrecoverableError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioGraphUnrecoverableErrorOccurredEventArgs>();
}
unsafe impl windows_core::Interface for AudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAudioGraphUnrecoverableErrorOccurredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
}
unsafe impl Send for AudioGraphUnrecoverableErrorOccurredEventArgs {}
unsafe impl Sync for AudioGraphUnrecoverableErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioNodeEmitter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioNodeEmitter, windows_core::IUnknown, windows_core::IInspectable);
impl AudioNodeEmitter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioNodeEmitter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition(&self, value: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Direction(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDirection(&self, value: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDirection)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Shape(&self) -> windows_core::Result<AudioNodeEmitterShape> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DecayModel(&self) -> windows_core::Result<AudioNodeEmitterDecayModel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecayModel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DistanceScale(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistanceScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDistanceScale(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDistanceScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DopplerScale(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DopplerScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDopplerScale(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDopplerScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DopplerVelocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity(&self, value: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDopplerVelocity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDopplerDisabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDopplerDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SpatialAudioModel(&self) -> windows_core::Result<SpatialAudioModel> {
        let this = &windows_core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpatialAudioModel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSpatialAudioModel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAudioNodeEmitter<P0, P1>(shape: P0, decaymodel: P1, settings: AudioNodeEmitterSettings) -> windows_core::Result<AudioNodeEmitter>
    where
        P0: windows_core::Param<AudioNodeEmitterShape>,
        P1: windows_core::Param<AudioNodeEmitterDecayModel>,
    {
        Self::IAudioNodeEmitterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAudioNodeEmitter)(windows_core::Interface::as_raw(this), shape.param().abi(), decaymodel.param().abi(), settings, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterFactory<R, F: FnOnce(&IAudioNodeEmitterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioNodeEmitter, IAudioNodeEmitterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioNodeEmitter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioNodeEmitter>();
}
unsafe impl windows_core::Interface for AudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_Vtbl;
    const IID: windows_core::GUID = <IAudioNodeEmitter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioNodeEmitter {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitter";
}
unsafe impl Send for AudioNodeEmitter {}
unsafe impl Sync for AudioNodeEmitter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioNodeEmitterConeProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioNodeEmitterConeProperties, windows_core::IUnknown, windows_core::IInspectable);
impl AudioNodeEmitterConeProperties {
    pub fn InnerAngle(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OuterAngle(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OuterAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OuterAngleGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OuterAngleGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AudioNodeEmitterConeProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioNodeEmitterConeProperties>();
}
unsafe impl windows_core::Interface for AudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_Vtbl;
    const IID: windows_core::GUID = <IAudioNodeEmitterConeProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioNodeEmitterConeProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterConeProperties";
}
unsafe impl Send for AudioNodeEmitterConeProperties {}
unsafe impl Sync for AudioNodeEmitterConeProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioNodeEmitterDecayModel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioNodeEmitterDecayModel, windows_core::IUnknown, windows_core::IInspectable);
impl AudioNodeEmitterDecayModel {
    pub fn Kind(&self) -> windows_core::Result<AudioNodeEmitterDecayKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NaturalProperties(&self) -> windows_core::Result<AudioNodeEmitterNaturalDecayModelProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateNatural(mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64) -> windows_core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNatural)(windows_core::Interface::as_raw(this), mingain, maxgain, unitygaindistance, cutoffdistance, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCustom(mingain: f64, maxgain: f64) -> windows_core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCustom)(windows_core::Interface::as_raw(this), mingain, maxgain, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterDecayModelStatics<R, F: FnOnce(&IAudioNodeEmitterDecayModelStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioNodeEmitterDecayModel, IAudioNodeEmitterDecayModelStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioNodeEmitterDecayModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioNodeEmitterDecayModel>();
}
unsafe impl windows_core::Interface for AudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_Vtbl;
    const IID: windows_core::GUID = <IAudioNodeEmitterDecayModel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioNodeEmitterDecayModel {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterDecayModel";
}
unsafe impl Send for AudioNodeEmitterDecayModel {}
unsafe impl Sync for AudioNodeEmitterDecayModel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioNodeEmitterNaturalDecayModelProperties, windows_core::IUnknown, windows_core::IInspectable);
impl AudioNodeEmitterNaturalDecayModelProperties {
    pub fn UnityGainDistance(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnityGainDistance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CutoffDistance(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CutoffDistance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AudioNodeEmitterNaturalDecayModelProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioNodeEmitterNaturalDecayModelProperties>();
}
unsafe impl windows_core::Interface for AudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_Vtbl;
    const IID: windows_core::GUID = <IAudioNodeEmitterNaturalDecayModelProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioNodeEmitterNaturalDecayModelProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
}
unsafe impl Send for AudioNodeEmitterNaturalDecayModelProperties {}
unsafe impl Sync for AudioNodeEmitterNaturalDecayModelProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioNodeEmitterShape(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioNodeEmitterShape, windows_core::IUnknown, windows_core::IInspectable);
impl AudioNodeEmitterShape {
    pub fn Kind(&self) -> windows_core::Result<AudioNodeEmitterShapeKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConeProperties(&self) -> windows_core::Result<AudioNodeEmitterConeProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConeProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateCone(innerangle: f64, outerangle: f64, outeranglegain: f64) -> windows_core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCone)(windows_core::Interface::as_raw(this), innerangle, outerangle, outeranglegain, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateOmnidirectional() -> windows_core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOmnidirectional)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterShapeStatics<R, F: FnOnce(&IAudioNodeEmitterShapeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioNodeEmitterShape, IAudioNodeEmitterShapeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioNodeEmitterShape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioNodeEmitterShape>();
}
unsafe impl windows_core::Interface for AudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_Vtbl;
    const IID: windows_core::GUID = <IAudioNodeEmitterShape as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioNodeEmitterShape {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterShape";
}
unsafe impl Send for AudioNodeEmitterShape {}
unsafe impl Sync for AudioNodeEmitterShape {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioNodeListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioNodeListener, windows_core::IUnknown, windows_core::IInspectable);
impl AudioNodeListener {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioNodeListener, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition(&self, value: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpeedOfSound(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpeedOfSound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpeedOfSound(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpeedOfSound)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DopplerVelocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity(&self, value: super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDopplerVelocity)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AudioNodeListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioNodeListener>();
}
unsafe impl windows_core::Interface for AudioNodeListener {
    type Vtable = IAudioNodeListener_Vtbl;
    const IID: windows_core::GUID = <IAudioNodeListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioNodeListener {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeListener";
}
unsafe impl Send for AudioNodeListener {}
unsafe impl Sync for AudioNodeListener {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioPlaybackConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioPlaybackConnection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioPlaybackConnection, super::super::Foundation::IClosable);
impl AudioPlaybackConnection {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<AudioPlaybackConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Open(&self) -> windows_core::Result<AudioPlaybackConnectionOpenResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Open)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, windows_core::IInspectable>>,
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
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryCreateFromId(id: &windows_core::HSTRING) -> windows_core::Result<AudioPlaybackConnection> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAudioPlaybackConnectionStatics<R, F: FnOnce(&IAudioPlaybackConnectionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioPlaybackConnection, IAudioPlaybackConnectionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioPlaybackConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioPlaybackConnection>();
}
unsafe impl windows_core::Interface for AudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_Vtbl;
    const IID: windows_core::GUID = <IAudioPlaybackConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioPlaybackConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnection";
}
unsafe impl Send for AudioPlaybackConnection {}
unsafe impl Sync for AudioPlaybackConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioPlaybackConnectionOpenResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioPlaybackConnectionOpenResult, windows_core::IUnknown, windows_core::IInspectable);
impl AudioPlaybackConnectionOpenResult {
    pub fn Status(&self) -> windows_core::Result<AudioPlaybackConnectionOpenResultStatus> {
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
impl windows_core::RuntimeType for AudioPlaybackConnectionOpenResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioPlaybackConnectionOpenResult>();
}
unsafe impl windows_core::Interface for AudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_Vtbl;
    const IID: windows_core::GUID = <IAudioPlaybackConnectionOpenResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioPlaybackConnectionOpenResult {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
}
unsafe impl Send for AudioPlaybackConnectionOpenResult {}
unsafe impl Sync for AudioPlaybackConnectionOpenResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioStateMonitor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioStateMonitor, windows_core::IUnknown, windows_core::IInspectable);
impl AudioStateMonitor {
    pub fn SoundLevelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioStateMonitor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoundLevelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSoundLevelChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SoundLevel(&self) -> windows_core::Result<super::SoundLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoundLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateForRenderMonitoring() -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForRenderMonitoring)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategory(category: super::Render::AudioRenderCategory) -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForRenderMonitoringWithCategory)(windows_core::Interface::as_raw(this), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceRole(category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole) -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceRole)(windows_core::Interface::as_raw(this), category, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceId(category: super::Render::AudioRenderCategory, deviceid: &windows_core::HSTRING) -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceId)(windows_core::Interface::as_raw(this), category, core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateForCaptureMonitoring() -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForCaptureMonitoring)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategory(category: super::Capture::MediaCategory) -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForCaptureMonitoringWithCategory)(windows_core::Interface::as_raw(this), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole) -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceRole)(windows_core::Interface::as_raw(this), category, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceId(category: super::Capture::MediaCategory, deviceid: &windows_core::HSTRING) -> windows_core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceId)(windows_core::Interface::as_raw(this), category, core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioStateMonitorStatics<R, F: FnOnce(&IAudioStateMonitorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioStateMonitor, IAudioStateMonitorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioStateMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioStateMonitor>();
}
unsafe impl windows_core::Interface for AudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
    const IID: windows_core::GUID = <IAudioStateMonitor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioStateMonitor {
    const NAME: &'static str = "Windows.Media.Audio.AudioStateMonitor";
}
unsafe impl Send for AudioStateMonitor {}
unsafe impl Sync for AudioStateMonitor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AudioSubmixNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioSubmixNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioSubmixNode, IAudioInputNode, IAudioInputNode2, IAudioNode, super::super::Foundation::IClosable);
impl AudioSubmixNode {
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(windows_core::Interface::as_raw(this), destination.param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn Emitter(&self) -> windows_core::Result<AudioNodeEmitter> {
        let this = &windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emitter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioSubmixNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioInputNode>();
}
unsafe impl windows_core::Interface for AudioSubmixNode {
    type Vtable = IAudioInputNode_Vtbl;
    const IID: windows_core::GUID = <IAudioInputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioSubmixNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioSubmixNode";
}
unsafe impl Send for AudioSubmixNode {}
unsafe impl Sync for AudioSubmixNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CreateAudioDeviceInputNodeResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateAudioDeviceInputNodeResult, windows_core::IUnknown, windows_core::IInspectable);
impl CreateAudioDeviceInputNodeResult {
    pub fn Status(&self) -> windows_core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceInputNode(&self) -> windows_core::Result<AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInputNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<ICreateAudioDeviceInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CreateAudioDeviceInputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateAudioDeviceInputNodeResult>();
}
unsafe impl windows_core::Interface for CreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_Vtbl;
    const IID: windows_core::GUID = <ICreateAudioDeviceInputNodeResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateAudioDeviceInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
}
unsafe impl Send for CreateAudioDeviceInputNodeResult {}
unsafe impl Sync for CreateAudioDeviceInputNodeResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CreateAudioDeviceOutputNodeResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateAudioDeviceOutputNodeResult, windows_core::IUnknown, windows_core::IInspectable);
impl CreateAudioDeviceOutputNodeResult {
    pub fn Status(&self) -> windows_core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceOutputNode(&self) -> windows_core::Result<AudioDeviceOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceOutputNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<ICreateAudioDeviceOutputNodeResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CreateAudioDeviceOutputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateAudioDeviceOutputNodeResult>();
}
unsafe impl windows_core::Interface for CreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_Vtbl;
    const IID: windows_core::GUID = <ICreateAudioDeviceOutputNodeResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateAudioDeviceOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
}
unsafe impl Send for CreateAudioDeviceOutputNodeResult {}
unsafe impl Sync for CreateAudioDeviceOutputNodeResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CreateAudioFileInputNodeResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateAudioFileInputNodeResult, windows_core::IUnknown, windows_core::IInspectable);
impl CreateAudioFileInputNodeResult {
    pub fn Status(&self) -> windows_core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FileInputNode(&self) -> windows_core::Result<AudioFileInputNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileInputNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<ICreateAudioFileInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CreateAudioFileInputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateAudioFileInputNodeResult>();
}
unsafe impl windows_core::Interface for CreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_Vtbl;
    const IID: windows_core::GUID = <ICreateAudioFileInputNodeResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateAudioFileInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileInputNodeResult";
}
unsafe impl Send for CreateAudioFileInputNodeResult {}
unsafe impl Sync for CreateAudioFileInputNodeResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CreateAudioFileOutputNodeResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateAudioFileOutputNodeResult, windows_core::IUnknown, windows_core::IInspectable);
impl CreateAudioFileOutputNodeResult {
    pub fn Status(&self) -> windows_core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FileOutputNode(&self) -> windows_core::Result<AudioFileOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileOutputNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<ICreateAudioFileOutputNodeResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CreateAudioFileOutputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateAudioFileOutputNodeResult>();
}
unsafe impl windows_core::Interface for CreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_Vtbl;
    const IID: windows_core::GUID = <ICreateAudioFileOutputNodeResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateAudioFileOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileOutputNodeResult";
}
unsafe impl Send for CreateAudioFileOutputNodeResult {}
unsafe impl Sync for CreateAudioFileOutputNodeResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CreateAudioGraphResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateAudioGraphResult, windows_core::IUnknown, windows_core::IInspectable);
impl CreateAudioGraphResult {
    pub fn Status(&self) -> windows_core::Result<AudioGraphCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Graph(&self) -> windows_core::Result<AudioGraph> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Graph)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<ICreateAudioGraphResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CreateAudioGraphResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateAudioGraphResult>();
}
unsafe impl windows_core::Interface for CreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_Vtbl;
    const IID: windows_core::GUID = <ICreateAudioGraphResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateAudioGraphResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioGraphResult";
}
unsafe impl Send for CreateAudioGraphResult {}
unsafe impl Sync for CreateAudioGraphResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CreateMediaSourceAudioInputNodeResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateMediaSourceAudioInputNodeResult, windows_core::IUnknown, windows_core::IInspectable);
impl CreateMediaSourceAudioInputNodeResult {
    pub fn Status(&self) -> windows_core::Result<MediaSourceAudioInputNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Node(&self) -> windows_core::Result<MediaSourceAudioInputNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Node)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<ICreateMediaSourceAudioInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CreateMediaSourceAudioInputNodeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateMediaSourceAudioInputNodeResult>();
}
unsafe impl windows_core::Interface for CreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_Vtbl;
    const IID: windows_core::GUID = <ICreateMediaSourceAudioInputNodeResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateMediaSourceAudioInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
}
unsafe impl Send for CreateMediaSourceAudioInputNodeResult {}
unsafe impl Sync for CreateMediaSourceAudioInputNodeResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct EchoEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EchoEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
windows_core::imp::required_hierarchy!(EchoEffectDefinition, super::Effects::IAudioEffectDefinition);
impl EchoEffectDefinition {
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWetDryMix(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWetDryMix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WetDryMix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFeedback(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFeedback)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Feedback(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Feedback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDelay(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Delay(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create<P0>(audiograph: P0) -> windows_core::Result<EchoEffectDefinition>
    where
        P0: windows_core::Param<AudioGraph>,
    {
        Self::IEchoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), audiograph.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IEchoEffectDefinitionFactory<R, F: FnOnce(&IEchoEffectDefinitionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<EchoEffectDefinition, IEchoEffectDefinitionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for EchoEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEchoEffectDefinition>();
}
unsafe impl windows_core::Interface for EchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <IEchoEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EchoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EchoEffectDefinition";
}
unsafe impl Send for EchoEffectDefinition {}
unsafe impl Sync for EchoEffectDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct EqualizerBand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EqualizerBand, windows_core::IUnknown, windows_core::IInspectable);
impl EqualizerBand {
    pub fn Bandwidth(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bandwidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBandwidth(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBandwidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FrequencyCenter(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrequencyCenter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFrequencyCenter(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFrequencyCenter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for EqualizerBand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEqualizerBand>();
}
unsafe impl windows_core::Interface for EqualizerBand {
    type Vtable = IEqualizerBand_Vtbl;
    const IID: windows_core::GUID = <IEqualizerBand as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EqualizerBand {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerBand";
}
unsafe impl Send for EqualizerBand {}
unsafe impl Sync for EqualizerBand {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct EqualizerEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EqualizerEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
windows_core::imp::required_hierarchy!(EqualizerEffectDefinition, super::Effects::IAudioEffectDefinition);
impl EqualizerEffectDefinition {
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bands(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<EqualizerBand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(audiograph: P0) -> windows_core::Result<EqualizerEffectDefinition>
    where
        P0: windows_core::Param<AudioGraph>,
    {
        Self::IEqualizerEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), audiograph.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IEqualizerEffectDefinitionFactory<R, F: FnOnce(&IEqualizerEffectDefinitionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<EqualizerEffectDefinition, IEqualizerEffectDefinitionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for EqualizerEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEqualizerEffectDefinition>();
}
unsafe impl windows_core::Interface for EqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <IEqualizerEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EqualizerEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerEffectDefinition";
}
unsafe impl Send for EqualizerEffectDefinition {}
unsafe impl Sync for EqualizerEffectDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FrameInputNodeQuantumStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FrameInputNodeQuantumStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FrameInputNodeQuantumStartedEventArgs {
    pub fn RequiredSamples(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiredSamples)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FrameInputNodeQuantumStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameInputNodeQuantumStartedEventArgs>();
}
unsafe impl windows_core::Interface for FrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IFrameInputNodeQuantumStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FrameInputNodeQuantumStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
}
unsafe impl Send for FrameInputNodeQuantumStartedEventArgs {}
unsafe impl Sync for FrameInputNodeQuantumStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LimiterEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LimiterEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
windows_core::imp::required_hierarchy!(LimiterEffectDefinition, super::Effects::IAudioEffectDefinition);
impl LimiterEffectDefinition {
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRelease(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelease)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Release(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Release)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLoudness(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLoudness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Loudness(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Loudness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create<P0>(audiograph: P0) -> windows_core::Result<LimiterEffectDefinition>
    where
        P0: windows_core::Param<AudioGraph>,
    {
        Self::ILimiterEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), audiograph.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILimiterEffectDefinitionFactory<R, F: FnOnce(&ILimiterEffectDefinitionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LimiterEffectDefinition, ILimiterEffectDefinitionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LimiterEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILimiterEffectDefinition>();
}
unsafe impl windows_core::Interface for LimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <ILimiterEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LimiterEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.LimiterEffectDefinition";
}
unsafe impl Send for LimiterEffectDefinition {}
unsafe impl Sync for LimiterEffectDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaSourceAudioInputNode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaSourceAudioInputNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaSourceAudioInputNode, IAudioInputNode, IAudioInputNode2, IAudioNode, super::super::Foundation::IClosable);
impl MediaSourceAudioInputNode {
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingConnections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddOutgoingConnectionWithGain)(windows_core::Interface::as_raw(this), destination.param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioNode>,
    {
        let this = &windows_core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveOutgoingConnection)(windows_core::Interface::as_raw(this), destination.param().abi()).ok() }
    }
    pub fn Emitter(&self) -> windows_core::Result<AudioNodeEmitter> {
        let this = &windows_core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emitter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutgoingGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutgoingGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConsumeInput(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConsumeInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetConsumeInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DisableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Effects::IAudioEffectDefinition>,
    {
        let this = &windows_core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableEffectsByDefinition)(windows_core::Interface::as_raw(this), definition.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaybackSpeedFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackSpeedFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEndTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEndTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LoopCount(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoopCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLoopCount<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLoopCount)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
    pub fn MediaSourceCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaSourceCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMediaSourceCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMediaSourceCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for MediaSourceAudioInputNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaSourceAudioInputNode>();
}
unsafe impl windows_core::Interface for MediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_Vtbl;
    const IID: windows_core::GUID = <IMediaSourceAudioInputNode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaSourceAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.MediaSourceAudioInputNode";
}
unsafe impl Send for MediaSourceAudioInputNode {}
unsafe impl Sync for MediaSourceAudioInputNode {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ReverbEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ReverbEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
windows_core::imp::required_hierarchy!(ReverbEffectDefinition, super::Effects::IAudioEffectDefinition);
impl ReverbEffectDefinition {
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWetDryMix(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWetDryMix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WetDryMix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReflectionsDelay(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReflectionsDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectionsDelay(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReflectionsDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReverbDelay(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReverbDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverbDelay(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReverbDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRearDelay(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRearDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RearDelay(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RearDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionLeft(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionLeft)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionLeft(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionLeft)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionRight(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionRight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionRight(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionRight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionMatrixLeft(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionMatrixLeft)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionMatrixLeft(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionMatrixLeft)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionMatrixRight(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionMatrixRight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionMatrixRight(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionMatrixRight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEarlyDiffusion(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEarlyDiffusion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EarlyDiffusion(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EarlyDiffusion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLateDiffusion(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLateDiffusion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LateDiffusion(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LateDiffusion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLowEQGain(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLowEQGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowEQGain(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LowEQGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLowEQCutoff(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLowEQCutoff)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowEQCutoff(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LowEQCutoff)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHighEQGain(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHighEQGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HighEQGain(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighEQGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHighEQCutoff(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHighEQCutoff)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HighEQCutoff(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighEQCutoff)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoomFilterFreq(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoomFilterFreq)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterFreq(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoomFilterFreq)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoomFilterMain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoomFilterMain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterMain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoomFilterMain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoomFilterHF(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoomFilterHF)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterHF(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoomFilterHF)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReflectionsGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReflectionsGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectionsGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReflectionsGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReverbGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReverbGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverbGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReverbGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDecayTime(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDecayTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecayTime(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecayTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDensity(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDensity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Density(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Density)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoomSize(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoomSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomSize(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoomSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisableLateField(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisableLateField)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisableLateField(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableLateField)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create<P0>(audiograph: P0) -> windows_core::Result<ReverbEffectDefinition>
    where
        P0: windows_core::Param<AudioGraph>,
    {
        Self::IReverbEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), audiograph.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IReverbEffectDefinitionFactory<R, F: FnOnce(&IReverbEffectDefinitionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ReverbEffectDefinition, IReverbEffectDefinitionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ReverbEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IReverbEffectDefinition>();
}
unsafe impl windows_core::Interface for ReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <IReverbEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ReverbEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.ReverbEffectDefinition";
}
unsafe impl Send for ReverbEffectDefinition {}
unsafe impl Sync for ReverbEffectDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SetDefaultSpatialAudioFormatResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SetDefaultSpatialAudioFormatResult, windows_core::IUnknown, windows_core::IInspectable);
impl SetDefaultSpatialAudioFormatResult {
    pub fn Status(&self) -> windows_core::Result<SetDefaultSpatialAudioFormatStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SetDefaultSpatialAudioFormatResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISetDefaultSpatialAudioFormatResult>();
}
unsafe impl windows_core::Interface for SetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_Vtbl;
    const IID: windows_core::GUID = <ISetDefaultSpatialAudioFormatResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SetDefaultSpatialAudioFormatResult {
    const NAME: &'static str = "Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
}
unsafe impl Send for SetDefaultSpatialAudioFormatResult {}
unsafe impl Sync for SetDefaultSpatialAudioFormatResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialAudioDeviceConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialAudioDeviceConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialAudioDeviceConfiguration {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSpatialAudioSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSpatialAudioSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSpatialAudioFormatSupported(&self, subtype: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSpatialAudioFormatSupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(subtype), &mut result__).map(|| result__)
        }
    }
    pub fn ActiveSpatialAudioFormat(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActiveSpatialAudioFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultSpatialAudioFormat(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultSpatialAudioFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultSpatialAudioFormatAsync(&self, subtype: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetDefaultSpatialAudioFormatAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(subtype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfigurationChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConfigurationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConfigurationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConfigurationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForDeviceId(deviceid: &windows_core::HSTRING) -> windows_core::Result<SpatialAudioDeviceConfiguration> {
        Self::ISpatialAudioDeviceConfigurationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForDeviceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioDeviceConfigurationStatics<R, F: FnOnce(&ISpatialAudioDeviceConfigurationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAudioDeviceConfiguration, ISpatialAudioDeviceConfigurationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialAudioDeviceConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialAudioDeviceConfiguration>();
}
unsafe impl windows_core::Interface for SpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_Vtbl;
    const IID: windows_core::GUID = <ISpatialAudioDeviceConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialAudioDeviceConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioDeviceConfiguration";
}
unsafe impl Send for SpatialAudioDeviceConfiguration {}
unsafe impl Sync for SpatialAudioDeviceConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialAudioFormatConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialAudioFormatConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialAudioFormatConfiguration {
    pub fn ReportLicenseChangedAsync(&self, subtype: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLicenseChangedAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(subtype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportConfigurationChangedAsync(&self, subtype: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportConfigurationChangedAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(subtype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MixedRealityExclusiveModePolicy(&self) -> windows_core::Result<MixedRealitySpatialAudioFormatPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MixedRealityExclusiveModePolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMixedRealityExclusiveModePolicy(&self, value: MixedRealitySpatialAudioFormatPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMixedRealityExclusiveModePolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<SpatialAudioFormatConfiguration> {
        Self::ISpatialAudioFormatConfigurationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatConfigurationStatics<R, F: FnOnce(&ISpatialAudioFormatConfigurationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAudioFormatConfiguration, ISpatialAudioFormatConfigurationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialAudioFormatConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialAudioFormatConfiguration>();
}
unsafe impl windows_core::Interface for SpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_Vtbl;
    const IID: windows_core::GUID = <ISpatialAudioFormatConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialAudioFormatConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatConfiguration";
}
unsafe impl Send for SpatialAudioFormatConfiguration {}
unsafe impl Sync for SpatialAudioFormatConfiguration {}
pub struct SpatialAudioFormatSubtype;
impl SpatialAudioFormatSubtype {
    pub fn WindowsSonic() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowsSonic)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DolbyAtmosForHeadphones() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DolbyAtmosForHeadphones)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DolbyAtmosForHomeTheater() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DolbyAtmosForHomeTheater)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DolbyAtmosForSpeakers() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DolbyAtmosForSpeakers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DTSHeadphoneX() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DTSHeadphoneX)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DTSXUltra() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DTSXUltra)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DTSXForHomeTheater() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DTSXForHomeTheater)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatSubtypeStatics<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatSubtypeStatics2<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for SpatialAudioFormatSubtype {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatSubtype";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl windows_core::TypeKind for AudioDeviceNodeCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioDeviceNodeCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioDeviceNodeCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioDeviceNodeCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioDeviceNodeCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FileNotFound: Self = Self(1i32);
    pub const InvalidFileType: Self = Self(2i32);
    pub const FormatNotSupported: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl windows_core::TypeKind for AudioFileNodeCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioFileNodeCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioFileNodeCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioFileNodeCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioFileNodeCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for AudioGraphCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioGraphCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioGraphCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioGraphCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: Self = Self(0i32);
    pub const AudioDeviceLost: Self = Self(1i32);
    pub const AudioSessionDisconnected: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for AudioGraphUnrecoverableError {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioGraphUnrecoverableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioGraphUnrecoverableError").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioGraphUnrecoverableError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphUnrecoverableError;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl windows_core::TypeKind for AudioNodeEmitterDecayKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioNodeEmitterDecayKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterDecayKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioNodeEmitterDecayKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterDecayKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: Self = Self(0u32);
    pub const DisableDoppler: Self = Self(1u32);
}
impl windows_core::TypeKind for AudioNodeEmitterSettings {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioNodeEmitterSettings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterSettings").field(&self.0).finish()
    }
}
impl AudioNodeEmitterSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AudioNodeEmitterSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AudioNodeEmitterSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AudioNodeEmitterSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for AudioNodeEmitterSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterSettings;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: Self = Self(0i32);
    pub const Cone: Self = Self(1i32);
}
impl windows_core::TypeKind for AudioNodeEmitterShapeKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioNodeEmitterShapeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterShapeKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioNodeEmitterShapeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterShapeKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: Self = Self(0i32);
    pub const RequestTimedOut: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for AudioPlaybackConnectionOpenResultStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioPlaybackConnectionOpenResultStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionOpenResultStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioPlaybackConnectionOpenResultStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
}
impl windows_core::TypeKind for AudioPlaybackConnectionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioPlaybackConnectionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioPlaybackConnectionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for MediaSourceAudioInputNodeCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaSourceAudioInputNodeCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaSourceAudioInputNodeCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaSourceAudioInputNodeCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: Self = Self(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: Self = Self(1i32);
}
impl windows_core::TypeKind for MixedRealitySpatialAudioFormatPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MixedRealitySpatialAudioFormatPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MixedRealitySpatialAudioFormatPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MixedRealitySpatialAudioFormatPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: Self = Self(0i32);
    pub const LowestLatency: Self = Self(1i32);
    pub const ClosestToDesired: Self = Self(2i32);
}
impl windows_core::TypeKind for QuantumSizeSelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QuantumSizeSelectionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QuantumSizeSelectionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for QuantumSizeSelectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.QuantumSizeSelectionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const LicenseExpired: Self = Self(2i32);
    pub const LicenseNotValidForAudioEndpoint: Self = Self(3i32);
    pub const NotSupportedOnAudioEndpoint: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl windows_core::TypeKind for SetDefaultSpatialAudioFormatStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SetDefaultSpatialAudioFormatStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SetDefaultSpatialAudioFormatStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SetDefaultSpatialAudioFormatStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: Self = Self(0i32);
    pub const FoldDown: Self = Self(1i32);
}
impl windows_core::TypeKind for SpatialAudioModel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialAudioModel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialAudioModel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialAudioModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SpatialAudioModel;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
