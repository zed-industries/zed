#[cfg(feature = "UI_Input_Inking_Analysis")]
pub mod Analysis;
#[cfg(feature = "UI_Input_Inking_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking_Preview")]
pub mod Preview;
windows_core::imp::define_interface!(IInkDrawingAttributes, IInkDrawingAttributes_Vtbl, 0x97a2176c_6774_48ad_84f0_48f5a9be74f9);
impl windows_core::RuntimeType for IInkDrawingAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkDrawingAttributes_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Color) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Color) -> windows_core::HRESULT,
    pub PenTip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PenTipShape) -> windows_core::HRESULT,
    pub SetPenTip: unsafe extern "system" fn(*mut core::ffi::c_void, PenTipShape) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Size) -> windows_core::HRESULT,
    pub IgnorePressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIgnorePressure: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub FitToCurve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetFitToCurve: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkDrawingAttributes2, IInkDrawingAttributes2_Vtbl, 0x7cab6508_8ec4_42fd_a5a5_e4b7d1d5316d);
impl windows_core::RuntimeType for IInkDrawingAttributes2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkDrawingAttributes2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PenTipTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PenTipTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPenTipTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPenTipTransform: usize,
    pub DrawAsHighlighter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDrawAsHighlighter: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkDrawingAttributes3, IInkDrawingAttributes3_Vtbl, 0x72020002_7d5b_4690_8af4_e664cbe2b74f);
impl windows_core::RuntimeType for IInkDrawingAttributes3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkDrawingAttributes3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkDrawingAttributesKind) -> windows_core::HRESULT,
    pub PencilProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkDrawingAttributes4, IInkDrawingAttributes4_Vtbl, 0xef65dc25_9f19_456d_91a3_bc3a3d91c5fb);
impl windows_core::RuntimeType for IInkDrawingAttributes4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkDrawingAttributes4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IgnoreTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIgnoreTilt: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkDrawingAttributes5, IInkDrawingAttributes5_Vtbl, 0xd11aa0bb_0775_4852_ae64_41143a7ae6c9);
impl windows_core::RuntimeType for IInkDrawingAttributes5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkDrawingAttributes5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ModelerAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkDrawingAttributesPencilProperties, IInkDrawingAttributesPencilProperties_Vtbl, 0x4f2534cb_2d86_41bb_b0e8_e4c2a0253c52);
impl windows_core::RuntimeType for IInkDrawingAttributesPencilProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkDrawingAttributesPencilProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Opacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkDrawingAttributesStatics, IInkDrawingAttributesStatics_Vtbl, 0xf731e03f_1a65_4862_96cb_6e1665e17f6d);
impl windows_core::RuntimeType for IInkDrawingAttributesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkDrawingAttributesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForPencil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkInputConfiguration, IInkInputConfiguration_Vtbl, 0x93a68dc4_0b7b_49d7_b34f_9901e524dcf2);
impl windows_core::RuntimeType for IInkInputConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkInputConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPrimaryBarrelButtonInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPrimaryBarrelButtonInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsEraserInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEraserInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkInputConfiguration2, IInkInputConfiguration2_Vtbl, 0x6ac2272e_81b4_5cc4_a36d_d057c387dfda);
impl windows_core::RuntimeType for IInkInputConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkInputConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPenHapticFeedbackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPenHapticFeedbackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkInputProcessingConfiguration, IInkInputProcessingConfiguration_Vtbl, 0x2778d85e_33ca_4b06_a6d3_ac3945116d37);
impl windows_core::RuntimeType for IInkInputProcessingConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkInputProcessingConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkInputProcessingMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, InkInputProcessingMode) -> windows_core::HRESULT,
    pub RightDragAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkInputRightDragAction) -> windows_core::HRESULT,
    pub SetRightDragAction: unsafe extern "system" fn(*mut core::ffi::c_void, InkInputRightDragAction) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkManager, IInkManager_Vtbl, 0x4744737d_671b_4163_9c95_4e8d7a035fe1);
impl windows_core::RuntimeType for IInkManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkManipulationMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, InkManipulationMode) -> windows_core::HRESULT,
    pub ProcessPointerDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessPointerUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessPointerUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetDefaultDrawingAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecognizeAsync2: unsafe extern "system" fn(*mut core::ffi::c_void, InkRecognitionTarget, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecognizeAsync2: usize,
}
windows_core::imp::define_interface!(IInkModelerAttributes, IInkModelerAttributes_Vtbl, 0xbad31f27_0cd9_4bfd_b6f3_9e03ba8d7454);
impl windows_core::RuntimeType for IInkModelerAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkModelerAttributes_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PredictionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetPredictionTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ScalingFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetScalingFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkModelerAttributes2, IInkModelerAttributes2_Vtbl, 0x86d1d09a_4ef8_5e25_b7bc_b65424f16bb3);
impl windows_core::RuntimeType for IInkModelerAttributes2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkModelerAttributes2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UseVelocityBasedPressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseVelocityBasedPressure: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPoint, IInkPoint_Vtbl, 0x9f87272b_858c_46a5_9b41_d195970459fd);
impl windows_core::RuntimeType for IInkPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPoint2, IInkPoint2_Vtbl, 0xfba9c3f7_ae56_4d5c_bd2f_0ac45f5e4af9);
impl windows_core::RuntimeType for IInkPoint2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPoint2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TiltX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub TiltY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPointFactory, IInkPointFactory_Vtbl, 0x29e5d51c_c98f_405d_9f3b_e53e31068d4d);
impl core::ops::Deref for IInkPointFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkPointFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IInkPointFactory {
    pub fn CreateInkPoint(&self, position: super::super::super::Foundation::Point, pressure: f32) -> windows_core::Result<InkPoint> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInkPoint)(windows_core::Interface::as_raw(this), position, pressure, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IInkPointFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPointFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInkPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPointFactory2, IInkPointFactory2_Vtbl, 0xe0145e85_daff_45f2_ad69_050d8256a209);
