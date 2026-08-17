windows_core::imp::define_interface!(IMcpHttpConnectionResult, IMcpHttpConnectionResult_Vtbl, 0xd2c3755f_6d3c_5e90_84dd_3e0973049606);
impl windows_core::RuntimeType for IMcpHttpConnectionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpHttpConnectionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
}
windows_core::imp::define_interface!(IMcpNamedPipeConnectionResult, IMcpNamedPipeConnectionResult_Vtbl, 0x8a2aef6f_b4dc_5180_a3e1_47b63dbbb70a);
impl windows_core::RuntimeType for IMcpNamedPipeConnectionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpNamedPipeConnectionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMcpNamedPipeConnectionServer, IMcpNamedPipeConnectionServer_Vtbl, 0x52f204a5_2ad1_5430_96c9_ea7e090be839);
impl windows_core::RuntimeType for IMcpNamedPipeConnectionServer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IMcpNamedPipeConnectionServer, windows_core::IUnknown, windows_core::IInspectable);
impl IMcpNamedPipeConnectionServer {
    pub fn Connect<P0, P2>(&self, hostcontext: P0, pipename: &windows_core::HSTRING, connectionresult: P2) -> windows_core::Result<McpNamedPipeConnectionResult>
    where
        P0: windows_core::Param<super::AgentContext>,
        P2: windows_core::Param<McpNamedPipeConnectionResult>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connect)(windows_core::Interface::as_raw(this), hostcontext.param().abi(), core::mem::transmute_copy(pipename), connectionresult.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IMcpNamedPipeConnectionServer {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.IMcpNamedPipeConnectionServer";
}
pub trait IMcpNamedPipeConnectionServer_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, hostContext: windows_core::Ref<super::AgentContext>, pipeName: &windows_core::HSTRING, connectionResult: windows_core::Ref<McpNamedPipeConnectionResult>) -> windows_core::Result<McpNamedPipeConnectionResult>;
}
impl IMcpNamedPipeConnectionServer_Vtbl {
    pub const fn new<Identity: IMcpNamedPipeConnectionServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IMcpNamedPipeConnectionServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostcontext: *mut core::ffi::c_void, pipename: *mut core::ffi::c_void, connectionresult: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMcpNamedPipeConnectionServer_Impl::Connect(this, core::mem::transmute_copy(&hostcontext), core::mem::transmute(&pipename), core::mem::transmute_copy(&connectionresult)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMcpNamedPipeConnectionServer, OFFSET>(), Connect: Connect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcpNamedPipeConnectionServer as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpNamedPipeConnectionServer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMcpServerRegistry, IMcpServerRegistry_Vtbl, 0x150f795b_3f93_4493_abc7_48a04fd2d7b6);
impl windows_core::RuntimeType for IMcpServerRegistry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpServerRegistry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAgentInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub GetMcpConnectionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::super::super::UI::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetMcpConnectionInfo: usize,
}
windows_core::imp::define_interface!(IMcpServerRegistryStatics, IMcpServerRegistryStatics_Vtbl, 0x4acf7fed_d300_55bc_9dde_9f433cdc903d);
impl windows_core::RuntimeType for IMcpServerRegistryStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpServerRegistryStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMcpSseConnectionServer, IMcpSseConnectionServer_Vtbl, 0x6c558671_1b20_5b6b_920d_b8afc2509771);
impl windows_core::RuntimeType for IMcpSseConnectionServer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IMcpSseConnectionServer, windows_core::IUnknown, windows_core::IInspectable);
impl IMcpSseConnectionServer {
    pub fn Connect<P0, P1>(&self, hostcontext: P0, connectionresult: P1) -> windows_core::Result<McpHttpConnectionResult>
    where
        P0: windows_core::Param<super::AgentContext>,
        P1: windows_core::Param<McpHttpConnectionResult>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connect)(windows_core::Interface::as_raw(this), hostcontext.param().abi(), connectionresult.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IMcpSseConnectionServer {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.IMcpSseConnectionServer";
}
pub trait IMcpSseConnectionServer_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, hostContext: windows_core::Ref<super::AgentContext>, connectionResult: windows_core::Ref<McpHttpConnectionResult>) -> windows_core::Result<McpHttpConnectionResult>;
}
impl IMcpSseConnectionServer_Vtbl {
    pub const fn new<Identity: IMcpSseConnectionServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IMcpSseConnectionServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostcontext: *mut core::ffi::c_void, connectionresult: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMcpSseConnectionServer_Impl::Connect(this, core::mem::transmute_copy(&hostcontext), core::mem::transmute_copy(&connectionresult)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMcpSseConnectionServer, OFFSET>(), Connect: Connect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMcpSseConnectionServer as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpSseConnectionServer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMcpStdioConnectionInfo, IMcpStdioConnectionInfo_Vtbl, 0x93d9827b_32a2_5b89_ba8a_05bd2093598e);
impl windows_core::RuntimeType for IMcpStdioConnectionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMcpStdioConnectionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCommandArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpHttpConnectionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(McpHttpConnectionResult, windows_core::IUnknown, windows_core::IInspectable);
impl McpHttpConnectionResult {
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for McpHttpConnectionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMcpHttpConnectionResult>();
}
unsafe impl windows_core::Interface for McpHttpConnectionResult {
    type Vtable = <IMcpHttpConnectionResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMcpHttpConnectionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for McpHttpConnectionResult {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.McpHttpConnectionResult";
}
unsafe impl Send for McpHttpConnectionResult {}
unsafe impl Sync for McpHttpConnectionResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpNamedPipeConnectionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(McpNamedPipeConnectionResult, windows_core::IUnknown, windows_core::IInspectable);
impl McpNamedPipeConnectionResult {}
impl windows_core::RuntimeType for McpNamedPipeConnectionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMcpNamedPipeConnectionResult>();
}
unsafe impl windows_core::Interface for McpNamedPipeConnectionResult {
    type Vtable = <IMcpNamedPipeConnectionResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMcpNamedPipeConnectionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for McpNamedPipeConnectionResult {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.McpNamedPipeConnectionResult";
}
unsafe impl Send for McpNamedPipeConnectionResult {}
unsafe impl Sync for McpNamedPipeConnectionResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpServerRegistry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(McpServerRegistry, windows_core::IUnknown, windows_core::IInspectable);
impl McpServerRegistry {
    pub fn GetAgentInfos(&self) -> windows_core::Result<windows_core::Array<super::AgentInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetAgentInfos)(windows_core::Interface::as_raw(this), windows_core::Array::<super::AgentInfo>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "UI")]
    pub fn GetMcpConnectionInfo(&self, agentid: windows_core::GUID, ownerwindowid: super::super::super::UI::WindowId) -> windows_core::Result<McpStdioConnectionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMcpConnectionInfo)(windows_core::Interface::as_raw(this), agentid, ownerwindowid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<McpServerRegistry> {
        Self::IMcpServerRegistryStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IMcpServerRegistryStatics<R, F: FnOnce(&IMcpServerRegistryStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<McpServerRegistry, IMcpServerRegistryStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for McpServerRegistry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMcpServerRegistry>();
}
unsafe impl windows_core::Interface for McpServerRegistry {
    type Vtable = <IMcpServerRegistry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMcpServerRegistry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for McpServerRegistry {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.McpServerRegistry";
}
unsafe impl Send for McpServerRegistry {}
unsafe impl Sync for McpServerRegistry {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpStdioConnectionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(McpStdioConnectionInfo, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(McpStdioConnectionInfo, super::super::super::Foundation::IClosable);
impl McpStdioConnectionInfo {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Command(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Command)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetCommandArguments(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetCommandArguments)(windows_core::Interface::as_raw(this), windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Info(&self) -> windows_core::Result<super::AgentInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Info)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for McpStdioConnectionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMcpStdioConnectionInfo>();
}
unsafe impl windows_core::Interface for McpStdioConnectionInfo {
    type Vtable = <IMcpStdioConnectionInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMcpStdioConnectionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for McpStdioConnectionInfo {
    const NAME: &'static str = "Windows.AI.Agents.Mcp.McpStdioConnectionInfo";
}
unsafe impl Send for McpStdioConnectionInfo {}
unsafe impl Sync for McpStdioConnectionInfo {}
