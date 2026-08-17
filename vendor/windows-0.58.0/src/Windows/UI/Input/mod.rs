#[cfg(feature = "UI_Input_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking")]
pub mod Inking;
#[cfg(feature = "UI_Input_Preview")]
pub mod Preview;
#[cfg(feature = "UI_Input_Spatial")]
pub mod Spatial;
windows_core::imp::define_interface!(IAttachableInputObject, IAttachableInputObject_Vtbl, 0x9b822734_a3c1_542a_b2f4_0e32b773fb07);
impl windows_core::RuntimeType for IAttachableInputObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAttachableInputObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IAttachableInputObjectFactory, IAttachableInputObjectFactory_Vtbl, 0xa4c54c4e_42bc_58fa_a640_ea1516f4c06b);
impl windows_core::RuntimeType for IAttachableInputObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAttachableInputObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICrossSlidingEventArgs, ICrossSlidingEventArgs_Vtbl, 0xe9374738_6f88_41d9_8720_78e08e398349);
impl windows_core::RuntimeType for ICrossSlidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICrossSlidingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub CrossSlidingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CrossSlidingState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICrossSlidingEventArgs2, ICrossSlidingEventArgs2_Vtbl, 0xeefb7d48_c070_59f3_8dab_bcaf621d8687);
impl windows_core::RuntimeType for ICrossSlidingEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICrossSlidingEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDraggingEventArgs, IDraggingEventArgs_Vtbl, 0x1c905384_083c_4bd3_b559_179cddeb33ec);
impl windows_core::RuntimeType for IDraggingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDraggingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub DraggingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DraggingState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDraggingEventArgs2, IDraggingEventArgs2_Vtbl, 0x71efdbf9_382a_55ca_b4b9_008123c1bf1a);
impl windows_core::RuntimeType for IDraggingEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDraggingEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEdgeGesture, IEdgeGesture_Vtbl, 0x580d5292_2ab1_49aa_a7f0_33bd3f8df9f1);
impl windows_core::RuntimeType for IEdgeGesture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEdgeGesture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Starting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStarting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Canceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEdgeGestureEventArgs, IEdgeGestureEventArgs_Vtbl, 0x44fa4a24_2d09_42e1_8b5e_368208796a4c);
impl windows_core::RuntimeType for IEdgeGestureEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEdgeGestureEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EdgeGestureKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEdgeGestureStatics, IEdgeGestureStatics_Vtbl, 0xbc6a8519_18ee_4043_9839_4fc584d60a14);
impl windows_core::RuntimeType for IEdgeGestureStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEdgeGestureStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGestureRecognizer, IGestureRecognizer_Vtbl, 0xb47a37bf_3d6b_4f88_83e8_6dcb4012ffb0);
impl windows_core::RuntimeType for IGestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGestureRecognizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GestureSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GestureSettings) -> windows_core::HRESULT,
    pub SetGestureSettings: unsafe extern "system" fn(*mut core::ffi::c_void, GestureSettings) -> windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShowGestureFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowGestureFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PivotCenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub SetPivotCenter: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaTranslationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaTranslationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaRotationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaRotationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaTranslationDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaTranslationDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ManipulationExact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetManipulationExact: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CrossSlideThresholds) -> windows_core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, CrossSlideThresholds) -> windows_core::HRESULT,
    pub CrossSlideHorizontally: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCrossSlideHorizontally: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideExact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCrossSlideExact: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AutoProcessInertia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoProcessInertia: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProcessMoveEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProcessMoveEvents: usize,
    pub ProcessUpEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, bool) -> windows_core::HRESULT,
    pub ProcessInertia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompleteGesture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTapped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RightTapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRightTapped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Holding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHolding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Dragging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDragging: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationInertiaStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CrossSliding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCrossSliding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGestureRecognizer2, IGestureRecognizer2_Vtbl, 0xd646097f_6ef7_5746_8ba8_8ff2206e6f3b);
impl windows_core::RuntimeType for IGestureRecognizer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGestureRecognizer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TapMinContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTapMinContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TapMaxContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTapMaxContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HoldMinContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHoldMinContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HoldMaxContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHoldMaxContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HoldRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetHoldRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub HoldStartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetHoldStartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub TranslationMinContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTranslationMinContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TranslationMaxContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTranslationMaxContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHoldingEventArgs, IHoldingEventArgs_Vtbl, 0x2bf755c5_e799_41b4_bb40_242f40959b71);
impl windows_core::RuntimeType for IHoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHoldingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub HoldingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HoldingState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHoldingEventArgs2, IHoldingEventArgs2_Vtbl, 0x141da9ea_4c79_5674_afea_493fdeb91f19);
impl windows_core::RuntimeType for IHoldingEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHoldingEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputActivationListener, IInputActivationListener_Vtbl, 0x5d6d4ed2_28c7_5ae3_aa74_c918a9f243ca);
impl windows_core::RuntimeType for IInputActivationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputActivationListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputActivationState) -> windows_core::HRESULT,
    pub InputActivationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInputActivationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputActivationListenerActivationChangedEventArgs, IInputActivationListenerActivationChangedEventArgs_Vtbl, 0x7699b465_1dcf_5791_b4b9_6cafbeed2056);
impl windows_core::RuntimeType for IInputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputActivationListenerActivationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputActivationState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyboardDeliveryInterceptor, IKeyboardDeliveryInterceptor_Vtbl, 0xb4baf068_8f49_446c_8db5_8c0ffe85cc9e);
impl windows_core::RuntimeType for IKeyboardDeliveryInterceptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyboardDeliveryInterceptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsInterceptionEnabledWhenInForeground: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyDown: usize,
    pub RemoveKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyUp: usize,
    pub RemoveKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyboardDeliveryInterceptorStatics, IKeyboardDeliveryInterceptorStatics_Vtbl, 0xf9f63ba2_ceba_4755_8a7e_14c0ffecd239);
