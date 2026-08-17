#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
windows_core::imp::define_interface!(IAutomationConnection, IAutomationConnection_Vtbl, 0xaad262ed_0ef4_5d43_97be_a834e27b65b9);
impl windows_core::RuntimeType for IAutomationConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAutomationConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRemoteSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAutomationConnectionBoundObject, IAutomationConnectionBoundObject_Vtbl, 0x5e8558fb_ca52_5b65_9830_dd2905816093);
impl windows_core::RuntimeType for IAutomationConnectionBoundObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAutomationConnectionBoundObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAutomationElement, IAutomationElement_Vtbl, 0xa1898370_2c07_56fd_993f_61a72a08058c);
impl windows_core::RuntimeType for IAutomationElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAutomationElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRemoteSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAutomationTextRange, IAutomationTextRange_Vtbl, 0x7e101b65_40d3_5994_85a9_0a0cb9a4ec98);
impl windows_core::RuntimeType for IAutomationTextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAutomationTextRange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AutomationConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutomationConnection, windows_core::IUnknown, windows_core::IInspectable);
impl AutomationConnection {
    pub fn IsRemoteSystem(&self) -> bool {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).IsRemoteSystem)(windows_core::Interface::as_raw(this), &mut result__);
            debug_assert!(hresult__.0 == 0);
            result__
        }
    }
    pub fn AppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppUserModelId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExecutableFileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecutableFileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AutomationConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutomationConnection>();
}
unsafe impl windows_core::Interface for AutomationConnection {
    type Vtable = IAutomationConnection_Vtbl;
    const IID: windows_core::GUID = <IAutomationConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutomationConnection {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnection";
}
unsafe impl Send for AutomationConnection {}
unsafe impl Sync for AutomationConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AutomationConnectionBoundObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutomationConnectionBoundObject, windows_core::IUnknown, windows_core::IInspectable);
impl AutomationConnectionBoundObject {
    pub fn Connection(&self) -> windows_core::Result<AutomationConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AutomationConnectionBoundObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutomationConnectionBoundObject>();
}
unsafe impl windows_core::Interface for AutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObject_Vtbl;
    const IID: windows_core::GUID = <IAutomationConnectionBoundObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutomationConnectionBoundObject {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnectionBoundObject";
}
unsafe impl Send for AutomationConnectionBoundObject {}
unsafe impl Sync for AutomationConnectionBoundObject {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AutomationElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutomationElement, windows_core::IUnknown, windows_core::IInspectable);
impl AutomationElement {
    pub fn IsRemoteSystem(&self) -> bool {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).IsRemoteSystem)(windows_core::Interface::as_raw(this), &mut result__);
            debug_assert!(hresult__.0 == 0);
            result__
        }
    }
    pub fn AppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppUserModelId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExecutableFileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecutableFileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AutomationElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutomationElement>();
}
unsafe impl windows_core::Interface for AutomationElement {
    type Vtable = IAutomationElement_Vtbl;
    const IID: windows_core::GUID = <IAutomationElement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutomationElement {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationElement";
}
unsafe impl Send for AutomationElement {}
unsafe impl Sync for AutomationElement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AutomationTextRange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutomationTextRange, windows_core::IUnknown, windows_core::IInspectable);
impl AutomationTextRange {}
impl windows_core::RuntimeType for AutomationTextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutomationTextRange>();
}
unsafe impl windows_core::Interface for AutomationTextRange {
    type Vtable = IAutomationTextRange_Vtbl;
    const IID: windows_core::GUID = <IAutomationTextRange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutomationTextRange {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationTextRange";
}
unsafe impl Send for AutomationTextRange {}
unsafe impl Sync for AutomationTextRange {}
