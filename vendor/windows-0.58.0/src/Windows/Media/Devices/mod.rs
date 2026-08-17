#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
windows_core::imp::define_interface!(IAdvancedPhotoCaptureSettings, IAdvancedPhotoCaptureSettings_Vtbl, 0x08f3863a_0018_445b_93d2_646d1c5ed05c);
impl windows_core::RuntimeType for IAdvancedPhotoCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedPhotoCaptureSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdvancedPhotoMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, AdvancedPhotoMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedPhotoControl, IAdvancedPhotoControl_Vtbl, 0xc5b15486_9001_4682_9309_68eae0080eec);
impl windows_core::RuntimeType for IAdvancedPhotoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedPhotoControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdvancedPhotoMode) -> windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController, IAdvancedVideoCaptureDeviceController_Vtbl, 0xde6ff4d3_2b96_4583_80ab_b5b01dc6a8d7);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDeviceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController10, IAdvancedVideoCaptureDeviceController10_Vtbl, 0xc621b82d_d6f0_5c1b_a388_a6e938407146);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController10 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController10_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CameraOcclusionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController11, IAdvancedVideoCaptureDeviceController11_Vtbl, 0xd5b65ae2_3772_580c_a630_e75de9106904);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController11 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController11_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture")]
    pub TryAcquireExclusiveControl: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Capture::MediaCaptureDeviceExclusiveControlReleaseMode, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TryAcquireExclusiveControl: usize,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController2, IAdvancedVideoCaptureDeviceController2_Vtbl, 0x8bb94f8f_f11a_43db_b402_11930b80ae56);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LowLagPhotoSequence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LowLagPhoto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SceneModeControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TorchControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlashControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WhiteBalanceControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExposureControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegionsOfInterestControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrimaryUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CaptureUse) -> windows_core::HRESULT,
    pub SetPrimaryUse: unsafe extern "system" fn(*mut core::ffi::c_void, CaptureUse) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController3, IAdvancedVideoCaptureDeviceController3_Vtbl, 0xa98b8f34_ee0d_470c_b9f0_4229c4bbd089);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices_Core")]
    pub VariablePhotoSequenceController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    VariablePhotoSequenceController: usize,
    pub PhotoConfirmationControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ZoomControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController4, IAdvancedVideoCaptureDeviceController4_Vtbl, 0xea9fbfaf_d371_41c3_9a17_824a87ebdfd2);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExposurePriorityVideoControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DesiredOptimization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCaptureOptimization) -> windows_core::HRESULT,
    pub SetDesiredOptimization: unsafe extern "system" fn(*mut core::ffi::c_void, MediaCaptureOptimization) -> windows_core::HRESULT,
    pub HdrVideoControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpticalImageStabilizationControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdvancedPhotoControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController5, IAdvancedVideoCaptureDeviceController5_Vtbl, 0x33512b17_b9cb_4a23_b875_f9eaab535492);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDevicePropertyById: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDevicePropertyById: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut VideoDeviceControllerSetDevicePropertyStatus) -> windows_core::HRESULT,
    pub GetDevicePropertyByExtendedId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDevicePropertyByExtendedId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut VideoDeviceControllerSetDevicePropertyStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController6, IAdvancedVideoCaptureDeviceController6_Vtbl, 0xb6563a53_68a1_44b7_9f89_b5fa97ac0cbe);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoTemporalDenoisingControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController7, IAdvancedVideoCaptureDeviceController7_Vtbl, 0x8d2927f0_a054_50e7_b7df_7c04234d10f0);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InfraredTorchControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController8, IAdvancedVideoCaptureDeviceController8_Vtbl, 0xd843f010_e7fb_595b_9a78_0e54c4532b43);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController8 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController8_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PanelBasedOptimizationControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdvancedVideoCaptureDeviceController9, IAdvancedVideoCaptureDeviceController9_Vtbl, 0x8bdca95d_0255_51bc_a10d_5a169ec1625a);
impl windows_core::RuntimeType for IAdvancedVideoCaptureDeviceController9 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedVideoCaptureDeviceController9_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DigitalWindowControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioDeviceController, IAudioDeviceController_Vtbl, 0xedd4a388_79c7_4f7c_90e8_ef934b21580a);
impl windows_core::RuntimeType for IAudioDeviceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetMuted: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Muted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetVolumePercent: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub VolumePercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioDeviceController2, IAudioDeviceController2_Vtbl, 0x85326599_4c24_48b0_81dd_0c5cc79ddf05);
impl windows_core::RuntimeType for IAudioDeviceController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Effects")]
    pub AudioCaptureEffectsManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    AudioCaptureEffectsManager: usize,
}
windows_core::imp::define_interface!(IAudioDeviceModule, IAudioDeviceModule_Vtbl, 0x86cfac36_47c1_4b33_9852_8773ec4be123);
impl windows_core::RuntimeType for IAudioDeviceModule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceModule_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub InstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SendCommandAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendCommandAsync: usize,
}
windows_core::imp::define_interface!(IAudioDeviceModuleNotificationEventArgs, IAudioDeviceModuleNotificationEventArgs_Vtbl, 0xe3e3ccaf_224c_48be_956b_9a13134e96e8);
impl windows_core::RuntimeType for IAudioDeviceModuleNotificationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceModuleNotificationEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Module: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub NotificationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    NotificationData: usize,
}
windows_core::imp::define_interface!(IAudioDeviceModulesManager, IAudioDeviceModulesManager_Vtbl, 0x6aa40c4d_960a_4d1c_b318_0022604547ed);
impl windows_core::RuntimeType for IAudioDeviceModulesManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceModulesManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ModuleNotificationReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveModuleNotificationReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllById: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllById: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
}
windows_core::imp::define_interface!(IAudioDeviceModulesManagerFactory, IAudioDeviceModulesManagerFactory_Vtbl, 0x8db03670_e64d_4773_96c0_bc7ebf0e063f);
impl windows_core::RuntimeType for IAudioDeviceModulesManagerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioDeviceModulesManagerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallControl, ICallControl_Vtbl, 0xa520d0d6_ae8d_45db_8011_ca49d3b3e578);
impl windows_core::RuntimeType for ICallControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICallControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IndicateNewIncomingCall: unsafe extern "system" fn(*mut core::ffi::c_void, bool, core::mem::MaybeUninit<windows_core::HSTRING>, *mut u64) -> windows_core::HRESULT,
    pub IndicateNewOutgoingCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IndicateActiveCall: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub EndCall: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub HasRinger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AnswerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAnswerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub HangUpRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHangUpRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DialRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDialRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RedialRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRedialRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub KeypadPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveKeypadPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AudioTransferRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioTransferRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallControlStatics, ICallControlStatics_Vtbl, 0x03945ad5_85ab_40e1_af19_56c94303b019);
impl windows_core::RuntimeType for ICallControlStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICallControlStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraOcclusionInfo, ICameraOcclusionInfo_Vtbl, 0xaf6c4ad0_a84d_5db6_be58_a5da21cfe011);
impl windows_core::RuntimeType for ICameraOcclusionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraOcclusionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOcclusionKindSupported: unsafe extern "system" fn(*mut core::ffi::c_void, CameraOcclusionKind, *mut bool) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraOcclusionState, ICameraOcclusionState_Vtbl, 0x430adeb8_6842_5e55_9bde_04b4ef3a8a57);
impl windows_core::RuntimeType for ICameraOcclusionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraOcclusionState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOccluded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsOcclusionKind: unsafe extern "system" fn(*mut core::ffi::c_void, CameraOcclusionKind, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICameraOcclusionStateChangedEventArgs, ICameraOcclusionStateChangedEventArgs_Vtbl, 0x8512d848_c0de_57ca_a1ca_fb2c3d23df55);
impl windows_core::RuntimeType for ICameraOcclusionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraOcclusionStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDefaultAudioDeviceChangedEventArgs, IDefaultAudioDeviceChangedEventArgs_Vtbl, 0x110f882f_1c05_4657_a18e_47c9b69f07ab);
impl core::ops::Deref for IDefaultAudioDeviceChangedEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDefaultAudioDeviceChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IDefaultAudioDeviceChangedEventArgs {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Role(&self) -> windows_core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Role)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IDefaultAudioDeviceChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDefaultAudioDeviceChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioDeviceRole) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialRequestedEventArgs, IDialRequestedEventArgs_Vtbl, 0x037b929e_953c_4286_8866_4f0f376c855a);
impl windows_core::RuntimeType for IDialRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDigitalWindowBounds, IDigitalWindowBounds_Vtbl, 0xdd4f21dd_d173_5c6b_8c25_bdd26d5122b1);
impl windows_core::RuntimeType for IDigitalWindowBounds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDigitalWindowBounds_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NormalizedOriginTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetNormalizedOriginTop: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub NormalizedOriginLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetNormalizedOriginLeft: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetScale: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDigitalWindowCapability, IDigitalWindowCapability_Vtbl, 0xd78bad2c_f721_5244_a196_b56ccbec606c);
impl windows_core::RuntimeType for IDigitalWindowCapability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDigitalWindowCapability_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinScaleValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MaxScaleValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MinScaleValueWithoutUpsampling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub NormalizedFieldOfViewLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDigitalWindowControl, IDigitalWindowControl_Vtbl, 0x23b69eff_65d2_53ea_8780_de582b48b544);
impl windows_core::RuntimeType for IDigitalWindowControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDigitalWindowControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut DigitalWindowMode) -> windows_core::HRESULT,
    pub CurrentMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DigitalWindowMode) -> windows_core::HRESULT,
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, DigitalWindowMode) -> windows_core::HRESULT,
    pub ConfigureWithBounds: unsafe extern "system" fn(*mut core::ffi::c_void, DigitalWindowMode, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCapabilities: usize,
    pub GetCapabilityForSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExposureCompensationControl, IExposureCompensationControl_Vtbl, 0x81c8e834_dcec_4011_a610_1f3847e64aca);
