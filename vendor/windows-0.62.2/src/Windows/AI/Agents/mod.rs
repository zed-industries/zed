#[cfg(feature = "AI_Agents_Mcp")]
pub mod Mcp;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AgentAuthorizationResponse(pub i32);
impl AgentAuthorizationResponse {
    pub const Denied: Self = Self(0i32);
    pub const Approved: Self = Self(1i32);
}
impl windows_core::TypeKind for AgentAuthorizationResponse {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AgentAuthorizationResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Agents.AgentAuthorizationResponse;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AgentContext, windows_core::IUnknown, windows_core::IInspectable);
impl AgentContext {
    pub fn AppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppUserModelId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RequestResourceAccess<P0>(&self, resource: P0, description: &windows_core::HSTRING, reasonforasking: &windows_core::HSTRING) -> windows_core::Result<AgentAuthorizationResponse>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestResourceAccess)(windows_core::Interface::as_raw(this), resource.param().abi(), core::mem::transmute_copy(description), core::mem::transmute_copy(reasonforasking), &mut result__).map(|| result__)
        }
    }
    pub fn GetContextForCaller() -> windows_core::Result<AgentContext> {
        Self::IAgentContextStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContextForCaller)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAgentContextStatics<R, F: FnOnce(&IAgentContextStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AgentContext, IAgentContextStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AgentContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAgentContext>();
}
unsafe impl windows_core::Interface for AgentContext {
    type Vtable = <IAgentContext as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAgentContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AgentContext {
    const NAME: &'static str = "Windows.AI.Agents.AgentContext";
}
unsafe impl Send for AgentContext {}
unsafe impl Sync for AgentContext {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AgentInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AgentInfo {
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn GetPackage(&self) -> windows_core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPackage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AgentInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAgentInfo>();
}
unsafe impl windows_core::Interface for AgentInfo {
    type Vtable = <IAgentInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAgentInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AgentInfo {
    const NAME: &'static str = "Windows.AI.Agents.AgentInfo";
}
unsafe impl Send for AgentInfo {}
unsafe impl Sync for AgentInfo {}
pub struct AgentResources;
impl AgentResources {
    pub fn FileSystemRead() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileSystemRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FileSystemWrite() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileSystemWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FileSystemDelete() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileSystemDelete)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FileSystemCreate() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileSystemCreate)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn HttpGet() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HttpGet)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn HttpPost() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HttpPost)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn HttpPut() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HttpPut)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn HttpDelete() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAgentResourcesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HttpDelete)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    fn IAgentResourcesStatics<R, F: FnOnce(&IAgentResourcesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AgentResources, IAgentResourcesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AgentResources {
    const NAME: &'static str = "Windows.AI.Agents.AgentResources";
}
windows_core::imp::define_interface!(IAgentContext, IAgentContext_Vtbl, 0x67812fd9_f5fc_5431_b282_2fc753b0c2cd);
impl windows_core::RuntimeType for IAgentContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgentContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestResourceAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut AgentAuthorizationResponse) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAgentContextStatics, IAgentContextStatics_Vtbl, 0x0625abf6_79f6_5116_a14a_91b3967fc214);
impl windows_core::RuntimeType for IAgentContextStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgentContextStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetContextForCaller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAgentInfo, IAgentInfo_Vtbl, 0xb023d498_59ab_410b_83e7_1ed007ee2f68);
impl windows_core::RuntimeType for IAgentInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgentInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub GetPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    GetPackage: usize,
}
windows_core::imp::define_interface!(IAgentResourcesStatics, IAgentResourcesStatics_Vtbl, 0xadedaaf8_3487_50b4_ac42_490083642b05);
impl windows_core::RuntimeType for IAgentResourcesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgentResourcesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FileSystemRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileSystemWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileSystemDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileSystemCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HttpGet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HttpPost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HttpPut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HttpDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