impl windows_core::RuntimeType for IKeyboardDeliveryInterceptorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyboardDeliveryInterceptorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationCompletedEventArgs, IManipulationCompletedEventArgs_Vtbl, 0xb34ab22b_d19b_46ff_9f38_dec7754bb9e7);
impl windows_core::RuntimeType for IManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationVelocities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationCompletedEventArgs2, IManipulationCompletedEventArgs2_Vtbl, 0xf0c0dce7_30a9_5b96_886f_6560a85e4757);
impl windows_core::RuntimeType for IManipulationCompletedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationCompletedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationInertiaStartingEventArgs, IManipulationInertiaStartingEventArgs_Vtbl, 0xdd37a898_26bf_467a_9ce5_ccf3fb11371e);
impl windows_core::RuntimeType for IManipulationInertiaStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationInertiaStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationVelocities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationInertiaStartingEventArgs2, IManipulationInertiaStartingEventArgs2_Vtbl, 0xc25409b8_f9fa_5a45_bd97_dcbbb2201860);
impl windows_core::RuntimeType for IManipulationInertiaStartingEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationInertiaStartingEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationStartedEventArgs, IManipulationStartedEventArgs_Vtbl, 0xddec873e_cfce_4932_8c1d_3c3d011a34c0);
impl windows_core::RuntimeType for IManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationStartedEventArgs2, IManipulationStartedEventArgs2_Vtbl, 0x2da3db4e_e583_5055_afaa_16fd986531a6);
impl windows_core::RuntimeType for IManipulationStartedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationStartedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationUpdatedEventArgs, IManipulationUpdatedEventArgs_Vtbl, 0xcb354ce5_abb8_4f9f_b3ce_8181aa61ad82);
impl windows_core::RuntimeType for IManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationVelocities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationUpdatedEventArgs2, IManipulationUpdatedEventArgs2_Vtbl, 0xf3dfb96a_3306_5903_a1c5_ff9757a8689e);
impl windows_core::RuntimeType for IManipulationUpdatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IManipulationUpdatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMouseWheelParameters, IMouseWheelParameters_Vtbl, 0xead0ca44_9ded_4037_8149_5e4cc2564468);
impl windows_core::RuntimeType for IMouseWheelParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMouseWheelParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CharTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub SetCharTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
    pub DeltaScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDeltaScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DeltaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDeltaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub PageTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub SetPageTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPoint, IPointerPoint_Vtbl, 0xe995317d_7296_42d9_8233_c5be73b74a4a);
impl windows_core::RuntimeType for IPointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDevice: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub RawPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub PointerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointProperties, IPointerPointProperties_Vtbl, 0xc79d8a4b_c163_4ee7_803f_67ce79f9972d);
impl windows_core::RuntimeType for IPointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub IsInverted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEraser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub XTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub YTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Twist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub ContactRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub ContactRectRaw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub TouchConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLeftButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRightButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMiddleButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MouseWheelDelta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsHorizontalMouseWheel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBarrelButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton1Pressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton2Pressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerUpdateKind) -> windows_core::HRESULT,
    pub HasUsage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut bool) -> windows_core::HRESULT,
    pub GetUsageValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointProperties2, IPointerPointProperties2_Vtbl, 0x22c3433a_c83b_41c0_a296_5e232d64d6af);
impl windows_core::RuntimeType for IPointerPointProperties2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerPointProperties2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ZDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointStatics, IPointerPointStatics_Vtbl, 0xa506638d_2a1a_413e_bc75_9f38381cc069);
impl windows_core::RuntimeType for IPointerPointStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerPointStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
    pub GetCurrentPointTransformed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePointsTransformed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePointsTransformed: usize,
}
windows_core::imp::define_interface!(IPointerPointTransform, IPointerPointTransform_Vtbl, 0x4d5fe14f_b87c_4028_bc9c_59e9947fb056);
impl core::ops::Deref for IPointerPointTransform {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPointerPointTransform, windows_core::IUnknown, windows_core::IInspectable);
impl IPointerPointTransform {
    pub fn Inverse(&self) -> windows_core::Result<IPointerPointTransform> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Inverse)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryTransform(&self, inpoint: super::super::Foundation::Point, outpoint: &mut super::super::Foundation::Point) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryTransform)(windows_core::Interface::as_raw(this), inpoint, outpoint, &mut result__).map(|| result__)
        }
    }
    pub fn TransformBounds(&self, rect: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformBounds)(windows_core::Interface::as_raw(this), rect, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IPointerPointTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerPointTransform_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Inverse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point, *mut super::super::Foundation::Point, *mut bool) -> windows_core::HRESULT,
    pub TransformBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerVisualizationSettings, IPointerVisualizationSettings_Vtbl, 0x4d1e6461_84f7_499d_bd91_2a36e2b7aaa2);
impl windows_core::RuntimeType for IPointerVisualizationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerVisualizationSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsContactFeedbackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsContactFeedbackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsBarrelButtonFeedbackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerVisualizationSettingsStatics, IPointerVisualizationSettingsStatics_Vtbl, 0x68870edb_165b_4214_b4f3_584eca8c8a69);
impl windows_core::RuntimeType for IPointerVisualizationSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerVisualizationSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialController, IRadialController_Vtbl, 0x3055d1c8_df51_43d4_b23b_0e1037467a09);
impl windows_core::RuntimeType for IRadialController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Menu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RotationResolutionInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRotationResolutionInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub UseAutomaticHapticFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseAutomaticHapticFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ScreenContactStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScreenContactStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ScreenContactEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScreenContactEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ScreenContactContinued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScreenContactContinued: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ControlLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveControlLost: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RotationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRotationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ControlAcquired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveControlAcquired: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialController2, IRadialController2_Vtbl, 0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ButtonHolding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveButtonHolding: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ButtonReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveButtonReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerButtonClickedEventArgs, IRadialControllerButtonClickedEventArgs_Vtbl, 0x206aa438_e651_11e5_bf62_2c27d7404e85);
impl windows_core::RuntimeType for IRadialControllerButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerButtonClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerButtonClickedEventArgs2, IRadialControllerButtonClickedEventArgs2_Vtbl, 0x3d577ef3_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerButtonClickedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerButtonClickedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerButtonHoldingEventArgs, IRadialControllerButtonHoldingEventArgs_Vtbl, 0x3d577eee_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerButtonHoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerButtonHoldingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerButtonPressedEventArgs, IRadialControllerButtonPressedEventArgs_Vtbl, 0x3d577eed_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerButtonPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerButtonPressedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerButtonReleasedEventArgs, IRadialControllerButtonReleasedEventArgs_Vtbl, 0x3d577eef_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerButtonReleasedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerButtonReleasedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerConfiguration, IRadialControllerConfiguration_Vtbl, 0xa6b79ecb_6a52_4430_910c_56370a9d6b42);
impl windows_core::RuntimeType for IRadialControllerConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDefaultMenuItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDefaultMenuItems: usize,
    pub ResetToDefaultMenuItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySelectDefaultMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, RadialControllerSystemMenuItemKind, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerConfiguration2, IRadialControllerConfiguration2_Vtbl, 0x3d577ef7_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActiveControllerWhenMenuIsSuppressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsMenuSuppressed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsMenuSuppressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerConfigurationStatics, IRadialControllerConfigurationStatics_Vtbl, 0x79b6b0e5_069a_4486_a99d_8db772b9642f);
