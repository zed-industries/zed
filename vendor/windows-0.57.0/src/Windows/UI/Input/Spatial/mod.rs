windows_core::imp::define_interface!(ISpatialGestureRecognizer, ISpatialGestureRecognizer_Vtbl, 0x71605bcc_0c35_4673_adbd_cc04caa6ef45);
impl windows_core::RuntimeType for ISpatialGestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialGestureRecognizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecognitionStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRecognitionStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RecognitionEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRecognitionEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Tapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTapped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub HoldStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHoldStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub HoldCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHoldCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub HoldCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHoldCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NavigationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNavigationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NavigationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNavigationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NavigationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNavigationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NavigationCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNavigationCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CaptureInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelPendingGestures: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySetGestureSettings: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialGestureSettings, *mut bool) -> windows_core::HRESULT,
    pub GestureSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialGestureSettings) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialGestureRecognizerFactory, ISpatialGestureRecognizerFactory_Vtbl, 0x77214186_57b9_3150_8382_698b24e264d0);
impl windows_core::RuntimeType for ISpatialGestureRecognizerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialGestureRecognizerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialGestureSettings, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialHoldCanceledEventArgs, ISpatialHoldCanceledEventArgs_Vtbl, 0x5dfcb667_4caa_4093_8c35_b601a839f31b);
impl windows_core::RuntimeType for ISpatialHoldCanceledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialHoldCanceledEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialHoldCompletedEventArgs, ISpatialHoldCompletedEventArgs_Vtbl, 0x3f64470b_4cfd_43da_8dc4_e64552173971);
impl windows_core::RuntimeType for ISpatialHoldCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialHoldCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialHoldStartedEventArgs, ISpatialHoldStartedEventArgs_Vtbl, 0x8e343d79_acb6_4144_8615_2cfba8a3cb3f);
impl windows_core::RuntimeType for ISpatialHoldStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialHoldStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
windows_core::imp::define_interface!(ISpatialInteraction, ISpatialInteraction_Vtbl, 0xfc967639_88e6_4646_9112_4344aaec9dfa);
impl windows_core::RuntimeType for ISpatialInteraction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteraction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionController, ISpatialInteractionController_Vtbl, 0x5f0e5ba3_0954_4e97_86c5_e7f30b114dfd);
impl windows_core::RuntimeType for ISpatialInteractionController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasTouchpad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasThumbstick: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionController2, ISpatialInteractionController2_Vtbl, 0x35b6d924_c7a2_49b7_b72e_5436b2fb8f9c);
impl windows_core::RuntimeType for ISpatialInteractionController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub TryGetRenderableModelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryGetRenderableModelAsync: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionController3, ISpatialInteractionController3_Vtbl, 0x628466a0_9d91_4a0b_888d_165e670a8cd5);
impl windows_core::RuntimeType for ISpatialInteractionController3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionController3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Power")]
    pub TryGetBatteryReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))]
    TryGetBatteryReport: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionControllerProperties, ISpatialInteractionControllerProperties_Vtbl, 0x61056fb1_7ba9_4e35_b93f_9272cba9b28b);
impl windows_core::RuntimeType for ISpatialInteractionControllerProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionControllerProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTouchpadTouched: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTouchpadPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsThumbstickPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ThumbstickX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ThumbstickY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TouchpadX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TouchpadY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionDetectedEventArgs, ISpatialInteractionDetectedEventArgs_Vtbl, 0x075878e4_5961_3b41_9dfb_cea5d89cc38a);
impl windows_core::RuntimeType for ISpatialInteractionDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionDetectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub Interaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionDetectedEventArgs2, ISpatialInteractionDetectedEventArgs2_Vtbl, 0x7b263e93_5f13_419c_97d5_834678266aa6);
impl windows_core::RuntimeType for ISpatialInteractionDetectedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionDetectedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionManager, ISpatialInteractionManager_Vtbl, 0x32a64ea8_a15a_3995_b8bd_80513cb5adef);
impl windows_core::RuntimeType for ISpatialInteractionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourceDetected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SourceLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourceLost: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SourceUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourceUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SourcePressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourcePressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SourceReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourceReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InteractionDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInteractionDetected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))]
    pub GetDetectedSourcesAtTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Perception")))]
    GetDetectedSourcesAtTimestamp: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionManagerStatics, ISpatialInteractionManagerStatics_Vtbl, 0x00e31fa6_8ca2_30bf_91fe_d9cb4a008990);
