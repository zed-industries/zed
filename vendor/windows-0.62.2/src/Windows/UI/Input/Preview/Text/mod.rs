#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversionModeChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversionModeChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ConversionModeChangedEventArgs {
    pub fn NewConversionMode(&self) -> windows_core::Result<TextConversionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewConversionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ConversionModeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConversionModeChangedEventArgs>();
}
unsafe impl windows_core::Interface for ConversionModeChangedEventArgs {
    type Vtable = <IConversionModeChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IConversionModeChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversionModeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.ConversionModeChangedEventArgs";
}
unsafe impl Send for ConversionModeChangedEventArgs {}
unsafe impl Sync for ConversionModeChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusEnteredEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusEnteredEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FocusEnteredEventArgs {
    pub fn FocusedTextBoxInfo(&self) -> windows_core::Result<TextBoxInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusedTextBoxInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FocusEnteredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusEnteredEventArgs>();
}
unsafe impl windows_core::Interface for FocusEnteredEventArgs {
    type Vtable = <IFocusEnteredEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusEnteredEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusEnteredEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.FocusEnteredEventArgs";
}
unsafe impl Send for FocusEnteredEventArgs {}
unsafe impl Sync for FocusEnteredEventArgs {}
windows_core::imp::define_interface!(IConversionModeChangedEventArgs, IConversionModeChangedEventArgs_Vtbl, 0xb49761f9_5b21_513c_b6c0_78f27d26b010);
impl windows_core::RuntimeType for IConversionModeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversionModeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewConversionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextConversionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusEnteredEventArgs, IFocusEnteredEventArgs_Vtbl, 0xca4dc200_875f_501d_af14_413a0aa1ed5f);
impl windows_core::RuntimeType for IFocusEnteredEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusEnteredEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FocusedTextBoxInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputDelegationModeChangedEventArgs, IInputDelegationModeChangedEventArgs_Vtbl, 0x4bb448b2_67ba_5215_8783_b444bd28eed3);
impl windows_core::RuntimeType for IInputDelegationModeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDelegationModeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DelegationOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyEventReceivedEventArgs, IKeyEventReceivedEventArgs_Vtbl, 0x0c30f686_a058_5ecc_abd2_9cc861c1185b);
impl windows_core::RuntimeType for IKeyEventReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyEventReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Core::CorePhysicalKeyStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub Unicode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KeyEventDeviceType) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub IsKeyPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::System::VirtualKey, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsKeyPressed: usize,
    #[cfg(feature = "System")]
    pub IsToggleKeyOn: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::System::VirtualKey, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsToggleKeyOn: usize,
    pub EditSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyboardInputProcessor, IKeyboardInputProcessor_Vtbl, 0x2afe79b6_5818_50e0_8fa8_81bc96428c46);
impl windows_core::RuntimeType for IKeyboardInputProcessor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardInputProcessor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasFocusedTextBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub FocusedTextBoxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextBoxId) -> windows_core::HRESULT,
    pub FocusedTextBoxInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FocusedTextBoxBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConversionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextConversionMode) -> windows_core::HRESULT,
    pub SetConversionMode: unsafe extern "system" fn(*mut core::ffi::c_void, TextConversionMode) -> windows_core::HRESULT,
    pub CreateEditSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Deactivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDeactivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub KeyEventReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveKeyEventReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FocusEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFocusEntered: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FocusRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFocusRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ConversionModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveConversionModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TextBoxInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTextBoxInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TextBoxContentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTextBoxContentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CompositionTerminated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCompositionTerminated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ReconversionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReconversionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReconversionRequestedEventArgs, IReconversionRequestedEventArgs_Vtbl, 0x73852244_d202_55fe_9edf_beb7ec19f937);
