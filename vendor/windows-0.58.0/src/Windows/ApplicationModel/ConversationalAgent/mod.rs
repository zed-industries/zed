windows_core::imp::define_interface!(IActivationSignalDetectionConfiguration, IActivationSignalDetectionConfiguration_Vtbl, 0x40d8be16_5217_581c_9ab2_ce9b2f2e8e00);
impl windows_core::RuntimeType for IActivationSignalDetectionConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivationSignalDetectionConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SignalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetEnabledAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AvailabilityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AvailabilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataAsync: usize,
    pub GetModelDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetModelDataTypeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetModelData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetModelData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetModelDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetModelDataAsync: usize,
    pub ClearModelData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearModelDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrainingStepsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TrainingStepsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TrainingDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivationSignalDetectionTrainingDataFormat) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ApplyTrainingData: unsafe extern "system" fn(*mut core::ffi::c_void, ActivationSignalDetectionTrainingDataFormat, *mut core::ffi::c_void, *mut DetectionConfigurationTrainingStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ApplyTrainingData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ApplyTrainingDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ActivationSignalDetectionTrainingDataFormat, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ApplyTrainingDataAsync: usize,
    pub ClearTrainingData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearTrainingDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivationSignalDetectionConfiguration2, IActivationSignalDetectionConfiguration2_Vtbl, 0x71d9b022_562c_57ce_a78b_8b4ff0145bab);
impl windows_core::RuntimeType for IActivationSignalDetectionConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivationSignalDetectionConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataWithResult: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataWithResult: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataWithResultAsync: usize,
    pub SetEnabledWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEnabledWithResult: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut ActivationSignalDetectionConfigurationStateChangeResult) -> windows_core::HRESULT,
    pub TrainingStepCompletionMaxAllowedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivationSignalDetectionConfigurationCreationResult, IActivationSignalDetectionConfigurationCreationResult_Vtbl, 0x4c89bc1b_8d12_5e48_a71c_7f6bc1cd66e0);
impl windows_core::RuntimeType for IActivationSignalDetectionConfigurationCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivationSignalDetectionConfigurationCreationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivationSignalDetectionConfigurationCreationStatus) -> windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivationSignalDetector, IActivationSignalDetector_Vtbl, 0xb5bf345f_a4d0_5b2b_8e65_b3c55ee756ff);
impl windows_core::RuntimeType for IActivationSignalDetector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivationSignalDetector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivationSignalDetectorKind) -> windows_core::HRESULT,
    pub CanCreateConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModelDataTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModelDataTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTrainingDataFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTrainingDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPowerStates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPowerStates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalIdAsync: usize,
    pub CreateConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CreateConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurationsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurationsAsync: usize,
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivationSignalDetector2, IActivationSignalDetector2_Vtbl, 0xc7e2490a_baa5_59d2_85d1_ba42f7cf78c9);
impl windows_core::RuntimeType for IActivationSignalDetector2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivationSignalDetector2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalId: usize,
    pub CreateConfigurationWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConfigurationWithResult: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveConfigurationWithResultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveConfigurationWithResult: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut ActivationSignalDetectionConfigurationRemovalResult) -> windows_core::HRESULT,
    pub DetectorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConversationalAgentDetectorManager, IConversationalAgentDetectorManager_Vtbl, 0xde94fbb0_597a_5df8_8cfb_9dbb583ba3ff);