impl windows_core::RuntimeType for IInkPointFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPointFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInkPointWithTiltAndTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, f32, f32, f32, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenter, IInkPresenter_Vtbl, 0xa69b70e2_887b_458f_b173_4fe4438930a3);
impl windows_core::RuntimeType for IInkPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub InputDeviceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Core::CoreInputDeviceTypes) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    InputDeviceTypes: usize,
    #[cfg(feature = "UI_Core")]
    pub SetInputDeviceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Core::CoreInputDeviceTypes) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetInputDeviceTypes: usize,
    pub UnprocessedInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StrokeInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InputProcessingConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StrokeContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStrokeContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyDefaultDrawingAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateDefaultDrawingAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateCustomDrying: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPredefinedConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, InkPresenterPredefinedConfiguration) -> windows_core::HRESULT,
    pub StrokesCollected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStrokesCollected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StrokesErased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStrokesErased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenter2, IInkPresenter2_Vtbl, 0xcf53e612_9a34_11e6_9f33_a24fc0d9649c);
impl windows_core::RuntimeType for IInkPresenter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HighContrastAdjustment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkHighContrastAdjustment) -> windows_core::HRESULT,
    pub SetHighContrastAdjustment: unsafe extern "system" fn(*mut core::ffi::c_void, InkHighContrastAdjustment) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenter3, IInkPresenter3_Vtbl, 0x51e1ce89_d37d_4a90_83fc_7f5e9dfbf217);
impl windows_core::RuntimeType for IInkPresenter3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenter3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenterProtractor, IInkPresenterProtractor_Vtbl, 0x7de3f2aa_ef6c_4e91_a73b_5b70d56fbd17);
impl windows_core::RuntimeType for IInkPresenterProtractor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenterProtractor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AreTickMarksVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreTickMarksVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AreRaysVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreRaysVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCenterMarkerVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCenterMarkerVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAngleReadoutVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAngleReadoutVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsResizable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsResizable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Radius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub AccentColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Color) -> windows_core::HRESULT,
    pub SetAccentColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Color) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenterProtractorFactory, IInkPresenterProtractorFactory_Vtbl, 0x320103c9_68fa_47e9_8127_8370711fc46c);
impl windows_core::RuntimeType for IInkPresenterProtractorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenterProtractorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenterRuler, IInkPresenterRuler_Vtbl, 0x6cda7d5a_dec7_4dd7_877a_2133f183d48a);
impl windows_core::RuntimeType for IInkPresenterRuler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenterRuler_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenterRuler2, IInkPresenterRuler2_Vtbl, 0x45130dc1_bc61_44d4_a423_54712ae671c4);
impl windows_core::RuntimeType for IInkPresenterRuler2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenterRuler2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AreTickMarksVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreTickMarksVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsCompassVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCompassVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenterRulerFactory, IInkPresenterRulerFactory_Vtbl, 0x34361beb_9001_4a4b_a690_69dbaf63e501);
impl core::ops::Deref for IInkPresenterRulerFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkPresenterRulerFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IInkPresenterRulerFactory {
    pub fn Create<P0>(&self, inkpresenter: P0) -> windows_core::Result<InkPresenterRuler>
    where
        P0: windows_core::Param<InkPresenter>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), inkpresenter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IInkPresenterRulerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenterRulerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkPresenterStencil, IInkPresenterStencil_Vtbl, 0x30d12d6d_3e06_4d02_b116_277fb5d8addc);
impl core::ops::Deref for IInkPresenterStencil {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkPresenterStencil, windows_core::IUnknown, windows_core::IInspectable);
impl IInkPresenterStencil {
    pub fn Kind(&self) -> windows_core::Result<InkPresenterStencilKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForegroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for IInkPresenterStencil {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkPresenterStencil_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InkPresenterStencilKind) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Color) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Color) -> windows_core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Color) -> windows_core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Color) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Transform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Transform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
}
windows_core::imp::define_interface!(IInkRecognitionResult, IInkRecognitionResult_Vtbl, 0x36461a94_5068_40ef_8a05_2c2fb60908a2);
impl windows_core::RuntimeType for IInkRecognitionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkRecognitionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextCandidates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextCandidates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokes: usize,
}
windows_core::imp::define_interface!(IInkRecognizer, IInkRecognizer_Vtbl, 0x077ccea3_904d_442a_b151_aaca3631c43b);
impl windows_core::RuntimeType for IInkRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkRecognizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkRecognizerContainer, IInkRecognizerContainer_Vtbl, 0xa74d9a31_8047_4698_a912_f82a5085012f);
impl core::ops::Deref for IInkRecognizerContainer {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkRecognizerContainer, windows_core::IUnknown, windows_core::IInspectable);
impl IInkRecognizerContainer {
    pub fn SetDefaultRecognizer<P0>(&self, recognizer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkRecognizer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultRecognizer)(windows_core::Interface::as_raw(this), recognizer.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync<P0>(&self, strokecollection: P0, recognitiontarget: InkRecognitionTarget) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>
    where
        P0: windows_core::Param<InkStrokeContainer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizeAsync)(windows_core::Interface::as_raw(this), strokecollection.param().abi(), recognitiontarget, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecognizers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IInkRecognizerContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkRecognizerContainer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDefaultRecognizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecognizeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, InkRecognitionTarget, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecognizers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecognizers: usize,
}
windows_core::imp::define_interface!(IInkStroke, IInkStroke_Vtbl, 0x15144d60_cce3_4fcf_9d52_11518ab6afd4);
impl windows_core::RuntimeType for IInkStroke {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStroke_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DrawingAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDrawingAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Recognized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRenderingSegments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRenderingSegments: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStroke2, IInkStroke2_Vtbl, 0x5db9e4f4_bafa_4de1_89d3_201b1ed7d89b);
impl windows_core::RuntimeType for IInkStroke2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStroke2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub PointTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PointTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPointTransform: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPointTransform: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInkPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInkPoints: usize,
}
windows_core::imp::define_interface!(IInkStroke3, IInkStroke3_Vtbl, 0x4a807374_9499_411d_a1c4_68855d03d65f);
impl windows_core::RuntimeType for IInkStroke3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStroke3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub StrokeStartedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStrokeStartedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StrokeDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStrokeDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStroke4, IInkStroke4_Vtbl, 0xcd5b62e5_b6e9_5b91_a577_1921d2348690);
impl windows_core::RuntimeType for IInkStroke4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStroke4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStrokeBuilder, IInkStrokeBuilder_Vtbl, 0x82bbd1dc_1c63_41dc_9e07_4b4a70ced801);
impl windows_core::RuntimeType for IInkStrokeBuilder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeBuilder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BeginStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendToStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateStroke: usize,
    pub SetDefaultDrawingAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStrokeBuilder2, IInkStrokeBuilder2_Vtbl, 0xbd82bc27_731f_4cbc_bbbf_6d468044f1e5);