impl windows_core::RuntimeType for IRadialControllerConfigurationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerConfigurationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerConfigurationStatics2, IRadialControllerConfigurationStatics2_Vtbl, 0x53e08b17_e205_48d3_9caf_80ff47c4d7c7);
impl windows_core::RuntimeType for IRadialControllerConfigurationStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerConfigurationStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAppController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsAppControllerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAppControllerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerControlAcquiredEventArgs, IRadialControllerControlAcquiredEventArgs_Vtbl, 0x206aa439_e651_11e5_bf62_2c27d7404e85);
impl windows_core::RuntimeType for IRadialControllerControlAcquiredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerControlAcquiredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerControlAcquiredEventArgs2, IRadialControllerControlAcquiredEventArgs2_Vtbl, 0x3d577ef4_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerControlAcquiredEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerControlAcquiredEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerMenu, IRadialControllerMenu_Vtbl, 0x8506b35d_f640_4412_aba0_bad077e5ea8a);
impl windows_core::RuntimeType for IRadialControllerMenu {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerMenu_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetSelectedMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySelectPreviouslySelectedMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerMenuItem, IRadialControllerMenuItem_Vtbl, 0xc80fc98d_ad0b_4c9c_8f2f_136a2373a6ba);
impl windows_core::RuntimeType for IRadialControllerMenuItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerMenuItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Invoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerMenuItemStatics, IRadialControllerMenuItemStatics_Vtbl, 0x249e0887_d842_4524_9df8_e0d647edc887);
impl windows_core::RuntimeType for IRadialControllerMenuItemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerMenuItemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromIcon: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromIcon: usize,
    pub CreateFromKnownIcon: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, RadialControllerMenuKnownIcon, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerMenuItemStatics2, IRadialControllerMenuItemStatics2_Vtbl, 0x0cbb70be_7e3e_48bd_be04_2c7fcaa9c1ff);