impl windows_core::RuntimeType for ISpatialInteractionManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionManagerStatics2, ISpatialInteractionManagerStatics2_Vtbl, 0x93f16c52_b88a_5929_8d7c_48cb948b081c);
impl windows_core::RuntimeType for ISpatialInteractionManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSourceKindSupported: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialInteractionSourceKind, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionSource, ISpatialInteractionSource_Vtbl, 0xfb5433ba_b0b3_3148_9f3b_e9f5de568f5d);
impl windows_core::RuntimeType for ISpatialInteractionSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionSource2, ISpatialInteractionSource2_Vtbl, 0xe4c5b70c_0470_4028_88c0_a0eb44d34efe);
impl windows_core::RuntimeType for ISpatialInteractionSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPointingSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMenuSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGraspSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Controller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Perception")]
    pub TryGetStateAtTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    TryGetStateAtTimestamp: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionSource3, ISpatialInteractionSource3_Vtbl, 0x0406d9f9_9afd_44f9_85dc_700023a962e3);
impl windows_core::RuntimeType for ISpatialInteractionSource3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSource3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handedness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceHandedness) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionSource4, ISpatialInteractionSource4_Vtbl, 0x0073bc4d_df66_5a91_a2ba_cea3e5c58a19);
impl windows_core::RuntimeType for ISpatialInteractionSource4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSource4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Perception_People")]
    pub TryCreateHandMeshObserver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryCreateHandMeshObserver: usize,
    #[cfg(feature = "Perception_People")]
    pub TryCreateHandMeshObserverAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryCreateHandMeshObserverAsync: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceEventArgs, ISpatialInteractionSourceEventArgs_Vtbl, 0x23b786cf_ec23_3979_b27c_eb0e12feb7c7);
impl windows_core::RuntimeType for ISpatialInteractionSourceEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceEventArgs2, ISpatialInteractionSourceEventArgs2_Vtbl, 0xd8b4b467_e648_4d52_ab49_e0d227199f63);
impl windows_core::RuntimeType for ISpatialInteractionSourceEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PressKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionPressKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceLocation, ISpatialInteractionSourceLocation_Vtbl, 0xea4696c4_7e8b_30ca_bcc5_c77189cea30a);
impl windows_core::RuntimeType for ISpatialInteractionSourceLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceLocation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Velocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Velocity: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceLocation2, ISpatialInteractionSourceLocation2_Vtbl, 0x4c671045_3917_40fc_a9ac_31c9cf5ff91b);
impl windows_core::RuntimeType for ISpatialInteractionSourceLocation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceLocation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceLocation3, ISpatialInteractionSourceLocation3_Vtbl, 0x6702e65e_e915_4cfb_9c1b_0538efc86687);
impl windows_core::RuntimeType for ISpatialInteractionSourceLocation3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceLocation3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PositionAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourcePositionAccuracy) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub AngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AngularVelocity: usize,
    pub SourcePointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceProperties, ISpatialInteractionSourceProperties_Vtbl, 0x05604542_3ef7_3222_9f53_63c9cb7e3bc7);
impl windows_core::RuntimeType for ISpatialInteractionSourceProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub TryGetSourceLossMitigationDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    TryGetSourceLossMitigationDirection: usize,
    pub SourceLossRisk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetLocation: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceState, ISpatialInteractionSourceState_Vtbl, 0xd5c475ef_4b63_37ec_98b9_9fc652b9d2f2);