impl windows_core::RuntimeType for IReconversionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IReconversionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Text_Core")]
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Text::Core::CoreTextRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    Range: usize,
}
windows_core::imp::define_interface!(ITextBoxContentChangedEventArgs, ITextBoxContentChangedEventArgs_Vtbl, 0x2cb70a41_5aed_58c5_b4c1_8ee4e1492f9e);
impl windows_core::RuntimeType for ITextBoxContentChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextBoxContentChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextBoxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextBoxId) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextChangeSource) -> windows_core::HRESULT,
    pub SelectionBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub IsContentAttributeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, TextBoxContentAttribute, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextBoxInfo, ITextBoxInfo_Vtbl, 0xb122443d_e8f7_5f8b_813d_aaa0941d5fa0);
impl windows_core::RuntimeType for ITextBoxInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextBoxInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextBoxId) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Text_Core")]
    pub InputScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Text::Core::CoreTextInputScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    InputScope: usize,
    pub AppName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Url: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextBoxSettings) -> windows_core::HRESULT,
    pub DisabledFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextBoxFeatures) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextBoxInfoChangedEventArgs, ITextBoxInfoChangedEventArgs_Vtbl, 0xac1275af_648c_5bac_b29f_d1ea17e9e6d6);
impl windows_core::RuntimeType for ITextBoxInfoChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextBoxInfoChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextBoxInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextComposition, ITextComposition_Vtbl, 0x5cea9aea_524d_50a4_b08a_c83d8d25ec6e);
impl windows_core::RuntimeType for ITextComposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextComposition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FirstSegment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectedSegment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CaretPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCaretPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub InsertText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompleteUnconverted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompleteFirstSegment: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextCompositionSegment, ITextCompositionSegment_Vtbl, 0x0543f6c6_eb98_56d6_8808_2eca6d02f6a5);
impl windows_core::RuntimeType for ITextCompositionSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextCompositionSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConvertedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnconvertedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUnconvertedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Text_Core")]
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Text::Core::CoreTextRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    Range: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub ConversionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Text::Core::CoreTextFormatUpdatingReason) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    ConversionState: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub SetConversionState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Text::Core::CoreTextFormatUpdatingReason) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    SetConversionState: usize,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Previous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub GetTextStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextStyle) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    GetTextStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetTextStyle: unsafe extern "system" fn(*mut core::ffi::c_void, TextStyle) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetTextStyle: usize,
}
windows_core::imp::define_interface!(ITextEditSession, ITextEditSession_Vtbl, 0x0bcad18a_d31b_5787_aff9_995ee743aea8);
impl windows_core::RuntimeType for ITextEditSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextEditSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextBoxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextBoxId) -> windows_core::HRESULT,
    pub TextLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Text_Core")]
    pub PopulatedRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Text::Core::CoreTextRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    PopulatedRange: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub PopulateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Text::Core::CoreTextRange, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    PopulateAsync: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Text::Core::CoreTextRange, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    GetText: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub GetSelectedRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Text::Core::CoreTextRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    GetSelectedRange: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub SetSelectedRange: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Text::Core::CoreTextRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    SetSelectedRange: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub ReplaceText: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Text::Core::CoreTextRange, *mut core::ffi::c_void, *mut super::super::super::Text::Core::CoreTextRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    ReplaceText: usize,
    pub Composition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Text_Core")]
    pub StartReconversion: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Text::Core::CoreTextRange, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    StartReconversion: usize,
    pub SubmitPayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SubmitPayloadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextInputProvider, ITextInputProvider_Vtbl, 0xb0885fb7_e9f8_5849_b0ef_f8155ecf60d1);
impl windows_core::RuntimeType for ITextInputProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextInputServiceSubscription) -> windows_core::HRESULT,
    pub SetSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, TextInputServiceSubscription) -> windows_core::HRESULT,
    pub HasFocusedTextBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub FocusedTextBoxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TextBoxId) -> windows_core::HRESULT,
    pub FocusedTextBoxInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FocusedTextBoxBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEditSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryStartDelegation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub StopDelegation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FocusEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFocusEntered: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FocusRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveFocusRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TextBoxInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTextBoxInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TextBoxContentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTextBoxContentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CompositionTerminated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCompositionTerminated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ReconversionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReconversionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub InputDelegationModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveInputDelegationModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextInputService, ITextInputService_Vtbl, 0x8e23f89c_ab1f_551a_8751_7d4f29e34d88);