impl windows_core::RuntimeType for IInkStrokeBuilder2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeBuilder2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub CreateStrokeFromInkPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::Numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))]
    CreateStrokeFromInkPoints: usize,
}
windows_core::imp::define_interface!(IInkStrokeBuilder3, IInkStrokeBuilder3_Vtbl, 0xb2c71fcd_5472_46b1_a81d_c37a3d169441);
impl windows_core::RuntimeType for IInkStrokeBuilder3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeBuilder3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub CreateStrokeFromInkPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::Numerics::Matrix3x2, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))]
    CreateStrokeFromInkPoints: usize,
}
windows_core::imp::define_interface!(IInkStrokeContainer, IInkStrokeContainer_Vtbl, 0x22accbc6_faa9_4f14_b68c_f6cee670ae16);
impl core::ops::Deref for IInkStrokeContainer {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInkStrokeContainer, windows_core::IUnknown, windows_core::IInspectable);
impl IInkStrokeContainer {
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddStroke<P0>(&self, stroke: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkStroke>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStroke)(windows_core::Interface::as_raw(this), stroke.param().abi()).ok() }
    }
    pub fn DeleteSelected(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteSelected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSelected(&self, translation: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveSelected)(windows_core::Interface::as_raw(this), translation, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SelectWithPolyLine<P0>(&self, polyline: P0) -> windows_core::Result<super::super::super::Foundation::Rect>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectWithPolyLine)(windows_core::Interface::as_raw(this), polyline.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SelectWithLine(&self, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectWithLine)(windows_core::Interface::as_raw(this), from, to, &mut result__).map(|| result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CopySelectedToClipboard)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PasteFromClipboard(&self, position: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PasteFromClipboard)(windows_core::Interface::as_raw(this), position, &mut result__).map(|| result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPasteFromClipboard)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadAsync<P0>(&self, inputstream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAsync)(windows_core::Interface::as_raw(this), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveAsync<P0>(&self, outputstream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), outputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<P0>(&self, recognitionresults: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateRecognitionResults)(windows_core::Interface::as_raw(this), recognitionresults.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecognitionResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IInkStrokeContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeContainer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub AddStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub MoveSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectWithPolyLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectWithPolyLine: usize,
    pub SelectWithLine: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, super::super::super::Foundation::Point, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub CopySelectedToClipboard: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PasteFromClipboard: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Point, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub CanPasteFromClipboard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateRecognitionResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateRecognitionResults: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecognitionResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecognitionResults: usize,
}
windows_core::imp::define_interface!(IInkStrokeContainer2, IInkStrokeContainer2_Vtbl, 0x8901d364_da36_4bcf_9e5c_d195825995b4);
impl windows_core::RuntimeType for IInkStrokeContainer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeContainer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddStrokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddStrokes: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStrokeContainer3, IInkStrokeContainer3_Vtbl, 0x3d07bea5_baea_4c82_a719_7b83da1067d2);
impl windows_core::RuntimeType for IInkStrokeContainer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeContainer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SaveWithFormatAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, InkPersistenceFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SaveWithFormatAsync: usize,
    pub GetStrokeById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStrokeInput, IInkStrokeInput_Vtbl, 0xcf2ffe7b_5e10_43c6_a080_88f26e1dc67d);
impl windows_core::RuntimeType for IInkStrokeInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeInput_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub StrokeStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    StrokeStarted: usize,
    pub RemoveStrokeStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub StrokeContinued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    StrokeContinued: usize,
    pub RemoveStrokeContinued: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub StrokeEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    StrokeEnded: usize,
    pub RemoveStrokeEnded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub StrokeCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    StrokeCanceled: usize,
    pub RemoveStrokeCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStrokeRenderingSegment, IInkStrokeRenderingSegment_Vtbl, 0x68510f1f_88e3_477a_a2fa_569f5f1f9bd5);
impl windows_core::RuntimeType for IInkStrokeRenderingSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokeRenderingSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub BezierControlPoint1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub BezierControlPoint2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub TiltX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub TiltY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Twist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkStrokesCollectedEventArgs, IInkStrokesCollectedEventArgs_Vtbl, 0xc4f3f229_1938_495c_b4d9_6de4b08d4811);
impl windows_core::RuntimeType for IInkStrokesCollectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokesCollectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Strokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strokes: usize,
}
windows_core::imp::define_interface!(IInkStrokesErasedEventArgs, IInkStrokesErasedEventArgs_Vtbl, 0xa4216a22_1503_4ebf_8ff5_2de84584a8aa);
impl windows_core::RuntimeType for IInkStrokesErasedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkStrokesErasedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Strokes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strokes: usize,
}
windows_core::imp::define_interface!(IInkSynchronizer, IInkSynchronizer_Vtbl, 0x9b9ea160_ae9b_45f9_8407_4b493b163661);
impl windows_core::RuntimeType for IInkSynchronizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkSynchronizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BeginDry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BeginDry: usize,
    pub EndDry: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInkUnprocessedInput, IInkUnprocessedInput_Vtbl, 0xdb4445e0_8398_4921_ac3b_ab978c5ba256);
