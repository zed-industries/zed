pub struct GameControllerFactoryManager;
impl GameControllerFactoryManager {
    pub fn RegisterCustomFactoryForGipInterface<P0>(factory: P0, interfaceid: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RegisterCustomFactoryForGipInterface)(windows_core::Interface::as_raw(this), factory.param().abi(), interfaceid).ok() })
    }
    pub fn RegisterCustomFactoryForHardwareId<P0>(factory: P0, hardwarevendorid: u16, hardwareproductid: u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RegisterCustomFactoryForHardwareId)(windows_core::Interface::as_raw(this), factory.param().abi(), hardwarevendorid, hardwareproductid).ok() })
    }
    pub fn RegisterCustomFactoryForXusbType<P0>(factory: P0, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RegisterCustomFactoryForXusbType)(windows_core::Interface::as_raw(this), factory.param().abi(), xusbtype, xusbsubtype).ok() })
    }
    pub fn TryGetFactoryControllerFromGameController<P0, P1>(factory: P0, gamecontroller: P1) -> windows_core::Result<super::IGameController>
    where
        P0: windows_core::Param<ICustomGameControllerFactory>,
        P1: windows_core::Param<super::IGameController>,
    {
        Self::IGameControllerFactoryManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetFactoryControllerFromGameController)(windows_core::Interface::as_raw(this), factory.param().abi(), gamecontroller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGameControllerFactoryManagerStatics<R, F: FnOnce(&IGameControllerFactoryManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IGameControllerFactoryManagerStatics2<R, F: FnOnce(&IGameControllerFactoryManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GameControllerFactoryManager {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GameControllerFactoryManager";
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl windows_core::TypeKind for GameControllerVersionInfo {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameControllerVersionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GameControllerVersionInfo;u2;u2;u2;u2)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl windows_core::TypeKind for GipFirmwareUpdateProgress {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GipFirmwareUpdateProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress;f8;u4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GipFirmwareUpdateResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GipFirmwareUpdateResult, windows_core::IUnknown, windows_core::IInspectable);
impl GipFirmwareUpdateResult {
    pub fn ExtendedErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FinalComponentId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinalComponentId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<GipFirmwareUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GipFirmwareUpdateResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGipFirmwareUpdateResult>();
}
unsafe impl windows_core::Interface for GipFirmwareUpdateResult {
    type Vtable = <IGipFirmwareUpdateResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGipFirmwareUpdateResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
}
unsafe impl Send for GipFirmwareUpdateResult {}
unsafe impl Sync for GipFirmwareUpdateResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: Self = Self(0i32);
    pub const UpToDate: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl windows_core::TypeKind for GipFirmwareUpdateStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GipFirmwareUpdateStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GipGameControllerProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GipGameControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GipGameControllerProvider, IGameControllerProvider);
impl GipGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirmwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareProductId(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVendorId(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConnected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SendMessage(&self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendMessage)(windows_core::Interface::as_raw(this), messageclass, messageid, messagebuffer.len().try_into().unwrap(), messagebuffer.as_ptr()).ok() }
    }
    pub fn SendReceiveMessage(&self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[u8], responsemessagebuffer: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendReceiveMessage)(windows_core::Interface::as_raw(this), messageclass, messageid, requestmessagebuffer.len().try_into().unwrap(), requestmessagebuffer.as_ptr(), responsemessagebuffer.len().try_into().unwrap(), responsemessagebuffer.as_mut_ptr()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn UpdateFirmwareAsync<P0>(&self, firmwareimage: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateFirmwareAsync)(windows_core::Interface::as_raw(this), firmwareimage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GipGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGipGameControllerProvider>();
}
unsafe impl windows_core::Interface for GipGameControllerProvider {
    type Vtable = <IGipGameControllerProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGipGameControllerProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipGameControllerProvider";
}
unsafe impl Send for GipGameControllerProvider {}
unsafe impl Sync for GipGameControllerProvider {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
    pub const StandardLatency: Self = Self(2i32);
}
impl windows_core::TypeKind for GipMessageClass {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GipMessageClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipMessageClass;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HidGameControllerProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HidGameControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HidGameControllerProvider, IGameControllerProvider);
impl HidGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirmwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareProductId(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVendorId(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConnected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsageId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsageId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsagePage(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsagePage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetFeatureReport(&self, reportid: u8, reportbuffer: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetFeatureReport)(windows_core::Interface::as_raw(this), reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_mut_ptr()).ok() }
    }
    pub fn SendFeatureReport(&self, reportid: u8, reportbuffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendFeatureReport)(windows_core::Interface::as_raw(this), reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_ptr()).ok() }
    }
    pub fn SendOutputReport(&self, reportid: u8, reportbuffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendOutputReport)(windows_core::Interface::as_raw(this), reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_ptr()).ok() }
    }
}
impl windows_core::RuntimeType for HidGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHidGameControllerProvider>();
}
unsafe impl windows_core::Interface for HidGameControllerProvider {
    type Vtable = <IHidGameControllerProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHidGameControllerProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.HidGameControllerProvider";
}
unsafe impl Send for HidGameControllerProvider {}
unsafe impl Sync for HidGameControllerProvider {}
windows_core::imp::define_interface!(ICustomGameControllerFactory, ICustomGameControllerFactory_Vtbl, 0x69a0ae5e_758e_4cbe_ace6_62155fe9126f);
impl windows_core::RuntimeType for ICustomGameControllerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ICustomGameControllerFactory, windows_core::IUnknown, windows_core::IInspectable);
impl ICustomGameControllerFactory {
    pub fn CreateGameController<P0>(&self, provider: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<IGameControllerProvider>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateGameController)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnGameControllerAdded<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IGameController>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnGameControllerAdded)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OnGameControllerRemoved<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IGameController>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnGameControllerRemoved)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for ICustomGameControllerFactory {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.ICustomGameControllerFactory";
}
pub trait ICustomGameControllerFactory_Impl: windows_core::IUnknownImpl {
    fn CreateGameController(&self, provider: windows_core::Ref<IGameControllerProvider>) -> windows_core::Result<windows_core::IInspectable>;
    fn OnGameControllerAdded(&self, value: windows_core::Ref<super::IGameController>) -> windows_core::Result<()>;
    fn OnGameControllerRemoved(&self, value: windows_core::Ref<super::IGameController>) -> windows_core::Result<()>;
}
impl ICustomGameControllerFactory_Vtbl {
    pub const fn new<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateGameController<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICustomGameControllerFactory_Impl::CreateGameController(this, core::mem::transmute_copy(&provider)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnGameControllerAdded<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICustomGameControllerFactory_Impl::OnGameControllerAdded(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn OnGameControllerRemoved<Identity: ICustomGameControllerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICustomGameControllerFactory_Impl::OnGameControllerRemoved(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICustomGameControllerFactory, OFFSET>(),
            CreateGameController: CreateGameController::<Identity, OFFSET>,
            OnGameControllerAdded: OnGameControllerAdded::<Identity, OFFSET>,
            OnGameControllerRemoved: OnGameControllerRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomGameControllerFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomGameControllerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnGameControllerAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnGameControllerRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameControllerFactoryManagerStatics, IGameControllerFactoryManagerStatics_Vtbl, 0x36cb66e3_d0a1_4986_a24c_40b137deba9e);
impl windows_core::RuntimeType for IGameControllerFactoryManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RegisterCustomFactoryForGipInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterCustomFactoryForHardwareId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub RegisterCustomFactoryForXusbType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XusbDeviceType, XusbDeviceSubtype) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameControllerFactoryManagerStatics2, IGameControllerFactoryManagerStatics2_Vtbl, 0xeace5644_19df_4115_b32a_2793e2aea3bb);
impl windows_core::RuntimeType for IGameControllerFactoryManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetFactoryControllerFromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameControllerInputSink, IGameControllerInputSink_Vtbl, 0x1ff6f922_c640_4c78_a820_9a715c558bcb);
impl windows_core::RuntimeType for IGameControllerInputSink {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGameControllerInputSink, windows_core::IUnknown, windows_core::IInspectable);
impl IGameControllerInputSink {
    pub fn OnInputResumed(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnInputResumed)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnInputSuspended)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl windows_core::RuntimeName for IGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerInputSink";
}
pub trait IGameControllerInputSink_Impl: windows_core::IUnknownImpl {
    fn OnInputResumed(&self, timestamp: u64) -> windows_core::Result<()>;
    fn OnInputSuspended(&self, timestamp: u64) -> windows_core::Result<()>;
}
impl IGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IGameControllerInputSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInputResumed<Identity: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameControllerInputSink_Impl::OnInputResumed(this, timestamp).into()
            }
        }
        unsafe extern "system" fn OnInputSuspended<Identity: IGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameControllerInputSink_Impl::OnInputSuspended(this, timestamp).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameControllerInputSink, OFFSET>(),
            OnInputResumed: OnInputResumed::<Identity, OFFSET>,
            OnInputSuspended: OnInputSuspended::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameControllerInputSink as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerInputSink_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnInputResumed: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub OnInputSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameControllerProvider, IGameControllerProvider_Vtbl, 0xe6d73982_2996_4559_b16c_3e57d46e58d6);