impl windows_core::RuntimeType for ITextInputService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateKeyboardInputProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextInputProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextInputServiceStatics, ITextInputServiceStatics_Vtbl, 0x91b68f5e_02ed_4e09_ae89_dfd735cf10bc);
impl windows_core::RuntimeType for ITextInputServiceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputDelegationModeChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputDelegationModeChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InputDelegationModeChangedEventArgs {
    pub fn DelegationOn(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DelegationOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InputDelegationModeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputDelegationModeChangedEventArgs>();
}
unsafe impl windows_core::Interface for InputDelegationModeChangedEventArgs {
    type Vtable = <IInputDelegationModeChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputDelegationModeChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputDelegationModeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.InputDelegationModeChangedEventArgs";
}
unsafe impl Send for InputDelegationModeChangedEventArgs {}
unsafe impl Sync for InputDelegationModeChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyEventDeviceType(pub i32);
impl KeyEventDeviceType {
    pub const Undefined: Self = Self(0i32);
    pub const HardwareKeyboard: Self = Self(1i32);
    pub const SoftwareKeyboard: Self = Self(2i32);
    pub const Gamepad: Self = Self(3i32);
    pub const Injection: Self = Self(4i32);
}
impl windows_core::TypeKind for KeyEventDeviceType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for KeyEventDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.KeyEventDeviceType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyEventReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyEventReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl KeyEventReceivedEventArgs {
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> windows_core::Result<super::super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> windows_core::Result<super::super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Unicode(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unicode)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Source(&self) -> windows_core::Result<KeyEventDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn IsKeyPressed(&self, vkey: super::super::super::super::System::VirtualKey) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsKeyPressed)(windows_core::Interface::as_raw(this), vkey, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn IsToggleKeyOn(&self, vkey: super::super::super::super::System::VirtualKey) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsToggleKeyOn)(windows_core::Interface::as_raw(this), vkey, &mut result__).map(|| result__)
        }
    }
    pub fn EditSession(&self) -> windows_core::Result<TextEditSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EditSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for KeyEventReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyEventReceivedEventArgs>();
}
unsafe impl windows_core::Interface for KeyEventReceivedEventArgs {
    type Vtable = <IKeyEventReceivedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyEventReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyEventReceivedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.KeyEventReceivedEventArgs";
}
unsafe impl Send for KeyEventReceivedEventArgs {}
unsafe impl Sync for KeyEventReceivedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyboardInputProcessor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyboardInputProcessor, windows_core::IUnknown, windows_core::IInspectable);
impl KeyboardInputProcessor {
    pub fn InputProfile(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasFocusedTextBox(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasFocusedTextBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FocusedTextBoxId(&self) -> windows_core::Result<TextBoxId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusedTextBoxId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FocusedTextBoxInfo(&self) -> windows_core::Result<TextBoxInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusedTextBoxInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FocusedTextBoxBounds(&self) -> windows_core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusedTextBoxBounds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectionBounds(&self) -> windows_core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionBounds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConversionMode(&self) -> windows_core::Result<TextConversionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConversionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetConversionMode(&self, value: TextConversionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetConversionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateEditSession(&self) -> windows_core::Result<TextEditSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEditSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Activated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivated(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Deactivated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deactivated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeactivated(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDeactivated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn KeyEventReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, KeyEventReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyEventReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyEventReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyEventReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FocusEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, FocusEnteredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFocusEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEntered)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FocusRemoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFocusRemoved(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ConversionModeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, ConversionModeChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConversionModeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConversionModeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConversionModeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TextBoxInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, TextBoxInfoChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBoxInfoChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextBoxInfoChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextBoxInfoChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TextBoxContentChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, TextBoxContentChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBoxContentChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextBoxContentChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextBoxContentChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CompositionTerminated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositionTerminated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompositionTerminated(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompositionTerminated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReconversionRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<KeyboardInputProcessor, ReconversionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReconversionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReconversionRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReconversionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for KeyboardInputProcessor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyboardInputProcessor>();
}
unsafe impl windows_core::Interface for KeyboardInputProcessor {
    type Vtable = <IKeyboardInputProcessor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyboardInputProcessor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyboardInputProcessor {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.KeyboardInputProcessor";
}
unsafe impl Send for KeyboardInputProcessor {}
unsafe impl Sync for KeyboardInputProcessor {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PayloadResult(pub i32);
impl PayloadResult {
    pub const InEditing: Self = Self(0i32);
    pub const Pending: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Overridden: Self = Self(3i32);
    pub const Outrun: Self = Self(4i32);
    pub const Rejected: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
}
impl windows_core::TypeKind for PayloadResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PayloadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.PayloadResult;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconversionRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ReconversionRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ReconversionRequestedEventArgs {
    #[cfg(feature = "UI_Text_Core")]
    pub fn Range(&self) -> windows_core::Result<super::super::super::Text::Core::CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Range)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ReconversionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IReconversionRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ReconversionRequestedEventArgs {
    type Vtable = <IReconversionRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IReconversionRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ReconversionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.ReconversionRequestedEventArgs";
}
unsafe impl Send for ReconversionRequestedEventArgs {}
unsafe impl Sync for ReconversionRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextBoxContentAttribute(pub i32);
impl TextBoxContentAttribute {
    pub const None: Self = Self(0i32);
    pub const Selection: Self = Self(1i32);
    pub const Text: Self = Self(2i32);
    pub const Property: Self = Self(3i32);
    pub const Layout: Self = Self(4i32);
}
impl windows_core::TypeKind for TextBoxContentAttribute {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextBoxContentAttribute {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.TextBoxContentAttribute;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBoxContentChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextBoxContentChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TextBoxContentChangedEventArgs {
    pub fn TextBoxId(&self) -> windows_core::Result<TextBoxId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBoxId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Source(&self) -> windows_core::Result<TextChangeSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SelectionBounds(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionBounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsContentAttributeChanged(&self, value: TextBoxContentAttribute) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsContentAttributeChanged)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TextBoxContentChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextBoxContentChangedEventArgs>();
}
unsafe impl windows_core::Interface for TextBoxContentChangedEventArgs {
    type Vtable = <ITextBoxContentChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextBoxContentChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextBoxContentChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextBoxContentChangedEventArgs";
}
unsafe impl Send for TextBoxContentChangedEventArgs {}
unsafe impl Sync for TextBoxContentChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextBoxFeatures(pub u32);
impl TextBoxFeatures {
    pub const None: Self = Self(0u32);
    pub const ReadText: Self = Self(1u32);
    pub const WriteText: Self = Self(2u32);
    pub const AugmentText: Self = Self(4u32);
}
impl windows_core::TypeKind for TextBoxFeatures {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextBoxFeatures {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.TextBoxFeatures;u4)");
}
impl TextBoxFeatures {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextBoxFeatures {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextBoxFeatures {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextBoxFeatures {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TextBoxFeatures {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TextBoxFeatures {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextBoxId {
    pub Value: u32,
}
impl windows_core::TypeKind for TextBoxId {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextBoxId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Text.TextBoxId;u4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBoxInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextBoxInfo, windows_core::IUnknown, windows_core::IInspectable);
impl TextBoxInfo {
    pub fn Id(&self) -> windows_core::Result<TextBoxId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn InputScope(&self) -> windows_core::Result<super::super::super::Text::Core::CoreTextInputScope> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Url(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Url)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Settings(&self) -> windows_core::Result<TextBoxSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Settings)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisabledFeatures(&self) -> windows_core::Result<TextBoxFeatures> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisabledFeatures)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TextBoxInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextBoxInfo>();
}
unsafe impl windows_core::Interface for TextBoxInfo {
    type Vtable = <ITextBoxInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextBoxInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextBoxInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextBoxInfo";
}
unsafe impl Send for TextBoxInfo {}
unsafe impl Sync for TextBoxInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBoxInfoChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextBoxInfoChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TextBoxInfoChangedEventArgs {
    pub fn TextBoxInfo(&self) -> windows_core::Result<TextBoxInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBoxInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TextBoxInfoChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextBoxInfoChangedEventArgs>();
}
unsafe impl windows_core::Interface for TextBoxInfoChangedEventArgs {
    type Vtable = <ITextBoxInfoChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextBoxInfoChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextBoxInfoChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextBoxInfoChangedEventArgs";
}
unsafe impl Send for TextBoxInfoChangedEventArgs {}
unsafe impl Sync for TextBoxInfoChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextBoxSettings(pub u32);
impl TextBoxSettings {
    pub const None: Self = Self(0u32);
    pub const Private: Self = Self(1u32);
    pub const Multiline: Self = Self(2u32);
    pub const VerticalWriting: Self = Self(4u32);
}
impl windows_core::TypeKind for TextBoxSettings {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextBoxSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.TextBoxSettings;u4)");
}
impl TextBoxSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextBoxSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextBoxSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextBoxSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TextBoxSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TextBoxSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextChangeSource(pub i32);
impl TextChangeSource {
    pub const External: Self = Self(0i32);
    pub const HardwareKeyTyped: Self = Self(1i32);
    pub const SoftwareKeyTyped: Self = Self(2i32);
    pub const KeyboardImeInsertion: Self = Self(3i32);
    pub const OtherImeInsertion: Self = Self(4i32);
    pub const Reconversion: Self = Self(5i32);
    pub const AutoCompletion: Self = Self(6i32);
    pub const Mixed: Self = Self(7i32);
}
impl windows_core::TypeKind for TextChangeSource {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextChangeSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.TextChangeSource;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextComposition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextComposition, windows_core::IUnknown, windows_core::IInspectable);
impl TextComposition {
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FirstSegment(&self) -> windows_core::Result<TextCompositionSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstSegment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectedSegment(&self) -> windows_core::Result<TextCompositionSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedSegment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CaretPosition(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaretPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCaretPosition(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCaretPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InsertText(&self, text: &windows_core::HSTRING) -> windows_core::Result<TextCompositionSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CompleteUnconverted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CompleteUnconverted)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CompleteFirstSegment(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CompleteFirstSegment)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for TextComposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextComposition>();
}
unsafe impl windows_core::Interface for TextComposition {
    type Vtable = <ITextComposition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextComposition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextComposition {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextComposition";
}
unsafe impl Send for TextComposition {}
unsafe impl Sync for TextComposition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextCompositionSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextCompositionSegment, windows_core::IUnknown, windows_core::IInspectable);
impl TextCompositionSegment {
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ConvertedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConvertedText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetConvertedText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetConvertedText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UnconvertedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnconvertedText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetUnconvertedText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnconvertedText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn Range(&self) -> windows_core::Result<super::super::super::Text::Core::CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Range)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn ConversionState(&self) -> windows_core::Result<super::super::super::Text::Core::CoreTextFormatUpdatingReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConversionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn SetConversionState(&self, value: super::super::super::Text::Core::CoreTextFormatUpdatingReason) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetConversionState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Next(&self) -> windows_core::Result<TextCompositionSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Next)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Previous(&self) -> windows_core::Result<TextCompositionSegment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Previous)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Text")]
    pub fn GetTextStyle(&self) -> windows_core::Result<TextStyle> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTextStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    pub fn SetTextStyle(&self, value: TextStyle) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for TextCompositionSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextCompositionSegment>();
}
unsafe impl windows_core::Interface for TextCompositionSegment {
    type Vtable = <ITextCompositionSegment as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextCompositionSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextCompositionSegment";
}
unsafe impl Send for TextCompositionSegment {}
unsafe impl Sync for TextCompositionSegment {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextConversionMode(pub i32);
impl TextConversionMode {
    pub const Undefined: Self = Self(0i32);
    pub const AlphanumericHalfWidth: Self = Self(1i32);
    pub const AlphanumericFullWidth: Self = Self(2i32);
    pub const NativeHalfWidth: Self = Self(3i32);
    pub const NativeFullWidth: Self = Self(4i32);
    pub const KatakanaHalfWidth: Self = Self(5i32);
    pub const KatakanaFullWidth: Self = Self(6i32);
    pub const NativeHalfWidthNativeSymbol: Self = Self(7i32);
    pub const NativeFullWidthNativeSymbol: Self = Self(8i32);
    pub const NoConversion: Self = Self(9i32);
    pub const RequestConversion: Self = Self(10i32);
    pub const NativeEudc: Self = Self(11i32);
}
impl windows_core::TypeKind for TextConversionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextConversionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.TextConversionMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextEditSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextEditSession, windows_core::IUnknown, windows_core::IInspectable);
impl TextEditSession {
    pub fn TextBoxId(&self) -> windows_core::Result<TextBoxId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBoxId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TextLength(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn PopulatedRange(&self) -> windows_core::Result<super::super::super::Text::Core::CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PopulatedRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn PopulateAsync(&self, range: super::super::super::Text::Core::CoreTextRange) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PopulateAsync)(windows_core::Interface::as_raw(this), range, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn GetText(&self, range: super::super::super::Text::Core::CoreTextRange) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetText)(windows_core::Interface::as_raw(this), range, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn GetSelectedRange(&self) -> windows_core::Result<super::super::super::Text::Core::CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSelectedRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn SetSelectedRange(&self, value: super::super::super::Text::Core::CoreTextRange) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelectedRange)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn ReplaceText(&self, replacerange: super::super::super::Text::Core::CoreTextRange, text: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Text::Core::CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceText)(windows_core::Interface::as_raw(this), replacerange, core::mem::transmute_copy(text), &mut result__).map(|| result__)
        }
    }
    pub fn Composition(&self) -> windows_core::Result<TextComposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Composition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartComposition(&self) -> windows_core::Result<TextComposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartComposition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn StartReconversion(&self, range: super::super::super::Text::Core::CoreTextRange) -> windows_core::Result<TextComposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartReconversion)(windows_core::Interface::as_raw(this), range, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubmitPayload(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubmitPayload)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubmitPayloadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<PayloadResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubmitPayloadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TextEditSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextEditSession>();
}
unsafe impl windows_core::Interface for TextEditSession {
    type Vtable = <ITextEditSession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextEditSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextEditSession {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextEditSession";
}
unsafe impl Send for TextEditSession {}
unsafe impl Sync for TextEditSession {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextInputProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextInputProvider, windows_core::IUnknown, windows_core::IInspectable);
impl TextInputProvider {
    pub fn GetSubscription(&self) -> windows_core::Result<TextInputServiceSubscription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSubscription)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSubscription(&self, subscription: TextInputServiceSubscription) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubscription)(windows_core::Interface::as_raw(this), subscription).ok() }
    }
    pub fn HasFocusedTextBox(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasFocusedTextBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FocusedTextBoxId(&self) -> windows_core::Result<TextBoxId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusedTextBoxId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FocusedTextBoxInfo(&self) -> windows_core::Result<TextBoxInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusedTextBoxInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FocusedTextBoxBounds(&self) -> windows_core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusedTextBoxBounds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectionBounds(&self) -> windows_core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionBounds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateEditSession(&self) -> windows_core::Result<TextEditSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEditSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryStartDelegation(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStartDelegation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StopDelegation(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopDelegation)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FocusEntered<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<TextInputProvider, FocusEnteredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFocusEntered(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEntered)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn FocusRemoved<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<TextInputProvider, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFocusRemoved(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TextBoxInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<TextInputProvider, TextBoxInfoChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBoxInfoChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextBoxInfoChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextBoxInfoChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TextBoxContentChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<TextInputProvider, TextBoxContentChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBoxContentChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextBoxContentChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextBoxContentChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CompositionTerminated<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<TextInputProvider, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositionTerminated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompositionTerminated(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompositionTerminated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReconversionRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<TextInputProvider, ReconversionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReconversionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReconversionRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReconversionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn InputDelegationModeChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<TextInputProvider, InputDelegationModeChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputDelegationModeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputDelegationModeChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputDelegationModeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for TextInputProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextInputProvider>();
}
unsafe impl windows_core::Interface for TextInputProvider {
    type Vtable = <ITextInputProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextInputProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextInputProvider {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextInputProvider";
}
unsafe impl Send for TextInputProvider {}
unsafe impl Sync for TextInputProvider {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextInputService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextInputService, windows_core::IUnknown, windows_core::IInspectable);
impl TextInputService {
    pub fn CreateKeyboardInputProcessor(&self, inputprofile: &windows_core::HSTRING) -> windows_core::Result<KeyboardInputProcessor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateKeyboardInputProcessor)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(inputprofile), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTextInputProvider(&self, inputprofile: &windows_core::HSTRING) -> windows_core::Result<TextInputProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextInputProvider)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(inputprofile), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForCurrentThread() -> windows_core::Result<TextInputService> {
        Self::ITextInputServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentThread)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ITextInputServiceStatics<R, F: FnOnce(&ITextInputServiceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextInputService, ITextInputServiceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextInputService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextInputService>();
}
unsafe impl windows_core::Interface for TextInputService {
    type Vtable = <ITextInputService as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextInputService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextInputService {
    const NAME: &'static str = "Windows.UI.Input.Preview.Text.TextInputService";
}
unsafe impl Send for TextInputService {}
unsafe impl Sync for TextInputService {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextInputServiceSubscription {
    pub requiredEnabledFeatures: TextBoxFeatures,
    pub requiredDisabledFeatures: TextBoxFeatures,
}
impl windows_core::TypeKind for TextInputServiceSubscription {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextInputServiceSubscription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Text.TextInputServiceSubscription;enum(Windows.UI.Input.Preview.Text.TextBoxFeatures;u4);enum(Windows.UI.Input.Preview.Text.TextBoxFeatures;u4))");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextStyle {
    pub mask: TextStyleAttributes,
    pub textColor: super::super::super::Color,
    pub backgroundColor: super::super::super::Color,
    pub underlineColor: super::super::super::Color,
    pub underlineType: super::super::super::Text::UnderlineType,
}
impl windows_core::TypeKind for TextStyle {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Text.TextStyle;enum(Windows.UI.Input.Preview.Text.TextStyleAttributes;u4);struct(Windows.UI.Color;u1;u1;u1;u1);struct(Windows.UI.Color;u1;u1;u1;u1);struct(Windows.UI.Color;u1;u1;u1;u1);enum(Windows.UI.Text.UnderlineType;i4))");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextStyleAttributes(pub u32);
impl TextStyleAttributes {
    pub const None: Self = Self(0u32);
    pub const TextColor: Self = Self(1u32);
    pub const BackgroundColor: Self = Self(2u32);
    pub const UnderlineColor: Self = Self(4u32);
    pub const UnderlineType: Self = Self(8u32);
}
impl windows_core::TypeKind for TextStyleAttributes {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TextStyleAttributes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Text.TextStyleAttributes;u4)");
}
impl TextStyleAttributes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TextStyleAttributes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TextStyleAttributes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TextStyleAttributes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TextStyleAttributes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TextStyleAttributes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
