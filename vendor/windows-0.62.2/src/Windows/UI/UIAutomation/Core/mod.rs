#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AutomationAnnotationTypeRegistration {
    pub LocalId: i32,
}
impl windows_core::TypeKind for AutomationAnnotationTypeRegistration {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AutomationAnnotationTypeRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationAnnotationTypeRegistration;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AutomationRemoteOperationOperandId {
    pub Value: i32,
}
impl windows_core::TypeKind for AutomationRemoteOperationOperandId {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AutomationRemoteOperationOperandId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.UIAutomation.Core.AutomationRemoteOperationOperandId;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomationRemoteOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutomationRemoteOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl AutomationRemoteOperationResult {
    pub fn Status(&self) -> windows_core::Result<AutomationRemoteOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorLocation(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasOperand(&self, operandid: AutomationRemoteOperationOperandId) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasOperand)(windows_core::Interface::as_raw(this), operandid, &mut result__).map(|| result__)
        }
    }
    pub fn GetOperand(&self, operandid: AutomationRemoteOperationOperandId) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOperand)(windows_core::Interface::as_raw(this), operandid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AutomationRemoteOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutomationRemoteOperationResult>();
}
unsafe impl windows_core::Interface for AutomationRemoteOperationResult {
    type Vtable = <IAutomationRemoteOperationResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAutomationRemoteOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutomationRemoteOperationResult {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.AutomationRemoteOperationResult";
}
unsafe impl Send for AutomationRemoteOperationResult {}
unsafe impl Sync for AutomationRemoteOperationResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutomationRemoteOperationStatus(pub i32);
impl AutomationRemoteOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const MalformedBytecode: Self = Self(1i32);
    pub const InstructionLimitExceeded: Self = Self(2i32);
    pub const UnhandledException: Self = Self(3i32);
    pub const ExecutionFailure: Self = Self(4i32);
}
impl windows_core::TypeKind for AutomationRemoteOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AutomationRemoteOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.UIAutomation.Core.AutomationRemoteOperationStatus;i4)");
}
pub struct CoreAutomationRegistrar;
impl CoreAutomationRegistrar {
    pub fn RegisterAnnotationType(guid: windows_core::GUID) -> windows_core::Result<AutomationAnnotationTypeRegistration> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterAnnotationType)(windows_core::Interface::as_raw(this), guid, &mut result__).map(|| result__)
        })
    }
    pub fn UnregisterAnnotationType(registration: AutomationAnnotationTypeRegistration) -> windows_core::Result<()> {
        Self::ICoreAutomationRegistrarStatics(|this| unsafe { (windows_core::Interface::vtable(this).UnregisterAnnotationType)(windows_core::Interface::as_raw(this), registration).ok() })
    }
    fn ICoreAutomationRegistrarStatics<R, F: FnOnce(&ICoreAutomationRegistrarStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreAutomationRegistrar, ICoreAutomationRegistrarStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CoreAutomationRegistrar {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRegistrar";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreAutomationRemoteOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreAutomationRemoteOperation, windows_core::IUnknown, windows_core::IInspectable);
impl CoreAutomationRemoteOperation {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreAutomationRemoteOperation, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsOpcodeSupported(&self, opcode: u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOpcodeSupported)(windows_core::Interface::as_raw(this), opcode, &mut result__).map(|| result__)
        }
    }
    pub fn ImportElement<P1>(&self, operandid: AutomationRemoteOperationOperandId, element: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AutomationElement>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ImportElement)(windows_core::Interface::as_raw(this), operandid, element.param().abi()).ok() }
    }
    pub fn ImportTextRange<P1>(&self, operandid: AutomationRemoteOperationOperandId, textrange: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AutomationTextRange>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ImportTextRange)(windows_core::Interface::as_raw(this), operandid, textrange.param().abi()).ok() }
    }
    pub fn AddToResults(&self, operandid: AutomationRemoteOperationOperandId) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddToResults)(windows_core::Interface::as_raw(this), operandid).ok() }
    }
    pub fn Execute(&self, bytecodebuffer: &[u8]) -> windows_core::Result<AutomationRemoteOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Execute)(windows_core::Interface::as_raw(this), bytecodebuffer.len().try_into().unwrap(), bytecodebuffer.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImportConnectionBoundObject<P1>(&self, operandid: AutomationRemoteOperationOperandId, connectionboundobject: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AutomationConnectionBoundObject>,
    {
        let this = &windows_core::Interface::cast::<ICoreAutomationRemoteOperation2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ImportConnectionBoundObject)(windows_core::Interface::as_raw(this), operandid, connectionboundobject.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for CoreAutomationRemoteOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreAutomationRemoteOperation>();
}
unsafe impl windows_core::Interface for CoreAutomationRemoteOperation {
    type Vtable = <ICoreAutomationRemoteOperation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreAutomationRemoteOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreAutomationRemoteOperation {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperation";
}
unsafe impl Send for CoreAutomationRemoteOperation {}
unsafe impl Sync for CoreAutomationRemoteOperation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreAutomationRemoteOperationContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreAutomationRemoteOperationContext, windows_core::IUnknown, windows_core::IInspectable);
impl CoreAutomationRemoteOperationContext {
    pub fn GetOperand(&self, id: AutomationRemoteOperationOperandId) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOperand)(windows_core::Interface::as_raw(this), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOperand<P1>(&self, id: AutomationRemoteOperationOperandId, operand: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOperand)(windows_core::Interface::as_raw(this), id, operand.param().abi()).ok() }
    }
    pub fn SetOperand2<P1>(&self, id: AutomationRemoteOperationOperandId, operand: P1, operandinterfaceid: windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOperand2)(windows_core::Interface::as_raw(this), id, operand.param().abi(), operandinterfaceid).ok() }
    }
}
impl windows_core::RuntimeType for CoreAutomationRemoteOperationContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreAutomationRemoteOperationContext>();
}
unsafe impl windows_core::Interface for CoreAutomationRemoteOperationContext {
    type Vtable = <ICoreAutomationRemoteOperationContext as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreAutomationRemoteOperationContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreAutomationRemoteOperationContext {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.CoreAutomationRemoteOperationContext";
}
unsafe impl Send for CoreAutomationRemoteOperationContext {}
unsafe impl Sync for CoreAutomationRemoteOperationContext {}
windows_core::imp::define_interface!(IAutomationRemoteOperationResult, IAutomationRemoteOperationResult_Vtbl, 0xe0f80c42_4a67_5534_bf5a_09e8a99b36b1);
impl windows_core::RuntimeType for IAutomationRemoteOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationRemoteOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomationRemoteOperationStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub ErrorLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub HasOperand: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut bool) -> windows_core::HRESULT,
    pub GetOperand: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreAutomationConnectionBoundObjectProvider, ICoreAutomationConnectionBoundObjectProvider_Vtbl, 0x0620bb64_9616_5593_be3a_eb8e6daeb3fa);