impl windows_core::RuntimeType for IRadialControllerMenuItemStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerMenuItemStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromFontGlyph: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromFontGlyphWithUri: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerRotationChangedEventArgs, IRadialControllerRotationChangedEventArgs_Vtbl, 0x206aa435_e651_11e5_bf62_2c27d7404e85);
impl windows_core::RuntimeType for IRadialControllerRotationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerRotationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RotationDeltaInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerRotationChangedEventArgs2, IRadialControllerRotationChangedEventArgs2_Vtbl, 0x3d577eec_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerRotationChangedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerRotationChangedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerScreenContact, IRadialControllerScreenContact_Vtbl, 0x206aa434_e651_11e5_bf62_2c27d7404e85);
impl windows_core::RuntimeType for IRadialControllerScreenContact {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerScreenContact_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerScreenContactContinuedEventArgs, IRadialControllerScreenContactContinuedEventArgs_Vtbl, 0x206aa437_e651_11e5_bf62_2c27d7404e85);
impl windows_core::RuntimeType for IRadialControllerScreenContactContinuedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerScreenContactContinuedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerScreenContactContinuedEventArgs2, IRadialControllerScreenContactContinuedEventArgs2_Vtbl, 0x3d577ef1_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerScreenContactContinuedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerScreenContactContinuedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerScreenContactEndedEventArgs, IRadialControllerScreenContactEndedEventArgs_Vtbl, 0x3d577ef2_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerScreenContactEndedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerScreenContactEndedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerScreenContactStartedEventArgs, IRadialControllerScreenContactStartedEventArgs_Vtbl, 0x206aa436_e651_11e5_bf62_2c27d7404e85);
impl windows_core::RuntimeType for IRadialControllerScreenContactStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerScreenContactStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRadialControllerScreenContactStartedEventArgs2, IRadialControllerScreenContactStartedEventArgs2_Vtbl, 0x3d577ef0_3cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerScreenContactStartedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerScreenContactStartedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IRadialControllerStatics, IRadialControllerStatics_Vtbl, 0xfaded0b7_b84c_4894_87aa_8f25aa5f288b);
impl windows_core::RuntimeType for IRadialControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CreateForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRightTappedEventArgs, IRightTappedEventArgs_Vtbl, 0x4cbf40bd_af7a_4a36_9476_b1dce141709a);
impl windows_core::RuntimeType for IRightTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRightTappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRightTappedEventArgs2, IRightTappedEventArgs2_Vtbl, 0x61c7b7bb_9f57_5857_a33c_c58c3dfa959e);
impl windows_core::RuntimeType for IRightTappedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRightTappedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemButtonEventController, ISystemButtonEventController_Vtbl, 0x59b893a9_73bc_52b5_ba41_82511b2cb46c);
impl windows_core::RuntimeType for ISystemButtonEventController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemButtonEventController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SystemFunctionButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSystemFunctionButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SystemFunctionButtonReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSystemFunctionButtonReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SystemFunctionLockChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSystemFunctionLockChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SystemFunctionLockIndicatorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSystemFunctionLockIndicatorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemButtonEventControllerStatics, ISystemButtonEventControllerStatics_Vtbl, 0x632fb07b_20bd_5e15_af4a_00dbf2064ffa);
impl windows_core::RuntimeType for ISystemButtonEventControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemButtonEventControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForDispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForDispatcherQueue: usize,
}
windows_core::imp::define_interface!(ISystemFunctionButtonEventArgs, ISystemFunctionButtonEventArgs_Vtbl, 0x4833896f_80d1_5dd6_92a7_62a508ffef5a);
impl windows_core::RuntimeType for ISystemFunctionButtonEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemFunctionButtonEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemFunctionLockChangedEventArgs, ISystemFunctionLockChangedEventArgs_Vtbl, 0xcd040608_fcf9_585c_beab_f1d2eaf364ab);
impl windows_core::RuntimeType for ISystemFunctionLockChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemFunctionLockChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IsLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemFunctionLockIndicatorChangedEventArgs, ISystemFunctionLockIndicatorChangedEventArgs_Vtbl, 0xb212b94e_7a6f_58ae_b304_bae61d0371b9);
impl windows_core::RuntimeType for ISystemFunctionLockIndicatorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IsIndicatorOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITappedEventArgs, ITappedEventArgs_Vtbl, 0xcfa126e4_253a_4c3c_953b_395c37aed309);
impl windows_core::RuntimeType for ITappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub TapCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITappedEventArgs2, ITappedEventArgs2_Vtbl, 0x294388f2_177e_51d5_be56_ee0866fa968c);
impl windows_core::RuntimeType for ITappedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITappedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AttachableInputObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AttachableInputObject, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AttachableInputObject, super::super::Foundation::IClosable);
impl AttachableInputObject {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AttachableInputObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAttachableInputObject>();
}
unsafe impl windows_core::Interface for AttachableInputObject {
    type Vtable = IAttachableInputObject_Vtbl;
    const IID: windows_core::GUID = <IAttachableInputObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AttachableInputObject {
    const NAME: &'static str = "Windows.UI.Input.AttachableInputObject";
}
unsafe impl Send for AttachableInputObject {}
unsafe impl Sync for AttachableInputObject {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CrossSlidingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CrossSlidingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CrossSlidingEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CrossSlidingState(&self) -> windows_core::Result<CrossSlidingState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CrossSlidingState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICrossSlidingEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICrossSlidingEventArgs>();
}
unsafe impl windows_core::Interface for CrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICrossSlidingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.CrossSlidingEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DraggingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DraggingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DraggingEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DraggingState(&self) -> windows_core::Result<DraggingState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DraggingState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IDraggingEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DraggingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDraggingEventArgs>();
}
unsafe impl windows_core::Interface for DraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDraggingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.DraggingEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EdgeGesture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EdgeGesture, windows_core::IUnknown, windows_core::IInspectable);
impl EdgeGesture {
    pub fn Starting<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Starting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Completed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Canceled<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Canceled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCanceled(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCanceled)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<EdgeGesture> {
        Self::IEdgeGestureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IEdgeGestureStatics<R, F: FnOnce(&IEdgeGestureStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<EdgeGesture, IEdgeGestureStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for EdgeGesture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEdgeGesture>();
}
unsafe impl windows_core::Interface for EdgeGesture {
    type Vtable = IEdgeGesture_Vtbl;
    const IID: windows_core::GUID = <IEdgeGesture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EdgeGesture {
    const NAME: &'static str = "Windows.UI.Input.EdgeGesture";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EdgeGestureEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EdgeGestureEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EdgeGestureEventArgs {
    pub fn Kind(&self) -> windows_core::Result<EdgeGestureKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for EdgeGestureEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEdgeGestureEventArgs>();
}
unsafe impl windows_core::Interface for EdgeGestureEventArgs {
    type Vtable = IEdgeGestureEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEdgeGestureEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EdgeGestureEventArgs {
    const NAME: &'static str = "Windows.UI.Input.EdgeGestureEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GestureRecognizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GestureRecognizer, windows_core::IUnknown, windows_core::IInspectable);
impl GestureRecognizer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GestureRecognizer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GestureSettings(&self) -> windows_core::Result<GestureSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GestureSettings)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGestureSettings(&self, value: GestureSettings) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGestureSettings)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsInertial(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInertial)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShowGestureFeedback(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowGestureFeedback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowGestureFeedback(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShowGestureFeedback)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PivotCenter(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PivotCenter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPivotCenter(&self, value: super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPivotCenter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PivotRadius(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PivotRadius)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPivotRadius(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPivotRadius)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaTranslationDeceleration(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InertiaTranslationDeceleration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInertiaTranslationDeceleration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaRotationDeceleration(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InertiaRotationDeceleration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInertiaRotationDeceleration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaExpansionDeceleration(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InertiaExpansionDeceleration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInertiaExpansionDeceleration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaTranslationDisplacement(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InertiaTranslationDisplacement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInertiaTranslationDisplacement)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaRotationAngle(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InertiaRotationAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInertiaRotationAngle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InertiaExpansion(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InertiaExpansion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaExpansion(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInertiaExpansion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ManipulationExact(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationExact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetManipulationExact(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationExact)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CrossSlideThresholds(&self) -> windows_core::Result<CrossSlideThresholds> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CrossSlideThresholds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCrossSlideThresholds(&self, value: CrossSlideThresholds) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCrossSlideThresholds)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CrossSlideHorizontally(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CrossSlideHorizontally)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCrossSlideHorizontally)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CrossSlideExact(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CrossSlideExact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCrossSlideExact(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCrossSlideExact)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoProcessInertia(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoProcessInertia)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoProcessInertia(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoProcessInertia)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MouseWheelParameters(&self) -> windows_core::Result<MouseWheelParameters> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseWheelParameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanBeDoubleTap<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeDoubleTap)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn ProcessDownEvent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessDownEvent)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProcessMoveEvents<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<PointerPoint>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessMoveEvents)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProcessUpEvent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessUpEvent)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProcessMouseWheelEvent<P0>(&self, value: P0, isshiftkeydown: bool, iscontrolkeydown: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessMouseWheelEvent)(windows_core::Interface::as_raw(this), value.param().abi(), isshiftkeydown, iscontrolkeydown).ok() }
    }
    pub fn ProcessInertia(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessInertia)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CompleteGesture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CompleteGesture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Tapped<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTapped(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTapped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RightTapped<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRightTapped(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRightTapped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Holding<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHolding(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHolding)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Dragging<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dragging)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDragging(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDragging)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationStarted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationInertiaStarting<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ManipulationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveManipulationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveManipulationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CrossSliding<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CrossSliding)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCrossSliding(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCrossSliding)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TapMinContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TapMinContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTapMinContactCount(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTapMinContactCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TapMaxContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TapMaxContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTapMaxContactCount(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTapMaxContactCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldMinContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldMinContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHoldMinContactCount(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHoldMinContactCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldMaxContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldMaxContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHoldMaxContactCount(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHoldMaxContactCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldRadius(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldRadius)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHoldRadius(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHoldRadius)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoldStartDelay(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldStartDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHoldStartDelay(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHoldStartDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslationMinContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranslationMinContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslationMinContactCount(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslationMinContactCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TranslationMaxContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TranslationMaxContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslationMaxContactCount(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGestureRecognizer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslationMaxContactCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for GestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGestureRecognizer>();
}
unsafe impl windows_core::Interface for GestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: windows_core::GUID = <IGestureRecognizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Windows.UI.Input.GestureRecognizer";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HoldingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HoldingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HoldingEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HoldingState(&self) -> windows_core::Result<HoldingState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoldingState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IHoldingEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHoldingEventArgs>();
}
unsafe impl windows_core::Interface for HoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHoldingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.HoldingEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InputActivationListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputActivationListener, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputActivationListener, super::super::Foundation::IClosable, AttachableInputObject);
impl InputActivationListener {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn State(&self) -> windows_core::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InputActivationChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputActivationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputActivationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputActivationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for InputActivationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputActivationListener>();
}
unsafe impl windows_core::Interface for InputActivationListener {
    type Vtable = IInputActivationListener_Vtbl;
    const IID: windows_core::GUID = <IInputActivationListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputActivationListener {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListener";
}
unsafe impl Send for InputActivationListener {}
unsafe impl Sync for InputActivationListener {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InputActivationListenerActivationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputActivationListenerActivationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InputActivationListenerActivationChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<InputActivationState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputActivationListenerActivationChangedEventArgs>();
}
unsafe impl windows_core::Interface for InputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IInputActivationListenerActivationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.InputActivationListenerActivationChangedEventArgs";
}
unsafe impl Send for InputActivationListenerActivationChangedEventArgs {}
unsafe impl Sync for InputActivationListenerActivationChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KeyboardDeliveryInterceptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyboardDeliveryInterceptor, windows_core::IUnknown, windows_core::IInspectable);
impl KeyboardDeliveryInterceptor {
    pub fn IsInterceptionEnabledWhenInForeground(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInterceptionEnabledWhenInForeground)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsInterceptionEnabledWhenInForeground(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsInterceptionEnabledWhenInForeground)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyDown)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyUp)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<KeyboardDeliveryInterceptor> {
        Self::IKeyboardDeliveryInterceptorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKeyboardDeliveryInterceptorStatics<R, F: FnOnce(&IKeyboardDeliveryInterceptorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyboardDeliveryInterceptor, IKeyboardDeliveryInterceptorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for KeyboardDeliveryInterceptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyboardDeliveryInterceptor>();
}
unsafe impl windows_core::Interface for KeyboardDeliveryInterceptor {
    type Vtable = IKeyboardDeliveryInterceptor_Vtbl;
    const IID: windows_core::GUID = <IKeyboardDeliveryInterceptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyboardDeliveryInterceptor {
    const NAME: &'static str = "Windows.UI.Input.KeyboardDeliveryInterceptor";
}
unsafe impl Send for KeyboardDeliveryInterceptor {}
unsafe impl Sync for KeyboardDeliveryInterceptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ManipulationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationCompletedEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cumulative)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Velocities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IManipulationCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IManipulationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationCompletedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ManipulationInertiaStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationInertiaStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationInertiaStartingEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delta(&self) -> windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delta)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cumulative)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Velocities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IManipulationInertiaStartingEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationInertiaStartingEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IManipulationInertiaStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationInertiaStartingEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ManipulationStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationStartedEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cumulative)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IManipulationStartedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationStartedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IManipulationStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationStartedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ManipulationUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationUpdatedEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delta(&self) -> windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delta)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cumulative)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Velocities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IManipulationUpdatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IManipulationUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.ManipulationUpdatedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MouseWheelParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MouseWheelParameters, windows_core::IUnknown, windows_core::IInspectable);
impl MouseWheelParameters {
    pub fn CharTranslation(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharTranslation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharTranslation(&self, value: super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCharTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeltaScale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeltaScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeltaScale(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeltaScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeltaRotationAngle(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeltaRotationAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeltaRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeltaRotationAngle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PageTranslation(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageTranslation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPageTranslation(&self, value: super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPageTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for MouseWheelParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMouseWheelParameters>();
}
unsafe impl windows_core::Interface for MouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
    const IID: windows_core::GUID = <IMouseWheelParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Windows.UI.Input.MouseWheelParameters";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerPoint, windows_core::IUnknown, windows_core::IInspectable);
impl PointerPoint {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDevice(&self) -> windows_core::Result<super::super::Devices::Input::PointerDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RawPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FrameId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInContact(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInContact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Properties(&self) -> windows_core::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentPoint(pointerid: u32) -> windows_core::Result<PointerPoint> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPoint)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(pointerid: u32) -> windows_core::Result<super::super::Foundation::Collections::IVector<PointerPoint>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntermediatePoints)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCurrentPointTransformed<P0>(pointerid: u32, transform: P0) -> windows_core::Result<PointerPoint>
    where
        P0: windows_core::Param<IPointerPointTransform>,
    {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPointTransformed)(windows_core::Interface::as_raw(this), pointerid, transform.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePointsTransformed<P0>(pointerid: u32, transform: P0) -> windows_core::Result<super::super::Foundation::Collections::IVector<PointerPoint>>
    where
        P0: windows_core::Param<IPointerPointTransform>,
    {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntermediatePointsTransformed)(windows_core::Interface::as_raw(this), pointerid, transform.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerPoint, IPointerPointStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerPoint>();
}
unsafe impl windows_core::Interface for PointerPoint {
    type Vtable = IPointerPoint_Vtbl;
    const IID: windows_core::GUID = <IPointerPoint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Windows.UI.Input.PointerPoint";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerPointProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerPointProperties, windows_core::IUnknown, windows_core::IInspectable);
impl PointerPointProperties {
    pub fn Pressure(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInverted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInverted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEraser(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEraser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Orientation(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XTilt(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XTilt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn YTilt(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YTilt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Twist(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Twist)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactRectRaw(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactRectRaw)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TouchConfidence(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchConfidence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLeftButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLeftButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRightButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMiddleButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMiddleButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MouseWheelDelta(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseWheelDelta)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHorizontalMouseWheel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHorizontalMouseWheel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPrimary(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrimary)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInRange(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBarrelButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBarrelButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsXButton1Pressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsXButton1Pressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsXButton2Pressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsXButton2Pressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerUpdateKind(&self) -> windows_core::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerUpdateKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasUsage(&self, usagepage: u32, usageid: u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasUsage)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).map(|| result__)
        }
    }
    pub fn GetUsageValue(&self, usagepage: u32, usageid: u32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUsageValue)(windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).map(|| result__)
        }
    }
    pub fn ZDistance(&self) -> windows_core::Result<super::super::Foundation::IReference<f32>> {
        let this = &windows_core::Interface::cast::<IPointerPointProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZDistance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerPointProperties>();
}
unsafe impl windows_core::Interface for PointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
    const IID: windows_core::GUID = <IPointerPointProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Windows.UI.Input.PointerPointProperties";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerVisualizationSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerVisualizationSettings, windows_core::IUnknown, windows_core::IInspectable);
impl PointerVisualizationSettings {
    pub fn SetIsContactFeedbackEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsContactFeedbackEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsContactFeedbackEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsContactFeedbackEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsBarrelButtonFeedbackEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsBarrelButtonFeedbackEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsBarrelButtonFeedbackEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBarrelButtonFeedbackEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<PointerVisualizationSettings> {
        Self::IPointerVisualizationSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPointerVisualizationSettingsStatics<R, F: FnOnce(&IPointerVisualizationSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerVisualizationSettings, IPointerVisualizationSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerVisualizationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerVisualizationSettings>();
}
unsafe impl windows_core::Interface for PointerVisualizationSettings {
    type Vtable = IPointerVisualizationSettings_Vtbl;
    const IID: windows_core::GUID = <IPointerVisualizationSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerVisualizationSettings {
    const NAME: &'static str = "Windows.UI.Input.PointerVisualizationSettings";
}
unsafe impl Send for PointerVisualizationSettings {}
unsafe impl Sync for PointerVisualizationSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialController, windows_core::IUnknown, windows_core::IInspectable);
impl RadialController {
    pub fn Menu(&self) -> windows_core::Result<RadialControllerMenu> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Menu)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RotationResolutionInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationResolutionInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationResolutionInDegrees(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRotationResolutionInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UseAutomaticHapticFeedback(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseAutomaticHapticFeedback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseAutomaticHapticFeedback(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUseAutomaticHapticFeedback)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScreenContactStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenContactStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScreenContactStarted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScreenContactStarted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ScreenContactEnded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenContactEnded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScreenContactEnded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScreenContactEnded)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ScreenContactContinued<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenContactContinued)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScreenContactContinued(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScreenContactContinued)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ControlLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveControlLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveControlLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn RotationChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRotationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRotationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ButtonClicked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonClicked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveButtonClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveButtonClicked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ControlAcquired<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlAcquired)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveControlAcquired(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveControlAcquired)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ButtonPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveButtonPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveButtonPressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ButtonHolding<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonHolding)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveButtonHolding(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveButtonHolding)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ButtonReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveButtonReleased(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRadialController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveButtonReleased)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CreateForCurrentView() -> windows_core::Result<RadialController> {
        Self::IRadialControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerStatics<R, F: FnOnce(&IRadialControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadialController, IRadialControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadialController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialController>();
}
unsafe impl windows_core::Interface for RadialController {
    type Vtable = IRadialController_Vtbl;
    const IID: windows_core::GUID = <IRadialController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialController {
    const NAME: &'static str = "Windows.UI.Input.RadialController";
}
unsafe impl Send for RadialController {}
unsafe impl Sync for RadialController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerButtonClickedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerButtonClickedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerButtonClickedEventArgs {
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &windows_core::Interface::cast::<IRadialControllerButtonClickedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerButtonClickedEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerButtonClickedEventArgs {
    type Vtable = IRadialControllerButtonClickedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerButtonClickedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerButtonClickedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonClickedEventArgs";
}
unsafe impl Send for RadialControllerButtonClickedEventArgs {}
unsafe impl Sync for RadialControllerButtonClickedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerButtonHoldingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerButtonHoldingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerButtonHoldingEventArgs {
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerButtonHoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerButtonHoldingEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerButtonHoldingEventArgs {
    type Vtable = IRadialControllerButtonHoldingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerButtonHoldingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerButtonHoldingEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonHoldingEventArgs";
}
unsafe impl Send for RadialControllerButtonHoldingEventArgs {}
unsafe impl Sync for RadialControllerButtonHoldingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerButtonPressedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerButtonPressedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerButtonPressedEventArgs {
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerButtonPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerButtonPressedEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerButtonPressedEventArgs {
    type Vtable = IRadialControllerButtonPressedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerButtonPressedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerButtonPressedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonPressedEventArgs";
}
unsafe impl Send for RadialControllerButtonPressedEventArgs {}
unsafe impl Sync for RadialControllerButtonPressedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerButtonReleasedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerButtonReleasedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerButtonReleasedEventArgs {
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerButtonReleasedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerButtonReleasedEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerButtonReleasedEventArgs {
    type Vtable = IRadialControllerButtonReleasedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerButtonReleasedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerButtonReleasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerButtonReleasedEventArgs";
}
unsafe impl Send for RadialControllerButtonReleasedEventArgs {}
unsafe impl Sync for RadialControllerButtonReleasedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerConfiguration {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDefaultMenuItems<P0>(&self, buttons: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultMenuItems)(windows_core::Interface::as_raw(this), buttons.param().abi()).ok() }
    }
    pub fn ResetToDefaultMenuItems(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ResetToDefaultMenuItems)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TrySelectDefaultMenuItem(&self, r#type: RadialControllerSystemMenuItemKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySelectDefaultMenuItem)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        }
    }
    pub fn SetActiveControllerWhenMenuIsSuppressed<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RadialController>,
    {
        let this = &windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetActiveControllerWhenMenuIsSuppressed)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ActiveControllerWhenMenuIsSuppressed(&self) -> windows_core::Result<RadialController> {
        let this = &windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActiveControllerWhenMenuIsSuppressed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsMenuSuppressed(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsMenuSuppressed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMenuSuppressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRadialControllerConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMenuSuppressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<RadialControllerConfiguration> {
        Self::IRadialControllerConfigurationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetAppController<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RadialController>,
    {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetAppController)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    pub fn AppController() -> windows_core::Result<RadialController> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetIsAppControllerEnabled(value: bool) -> windows_core::Result<()> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetIsAppControllerEnabled)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsAppControllerEnabled() -> windows_core::Result<bool> {
        Self::IRadialControllerConfigurationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppControllerEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerConfigurationStatics<R, F: FnOnce(&IRadialControllerConfigurationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRadialControllerConfigurationStatics2<R, F: FnOnce(&IRadialControllerConfigurationStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadialControllerConfiguration, IRadialControllerConfigurationStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadialControllerConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerConfiguration>();
}
unsafe impl windows_core::Interface for RadialControllerConfiguration {
    type Vtable = IRadialControllerConfiguration_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerConfiguration {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerConfiguration";
}
unsafe impl Send for RadialControllerConfiguration {}
unsafe impl Sync for RadialControllerConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerControlAcquiredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerControlAcquiredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerControlAcquiredEventArgs {
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsButtonPressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &windows_core::Interface::cast::<IRadialControllerControlAcquiredEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerControlAcquiredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerControlAcquiredEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerControlAcquiredEventArgs {
    type Vtable = IRadialControllerControlAcquiredEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerControlAcquiredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerControlAcquiredEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerControlAcquiredEventArgs";
}
unsafe impl Send for RadialControllerControlAcquiredEventArgs {}
unsafe impl Sync for RadialControllerControlAcquiredEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerMenu(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerMenu, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerMenu {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    pub fn GetSelectedMenuItem(&self) -> windows_core::Result<RadialControllerMenuItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSelectedMenuItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectMenuItem<P0>(&self, menuitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RadialControllerMenuItem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SelectMenuItem)(windows_core::Interface::as_raw(this), menuitem.param().abi()).ok() }
    }
    pub fn TrySelectPreviouslySelectedMenuItem(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySelectPreviouslySelectedMenuItem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RadialControllerMenu {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerMenu>();
}
unsafe impl windows_core::Interface for RadialControllerMenu {
    type Vtable = IRadialControllerMenu_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerMenu as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerMenu {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenu";
}
unsafe impl Send for RadialControllerMenu {}
unsafe impl Sync for RadialControllerMenu {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerMenuItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerMenuItem, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerMenuItem {
    pub fn DisplayText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Invoked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInvoked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInvoked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromIcon<P0>(displaytext: &windows_core::HSTRING, icon: P0) -> windows_core::Result<RadialControllerMenuItem>
    where
        P0: windows_core::Param<super::super::Storage::Streams::RandomAccessStreamReference>,
    {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIcon)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displaytext), icon.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromKnownIcon(displaytext: &windows_core::HSTRING, value: RadialControllerMenuKnownIcon) -> windows_core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromKnownIcon)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displaytext), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromFontGlyph(displaytext: &windows_core::HSTRING, glyph: &windows_core::HSTRING, fontfamily: &windows_core::HSTRING) -> windows_core::Result<RadialControllerMenuItem> {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromFontGlyph)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displaytext), core::mem::transmute_copy(glyph), core::mem::transmute_copy(fontfamily), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromFontGlyphWithUri<P0>(displaytext: &windows_core::HSTRING, glyph: &windows_core::HSTRING, fontfamily: &windows_core::HSTRING, fonturi: P0) -> windows_core::Result<RadialControllerMenuItem>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IRadialControllerMenuItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromFontGlyphWithUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displaytext), core::mem::transmute_copy(glyph), core::mem::transmute_copy(fontfamily), fonturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerMenuItemStatics<R, F: FnOnce(&IRadialControllerMenuItemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRadialControllerMenuItemStatics2<R, F: FnOnce(&IRadialControllerMenuItemStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadialControllerMenuItem, IRadialControllerMenuItemStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadialControllerMenuItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerMenuItem>();
}
unsafe impl windows_core::Interface for RadialControllerMenuItem {
    type Vtable = IRadialControllerMenuItem_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerMenuItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerMenuItem {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerMenuItem";
}
unsafe impl Send for RadialControllerMenuItem {}
unsafe impl Sync for RadialControllerMenuItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerRotationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerRotationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerRotationChangedEventArgs {
    pub fn RotationDeltaInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationDeltaInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsButtonPressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &windows_core::Interface::cast::<IRadialControllerRotationChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerRotationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerRotationChangedEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerRotationChangedEventArgs {
    type Vtable = IRadialControllerRotationChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerRotationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerRotationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerRotationChangedEventArgs";
}
unsafe impl Send for RadialControllerRotationChangedEventArgs {}
unsafe impl Sync for RadialControllerRotationChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerScreenContact(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerScreenContact, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerScreenContact {
    pub fn Bounds(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RadialControllerScreenContact {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerScreenContact>();
}
unsafe impl windows_core::Interface for RadialControllerScreenContact {
    type Vtable = IRadialControllerScreenContact_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerScreenContact as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerScreenContact {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContact";
}
unsafe impl Send for RadialControllerScreenContact {}
unsafe impl Sync for RadialControllerScreenContact {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerScreenContactContinuedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerScreenContactContinuedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerScreenContactContinuedEventArgs {
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsButtonPressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &windows_core::Interface::cast::<IRadialControllerScreenContactContinuedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerScreenContactContinuedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerScreenContactContinuedEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerScreenContactContinuedEventArgs {
    type Vtable = IRadialControllerScreenContactContinuedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerScreenContactContinuedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerScreenContactContinuedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactContinuedEventArgs";
}
unsafe impl Send for RadialControllerScreenContactContinuedEventArgs {}
unsafe impl Sync for RadialControllerScreenContactContinuedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerScreenContactEndedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerScreenContactEndedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerScreenContactEndedEventArgs {
    pub fn IsButtonPressed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerScreenContactEndedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerScreenContactEndedEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerScreenContactEndedEventArgs {
    type Vtable = IRadialControllerScreenContactEndedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerScreenContactEndedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerScreenContactEndedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactEndedEventArgs";
}
unsafe impl Send for RadialControllerScreenContactEndedEventArgs {}
unsafe impl Sync for RadialControllerScreenContactEndedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RadialControllerScreenContactStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerScreenContactStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerScreenContactStartedEventArgs {
    pub fn Contact(&self) -> windows_core::Result<RadialControllerScreenContact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsButtonPressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsButtonPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::super::Devices::Haptics::SimpleHapticsController> {
        let this = &windows_core::Interface::cast::<IRadialControllerScreenContactStartedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RadialControllerScreenContactStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerScreenContactStartedEventArgs>();
}
unsafe impl windows_core::Interface for RadialControllerScreenContactStartedEventArgs {
    type Vtable = IRadialControllerScreenContactStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerScreenContactStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerScreenContactStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RadialControllerScreenContactStartedEventArgs";
}
unsafe impl Send for RadialControllerScreenContactStartedEventArgs {}
unsafe impl Sync for RadialControllerScreenContactStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RightTappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RightTappedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RightTappedEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IRightTappedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRightTappedEventArgs>();
}
unsafe impl windows_core::Interface for RightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRightTappedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.RightTappedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemButtonEventController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemButtonEventController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SystemButtonEventController, super::super::Foundation::IClosable, AttachableInputObject);
impl SystemButtonEventController {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SystemFunctionButtonPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemFunctionButtonPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSystemFunctionButtonPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSystemFunctionButtonPressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SystemFunctionButtonReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemFunctionButtonReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSystemFunctionButtonReleased(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSystemFunctionButtonReleased)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SystemFunctionLockChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemFunctionLockChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSystemFunctionLockChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSystemFunctionLockChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SystemFunctionLockIndicatorChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemFunctionLockIndicatorChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSystemFunctionLockIndicatorChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSystemFunctionLockIndicatorChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn CreateForDispatcherQueue<P0>(queue: P0) -> windows_core::Result<SystemButtonEventController>
    where
        P0: windows_core::Param<super::super::System::DispatcherQueue>,
    {
        Self::ISystemButtonEventControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForDispatcherQueue)(windows_core::Interface::as_raw(this), queue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISystemButtonEventControllerStatics<R, F: FnOnce(&ISystemButtonEventControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemButtonEventController, ISystemButtonEventControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemButtonEventController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemButtonEventController>();
}
unsafe impl windows_core::Interface for SystemButtonEventController {
    type Vtable = ISystemButtonEventController_Vtbl;
    const IID: windows_core::GUID = <ISystemButtonEventController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemButtonEventController {
    const NAME: &'static str = "Windows.UI.Input.SystemButtonEventController";
}
unsafe impl Send for SystemButtonEventController {}
unsafe impl Sync for SystemButtonEventController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemFunctionButtonEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemFunctionButtonEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SystemFunctionButtonEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SystemFunctionButtonEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemFunctionButtonEventArgs>();
}
unsafe impl windows_core::Interface for SystemFunctionButtonEventArgs {
    type Vtable = ISystemFunctionButtonEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISystemFunctionButtonEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemFunctionButtonEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionButtonEventArgs";
}
unsafe impl Send for SystemFunctionButtonEventArgs {}
unsafe impl Sync for SystemFunctionButtonEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemFunctionLockChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemFunctionLockChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SystemFunctionLockChangedEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLocked(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLocked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SystemFunctionLockChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemFunctionLockChangedEventArgs>();
}
unsafe impl windows_core::Interface for SystemFunctionLockChangedEventArgs {
    type Vtable = ISystemFunctionLockChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISystemFunctionLockChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemFunctionLockChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockChangedEventArgs";
}
unsafe impl Send for SystemFunctionLockChangedEventArgs {}
unsafe impl Sync for SystemFunctionLockChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemFunctionLockIndicatorChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemFunctionLockIndicatorChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SystemFunctionLockIndicatorChangedEventArgs {
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsIndicatorOn(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIndicatorOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SystemFunctionLockIndicatorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemFunctionLockIndicatorChangedEventArgs>();
}
unsafe impl windows_core::Interface for SystemFunctionLockIndicatorChangedEventArgs {
    type Vtable = ISystemFunctionLockIndicatorChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISystemFunctionLockIndicatorChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemFunctionLockIndicatorChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.SystemFunctionLockIndicatorChangedEventArgs";
}
unsafe impl Send for SystemFunctionLockIndicatorChangedEventArgs {}
unsafe impl Sync for SystemFunctionLockIndicatorChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TappedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TappedEventArgs {
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TapCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TapCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ITappedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITappedEventArgs>();
}
unsafe impl windows_core::Interface for TappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITappedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.TappedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0i32);
    pub const Dragging: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
    pub const SelectSpeedBumping: Self = Self(3i32);
    pub const SpeedBumping: Self = Self(4i32);
    pub const Rearranging: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl windows_core::TypeKind for CrossSlidingState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CrossSlidingState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CrossSlidingState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CrossSlidingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.CrossSlidingState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl windows_core::TypeKind for DraggingState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DraggingState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DraggingState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DraggingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.DraggingState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl windows_core::TypeKind for EdgeGestureKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EdgeGestureKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EdgeGestureKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for EdgeGestureKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.EdgeGestureKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl windows_core::TypeKind for GazeInputAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GazeInputAccessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GazeInputAccessStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GazeInputAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GazeInputAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const HoldWithMouse: Self = Self(8u32);
    pub const RightTap: Self = Self(16u32);
    pub const Drag: Self = Self(32u32);
    pub const ManipulationTranslateX: Self = Self(64u32);
    pub const ManipulationTranslateY: Self = Self(128u32);
    pub const ManipulationTranslateRailsX: Self = Self(256u32);
    pub const ManipulationTranslateRailsY: Self = Self(512u32);
    pub const ManipulationRotate: Self = Self(1024u32);
    pub const ManipulationScale: Self = Self(2048u32);
    pub const ManipulationTranslateInertia: Self = Self(4096u32);
    pub const ManipulationRotateInertia: Self = Self(8192u32);
    pub const ManipulationScaleInertia: Self = Self(16384u32);
    pub const CrossSlide: Self = Self(32768u32);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536u32);
}
impl windows_core::TypeKind for GestureSettings {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GestureSettings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GestureSettings").field(&self.0).finish()
    }
}
impl GestureSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for GestureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.GestureSettings;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl windows_core::TypeKind for HoldingState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HoldingState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HoldingState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HoldingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.HoldingState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl windows_core::TypeKind for InputActivationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InputActivationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InputActivationState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InputActivationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.InputActivationState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
impl windows_core::TypeKind for PointerUpdateKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PointerUpdateKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PointerUpdateKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.PointerUpdateKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RadialControllerMenuKnownIcon(pub i32);
impl RadialControllerMenuKnownIcon {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
    pub const Ruler: Self = Self(5i32);
    pub const InkColor: Self = Self(6i32);
    pub const InkThickness: Self = Self(7i32);
    pub const PenType: Self = Self(8i32);
}
impl windows_core::TypeKind for RadialControllerMenuKnownIcon {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RadialControllerMenuKnownIcon {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RadialControllerMenuKnownIcon").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RadialControllerMenuKnownIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerMenuKnownIcon;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RadialControllerSystemMenuItemKind(pub i32);
impl RadialControllerSystemMenuItemKind {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
}
impl windows_core::TypeKind for RadialControllerSystemMenuItemKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RadialControllerSystemMenuItemKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RadialControllerSystemMenuItemKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RadialControllerSystemMenuItemKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.RadialControllerSystemMenuItemKind;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl windows_core::TypeKind for CrossSlideThresholds {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)");
}
impl Default for CrossSlideThresholds {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ManipulationDelta {
    pub Translation: super::super::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
impl windows_core::TypeKind for ManipulationDelta {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ManipulationDelta {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)");
}
impl Default for ManipulationDelta {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ManipulationVelocities {
    pub Linear: super::super::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
impl windows_core::TypeKind for ManipulationVelocities {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ManipulationVelocities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)");
}
impl Default for ManipulationVelocities {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