impl windows_core::RuntimeType for ISpatialInteractionSourceState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceState2, ISpatialInteractionSourceState2_Vtbl, 0x45f6d0bd_1773_492e_9ba3_8ac1cbe77c08);
impl windows_core::RuntimeType for ISpatialInteractionSourceState2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceState2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSelectPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMenuPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGrasped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SelectPressedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ControllerProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionSourceState3, ISpatialInteractionSourceState3_Vtbl, 0xf2f00bc2_bd2b_4a01_a8fb_323e0158527c);
impl windows_core::RuntimeType for ISpatialInteractionSourceState3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialInteractionSourceState3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Perception_People")]
    pub TryGetHandPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    TryGetHandPose: usize,
}
windows_core::imp::define_interface!(ISpatialManipulationCanceledEventArgs, ISpatialManipulationCanceledEventArgs_Vtbl, 0x2d40d1cb_e7da_4220_b0bf_819301674780);
impl windows_core::RuntimeType for ISpatialManipulationCanceledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialManipulationCanceledEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialManipulationCompletedEventArgs, ISpatialManipulationCompletedEventArgs_Vtbl, 0x05086802_f301_4343_9250_2fbaa5f87a37);
impl windows_core::RuntimeType for ISpatialManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialManipulationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetCumulativeDelta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetCumulativeDelta: usize,
}
windows_core::imp::define_interface!(ISpatialManipulationDelta, ISpatialManipulationDelta_Vtbl, 0xa7ec967a_d123_3a81_a15b_992923dcbe91);
impl windows_core::RuntimeType for ISpatialManipulationDelta {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialManipulationDelta_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
}
windows_core::imp::define_interface!(ISpatialManipulationStartedEventArgs, ISpatialManipulationStartedEventArgs_Vtbl, 0xa1d6bbce_42a5_377b_ada6_d28e3d384737);
impl windows_core::RuntimeType for ISpatialManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialManipulationStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
}
windows_core::imp::define_interface!(ISpatialManipulationUpdatedEventArgs, ISpatialManipulationUpdatedEventArgs_Vtbl, 0x5f230b9b_60c6_4dc6_bdc9_9f4a6f15fe49);
impl windows_core::RuntimeType for ISpatialManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialManipulationUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetCumulativeDelta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetCumulativeDelta: usize,
}
windows_core::imp::define_interface!(ISpatialNavigationCanceledEventArgs, ISpatialNavigationCanceledEventArgs_Vtbl, 0xce503edc_e8a5_46f0_92d4_3c122b35112a);
impl windows_core::RuntimeType for ISpatialNavigationCanceledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialNavigationCanceledEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialNavigationCompletedEventArgs, ISpatialNavigationCompletedEventArgs_Vtbl, 0x012e80b7_af3b_42c2_9e41_baaa0e721f3a);
impl windows_core::RuntimeType for ISpatialNavigationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialNavigationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NormalizedOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NormalizedOffset: usize,
}
windows_core::imp::define_interface!(ISpatialNavigationStartedEventArgs, ISpatialNavigationStartedEventArgs_Vtbl, 0x754a348a_fb64_4656_8ebd_9deecaafe475);
impl windows_core::RuntimeType for ISpatialNavigationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialNavigationStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub IsNavigatingX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsNavigatingY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsNavigatingZ: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialNavigationUpdatedEventArgs, ISpatialNavigationUpdatedEventArgs_Vtbl, 0x9b713fd7_839d_4a74_8732_45466fc044b5);
impl windows_core::RuntimeType for ISpatialNavigationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialNavigationUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NormalizedOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NormalizedOffset: usize,
}
windows_core::imp::define_interface!(ISpatialPointerInteractionSourcePose, ISpatialPointerInteractionSourcePose_Vtbl, 0xa7104307_2c2b_4d3a_92a7_80ced7c4a0d0);
impl windows_core::RuntimeType for ISpatialPointerInteractionSourcePose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialPointerInteractionSourcePose_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ForwardDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ForwardDirection: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UpDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UpDirection: usize,
}
windows_core::imp::define_interface!(ISpatialPointerInteractionSourcePose2, ISpatialPointerInteractionSourcePose2_Vtbl, 0xeccd86b8_52db_469f_9e3f_80c47f74bce9);
impl windows_core::RuntimeType for ISpatialPointerInteractionSourcePose2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialPointerInteractionSourcePose2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    pub PositionAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourcePositionAccuracy) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialPointerPose, ISpatialPointerPose_Vtbl, 0x6953a42e_c17e_357d_97a1_7269d0ed2d10);
impl windows_core::RuntimeType for ISpatialPointerPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialPointerPose_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Perception")]
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception"))]
    Timestamp: usize,
    #[cfg(feature = "Perception_People")]
    pub Head: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    Head: usize,
}
windows_core::imp::define_interface!(ISpatialPointerPose2, ISpatialPointerPose2_Vtbl, 0x9d202b17_954e_4e0c_96d1_b6790b6fc2fd);
impl windows_core::RuntimeType for ISpatialPointerPose2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialPointerPose2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetInteractionSourcePose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialPointerPose3, ISpatialPointerPose3_Vtbl, 0x6342f3f0_ec49_5b4b_b8d1_d16cbb16be84);
impl windows_core::RuntimeType for ISpatialPointerPose3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialPointerPose3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Perception_People")]
    pub Eyes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_People"))]
    Eyes: usize,
    pub IsHeadCapturedBySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialPointerPoseStatics, ISpatialPointerPoseStatics_Vtbl, 0xa25591a9_aca1_3ee0_9816_785cfb2e3fb8);