impl windows_core::RuntimeType for ICoreAutomationConnectionBoundObjectProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ICoreAutomationConnectionBoundObjectProvider, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreAutomationConnectionBoundObjectProvider {
    pub fn IsComThreadingRequired(&self) -> bool {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).IsComThreadingRequired)(windows_core::Interface::as_raw(this), &mut result__);
            debug_assert!(hresult__.0 == 0);
            result__
        }
    }
}
impl windows_core::RuntimeName for ICoreAutomationConnectionBoundObjectProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
}
pub trait ICoreAutomationConnectionBoundObjectProvider_Impl: windows_core::IUnknownImpl {
    fn IsComThreadingRequired(&self) -> bool;
}
impl ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub const fn new<Identity: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsComThreadingRequired<Identity: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = ICoreAutomationConnectionBoundObjectProvider_Impl::IsComThreadingRequired(this);
                result__.write(core::mem::transmute_copy(&ok__));
                windows_core::HRESULT(0)
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreAutomationConnectionBoundObjectProvider, OFFSET>(),
            IsComThreadingRequired: IsComThreadingRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreAutomationConnectionBoundObjectProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsComThreadingRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreAutomationRegistrarStatics, ICoreAutomationRegistrarStatics_Vtbl, 0x3e50129b_d6dc_5680_b580_ffff78300304);
impl windows_core::RuntimeType for ICoreAutomationRegistrarStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRegistrarStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RegisterAnnotationType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut AutomationAnnotationTypeRegistration) -> windows_core::HRESULT,
    pub UnregisterAnnotationType: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationAnnotationTypeRegistration) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreAutomationRemoteOperation, ICoreAutomationRemoteOperation_Vtbl, 0x3ac656f4_e2bc_5c6e_b8e7_b224fb74b060);