impl windows_core::RuntimeType for IGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGameControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirmwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareProductId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVendorId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGameControllerProvider";
}
pub trait IGameControllerProvider_Impl: windows_core::IUnknownImpl {
    fn FirmwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo>;
    fn HardwareProductId(&self) -> windows_core::Result<u16>;
    fn HardwareVendorId(&self) -> windows_core::Result<u16>;
    fn HardwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo>;
    fn IsConnected(&self) -> windows_core::Result<bool>;
}
impl IGameControllerProvider_Vtbl {
    pub const fn new<Identity: IGameControllerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FirmwareVersionInfo<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameControllerProvider_Impl::FirmwareVersionInfo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HardwareProductId<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameControllerProvider_Impl::HardwareProductId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HardwareVendorId<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameControllerProvider_Impl::HardwareVendorId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HardwareVersionInfo<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameControllerProvider_Impl::HardwareVersionInfo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IGameControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameControllerProvider_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameControllerProvider, OFFSET>(),
            FirmwareVersionInfo: FirmwareVersionInfo::<Identity, OFFSET>,
            HardwareProductId: HardwareProductId::<Identity, OFFSET>,
            HardwareVendorId: HardwareVendorId::<Identity, OFFSET>,
            HardwareVersionInfo: HardwareVersionInfo::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameControllerProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FirmwareVersionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameControllerVersionInfo) -> windows_core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub HardwareVersionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameControllerVersionInfo) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGipFirmwareUpdateResult, IGipFirmwareUpdateResult_Vtbl, 0x6b794d32_8553_4292_8e03_e16651a2f8bc);