impl windows_core::RuntimeType for IExposureCompensationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExposureCompensationControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExposureControl, IExposureControl_Vtbl, 0x09e8cbe2_ad96_4f28_a0e0_96ed7e1b5fd2);
impl windows_core::RuntimeType for IExposureControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExposureControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Auto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExposurePriorityVideoControl, IExposurePriorityVideoControl_Vtbl, 0x2cb240a3_5168_4271_9ea5_47621a98a352);
impl windows_core::RuntimeType for IExposurePriorityVideoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExposurePriorityVideoControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFlashControl, IFlashControl_Vtbl, 0xdef41dbe_7d68_45e3_8c0f_be7bb32837d0);
impl windows_core::RuntimeType for IFlashControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlashControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Auto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFlashControl2, IFlashControl2_Vtbl, 0x7d29cc9e_75e1_4af7_bd7d_4e38e1c06cd6);
impl windows_core::RuntimeType for IFlashControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlashControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AssistantLightSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AssistantLightEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAssistantLightEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusControl, IFocusControl_Vtbl, 0xc0d889f6_5228_4453_b153_85606592b238);
impl windows_core::RuntimeType for IFocusControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFocusControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPresets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPresets: usize,
    pub Preset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusPreset) -> windows_core::HRESULT,
    pub SetPresetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, FocusPreset, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPresetWithCompletionOptionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, FocusPreset, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FocusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusControl2, IFocusControl2_Vtbl, 0x3f7cff48_c534_4e9e_94c3_52ef2afd5d07);
impl windows_core::RuntimeType for IFocusControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFocusControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FocusChangedSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub WaitForFocusSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusModes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusDistances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusDistances: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusRanges: usize,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusMode) -> windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaCaptureFocusState) -> windows_core::HRESULT,
    pub UnlockAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusSettings, IFocusSettings_Vtbl, 0x79958f6b_3263_4275_85d6_aeae891c96ee);
impl windows_core::RuntimeType for IFocusSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFocusSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, FocusMode) -> windows_core::HRESULT,
    pub AutoFocusRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoFocusRange) -> windows_core::HRESULT,
    pub SetAutoFocusRange: unsafe extern "system" fn(*mut core::ffi::c_void, AutoFocusRange) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Distance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetWaitForFocus: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DisableDriverFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDisableDriverFallback: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHdrVideoControl, IHdrVideoControl_Vtbl, 0x55d8e2d0_30c0_43bf_9b9a_9799d70ced94);
impl windows_core::RuntimeType for IHdrVideoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHdrVideoControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HdrVideoMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, HdrVideoMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInfraredTorchControl, IInfraredTorchControl_Vtbl, 0x1cba2c83_6cb6_5a04_a6fc_3be7b33ff056);
impl windows_core::RuntimeType for IInfraredTorchControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInfraredTorchControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub CurrentMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InfraredTorchMode) -> windows_core::HRESULT,
    pub SetCurrentMode: unsafe extern "system" fn(*mut core::ffi::c_void, InfraredTorchMode) -> windows_core::HRESULT,
    pub MinPower: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxPower: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PowerStep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Power: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPower: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIsoSpeedControl, IIsoSpeedControl_Vtbl, 0x27b6c322_25ad_4f1b_aaab_524ab376ca33);
impl windows_core::RuntimeType for IIsoSpeedControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIsoSpeedControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedPresets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedPresets: usize,
    #[cfg(feature = "deprecated")]
    pub Preset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IsoSpeedPreset) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Preset: usize,
    #[cfg(feature = "deprecated")]
    pub SetPresetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, IsoSpeedPreset, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPresetAsync: usize,
}
windows_core::imp::define_interface!(IIsoSpeedControl2, IIsoSpeedControl2_Vtbl, 0x6f1578f2_6d77_4f8a_8c2f_6130b6395053);
impl windows_core::RuntimeType for IIsoSpeedControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIsoSpeedControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Auto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeypadPressedEventArgs, IKeypadPressedEventArgs_Vtbl, 0xd3a43900_b4fa_49cd_9442_89af6568f601);
impl windows_core::RuntimeType for IKeypadPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeypadPressedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TelephonyKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TelephonyKey) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILowLagPhotoControl, ILowLagPhotoControl_Vtbl, 0x6d5c4dd0_fadf_415d_aee6_3baa529300c9);
impl windows_core::RuntimeType for ILowLagPhotoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLagPhotoControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::MediaThumbnailFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::MediaThumbnailFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILowLagPhotoSequenceControl, ILowLagPhotoSequenceControl_Vtbl, 0x3dcf909d_6d16_409c_bafe_b9a594c6fde6);
impl windows_core::RuntimeType for ILowLagPhotoSequenceControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLagPhotoSequenceControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxPastPhotos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub PastPhotoLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPastPhotoLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::MediaThumbnailFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::MediaThumbnailFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaDeviceControl, IMediaDeviceControl_Vtbl, 0xefa8dfa9_6f75_4863_ba0b_583f3036b4de);
impl windows_core::RuntimeType for IMediaDeviceControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaDeviceControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryGetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut bool) -> windows_core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut bool) -> windows_core::HRESULT,
    pub TryGetAuto: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool, *mut bool) -> windows_core::HRESULT,
    pub TrySetAuto: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaDeviceControlCapabilities, IMediaDeviceControlCapabilities_Vtbl, 0x23005816_eb85_43e2_b92b_8240d5ee70ec);
impl windows_core::RuntimeType for IMediaDeviceControlCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaDeviceControlCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Default: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AutoModeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaDeviceController, IMediaDeviceController_Vtbl, 0xf6f8f5ce_209a_48fb_86fc_d44578f317e6);
impl core::ops::Deref for IMediaDeviceController {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaDeviceController, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaDeviceController {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAvailableMediaStreamProperties)(windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMediaStreamProperties)(windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<P0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetMediaStreamPropertiesAsync)(windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IMediaDeviceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaDeviceController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetAvailableMediaStreamProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaStreamType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetAvailableMediaStreamProperties: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetMediaStreamProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaStreamType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetMediaStreamProperties: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub SetMediaStreamPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::MediaStreamType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))]
    SetMediaStreamPropertiesAsync: usize,
}
windows_core::imp::define_interface!(IMediaDeviceStatics, IMediaDeviceStatics_Vtbl, 0xaa2d9a40_909f_4bba_bf8b_0c0d296f14f0);
impl windows_core::RuntimeType for IMediaDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAudioCaptureSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetAudioRenderSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetVideoCaptureSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDefaultAudioCaptureId: unsafe extern "system" fn(*mut core::ffi::c_void, AudioDeviceRole, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDefaultAudioRenderId: unsafe extern "system" fn(*mut core::ffi::c_void, AudioDeviceRole, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DefaultAudioRenderDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDefaultAudioRenderDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IModuleCommandResult, IModuleCommandResult_Vtbl, 0x520d1eb4_1374_4c7d_b1e4_39dcdf3eae4e);
impl windows_core::RuntimeType for IModuleCommandResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IModuleCommandResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SendCommandStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
}
windows_core::imp::define_interface!(IOpticalImageStabilizationControl, IOpticalImageStabilizationControl_Vtbl, 0xbfad9c1d_00bc_423b_8eb2_a0178ca94247);
impl windows_core::RuntimeType for IOpticalImageStabilizationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOpticalImageStabilizationControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OpticalImageStabilizationMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, OpticalImageStabilizationMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPanelBasedOptimizationControl, IPanelBasedOptimizationControl_Vtbl, 0x33323223_6247_5419_a5a4_3d808645d917);
impl windows_core::RuntimeType for IPanelBasedOptimizationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPanelBasedOptimizationControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Panel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Enumeration::Panel) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Panel: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Devices::Enumeration::Panel) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPanel: usize,
}
windows_core::imp::define_interface!(IPhotoConfirmationControl, IPhotoConfirmationControl_Vtbl, 0xc8f3f363_ff5e_4582_a9a8_0550f85a4a76);
impl windows_core::RuntimeType for IPhotoConfirmationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhotoConfirmationControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub PixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::MediaPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PixelFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::MediaPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPixelFormat: usize,
}
windows_core::imp::define_interface!(IRedialRequestedEventArgs, IRedialRequestedEventArgs_Vtbl, 0x7eb55209_76ab_4c31_b40e_4b58379d580c);
impl windows_core::RuntimeType for IRedialRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRedialRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRegionOfInterest, IRegionOfInterest_Vtbl, 0xe5ecc834_ce66_4e05_a78f_cf391a5ec2d1);
impl windows_core::RuntimeType for IRegionOfInterest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRegionOfInterest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoFocusEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoFocusEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AutoWhiteBalanceEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoWhiteBalanceEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AutoExposureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoExposureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRegionOfInterest2, IRegionOfInterest2_Vtbl, 0x19fe2a91_73aa_4d51_8a9d_56ccf7db7f54);
impl windows_core::RuntimeType for IRegionOfInterest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRegionOfInterest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RegionOfInterestType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, RegionOfInterestType) -> windows_core::HRESULT,
    pub BoundsNormalized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetBoundsNormalized: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRegionsOfInterestControl, IRegionsOfInterestControl_Vtbl, 0xc323f527_ab0b_4558_8b5b_df5693db0378);
