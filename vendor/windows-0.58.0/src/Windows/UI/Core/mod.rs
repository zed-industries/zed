#[cfg(feature = "UI_Core_AnimationMetrics")]
pub mod AnimationMetrics;
#[cfg(feature = "UI_Core_Preview")]
pub mod Preview;
windows_core::imp::define_interface!(IAcceleratorKeyEventArgs, IAcceleratorKeyEventArgs_Vtbl, 0xff1c4c4a_9287_470b_836e_9086e3126ade);
impl windows_core::RuntimeType for IAcceleratorKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAcceleratorKeyEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreAcceleratorKeyEventType) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CorePhysicalKeyStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAcceleratorKeyEventArgs2, IAcceleratorKeyEventArgs2_Vtbl, 0xd300a9f6_2f7e_4873_a555_166e596ee1c5);
impl windows_core::RuntimeType for IAcceleratorKeyEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAcceleratorKeyEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAutomationProviderRequestedEventArgs, IAutomationProviderRequestedEventArgs_Vtbl, 0x961ff258_21bf_4b42_a298_fa479d4c52e2);
impl windows_core::RuntimeType for IAutomationProviderRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAutomationProviderRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutomationProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAutomationProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackRequestedEventArgs, IBackRequestedEventArgs_Vtbl, 0xd603d28a_e411_4a4e_ba41_6a327a8675bc);
impl windows_core::RuntimeType for IBackRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICharacterReceivedEventArgs, ICharacterReceivedEventArgs_Vtbl, 0xc584659f_99b2_4bcc_bd33_04e63f42902e);
impl windows_core::RuntimeType for ICharacterReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICharacterReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CorePhysicalKeyStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClosestInteractiveBoundsRequestedEventArgs, IClosestInteractiveBoundsRequestedEventArgs_Vtbl, 0x347c11d7_f6f8_40e3_b29f_ae50d3e86486);
impl windows_core::RuntimeType for IClosestInteractiveBoundsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClosestInteractiveBoundsRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub SearchBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub ClosestInteractiveBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetClosestInteractiveBounds: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreAcceleratorKeys, ICoreAcceleratorKeys_Vtbl, 0x9ffdf7f5_b8c9_4ef0_b7d2_1de626561fc8);
impl core::ops::Deref for ICoreAcceleratorKeys {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreAcceleratorKeys, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreAcceleratorKeys {
    pub fn AcceleratorKeyActivated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceleratorKeyActivated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAcceleratorKeyActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAcceleratorKeyActivated)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for ICoreAcceleratorKeys {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreAcceleratorKeys_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcceleratorKeyActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAcceleratorKeyActivated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreClosestInteractiveBoundsRequested, ICoreClosestInteractiveBoundsRequested_Vtbl, 0xf303043a_e8bf_4e8e_ae69_c9dadd57a114);
impl windows_core::RuntimeType for ICoreClosestInteractiveBoundsRequested {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreClosestInteractiveBoundsRequested_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClosestInteractiveBoundsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosestInteractiveBoundsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreComponentFocusable, ICoreComponentFocusable_Vtbl, 0x52f96fa3_8742_4411_ae69_79a85f29ac8b);
impl windows_core::RuntimeType for ICoreComponentFocusable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreComponentFocusable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreCursor, ICoreCursor_Vtbl, 0x96893acf_111d_442c_8a77_b87992f8e2d6);
impl windows_core::RuntimeType for ICoreCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreCursorType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreCursorFactory, ICoreCursorFactory_Vtbl, 0xf6359621_a79d_4ed3_8c32_a9ef9d6b76a4);
impl windows_core::RuntimeType for ICoreCursorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreCursorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCursor: unsafe extern "system" fn(*mut core::ffi::c_void, CoreCursorType, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDispatcher, ICoreDispatcher_Vtbl, 0x60db2fa8_b705_4fde_a7d6_ebbb1891d39e);
impl windows_core::RuntimeType for ICoreDispatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDispatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasThreadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ProcessEvents: unsafe extern "system" fn(*mut core::ffi::c_void, CoreProcessEventsOption) -> windows_core::HRESULT,
    pub RunAsync: unsafe extern "system" fn(*mut core::ffi::c_void, CoreDispatcherPriority, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunIdleAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDispatcher2, ICoreDispatcher2_Vtbl, 0x6f5e63c7_e3aa_4eae_b0e0_dcf321ca4b2f);
impl windows_core::RuntimeType for ICoreDispatcher2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDispatcher2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryRunAsync: unsafe extern "system" fn(*mut core::ffi::c_void, CoreDispatcherPriority, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryRunIdleAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDispatcherWithTaskPriority, ICoreDispatcherWithTaskPriority_Vtbl, 0xbafaecad_484d_41be_ba80_1d58c65263ea);
impl windows_core::RuntimeType for ICoreDispatcherWithTaskPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDispatcherWithTaskPriority_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreDispatcherPriority) -> windows_core::HRESULT,
    pub SetCurrentPriority: unsafe extern "system" fn(*mut core::ffi::c_void, CoreDispatcherPriority) -> windows_core::HRESULT,
    pub ShouldYield: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShouldYieldToPriority: unsafe extern "system" fn(*mut core::ffi::c_void, CoreDispatcherPriority, *mut bool) -> windows_core::HRESULT,
    pub StopProcessEvents: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreIndependentInputSourceController, ICoreIndependentInputSourceController_Vtbl, 0x0963261c_84fe_578a_83ca_6425309ccde4);
impl windows_core::RuntimeType for ICoreIndependentInputSourceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreIndependentInputSourceController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTransparentForUncontrolledInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsTransparentForUncontrolledInput: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPalmRejectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPalmRejectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetControlledInput: unsafe extern "system" fn(*mut core::ffi::c_void, CoreInputDeviceTypes) -> windows_core::HRESULT,
    pub SetControlledInputWithFilters: unsafe extern "system" fn(*mut core::ffi::c_void, CoreInputDeviceTypes, CoreIndependentInputFilters, CoreIndependentInputFilters) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreIndependentInputSourceControllerStatics, ICoreIndependentInputSourceControllerStatics_Vtbl, 0x3edc4e20_9a8a_5691_8586_fca4cb57526d);
