#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionCatalog(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionCatalog, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionCatalog, super::super::super::Foundation::IClosable);
impl ActionCatalog {
    pub fn GetAllActions(&self) -> windows_core::Result<windows_core::Array<ActionDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetAllActions)(windows_core::Interface::as_raw(this), windows_core::Array::<ActionDefinition>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<ActionCatalog, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetActionsForInputs(&self, inputentities: &[Option<super::ActionEntity>]) -> windows_core::Result<windows_core::Array<ActionInstance>> {
        let this = &windows_core::Interface::cast::<IActionCatalog2>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetActionsForInputs)(windows_core::Interface::as_raw(this), inputentities.len().try_into().unwrap(), core::mem::transmute(inputentities.as_ptr()), windows_core::Array::<ActionInstance>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "UI")]
    pub fn GetActionsForInputs2(&self, inputentities: &[Option<super::ActionEntity>], invokerwindowid: super::super::super::UI::WindowId) -> windows_core::Result<windows_core::Array<ActionInstance>> {
        let this = &windows_core::Interface::cast::<IActionCatalog2>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetActionsForInputs2)(windows_core::Interface::as_raw(this), inputentities.len().try_into().unwrap(), core::mem::transmute(inputentities.as_ptr()), invokerwindowid, windows_core::Array::<ActionInstance>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetActionsForCurrentApp(&self) -> windows_core::Result<windows_core::Array<ActionDefinition>> {
        let this = &windows_core::Interface::cast::<IActionCatalog3>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetActionsForCurrentApp)(windows_core::Interface::as_raw(this), windows_core::Array::<ActionDefinition>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionCatalog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionCatalog>();
}
unsafe impl windows_core::Interface for ActionCatalog {
    type Vtable = <IActionCatalog as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionCatalog as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionCatalog {
    const NAME: &'static str = "Windows.AI.Actions.Hosting.ActionCatalog";
}
unsafe impl Send for ActionCatalog {}
unsafe impl Sync for ActionCatalog {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionDefinition, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionDefinition, super::super::super::Foundation::IClosable);
impl ActionDefinition {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IconFullPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconFullPath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetInputs(&self) -> windows_core::Result<windows_core::Array<ActionEntityRegistrationInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetInputs)(windows_core::Interface::as_raw(this), windows_core::Array::<ActionEntityRegistrationInfo>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetOutputs(&self) -> windows_core::Result<windows_core::Array<ActionEntityRegistrationInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetOutputs)(windows_core::Interface::as_raw(this), windows_core::Array::<ActionEntityRegistrationInfo>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetOverloads(&self) -> windows_core::Result<windows_core::Array<ActionOverload>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetOverloads)(windows_core::Interface::as_raw(this), windows_core::Array::<ActionOverload>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn DisplaysUI(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IActionDefinition2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplaysUI)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsesGenerativeAI(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IActionDefinition2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsesGenerativeAI)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SchemaVersion(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IActionDefinition2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SchemaVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PackageRelativeApplicationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionDefinition3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageRelativeApplicationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsCurrentlyAvailable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IActionDefinition4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCurrentlyAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionDefinition>();
}
unsafe impl windows_core::Interface for ActionDefinition {
    type Vtable = <IActionDefinition as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionDefinition {
    const NAME: &'static str = "Windows.AI.Actions.Hosting.ActionDefinition";
}
unsafe impl Send for ActionDefinition {}
unsafe impl Sync for ActionDefinition {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEntityRegistrationInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionEntityRegistrationInfo, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionEntityRegistrationInfo, super::super::super::Foundation::IClosable);
impl ActionEntityRegistrationInfo {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<super::ActionEntityKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: super::ActionEntityKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionEntityRegistrationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionEntityRegistrationInfo>();
}
unsafe impl windows_core::Interface for ActionEntityRegistrationInfo {
    type Vtable = <IActionEntityRegistrationInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionEntityRegistrationInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionEntityRegistrationInfo {
    const NAME: &'static str = "Windows.AI.Actions.Hosting.ActionEntityRegistrationInfo";
}
unsafe impl Send for ActionEntityRegistrationInfo {}
unsafe impl Sync for ActionEntityRegistrationInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionInstance(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionInstance, windows_core::IUnknown, windows_core::IInspectable);
impl ActionInstance {
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionInstanceDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Definition(&self) -> windows_core::Result<ActionDefinition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Definition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Context(&self) -> windows_core::Result<super::ActionInvocationContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InvokeAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ActionInstance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionInstance>();
}
unsafe impl windows_core::Interface for ActionInstance {
    type Vtable = <IActionInstance as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionInstance as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionInstance {
    const NAME: &'static str = "Windows.AI.Actions.Hosting.ActionInstance";
}
unsafe impl Send for ActionInstance {}
unsafe impl Sync for ActionInstance {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionInstanceDisplayInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionInstanceDisplayInfo, windows_core::IUnknown, windows_core::IInspectable);
impl ActionInstanceDisplayInfo {
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for ActionInstanceDisplayInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionInstanceDisplayInfo>();
}
unsafe impl windows_core::Interface for ActionInstanceDisplayInfo {
    type Vtable = <IActionInstanceDisplayInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionInstanceDisplayInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionInstanceDisplayInfo {
    const NAME: &'static str = "Windows.AI.Actions.Hosting.ActionInstanceDisplayInfo";
}
unsafe impl Send for ActionInstanceDisplayInfo {}
unsafe impl Sync for ActionInstanceDisplayInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionOverload(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionOverload, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionOverload, super::super::super::Foundation::IClosable);
impl ActionOverload {
    pub fn DescriptionTemplate(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DescriptionTemplate)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetInputs(&self) -> windows_core::Result<windows_core::Array<ActionEntityRegistrationInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetInputs)(windows_core::Interface::as_raw(this), windows_core::Array::<ActionEntityRegistrationInfo>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn InvokeAsync<P0>(&self, context: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::ActionInvocationContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeAsync)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InvokeFeedbackAsync<P0, P1>(&self, context: P0, feedback: P1) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::ActionInvocationContext>,
        P1: windows_core::Param<super::ActionFeedback>,
    {
        let this = &windows_core::Interface::cast::<IActionOverload2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeFeedbackAsync)(windows_core::Interface::as_raw(this), context.param().abi(), feedback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSupportsFeedback(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IActionOverload2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportsFeedback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionOverload {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionOverload>();
}
unsafe impl windows_core::Interface for ActionOverload {
    type Vtable = <IActionOverload as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionOverload as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionOverload {
    const NAME: &'static str = "Windows.AI.Actions.Hosting.ActionOverload";
}
unsafe impl Send for ActionOverload {}
unsafe impl Sync for ActionOverload {}
windows_core::imp::define_interface!(IActionCatalog, IActionCatalog_Vtbl, 0xdbe7c537_66ea_5394_9085_4fc19d78375c);
impl windows_core::RuntimeType for IActionCatalog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionCatalog_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAllActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionCatalog2, IActionCatalog2_Vtbl, 0x370360b1_a14b_5ea8_b611_b5f70342ba44);
impl windows_core::RuntimeType for IActionCatalog2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionCatalog2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetActionsForInputs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::ActionEntity, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub GetActionsForInputs2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::ActionEntity, super::super::super::UI::WindowId, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetActionsForInputs2: usize,
}
windows_core::imp::define_interface!(IActionCatalog3, IActionCatalog3_Vtbl, 0x2e05d518_8680_55d3_820d_2605adb7d62d);
impl windows_core::RuntimeType for IActionCatalog3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionCatalog3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetActionsForCurrentApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionDefinition, IActionDefinition_Vtbl, 0xfe766add_924d_5231_855e_dac9e82c7e6c);
impl windows_core::RuntimeType for IActionDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconFullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOverloads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionDefinition2, IActionDefinition2_Vtbl, 0xc1f44733_f563_54e2_bd2b_dc4c732054cf);
impl windows_core::RuntimeType for IActionDefinition2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionDefinition2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplaysUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub UsesGenerativeAI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SchemaVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionDefinition3, IActionDefinition3_Vtbl, 0x89c9a7e0_4bfd_55f4_9eed_dce2250114fa);
impl windows_core::RuntimeType for IActionDefinition3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionDefinition3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageRelativeApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionDefinition4, IActionDefinition4_Vtbl, 0x6dd91071_8847_55b6_9518_9ff8de421eb7);
impl windows_core::RuntimeType for IActionDefinition4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionDefinition4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCurrentlyAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntityRegistrationInfo, IActionEntityRegistrationInfo_Vtbl, 0xc3b92bdb_03c3_5a9e_b049_002fa0405699);
impl windows_core::RuntimeType for IActionEntityRegistrationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityRegistrationInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ActionEntityKind) -> windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, super::ActionEntityKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionInstance, IActionInstance_Vtbl, 0x809bcb6e_e6ef_5f16_b89a_06b8893df20e);
impl windows_core::RuntimeType for IActionInstance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionInstance_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Definition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionInstanceDisplayInfo, IActionInstanceDisplayInfo_Vtbl, 0xfcfdce21_678b_5602_b9dc_2f4533a6f4b2);
impl windows_core::RuntimeType for IActionInstanceDisplayInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionInstanceDisplayInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionOverload, IActionOverload_Vtbl, 0x5d184610_d09d_5375_9849_505c359dca01);
impl windows_core::RuntimeType for IActionOverload {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionOverload_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DescriptionTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionOverload2, IActionOverload2_Vtbl, 0x57ec9906_8231_5a9e_929f_bf39e952eb93);
impl windows_core::RuntimeType for IActionOverload2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionOverload2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvokeFeedbackAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportsFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