impl windows_core::RuntimeType for IRegionsOfInterestControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRegionsOfInterestControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxRegions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsWithLockAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsWithLockAsync: usize,
    pub ClearRegionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutoFocusSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AutoWhiteBalanceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AutoExposureSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneModeControl, ISceneModeControl_Vtbl, 0xd48e5af7_8d59_4854_8c62_12c70ba89b7c);
impl windows_core::RuntimeType for ISceneModeControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneModeControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CaptureSceneMode) -> windows_core::HRESULT,
    pub SetValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, CaptureSceneMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITorchControl, ITorchControl_Vtbl, 0xa6053665_8250_416c_919a_724296afa306);
impl windows_core::RuntimeType for ITorchControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITorchControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoDeviceController, IVideoDeviceController_Vtbl, 0x99555575_2e2e_40b8_b6c7_f82d10013210);
impl windows_core::RuntimeType for IVideoDeviceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoDeviceController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Brightness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Contrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WhiteBalance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BacklightCompensation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Zoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Roll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Exposure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Focus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub TrySetPowerlineFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, super::Capture::PowerlineFrequency, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TrySetPowerlineFrequency: usize,
    #[cfg(feature = "Media_Capture")]
    pub TryGetPowerlineFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Capture::PowerlineFrequency, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TryGetPowerlineFrequency: usize,
}
windows_core::imp::define_interface!(IVideoDeviceControllerGetDevicePropertyResult, IVideoDeviceControllerGetDevicePropertyResult_Vtbl, 0xc5d88395_6ed5_4790_8b5d_0ef13935d0f8);
impl windows_core::RuntimeType for IVideoDeviceControllerGetDevicePropertyResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoDeviceControllerGetDevicePropertyResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VideoDeviceControllerGetDevicePropertyStatus) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoTemporalDenoisingControl, IVideoTemporalDenoisingControl_Vtbl, 0x7ab34735_3e2a_4a32_baff_4358c4fbdd57);
impl windows_core::RuntimeType for IVideoTemporalDenoisingControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoTemporalDenoisingControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VideoTemporalDenoisingMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, VideoTemporalDenoisingMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWhiteBalanceControl, IWhiteBalanceControl_Vtbl, 0x781f047e_7162_49c8_a8f9_9481c565363e);
impl windows_core::RuntimeType for IWhiteBalanceControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWhiteBalanceControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Preset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ColorTemperaturePreset) -> windows_core::HRESULT,
    pub SetPresetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ColorTemperaturePreset, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValueAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IZoomControl, IZoomControl_Vtbl, 0x3a1e0b12_32da_4c17_bfd7_8d0c73c8f5a5);