impl windows_core::RuntimeType for ICoreIndependentInputSourceControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreIndependentInputSourceControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateForIVisualElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForIVisualElement: usize,
}
windows_core::imp::define_interface!(ICoreInputSourceBase, ICoreInputSourceBase_Vtbl, 0x9f488807_4580_4be8_be68_92a9311713bb);
impl core::ops::Deref for ICoreInputSourceBase {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreInputSourceBase, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreInputSourceBase {
    pub fn Dispatcher(&self) -> windows_core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    pub fn InputEnabled<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, InputEnabledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputEnabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputEnabled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for ICoreInputSourceBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInputSourceBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreKeyboardInputSource, ICoreKeyboardInputSource_Vtbl, 0x231c9088_e469_4df1_b208_6e490d71cb90);
impl windows_core::RuntimeType for ICoreKeyboardInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreKeyboardInputSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetCurrentKeyState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey, *mut CoreVirtualKeyStates) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetCurrentKeyState: usize,
    pub CharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreKeyboardInputSource2, ICoreKeyboardInputSource2_Vtbl, 0xfa24cb94_f963_47a5_8778_207c482b0afd);
impl windows_core::RuntimeType for ICoreKeyboardInputSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreKeyboardInputSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentKeyEventDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorePointerInputSource, ICorePointerInputSource_Vtbl, 0xbbf1bb18_e47a_48eb_8807_f8f8d3ea4551);
impl core::ops::Deref for ICorePointerInputSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorePointerInputSource, windows_core::IUnknown, windows_core::IInspectable);
impl ICorePointerInputSource {
    pub fn ReleasePointerCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCapture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerCursor(&self) -> windows_core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPointerCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreCursor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExited)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for ICorePointerInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICorePointerInputSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReleasePointerCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPointerCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasCapture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PointerPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub PointerCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPointerCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorePointerInputSource2, ICorePointerInputSource2_Vtbl, 0xd703708a_4516_4786_b1e5_2751d563f997);
impl core::ops::Deref for ICorePointerInputSource2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorePointerInputSource2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ICorePointerInputSource2, ICorePointerInputSource);
impl ICorePointerInputSource2 {
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReleasePointerCapture(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCapture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerCursor(&self) -> windows_core::Result<CoreCursor> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPointerCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreCursor>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExited)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for ICorePointerInputSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICorePointerInputSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
windows_core::imp::define_interface!(ICorePointerRedirector, ICorePointerRedirector_Vtbl, 0x8f9d0c94_5688_4b0c_a9f1_f931f7fa3dc3);
impl core::ops::Deref for ICorePointerRedirector {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorePointerRedirector, windows_core::IUnknown, windows_core::IInspectable);
impl ICorePointerRedirector {
    pub fn PointerRoutedAway<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedAway)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedAway(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedAway)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerRoutedTo<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedTo)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedTo(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedTo)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerRoutedReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for ICorePointerRedirector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICorePointerRedirector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerRoutedAway: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerRoutedAway: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerRoutedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerRoutedTo: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerRoutedReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerRoutedReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreTouchHitTesting, ICoreTouchHitTesting_Vtbl, 0xb1d8a289_3acf_4124_9fa3_ea8aba353c21);
impl windows_core::RuntimeType for ICoreTouchHitTesting {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreTouchHitTesting_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TouchHitTesting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTouchHitTesting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindow, ICoreWindow_Vtbl, 0x79b9d5f2_879e_4b89_b798_79e47598030c);
impl core::ops::Deref for ICoreWindow {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreWindow, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreWindow {
    pub fn AutomationHostProvider(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomationHostProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Bounds(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlowDirection(&self) -> windows_core::Result<CoreWindowFlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlowDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFlowDirection)(windows_core::Interface::as_raw(this), value).ok() }
    }
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
    pub fn PointerCursor(&self) -> windows_core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPointerCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreCursor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Visible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Activate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Activate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> windows_core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsyncKeyState)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> windows_core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKeyState)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Activated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivated)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AutomationProviderRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomationProviderRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAutomationProviderRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAutomationProviderRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CharacterReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCharacterReceived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCharacterReceived)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn InputEnabled<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputEnabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputEnabled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyDown)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyUp)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExited)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn TouchHitTesting<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchHitTesting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTouchHitTesting(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTouchHitTesting)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SizeChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSizeChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSizeChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn VisibilityChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisibilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVisibilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVisibilityChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for ICoreWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutomationHostProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreWindowFlowDirection) -> windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(*mut core::ffi::c_void, CoreWindowFlowDirection) -> windows_core::HRESULT,
    pub IsInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PointerCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPointerCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PointerPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetAsyncKeyState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey, *mut CoreVirtualKeyStates) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetAsyncKeyState: usize,
    #[cfg(feature = "System")]
    pub GetKeyState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::VirtualKey, *mut CoreVirtualKeyStates) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetKeyState: usize,
    pub ReleasePointerCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPointerCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AutomationProviderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAutomationProviderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TouchHitTesting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTouchHitTesting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub VisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindow2, ICoreWindow2_Vtbl, 0x7c2b1b85_6917_4361_9c02_0d9e3a420b95);