impl windows_core::RuntimeType for IInkUnprocessedInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkUnprocessedInput_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub PointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerEntered: usize,
    pub RemovePointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerHovered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerHovered: usize,
    pub RemovePointerHovered: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerExited: usize,
    pub RemovePointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerPressed: usize,
    pub RemovePointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerMoved: usize,
    pub RemovePointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerReleased: usize,
    pub RemovePointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerLost: usize,
    pub RemovePointerLost: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenAndInkSettings, IPenAndInkSettings_Vtbl, 0xbc2ceb8f_0066_44a8_bb7a_b839b3deb8f5);
impl windows_core::RuntimeType for IPenAndInkSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenAndInkSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsHandwritingDirectlyIntoTextFieldEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PenHandedness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PenHandedness) -> windows_core::HRESULT,
    pub HandwritingLineHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HandwritingLineHeight) -> windows_core::HRESULT,
    pub FontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserConsentsToHandwritingTelemetryCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTouchHandwritingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenAndInkSettings2, IPenAndInkSettings2_Vtbl, 0x3262da53_1f44_55e2_9929_ebf77e5481b8);
impl windows_core::RuntimeType for IPenAndInkSettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenAndInkSettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPenHandedness: unsafe extern "system" fn(*mut core::ffi::c_void, PenHandedness) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenAndInkSettingsStatics, IPenAndInkSettingsStatics_Vtbl, 0xed6dd036_5708_5c3c_96db_f2f552eab641);
impl windows_core::RuntimeType for IPenAndInkSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenAndInkSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkDrawingAttributes(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkDrawingAttributes, windows_core::IUnknown, windows_core::IInspectable);
impl InkDrawingAttributes {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkDrawingAttributes, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Color(&self) -> windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Color)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PenTip(&self) -> windows_core::Result<PenTipShape> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PenTip)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPenTip(&self, value: PenTipShape) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPenTip)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Size(&self) -> windows_core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: super::super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IgnorePressure(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnorePressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIgnorePressure(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIgnorePressure)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FitToCurve(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FitToCurve)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFitToCurve(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFitToCurve)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PenTipTransform(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PenTipTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPenTipTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPenTipTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DrawAsHighlighter(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrawAsHighlighter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDrawAsHighlighter(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDrawAsHighlighter)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<InkDrawingAttributesKind> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PencilProperties(&self) -> windows_core::Result<InkDrawingAttributesPencilProperties> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PencilProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IgnoreTilt(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnoreTilt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIgnoreTilt(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIgnoreTilt)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ModelerAttributes(&self) -> windows_core::Result<InkModelerAttributes> {
        let this = &windows_core::Interface::cast::<IInkDrawingAttributes5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelerAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateForPencil() -> windows_core::Result<InkDrawingAttributes> {
        Self::IInkDrawingAttributesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForPencil)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInkDrawingAttributesStatics<R, F: FnOnce(&IInkDrawingAttributesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkDrawingAttributes, IInkDrawingAttributesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InkDrawingAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkDrawingAttributes>();
}
unsafe impl windows_core::Interface for InkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_Vtbl;
    const IID: windows_core::GUID = <IInkDrawingAttributes as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkDrawingAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributes";
}
unsafe impl Send for InkDrawingAttributes {}
unsafe impl Sync for InkDrawingAttributes {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkDrawingAttributesPencilProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkDrawingAttributesPencilProperties, windows_core::IUnknown, windows_core::IInspectable);
impl InkDrawingAttributesPencilProperties {
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InkDrawingAttributesPencilProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkDrawingAttributesPencilProperties>();
}
unsafe impl windows_core::Interface for InkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilProperties_Vtbl;
    const IID: windows_core::GUID = <IInkDrawingAttributesPencilProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkDrawingAttributesPencilProperties {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties";
}
unsafe impl Send for InkDrawingAttributesPencilProperties {}
unsafe impl Sync for InkDrawingAttributesPencilProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkInputConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkInputConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl InkInputConfiguration {
    pub fn IsPrimaryBarrelButtonInputEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPrimaryBarrelButtonInputEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPrimaryBarrelButtonInputEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPrimaryBarrelButtonInputEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEraserInputEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEraserInputEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEraserInputEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEraserInputEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPenHapticFeedbackEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPenHapticFeedbackEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPenHapticFeedbackEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsPenHapticFeedbackEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InkInputConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkInputConfiguration>();
}
unsafe impl windows_core::Interface for InkInputConfiguration {
    type Vtable = IInkInputConfiguration_Vtbl;
    const IID: windows_core::GUID = <IInkInputConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkInputConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputConfiguration";
}
unsafe impl Send for InkInputConfiguration {}
unsafe impl Sync for InkInputConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkInputProcessingConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkInputProcessingConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl InkInputProcessingConfiguration {
    pub fn Mode(&self) -> windows_core::Result<InkInputProcessingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: InkInputProcessingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightDragAction(&self) -> windows_core::Result<InkInputRightDragAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightDragAction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRightDragAction(&self, value: InkInputRightDragAction) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRightDragAction)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InkInputProcessingConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkInputProcessingConfiguration>();
}
unsafe impl windows_core::Interface for InkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfiguration_Vtbl;
    const IID: windows_core::GUID = <IInkInputProcessingConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkInputProcessingConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputProcessingConfiguration";
}
unsafe impl Send for InkInputProcessingConfiguration {}
unsafe impl Sync for InkInputProcessingConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkManager, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkManager, IInkRecognizerContainer, IInkStrokeContainer);
impl InkManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkManager, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> windows_core::Result<InkManipulationMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: InkManipulationMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProcessPointerDown<P0>(&self, pointerpoint: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::PointerPoint>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessPointerDown)(windows_core::Interface::as_raw(this), pointerpoint.param().abi()).ok() }
    }
    pub fn ProcessPointerUpdate<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<super::PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessPointerUpdate)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessPointerUp<P0>(&self, pointerpoint: P0) -> windows_core::Result<super::super::super::Foundation::Rect>
    where
        P0: windows_core::Param<super::PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessPointerUp)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetDefaultDrawingAttributes<P0>(&self, drawingattributes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkDrawingAttributes>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultDrawingAttributes)(windows_core::Interface::as_raw(this), drawingattributes.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync2(&self, recognitiontarget: InkRecognitionTarget) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizeAsync2)(windows_core::Interface::as_raw(this), recognitiontarget, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultRecognizer<P0>(&self, recognizer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkRecognizer>,
    {
        let this = &windows_core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultRecognizer)(windows_core::Interface::as_raw(this), recognizer.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync<P0>(&self, strokecollection: P0, recognitiontarget: InkRecognitionTarget) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>
    where
        P0: windows_core::Param<InkStrokeContainer>,
    {
        let this = &windows_core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizeAsync)(windows_core::Interface::as_raw(this), strokecollection.param().abi(), recognitiontarget, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = &windows_core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecognizers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddStroke<P0>(&self, stroke: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkStroke>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddStroke)(windows_core::Interface::as_raw(this), stroke.param().abi()).ok() }
    }
    pub fn DeleteSelected(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteSelected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSelected(&self, translation: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveSelected)(windows_core::Interface::as_raw(this), translation, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SelectWithPolyLine<P0>(&self, polyline: P0) -> windows_core::Result<super::super::super::Foundation::Rect>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectWithPolyLine)(windows_core::Interface::as_raw(this), polyline.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SelectWithLine(&self, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectWithLine)(windows_core::Interface::as_raw(this), from, to, &mut result__).map(|| result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CopySelectedToClipboard)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PasteFromClipboard(&self, position: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PasteFromClipboard)(windows_core::Interface::as_raw(this), position, &mut result__).map(|| result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPasteFromClipboard)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadAsync<P0>(&self, inputstream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAsync)(windows_core::Interface::as_raw(this), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveAsync<P0>(&self, outputstream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), outputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<P0>(&self, recognitionresults: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateRecognitionResults)(windows_core::Interface::as_raw(this), recognitionresults.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecognitionResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkManager>();
}
unsafe impl windows_core::Interface for InkManager {
    type Vtable = IInkManager_Vtbl;
    const IID: windows_core::GUID = <IInkManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkManager {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkModelerAttributes(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkModelerAttributes, windows_core::IUnknown, windows_core::IInspectable);
impl InkModelerAttributes {
    pub fn PredictionTime(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PredictionTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPredictionTime(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPredictionTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScalingFactor(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScalingFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScalingFactor(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScalingFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UseVelocityBasedPressure(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseVelocityBasedPressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseVelocityBasedPressure(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseVelocityBasedPressure)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InkModelerAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkModelerAttributes>();
}
unsafe impl windows_core::Interface for InkModelerAttributes {
    type Vtable = IInkModelerAttributes_Vtbl;
    const IID: windows_core::GUID = <IInkModelerAttributes as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkModelerAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkModelerAttributes";
}
unsafe impl Send for InkModelerAttributes {}
unsafe impl Sync for InkModelerAttributes {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkPoint, windows_core::IUnknown, windows_core::IInspectable);
impl InkPoint {
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pressure(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TiltX(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiltX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TiltY(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiltY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInkPoint(position: super::super::super::Foundation::Point, pressure: f32) -> windows_core::Result<InkPoint> {
        Self::IInkPointFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInkPoint)(windows_core::Interface::as_raw(this), position, pressure, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInkPointWithTiltAndTimestamp(position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> windows_core::Result<InkPoint> {
        Self::IInkPointFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInkPointWithTiltAndTimestamp)(windows_core::Interface::as_raw(this), position, pressure, tiltx, tilty, timestamp, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInkPointFactory<R, F: FnOnce(&IInkPointFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkPoint, IInkPointFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInkPointFactory2<R, F: FnOnce(&IInkPointFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkPoint, IInkPointFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InkPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkPoint>();
}
unsafe impl windows_core::Interface for InkPoint {
    type Vtable = IInkPoint_Vtbl;
    const IID: windows_core::GUID = <IInkPoint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkPoint {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPoint";
}
unsafe impl Send for InkPoint {}
unsafe impl Sync for InkPoint {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkPresenter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkPresenter, windows_core::IUnknown, windows_core::IInspectable);
impl InkPresenter {
    pub fn IsInputEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInputEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsInputEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn InputDeviceTypes(&self) -> windows_core::Result<super::super::Core::CoreInputDeviceTypes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputDeviceTypes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInputDeviceTypes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UnprocessedInput(&self) -> windows_core::Result<InkUnprocessedInput> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnprocessedInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StrokeInput(&self) -> windows_core::Result<InkStrokeInput> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InputProcessingConfiguration(&self) -> windows_core::Result<InkInputProcessingConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputProcessingConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StrokeContainer(&self) -> windows_core::Result<InkStrokeContainer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeContainer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStrokeContainer<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkStrokeContainer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStrokeContainer)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CopyDefaultDrawingAttributes(&self) -> windows_core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyDefaultDrawingAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateDefaultDrawingAttributes<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkDrawingAttributes>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateDefaultDrawingAttributes)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ActivateCustomDrying(&self) -> windows_core::Result<InkSynchronizer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivateCustomDrying)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPredefinedConfiguration(&self, value: InkPresenterPredefinedConfiguration) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPredefinedConfiguration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StrokesCollected<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokesCollected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStrokesCollected(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStrokesCollected)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn StrokesErased<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokesErased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStrokesErased(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStrokesErased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn HighContrastAdjustment(&self) -> windows_core::Result<InkHighContrastAdjustment> {
        let this = &windows_core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HighContrastAdjustment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHighContrastAdjustment(&self, value: InkHighContrastAdjustment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHighContrastAdjustment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputConfiguration(&self) -> windows_core::Result<InkInputConfiguration> {
        let this = &windows_core::Interface::cast::<IInkPresenter3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkPresenter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkPresenter>();
}
unsafe impl windows_core::Interface for InkPresenter {
    type Vtable = IInkPresenter_Vtbl;
    const IID: windows_core::GUID = <IInkPresenter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkPresenter {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenter";
}
unsafe impl Send for InkPresenter {}
unsafe impl Sync for InkPresenter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkPresenterProtractor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkPresenterProtractor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkPresenterProtractor, IInkPresenterStencil);
impl InkPresenterProtractor {
    pub fn AreTickMarksVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreTickMarksVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAreTickMarksVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAreTickMarksVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AreRaysVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreRaysVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAreRaysVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAreRaysVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCenterMarkerVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCenterMarkerVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCenterMarkerVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCenterMarkerVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAngleReadoutVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAngleReadoutVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAngleReadoutVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAngleReadoutVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsResizable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsResizable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsResizable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Radius(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Radius)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRadius(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRadius)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccentColor(&self) -> windows_core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccentColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAccentColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAccentColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(inkpresenter: P0) -> windows_core::Result<InkPresenterProtractor>
    where
        P0: windows_core::Param<InkPresenter>,
    {
        Self::IInkPresenterProtractorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), inkpresenter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Kind(&self) -> windows_core::Result<InkPresenterStencilKind> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::Color> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> windows_core::Result<super::super::Color> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForegroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IInkPresenterProtractorFactory<R, F: FnOnce(&IInkPresenterProtractorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkPresenterProtractor, IInkPresenterProtractorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InkPresenterProtractor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkPresenterProtractor>();
}
unsafe impl windows_core::Interface for InkPresenterProtractor {
    type Vtable = IInkPresenterProtractor_Vtbl;
    const IID: windows_core::GUID = <IInkPresenterProtractor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkPresenterProtractor {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterProtractor";
}
unsafe impl Send for InkPresenterProtractor {}
unsafe impl Sync for InkPresenterProtractor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkPresenterRuler(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkPresenterRuler, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkPresenterRuler, IInkPresenterStencil);
impl InkPresenterRuler {
    pub fn Length(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLength(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AreTickMarksVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreTickMarksVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAreTickMarksVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAreTickMarksVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCompassVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCompassVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCompassVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsCompassVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(inkpresenter: P0) -> windows_core::Result<InkPresenterRuler>
    where
        P0: windows_core::Param<InkPresenter>,
    {
        Self::IInkPresenterRulerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), inkpresenter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Kind(&self) -> windows_core::Result<InkPresenterStencilKind> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::Color> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> windows_core::Result<super::super::Color> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForegroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForegroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Transform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IInkPresenterRulerFactory<R, F: FnOnce(&IInkPresenterRulerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkPresenterRuler, IInkPresenterRulerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InkPresenterRuler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkPresenterRuler>();
}
unsafe impl windows_core::Interface for InkPresenterRuler {
    type Vtable = IInkPresenterRuler_Vtbl;
    const IID: windows_core::GUID = <IInkPresenterRuler as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkPresenterRuler {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterRuler";
}
unsafe impl Send for InkPresenterRuler {}
unsafe impl Sync for InkPresenterRuler {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkRecognitionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkRecognitionResult, windows_core::IUnknown, windows_core::IInspectable);
impl InkRecognitionResult {
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextCandidates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTextCandidates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkRecognitionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkRecognitionResult>();
}
unsafe impl windows_core::Interface for InkRecognitionResult {
    type Vtable = IInkRecognitionResult_Vtbl;
    const IID: windows_core::GUID = <IInkRecognitionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkRecognitionResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognitionResult";
}
unsafe impl Send for InkRecognitionResult {}
unsafe impl Sync for InkRecognitionResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkRecognizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkRecognizer, windows_core::IUnknown, windows_core::IInspectable);
impl InkRecognizer {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkRecognizer>();
}
unsafe impl windows_core::Interface for InkRecognizer {
    type Vtable = IInkRecognizer_Vtbl;
    const IID: windows_core::GUID = <IInkRecognizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizer";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkRecognizerContainer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkRecognizerContainer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkRecognizerContainer, IInkRecognizerContainer);
impl InkRecognizerContainer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkRecognizerContainer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDefaultRecognizer<P0>(&self, recognizer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkRecognizer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultRecognizer)(windows_core::Interface::as_raw(this), recognizer.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecognizeAsync<P0>(&self, strokecollection: P0, recognitiontarget: InkRecognitionTarget) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>
    where
        P0: windows_core::Param<InkStrokeContainer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecognizeAsync)(windows_core::Interface::as_raw(this), strokecollection.param().abi(), recognitiontarget, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecognizers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkRecognizerContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkRecognizerContainer>();
}
unsafe impl windows_core::Interface for InkRecognizerContainer {
    type Vtable = IInkRecognizerContainer_Vtbl;
    const IID: windows_core::GUID = <IInkRecognizerContainer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizerContainer";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkStroke(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkStroke, windows_core::IUnknown, windows_core::IInspectable);
impl InkStroke {
    pub fn DrawingAttributes(&self) -> windows_core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrawingAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDrawingAttributes<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkDrawingAttributes>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDrawingAttributes)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Selected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Selected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSelected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelected)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Recognized(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recognized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRenderingSegments(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRenderingSegments)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Clone(&self) -> windows_core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PointTransform(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &windows_core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPointTransform(&self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkStroke2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointTransform)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInkPoints(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>> {
        let this = &windows_core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInkPoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StrokeStartedTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeStartedTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStrokeStartedTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
    {
        let this = &windows_core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStrokeStartedTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StrokeDuration(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeDuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStrokeDuration<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStrokeDuration)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IInkStroke4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InkStroke {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkStroke>();
}
unsafe impl windows_core::Interface for InkStroke {
    type Vtable = IInkStroke_Vtbl;
    const IID: windows_core::GUID = <IInkStroke as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStroke";
}
unsafe impl Send for InkStroke {}
unsafe impl Sync for InkStroke {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkStrokeBuilder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkStrokeBuilder, windows_core::IUnknown, windows_core::IInspectable);
impl InkStrokeBuilder {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkStrokeBuilder, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn BeginStroke<P0>(&self, pointerpoint: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::PointerPoint>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BeginStroke)(windows_core::Interface::as_raw(this), pointerpoint.param().abi()).ok() }
    }
    pub fn AppendToStroke<P0>(&self, pointerpoint: P0) -> windows_core::Result<super::PointerPoint>
    where
        P0: windows_core::Param<super::PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendToStroke)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EndStroke<P0>(&self, pointerpoint: P0) -> windows_core::Result<InkStroke>
    where
        P0: windows_core::Param<super::PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndStroke)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateStroke<P0>(&self, points: P0) -> windows_core::Result<InkStroke>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStroke)(windows_core::Interface::as_raw(this), points.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultDrawingAttributes<P0>(&self, drawingattributes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkDrawingAttributes>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultDrawingAttributes)(windows_core::Interface::as_raw(this), drawingattributes.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints<P0>(&self, inkpoints: P0, transform: super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<InkStroke>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<InkPoint>>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeBuilder2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStrokeFromInkPoints)(windows_core::Interface::as_raw(this), inkpoints.param().abi(), transform, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints2<P0, P1, P2>(&self, inkpoints: P0, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: P1, strokeduration: P2) -> windows_core::Result<InkStroke>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<InkPoint>>,
        P1: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
        P2: windows_core::Param<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeBuilder3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStrokeFromInkPoints)(windows_core::Interface::as_raw(this), inkpoints.param().abi(), transform, strokestartedtime.param().abi(), strokeduration.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkStrokeBuilder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkStrokeBuilder>();
}
unsafe impl windows_core::Interface for InkStrokeBuilder {
    type Vtable = IInkStrokeBuilder_Vtbl;
    const IID: windows_core::GUID = <IInkStrokeBuilder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkStrokeBuilder {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeBuilder";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkStrokeContainer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkStrokeContainer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InkStrokeContainer, IInkStrokeContainer);
impl InkStrokeContainer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkStrokeContainer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddStroke<P0>(&self, stroke: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InkStroke>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddStroke)(windows_core::Interface::as_raw(this), stroke.param().abi()).ok() }
    }
    pub fn DeleteSelected(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteSelected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSelected(&self, translation: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveSelected)(windows_core::Interface::as_raw(this), translation, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SelectWithPolyLine<P0>(&self, polyline: P0) -> windows_core::Result<super::super::super::Foundation::Rect>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectWithPolyLine)(windows_core::Interface::as_raw(this), polyline.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SelectWithLine(&self, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectWithLine)(windows_core::Interface::as_raw(this), from, to, &mut result__).map(|| result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CopySelectedToClipboard)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PasteFromClipboard(&self, position: super::super::super::Foundation::Point) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PasteFromClipboard)(windows_core::Interface::as_raw(this), position, &mut result__).map(|| result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanPasteFromClipboard)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadAsync<P0>(&self, inputstream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAsync)(windows_core::Interface::as_raw(this), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveAsync<P0>(&self, outputstream: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), outputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<P0>(&self, recognitionresults: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateRecognitionResults)(windows_core::Interface::as_raw(this), recognitionresults.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecognitionResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddStrokes<P0>(&self, strokes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<InkStroke>>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddStrokes)(windows_core::Interface::as_raw(this), strokes.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveWithFormatAsync<P0>(&self, outputstream: P0, inkpersistenceformat: InkPersistenceFormat) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveWithFormatAsync)(windows_core::Interface::as_raw(this), outputstream.param().abi(), inkpersistenceformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetStrokeById(&self, id: u32) -> windows_core::Result<InkStroke> {
        let this = &windows_core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStrokeById)(windows_core::Interface::as_raw(this), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkStrokeContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkStrokeContainer>();
}
unsafe impl windows_core::Interface for InkStrokeContainer {
    type Vtable = IInkStrokeContainer_Vtbl;
    const IID: windows_core::GUID = <IInkStrokeContainer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeContainer";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkStrokeInput(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkStrokeInput, windows_core::IUnknown, windows_core::IInspectable);
impl InkStrokeInput {
    #[cfg(feature = "UI_Core")]
    pub fn StrokeStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStrokeStarted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStrokeStarted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn StrokeContinued<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeContinued)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStrokeContinued(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStrokeContinued)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn StrokeEnded<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeEnded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStrokeEnded(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStrokeEnded)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn StrokeCanceled<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeCanceled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStrokeCanceled(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStrokeCanceled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn InkPresenter(&self) -> windows_core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InkPresenter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkStrokeInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkStrokeInput>();
}
unsafe impl windows_core::Interface for InkStrokeInput {
    type Vtable = IInkStrokeInput_Vtbl;
    const IID: windows_core::GUID = <IInkStrokeInput as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkStrokeInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeInput";
}
unsafe impl Send for InkStrokeInput {}
unsafe impl Sync for InkStrokeInput {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkStrokeRenderingSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkStrokeRenderingSegment, windows_core::IUnknown, windows_core::IInspectable);
impl InkStrokeRenderingSegment {
    pub fn Position(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BezierControlPoint1(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BezierControlPoint1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BezierControlPoint2(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BezierControlPoint2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pressure(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TiltX(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiltX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TiltY(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiltY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Twist(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Twist)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InkStrokeRenderingSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkStrokeRenderingSegment>();
}
unsafe impl windows_core::Interface for InkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegment_Vtbl;
    const IID: windows_core::GUID = <IInkStrokeRenderingSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkStrokeRenderingSegment {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeRenderingSegment";
}
unsafe impl Send for InkStrokeRenderingSegment {}
unsafe impl Sync for InkStrokeRenderingSegment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkStrokesCollectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkStrokesCollectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InkStrokesCollectedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Strokes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkStrokesCollectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkStrokesCollectedEventArgs>();
}
unsafe impl windows_core::Interface for InkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IInkStrokesCollectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkStrokesCollectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesCollectedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkStrokesErasedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkStrokesErasedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InkStrokesErasedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Strokes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkStrokesErasedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkStrokesErasedEventArgs>();
}
unsafe impl windows_core::Interface for InkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IInkStrokesErasedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkStrokesErasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesErasedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkSynchronizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkSynchronizer, windows_core::IUnknown, windows_core::IInspectable);
impl InkSynchronizer {
    #[cfg(feature = "Foundation_Collections")]
    pub fn BeginDry(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginDry)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EndDry(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EndDry)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for InkSynchronizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkSynchronizer>();
}
unsafe impl windows_core::Interface for InkSynchronizer {
    type Vtable = IInkSynchronizer_Vtbl;
    const IID: windows_core::GUID = <IInkSynchronizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkSynchronizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkSynchronizer";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkUnprocessedInput(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkUnprocessedInput, windows_core::IUnknown, windows_core::IInspectable);
impl InkUnprocessedInput {
    #[cfg(feature = "UI_Core")]
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerHovered<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerHovered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerHovered(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerHovered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExited)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerLost(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn InkPresenter(&self) -> windows_core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InkPresenter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InkUnprocessedInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkUnprocessedInput>();
}
unsafe impl windows_core::Interface for InkUnprocessedInput {
    type Vtable = IInkUnprocessedInput_Vtbl;
    const IID: windows_core::GUID = <IInkUnprocessedInput as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkUnprocessedInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkUnprocessedInput";
}
unsafe impl Send for InkUnprocessedInput {}
unsafe impl Sync for InkUnprocessedInput {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenAndInkSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenAndInkSettings, windows_core::IUnknown, windows_core::IInspectable);
impl PenAndInkSettings {
    pub fn IsHandwritingDirectlyIntoTextFieldEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHandwritingDirectlyIntoTextFieldEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PenHandedness(&self) -> windows_core::Result<PenHandedness> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PenHandedness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HandwritingLineHeight(&self) -> windows_core::Result<HandwritingLineHeight> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HandwritingLineHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FontFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserConsentsToHandwritingTelemetryCollection(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserConsentsToHandwritingTelemetryCollection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsTouchHandwritingEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTouchHandwritingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPenHandedness(&self, value: PenHandedness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPenAndInkSettings2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPenHandedness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<PenAndInkSettings> {
        Self::IPenAndInkSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPenAndInkSettingsStatics<R, F: FnOnce(&IPenAndInkSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PenAndInkSettings, IPenAndInkSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PenAndInkSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenAndInkSettings>();
}
unsafe impl windows_core::Interface for PenAndInkSettings {
    type Vtable = IPenAndInkSettings_Vtbl;
    const IID: windows_core::GUID = <IPenAndInkSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenAndInkSettings {
    const NAME: &'static str = "Windows.UI.Input.Inking.PenAndInkSettings";
}
unsafe impl Send for PenAndInkSettings {}
unsafe impl Sync for PenAndInkSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HandwritingLineHeight(pub i32);
impl HandwritingLineHeight {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl windows_core::TypeKind for HandwritingLineHeight {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HandwritingLineHeight {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HandwritingLineHeight").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HandwritingLineHeight {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.HandwritingLineHeight;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: Self = Self(0i32);
    pub const Pencil: Self = Self(1i32);
}
impl windows_core::TypeKind for InkDrawingAttributesKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkDrawingAttributesKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkDrawingAttributesKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkDrawingAttributesKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkDrawingAttributesKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: Self = Self(0i32);
    pub const UseSystemColors: Self = Self(1i32);
    pub const UseOriginalColors: Self = Self(2i32);
}
impl windows_core::TypeKind for InkHighContrastAdjustment {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkHighContrastAdjustment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkHighContrastAdjustment").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkHighContrastAdjustment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkHighContrastAdjustment;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: Self = Self(0i32);
    pub const Inking: Self = Self(1i32);
    pub const Erasing: Self = Self(2i32);
}
impl windows_core::TypeKind for InkInputProcessingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkInputProcessingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkInputProcessingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkInputProcessingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputProcessingMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: Self = Self(0i32);
    pub const AllowProcessing: Self = Self(1i32);
}
impl windows_core::TypeKind for InkInputRightDragAction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkInputRightDragAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkInputRightDragAction").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkInputRightDragAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputRightDragAction;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: Self = Self(0i32);
    pub const Erasing: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
}
impl windows_core::TypeKind for InkManipulationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkManipulationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkManipulationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkManipulationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkManipulationMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: Self = Self(0i32);
    pub const Isf: Self = Self(1i32);
}
impl windows_core::TypeKind for InkPersistenceFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkPersistenceFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkPersistenceFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkPersistenceFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPersistenceFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: Self = Self(0i32);
    pub const SimpleMultiplePointer: Self = Self(1i32);
}
impl windows_core::TypeKind for InkPresenterPredefinedConfiguration {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkPresenterPredefinedConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkPresenterPredefinedConfiguration").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkPresenterPredefinedConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterPredefinedConfiguration;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: Self = Self(0i32);
    pub const Ruler: Self = Self(1i32);
    pub const Protractor: Self = Self(2i32);
}
impl windows_core::TypeKind for InkPresenterStencilKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkPresenterStencilKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkPresenterStencilKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkPresenterStencilKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterStencilKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Recent: Self = Self(2i32);
}
impl windows_core::TypeKind for InkRecognitionTarget {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InkRecognitionTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InkRecognitionTarget").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InkRecognitionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkRecognitionTarget;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl windows_core::TypeKind for PenHandedness {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PenHandedness {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PenHandedness").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PenHandedness {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenHandedness;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: Self = Self(0i32);
    pub const Rectangle: Self = Self(1i32);
}
impl windows_core::TypeKind for PenTipShape {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PenTipShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PenTipShape").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PenTipShape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenTipShape;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