impl windows_core::RuntimeType for IZoomControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IZoomControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IZoomControl2, IZoomControl2_Vtbl, 0x69843db0_2e99_4641_8529_184f319d1671);
impl windows_core::RuntimeType for IZoomControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IZoomControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ZoomTransitionMode) -> windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IZoomSettings, IZoomSettings_Vtbl, 0x6ad66b24_14b4_4bfd_b18f_88fe24463b52);
impl windows_core::RuntimeType for IZoomSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IZoomSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ZoomTransitionMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, ZoomTransitionMode) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdvancedPhotoCaptureSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdvancedPhotoCaptureSettings, windows_core::IUnknown, windows_core::IInspectable);
impl AdvancedPhotoCaptureSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AdvancedPhotoCaptureSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> windows_core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: AdvancedPhotoMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AdvancedPhotoCaptureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdvancedPhotoCaptureSettings>();
}
unsafe impl windows_core::Interface for AdvancedPhotoCaptureSettings {
    type Vtable = IAdvancedPhotoCaptureSettings_Vtbl;
    const IID: windows_core::GUID = <IAdvancedPhotoCaptureSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdvancedPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoCaptureSettings";
}
unsafe impl Send for AdvancedPhotoCaptureSettings {}
unsafe impl Sync for AdvancedPhotoCaptureSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdvancedPhotoControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdvancedPhotoControl, windows_core::IUnknown, windows_core::IInspectable);
impl AdvancedPhotoControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Mode(&self) -> windows_core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Configure<P0>(&self, settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AdvancedPhotoCaptureSettings>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Configure)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for AdvancedPhotoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdvancedPhotoControl>();
}
unsafe impl windows_core::Interface for AdvancedPhotoControl {
    type Vtable = IAdvancedPhotoControl_Vtbl;
    const IID: windows_core::GUID = <IAdvancedPhotoControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdvancedPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoControl";
}
unsafe impl Send for AdvancedPhotoControl {}
unsafe impl Sync for AdvancedPhotoControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioDeviceController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioDeviceController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioDeviceController, IMediaDeviceController);
impl AudioDeviceController {
    pub fn SetMuted(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMuted)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Muted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Muted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVolumePercent(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVolumePercent)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VolumePercent(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VolumePercent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn AudioCaptureEffectsManager(&self) -> windows_core::Result<super::Effects::AudioCaptureEffectsManager> {
        let this = &windows_core::Interface::cast::<IAudioDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioCaptureEffectsManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &windows_core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAvailableMediaStreamProperties)(windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &windows_core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMediaStreamProperties)(windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<P0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = &windows_core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetMediaStreamPropertiesAsync)(windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AudioDeviceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioDeviceController>();
}
unsafe impl windows_core::Interface for AudioDeviceController {
    type Vtable = IAudioDeviceController_Vtbl;
    const IID: windows_core::GUID = <IAudioDeviceController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceController";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioDeviceModule(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioDeviceModule, windows_core::IUnknown, windows_core::IInspectable);
impl AudioDeviceModule {
    pub fn ClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstanceId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MajorVersion(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MajorVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinorVersion(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinorVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendCommandAsync<P0>(&self, command: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendCommandAsync)(windows_core::Interface::as_raw(this), command.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AudioDeviceModule {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioDeviceModule>();
}
unsafe impl windows_core::Interface for AudioDeviceModule {
    type Vtable = IAudioDeviceModule_Vtbl;
    const IID: windows_core::GUID = <IAudioDeviceModule as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioDeviceModule {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModule";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioDeviceModuleNotificationEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioDeviceModuleNotificationEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AudioDeviceModuleNotificationEventArgs {
    pub fn Module(&self) -> windows_core::Result<AudioDeviceModule> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Module)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn NotificationData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AudioDeviceModuleNotificationEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioDeviceModuleNotificationEventArgs>();
}
unsafe impl windows_core::Interface for AudioDeviceModuleNotificationEventArgs {
    type Vtable = IAudioDeviceModuleNotificationEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAudioDeviceModuleNotificationEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioDeviceModuleNotificationEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs";
}
unsafe impl Send for AudioDeviceModuleNotificationEventArgs {}
unsafe impl Sync for AudioDeviceModuleNotificationEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioDeviceModulesManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioDeviceModulesManager, windows_core::IUnknown, windows_core::IInspectable);
impl AudioDeviceModulesManager {
    pub fn ModuleNotificationReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModuleNotificationReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveModuleNotificationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveModuleNotificationReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllById(&self, moduleid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllById)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(moduleid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAll)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(deviceid: &windows_core::HSTRING) -> windows_core::Result<AudioDeviceModulesManager> {
        Self::IAudioDeviceModulesManagerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioDeviceModulesManagerFactory<R, F: FnOnce(&IAudioDeviceModulesManagerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioDeviceModulesManager, IAudioDeviceModulesManagerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioDeviceModulesManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioDeviceModulesManager>();
}
unsafe impl windows_core::Interface for AudioDeviceModulesManager {
    type Vtable = IAudioDeviceModulesManager_Vtbl;
    const IID: windows_core::GUID = <IAudioDeviceModulesManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioDeviceModulesManager {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModulesManager";
}
unsafe impl Send for AudioDeviceModulesManager {}
unsafe impl Sync for AudioDeviceModulesManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CallControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CallControl, windows_core::IUnknown, windows_core::IInspectable);
impl CallControl {
    pub fn IndicateNewIncomingCall(&self, enableringer: bool, callerid: &windows_core::HSTRING) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndicateNewIncomingCall)(windows_core::Interface::as_raw(this), enableringer, core::mem::transmute_copy(callerid), &mut result__).map(|| result__)
        }
    }
    pub fn IndicateNewOutgoingCall(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndicateNewOutgoingCall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IndicateActiveCall(&self, calltoken: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).IndicateActiveCall)(windows_core::Interface::as_raw(this), calltoken).ok() }
    }
    pub fn EndCall(&self, calltoken: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EndCall)(windows_core::Interface::as_raw(this), calltoken).ok() }
    }
    pub fn HasRinger(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasRinger)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AnswerRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<CallControlEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnswerRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAnswerRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAnswerRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HangUpRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<CallControlEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HangUpRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHangUpRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHangUpRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DialRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DialRequestedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDialRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDialRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RedialRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<RedialRequestedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedialRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRedialRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRedialRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn KeypadPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<KeypadPressedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeypadPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeypadPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeypadPressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AudioTransferRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<CallControlEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioTransferRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioTransferRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioTransferRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromId(deviceid: &windows_core::HSTRING) -> windows_core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICallControlStatics<R, F: FnOnce(&ICallControlStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CallControl, ICallControlStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CallControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICallControl>();
}
unsafe impl windows_core::Interface for CallControl {
    type Vtable = ICallControl_Vtbl;
    const IID: windows_core::GUID = <ICallControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CallControl {
    const NAME: &'static str = "Windows.Media.Devices.CallControl";
}
unsafe impl Send for CallControl {}
unsafe impl Sync for CallControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CameraOcclusionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraOcclusionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl CameraOcclusionInfo {
    pub fn GetState(&self) -> windows_core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsOcclusionKindSupported(&self, occlusionkind: CameraOcclusionKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOcclusionKindSupported)(windows_core::Interface::as_raw(this), occlusionkind, &mut result__).map(|| result__)
        }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs>>,
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
impl windows_core::RuntimeType for CameraOcclusionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraOcclusionInfo>();
}
unsafe impl windows_core::Interface for CameraOcclusionInfo {
    type Vtable = ICameraOcclusionInfo_Vtbl;
    const IID: windows_core::GUID = <ICameraOcclusionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraOcclusionInfo {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionInfo";
}
unsafe impl Send for CameraOcclusionInfo {}
unsafe impl Sync for CameraOcclusionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CameraOcclusionState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraOcclusionState, windows_core::IUnknown, windows_core::IInspectable);
impl CameraOcclusionState {
    pub fn IsOccluded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOccluded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOcclusionKind(&self, occlusionkind: CameraOcclusionKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOcclusionKind)(windows_core::Interface::as_raw(this), occlusionkind, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CameraOcclusionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraOcclusionState>();
}
unsafe impl windows_core::Interface for CameraOcclusionState {
    type Vtable = ICameraOcclusionState_Vtbl;
    const IID: windows_core::GUID = <ICameraOcclusionState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraOcclusionState {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionState";
}
unsafe impl Send for CameraOcclusionState {}
unsafe impl Sync for CameraOcclusionState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CameraOcclusionStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CameraOcclusionStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CameraOcclusionStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CameraOcclusionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICameraOcclusionStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for CameraOcclusionStateChangedEventArgs {
    type Vtable = ICameraOcclusionStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICameraOcclusionStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CameraOcclusionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionStateChangedEventArgs";
}
unsafe impl Send for CameraOcclusionStateChangedEventArgs {}
unsafe impl Sync for CameraOcclusionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DefaultAudioCaptureDeviceChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DefaultAudioCaptureDeviceChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DefaultAudioCaptureDeviceChangedEventArgs, IDefaultAudioDeviceChangedEventArgs);
impl DefaultAudioCaptureDeviceChangedEventArgs {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Role(&self) -> windows_core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Role)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DefaultAudioCaptureDeviceChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDefaultAudioDeviceChangedEventArgs>();
}
unsafe impl windows_core::Interface for DefaultAudioCaptureDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDefaultAudioDeviceChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DefaultAudioCaptureDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs";
}
unsafe impl Send for DefaultAudioCaptureDeviceChangedEventArgs {}
unsafe impl Sync for DefaultAudioCaptureDeviceChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DefaultAudioRenderDeviceChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DefaultAudioRenderDeviceChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DefaultAudioRenderDeviceChangedEventArgs, IDefaultAudioDeviceChangedEventArgs);
impl DefaultAudioRenderDeviceChangedEventArgs {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Role(&self) -> windows_core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Role)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DefaultAudioRenderDeviceChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDefaultAudioDeviceChangedEventArgs>();
}
unsafe impl windows_core::Interface for DefaultAudioRenderDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDefaultAudioDeviceChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DefaultAudioRenderDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs";
}
unsafe impl Send for DefaultAudioRenderDeviceChangedEventArgs {}
unsafe impl Sync for DefaultAudioRenderDeviceChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DialRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DialRequestedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Contact(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DialRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialRequestedEventArgs>();
}
unsafe impl windows_core::Interface for DialRequestedEventArgs {
    type Vtable = IDialRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDialRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DialRequestedEventArgs";
}
unsafe impl Send for DialRequestedEventArgs {}
unsafe impl Sync for DialRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DigitalWindowBounds(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DigitalWindowBounds, windows_core::IUnknown, windows_core::IInspectable);
impl DigitalWindowBounds {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DigitalWindowBounds, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NormalizedOriginTop(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizedOriginTop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNormalizedOriginTop(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNormalizedOriginTop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalizedOriginLeft(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizedOriginLeft)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNormalizedOriginLeft(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNormalizedOriginLeft)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for DigitalWindowBounds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDigitalWindowBounds>();
}
unsafe impl windows_core::Interface for DigitalWindowBounds {
    type Vtable = IDigitalWindowBounds_Vtbl;
    const IID: windows_core::GUID = <IDigitalWindowBounds as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DigitalWindowBounds {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowBounds";
}
unsafe impl Send for DigitalWindowBounds {}
unsafe impl Sync for DigitalWindowBounds {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DigitalWindowCapability(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DigitalWindowCapability, windows_core::IUnknown, windows_core::IInspectable);
impl DigitalWindowCapability {
    pub fn Width(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinScaleValue(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinScaleValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxScaleValue(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxScaleValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinScaleValueWithoutUpsampling(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinScaleValueWithoutUpsampling)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NormalizedFieldOfViewLimit(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizedFieldOfViewLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DigitalWindowCapability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDigitalWindowCapability>();
}
unsafe impl windows_core::Interface for DigitalWindowCapability {
    type Vtable = IDigitalWindowCapability_Vtbl;
    const IID: windows_core::GUID = <IDigitalWindowCapability as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DigitalWindowCapability {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowCapability";
}
unsafe impl Send for DigitalWindowCapability {}
unsafe impl Sync for DigitalWindowCapability {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DigitalWindowControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DigitalWindowControl, windows_core::IUnknown, windows_core::IInspectable);
impl DigitalWindowControl {
    pub fn IsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedModes(&self) -> windows_core::Result<windows_core::Array<DigitalWindowMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), windows_core::Array::<DigitalWindowMode>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CurrentMode(&self) -> windows_core::Result<DigitalWindowMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBounds(&self) -> windows_core::Result<DigitalWindowBounds> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBounds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configure(&self, digitalwindowmode: DigitalWindowMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Configure)(windows_core::Interface::as_raw(this), digitalwindowmode).ok() }
    }
    pub fn ConfigureWithBounds<P0>(&self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DigitalWindowBounds>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ConfigureWithBounds)(windows_core::Interface::as_raw(this), digitalwindowmode, digitalwindowbounds.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCapabilities(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCapabilityForSize(&self, width: i32, height: i32) -> windows_core::Result<DigitalWindowCapability> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCapabilityForSize)(windows_core::Interface::as_raw(this), width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DigitalWindowControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDigitalWindowControl>();
}
unsafe impl windows_core::Interface for DigitalWindowControl {
    type Vtable = IDigitalWindowControl_Vtbl;
    const IID: windows_core::GUID = <IDigitalWindowControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DigitalWindowControl {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowControl";
}
unsafe impl Send for DigitalWindowControl {}
unsafe impl Sync for DigitalWindowControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExposureCompensationControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExposureCompensationControl, windows_core::IUnknown, windows_core::IInspectable);
impl ExposureCompensationControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Min(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValueAsync(&self, value: f32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetValueAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ExposureCompensationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExposureCompensationControl>();
}
unsafe impl windows_core::Interface for ExposureCompensationControl {
    type Vtable = IExposureCompensationControl_Vtbl;
    const IID: windows_core::GUID = <IExposureCompensationControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureCompensationControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExposureControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExposureControl, windows_core::IUnknown, windows_core::IInspectable);
impl ExposureControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Auto(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Auto)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoAsync(&self, value: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAutoAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Min(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValueAsync(&self, shutterduration: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetValueAsync)(windows_core::Interface::as_raw(this), shutterduration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ExposureControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExposureControl>();
}
unsafe impl windows_core::Interface for ExposureControl {
    type Vtable = IExposureControl_Vtbl;
    const IID: windows_core::GUID = <IExposureControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExposurePriorityVideoControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExposurePriorityVideoControl, windows_core::IUnknown, windows_core::IInspectable);
impl ExposurePriorityVideoControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ExposurePriorityVideoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExposurePriorityVideoControl>();
}
unsafe impl windows_core::Interface for ExposurePriorityVideoControl {
    type Vtable = IExposurePriorityVideoControl_Vtbl;
    const IID: windows_core::GUID = <IExposurePriorityVideoControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExposurePriorityVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposurePriorityVideoControl";
}
unsafe impl Send for ExposurePriorityVideoControl {}
unsafe impl Sync for ExposurePriorityVideoControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FlashControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FlashControl, windows_core::IUnknown, windows_core::IInspectable);
impl FlashControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PowerSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RedEyeReductionSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedEyeReductionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Auto(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Auto)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuto)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RedEyeReduction(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedEyeReduction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRedEyeReduction(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRedEyeReduction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerPercent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPowerPercent)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AssistantLightSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AssistantLightSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AssistantLightEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AssistantLightEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAssistantLightEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFlashControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAssistantLightEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for FlashControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFlashControl>();
}
unsafe impl windows_core::Interface for FlashControl {
    type Vtable = IFlashControl_Vtbl;
    const IID: windows_core::GUID = <IFlashControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FlashControl {
    const NAME: &'static str = "Windows.Media.Devices.FlashControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FocusControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusControl, windows_core::IUnknown, windows_core::IInspectable);
impl FocusControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPresets(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<FocusPreset>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPresets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Preset(&self) -> windows_core::Result<FocusPreset> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Preset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPresetAsync(&self, preset: FocusPreset) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPresetAsync)(windows_core::Interface::as_raw(this), preset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPresetWithCompletionOptionAsync(&self, preset: FocusPreset, completebeforefocus: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPresetWithCompletionOptionAsync)(windows_core::Interface::as_raw(this), preset, completebeforefocus, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Min(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValueAsync(&self, focus: u32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetValueAsync)(windows_core::Interface::as_raw(this), focus, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FocusAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FocusChangedSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusChangedSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WaitForFocusSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WaitForFocusSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<FocusMode>> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedFocusModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusDistances(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedFocusDistances)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AutoFocusRange>> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedFocusRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Mode(&self) -> windows_core::Result<FocusMode> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FocusState(&self) -> windows_core::Result<MediaCaptureFocusState> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnlockAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnlockAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LockAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LockAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configure<P0>(&self, settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<FocusSettings>,
    {
        let this = &windows_core::Interface::cast::<IFocusControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Configure)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for FocusControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusControl>();
}
unsafe impl windows_core::Interface for FocusControl {
    type Vtable = IFocusControl_Vtbl;
    const IID: windows_core::GUID = <IFocusControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusControl {
    const NAME: &'static str = "Windows.Media.Devices.FocusControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FocusSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusSettings, windows_core::IUnknown, windows_core::IInspectable);
impl FocusSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FocusSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> windows_core::Result<FocusMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: FocusMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoFocusRange(&self) -> windows_core::Result<AutoFocusRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoFocusRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoFocusRange(&self, value: AutoFocusRange) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoFocusRange)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Distance(&self) -> windows_core::Result<super::super::Foundation::IReference<ManualFocusDistance>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Distance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDistance<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<ManualFocusDistance>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDistance)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn WaitForFocus(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WaitForFocus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWaitForFocus(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWaitForFocus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisableDriverFallback(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisableDriverFallback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisableDriverFallback(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisableDriverFallback)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for FocusSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusSettings>();
}
unsafe impl windows_core::Interface for FocusSettings {
    type Vtable = IFocusSettings_Vtbl;
    const IID: windows_core::GUID = <IFocusSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusSettings {
    const NAME: &'static str = "Windows.Media.Devices.FocusSettings";
}
unsafe impl Send for FocusSettings {}
unsafe impl Sync for FocusSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HdrVideoControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HdrVideoControl, windows_core::IUnknown, windows_core::IInspectable);
impl HdrVideoControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<HdrVideoMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Mode(&self) -> windows_core::Result<HdrVideoMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: HdrVideoMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for HdrVideoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHdrVideoControl>();
}
unsafe impl windows_core::Interface for HdrVideoControl {
    type Vtable = IHdrVideoControl_Vtbl;
    const IID: windows_core::GUID = <IHdrVideoControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HdrVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.HdrVideoControl";
}
unsafe impl Send for HdrVideoControl {}
unsafe impl Sync for HdrVideoControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InfraredTorchControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InfraredTorchControl, windows_core::IUnknown, windows_core::IInspectable);
impl InfraredTorchControl {
    pub fn IsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentMode(&self) -> windows_core::Result<InfraredTorchMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCurrentMode(&self, value: InfraredTorchMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCurrentMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinPower(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinPower)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPower(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPower)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PowerStep(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Power(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Power)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPower(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPower)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InfraredTorchControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInfraredTorchControl>();
}
unsafe impl windows_core::Interface for InfraredTorchControl {
    type Vtable = IInfraredTorchControl_Vtbl;
    const IID: windows_core::GUID = <IInfraredTorchControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InfraredTorchControl {
    const NAME: &'static str = "Windows.Media.Devices.InfraredTorchControl";
}
unsafe impl Send for InfraredTorchControl {}
unsafe impl Sync for InfraredTorchControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IsoSpeedControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IsoSpeedControl, windows_core::IUnknown, windows_core::IInspectable);
impl IsoSpeedControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SupportedPresets(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPresets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Preset(&self) -> windows_core::Result<IsoSpeedPreset> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Preset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetPresetAsync(&self, preset: IsoSpeedPreset) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPresetAsync)(windows_core::Interface::as_raw(this), preset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Min(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValueAsync(&self, isospeed: u32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetValueAsync)(windows_core::Interface::as_raw(this), isospeed, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Auto(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Auto)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAutoAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IsoSpeedControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIsoSpeedControl>();
}
unsafe impl windows_core::Interface for IsoSpeedControl {
    type Vtable = IIsoSpeedControl_Vtbl;
    const IID: windows_core::GUID = <IIsoSpeedControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.IsoSpeedControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KeypadPressedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeypadPressedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl KeypadPressedEventArgs {
    pub fn TelephonyKey(&self) -> windows_core::Result<TelephonyKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TelephonyKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for KeypadPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeypadPressedEventArgs>();
}
unsafe impl windows_core::Interface for KeypadPressedEventArgs {
    type Vtable = IKeypadPressedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IKeypadPressedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeypadPressedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.KeypadPressedEventArgs";
}
unsafe impl Send for KeypadPressedEventArgs {}
unsafe impl Sync for KeypadPressedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LowLagPhotoControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LowLagPhotoControl, windows_core::IUnknown, windows_core::IInspectable);
impl LowLagPhotoControl {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<P0>(&self, captureproperties: P0) -> windows_core::Result<super::MediaProperties::MediaRatio>
    where
        P0: windows_core::Param<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHighestConcurrentFrameRate)(windows_core::Interface::as_raw(this), captureproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> windows_core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentFrameRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThumbnailEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbnailEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnailEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> windows_core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbnailFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnailFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredThumbnailSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredThumbnailSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareAcceleratedThumbnailSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LowLagPhotoControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILowLagPhotoControl>();
}
unsafe impl windows_core::Interface for LowLagPhotoControl {
    type Vtable = ILowLagPhotoControl_Vtbl;
    const IID: windows_core::GUID = <ILowLagPhotoControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LowLagPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LowLagPhotoSequenceControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LowLagPhotoSequenceControl, windows_core::IUnknown, windows_core::IInspectable);
impl LowLagPhotoSequenceControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPastPhotos(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPastPhotos)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPhotosPerSecond(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPhotosPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PastPhotoLimit(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PastPhotoLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPastPhotoLimit(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPastPhotoLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhotosPerSecondLimit(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotosPerSecondLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPhotosPerSecondLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<P0>(&self, captureproperties: P0) -> windows_core::Result<super::MediaProperties::MediaRatio>
    where
        P0: windows_core::Param<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHighestConcurrentFrameRate)(windows_core::Interface::as_raw(this), captureproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> windows_core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentFrameRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThumbnailEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbnailEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnailEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> windows_core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbnailFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnailFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredThumbnailSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredThumbnailSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareAcceleratedThumbnailSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LowLagPhotoSequenceControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILowLagPhotoSequenceControl>();
}
unsafe impl windows_core::Interface for LowLagPhotoSequenceControl {
    type Vtable = ILowLagPhotoSequenceControl_Vtbl;
    const IID: windows_core::GUID = <ILowLagPhotoSequenceControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LowLagPhotoSequenceControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoSequenceControl";
}
pub struct MediaDevice;
impl MediaDevice {
    pub fn GetAudioCaptureSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioCaptureSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAudioRenderSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioRenderSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetVideoCaptureSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVideoCaptureSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAudioCaptureId(role: AudioDeviceRole) -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAudioCaptureId)(windows_core::Interface::as_raw(this), role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAudioRenderId(role: AudioDeviceRole) -> windows_core::Result<windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAudioRenderId)(windows_core::Interface::as_raw(this), role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DefaultAudioCaptureDeviceChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs>>,
    {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultAudioCaptureDeviceChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveDefaultAudioCaptureDeviceChanged(cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveDefaultAudioCaptureDeviceChanged)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    pub fn DefaultAudioRenderDeviceChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs>>,
    {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultAudioRenderDeviceChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveDefaultAudioRenderDeviceChanged(cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveDefaultAudioRenderDeviceChanged)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc(hidden)]
    pub fn IMediaDeviceStatics<R, F: FnOnce(&IMediaDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaDevice, IMediaDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for MediaDevice {
    const NAME: &'static str = "Windows.Media.Devices.MediaDevice";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaDeviceControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaDeviceControl, windows_core::IUnknown, windows_core::IInspectable);
impl MediaDeviceControl {
    pub fn Capabilities(&self) -> windows_core::Result<MediaDeviceControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetValue(&self, value: &mut f64) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetValue)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn TrySetValue(&self, value: f64) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetValue)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn TryGetAuto(&self, value: &mut bool) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAuto)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn TrySetAuto(&self, value: bool) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetAuto)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaDeviceControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaDeviceControl>();
}
unsafe impl windows_core::Interface for MediaDeviceControl {
    type Vtable = IMediaDeviceControl_Vtbl;
    const IID: windows_core::GUID = <IMediaDeviceControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaDeviceControl {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaDeviceControlCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaDeviceControlCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl MediaDeviceControlCapabilities {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Min(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Default(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Default)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutoModeSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoModeSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaDeviceControlCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaDeviceControlCapabilities>();
}
unsafe impl windows_core::Interface for MediaDeviceControlCapabilities {
    type Vtable = IMediaDeviceControlCapabilities_Vtbl;
    const IID: windows_core::GUID = <IMediaDeviceControlCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaDeviceControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControlCapabilities";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ModuleCommandResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ModuleCommandResult, windows_core::IUnknown, windows_core::IInspectable);
impl ModuleCommandResult {
    pub fn Status(&self) -> windows_core::Result<SendCommandStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ModuleCommandResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IModuleCommandResult>();
}
unsafe impl windows_core::Interface for ModuleCommandResult {
    type Vtable = IModuleCommandResult_Vtbl;
    const IID: windows_core::GUID = <IModuleCommandResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ModuleCommandResult {
    const NAME: &'static str = "Windows.Media.Devices.ModuleCommandResult";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OpticalImageStabilizationControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OpticalImageStabilizationControl, windows_core::IUnknown, windows_core::IInspectable);
impl OpticalImageStabilizationControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Mode(&self) -> windows_core::Result<OpticalImageStabilizationMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: OpticalImageStabilizationMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for OpticalImageStabilizationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOpticalImageStabilizationControl>();
}
unsafe impl windows_core::Interface for OpticalImageStabilizationControl {
    type Vtable = IOpticalImageStabilizationControl_Vtbl;
    const IID: windows_core::GUID = <IOpticalImageStabilizationControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OpticalImageStabilizationControl {
    const NAME: &'static str = "Windows.Media.Devices.OpticalImageStabilizationControl";
}
unsafe impl Send for OpticalImageStabilizationControl {}
unsafe impl Sync for OpticalImageStabilizationControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PanelBasedOptimizationControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PanelBasedOptimizationControl, windows_core::IUnknown, windows_core::IInspectable);
impl PanelBasedOptimizationControl {
    pub fn IsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Panel(&self) -> windows_core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Panel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPanel(&self, value: super::super::Devices::Enumeration::Panel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPanel)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PanelBasedOptimizationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPanelBasedOptimizationControl>();
}
unsafe impl windows_core::Interface for PanelBasedOptimizationControl {
    type Vtable = IPanelBasedOptimizationControl_Vtbl;
    const IID: windows_core::GUID = <IPanelBasedOptimizationControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PanelBasedOptimizationControl {
    const NAME: &'static str = "Windows.Media.Devices.PanelBasedOptimizationControl";
}
unsafe impl Send for PanelBasedOptimizationControl {}
unsafe impl Sync for PanelBasedOptimizationControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhotoConfirmationControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhotoConfirmationControl, windows_core::IUnknown, windows_core::IInspectable);
impl PhotoConfirmationControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PixelFormat(&self) -> windows_core::Result<super::MediaProperties::MediaPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetPixelFormat(&self, format: super::MediaProperties::MediaPixelFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPixelFormat)(windows_core::Interface::as_raw(this), format).ok() }
    }
}
impl windows_core::RuntimeType for PhotoConfirmationControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhotoConfirmationControl>();
}
unsafe impl windows_core::Interface for PhotoConfirmationControl {
    type Vtable = IPhotoConfirmationControl_Vtbl;
    const IID: windows_core::GUID = <IPhotoConfirmationControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhotoConfirmationControl {
    const NAME: &'static str = "Windows.Media.Devices.PhotoConfirmationControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RedialRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RedialRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RedialRequestedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for RedialRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRedialRequestedEventArgs>();
}
unsafe impl windows_core::Interface for RedialRequestedEventArgs {
    type Vtable = IRedialRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRedialRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RedialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.RedialRequestedEventArgs";
}
unsafe impl Send for RedialRequestedEventArgs {}
unsafe impl Sync for RedialRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RegionOfInterest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RegionOfInterest, windows_core::IUnknown, windows_core::IInspectable);
impl RegionOfInterest {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RegionOfInterest, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AutoFocusEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoFocusEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoFocusEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoFocusEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoWhiteBalanceEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoWhiteBalanceEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoWhiteBalanceEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoWhiteBalanceEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoExposureEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoExposureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoExposureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoExposureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bounds(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBounds(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBounds)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<RegionOfInterestType> {
        let this = &windows_core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetType(&self, value: RegionOfInterestType) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BoundsNormalized(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundsNormalized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBoundsNormalized(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBoundsNormalized)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Weight(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Weight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWeight(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for RegionOfInterest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRegionOfInterest>();
}
unsafe impl windows_core::Interface for RegionOfInterest {
    type Vtable = IRegionOfInterest_Vtbl;
    const IID: windows_core::GUID = <IRegionOfInterest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RegionOfInterest {
    const NAME: &'static str = "Windows.Media.Devices.RegionOfInterest";
}
unsafe impl Send for RegionOfInterest {}
unsafe impl Sync for RegionOfInterest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RegionsOfInterestControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RegionsOfInterestControl, windows_core::IUnknown, windows_core::IInspectable);
impl RegionsOfInterestControl {
    pub fn MaxRegions(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxRegions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRegionsAsync<P0>(&self, regions: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<RegionOfInterest>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetRegionsAsync)(windows_core::Interface::as_raw(this), regions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRegionsWithLockAsync<P0>(&self, regions: P0, lockvalues: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<RegionOfInterest>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetRegionsWithLockAsync)(windows_core::Interface::as_raw(this), regions.param().abi(), lockvalues, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearRegionsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearRegionsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoFocusSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoFocusSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutoWhiteBalanceSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoWhiteBalanceSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutoExposureSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoExposureSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RegionsOfInterestControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRegionsOfInterestControl>();
}
unsafe impl windows_core::Interface for RegionsOfInterestControl {
    type Vtable = IRegionsOfInterestControl_Vtbl;
    const IID: windows_core::GUID = <IRegionsOfInterestControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RegionsOfInterestControl {
    const NAME: &'static str = "Windows.Media.Devices.RegionsOfInterestControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SceneModeControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneModeControl, windows_core::IUnknown, windows_core::IInspectable);
impl SceneModeControl {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<CaptureSceneMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValueAsync(&self, scenemode: CaptureSceneMode) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetValueAsync)(windows_core::Interface::as_raw(this), scenemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SceneModeControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneModeControl>();
}
unsafe impl windows_core::Interface for SceneModeControl {
    type Vtable = ISceneModeControl_Vtbl;
    const IID: windows_core::GUID = <ISceneModeControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneModeControl {
    const NAME: &'static str = "Windows.Media.Devices.SceneModeControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TorchControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TorchControl, windows_core::IUnknown, windows_core::IInspectable);
impl TorchControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PowerSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Enabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PowerPercent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPowerPercent)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for TorchControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITorchControl>();
}
unsafe impl windows_core::Interface for TorchControl {
    type Vtable = ITorchControl_Vtbl;
    const IID: windows_core::GUID = <ITorchControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TorchControl {
    const NAME: &'static str = "Windows.Media.Devices.TorchControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoDeviceController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoDeviceController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VideoDeviceController, IMediaDeviceController);
impl VideoDeviceController {
    pub fn SetDeviceProperty<P0>(&self, propertyid: &windows_core::HSTRING, propertyvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDeviceProperty)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyid), propertyvalue.param().abi()).ok() }
    }
    pub fn GetDeviceProperty(&self, propertyid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceProperty)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CameraOcclusionInfo(&self) -> windows_core::Result<CameraOcclusionInfo> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CameraOcclusionInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Capture")]
    pub fn TryAcquireExclusiveControl(&self, deviceid: &windows_core::HSTRING, mode: super::Capture::MediaCaptureDeviceExclusiveControlReleaseMode) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController11>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryAcquireExclusiveControl)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), mode, &mut result__).map(|| result__)
        }
    }
    pub fn LowLagPhotoSequence(&self) -> windows_core::Result<LowLagPhotoSequenceControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LowLagPhotoSequence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LowLagPhoto(&self) -> windows_core::Result<LowLagPhotoControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LowLagPhoto)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SceneModeControl(&self) -> windows_core::Result<SceneModeControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SceneModeControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TorchControl(&self) -> windows_core::Result<TorchControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TorchControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlashControl(&self) -> windows_core::Result<FlashControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlashControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WhiteBalanceControl(&self) -> windows_core::Result<WhiteBalanceControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WhiteBalanceControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExposureControl(&self) -> windows_core::Result<ExposureControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExposureControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FocusControl(&self) -> windows_core::Result<FocusControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExposureCompensationControl(&self) -> windows_core::Result<ExposureCompensationControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExposureCompensationControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsoSpeedControl(&self) -> windows_core::Result<IsoSpeedControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsoSpeedControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegionsOfInterestControl(&self) -> windows_core::Result<RegionsOfInterestControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegionsOfInterestControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrimaryUse(&self) -> windows_core::Result<CaptureUse> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryUse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPrimaryUse(&self, value: CaptureUse) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrimaryUse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Devices_Core")]
    pub fn VariablePhotoSequenceController(&self) -> windows_core::Result<Core::VariablePhotoSequenceController> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VariablePhotoSequenceController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhotoConfirmationControl(&self) -> windows_core::Result<PhotoConfirmationControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhotoConfirmationControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ZoomControl(&self) -> windows_core::Result<ZoomControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZoomControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExposurePriorityVideoControl(&self) -> windows_core::Result<ExposurePriorityVideoControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExposurePriorityVideoControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DesiredOptimization(&self) -> windows_core::Result<MediaCaptureOptimization> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredOptimization)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredOptimization(&self, value: MediaCaptureOptimization) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredOptimization)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HdrVideoControl(&self) -> windows_core::Result<HdrVideoControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HdrVideoControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpticalImageStabilizationControl(&self) -> windows_core::Result<OpticalImageStabilizationControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpticalImageStabilizationControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AdvancedPhotoControl(&self) -> windows_core::Result<AdvancedPhotoControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvancedPhotoControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDevicePropertyById<P0>(&self, propertyid: &windows_core::HSTRING, maxpropertyvaluesize: P0) -> windows_core::Result<VideoDeviceControllerGetDevicePropertyResult>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDevicePropertyById)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyid), maxpropertyvaluesize.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDevicePropertyById<P0>(&self, propertyid: &windows_core::HSTRING, propertyvalue: P0) -> windows_core::Result<VideoDeviceControllerSetDevicePropertyStatus>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetDevicePropertyById)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyid), propertyvalue.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetDevicePropertyByExtendedId<P0>(&self, extendedpropertyid: &[u8], maxpropertyvaluesize: P0) -> windows_core::Result<VideoDeviceControllerGetDevicePropertyResult>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDevicePropertyByExtendedId)(windows_core::Interface::as_raw(this), extendedpropertyid.len().try_into().unwrap(), extendedpropertyid.as_ptr(), maxpropertyvaluesize.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDevicePropertyByExtendedId(&self, extendedpropertyid: &[u8], propertyvalue: &[u8]) -> windows_core::Result<VideoDeviceControllerSetDevicePropertyStatus> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetDevicePropertyByExtendedId)(windows_core::Interface::as_raw(this), extendedpropertyid.len().try_into().unwrap(), extendedpropertyid.as_ptr(), propertyvalue.len().try_into().unwrap(), propertyvalue.as_ptr(), &mut result__).map(|| result__)
        }
    }
    pub fn VideoTemporalDenoisingControl(&self) -> windows_core::Result<VideoTemporalDenoisingControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoTemporalDenoisingControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InfraredTorchControl(&self) -> windows_core::Result<InfraredTorchControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InfraredTorchControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PanelBasedOptimizationControl(&self) -> windows_core::Result<PanelBasedOptimizationControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PanelBasedOptimizationControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DigitalWindowControl(&self) -> windows_core::Result<DigitalWindowControl> {
        let this = &windows_core::Interface::cast::<IAdvancedVideoCaptureDeviceController9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DigitalWindowControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &windows_core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAvailableMediaStreamProperties)(windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> windows_core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &windows_core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMediaStreamProperties)(windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<P0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = &windows_core::Interface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetMediaStreamPropertiesAsync)(windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Brightness(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Brightness)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Contrast(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contrast)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Hue(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WhiteBalance(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WhiteBalance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BacklightCompensation(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BacklightCompensation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Pan(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pan)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Tilt(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tilt)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Zoom(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Zoom)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Roll(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Roll)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Exposure(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Exposure)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Focus(&self) -> windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Capture")]
    pub fn TrySetPowerlineFrequency(&self, value: super::Capture::PowerlineFrequency) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetPowerlineFrequency)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Capture")]
    pub fn TryGetPowerlineFrequency(&self, value: &mut super::Capture::PowerlineFrequency) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPowerlineFrequency)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VideoDeviceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoDeviceController>();
}
unsafe impl windows_core::Interface for VideoDeviceController {
    type Vtable = IVideoDeviceController_Vtbl;
    const IID: windows_core::GUID = <IVideoDeviceController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceController";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoDeviceControllerGetDevicePropertyResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoDeviceControllerGetDevicePropertyResult, windows_core::IUnknown, windows_core::IInspectable);
impl VideoDeviceControllerGetDevicePropertyResult {
    pub fn Status(&self) -> windows_core::Result<VideoDeviceControllerGetDevicePropertyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VideoDeviceControllerGetDevicePropertyResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoDeviceControllerGetDevicePropertyResult>();
}
unsafe impl windows_core::Interface for VideoDeviceControllerGetDevicePropertyResult {
    type Vtable = IVideoDeviceControllerGetDevicePropertyResult_Vtbl;
    const IID: windows_core::GUID = <IVideoDeviceControllerGetDevicePropertyResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoDeviceControllerGetDevicePropertyResult {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult";
}
unsafe impl Send for VideoDeviceControllerGetDevicePropertyResult {}
unsafe impl Sync for VideoDeviceControllerGetDevicePropertyResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoTemporalDenoisingControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoTemporalDenoisingControl, windows_core::IUnknown, windows_core::IInspectable);
impl VideoTemporalDenoisingControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Mode(&self) -> windows_core::Result<VideoTemporalDenoisingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: VideoTemporalDenoisingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for VideoTemporalDenoisingControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoTemporalDenoisingControl>();
}
unsafe impl windows_core::Interface for VideoTemporalDenoisingControl {
    type Vtable = IVideoTemporalDenoisingControl_Vtbl;
    const IID: windows_core::GUID = <IVideoTemporalDenoisingControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoTemporalDenoisingControl {
    const NAME: &'static str = "Windows.Media.Devices.VideoTemporalDenoisingControl";
}
unsafe impl Send for VideoTemporalDenoisingControl {}
unsafe impl Sync for VideoTemporalDenoisingControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WhiteBalanceControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WhiteBalanceControl, windows_core::IUnknown, windows_core::IInspectable);
impl WhiteBalanceControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Preset(&self) -> windows_core::Result<ColorTemperaturePreset> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Preset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPresetAsync(&self, preset: ColorTemperaturePreset) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPresetAsync)(windows_core::Interface::as_raw(this), preset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Min(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValueAsync(&self, temperature: u32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetValueAsync)(windows_core::Interface::as_raw(this), temperature, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WhiteBalanceControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWhiteBalanceControl>();
}
unsafe impl windows_core::Interface for WhiteBalanceControl {
    type Vtable = IWhiteBalanceControl_Vtbl;
    const IID: windows_core::GUID = <IWhiteBalanceControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WhiteBalanceControl {
    const NAME: &'static str = "Windows.Media.Devices.WhiteBalanceControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ZoomControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ZoomControl, windows_core::IUnknown, windows_core::IInspectable);
impl ZoomControl {
    pub fn Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Min(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Min)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Max(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Max)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Step(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Step)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>> {
        let this = &windows_core::Interface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Mode(&self) -> windows_core::Result<ZoomTransitionMode> {
        let this = &windows_core::Interface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Configure<P0>(&self, settings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ZoomSettings>,
    {
        let this = &windows_core::Interface::cast::<IZoomControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Configure)(windows_core::Interface::as_raw(this), settings.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for ZoomControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IZoomControl>();
}
unsafe impl windows_core::Interface for ZoomControl {
    type Vtable = IZoomControl_Vtbl;
    const IID: windows_core::GUID = <IZoomControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ZoomControl {
    const NAME: &'static str = "Windows.Media.Devices.ZoomControl";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ZoomSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ZoomSettings, windows_core::IUnknown, windows_core::IInspectable);
impl ZoomSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ZoomSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> windows_core::Result<ZoomTransitionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: ZoomTransitionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ZoomSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IZoomSettings>();
}
unsafe impl windows_core::Interface for ZoomSettings {
    type Vtable = IZoomSettings_Vtbl;
    const IID: windows_core::GUID = <IZoomSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ZoomSettings {
    const NAME: &'static str = "Windows.Media.Devices.ZoomSettings";
}
unsafe impl Send for ZoomSettings {}
unsafe impl Sync for ZoomSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdvancedPhotoMode(pub i32);
impl AdvancedPhotoMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Hdr: Self = Self(2i32);
    pub const LowLight: Self = Self(3i32);
}
impl windows_core::TypeKind for AdvancedPhotoMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdvancedPhotoMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdvancedPhotoMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AdvancedPhotoMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AdvancedPhotoMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioDeviceRole(pub i32);
impl AudioDeviceRole {
    pub const Default: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
}
impl windows_core::TypeKind for AudioDeviceRole {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioDeviceRole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioDeviceRole").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioDeviceRole {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AudioDeviceRole;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: Self = Self(0i32);
    pub const Macro: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
}
impl windows_core::TypeKind for AutoFocusRange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutoFocusRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutoFocusRange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AutoFocusRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AutoFocusRange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: Self = Self(0i32);
    pub const CameraHardware: Self = Self(1i32);
}
impl windows_core::TypeKind for CameraOcclusionKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CameraOcclusionKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CameraOcclusionKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CameraOcclusionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraOcclusionKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: Self = Self(0i32);
    pub const Streaming: Self = Self(1i32);
    pub const BlockedForPrivacy: Self = Self(2i32);
    pub const Shutdown: Self = Self(3i32);
}
impl windows_core::TypeKind for CameraStreamState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CameraStreamState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CameraStreamState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CameraStreamState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraStreamState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CaptureSceneMode(pub i32);
impl CaptureSceneMode {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Macro: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const Sport: Self = Self(4i32);
    pub const Snow: Self = Self(5i32);
    pub const Night: Self = Self(6i32);
    pub const Beach: Self = Self(7i32);
    pub const Sunset: Self = Self(8i32);
    pub const Candlelight: Self = Self(9i32);
    pub const Landscape: Self = Self(10i32);
    pub const NightPortrait: Self = Self(11i32);
    pub const Backlit: Self = Self(12i32);
}
impl windows_core::TypeKind for CaptureSceneMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CaptureSceneMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CaptureSceneMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CaptureSceneMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureSceneMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl windows_core::TypeKind for CaptureUse {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CaptureUse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CaptureUse").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CaptureUse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureUse;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ColorTemperaturePreset(pub i32);
impl ColorTemperaturePreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Cloudy: Self = Self(2i32);
    pub const Daylight: Self = Self(3i32);
    pub const Flash: Self = Self(4i32);
    pub const Fluorescent: Self = Self(5i32);
    pub const Tungsten: Self = Self(6i32);
    pub const Candlelight: Self = Self(7i32);
}
impl windows_core::TypeKind for ColorTemperaturePreset {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ColorTemperaturePreset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ColorTemperaturePreset").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ColorTemperaturePreset {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ColorTemperaturePreset;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DigitalWindowMode(pub i32);
impl DigitalWindowMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl windows_core::TypeKind for DigitalWindowMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DigitalWindowMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DigitalWindowMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DigitalWindowMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.DigitalWindowMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FocusMode(pub i32);
impl FocusMode {
    pub const Auto: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Continuous: Self = Self(2i32);
    pub const Manual: Self = Self(3i32);
}
impl windows_core::TypeKind for FocusMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FocusMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FocusMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FocusMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FocusPreset(pub i32);
impl FocusPreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const AutoMacro: Self = Self(2i32);
    pub const AutoNormal: Self = Self(3i32);
    pub const AutoInfinity: Self = Self(4i32);
    pub const AutoHyperfocal: Self = Self(5i32);
}
impl windows_core::TypeKind for FocusPreset {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FocusPreset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FocusPreset").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FocusPreset {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusPreset;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl windows_core::TypeKind for HdrVideoMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HdrVideoMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HdrVideoMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HdrVideoMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.HdrVideoMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InfraredTorchMode(pub i32);
impl InfraredTorchMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const AlternatingFrameIllumination: Self = Self(2i32);
}
impl windows_core::TypeKind for InfraredTorchMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InfraredTorchMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InfraredTorchMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InfraredTorchMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.InfraredTorchMode;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IsoSpeedPreset(pub i32);
#[cfg(feature = "deprecated")]
impl IsoSpeedPreset {
    pub const Auto: Self = Self(0i32);
    pub const Iso50: Self = Self(1i32);
    pub const Iso80: Self = Self(2i32);
    pub const Iso100: Self = Self(3i32);
    pub const Iso200: Self = Self(4i32);
    pub const Iso400: Self = Self(5i32);
    pub const Iso800: Self = Self(6i32);
    pub const Iso1600: Self = Self(7i32);
    pub const Iso3200: Self = Self(8i32);
    pub const Iso6400: Self = Self(9i32);
    pub const Iso12800: Self = Self(10i32);
    pub const Iso25600: Self = Self(11i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for IsoSpeedPreset {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for IsoSpeedPreset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IsoSpeedPreset").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IsoSpeedPreset {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.IsoSpeedPreset;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ManualFocusDistance(pub i32);
impl ManualFocusDistance {
    pub const Infinity: Self = Self(0i32);
    pub const Hyperfocal: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl windows_core::TypeKind for ManualFocusDistance {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ManualFocusDistance {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ManualFocusDistance").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ManualFocusDistance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ManualFocusDistance;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCaptureFocusState(pub i32);
impl MediaCaptureFocusState {
    pub const Uninitialized: Self = Self(0i32);
    pub const Lost: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Focused: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl windows_core::TypeKind for MediaCaptureFocusState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCaptureFocusState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCaptureFocusState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCaptureFocusState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureFocusState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCaptureOptimization(pub i32);
impl MediaCaptureOptimization {
    pub const Default: Self = Self(0i32);
    pub const Quality: Self = Self(1i32);
    pub const Latency: Self = Self(2i32);
    pub const Power: Self = Self(3i32);
    pub const LatencyThenQuality: Self = Self(4i32);
    pub const LatencyThenPower: Self = Self(5i32);
    pub const PowerAndQuality: Self = Self(6i32);
}
impl windows_core::TypeKind for MediaCaptureOptimization {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCaptureOptimization {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCaptureOptimization").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCaptureOptimization {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureOptimization;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: Self = Self(0i32);
    pub const ReleaseHardwareResources: Self = Self(1i32);
}
impl windows_core::TypeKind for MediaCapturePauseBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaCapturePauseBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaCapturePauseBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaCapturePauseBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCapturePauseBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OpticalImageStabilizationMode(pub i32);
impl OpticalImageStabilizationMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl windows_core::TypeKind for OpticalImageStabilizationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OpticalImageStabilizationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OpticalImageStabilizationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for OpticalImageStabilizationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.OpticalImageStabilizationMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RegionOfInterestType(pub i32);
impl RegionOfInterestType {
    pub const Unknown: Self = Self(0i32);
    pub const Face: Self = Self(1i32);
}
impl windows_core::TypeKind for RegionOfInterestType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RegionOfInterestType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RegionOfInterestType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RegionOfInterestType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.RegionOfInterestType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
}
impl windows_core::TypeKind for SendCommandStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SendCommandStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SendCommandStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SendCommandStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.SendCommandStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TelephonyKey(pub i32);
impl TelephonyKey {
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
    pub const A: Self = Self(12i32);
    pub const B: Self = Self(13i32);
    pub const C: Self = Self(14i32);
    pub const D: Self = Self(15i32);
}
impl windows_core::TypeKind for TelephonyKey {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TelephonyKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TelephonyKey").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TelephonyKey {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.TelephonyKey;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VideoDeviceControllerGetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerGetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const BufferTooSmall: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(5i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(6i32);
}
impl windows_core::TypeKind for VideoDeviceControllerGetDevicePropertyStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VideoDeviceControllerGetDevicePropertyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerGetDevicePropertyStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VideoDeviceControllerGetDevicePropertyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VideoDeviceControllerSetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerSetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl windows_core::TypeKind for VideoDeviceControllerSetDevicePropertyStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VideoDeviceControllerSetDevicePropertyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerSetDevicePropertyStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VideoDeviceControllerSetDevicePropertyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerSetDevicePropertyStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl windows_core::TypeKind for VideoTemporalDenoisingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VideoTemporalDenoisingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VideoTemporalDenoisingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VideoTemporalDenoisingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoTemporalDenoisingMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ZoomTransitionMode(pub i32);
impl ZoomTransitionMode {
    pub const Auto: Self = Self(0i32);
    pub const Direct: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
impl windows_core::TypeKind for ZoomTransitionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ZoomTransitionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ZoomTransitionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ZoomTransitionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ZoomTransitionMode;i4)");
}
windows_core::imp::define_interface!(CallControlEventHandler, CallControlEventHandler_Vtbl, 0x596f759f_50df_4454_bc63_4d3d01b61958);
impl CallControlEventHandler {
    pub fn new<F: FnMut(Option<&CallControl>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = CallControlEventHandlerBox::<F> { vtable: &CallControlEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CallControl>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
struct CallControlEventHandlerBox<F: FnMut(Option<&CallControl>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const CallControlEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&CallControl>) -> windows_core::Result<()> + Send + 'static> CallControlEventHandlerBox<F> {
    const VTABLE: CallControlEventHandler_Vtbl = CallControlEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <CallControlEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
impl windows_core::RuntimeType for CallControlEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct CallControlEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(DialRequestedEventHandler, DialRequestedEventHandler_Vtbl, 0x5abbffdb_c21f_4bc4_891b_257e28c1b1a4);
impl DialRequestedEventHandler {
    pub fn new<F: FnMut(Option<&CallControl>, Option<&DialRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = DialRequestedEventHandlerBox::<F> { vtable: &DialRequestedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CallControl>,
        P1: windows_core::Param<DialRequestedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
struct DialRequestedEventHandlerBox<F: FnMut(Option<&CallControl>, Option<&DialRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DialRequestedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&CallControl>, Option<&DialRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> DialRequestedEventHandlerBox<F> {
    const VTABLE: DialRequestedEventHandler_Vtbl = DialRequestedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DialRequestedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&e)).into()
    }
}
impl windows_core::RuntimeType for DialRequestedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DialRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(KeypadPressedEventHandler, KeypadPressedEventHandler_Vtbl, 0xe637a454_c527_422c_8926_c9af83b559a0);
impl KeypadPressedEventHandler {
    pub fn new<F: FnMut(Option<&CallControl>, Option<&KeypadPressedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = KeypadPressedEventHandlerBox::<F> { vtable: &KeypadPressedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CallControl>,
        P1: windows_core::Param<KeypadPressedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
struct KeypadPressedEventHandlerBox<F: FnMut(Option<&CallControl>, Option<&KeypadPressedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const KeypadPressedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&CallControl>, Option<&KeypadPressedEventArgs>) -> windows_core::Result<()> + Send + 'static> KeypadPressedEventHandlerBox<F> {
    const VTABLE: KeypadPressedEventHandler_Vtbl = KeypadPressedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <KeypadPressedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&e)).into()
    }
}
impl windows_core::RuntimeType for KeypadPressedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct KeypadPressedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(RedialRequestedEventHandler, RedialRequestedEventHandler_Vtbl, 0xbaf257d1_4ebd_4b84_9f47_6ec43d75d8b1);
impl RedialRequestedEventHandler {
    pub fn new<F: FnMut(Option<&CallControl>, Option<&RedialRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = RedialRequestedEventHandlerBox::<F> { vtable: &RedialRequestedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CallControl>,
        P1: windows_core::Param<RedialRequestedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
struct RedialRequestedEventHandlerBox<F: FnMut(Option<&CallControl>, Option<&RedialRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const RedialRequestedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&CallControl>, Option<&RedialRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> RedialRequestedEventHandlerBox<F> {
    const VTABLE: RedialRequestedEventHandler_Vtbl = RedialRequestedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <RedialRequestedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&e)).into()
    }
}
impl windows_core::RuntimeType for RedialRequestedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct RedialRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