impl windows_core::RuntimeType for ICoreWindow2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPointerPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindow3, ICoreWindow3_Vtbl, 0x32c20dd8_faef_4375_a2ab_32640e4815c7);
impl windows_core::RuntimeType for ICoreWindow3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindow3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClosestInteractiveBoundsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosestInteractiveBoundsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GetCurrentKeyEventDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindow4, ICoreWindow4_Vtbl, 0x35caf0d0_47f0_436c_af97_0dd88f6f5f02);
impl windows_core::RuntimeType for ICoreWindow4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindow4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResizeStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveResizeStarted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ResizeCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveResizeCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindow5, ICoreWindow5_Vtbl, 0x4b4ae1e1_2e6d_4eaa_bda1_1c5cc1bee141);
impl windows_core::RuntimeType for ICoreWindow5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindow5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub ActivationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreWindowActivationMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowDialog, ICoreWindowDialog_Vtbl, 0xe7392ce0_c78d_427e_8b2c_01ff420c69d5);
impl windows_core::RuntimeType for ICoreWindowDialog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowDialog_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Showing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveShowing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub MinSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsInteractionDelayed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIsInteractionDelayed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CancelCommandIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCancelCommandIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub BackButtonCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    BackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetBackButtonCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetBackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub ShowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowAsync: usize,
}
windows_core::imp::define_interface!(ICoreWindowDialogFactory, ICoreWindowDialogFactory_Vtbl, 0xcfb2a855_1c59_4b13_b1e5_16e29805f7c4);
impl windows_core::RuntimeType for ICoreWindowDialogFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowDialogFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowEventArgs, ICoreWindowEventArgs_Vtbl, 0x272b1ef3_c633_4da5_a26c_c6d0f56b29da);
impl core::ops::Deref for ICoreWindowEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreWindowEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreWindowEventArgs {
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
impl windows_core::RuntimeType for ICoreWindowEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowFlyout, ICoreWindowFlyout_Vtbl, 0xe89d854d_2050_40bb_b344_f6f355eeb314);
impl windows_core::RuntimeType for ICoreWindowFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowFlyout_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Showing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveShowing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub MinSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsInteractionDelayed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIsInteractionDelayed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub BackButtonCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    BackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetBackButtonCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetBackButtonCommand: usize,
    #[cfg(feature = "UI_Popups")]
    pub ShowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowAsync: usize,
}
windows_core::imp::define_interface!(ICoreWindowFlyoutFactory, ICoreWindowFlyoutFactory_Vtbl, 0xdec4c6c4_93e8_4f7c_be27_cefaa1af68a7);
impl windows_core::RuntimeType for ICoreWindowFlyoutFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowFlyoutFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithTitle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Point, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowPopupShowingEventArgs, ICoreWindowPopupShowingEventArgs_Vtbl, 0x26155fa2_5ba5_4ea4_a3b4_2dc7d63c8e26);
impl windows_core::RuntimeType for ICoreWindowPopupShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowPopupShowingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDesiredSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowResizeManager, ICoreWindowResizeManager_Vtbl, 0xb8f0b925_b350_48b3_a198_5c1a84700243);
impl windows_core::RuntimeType for ICoreWindowResizeManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowResizeManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotifyLayoutCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowResizeManagerLayoutCapability, ICoreWindowResizeManagerLayoutCapability_Vtbl, 0xbb74f27b_a544_4301_80e6_0ae033ef4536);
impl windows_core::RuntimeType for ICoreWindowResizeManagerLayoutCapability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowResizeManagerLayoutCapability_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetShouldWaitForLayoutCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShouldWaitForLayoutCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowResizeManagerStatics, ICoreWindowResizeManagerStatics_Vtbl, 0xae4a9045_6d70_49db_8e68_46ffbd17d38d);
impl windows_core::RuntimeType for ICoreWindowResizeManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowResizeManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowStatic, ICoreWindowStatic_Vtbl, 0x4d239005_3c2a_41b1_9022_536bb9cf93b1);
impl windows_core::RuntimeType for ICoreWindowStatic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowStatic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowWithContext, ICoreWindowWithContext_Vtbl, 0x9ac40241_3575_4c3b_af66_e8c529d4d06c);
impl windows_core::RuntimeType for ICoreWindowWithContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWindowWithContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIdleDispatchedHandlerArgs, IIdleDispatchedHandlerArgs_Vtbl, 0x98bb6a24_dc1c_43cb_b4ed_d1c0eb2391f3);
impl windows_core::RuntimeType for IIdleDispatchedHandlerArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIdleDispatchedHandlerArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDispatcherIdle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInitializeWithCoreWindow, IInitializeWithCoreWindow_Vtbl, 0x188f20d6_9873_464a_ace5_57e010f465e6);
impl core::ops::Deref for IInitializeWithCoreWindow {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInitializeWithCoreWindow, windows_core::IUnknown, windows_core::IInspectable);
impl IInitializeWithCoreWindow {
    pub fn Initialize<P0>(&self, window: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreWindow>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Initialize)(windows_core::Interface::as_raw(this), window.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IInitializeWithCoreWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInitializeWithCoreWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputEnabledEventArgs, IInputEnabledEventArgs_Vtbl, 0x80371d4f_2fd8_4c24_aa86_3163a87b4e5a);
impl windows_core::RuntimeType for IInputEnabledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputEnabledEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyEventArgs, IKeyEventArgs_Vtbl, 0x5ff5e930_2544_4a17_bd78_1f2fdebb106b);
impl windows_core::RuntimeType for IKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKey) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CorePhysicalKeyStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyEventArgs2, IKeyEventArgs2_Vtbl, 0x583add98_0790_4571_9b12_645ef9d79e42);
impl windows_core::RuntimeType for IKeyEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerEventArgs, IPointerEventArgs_Vtbl, 0x920d9cb1_a5fc_4a21_8c09_49dfe6ffe25f);
impl windows_core::RuntimeType for IPointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input")]
    pub CurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    CurrentPoint: usize,
    #[cfg(feature = "System")]
    pub KeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    KeyModifiers: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub GetIntermediatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))]
    GetIntermediatePoints: usize,
}
windows_core::imp::define_interface!(ISystemNavigationManager, ISystemNavigationManager_Vtbl, 0x93023118_cf50_42a6_9706_69107fa122e1);
impl windows_core::RuntimeType for ISystemNavigationManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemNavigationManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BackRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBackRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemNavigationManager2, ISystemNavigationManager2_Vtbl, 0x8c510401_67be_49ae_9509_671c1e54a389);
impl windows_core::RuntimeType for ISystemNavigationManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemNavigationManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppViewBackButtonVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppViewBackButtonVisibility) -> windows_core::HRESULT,
    pub SetAppViewBackButtonVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, AppViewBackButtonVisibility) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemNavigationManagerStatics, ISystemNavigationManagerStatics_Vtbl, 0xdc52b5ce_bee0_4305_8c54_68228ed683b5);
impl windows_core::RuntimeType for ISystemNavigationManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemNavigationManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITouchHitTestingEventArgs, ITouchHitTestingEventArgs_Vtbl, 0x22f3b823_0b7c_424e_9df7_33d4f962931b);
impl windows_core::RuntimeType for ITouchHitTestingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITouchHitTestingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProximityEvaluation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreProximityEvaluation) -> windows_core::HRESULT,
    pub SetProximityEvaluation: unsafe extern "system" fn(*mut core::ffi::c_void, CoreProximityEvaluation) -> windows_core::HRESULT,
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub BoundingBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub EvaluateProximityToRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, *mut CoreProximityEvaluation) -> windows_core::HRESULT,
    pub EvaluateProximityToPolygon: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::Point, *mut CoreProximityEvaluation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVisibilityChangedEventArgs, IVisibilityChangedEventArgs_Vtbl, 0xbf9918ea_d801_4564_a495_b1e84f8ad085);