impl windows_core::RuntimeType for IGipFirmwareUpdateResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FinalComponentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GipFirmwareUpdateStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGipGameControllerInputSink, IGipGameControllerInputSink_Vtbl, 0xa2108abf_09f1_43bc_a140_80f899ec36fb);
impl windows_core::RuntimeType for IGipGameControllerInputSink {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGipGameControllerInputSink, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IGipGameControllerInputSink, IGameControllerInputSink);
impl IGipGameControllerInputSink {
    pub fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnKeyReceived)(windows_core::Interface::as_raw(this), timestamp, keycode, ispressed).ok() }
    }
    pub fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnMessageReceived)(windows_core::Interface::as_raw(this), timestamp, messageclass, messageid, sequenceid, messagebuffer.len().try_into().unwrap(), messagebuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnInputResumed)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnInputSuspended)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl windows_core::RuntimeName for IGipGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IGipGameControllerInputSink";
}
pub trait IGipGameControllerInputSink_Impl: IGameControllerInputSink_Impl {
    fn OnKeyReceived(&self, timestamp: u64, keyCode: u8, isPressed: bool) -> windows_core::Result<()>;
    fn OnMessageReceived(&self, timestamp: u64, messageClass: GipMessageClass, messageId: u8, sequenceId: u8, messageBuffer: &[u8]) -> windows_core::Result<()>;
}
impl IGipGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IGipGameControllerInputSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnKeyReceived<Identity: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGipGameControllerInputSink_Impl::OnKeyReceived(this, timestamp, keycode, ispressed).into()
            }
        }
        unsafe extern "system" fn OnMessageReceived<Identity: IGipGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer_array_size: u32, messagebuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGipGameControllerInputSink_Impl::OnMessageReceived(this, timestamp, messageclass, messageid, sequenceid, core::slice::from_raw_parts(core::mem::transmute_copy(&messagebuffer), messagebuffer_array_size as usize)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGipGameControllerInputSink, OFFSET>(),
            OnKeyReceived: OnKeyReceived::<Identity, OFFSET>,
            OnMessageReceived: OnMessageReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGipGameControllerInputSink as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerInputSink_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnKeyReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u8, bool) -> windows_core::HRESULT,
    pub OnMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u64, GipMessageClass, u8, u8, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGipGameControllerProvider, IGipGameControllerProvider_Vtbl, 0xdbcf1e19_1af5_45a8_bf02_a0ee50c823fc);