impl windows_core::RuntimeType for ICoreAutomationRemoteOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOpcodeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
    pub ImportElement: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportTextRange: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToResults: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId) -> windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreAutomationRemoteOperation2, ICoreAutomationRemoteOperation2_Vtbl, 0xeefaf86f_e953_5099_8ce9_dca813482ba0);
impl windows_core::RuntimeType for ICoreAutomationRemoteOperation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ImportConnectionBoundObject: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreAutomationRemoteOperationContext, ICoreAutomationRemoteOperationContext_Vtbl, 0xb9af9cbb_3d3e_5918_a16b_7861626a3aeb);
impl windows_core::RuntimeType for ICoreAutomationRemoteOperationContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetOperand: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOperand: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOperand2: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationRemoteOperationOperandId, *mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreAutomationRemoteOperationExtensionProvider, ICoreAutomationRemoteOperationExtensionProvider_Vtbl, 0x88f53e67_dc69_553b_a0aa_70477e724da8);
impl windows_core::RuntimeType for ICoreAutomationRemoteOperationExtensionProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ICoreAutomationRemoteOperationExtensionProvider, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreAutomationRemoteOperationExtensionProvider {
    pub fn CallExtension<P1>(&self, extensionid: windows_core::GUID, context: P1, operandids: &[AutomationRemoteOperationOperandId]) -> windows_core::Result<()>
    where
        P1: windows_core::Param<CoreAutomationRemoteOperationContext>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CallExtension)(windows_core::Interface::as_raw(this), extensionid, context.param().abi(), operandids.len().try_into().unwrap(), operandids.as_ptr()).ok() }
    }
    pub fn IsExtensionSupported(&self, extensionid: windows_core::GUID) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExtensionSupported)(windows_core::Interface::as_raw(this), extensionid, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ICoreAutomationRemoteOperationExtensionProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
}
pub trait ICoreAutomationRemoteOperationExtensionProvider_Impl: windows_core::IUnknownImpl {
    fn CallExtension(&self, extensionId: &windows_core::GUID, context: windows_core::Ref<CoreAutomationRemoteOperationContext>, operandIds: &[AutomationRemoteOperationOperandId]) -> windows_core::Result<()>;
    fn IsExtensionSupported(&self, extensionId: &windows_core::GUID) -> windows_core::Result<bool>;
}
impl ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub const fn new<Identity: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallExtension<Identity: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extensionid: windows_core::GUID, context: *mut core::ffi::c_void, operandids_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreAutomationRemoteOperationExtensionProvider_Impl::CallExtension(this, core::mem::transmute(&extensionid), core::mem::transmute_copy(&context), core::slice::from_raw_parts(core::mem::transmute_copy(&operandids), operandids_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn IsExtensionSupported<Identity: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extensionid: windows_core::GUID, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreAutomationRemoteOperationExtensionProvider_Impl::IsExtensionSupported(this, core::mem::transmute(&extensionid)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreAutomationRemoteOperationExtensionProvider, OFFSET>(),
            CallExtension: CallExtension::<Identity, OFFSET>,
            IsExtensionSupported: IsExtensionSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperationExtensionProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CallExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, u32, *const AutomationRemoteOperationOperandId) -> windows_core::HRESULT,
    pub IsExtensionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteAutomationClientSession, IRemoteAutomationClientSession_Vtbl, 0x5c8a091d_94cc_5b33_afdb_678cded2bd54);
impl windows_core::RuntimeType for IRemoteAutomationClientSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ConnectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveConnectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteAutomationClientSessionFactory, IRemoteAutomationClientSessionFactory_Vtbl, 0xf250263d_6057_5373_a5a5_ed7265fe0376);
impl windows_core::RuntimeType for IRemoteAutomationClientSessionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationClientSessionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteAutomationConnectionRequestedEventArgs, IRemoteAutomationConnectionRequestedEventArgs_Vtbl, 0xea3319a8_e3a8_5dc6_adf8_044e46b14af5);
impl windows_core::RuntimeType for IRemoteAutomationConnectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationConnectionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalPipeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteAutomationDisconnectedEventArgs, IRemoteAutomationDisconnectedEventArgs_Vtbl, 0xbbb33a3d_5d90_5c38_9eb2_dd9dcc1b2e3f);
impl windows_core::RuntimeType for IRemoteAutomationDisconnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationDisconnectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalPipeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteAutomationServerStatics, IRemoteAutomationServerStatics_Vtbl, 0xe6e8945e_0c11_5028_9ae3_c2771288b6b7);
impl windows_core::RuntimeType for IRemoteAutomationServerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationServerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportSession: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteAutomationWindow, IRemoteAutomationWindow_Vtbl, 0x7c607689_496d_512a_9bd5_c050cfaf1428);
impl windows_core::RuntimeType for IRemoteAutomationWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteAutomationWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutomationProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteAutomationClientSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteAutomationClientSession, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteAutomationClientSession {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateWindowAsync<P2>(&self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: P2) -> windows_core::Result<windows_future::IAsyncOperation<RemoteAutomationWindow>>
    where
        P2: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWindowAsync)(windows_core::Interface::as_raw(this), remotewindowid, remoteprocessid, parentautomationelement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SessionId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConnectionRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionRequested(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Disconnected<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Disconnected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisconnected(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateInstance(name: &windows_core::HSTRING) -> windows_core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2(name: &windows_core::HSTRING, sessionid: windows_core::GUID) -> windows_core::Result<RemoteAutomationClientSession> {
        Self::IRemoteAutomationClientSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), sessionid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IRemoteAutomationClientSessionFactory<R, F: FnOnce(&IRemoteAutomationClientSessionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteAutomationClientSession, IRemoteAutomationClientSessionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteAutomationClientSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteAutomationClientSession>();
}
unsafe impl windows_core::Interface for RemoteAutomationClientSession {
    type Vtable = <IRemoteAutomationClientSession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRemoteAutomationClientSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteAutomationClientSession {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationClientSession";
}
unsafe impl Send for RemoteAutomationClientSession {}
unsafe impl Sync for RemoteAutomationClientSession {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteAutomationConnectionRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteAutomationConnectionRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteAutomationConnectionRequestedEventArgs {
    pub fn LocalPipeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalPipeName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RemoteProcessId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteProcessId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RemoteAutomationConnectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteAutomationConnectionRequestedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteAutomationConnectionRequestedEventArgs {
    type Vtable = <IRemoteAutomationConnectionRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRemoteAutomationConnectionRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteAutomationConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationConnectionRequestedEventArgs";
}
unsafe impl Send for RemoteAutomationConnectionRequestedEventArgs {}
unsafe impl Sync for RemoteAutomationConnectionRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteAutomationDisconnectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteAutomationDisconnectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteAutomationDisconnectedEventArgs {
    pub fn LocalPipeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalPipeName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteAutomationDisconnectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteAutomationDisconnectedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteAutomationDisconnectedEventArgs {
    type Vtable = <IRemoteAutomationDisconnectedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRemoteAutomationDisconnectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteAutomationDisconnectedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationDisconnectedEventArgs";
}
unsafe impl Send for RemoteAutomationDisconnectedEventArgs {}
unsafe impl Sync for RemoteAutomationDisconnectedEventArgs {}
pub struct RemoteAutomationServer;
impl RemoteAutomationServer {
    pub fn ReportSession(sessionid: windows_core::GUID) -> windows_core::Result<()> {
        Self::IRemoteAutomationServerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ReportSession)(windows_core::Interface::as_raw(this), sessionid).ok() })
    }
    fn IRemoteAutomationServerStatics<R, F: FnOnce(&IRemoteAutomationServerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteAutomationServer, IRemoteAutomationServerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for RemoteAutomationServer {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationServer";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteAutomationWindow(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteAutomationWindow, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteAutomationWindow {
    pub fn AutomationProvider(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomationProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnregisterAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnregisterAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RemoteAutomationWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteAutomationWindow>();
}
unsafe impl windows_core::Interface for RemoteAutomationWindow {
    type Vtable = <IRemoteAutomationWindow as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRemoteAutomationWindow as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteAutomationWindow {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.RemoteAutomationWindow";
}
unsafe impl Send for RemoteAutomationWindow {}
unsafe impl Sync for RemoteAutomationWindow {}