impl windows_core::RuntimeType for ISpatialPointerPoseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialPointerPoseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetAtTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetAtTimestamp: usize,
}
windows_core::imp::define_interface!(ISpatialRecognitionEndedEventArgs, ISpatialRecognitionEndedEventArgs_Vtbl, 0x0e35f5cb_3f75_43f3_ac81_d1dc2df9b1fb);
impl windows_core::RuntimeType for ISpatialRecognitionEndedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialRecognitionEndedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialRecognitionStartedEventArgs, ISpatialRecognitionStartedEventArgs_Vtbl, 0x24da128f_0008_4a6d_aa50_2a76f9cfb264);
impl windows_core::RuntimeType for ISpatialRecognitionStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialRecognitionStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub IsGesturePossible: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialGestureSettings, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialTappedEventArgs, ISpatialTappedEventArgs_Vtbl, 0x296d83de_f444_4aa1_b2bf_9dc88d567da6);
impl windows_core::RuntimeType for ISpatialTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialTappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSourceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialInteractionSourceKind) -> windows_core::HRESULT,
    #[cfg(feature = "Perception_Spatial")]
    pub TryGetPointerPose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))]
    TryGetPointerPose: usize,
    pub TapCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialGestureRecognizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialGestureRecognizer, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialGestureRecognizer {
    pub fn RecognitionStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognitionStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRecognitionStarted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRecognitionStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RecognitionEnded<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognitionEnded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRecognitionEnded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRecognitionEnded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Tapped<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTapped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTapped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HoldStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHoldStarted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHoldStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HoldCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHoldCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHoldCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HoldCanceled<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldCanceled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHoldCanceled(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHoldCanceled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationStarted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationCanceled<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCanceled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationCanceled(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationCanceled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NavigationStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationStarted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NavigationUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NavigationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NavigationCanceled<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NavigationCanceled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNavigationCanceled(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNavigationCanceled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CaptureInteraction<P0>(&self, interaction: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SpatialInteraction>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CaptureInteraction)(windows_core::Interface::as_raw(this), interaction.param().abi()).ok() }
    }
    pub fn CancelPendingGestures(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CancelPendingGestures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TrySetGestureSettings(&self, settings: SpatialGestureSettings) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetGestureSettings)(windows_core::Interface::as_raw(this), settings, &mut result__).map(|| result__)
        }
    }
    pub fn GestureSettings(&self) -> windows_core::Result<SpatialGestureSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GestureSettings)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(settings: SpatialGestureSettings) -> windows_core::Result<SpatialGestureRecognizer> {
        Self::ISpatialGestureRecognizerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), settings, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialGestureRecognizerFactory<R, F: FnOnce(&ISpatialGestureRecognizerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialGestureRecognizer, ISpatialGestureRecognizerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialGestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialGestureRecognizer>();
}
unsafe impl windows_core::Interface for SpatialGestureRecognizer {
    type Vtable = ISpatialGestureRecognizer_Vtbl;
    const IID: windows_core::GUID = <ISpatialGestureRecognizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialGestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialGestureRecognizer";
}
unsafe impl Send for SpatialGestureRecognizer {}
unsafe impl Sync for SpatialGestureRecognizer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialHoldCanceledEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialHoldCanceledEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialHoldCanceledEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialHoldCanceledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialHoldCanceledEventArgs>();
}
unsafe impl windows_core::Interface for SpatialHoldCanceledEventArgs {
    type Vtable = ISpatialHoldCanceledEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialHoldCanceledEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialHoldCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCanceledEventArgs";
}
unsafe impl Send for SpatialHoldCanceledEventArgs {}
unsafe impl Sync for SpatialHoldCanceledEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialHoldCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialHoldCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialHoldCompletedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialHoldCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialHoldCompletedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialHoldCompletedEventArgs {
    type Vtable = ISpatialHoldCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialHoldCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialHoldCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldCompletedEventArgs";
}
unsafe impl Send for SpatialHoldCompletedEventArgs {}
unsafe impl Sync for SpatialHoldCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialHoldStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialHoldStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialHoldStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPointerPose)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialHoldStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialHoldStartedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialHoldStartedEventArgs {
    type Vtable = ISpatialHoldStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialHoldStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialHoldStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialHoldStartedEventArgs";
}
unsafe impl Send for SpatialHoldStartedEventArgs {}
unsafe impl Sync for SpatialHoldStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteraction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteraction, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteraction {
    pub fn SourceState(&self) -> windows_core::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialInteraction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteraction>();
}
unsafe impl windows_core::Interface for SpatialInteraction {
    type Vtable = ISpatialInteraction_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteraction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteraction {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteraction";
}
unsafe impl Send for SpatialInteraction {}
unsafe impl Sync for SpatialInteraction {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionController, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionController {
    pub fn HasTouchpad(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasTouchpad)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasThumbstick(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasThumbstick)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VendorId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProductId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Version(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryGetRenderableModelAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetRenderableModelAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionController3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionController>();
}
unsafe impl windows_core::Interface for SpatialInteractionController {
    type Vtable = ISpatialInteractionController_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionController {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionController";
}
unsafe impl Send for SpatialInteractionController {}
unsafe impl Sync for SpatialInteractionController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionControllerProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionControllerProperties, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionControllerProperties {
    pub fn IsTouchpadTouched(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTouchpadTouched)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTouchpadPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTouchpadPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsThumbstickPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsThumbstickPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ThumbstickX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbstickX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ThumbstickY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThumbstickY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TouchpadX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchpadX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TouchpadY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchpadY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionControllerProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionControllerProperties>();
}
unsafe impl windows_core::Interface for SpatialInteractionControllerProperties {
    type Vtable = ISpatialInteractionControllerProperties_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionControllerProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionControllerProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionControllerProperties";
}
unsafe impl Send for SpatialInteractionControllerProperties {}
unsafe impl Sync for SpatialInteractionControllerProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionDetectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionDetectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionDetectedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPointerPose)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Interaction(&self) -> windows_core::Result<SpatialInteraction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Interaction)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InteractionSource(&self) -> windows_core::Result<SpatialInteractionSource> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionDetectedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionDetectedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialInteractionDetectedEventArgs {
    type Vtable = ISpatialInteractionDetectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionDetectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionDetectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionDetectedEventArgs";
}
unsafe impl Send for SpatialInteractionDetectedEventArgs {}
unsafe impl Sync for SpatialInteractionDetectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionManager, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionManager {
    pub fn SourceDetected<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceDetected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceLost(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceLost)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourcePressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourcePressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourcePressed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourcePressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceReleased(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceReleased)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn InteractionDetected<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInteractionDetected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInteractionDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Perception"))]
    pub fn GetDetectedSourcesAtTimestamp<P0>(&self, timestamp: P0) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>>
    where
        P0: windows_core::Param<super::super::super::Perception::PerceptionTimestamp>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDetectedSourcesAtTimestamp)(windows_core::Interface::as_raw(this), timestamp.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<SpatialInteractionManager> {
        Self::ISpatialInteractionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSourceKindSupported(kind: SpatialInteractionSourceKind) -> windows_core::Result<bool> {
        Self::ISpatialInteractionManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSourceKindSupported)(windows_core::Interface::as_raw(this), kind, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialInteractionManagerStatics<R, F: FnOnce(&ISpatialInteractionManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialInteractionManager, ISpatialInteractionManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialInteractionManagerStatics2<R, F: FnOnce(&ISpatialInteractionManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialInteractionManager, ISpatialInteractionManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialInteractionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionManager>();
}
unsafe impl windows_core::Interface for SpatialInteractionManager {
    type Vtable = ISpatialInteractionManager_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionManager {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionManager";
}
unsafe impl Send for SpatialInteractionManager {}
unsafe impl Sync for SpatialInteractionManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionSource, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionSource {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPointingSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPointingSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMenuSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMenuSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGraspSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGraspSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Controller(&self) -> windows_core::Result<SpatialInteractionController> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Controller)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception")]
    pub fn TryGetStateAtTimestamp<P0>(&self, timestamp: P0) -> windows_core::Result<SpatialInteractionSourceState>
    where
        P0: windows_core::Param<super::super::super::Perception::PerceptionTimestamp>,
    {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetStateAtTimestamp)(windows_core::Interface::as_raw(this), timestamp.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Handedness(&self) -> windows_core::Result<SpatialInteractionSourceHandedness> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handedness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn TryCreateHandMeshObserver(&self) -> windows_core::Result<super::super::super::Perception::People::HandMeshObserver> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateHandMeshObserver)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn TryCreateHandMeshObserverAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSource4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateHandMeshObserverAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionSource>();
}
unsafe impl windows_core::Interface for SpatialInteractionSource {
    type Vtable = ISpatialInteractionSource_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionSource {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSource";
}
unsafe impl Send for SpatialInteractionSource {}
unsafe impl Sync for SpatialInteractionSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionSourceEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionSourceEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionSourceEventArgs {
    pub fn State(&self) -> windows_core::Result<SpatialInteractionSourceState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PressKind(&self) -> windows_core::Result<SpatialInteractionPressKind> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PressKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionSourceEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionSourceEventArgs>();
}
unsafe impl windows_core::Interface for SpatialInteractionSourceEventArgs {
    type Vtable = ISpatialInteractionSourceEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionSourceEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionSourceEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceEventArgs";
}
unsafe impl Send for SpatialInteractionSourceEventArgs {}
unsafe impl Sync for SpatialInteractionSourceEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionSourceLocation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionSourceLocation, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionSourceLocation {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Velocity(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Velocity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceLocation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PositionAccuracy(&self) -> windows_core::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AngularVelocity(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AngularVelocity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourcePointerPose(&self) -> windows_core::Result<SpatialPointerInteractionSourcePose> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceLocation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourcePointerPose)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionSourceLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionSourceLocation>();
}
unsafe impl windows_core::Interface for SpatialInteractionSourceLocation {
    type Vtable = ISpatialInteractionSourceLocation_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionSourceLocation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionSourceLocation {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceLocation";
}
unsafe impl Send for SpatialInteractionSourceLocation {}
unsafe impl Sync for SpatialInteractionSourceLocation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionSourceProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionSourceProperties, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionSourceProperties {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn TryGetSourceLossMitigationDirection<P0>(&self, coordinatesystem: P0) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetSourceLossMitigationDirection)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceLossRisk(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceLossRisk)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetLocation<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialInteractionSourceLocation>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetLocation)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionSourceProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionSourceProperties>();
}
unsafe impl windows_core::Interface for SpatialInteractionSourceProperties {
    type Vtable = ISpatialInteractionSourceProperties_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionSourceProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionSourceProperties {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceProperties";
}
unsafe impl Send for SpatialInteractionSourceProperties {}
unsafe impl Sync for SpatialInteractionSourceProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialInteractionSourceState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialInteractionSourceState, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialInteractionSourceState {
    pub fn Source(&self) -> windows_core::Result<SpatialInteractionSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<SpatialInteractionSourceProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception")]
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPointerPose)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSelectPressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSelectPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMenuPressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMenuPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGrasped(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrasped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelectPressedValue(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectPressedValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ControllerProperties(&self) -> windows_core::Result<SpatialInteractionControllerProperties> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceState2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControllerProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn TryGetHandPose(&self) -> windows_core::Result<super::super::super::Perception::People::HandPose> {
        let this = &windows_core::Interface::cast::<ISpatialInteractionSourceState3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetHandPose)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialInteractionSourceState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialInteractionSourceState>();
}
unsafe impl windows_core::Interface for SpatialInteractionSourceState {
    type Vtable = ISpatialInteractionSourceState_Vtbl;
    const IID: windows_core::GUID = <ISpatialInteractionSourceState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialInteractionSourceState {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialInteractionSourceState";
}
unsafe impl Send for SpatialInteractionSourceState {}
unsafe impl Sync for SpatialInteractionSourceState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialManipulationCanceledEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialManipulationCanceledEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialManipulationCanceledEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialManipulationCanceledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialManipulationCanceledEventArgs>();
}
unsafe impl windows_core::Interface for SpatialManipulationCanceledEventArgs {
    type Vtable = ISpatialManipulationCanceledEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialManipulationCanceledEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialManipulationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCanceledEventArgs";
}
unsafe impl Send for SpatialManipulationCanceledEventArgs {}
unsafe impl Sync for SpatialManipulationCanceledEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialManipulationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialManipulationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialManipulationCompletedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetCumulativeDelta<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialManipulationDelta>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetCumulativeDelta)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialManipulationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialManipulationCompletedEventArgs {
    type Vtable = ISpatialManipulationCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialManipulationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationCompletedEventArgs";
}
unsafe impl Send for SpatialManipulationCompletedEventArgs {}
unsafe impl Sync for SpatialManipulationCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialManipulationDelta(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialManipulationDelta, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialManipulationDelta {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Translation(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialManipulationDelta {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialManipulationDelta>();
}
unsafe impl windows_core::Interface for SpatialManipulationDelta {
    type Vtable = ISpatialManipulationDelta_Vtbl;
    const IID: windows_core::GUID = <ISpatialManipulationDelta as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialManipulationDelta {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationDelta";
}
unsafe impl Send for SpatialManipulationDelta {}
unsafe impl Sync for SpatialManipulationDelta {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialManipulationStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialManipulationStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialManipulationStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPointerPose)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialManipulationStartedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialManipulationStartedEventArgs {
    type Vtable = ISpatialManipulationStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialManipulationStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationStartedEventArgs";
}
unsafe impl Send for SpatialManipulationStartedEventArgs {}
unsafe impl Sync for SpatialManipulationStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialManipulationUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialManipulationUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialManipulationUpdatedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetCumulativeDelta<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialManipulationDelta>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetCumulativeDelta)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialManipulationUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialManipulationUpdatedEventArgs {
    type Vtable = ISpatialManipulationUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialManipulationUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialManipulationUpdatedEventArgs";
}
unsafe impl Send for SpatialManipulationUpdatedEventArgs {}
unsafe impl Sync for SpatialManipulationUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialNavigationCanceledEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialNavigationCanceledEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialNavigationCanceledEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialNavigationCanceledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialNavigationCanceledEventArgs>();
}
unsafe impl windows_core::Interface for SpatialNavigationCanceledEventArgs {
    type Vtable = ISpatialNavigationCanceledEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialNavigationCanceledEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialNavigationCanceledEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCanceledEventArgs";
}
unsafe impl Send for SpatialNavigationCanceledEventArgs {}
unsafe impl Sync for SpatialNavigationCanceledEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialNavigationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialNavigationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialNavigationCompletedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NormalizedOffset(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizedOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialNavigationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialNavigationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialNavigationCompletedEventArgs {
    type Vtable = ISpatialNavigationCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialNavigationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationCompletedEventArgs";
}
unsafe impl Send for SpatialNavigationCompletedEventArgs {}
unsafe impl Sync for SpatialNavigationCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialNavigationStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialNavigationStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialNavigationStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPointerPose)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsNavigatingX(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNavigatingX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsNavigatingY(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNavigatingY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsNavigatingZ(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNavigatingZ)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialNavigationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialNavigationStartedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialNavigationStartedEventArgs {
    type Vtable = ISpatialNavigationStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialNavigationStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialNavigationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationStartedEventArgs";
}
unsafe impl Send for SpatialNavigationStartedEventArgs {}
unsafe impl Sync for SpatialNavigationStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialNavigationUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialNavigationUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialNavigationUpdatedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NormalizedOffset(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizedOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialNavigationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialNavigationUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialNavigationUpdatedEventArgs {
    type Vtable = ISpatialNavigationUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialNavigationUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialNavigationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialNavigationUpdatedEventArgs";
}
unsafe impl Send for SpatialNavigationUpdatedEventArgs {}
unsafe impl Sync for SpatialNavigationUpdatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialPointerInteractionSourcePose(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialPointerInteractionSourcePose, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialPointerInteractionSourcePose {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ForwardDirection(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForwardDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UpDirection(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = &windows_core::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PositionAccuracy(&self) -> windows_core::Result<SpatialInteractionSourcePositionAccuracy> {
        let this = &windows_core::Interface::cast::<ISpatialPointerInteractionSourcePose2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionAccuracy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialPointerInteractionSourcePose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialPointerInteractionSourcePose>();
}
unsafe impl windows_core::Interface for SpatialPointerInteractionSourcePose {
    type Vtable = ISpatialPointerInteractionSourcePose_Vtbl;
    const IID: windows_core::GUID = <ISpatialPointerInteractionSourcePose as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialPointerInteractionSourcePose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerInteractionSourcePose";
}
unsafe impl Send for SpatialPointerInteractionSourcePose {}
unsafe impl Sync for SpatialPointerInteractionSourcePose {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialPointerPose(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialPointerPose, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialPointerPose {
    #[cfg(feature = "Perception")]
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn Head(&self) -> windows_core::Result<super::super::super::Perception::People::HeadPose> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Head)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetInteractionSourcePose<P0>(&self, source: P0) -> windows_core::Result<SpatialPointerInteractionSourcePose>
    where
        P0: windows_core::Param<SpatialInteractionSource>,
    {
        let this = &windows_core::Interface::cast::<ISpatialPointerPose2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetInteractionSourcePose)(windows_core::Interface::as_raw(this), source.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Perception_People")]
    pub fn Eyes(&self) -> windows_core::Result<super::super::super::Perception::People::EyesPose> {
        let this = &windows_core::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Eyes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsHeadCapturedBySystem(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISpatialPointerPose3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHeadCapturedBySystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetAtTimestamp<P0, P1>(coordinatesystem: P0, timestamp: P1) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
        P1: windows_core::Param<super::super::super::Perception::PerceptionTimestamp>,
    {
        Self::ISpatialPointerPoseStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAtTimestamp)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), timestamp.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialPointerPoseStatics<R, F: FnOnce(&ISpatialPointerPoseStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialPointerPose, ISpatialPointerPoseStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialPointerPose {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialPointerPose>();
}
unsafe impl windows_core::Interface for SpatialPointerPose {
    type Vtable = ISpatialPointerPose_Vtbl;
    const IID: windows_core::GUID = <ISpatialPointerPose as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialPointerPose {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialPointerPose";
}
unsafe impl Send for SpatialPointerPose {}
unsafe impl Sync for SpatialPointerPose {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialRecognitionEndedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialRecognitionEndedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialRecognitionEndedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialRecognitionEndedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialRecognitionEndedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialRecognitionEndedEventArgs {
    type Vtable = ISpatialRecognitionEndedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialRecognitionEndedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialRecognitionEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionEndedEventArgs";
}
unsafe impl Send for SpatialRecognitionEndedEventArgs {}
unsafe impl Sync for SpatialRecognitionEndedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialRecognitionStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialRecognitionStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialRecognitionStartedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPointerPose)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsGesturePossible(&self, gesture: SpatialGestureSettings) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGesturePossible)(windows_core::Interface::as_raw(this), gesture, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialRecognitionStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialRecognitionStartedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialRecognitionStartedEventArgs {
    type Vtable = ISpatialRecognitionStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialRecognitionStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialRecognitionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialRecognitionStartedEventArgs";
}
unsafe impl Send for SpatialRecognitionStartedEventArgs {}
unsafe impl Sync for SpatialRecognitionStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SpatialTappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialTappedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialTappedEventArgs {
    pub fn InteractionSourceKind(&self) -> windows_core::Result<SpatialInteractionSourceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSourceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    pub fn TryGetPointerPose<P0>(&self, coordinatesystem: P0) -> windows_core::Result<SpatialPointerPose>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetPointerPose)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TapCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TapCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialTappedEventArgs>();
}
unsafe impl windows_core::Interface for SpatialTappedEventArgs {
    type Vtable = ISpatialTappedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISpatialTappedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Spatial.SpatialTappedEventArgs";
}
unsafe impl Send for SpatialTappedEventArgs {}
unsafe impl Sync for SpatialTappedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialGestureSettings(pub u32);
impl SpatialGestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const ManipulationTranslate: Self = Self(8u32);
    pub const NavigationX: Self = Self(16u32);
    pub const NavigationY: Self = Self(32u32);
    pub const NavigationZ: Self = Self(64u32);
    pub const NavigationRailsX: Self = Self(128u32);
    pub const NavigationRailsY: Self = Self(256u32);
    pub const NavigationRailsZ: Self = Self(512u32);
}
impl windows_core::TypeKind for SpatialGestureSettings {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialGestureSettings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialGestureSettings").field(&self.0).finish()
    }
}
impl SpatialGestureSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SpatialGestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SpatialGestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SpatialGestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SpatialGestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SpatialGestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for SpatialGestureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialGestureSettings;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialInteractionPressKind(pub i32);
impl SpatialInteractionPressKind {
    pub const None: Self = Self(0i32);
    pub const Select: Self = Self(1i32);
    pub const Menu: Self = Self(2i32);
    pub const Grasp: Self = Self(3i32);
    pub const Touchpad: Self = Self(4i32);
    pub const Thumbstick: Self = Self(5i32);
}
impl windows_core::TypeKind for SpatialInteractionPressKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialInteractionPressKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialInteractionPressKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialInteractionPressKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionPressKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialInteractionSourceHandedness(pub i32);
impl SpatialInteractionSourceHandedness {
    pub const Unspecified: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl windows_core::TypeKind for SpatialInteractionSourceHandedness {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialInteractionSourceHandedness {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceHandedness").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialInteractionSourceHandedness {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourceHandedness;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialInteractionSourceKind(pub i32);
impl SpatialInteractionSourceKind {
    pub const Other: Self = Self(0i32);
    pub const Hand: Self = Self(1i32);
    pub const Voice: Self = Self(2i32);
    pub const Controller: Self = Self(3i32);
}
impl windows_core::TypeKind for SpatialInteractionSourceKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialInteractionSourceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourceKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialInteractionSourceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourceKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialInteractionSourcePositionAccuracy(pub i32);
impl SpatialInteractionSourcePositionAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
impl windows_core::TypeKind for SpatialInteractionSourcePositionAccuracy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialInteractionSourcePositionAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialInteractionSourcePositionAccuracy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SpatialInteractionSourcePositionAccuracy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Spatial.SpatialInteractionSourcePositionAccuracy;i4)");
}
