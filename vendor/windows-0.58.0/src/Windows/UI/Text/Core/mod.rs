windows_core::imp::define_interface!(ICoreTextCompositionCompletedEventArgs, ICoreTextCompositionCompletedEventArgs_Vtbl, 0x1f34ebb6_b79f_4121_a5e7_fda9b8616e30);
impl windows_core::RuntimeType for ICoreTextCompositionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextCompositionCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CompositionSegments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CompositionSegments: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextCompositionSegment, ICoreTextCompositionSegment_Vtbl, 0x776c6bd9_4ead_4da7_8f47_3a88b523cc34);
impl windows_core::RuntimeType for ICoreTextCompositionSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextCompositionSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreconversionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextCompositionStartedEventArgs, ICoreTextCompositionStartedEventArgs_Vtbl, 0x276b16a9_64e7_4ab0_bc4b_a02d73835bfb);
impl windows_core::RuntimeType for ICoreTextCompositionStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextCompositionStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextEditContext, ICoreTextEditContext_Vtbl, 0xbf6608af_4041_47c3_b263_a918eb5eaef2);
impl windows_core::RuntimeType for ICoreTextEditContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextEditContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub InputScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextInputScope) -> windows_core::HRESULT,
    pub SetInputScope: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextInputScope) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InputPaneDisplayPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextInputPaneDisplayPolicy) -> windows_core::HRESULT,
    pub SetInputPaneDisplayPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextInputPaneDisplayPolicy) -> windows_core::HRESULT,
    pub TextRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTextRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SelectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSelectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub LayoutRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLayoutRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TextUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTextUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SelectionUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSelectionUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub FormatUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFormatUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CompositionStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCompositionStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CompositionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCompositionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub FocusRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFocusRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NotifyFocusEnter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyFocusLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyTextChanged: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextRange, i32, CoreTextRange) -> windows_core::HRESULT,
    pub NotifySelectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextRange) -> windows_core::HRESULT,
    pub NotifyLayoutChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextEditContext2, ICoreTextEditContext2_Vtbl, 0xb1867dbb_083b_49e1_b281_2b35d62bf466);
impl windows_core::RuntimeType for ICoreTextEditContext2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextEditContext2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotifyFocusLeaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNotifyFocusLeaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextFormatUpdatingEventArgs, ICoreTextFormatUpdatingEventArgs_Vtbl, 0x7310bd33_b4a8_43b1_b37b_0724d4aca7ab);
impl windows_core::RuntimeType for ICoreTextFormatUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextFormatUpdatingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
    #[cfg(feature = "UI_ViewManagement")]
    pub TextColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    TextColor: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub UnderlineColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    UnderlineColor: usize,
    pub UnderlineType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextFormatUpdatingReason) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextFormatUpdatingResult) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextFormatUpdatingResult) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextLayoutBounds, ICoreTextLayoutBounds_Vtbl, 0xe972c974_4436_4917_80d0_a525e4ca6780);
impl windows_core::RuntimeType for ICoreTextLayoutBounds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextLayoutBounds_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetTextBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub ControlBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetControlBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextLayoutRequest, ICoreTextLayoutRequest_Vtbl, 0x2555a8cc_51fd_4f03_98bf_ac78174d68e0);
impl windows_core::RuntimeType for ICoreTextLayoutRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextLayoutRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
    pub LayoutBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextLayoutRequest2, ICoreTextLayoutRequest2_Vtbl, 0x676de624_cd3d_4bcd_bf01_7f7110954511);