impl windows_core::RuntimeType for IVisibilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVisibilityChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowActivatedEventArgs, IWindowActivatedEventArgs_Vtbl, 0x179d65e7_4658_4cb6_aa13_41d094ea255e);
impl windows_core::RuntimeType for IWindowActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowActivationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreWindowActivationState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowSizeChangedEventArgs, IWindowSizeChangedEventArgs_Vtbl, 0x5a200ec7_0426_47dc_b86c_6f475915e451);
impl windows_core::RuntimeType for IWindowSizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowSizeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AcceleratorKeyEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AcceleratorKeyEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AcceleratorKeyEventArgs, ICoreWindowEventArgs);
impl AcceleratorKeyEventArgs {
    pub fn EventType(&self) -> windows_core::Result<CoreAcceleratorKeyEventType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAcceleratorKeyEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AcceleratorKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAcceleratorKeyEventArgs>();
}
unsafe impl windows_core::Interface for AcceleratorKeyEventArgs {
    type Vtable = IAcceleratorKeyEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAcceleratorKeyEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AcceleratorKeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.AcceleratorKeyEventArgs";
}
unsafe impl Send for AcceleratorKeyEventArgs {}
unsafe impl Sync for AcceleratorKeyEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AutomationProviderRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutomationProviderRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AutomationProviderRequestedEventArgs, ICoreWindowEventArgs);
impl AutomationProviderRequestedEventArgs {
    pub fn AutomationProvider(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomationProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAutomationProvider<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutomationProvider)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AutomationProviderRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutomationProviderRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AutomationProviderRequestedEventArgs {
    type Vtable = IAutomationProviderRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAutomationProviderRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutomationProviderRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.AutomationProviderRequestedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BackRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BackRequestedEventArgs {
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
impl windows_core::RuntimeType for BackRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackRequestedEventArgs>();
}
unsafe impl windows_core::Interface for BackRequestedEventArgs {
    type Vtable = IBackRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBackRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.BackRequestedEventArgs";
}
unsafe impl Send for BackRequestedEventArgs {}
unsafe impl Sync for BackRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CharacterReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CharacterReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CharacterReceivedEventArgs, ICoreWindowEventArgs);
impl CharacterReceivedEventArgs {
    pub fn KeyCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CharacterReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICharacterReceivedEventArgs>();
}
unsafe impl windows_core::Interface for CharacterReceivedEventArgs {
    type Vtable = ICharacterReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICharacterReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CharacterReceivedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CharacterReceivedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ClosestInteractiveBoundsRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClosestInteractiveBoundsRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ClosestInteractiveBoundsRequestedEventArgs {
    pub fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SearchBounds(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchBounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ClosestInteractiveBounds(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosestInteractiveBounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetClosestInteractiveBounds(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClosestInteractiveBounds)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ClosestInteractiveBoundsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClosestInteractiveBoundsRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ClosestInteractiveBoundsRequestedEventArgs {
    type Vtable = IClosestInteractiveBoundsRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IClosestInteractiveBoundsRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClosestInteractiveBoundsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreAcceleratorKeys(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreAcceleratorKeys, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CoreAcceleratorKeys, ICoreAcceleratorKeys);
impl CoreAcceleratorKeys {
    pub fn AcceleratorKeyActivated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceleratorKeyActivated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAcceleratorKeyActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAcceleratorKeyActivated)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for CoreAcceleratorKeys {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreAcceleratorKeys>();
}
unsafe impl windows_core::Interface for CoreAcceleratorKeys {
    type Vtable = ICoreAcceleratorKeys_Vtbl;
    const IID: windows_core::GUID = <ICoreAcceleratorKeys as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreAcceleratorKeys {
    const NAME: &'static str = "Windows.UI.Core.CoreAcceleratorKeys";
}
unsafe impl Send for CoreAcceleratorKeys {}
unsafe impl Sync for CoreAcceleratorKeys {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreComponentInputSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreComponentInputSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CoreComponentInputSource, ICoreInputSourceBase, ICorePointerInputSource, ICorePointerInputSource2);
impl CoreComponentInputSource {
    pub fn ClosestInteractiveBoundsRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreComponentInputSource, ClosestInteractiveBoundsRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreClosestInteractiveBoundsRequested>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosestInteractiveBoundsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosestInteractiveBoundsRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreClosestInteractiveBoundsRequested>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosestInteractiveBoundsRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn HasFocus(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasFocus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GotFocus<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, CoreWindowEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGotFocus(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveGotFocus)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, CoreWindowEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLostFocus(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveLostFocus)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    pub fn InputEnabled<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, InputEnabledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputEnabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputEnabled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "System")]
    pub fn GetCurrentKeyState(&self, virtualkey: super::super::System::VirtualKey) -> windows_core::Result<CoreVirtualKeyStates> {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentKeyState)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        }
    }
    pub fn CharacterReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, CharacterReceivedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCharacterReceived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCharacterReceived)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, KeyEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyDown)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, KeyEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyUp)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetCurrentKeyEventDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICoreKeyboardInputSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentKeyEventDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReleasePointerCapture(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCapture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerCursor(&self) -> windows_core::Result<CoreCursor> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPointerCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreCursor>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExited)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TouchHitTesting<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, TouchHitTestingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreTouchHitTesting>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchHitTesting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTouchHitTesting(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreTouchHitTesting>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveTouchHitTesting)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for CoreComponentInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputSourceBase>();
}
unsafe impl windows_core::Interface for CoreComponentInputSource {
    type Vtable = ICoreInputSourceBase_Vtbl;
    const IID: windows_core::GUID = <ICoreInputSourceBase as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreComponentInputSource {
    const NAME: &'static str = "Windows.UI.Core.CoreComponentInputSource";
}
unsafe impl Send for CoreComponentInputSource {}
unsafe impl Sync for CoreComponentInputSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreCursor, windows_core::IUnknown, windows_core::IInspectable);
impl CoreCursor {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<CoreCursorType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateCursor(r#type: CoreCursorType, id: u32) -> windows_core::Result<CoreCursor> {
        Self::ICoreCursorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCursor)(windows_core::Interface::as_raw(this), r#type, id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreCursorFactory<R, F: FnOnce(&ICoreCursorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreCursor, ICoreCursorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreCursor>();
}
unsafe impl windows_core::Interface for CoreCursor {
    type Vtable = ICoreCursor_Vtbl;
    const IID: windows_core::GUID = <ICoreCursor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreCursor {
    const NAME: &'static str = "Windows.UI.Core.CoreCursor";
}
unsafe impl Send for CoreCursor {}
unsafe impl Sync for CoreCursor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreDispatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDispatcher, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CoreDispatcher, ICoreAcceleratorKeys);
impl CoreDispatcher {
    pub fn AcceleratorKeyActivated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreAcceleratorKeys>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceleratorKeyActivated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAcceleratorKeyActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreAcceleratorKeys>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAcceleratorKeyActivated)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn HasThreadAccess(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasThreadAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProcessEvents(&self, options: CoreProcessEventsOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessEvents)(windows_core::Interface::as_raw(this), options).ok() }
    }
    pub fn RunAsync<P0>(&self, priority: CoreDispatcherPriority, agilecallback: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<DispatchedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunAsync)(windows_core::Interface::as_raw(this), priority, agilecallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RunIdleAsync<P0>(&self, agilecallback: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<IdleDispatchedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunIdleAsync)(windows_core::Interface::as_raw(this), agilecallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryRunAsync<P0>(&self, priority: CoreDispatcherPriority, agilecallback: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<DispatchedHandler>,
    {
        let this = &windows_core::Interface::cast::<ICoreDispatcher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRunAsync)(windows_core::Interface::as_raw(this), priority, agilecallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryRunIdleAsync<P0>(&self, agilecallback: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<IdleDispatchedHandler>,
    {
        let this = &windows_core::Interface::cast::<ICoreDispatcher2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryRunIdleAsync)(windows_core::Interface::as_raw(this), agilecallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentPriority(&self) -> windows_core::Result<CoreDispatcherPriority> {
        let this = &windows_core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentPriority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCurrentPriority(&self, value: CoreDispatcherPriority) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCurrentPriority)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShouldYield(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldYield)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShouldYieldToPriority(&self, priority: CoreDispatcherPriority) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldYieldToPriority)(windows_core::Interface::as_raw(this), priority, &mut result__).map(|| result__)
        }
    }
    pub fn StopProcessEvents(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopProcessEvents)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for CoreDispatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDispatcher>();
}
unsafe impl windows_core::Interface for CoreDispatcher {
    type Vtable = ICoreDispatcher_Vtbl;
    const IID: windows_core::GUID = <ICoreDispatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDispatcher {
    const NAME: &'static str = "Windows.UI.Core.CoreDispatcher";
}
unsafe impl Send for CoreDispatcher {}
unsafe impl Sync for CoreDispatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreIndependentInputSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreIndependentInputSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CoreIndependentInputSource, ICoreInputSourceBase, ICorePointerInputSource, ICorePointerInputSource2, ICorePointerRedirector);
impl CoreIndependentInputSource {
    pub fn Dispatcher(&self) -> windows_core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    pub fn InputEnabled<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, InputEnabledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputEnabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputEnabled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ReleasePointerCapture(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn HasCapture(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCapture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PointerCursor(&self) -> windows_core::Result<CoreCursor> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPointerCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreCursor>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExited)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<ICorePointerInputSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PointerRoutedAway<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedAway)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedAway(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedAway)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerRoutedTo<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedTo)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedTo(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedTo)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerRoutedReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl windows_core::RuntimeType for CoreIndependentInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputSourceBase>();
}
unsafe impl windows_core::Interface for CoreIndependentInputSource {
    type Vtable = ICoreInputSourceBase_Vtbl;
    const IID: windows_core::GUID = <ICoreInputSourceBase as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Core.CoreIndependentInputSource";
}
unsafe impl Send for CoreIndependentInputSource {}
unsafe impl Sync for CoreIndependentInputSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreIndependentInputSourceController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreIndependentInputSourceController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CoreIndependentInputSourceController, super::super::Foundation::IClosable);
impl CoreIndependentInputSourceController {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsTransparentForUncontrolledInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTransparentForUncontrolledInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTransparentForUncontrolledInput(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsTransparentForUncontrolledInput)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPalmRejectionEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPalmRejectionEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPalmRejectionEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPalmRejectionEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Source(&self) -> windows_core::Result<CoreIndependentInputSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetControlledInput(&self, inputtypes: CoreInputDeviceTypes) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetControlledInput)(windows_core::Interface::as_raw(this), inputtypes).ok() }
    }
    pub fn SetControlledInputWithFilters(&self, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetControlledInputWithFilters)(windows_core::Interface::as_raw(this), inputtypes, required, excluded).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisual<P0>(visual: P0) -> windows_core::Result<CoreIndependentInputSourceController>
    where
        P0: windows_core::Param<super::Composition::Visual>,
    {
        Self::ICoreIndependentInputSourceControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForVisual)(windows_core::Interface::as_raw(this), visual.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForIVisualElement<P0>(visualelement: P0) -> windows_core::Result<CoreIndependentInputSourceController>
    where
        P0: windows_core::Param<super::Composition::IVisualElement>,
    {
        Self::ICoreIndependentInputSourceControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForIVisualElement)(windows_core::Interface::as_raw(this), visualelement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreIndependentInputSourceControllerStatics<R, F: FnOnce(&ICoreIndependentInputSourceControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreIndependentInputSourceController, ICoreIndependentInputSourceControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreIndependentInputSourceController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreIndependentInputSourceController>();
}
unsafe impl windows_core::Interface for CoreIndependentInputSourceController {
    type Vtable = ICoreIndependentInputSourceController_Vtbl;
    const IID: windows_core::GUID = <ICoreIndependentInputSourceController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreIndependentInputSourceController {
    const NAME: &'static str = "Windows.UI.Core.CoreIndependentInputSourceController";
}
unsafe impl Send for CoreIndependentInputSourceController {}
unsafe impl Sync for CoreIndependentInputSourceController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreWindow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWindow, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CoreWindow, ICorePointerRedirector, ICoreWindow);
impl CoreWindow {
    pub fn PointerRoutedAway<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedAway)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedAway(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedAway)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerRoutedTo<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedTo)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedTo(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedTo)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerRoutedReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerRoutedReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerRoutedReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerRoutedReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AutomationHostProvider(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomationHostProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Bounds(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlowDirection(&self) -> windows_core::Result<CoreWindowFlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlowDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFlowDirection)(windows_core::Interface::as_raw(this), value).ok() }
    }
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
    pub fn PointerCursor(&self) -> windows_core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPointerCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CoreCursor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Visible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Activate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Activate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> windows_core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsyncKeyState)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> windows_core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKeyState)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Activated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivated)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AutomationProviderRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomationProviderRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAutomationProviderRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAutomationProviderRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CharacterReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCharacterReceived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCharacterReceived)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn InputEnabled<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputEnabled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInputEnabled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInputEnabled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn KeyDown<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyDown(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyDown)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn KeyUp<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKeyUp(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKeyUp)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerCaptureLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerCaptureLost(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerCaptureLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerEntered<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntered(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntered)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerExited<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExited(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExited)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerReleased<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleased(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleased)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn TouchHitTesting<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchHitTesting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTouchHitTesting(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTouchHitTesting)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn PointerWheelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerWheelChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerWheelChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SizeChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SizeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSizeChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSizeChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn VisibilityChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisibilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVisibilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVisibilityChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SetPointerPosition(&self, value: super::super::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindow2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClosestInteractiveBoundsRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, ClosestInteractiveBoundsRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreWindow3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosestInteractiveBoundsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosestInteractiveBoundsRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindow3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosestInteractiveBoundsRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetCurrentKeyEventDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICoreWindow3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentKeyEventDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResizeStarted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResizeStarted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResizeStarted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveResizeStarted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ResizeCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResizeCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResizeCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveResizeCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<ICoreWindow5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivationMode(&self) -> windows_core::Result<CoreWindowActivationMode> {
        let this = &windows_core::Interface::cast::<ICoreWindow5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetForCurrentThread() -> windows_core::Result<CoreWindow> {
        Self::ICoreWindowStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentThread)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UIContext(&self) -> windows_core::Result<super::UIContext> {
        let this = &windows_core::Interface::cast::<ICoreWindowWithContext>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn ICoreWindowStatic<R, F: FnOnce(&ICoreWindowStatic) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreWindow, ICoreWindowStatic> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWindow>();
}
unsafe impl windows_core::Interface for CoreWindow {
    type Vtable = ICoreWindow_Vtbl;
    const IID: windows_core::GUID = <ICoreWindow as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWindow {
    const NAME: &'static str = "Windows.UI.Core.CoreWindow";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreWindowDialog(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWindowDialog, windows_core::IUnknown, windows_core::IInspectable);
impl CoreWindowDialog {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreWindowDialog, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Showing<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Showing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShowing(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveShowing)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn MaxSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsInteractionDelayed(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInteractionDelayed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsInteractionDelayed(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsInteractionDelayed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Commands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultCommandIndex(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultCommandIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultCommandIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CancelCommandIndex(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelCommandIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCancelCommandIndex(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCancelCommandIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn BackButtonCommand(&self) -> windows_core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackButtonCommand)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn SetBackButtonCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Popups::UICommandInvokedHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackButtonCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWithTitle(title: &windows_core::HSTRING) -> windows_core::Result<CoreWindowDialog> {
        Self::ICoreWindowDialogFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreWindowDialogFactory<R, F: FnOnce(&ICoreWindowDialogFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreWindowDialog, ICoreWindowDialogFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreWindowDialog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWindowDialog>();
}
unsafe impl windows_core::Interface for CoreWindowDialog {
    type Vtable = ICoreWindowDialog_Vtbl;
    const IID: windows_core::GUID = <ICoreWindowDialog as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWindowDialog {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowDialog";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreWindowEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWindowEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CoreWindowEventArgs, ICoreWindowEventArgs);
impl CoreWindowEventArgs {
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
impl windows_core::RuntimeType for CoreWindowEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWindowEventArgs>();
}
unsafe impl windows_core::Interface for CoreWindowEventArgs {
    type Vtable = ICoreWindowEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreWindowEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWindowEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreWindowFlyout(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWindowFlyout, windows_core::IUnknown, windows_core::IInspectable);
impl CoreWindowFlyout {
    pub fn Showing<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Showing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveShowing(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveShowing)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn MaxSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsInteractionDelayed(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInteractionDelayed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsInteractionDelayed(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsInteractionDelayed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Commands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultCommandIndex(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultCommandIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultCommandIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn BackButtonCommand(&self) -> windows_core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackButtonCommand)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn SetBackButtonCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Popups::UICommandInvokedHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackButtonCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(position: super::super::Foundation::Point) -> windows_core::Result<CoreWindowFlyout> {
        Self::ICoreWindowFlyoutFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithTitle(position: super::super::Foundation::Point, title: &windows_core::HSTRING) -> windows_core::Result<CoreWindowFlyout> {
        Self::ICoreWindowFlyoutFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTitle)(windows_core::Interface::as_raw(this), position, core::mem::transmute_copy(title), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreWindowFlyoutFactory<R, F: FnOnce(&ICoreWindowFlyoutFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreWindowFlyout, ICoreWindowFlyoutFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreWindowFlyout {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWindowFlyout>();
}
unsafe impl windows_core::Interface for CoreWindowFlyout {
    type Vtable = ICoreWindowFlyout_Vtbl;
    const IID: windows_core::GUID = <ICoreWindowFlyout as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWindowFlyout {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowFlyout";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreWindowPopupShowingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWindowPopupShowingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreWindowPopupShowingEventArgs {
    pub fn SetDesiredSize(&self, value: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CoreWindowPopupShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWindowPopupShowingEventArgs>();
}
unsafe impl windows_core::Interface for CoreWindowPopupShowingEventArgs {
    type Vtable = ICoreWindowPopupShowingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreWindowPopupShowingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWindowPopupShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowPopupShowingEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreWindowResizeManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWindowResizeManager, windows_core::IUnknown, windows_core::IInspectable);
impl CoreWindowResizeManager {
    pub fn NotifyLayoutCompleted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyLayoutCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetShouldWaitForLayoutCompletion(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowResizeManagerLayoutCapability>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetShouldWaitForLayoutCompletion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShouldWaitForLayoutCompletion(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowResizeManagerLayoutCapability>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldWaitForLayoutCompletion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetForCurrentView() -> windows_core::Result<CoreWindowResizeManager> {
        Self::ICoreWindowResizeManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreWindowResizeManagerStatics<R, F: FnOnce(&ICoreWindowResizeManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreWindowResizeManager, ICoreWindowResizeManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreWindowResizeManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWindowResizeManager>();
}
unsafe impl windows_core::Interface for CoreWindowResizeManager {
    type Vtable = ICoreWindowResizeManager_Vtbl;
    const IID: windows_core::GUID = <ICoreWindowResizeManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWindowResizeManager {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowResizeManager";
}
unsafe impl Send for CoreWindowResizeManager {}
unsafe impl Sync for CoreWindowResizeManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IdleDispatchedHandlerArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IdleDispatchedHandlerArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IdleDispatchedHandlerArgs {
    pub fn IsDispatcherIdle(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDispatcherIdle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IdleDispatchedHandlerArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIdleDispatchedHandlerArgs>();
}
unsafe impl windows_core::Interface for IdleDispatchedHandlerArgs {
    type Vtable = IIdleDispatchedHandlerArgs_Vtbl;
    const IID: windows_core::GUID = <IIdleDispatchedHandlerArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IdleDispatchedHandlerArgs {
    const NAME: &'static str = "Windows.UI.Core.IdleDispatchedHandlerArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InputEnabledEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputEnabledEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputEnabledEventArgs, ICoreWindowEventArgs);
impl InputEnabledEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InputEnabledEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputEnabledEventArgs>();
}
unsafe impl windows_core::Interface for InputEnabledEventArgs {
    type Vtable = IInputEnabledEventArgs_Vtbl;
    const IID: windows_core::GUID = <IInputEnabledEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputEnabledEventArgs {
    const NAME: &'static str = "Windows.UI.Core.InputEnabledEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KeyEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(KeyEventArgs, ICoreWindowEventArgs);
impl KeyEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IKeyEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for KeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyEventArgs>();
}
unsafe impl windows_core::Interface for KeyEventArgs {
    type Vtable = IKeyEventArgs_Vtbl;
    const IID: windows_core::GUID = <IKeyEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.KeyEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PointerEventArgs, ICoreWindowEventArgs);
impl PointerEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn CurrentPoint(&self) -> windows_core::Result<super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentPoint)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn KeyModifiers(&self) -> windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyModifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub fn GetIntermediatePoints(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Input::PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntermediatePoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerEventArgs>();
}
unsafe impl windows_core::Interface for PointerEventArgs {
    type Vtable = IPointerEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPointerEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Windows.UI.Core.PointerEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemNavigationManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemNavigationManager, windows_core::IUnknown, windows_core::IInspectable);
impl SystemNavigationManager {
    pub fn BackRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<BackRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBackRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBackRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AppViewBackButtonVisibility(&self) -> windows_core::Result<AppViewBackButtonVisibility> {
        let this = &windows_core::Interface::cast::<ISystemNavigationManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppViewBackButtonVisibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAppViewBackButtonVisibility(&self, value: AppViewBackButtonVisibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemNavigationManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAppViewBackButtonVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<SystemNavigationManager> {
        Self::ISystemNavigationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISystemNavigationManagerStatics<R, F: FnOnce(&ISystemNavigationManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemNavigationManager, ISystemNavigationManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemNavigationManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemNavigationManager>();
}
unsafe impl windows_core::Interface for SystemNavigationManager {
    type Vtable = ISystemNavigationManager_Vtbl;
    const IID: windows_core::GUID = <ISystemNavigationManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemNavigationManager {
    const NAME: &'static str = "Windows.UI.Core.SystemNavigationManager";
}
unsafe impl Send for SystemNavigationManager {}
unsafe impl Sync for SystemNavigationManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TouchHitTestingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TouchHitTestingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TouchHitTestingEventArgs, ICoreWindowEventArgs);
impl TouchHitTestingEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProximityEvaluation(&self) -> windows_core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProximityEvaluation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProximityEvaluation(&self, value: CoreProximityEvaluation) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProximityEvaluation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Point(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Point)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingBox(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingBox)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EvaluateProximityToRect(&self, controlboundingbox: super::super::Foundation::Rect) -> windows_core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateProximityToRect)(windows_core::Interface::as_raw(this), controlboundingbox, &mut result__).map(|| result__)
        }
    }
    pub fn EvaluateProximityToPolygon(&self, controlvertices: &[super::super::Foundation::Point]) -> windows_core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateProximityToPolygon)(windows_core::Interface::as_raw(this), controlvertices.len().try_into().unwrap(), controlvertices.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TouchHitTestingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITouchHitTestingEventArgs>();
}
unsafe impl windows_core::Interface for TouchHitTestingEventArgs {
    type Vtable = ITouchHitTestingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITouchHitTestingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TouchHitTestingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.TouchHitTestingEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VisibilityChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VisibilityChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VisibilityChangedEventArgs, ICoreWindowEventArgs);
impl VisibilityChangedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VisibilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVisibilityChangedEventArgs>();
}
unsafe impl windows_core::Interface for VisibilityChangedEventArgs {
    type Vtable = IVisibilityChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IVisibilityChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.VisibilityChangedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WindowActivatedEventArgs, ICoreWindowEventArgs);
impl WindowActivatedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WindowActivationState(&self) -> windows_core::Result<CoreWindowActivationState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowActivationState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowActivatedEventArgs>();
}
unsafe impl windows_core::Interface for WindowActivatedEventArgs {
    type Vtable = IWindowActivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.WindowActivatedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowSizeChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowSizeChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WindowSizeChangedEventArgs, ICoreWindowEventArgs);
impl WindowSizeChangedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Size(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowSizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowSizeChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowSizeChangedEventArgs {
    type Vtable = IWindowSizeChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowSizeChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.WindowSizeChangedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppViewBackButtonVisibility(pub i32);
impl AppViewBackButtonVisibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl windows_core::TypeKind for AppViewBackButtonVisibility {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppViewBackButtonVisibility {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppViewBackButtonVisibility").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppViewBackButtonVisibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AppViewBackButtonVisibility;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreAcceleratorKeyEventType(pub i32);
impl CoreAcceleratorKeyEventType {
    pub const Character: Self = Self(2i32);
    pub const DeadCharacter: Self = Self(3i32);
    pub const KeyDown: Self = Self(0i32);
    pub const KeyUp: Self = Self(1i32);
    pub const SystemCharacter: Self = Self(6i32);
    pub const SystemDeadCharacter: Self = Self(7i32);
    pub const SystemKeyDown: Self = Self(4i32);
    pub const SystemKeyUp: Self = Self(5i32);
    pub const UnicodeCharacter: Self = Self(8i32);
}
impl windows_core::TypeKind for CoreAcceleratorKeyEventType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreAcceleratorKeyEventType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreAcceleratorKeyEventType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreAcceleratorKeyEventType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreAcceleratorKeyEventType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreCursorType(pub i32);
impl CoreCursorType {
    pub const Arrow: Self = Self(0i32);
    pub const Cross: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Hand: Self = Self(3i32);
    pub const Help: Self = Self(4i32);
    pub const IBeam: Self = Self(5i32);
    pub const SizeAll: Self = Self(6i32);
    pub const SizeNortheastSouthwest: Self = Self(7i32);
    pub const SizeNorthSouth: Self = Self(8i32);
    pub const SizeNorthwestSoutheast: Self = Self(9i32);
    pub const SizeWestEast: Self = Self(10i32);
    pub const UniversalNo: Self = Self(11i32);
    pub const UpArrow: Self = Self(12i32);
    pub const Wait: Self = Self(13i32);
    pub const Pin: Self = Self(14i32);
    pub const Person: Self = Self(15i32);
}
impl windows_core::TypeKind for CoreCursorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreCursorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreCursorType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreCursorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreCursorType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreDispatcherPriority(pub i32);
impl CoreDispatcherPriority {
    pub const Idle: Self = Self(-2i32);
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for CoreDispatcherPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreDispatcherPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreDispatcherPriority").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreDispatcherPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreDispatcherPriority;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreIndependentInputFilters(pub u32);
impl CoreIndependentInputFilters {
    pub const None: Self = Self(0u32);
    pub const MouseButton: Self = Self(1u32);
    pub const MouseWheel: Self = Self(2u32);
    pub const MouseHover: Self = Self(4u32);
    pub const PenWithBarrel: Self = Self(8u32);
    pub const PenInverted: Self = Self(16u32);
}
impl windows_core::TypeKind for CoreIndependentInputFilters {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreIndependentInputFilters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreIndependentInputFilters").field(&self.0).finish()
    }
}
impl CoreIndependentInputFilters {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CoreIndependentInputFilters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CoreIndependentInputFilters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CoreIndependentInputFilters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CoreIndependentInputFilters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CoreIndependentInputFilters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for CoreIndependentInputFilters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreIndependentInputFilters;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreInputDeviceTypes(pub u32);
impl CoreInputDeviceTypes {
    pub const None: Self = Self(0u32);
    pub const Touch: Self = Self(1u32);
    pub const Pen: Self = Self(2u32);
    pub const Mouse: Self = Self(4u32);
}
impl windows_core::TypeKind for CoreInputDeviceTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreInputDeviceTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreInputDeviceTypes").field(&self.0).finish()
    }
}
impl CoreInputDeviceTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CoreInputDeviceTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CoreInputDeviceTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CoreInputDeviceTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CoreInputDeviceTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CoreInputDeviceTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for CoreInputDeviceTypes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreInputDeviceTypes;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreProcessEventsOption(pub i32);
impl CoreProcessEventsOption {
    pub const ProcessOneAndAllPending: Self = Self(0i32);
    pub const ProcessOneIfPresent: Self = Self(1i32);
    pub const ProcessUntilQuit: Self = Self(2i32);
    pub const ProcessAllIfPresent: Self = Self(3i32);
}
impl windows_core::TypeKind for CoreProcessEventsOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreProcessEventsOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreProcessEventsOption").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreProcessEventsOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreProcessEventsOption;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreProximityEvaluationScore(pub i32);
impl CoreProximityEvaluationScore {
    pub const Closest: Self = Self(0i32);
    pub const Farthest: Self = Self(2147483647i32);
}
impl windows_core::TypeKind for CoreProximityEvaluationScore {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreProximityEvaluationScore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreProximityEvaluationScore").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreProximityEvaluationScore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreProximityEvaluationScore;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreVirtualKeyStates(pub u32);
impl CoreVirtualKeyStates {
    pub const None: Self = Self(0u32);
    pub const Down: Self = Self(1u32);
    pub const Locked: Self = Self(2u32);
}
impl windows_core::TypeKind for CoreVirtualKeyStates {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreVirtualKeyStates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreVirtualKeyStates").field(&self.0).finish()
    }
}
impl CoreVirtualKeyStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CoreVirtualKeyStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CoreVirtualKeyStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CoreVirtualKeyStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CoreVirtualKeyStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CoreVirtualKeyStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for CoreVirtualKeyStates {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreVirtualKeyStates;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreWindowActivationMode(pub i32);
impl CoreWindowActivationMode {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl windows_core::TypeKind for CoreWindowActivationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreWindowActivationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreWindowActivationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreWindowActivationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowActivationMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreWindowActivationState(pub i32);
impl CoreWindowActivationState {
    pub const CodeActivated: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const PointerActivated: Self = Self(2i32);
}
impl windows_core::TypeKind for CoreWindowActivationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreWindowActivationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreWindowActivationState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreWindowActivationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowActivationState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreWindowFlowDirection(pub i32);
impl CoreWindowFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl windows_core::TypeKind for CoreWindowFlowDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreWindowFlowDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreWindowFlowDirection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreWindowFlowDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowFlowDirection;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CorePhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: bool,
    pub IsMenuKeyDown: bool,
    pub WasKeyDown: bool,
    pub IsKeyReleased: bool,
}
impl windows_core::TypeKind for CorePhysicalKeyStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CorePhysicalKeyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Core.CorePhysicalKeyStatus;u4;u4;b1;b1;b1;b1)");
}
impl Default for CorePhysicalKeyStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoreProximityEvaluation {
    pub Score: i32,
    pub AdjustedPoint: super::super::Foundation::Point,
}
impl windows_core::TypeKind for CoreProximityEvaluation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CoreProximityEvaluation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Core.CoreProximityEvaluation;i4;struct(Windows.Foundation.Point;f4;f4))");
}
impl Default for CoreProximityEvaluation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(DispatchedHandler, DispatchedHandler_Vtbl, 0xd1f276c4_98d8_4636_bf49_eb79507548e9);
impl DispatchedHandler {
    pub fn new<F: FnMut() -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = DispatchedHandlerBox::<F> { vtable: &DispatchedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct DispatchedHandlerBox<F: FnMut() -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DispatchedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut() -> windows_core::Result<()> + Send + 'static> DispatchedHandlerBox<F> {
    const VTABLE: DispatchedHandler_Vtbl = DispatchedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DispatchedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)().into()
    }
}
impl windows_core::RuntimeType for DispatchedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DispatchedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IdleDispatchedHandler, IdleDispatchedHandler_Vtbl, 0xa42b0c24_7f21_4abc_99c1_8f01007f0880);
impl IdleDispatchedHandler {
    pub fn new<F: FnMut(Option<&IdleDispatchedHandlerArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = IdleDispatchedHandlerBox::<F> { vtable: &IdleDispatchedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IdleDispatchedHandlerArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
}
#[repr(C)]
struct IdleDispatchedHandlerBox<F: FnMut(Option<&IdleDispatchedHandlerArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const IdleDispatchedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&IdleDispatchedHandlerArgs>) -> windows_core::Result<()> + Send + 'static> IdleDispatchedHandlerBox<F> {
    const VTABLE: IdleDispatchedHandler_Vtbl = IdleDispatchedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <IdleDispatchedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&e)).into()
    }
}
impl windows_core::RuntimeType for IdleDispatchedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IdleDispatchedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