impl windows_core::RuntimeType for IGipGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessage: unsafe extern "system" fn(*mut core::ffi::c_void, GipMessageClass, u8, u32, *const u8) -> windows_core::HRESULT,
    pub SendReceiveMessage: unsafe extern "system" fn(*mut core::ffi::c_void, GipMessageClass, u8, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub UpdateFirmwareAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UpdateFirmwareAsync: usize,
}
windows_core::imp::define_interface!(IHidGameControllerInputSink, IHidGameControllerInputSink_Vtbl, 0xf754c322_182d_40e4_a126_fcee4ffa1e31);
impl windows_core::RuntimeType for IHidGameControllerInputSink {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IHidGameControllerInputSink, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IHidGameControllerInputSink, IGameControllerInputSink);
impl IHidGameControllerInputSink {
    pub fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnInputReportReceived)(windows_core::Interface::as_raw(this), timestamp, reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnInputResumed)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnInputSuspended)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl windows_core::RuntimeName for IHidGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IHidGameControllerInputSink";
}
pub trait IHidGameControllerInputSink_Impl: IGameControllerInputSink_Impl {
    fn OnInputReportReceived(&self, timestamp: u64, reportId: u8, reportBuffer: &[u8]) -> windows_core::Result<()>;
}
impl IHidGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IHidGameControllerInputSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInputReportReceived<Identity: IHidGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, reportid: u8, reportbuffer_array_size: u32, reportbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHidGameControllerInputSink_Impl::OnInputReportReceived(this, timestamp, reportid, core::slice::from_raw_parts(core::mem::transmute_copy(&reportbuffer), reportbuffer_array_size as usize)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHidGameControllerInputSink, OFFSET>(),
            OnInputReportReceived: OnInputReportReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHidGameControllerInputSink as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerInputSink_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnInputReportReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u8, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHidGameControllerProvider, IHidGameControllerProvider_Vtbl, 0x95ce3af4_abf0_4b68_a081_3b7de73ff0e7);
impl windows_core::RuntimeType for IHidGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UsageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetFeatureReport: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u32, *mut u8) -> windows_core::HRESULT,
    pub SendFeatureReport: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u32, *const u8) -> windows_core::HRESULT,
    pub SendOutputReport: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXusbGameControllerInputSink, IXusbGameControllerInputSink_Vtbl, 0xb2ac1d95_6ecb_42b3_8aab_025401ca4712);
impl windows_core::RuntimeType for IXusbGameControllerInputSink {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IXusbGameControllerInputSink, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IXusbGameControllerInputSink, IGameControllerInputSink);
impl IXusbGameControllerInputSink {
    pub fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnInputReceived)(windows_core::Interface::as_raw(this), timestamp, reportid, inputbuffer.len().try_into().unwrap(), inputbuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnInputResumed)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnInputSuspended)(windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl windows_core::RuntimeName for IXusbGameControllerInputSink {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.IXusbGameControllerInputSink";
}
pub trait IXusbGameControllerInputSink_Impl: IGameControllerInputSink_Impl {
    fn OnInputReceived(&self, timestamp: u64, reportId: u8, inputBuffer: &[u8]) -> windows_core::Result<()>;
}
impl IXusbGameControllerInputSink_Vtbl {
    pub const fn new<Identity: IXusbGameControllerInputSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInputReceived<Identity: IXusbGameControllerInputSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u64, reportid: u8, inputbuffer_array_size: u32, inputbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXusbGameControllerInputSink_Impl::OnInputReceived(this, timestamp, reportid, core::slice::from_raw_parts(core::mem::transmute_copy(&inputbuffer), inputbuffer_array_size as usize)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXusbGameControllerInputSink, OFFSET>(),
            OnInputReceived: OnInputReceived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXusbGameControllerInputSink as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerInputSink_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnInputReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u8, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXusbGameControllerProvider, IXusbGameControllerProvider_Vtbl, 0x6e2971eb_0efb_48b4_808b_837643b2f216);
impl windows_core::RuntimeType for IXusbGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetVibration: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
    pub const ArcadePad: Self = Self(2i32);
    pub const ArcadeStick: Self = Self(3i32);
    pub const FlightStick: Self = Self(4i32);
    pub const Wheel: Self = Self(5i32);
    pub const Guitar: Self = Self(6i32);
    pub const GuitarAlternate: Self = Self(7i32);
    pub const GuitarBass: Self = Self(8i32);
    pub const DrumKit: Self = Self(9i32);
    pub const DancePad: Self = Self(10i32);
}
impl windows_core::TypeKind for XusbDeviceSubtype {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for XusbDeviceSubtype {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceSubtype;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
}
impl windows_core::TypeKind for XusbDeviceType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for XusbDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XusbGameControllerProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XusbGameControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XusbGameControllerProvider, IGameControllerProvider);
impl XusbGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirmwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareProductId(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVendorId(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> windows_core::Result<GameControllerVersionInfo> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVersionInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConnected(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVibration)(windows_core::Interface::as_raw(this), lowfrequencymotorspeed, highfrequencymotorspeed).ok() }
    }
}
impl windows_core::RuntimeType for XusbGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXusbGameControllerProvider>();
}
unsafe impl windows_core::Interface for XusbGameControllerProvider {
    type Vtable = <IXusbGameControllerProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXusbGameControllerProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.XusbGameControllerProvider";
}
unsafe impl Send for XusbGameControllerProvider {}
unsafe impl Sync for XusbGameControllerProvider {}
