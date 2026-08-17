windows_core::imp::define_interface!(IPlatformTelemetryClientStatics, IPlatformTelemetryClientStatics_Vtbl, 0x9bf3f25d_d5c3_4eea_8dbe_9c8dbb0d9d8f);
impl windows_core::RuntimeType for IPlatformTelemetryClientStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformTelemetryClientStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlatformTelemetryRegistrationResult, IPlatformTelemetryRegistrationResult_Vtbl, 0x4d8518ab_2292_49bd_a15a_3d71d2145112);
impl windows_core::RuntimeType for IPlatformTelemetryRegistrationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformTelemetryRegistrationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformTelemetryRegistrationStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlatformTelemetryRegistrationSettings, IPlatformTelemetryRegistrationSettings_Vtbl, 0x819a8582_ca19_415e_bb79_9c224bfa3a73);
impl windows_core::RuntimeType for IPlatformTelemetryRegistrationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlatformTelemetryRegistrationSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StorageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetStorageSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UploadQuotaSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetUploadQuotaSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub struct PlatformTelemetryClient;
impl PlatformTelemetryClient {
    pub fn Register(id: &windows_core::HSTRING) -> windows_core::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RegisterWithSettings<P0>(id: &windows_core::HSTRING, settings: P0) -> windows_core::Result<PlatformTelemetryRegistrationResult>
    where
        P0: windows_core::Param<PlatformTelemetryRegistrationSettings>,
    {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlatformTelemetryClientStatics<R, F: FnOnce(&IPlatformTelemetryClientStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlatformTelemetryClient, IPlatformTelemetryClientStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlatformTelemetryClient {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlatformTelemetryRegistrationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlatformTelemetryRegistrationResult, windows_core::IUnknown, windows_core::IInspectable);
impl PlatformTelemetryRegistrationResult {
    pub fn Status(&self) -> windows_core::Result<PlatformTelemetryRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlatformTelemetryRegistrationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlatformTelemetryRegistrationResult>();
}
unsafe impl windows_core::Interface for PlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_Vtbl;
    const IID: windows_core::GUID = <IPlatformTelemetryRegistrationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlatformTelemetryRegistrationResult {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult";
}
unsafe impl Send for PlatformTelemetryRegistrationResult {}
unsafe impl Sync for PlatformTelemetryRegistrationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlatformTelemetryRegistrationSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlatformTelemetryRegistrationSettings, windows_core::IUnknown, windows_core::IInspectable);
impl PlatformTelemetryRegistrationSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlatformTelemetryRegistrationSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StorageSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StorageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStorageSize(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStorageSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UploadQuotaSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UploadQuotaSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUploadQuotaSize(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUploadQuotaSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PlatformTelemetryRegistrationSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlatformTelemetryRegistrationSettings>();
}
unsafe impl windows_core::Interface for PlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_Vtbl;
    const IID: windows_core::GUID = <IPlatformTelemetryRegistrationSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlatformTelemetryRegistrationSettings {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings";
}
unsafe impl Send for PlatformTelemetryRegistrationSettings {}
unsafe impl Sync for PlatformTelemetryRegistrationSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const SettingsOutOfRange: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl windows_core::TypeKind for PlatformTelemetryRegistrationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlatformTelemetryRegistrationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlatformTelemetryRegistrationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlatformTelemetryRegistrationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationStatus;i4)");
}