impl windows_core::RuntimeType for ICoreTextLayoutRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextLayoutRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LayoutBoundsVisualPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextLayoutRequestedEventArgs, ICoreTextLayoutRequestedEventArgs_Vtbl, 0xb1dc6ae0_9a7b_4e9e_a566_4a6b5f8ad676);
impl windows_core::RuntimeType for ICoreTextLayoutRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextLayoutRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextSelectionRequest, ICoreTextSelectionRequest_Vtbl, 0xf0a70403_208b_4301_883c_74ca7485fd8d);
impl windows_core::RuntimeType for ICoreTextSelectionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextSelectionRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Selection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextRange) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextSelectionRequestedEventArgs, ICoreTextSelectionRequestedEventArgs_Vtbl, 0x13c6682b_f614_421a_8f4b_9ec8a5a37fcd);
impl windows_core::RuntimeType for ICoreTextSelectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextSelectionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextSelectionUpdatingEventArgs, ICoreTextSelectionUpdatingEventArgs_Vtbl, 0xd445839f_fe7f_4bd5_8a26_0922c1b3e639);
impl windows_core::RuntimeType for ICoreTextSelectionUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextSelectionUpdatingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Selection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextSelectionUpdatingResult) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextSelectionUpdatingResult) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextServicesManager, ICoreTextServicesManager_Vtbl, 0xc2507d83_6e0a_4a8a_bdf8_1948874854ba);
impl windows_core::RuntimeType for ICoreTextServicesManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextServicesManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    pub InputLanguageChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInputLanguageChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateEditContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextServicesManagerStatics, ICoreTextServicesManagerStatics_Vtbl, 0x1520a388_e2cf_4d65_aeb9_b32d86fe39b9);
impl windows_core::RuntimeType for ICoreTextServicesManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextServicesManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextServicesStatics, ICoreTextServicesStatics_Vtbl, 0x91859a46_eccf_47a4_8ae7_098a9c6fbb15);
impl windows_core::RuntimeType for ICoreTextServicesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextServicesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HiddenCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextTextRequest, ICoreTextTextRequest_Vtbl, 0x50d950a9_f51e_4cc1_8ca1_e6346d1a61be);
impl windows_core::RuntimeType for ICoreTextTextRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextTextRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextTextRequestedEventArgs, ICoreTextTextRequestedEventArgs_Vtbl, 0xf096a2d0_41c6_4c02_8b1a_d953b00cabb3);
impl windows_core::RuntimeType for ICoreTextTextRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextTextRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTextTextUpdatingEventArgs, ICoreTextTextUpdatingEventArgs_Vtbl, 0xeea7918d_cc2b_4f03_8ff6_02fd217db450);
impl windows_core::RuntimeType for ICoreTextTextUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTextTextUpdatingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Range: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NewSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextRange) -> windows_core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub InputLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    InputLanguage: usize,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreTextTextUpdatingResult) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(*mut core::ffi::c_void, CoreTextTextUpdatingResult) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextCompositionCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextCompositionCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextCompositionCompletedEventArgs {
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CompositionSegments(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositionSegments)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextCompositionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextCompositionCompletedEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextCompositionCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextCompositionCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs";
}
unsafe impl Send for CoreTextCompositionCompletedEventArgs {}
unsafe impl Sync for CoreTextCompositionCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextCompositionSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextCompositionSegment, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextCompositionSegment {
    pub fn PreconversionString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreconversionString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Range(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Range)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreTextCompositionSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextCompositionSegment>();
}
unsafe impl windows_core::Interface for CoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_Vtbl;
    const IID: windows_core::GUID = <ICoreTextCompositionSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionSegment";
}
unsafe impl Send for CoreTextCompositionSegment {}
unsafe impl Sync for CoreTextCompositionSegment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextCompositionStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextCompositionStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextCompositionStartedEventArgs {
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextCompositionStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextCompositionStartedEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextCompositionStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextCompositionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs";
}
unsafe impl Send for CoreTextCompositionStartedEventArgs {}
unsafe impl Sync for CoreTextCompositionStartedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextEditContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextEditContext, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextEditContext {
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
    pub fn InputScope(&self) -> windows_core::Result<CoreTextInputScope> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInputScope(&self, value: CoreTextInputScope) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInputScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsReadOnly(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsReadOnly)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputPaneDisplayPolicy(&self) -> windows_core::Result<CoreTextInputPaneDisplayPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputPaneDisplayPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInputPaneDisplayPolicy(&self, value: CoreTextInputPaneDisplayPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInputPaneDisplayPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TextRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextRequested(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SelectionRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSelectionRequested(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSelectionRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn LayoutRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLayoutRequested(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLayoutRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn TextUpdating<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextUpdating)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextUpdating(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextUpdating)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SelectionUpdating<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionUpdating)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSelectionUpdating(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSelectionUpdating)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn FormatUpdating<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FormatUpdating)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFormatUpdating(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFormatUpdating)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CompositionStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositionStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompositionStarted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompositionStarted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CompositionCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositionCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompositionCompleted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompositionCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn FocusRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFocusRemoved(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusRemoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn NotifyFocusEnter(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyFocusEnter)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyFocusLeave(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyFocusLeave)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyTextChanged(&self, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyTextChanged)(windows_core::Interface::as_raw(this), modifiedrange, newlength, newselection).ok() }
    }
    pub fn NotifySelectionChanged(&self, selection: CoreTextRange) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifySelectionChanged)(windows_core::Interface::as_raw(this), selection).ok() }
    }
    pub fn NotifyLayoutChanged(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyLayoutChanged)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyFocusLeaveCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotifyFocusLeaveCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNotifyFocusLeaveCompleted(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveNotifyFocusLeaveCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for CoreTextEditContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextEditContext>();
}
unsafe impl windows_core::Interface for CoreTextEditContext {
    type Vtable = ICoreTextEditContext_Vtbl;
    const IID: windows_core::GUID = <ICoreTextEditContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextEditContext {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextEditContext";
}
unsafe impl Send for CoreTextEditContext {}
unsafe impl Sync for CoreTextEditContext {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextFormatUpdatingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextFormatUpdatingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextFormatUpdatingEventArgs {
    pub fn Range(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Range)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn TextColor(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn UnderlineColor(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnderlineColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnderlineType(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::UnderlineType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnderlineType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reason(&self) -> windows_core::Result<CoreTextFormatUpdatingReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Result(&self) -> windows_core::Result<CoreTextFormatUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextFormatUpdatingResult) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResult)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextFormatUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextFormatUpdatingEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextFormatUpdatingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextFormatUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs";
}
unsafe impl Send for CoreTextFormatUpdatingEventArgs {}
unsafe impl Sync for CoreTextFormatUpdatingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextLayoutBounds(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextLayoutBounds, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextLayoutBounds {
    pub fn TextBounds(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextBounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTextBounds(&self, value: super::super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextBounds)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ControlBounds(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlBounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetControlBounds(&self, value: super::super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetControlBounds)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CoreTextLayoutBounds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextLayoutBounds>();
}
unsafe impl windows_core::Interface for CoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_Vtbl;
    const IID: windows_core::GUID = <ICoreTextLayoutBounds as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextLayoutBounds {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutBounds";
}
unsafe impl Send for CoreTextLayoutBounds {}
unsafe impl Sync for CoreTextLayoutBounds {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextLayoutRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextLayoutRequest, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextLayoutRequest {
    pub fn Range(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Range)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LayoutBounds(&self) -> windows_core::Result<CoreTextLayoutBounds> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutBounds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LayoutBoundsVisualPixels(&self) -> windows_core::Result<CoreTextLayoutBounds> {
        let this = &windows_core::Interface::cast::<ICoreTextLayoutRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LayoutBoundsVisualPixels)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextLayoutRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextLayoutRequest>();
}
unsafe impl windows_core::Interface for CoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_Vtbl;
    const IID: windows_core::GUID = <ICoreTextLayoutRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextLayoutRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequest";
}
unsafe impl Send for CoreTextLayoutRequest {}
unsafe impl Sync for CoreTextLayoutRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextLayoutRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextLayoutRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextLayoutRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<CoreTextLayoutRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextLayoutRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextLayoutRequestedEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextLayoutRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextLayoutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs";
}
unsafe impl Send for CoreTextLayoutRequestedEventArgs {}
unsafe impl Sync for CoreTextLayoutRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextSelectionRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextSelectionRequest, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextSelectionRequest {
    pub fn Selection(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Selection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSelection(&self, value: CoreTextRange) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSelection)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextSelectionRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextSelectionRequest>();
}
unsafe impl windows_core::Interface for CoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_Vtbl;
    const IID: windows_core::GUID = <ICoreTextSelectionRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextSelectionRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequest";
}
unsafe impl Send for CoreTextSelectionRequest {}
unsafe impl Sync for CoreTextSelectionRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextSelectionRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextSelectionRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextSelectionRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<CoreTextSelectionRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextSelectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextSelectionRequestedEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextSelectionRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextSelectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs";
}
unsafe impl Send for CoreTextSelectionRequestedEventArgs {}
unsafe impl Sync for CoreTextSelectionRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextSelectionUpdatingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextSelectionUpdatingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextSelectionUpdatingEventArgs {
    pub fn Selection(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Selection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Result(&self) -> windows_core::Result<CoreTextSelectionUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextSelectionUpdatingResult) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResult)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextSelectionUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextSelectionUpdatingEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextSelectionUpdatingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextSelectionUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs";
}
unsafe impl Send for CoreTextSelectionUpdatingEventArgs {}
unsafe impl Sync for CoreTextSelectionUpdatingEventArgs {}
pub struct CoreTextServicesConstants;
impl CoreTextServicesConstants {
    pub fn HiddenCharacter() -> windows_core::Result<u16> {
        Self::ICoreTextServicesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HiddenCharacter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreTextServicesStatics<R, F: FnOnce(&ICoreTextServicesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreTextServicesConstants, ICoreTextServicesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CoreTextServicesConstants {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesConstants";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextServicesManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextServicesManager, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextServicesManager {
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> windows_core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InputLanguageChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputLanguageChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputLanguageChanged(&self, cookie: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputLanguageChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CreateEditContext(&self) -> windows_core::Result<CoreTextEditContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEditContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<CoreTextServicesManager> {
        Self::ICoreTextServicesManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreTextServicesManagerStatics<R, F: FnOnce(&ICoreTextServicesManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreTextServicesManager, ICoreTextServicesManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreTextServicesManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextServicesManager>();
}
unsafe impl windows_core::Interface for CoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_Vtbl;
    const IID: windows_core::GUID = <ICoreTextServicesManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextServicesManager {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesManager";
}
unsafe impl Send for CoreTextServicesManager {}
unsafe impl Sync for CoreTextServicesManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextTextRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextTextRequest, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextTextRequest {
    pub fn Range(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Range)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextTextRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextTextRequest>();
}
unsafe impl windows_core::Interface for CoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_Vtbl;
    const IID: windows_core::GUID = <ICoreTextTextRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextTextRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequest";
}
unsafe impl Send for CoreTextTextRequest {}
unsafe impl Sync for CoreTextTextRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextTextRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextTextRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextTextRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<CoreTextTextRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextTextRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextTextRequestedEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextTextRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextTextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequestedEventArgs";
}
unsafe impl Send for CoreTextTextRequestedEventArgs {}
unsafe impl Sync for CoreTextTextRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreTextTextUpdatingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreTextTextUpdatingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreTextTextUpdatingEventArgs {
    pub fn Range(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Range)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NewSelection(&self) -> windows_core::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewSelection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> windows_core::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputLanguage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Result(&self) -> windows_core::Result<CoreTextTextUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextTextUpdatingResult) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResult)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CoreTextTextUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreTextTextUpdatingEventArgs>();
}
unsafe impl windows_core::Interface for CoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreTextTextUpdatingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreTextTextUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs";
}
unsafe impl Send for CoreTextTextUpdatingEventArgs {}
unsafe impl Sync for CoreTextTextUpdatingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreTextFormatUpdatingReason(pub i32);
impl CoreTextFormatUpdatingReason {
    pub const None: Self = Self(0i32);
    pub const CompositionUnconverted: Self = Self(1i32);
    pub const CompositionConverted: Self = Self(2i32);
    pub const CompositionTargetUnconverted: Self = Self(3i32);
    pub const CompositionTargetConverted: Self = Self(4i32);
}
impl windows_core::TypeKind for CoreTextFormatUpdatingReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreTextFormatUpdatingReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreTextFormatUpdatingReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl windows_core::TypeKind for CoreTextFormatUpdatingResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreTextFormatUpdatingResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreTextFormatUpdatingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl windows_core::TypeKind for CoreTextInputPaneDisplayPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreTextInputPaneDisplayPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreTextInputPaneDisplayPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreTextInputPaneDisplayPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputPaneDisplayPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
    pub const FileName: Self = Self(3i32);
    pub const EmailUserName: Self = Self(4i32);
    pub const EmailAddress: Self = Self(5i32);
    pub const UserName: Self = Self(6i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const PersonalNamePrefix: Self = Self(8i32);
    pub const PersonalGivenName: Self = Self(9i32);
    pub const PersonalMiddleName: Self = Self(10i32);
    pub const PersonalSurname: Self = Self(11i32);
    pub const PersonalNameSuffix: Self = Self(12i32);
    pub const Address: Self = Self(13i32);
    pub const AddressPostalCode: Self = Self(14i32);
    pub const AddressStreet: Self = Self(15i32);
    pub const AddressStateOrProvince: Self = Self(16i32);
    pub const AddressCity: Self = Self(17i32);
    pub const AddressCountryName: Self = Self(18i32);
    pub const AddressCountryShortName: Self = Self(19i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const Date: Self = Self(22i32);
    pub const DateMonth: Self = Self(23i32);
    pub const DateDay: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const DateMonthName: Self = Self(26i32);
    pub const DateDayName: Self = Self(27i32);
    pub const Number: Self = Self(29i32);
    pub const SingleCharacter: Self = Self(30i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const Time: Self = Self(36i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinuteOrSecond: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const CurrencyChinese: Self = Self(42i32);
    pub const Bopomofo: Self = Self(43i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailUserNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const PasswordNumeric: Self = Self(63i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
    pub const Digits: Self = Self(28i32);
    pub const PinNumeric: Self = Self(64i32);
    pub const PinAlphanumeric: Self = Self(65i32);
}
impl windows_core::TypeKind for CoreTextInputScope {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreTextInputScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreTextInputScope").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreTextInputScope {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputScope;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreTextSelectionUpdatingResult(pub i32);
impl CoreTextSelectionUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl windows_core::TypeKind for CoreTextSelectionUpdatingResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreTextSelectionUpdatingResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreTextSelectionUpdatingResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreTextSelectionUpdatingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextSelectionUpdatingResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreTextTextUpdatingResult(pub i32);
impl CoreTextTextUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl windows_core::TypeKind for CoreTextTextUpdatingResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreTextTextUpdatingResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreTextTextUpdatingResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreTextTextUpdatingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextTextUpdatingResult;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoreTextRange {
    pub StartCaretPosition: i32,
    pub EndCaretPosition: i32,
}
impl windows_core::TypeKind for CoreTextRange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CoreTextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Text.Core.CoreTextRange;i4;i4)");
}
impl Default for CoreTextRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