impl windows_core::RuntimeType for IConversationalAgentDetectorManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentDetectorManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectorsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectors: unsafe extern "system" fn(*mut core::ffi::c_void, ActivationSignalDetectorKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ActivationSignalDetectorKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectorsAsync: usize,
}
windows_core::imp::define_interface!(IConversationalAgentDetectorManager2, IConversationalAgentDetectorManager2_Vtbl, 0x84610f31_d7f3_52fe_9311_c9eb4e3eb30a);
impl windows_core::RuntimeType for IConversationalAgentDetectorManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentDetectorManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetActivationSignalDetectorFromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivationSignalDetectorFromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConversationalAgentDetectorManagerStatics, IConversationalAgentDetectorManagerStatics_Vtbl, 0x36a8d283_fa0e_5693_8489_0fb2f0ab40d3);
impl windows_core::RuntimeType for IConversationalAgentDetectorManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentDetectorManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConversationalAgentSession, IConversationalAgentSession_Vtbl, 0xdaaae09a_b7ba_57e5_ad13_df520f9b6fa7);
impl windows_core::RuntimeType for IConversationalAgentSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionInterrupted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSessionInterrupted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SignalDetected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSignalDetected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SystemStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSystemStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AgentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConversationalAgentState) -> windows_core::HRESULT,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsIndicatorLightAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsScreenAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUserAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVoiceActivationAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInterruptible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInterrupted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RequestInterruptibleAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestInterruptible: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut ConversationalAgentSessionUpdateResponse) -> windows_core::HRESULT,
    pub RequestAgentStateChangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ConversationalAgentState, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAgentStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, ConversationalAgentState, *mut ConversationalAgentSessionUpdateResponse) -> windows_core::HRESULT,
    pub RequestForegroundActivationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestForegroundActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConversationalAgentSessionUpdateResponse) -> windows_core::HRESULT,
    pub GetAudioClientAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAudioClient: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Audio")]
    pub CreateAudioDeviceInputNodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    CreateAudioDeviceInputNodeAsync: usize,
    #[cfg(feature = "Media_Audio")]
    pub CreateAudioDeviceInputNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    CreateAudioDeviceInputNode: usize,
    pub GetAudioCaptureDeviceIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAudioCaptureDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetAudioRenderDeviceIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAudioRenderDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetSignalModelIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSignalModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSignalModelIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSignalModelId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIdsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIds: usize,
}
windows_core::imp::define_interface!(IConversationalAgentSession2, IConversationalAgentSession2_Vtbl, 0xa7a9fbf9_ac78_57ff_9596_acc7a1c9a607);
impl windows_core::RuntimeType for IConversationalAgentSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestActivationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, ConversationalAgentActivationKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestActivation: unsafe extern "system" fn(*mut core::ffi::c_void, ConversationalAgentActivationKind, *mut ConversationalAgentActivationResult) -> windows_core::HRESULT,
    pub SetSupportLockScreenActivationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSupportLockScreenActivation: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisites: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisitesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisitesAsync: usize,
}
windows_core::imp::define_interface!(IConversationalAgentSessionInterruptedEventArgs, IConversationalAgentSessionInterruptedEventArgs_Vtbl, 0x9766591f_f63d_5d3e_9bf2_bd0760552686);
impl windows_core::RuntimeType for IConversationalAgentSessionInterruptedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSessionInterruptedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IConversationalAgentSessionStatics, IConversationalAgentSessionStatics_Vtbl, 0xa005166e_e954_576e_be04_11b8ed10f37b);
impl windows_core::RuntimeType for IConversationalAgentSessionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSessionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentSessionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSessionSync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConversationalAgentSignal, IConversationalAgentSignal_Vtbl, 0x20ed25f7_b120_51f2_8603_265d6a47f232);
impl windows_core::RuntimeType for IConversationalAgentSignal {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSignal_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSignalVerificationRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSignalVerificationRequired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SignalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSignalId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SignalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSignalName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SignalContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSignalContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SignalStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetSignalStart: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SignalEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetSignalEnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConversationalAgentSignal2, IConversationalAgentSignal2_Vtbl, 0xd0cc7ba9_9a7b_5c34_880e_b6146c904ecb);
impl windows_core::RuntimeType for IConversationalAgentSignal2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSignal2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DetectorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DetectorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivationSignalDetectorKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConversationalAgentSignalDetectedEventArgs, IConversationalAgentSignalDetectedEventArgs_Vtbl, 0x4d57eb8f_f88a_599b_91d3_d604876708bc);
impl windows_core::RuntimeType for IConversationalAgentSignalDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSignalDetectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IConversationalAgentSystemStateChangedEventArgs, IConversationalAgentSystemStateChangedEventArgs_Vtbl, 0x1c2c6e3e_2785_59a7_8e71_38adeef79928);
impl windows_core::RuntimeType for IConversationalAgentSystemStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConversationalAgentSystemStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SystemStateChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConversationalAgentSystemStateChangeType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDetectionConfigurationAvailabilityChangedEventArgs, IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl, 0x5129c9fb_4be8_5f14_af2b_88d62b1b4462);
impl windows_core::RuntimeType for IDetectionConfigurationAvailabilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DetectionConfigurationAvailabilityChangeKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDetectionConfigurationAvailabilityInfo, IDetectionConfigurationAvailabilityInfo_Vtbl, 0xb5affeb0_40f0_5398_b838_91979c2c6208);
impl windows_core::RuntimeType for IDetectionConfigurationAvailabilityInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDetectionConfigurationAvailabilityInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasSystemResourceAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasPermission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasLockScreenPermission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDetectionConfigurationAvailabilityInfo2, IDetectionConfigurationAvailabilityInfo2_Vtbl, 0x30e06433_38b3_5c4b_84c3_62b6e685b2ff);
impl windows_core::RuntimeType for IDetectionConfigurationAvailabilityInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDetectionConfigurationAvailabilityInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UnavailableSystemResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnavailableSystemResources: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivationSignalDetectionConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivationSignalDetectionConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActivationSignalDetectionConfiguration, super::super::Foundation::IClosable);
impl ActivationSignalDetectionConfiguration {
    pub fn SignalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetEnabledAsync(&self, value: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetEnabledAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AvailabilityInfo(&self) -> windows_core::Result<DetectionConfigurationAvailabilityInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailabilityInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AvailabilityChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailabilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAvailabilityChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAvailabilityChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelData<P0>(&self, datatype: &windows_core::HSTRING, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetModelData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(datatype), data.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataAsync<P0>(&self, datatype: &windows_core::HSTRING, data: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetModelDataAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(datatype), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetModelDataType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetModelDataType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetModelDataTypeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetModelDataTypeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetModelData(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetModelData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetModelDataAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetModelDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearModelData(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearModelData)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ClearModelDataAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearModelDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrainingStepsCompleted(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrainingStepsCompleted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TrainingStepsRemaining(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrainingStepsRemaining)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TrainingDataFormat(&self) -> windows_core::Result<ActivationSignalDetectionTrainingDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrainingDataFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ApplyTrainingData<P0>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: P0) -> windows_core::Result<DetectionConfigurationTrainingStatus>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTrainingData)(windows_core::Interface::as_raw(this), trainingdataformat, trainingdata.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ApplyTrainingDataAsync<P0>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTrainingDataAsync)(windows_core::Interface::as_raw(this), trainingdataformat, trainingdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearTrainingData(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearTrainingData)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ClearTrainingDataAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearTrainingDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataWithResult<P0>(&self, datatype: &windows_core::HSTRING, data: P0) -> windows_core::Result<ActivationSignalDetectionConfigurationSetModelDataResult>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = &windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetModelDataWithResult)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(datatype), data.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataWithResultAsync<P0>(&self, datatype: &windows_core::HSTRING, data: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IInputStream>,
    {
        let this = &windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetModelDataWithResultAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(datatype), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEnabledWithResultAsync(&self, value: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetEnabledWithResultAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEnabledWithResult(&self, value: bool) -> windows_core::Result<ActivationSignalDetectionConfigurationStateChangeResult> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetEnabledWithResult)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn TrainingStepCompletionMaxAllowedTime(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrainingStepCompletionMaxAllowedTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectionConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivationSignalDetectionConfiguration>();
}
unsafe impl windows_core::Interface for ActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_Vtbl;
    const IID: windows_core::GUID = <IActivationSignalDetectionConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
}
unsafe impl Send for ActivationSignalDetectionConfiguration {}
unsafe impl Sync for ActivationSignalDetectionConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivationSignalDetectionConfigurationCreationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivationSignalDetectionConfigurationCreationResult, windows_core::IUnknown, windows_core::IInspectable);
impl ActivationSignalDetectionConfigurationCreationResult {
    pub fn Status(&self) -> windows_core::Result<ActivationSignalDetectionConfigurationCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectionConfigurationCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivationSignalDetectionConfigurationCreationResult>();
}
unsafe impl windows_core::Interface for ActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_Vtbl;
    const IID: windows_core::GUID = <IActivationSignalDetectionConfigurationCreationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
}
unsafe impl Send for ActivationSignalDetectionConfigurationCreationResult {}
unsafe impl Sync for ActivationSignalDetectionConfigurationCreationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ActivationSignalDetector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivationSignalDetector, windows_core::IUnknown, windows_core::IInspectable);
impl ActivationSignalDetector {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ActivationSignalDetectorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanCreateConfigurations(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCreateConfigurations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModelDataTypes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedModelDataTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTrainingDataFormats(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedTrainingDataFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPowerStates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedPowerStates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalId(&self, signalid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedModelIdsForSignalId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalIdAsync(&self, signalid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedModelIdsForSignalIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateConfiguration(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CreateConfiguration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), core::mem::transmute_copy(displayname)).ok() }
    }
    pub fn CreateConfigurationAsync(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateConfigurationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurations(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConfigurations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurationsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConfigurationsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetConfiguration(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING) -> windows_core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConfiguration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetConfigurationAsync(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConfigurationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveConfiguration(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConfiguration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid)).ok() }
    }
    pub fn RemoveConfigurationAsync(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveConfigurationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalIdAsync(&self, signalid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAvailableModelIdsForSignalIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalId(&self, signalid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAvailableModelIdsForSignalId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateConfigurationWithResultAsync(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateConfigurationWithResultAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateConfigurationWithResult(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING, displayname: &windows_core::HSTRING) -> windows_core::Result<ActivationSignalDetectionConfigurationCreationResult> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateConfigurationWithResult)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), core::mem::transmute_copy(displayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveConfigurationWithResultAsync(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveConfigurationWithResultAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveConfigurationWithResult(&self, signalid: &windows_core::HSTRING, modelid: &windows_core::HSTRING) -> windows_core::Result<ActivationSignalDetectionConfigurationRemovalResult> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveConfigurationWithResult)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(signalid), core::mem::transmute_copy(modelid), &mut result__).map(|| result__)
        }
    }
    pub fn DetectorId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetectorId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActivationSignalDetector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivationSignalDetector>();
}
unsafe impl windows_core::Interface for ActivationSignalDetector {
    type Vtable = IActivationSignalDetector_Vtbl;
    const IID: windows_core::GUID = <IActivationSignalDetector as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
}
unsafe impl Send for ActivationSignalDetector {}
unsafe impl Sync for ActivationSignalDetector {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConversationalAgentDetectorManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversationalAgentDetectorManager, windows_core::IUnknown, windows_core::IInspectable);
impl ConversationalAgentDetectorManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllActivationSignalDetectors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectorsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllActivationSignalDetectorsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActivationSignalDetectors)(windows_core::Interface::as_raw(this), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActivationSignalDetectorsAsync)(windows_core::Interface::as_raw(this), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetActivationSignalDetectorFromId(&self, detectorid: &windows_core::HSTRING) -> windows_core::Result<ActivationSignalDetector> {
        let this = &windows_core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActivationSignalDetectorFromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(detectorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetActivationSignalDetectorFromIdAsync(&self, detectorid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>> {
        let this = &windows_core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActivationSignalDetectorFromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(detectorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Default() -> windows_core::Result<ConversationalAgentDetectorManager> {
        Self::IConversationalAgentDetectorManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Default)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IConversationalAgentDetectorManagerStatics<R, F: FnOnce(&IConversationalAgentDetectorManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ConversationalAgentDetectorManager, IConversationalAgentDetectorManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ConversationalAgentDetectorManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConversationalAgentDetectorManager>();
}
unsafe impl windows_core::Interface for ConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_Vtbl;
    const IID: windows_core::GUID = <IConversationalAgentDetectorManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
}
unsafe impl Send for ConversationalAgentDetectorManager {}
unsafe impl Sync for ConversationalAgentDetectorManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConversationalAgentSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversationalAgentSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ConversationalAgentSession, super::super::Foundation::IClosable);
impl ConversationalAgentSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SessionInterrupted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionInterrupted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionInterrupted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSessionInterrupted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SignalDetected<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalDetected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSignalDetected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSignalDetected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SystemStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSystemStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSystemStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AgentState(&self) -> windows_core::Result<ConversationalAgentState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AgentState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Signal(&self) -> windows_core::Result<ConversationalAgentSignal> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Signal)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsIndicatorLightAvailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIndicatorLightAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsScreenAvailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScreenAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUserAuthenticated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUserAuthenticated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVoiceActivationAvailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVoiceActivationAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInterruptible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInterruptible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInterrupted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInterrupted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestInterruptibleAsync(&self, interruptible: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestInterruptibleAsync)(windows_core::Interface::as_raw(this), interruptible, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestInterruptible(&self, interruptible: bool) -> windows_core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestInterruptible)(windows_core::Interface::as_raw(this), interruptible, &mut result__).map(|| result__)
        }
    }
    pub fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAgentStateChangeAsync)(windows_core::Interface::as_raw(this), state, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> windows_core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAgentStateChange)(windows_core::Interface::as_raw(this), state, &mut result__).map(|| result__)
        }
    }
    pub fn RequestForegroundActivationAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestForegroundActivationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestForegroundActivation(&self) -> windows_core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestForegroundActivation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetAudioClientAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioClientAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAudioClient(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioClient)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Audio")]
    pub fn CreateAudioDeviceInputNodeAsync<P0>(&self, graph: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>>
    where
        P0: windows_core::Param<super::super::Media::Audio::AudioGraph>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAudioDeviceInputNodeAsync)(windows_core::Interface::as_raw(this), graph.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Audio")]
    pub fn CreateAudioDeviceInputNode<P0>(&self, graph: P0) -> windows_core::Result<super::super::Media::Audio::AudioDeviceInputNode>
    where
        P0: windows_core::Param<super::super::Media::Audio::AudioGraph>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAudioDeviceInputNode)(windows_core::Interface::as_raw(this), graph.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAudioCaptureDeviceIdAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioCaptureDeviceIdAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAudioCaptureDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioCaptureDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAudioRenderDeviceIdAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioRenderDeviceIdAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAudioRenderDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioRenderDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSignalModelIdAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSignalModelIdAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSignalModelId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSignalModelId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetSignalModelIdAsync)(windows_core::Interface::as_raw(this), signalmodelid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSignalModelId(&self, signalmodelid: u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetSignalModelId)(windows_core::Interface::as_raw(this), signalmodelid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIdsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedSignalModelIdsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIds(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedSignalModelIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestActivationAsync)(windows_core::Interface::as_raw(this), activationkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> windows_core::Result<ConversationalAgentActivationResult> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestActivation)(windows_core::Interface::as_raw(this), activationkind, &mut result__).map(|| result__)
        }
    }
    pub fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetSupportLockScreenActivationAsync)(windows_core::Interface::as_raw(this), lockscreenactivationsupported, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSupportLockScreenActivation)(windows_core::Interface::as_raw(this), lockscreenactivationsupported).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisites(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMissingPrerequisites)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisitesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMissingPrerequisitesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentSessionAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSessionAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCurrentSessionSync() -> windows_core::Result<ConversationalAgentSession> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSessionSync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IConversationalAgentSessionStatics<R, F: FnOnce(&IConversationalAgentSessionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ConversationalAgentSession, IConversationalAgentSessionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ConversationalAgentSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConversationalAgentSession>();
}
unsafe impl windows_core::Interface for ConversationalAgentSession {
    type Vtable = IConversationalAgentSession_Vtbl;
    const IID: windows_core::GUID = <IConversationalAgentSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
}
unsafe impl Send for ConversationalAgentSession {}
unsafe impl Sync for ConversationalAgentSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConversationalAgentSessionInterruptedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversationalAgentSessionInterruptedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ConversationalAgentSessionInterruptedEventArgs {}
impl windows_core::RuntimeType for ConversationalAgentSessionInterruptedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConversationalAgentSessionInterruptedEventArgs>();
}
unsafe impl windows_core::Interface for ConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IConversationalAgentSessionInterruptedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
}
unsafe impl Send for ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl Sync for ConversationalAgentSessionInterruptedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConversationalAgentSignal(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversationalAgentSignal, windows_core::IUnknown, windows_core::IInspectable);
impl ConversationalAgentSignal {
    pub fn IsSignalVerificationRequired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSignalVerificationRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSignalVerificationRequired(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsSignalVerificationRequired)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSignalId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignalId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SignalName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSignalName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignalName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SignalContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSignalContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignalContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SignalStart(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalStart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignalStart(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignalStart)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignalEnd(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalEnd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSignalEnd(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignalEnd)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DetectorId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetectorId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetectorKind(&self) -> windows_core::Result<ActivationSignalDetectorKind> {
        let this = &windows_core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetectorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ConversationalAgentSignal {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConversationalAgentSignal>();
}
unsafe impl windows_core::Interface for ConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_Vtbl;
    const IID: windows_core::GUID = <IConversationalAgentSignal as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
}
unsafe impl Send for ConversationalAgentSignal {}
unsafe impl Sync for ConversationalAgentSignal {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConversationalAgentSignalDetectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversationalAgentSignalDetectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ConversationalAgentSignalDetectedEventArgs {}
impl windows_core::RuntimeType for ConversationalAgentSignalDetectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConversationalAgentSignalDetectedEventArgs>();
}
unsafe impl windows_core::Interface for ConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IConversationalAgentSignalDetectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
}
unsafe impl Send for ConversationalAgentSignalDetectedEventArgs {}
unsafe impl Sync for ConversationalAgentSignalDetectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConversationalAgentSystemStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversationalAgentSystemStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ConversationalAgentSystemStateChangedEventArgs {
    pub fn SystemStateChangeType(&self) -> windows_core::Result<ConversationalAgentSystemStateChangeType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemStateChangeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ConversationalAgentSystemStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConversationalAgentSystemStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for ConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IConversationalAgentSystemStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
}
unsafe impl Send for ConversationalAgentSystemStateChangedEventArgs {}
unsafe impl Sync for ConversationalAgentSystemStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DetectionConfigurationAvailabilityChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DetectionConfigurationAvailabilityChangedEventArgs {
    pub fn Kind(&self) -> windows_core::Result<DetectionConfigurationAvailabilityChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DetectionConfigurationAvailabilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDetectionConfigurationAvailabilityChangedEventArgs>();
}
unsafe impl windows_core::Interface for DetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDetectionConfigurationAvailabilityChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
}
unsafe impl Send for DetectionConfigurationAvailabilityChangedEventArgs {}
unsafe impl Sync for DetectionConfigurationAvailabilityChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DetectionConfigurationAvailabilityInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DetectionConfigurationAvailabilityInfo, windows_core::IUnknown, windows_core::IInspectable);
impl DetectionConfigurationAvailabilityInfo {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasSystemResourceAccess(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasSystemResourceAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasPermission(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasPermission)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasLockScreenPermission(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasLockScreenPermission)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnavailableSystemResources(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>> {
        let this = &windows_core::Interface::cast::<IDetectionConfigurationAvailabilityInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnavailableSystemResources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DetectionConfigurationAvailabilityInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDetectionConfigurationAvailabilityInfo>();
}
unsafe impl windows_core::Interface for DetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_Vtbl;
    const IID: windows_core::GUID = <IDetectionConfigurationAvailabilityInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
}
unsafe impl Send for DetectionConfigurationAvailabilityInfo {}
unsafe impl Sync for DetectionConfigurationAvailabilityInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SignalIdNotAvailable: Self = Self(1i32);
    pub const ModelIdNotSupported: Self = Self(2i32);
    pub const InvalidSignalId: Self = Self(3i32);
    pub const InvalidModelId: Self = Self(4i32);
    pub const InvalidDisplayName: Self = Self(5i32);
    pub const ConfigurationAlreadyExists: Self = Self(6i32);
    pub const CreationNotSupported: Self = Self(7i32);
}
impl windows_core::TypeKind for ActivationSignalDetectionConfigurationCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivationSignalDetectionConfigurationCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectionConfigurationCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: Self = Self(0i32);
    pub const NotFound: Self = Self(1i32);
    pub const CurrentlyEnabled: Self = Self(2i32);
    pub const RemovalNotSupported: Self = Self(3i32);
}
impl windows_core::TypeKind for ActivationSignalDetectionConfigurationRemovalResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivationSignalDetectionConfigurationRemovalResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationRemovalResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectionConfigurationRemovalResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: Self = Self(0i32);
    pub const EmptyModelData: Self = Self(1i32);
    pub const UnsupportedFormat: Self = Self(2i32);
    pub const ConfigurationCurrentlyEnabled: Self = Self(3i32);
    pub const InvalidData: Self = Self(4i32);
    pub const SetModelDataNotSupported: Self = Self(5i32);
    pub const ConfigurationNotFound: Self = Self(6i32);
    pub const UnknownError: Self = Self(7i32);
}
impl windows_core::TypeKind for ActivationSignalDetectionConfigurationSetModelDataResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationSetModelDataResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectionConfigurationSetModelDataResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: Self = Self(0i32);
    pub const NoModelData: Self = Self(1i32);
    pub const ConfigurationNotFound: Self = Self(2i32);
}
impl windows_core::TypeKind for ActivationSignalDetectionConfigurationStateChangeResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivationSignalDetectionConfigurationStateChangeResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationStateChangeResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectionConfigurationStateChangeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: Self = Self(0i32);
    pub const Voice8kHz16BitMono: Self = Self(1i32);
    pub const Voice16kHz8BitMono: Self = Self(2i32);
    pub const Voice16kHz16BitMono: Self = Self(3i32);
    pub const VoiceOEMDefined: Self = Self(4i32);
    pub const Audio44kHz8BitMono: Self = Self(5i32);
    pub const Audio44kHz16BitMono: Self = Self(6i32);
    pub const Audio48kHz8BitMono: Self = Self(7i32);
    pub const Audio48kHz16BitMono: Self = Self(8i32);
    pub const AudioOEMDefined: Self = Self(9i32);
    pub const OtherOEMDefined: Self = Self(10i32);
}
impl windows_core::TypeKind for ActivationSignalDetectionTrainingDataFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivationSignalDetectionTrainingDataFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionTrainingDataFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectionTrainingDataFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: Self = Self(0i32);
    pub const AudioImpulse: Self = Self(1i32);
    pub const HardwareEvent: Self = Self(2i32);
}
impl windows_core::TypeKind for ActivationSignalDetectorKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivationSignalDetectorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: Self = Self(0i32);
    pub const ConnectedLowPower: Self = Self(1i32);
    pub const DisconnectedLowPower: Self = Self(2i32);
}
impl windows_core::TypeKind for ActivationSignalDetectorPowerState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ActivationSignalDetectorPowerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorPowerState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ActivationSignalDetectorPowerState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
}
impl windows_core::TypeKind for ConversationalAgentActivationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConversationalAgentActivationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConversationalAgentActivationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: Self = Self(0i32);
    pub const AgentInactive: Self = Self(1i32);
    pub const ScreenNotAvailable: Self = Self(2i32);
    pub const AgentInterrupted: Self = Self(3i32);
}
impl windows_core::TypeKind for ConversationalAgentActivationResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConversationalAgentActivationResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConversationalAgentActivationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl windows_core::TypeKind for ConversationalAgentSessionUpdateResponse {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConversationalAgentSessionUpdateResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConversationalAgentSessionUpdateResponse").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConversationalAgentSessionUpdateResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const Listening: Self = Self(2i32);
    pub const Working: Self = Self(3i32);
    pub const Speaking: Self = Self(4i32);
    pub const ListeningAndSpeaking: Self = Self(5i32);
}
impl windows_core::TypeKind for ConversationalAgentState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConversationalAgentState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConversationalAgentState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConversationalAgentState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: Self = Self(0i32);
    pub const ScreenAvailability: Self = Self(1i32);
    pub const IndicatorLightAvailability: Self = Self(2i32);
    pub const VoiceActivationAvailability: Self = Self(3i32);
}
impl windows_core::TypeKind for ConversationalAgentSystemStateChangeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConversationalAgentSystemStateChangeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConversationalAgentSystemStateChangeType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConversationalAgentSystemStateChangeType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: Self = Self(0i32);
    pub const KnownAgents: Self = Self(1i32);
    pub const AgentAllowed: Self = Self(2i32);
    pub const AppCapability: Self = Self(3i32);
    pub const BackgroundTaskRegistration: Self = Self(4i32);
    pub const PolicyPermission: Self = Self(5i32);
}
impl windows_core::TypeKind for ConversationalAgentVoiceActivationPrerequisiteKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConversationalAgentVoiceActivationPrerequisiteKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConversationalAgentVoiceActivationPrerequisiteKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: Self = Self(0i32);
    pub const Permission: Self = Self(1i32);
    pub const LockScreenPermission: Self = Self(2i32);
}
impl windows_core::TypeKind for DetectionConfigurationAvailabilityChangeKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DetectionConfigurationAvailabilityChangeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityChangeKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DetectionConfigurationAvailabilityChangeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const VoiceTooQuiet: Self = Self(2i32);
    pub const VoiceTooLoud: Self = Self(3i32);
    pub const VoiceTooFast: Self = Self(4i32);
    pub const VoiceTooSlow: Self = Self(5i32);
    pub const VoiceQualityProblem: Self = Self(6i32);
    pub const TrainingSystemInternalError: Self = Self(7i32);
    pub const TrainingTimedOut: Self = Self(8i32);
    pub const ConfigurationNotFound: Self = Self(9i32);
}
impl windows_core::TypeKind for DetectionConfigurationTrainingStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DetectionConfigurationTrainingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DetectionConfigurationTrainingStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DetectionConfigurationTrainingStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: Self = Self(0i32);
    pub const ParallelModelSupportForAgent: Self = Self(1i32);
    pub const ParallelSignalSupport: Self = Self(2i32);
    pub const ParallelSignalSupportForAgent: Self = Self(3i32);
    pub const DisplayOffSupport: Self = Self(4i32);
    pub const PluggedInPower: Self = Self(5i32);
    pub const Detector: Self = Self(6i32);
    pub const SupportedSleepState: Self = Self(7i32);
    pub const SupportedBatterySaverState: Self = Self(8i32);
    pub const ScreenAvailability: Self = Self(9i32);
    pub const InputHardware: Self = Self(10i32);
    pub const AcousticEchoCancellation: Self = Self(11i32);
    pub const ModelIdSupport: Self = Self(12i32);
    pub const DataChannel: Self = Self(13i32);
}
impl windows_core::TypeKind for SignalDetectorResourceKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SignalDetectorResourceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SignalDetectorResourceKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SignalDetectorResourceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind;i4)");
}
