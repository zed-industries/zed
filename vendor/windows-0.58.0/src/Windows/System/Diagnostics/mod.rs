#[cfg(feature = "System_Diagnostics_DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "System_Diagnostics_Telemetry")]
pub mod Telemetry;
#[cfg(feature = "System_Diagnostics_TraceReporting")]
pub mod TraceReporting;
windows_core::imp::define_interface!(IDiagnosticActionResult, IDiagnosticActionResult_Vtbl, 0xc265a296_e73b_4097_b28f_3442f03dd831);
impl windows_core::RuntimeType for IDiagnosticActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDiagnosticActionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Results: usize,
}
windows_core::imp::define_interface!(IDiagnosticInvoker, IDiagnosticInvoker_Vtbl, 0x187b270a_02e3_4f86_84fc_fdd892b5940f);
impl windows_core::RuntimeType for IDiagnosticInvoker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDiagnosticInvoker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Json")]
    pub RunDiagnosticActionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Json"))]
    RunDiagnosticActionAsync: usize,
}
windows_core::imp::define_interface!(IDiagnosticInvoker2, IDiagnosticInvoker2_Vtbl, 0xe3bf945c_155a_4b52_a8ec_070c44f95000);
impl windows_core::RuntimeType for IDiagnosticInvoker2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDiagnosticInvoker2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RunDiagnosticActionFromStringAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDiagnosticInvokerStatics, IDiagnosticInvokerStatics_Vtbl, 0x5cfad8de_f15c_4554_a813_c113c3881b09);
impl windows_core::RuntimeType for IDiagnosticInvokerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDiagnosticInvokerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessCpuUsage, IProcessCpuUsage_Vtbl, 0x0bbb2472_c8bf_423a_a810_b559ae4354e2);
impl windows_core::RuntimeType for IProcessCpuUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessCpuUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessCpuUsageReport, IProcessCpuUsageReport_Vtbl, 0x8a6d9cac_3987_4e2f_a119_6b5fa214f1b4);
impl windows_core::RuntimeType for IProcessCpuUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessCpuUsageReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KernelTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UserTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessDiagnosticInfo, IProcessDiagnosticInfo_Vtbl, 0xe830b04b_300e_4ee6_a0ab_5b5f5231b434);
impl windows_core::RuntimeType for IProcessDiagnosticInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessDiagnosticInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub DiskUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MemoryUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CpuUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessDiagnosticInfo2, IProcessDiagnosticInfo2_Vtbl, 0x9558cb1a_3d0b_49ec_ab70_4f7a112805de);
impl windows_core::RuntimeType for IProcessDiagnosticInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessDiagnosticInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppDiagnosticInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppDiagnosticInfos: usize,
    pub IsPackaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessDiagnosticInfoStatics, IProcessDiagnosticInfoStatics_Vtbl, 0x2f41b260_b49f_428c_aa0e_84744f49ca95);
impl windows_core::RuntimeType for IProcessDiagnosticInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessDiagnosticInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForProcesses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForProcesses: usize,
    pub GetForCurrentProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessDiagnosticInfoStatics2, IProcessDiagnosticInfoStatics2_Vtbl, 0x4a869897_9899_4a44_a29b_091663be09b6);
impl windows_core::RuntimeType for IProcessDiagnosticInfoStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessDiagnosticInfoStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetForProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessDiskUsage, IProcessDiskUsage_Vtbl, 0x5ad78bfd_7e51_4e53_bfaa_5a6ee1aabbf8);
impl windows_core::RuntimeType for IProcessDiskUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessDiskUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessDiskUsageReport, IProcessDiskUsageReport_Vtbl, 0x401627fd_535d_4c1f_81b8_da54e1be635e);
impl windows_core::RuntimeType for IProcessDiskUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessDiskUsageReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadOperationCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub WriteOperationCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub OtherOperationCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub BytesReadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub BytesWrittenCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub OtherBytesCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessMemoryUsage, IProcessMemoryUsage_Vtbl, 0xf50b229b_827c_42b7_b07c_0e32627e6b3e);
impl windows_core::RuntimeType for IProcessMemoryUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessMemoryUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessMemoryUsageReport, IProcessMemoryUsageReport_Vtbl, 0xc2c77cba_1951_4685_8532_7e749ecf8eeb);
impl windows_core::RuntimeType for IProcessMemoryUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessMemoryUsageReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NonPagedPoolSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PageFaultCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PageFileSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PagedPoolSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PeakNonPagedPoolSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PeakPageFileSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PeakPagedPoolSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PeakVirtualMemorySizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PeakWorkingSetSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PrivatePageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub VirtualMemorySizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub WorkingSetSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemCpuUsage, ISystemCpuUsage_Vtbl, 0x6037b3ac_02d6_4234_8362_7fe3adc81f5f);
impl windows_core::RuntimeType for ISystemCpuUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemCpuUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemCpuUsageReport, ISystemCpuUsageReport_Vtbl, 0x2c26d0b2_9483_4f62_ab57_82b29d9719b8);
impl windows_core::RuntimeType for ISystemCpuUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemCpuUsageReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KernelTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UserTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub IdleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemDiagnosticInfo, ISystemDiagnosticInfo_Vtbl, 0xa290fe05_dff3_407f_9a1b_0b2b317ca800);
impl windows_core::RuntimeType for ISystemDiagnosticInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemDiagnosticInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MemoryUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CpuUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemDiagnosticInfoStatics, ISystemDiagnosticInfoStatics_Vtbl, 0xd404ac21_fc7d_45f0_9a3f_39203aed9f7e);
impl windows_core::RuntimeType for ISystemDiagnosticInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemDiagnosticInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemDiagnosticInfoStatics2, ISystemDiagnosticInfoStatics2_Vtbl, 0x79ded189_6af9_4da9_a422_15f73255b3eb);
impl windows_core::RuntimeType for ISystemDiagnosticInfoStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemDiagnosticInfoStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsArchitectureSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::ProcessorArchitecture, *mut bool) -> windows_core::HRESULT,
    pub PreferredArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ProcessorArchitecture) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMemoryUsage, ISystemMemoryUsage_Vtbl, 0x17ffc595_1702_49cf_aa27_2f0a32591404);
impl windows_core::RuntimeType for ISystemMemoryUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMemoryUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMemoryUsageReport, ISystemMemoryUsageReport_Vtbl, 0x38663c87_2a9f_403a_bd19_2cf3e8169500);
impl windows_core::RuntimeType for ISystemMemoryUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemMemoryUsageReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TotalPhysicalSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub AvailableSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub CommittedSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DiagnosticActionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DiagnosticActionResult, windows_core::IUnknown, windows_core::IInspectable);
impl DiagnosticActionResult {
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Results(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Results)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DiagnosticActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDiagnosticActionResult>();
}
unsafe impl windows_core::Interface for DiagnosticActionResult {
    type Vtable = IDiagnosticActionResult_Vtbl;
    const IID: windows_core::GUID = <IDiagnosticActionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DiagnosticActionResult {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticActionResult";
}
unsafe impl Send for DiagnosticActionResult {}
unsafe impl Sync for DiagnosticActionResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DiagnosticInvoker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DiagnosticInvoker, windows_core::IUnknown, windows_core::IInspectable);
impl DiagnosticInvoker {
    #[cfg(feature = "Data_Json")]
    pub fn RunDiagnosticActionAsync<P0>(&self, context: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>
    where
        P0: windows_core::Param<super::super::Data::Json::JsonObject>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunDiagnosticActionAsync)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RunDiagnosticActionFromStringAsync(&self, context: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>> {
        let this = &windows_core::Interface::cast::<IDiagnosticInvoker2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunDiagnosticActionFromStringAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(context), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<DiagnosticInvoker> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<DiagnosticInvoker>
    where
        P0: windows_core::Param<super::User>,
    {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IDiagnosticInvokerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IDiagnosticInvokerStatics<R, F: FnOnce(&IDiagnosticInvokerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DiagnosticInvoker, IDiagnosticInvokerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DiagnosticInvoker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDiagnosticInvoker>();
}
unsafe impl windows_core::Interface for DiagnosticInvoker {
    type Vtable = IDiagnosticInvoker_Vtbl;
    const IID: windows_core::GUID = <IDiagnosticInvoker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DiagnosticInvoker {
    const NAME: &'static str = "Windows.System.Diagnostics.DiagnosticInvoker";
}
unsafe impl Send for DiagnosticInvoker {}
unsafe impl Sync for DiagnosticInvoker {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessCpuUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessCpuUsage, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessCpuUsage {
    pub fn GetReport(&self) -> windows_core::Result<ProcessCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProcessCpuUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessCpuUsage>();
}
unsafe impl windows_core::Interface for ProcessCpuUsage {
    type Vtable = IProcessCpuUsage_Vtbl;
    const IID: windows_core::GUID = <IProcessCpuUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsage";
}
unsafe impl Send for ProcessCpuUsage {}
unsafe impl Sync for ProcessCpuUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessCpuUsageReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessCpuUsageReport, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessCpuUsageReport {
    pub fn KernelTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KernelTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UserTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ProcessCpuUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessCpuUsageReport>();
}
unsafe impl windows_core::Interface for ProcessCpuUsageReport {
    type Vtable = IProcessCpuUsageReport_Vtbl;
    const IID: windows_core::GUID = <IProcessCpuUsageReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessCpuUsageReport";
}
unsafe impl Send for ProcessCpuUsageReport {}
unsafe impl Sync for ProcessCpuUsageReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessDiagnosticInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessDiagnosticInfo, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessDiagnosticInfo {
    pub fn ProcessId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExecutableFileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecutableFileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parent(&self) -> windows_core::Result<ProcessDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessStartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessStartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DiskUsage(&self) -> windows_core::Result<ProcessDiskUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiskUsage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MemoryUsage(&self) -> windows_core::Result<ProcessMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MemoryUsage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CpuUsage(&self) -> windows_core::Result<ProcessCpuUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CpuUsage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppDiagnosticInfos(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>> {
        let this = &windows_core::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppDiagnosticInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPackaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IProcessDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPackaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetForProcesses() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForProcesses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForCurrentProcess() -> windows_core::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentProcess)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryGetForProcessId(processid: u32) -> windows_core::Result<ProcessDiagnosticInfo> {
        Self::IProcessDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetForProcessId)(windows_core::Interface::as_raw(this), processid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IProcessDiagnosticInfoStatics<R, F: FnOnce(&IProcessDiagnosticInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProcessDiagnosticInfoStatics2<R, F: FnOnce(&IProcessDiagnosticInfoStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProcessDiagnosticInfo, IProcessDiagnosticInfoStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProcessDiagnosticInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessDiagnosticInfo>();
}
unsafe impl windows_core::Interface for ProcessDiagnosticInfo {
    type Vtable = IProcessDiagnosticInfo_Vtbl;
    const IID: windows_core::GUID = <IProcessDiagnosticInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiagnosticInfo";
}
unsafe impl Send for ProcessDiagnosticInfo {}
unsafe impl Sync for ProcessDiagnosticInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessDiskUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessDiskUsage, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessDiskUsage {
    pub fn GetReport(&self) -> windows_core::Result<ProcessDiskUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProcessDiskUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessDiskUsage>();
}
unsafe impl windows_core::Interface for ProcessDiskUsage {
    type Vtable = IProcessDiskUsage_Vtbl;
    const IID: windows_core::GUID = <IProcessDiskUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessDiskUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsage";
}
unsafe impl Send for ProcessDiskUsage {}
unsafe impl Sync for ProcessDiskUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessDiskUsageReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessDiskUsageReport, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessDiskUsageReport {
    pub fn ReadOperationCount(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadOperationCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WriteOperationCount(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteOperationCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OtherOperationCount(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherOperationCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BytesReadCount(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesReadCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BytesWrittenCount(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesWrittenCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OtherBytesCount(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherBytesCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ProcessDiskUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessDiskUsageReport>();
}
unsafe impl windows_core::Interface for ProcessDiskUsageReport {
    type Vtable = IProcessDiskUsageReport_Vtbl;
    const IID: windows_core::GUID = <IProcessDiskUsageReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessDiskUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessDiskUsageReport";
}
unsafe impl Send for ProcessDiskUsageReport {}
unsafe impl Sync for ProcessDiskUsageReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessMemoryUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessMemoryUsage, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessMemoryUsage {
    pub fn GetReport(&self) -> windows_core::Result<ProcessMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProcessMemoryUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessMemoryUsage>();
}
unsafe impl windows_core::Interface for ProcessMemoryUsage {
    type Vtable = IProcessMemoryUsage_Vtbl;
    const IID: windows_core::GUID = <IProcessMemoryUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsage";
}
unsafe impl Send for ProcessMemoryUsage {}
unsafe impl Sync for ProcessMemoryUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessMemoryUsageReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessMemoryUsageReport, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessMemoryUsageReport {
    pub fn NonPagedPoolSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonPagedPoolSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PageFaultCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageFaultCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PageFileSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PageFileSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PagedPoolSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PagedPoolSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PeakNonPagedPoolSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeakNonPagedPoolSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PeakPageFileSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeakPageFileSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PeakPagedPoolSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeakPagedPoolSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PeakVirtualMemorySizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeakVirtualMemorySizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PeakWorkingSetSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeakWorkingSetSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PrivatePageCount(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrivatePageCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VirtualMemorySizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualMemorySizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WorkingSetSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WorkingSetSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ProcessMemoryUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessMemoryUsageReport>();
}
unsafe impl windows_core::Interface for ProcessMemoryUsageReport {
    type Vtable = IProcessMemoryUsageReport_Vtbl;
    const IID: windows_core::GUID = <IProcessMemoryUsageReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.ProcessMemoryUsageReport";
}
unsafe impl Send for ProcessMemoryUsageReport {}
unsafe impl Sync for ProcessMemoryUsageReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemCpuUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemCpuUsage, windows_core::IUnknown, windows_core::IInspectable);
impl SystemCpuUsage {
    pub fn GetReport(&self) -> windows_core::Result<SystemCpuUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SystemCpuUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemCpuUsage>();
}
unsafe impl windows_core::Interface for SystemCpuUsage {
    type Vtable = ISystemCpuUsage_Vtbl;
    const IID: windows_core::GUID = <ISystemCpuUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemCpuUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsage";
}
unsafe impl Send for SystemCpuUsage {}
unsafe impl Sync for SystemCpuUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemCpuUsageReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemCpuUsageReport, windows_core::IUnknown, windows_core::IInspectable);
impl SystemCpuUsageReport {
    pub fn KernelTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KernelTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UserTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IdleTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IdleTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SystemCpuUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemCpuUsageReport>();
}
unsafe impl windows_core::Interface for SystemCpuUsageReport {
    type Vtable = ISystemCpuUsageReport_Vtbl;
    const IID: windows_core::GUID = <ISystemCpuUsageReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemCpuUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemCpuUsageReport";
}
unsafe impl Send for SystemCpuUsageReport {}
unsafe impl Sync for SystemCpuUsageReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemDiagnosticInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemDiagnosticInfo, windows_core::IUnknown, windows_core::IInspectable);
impl SystemDiagnosticInfo {
    pub fn MemoryUsage(&self) -> windows_core::Result<SystemMemoryUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MemoryUsage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CpuUsage(&self) -> windows_core::Result<SystemCpuUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CpuUsage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForCurrentSystem() -> windows_core::Result<SystemDiagnosticInfo> {
        Self::ISystemDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsArchitectureSupported(r#type: super::ProcessorArchitecture) -> windows_core::Result<bool> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsArchitectureSupported)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        })
    }
    pub fn PreferredArchitecture() -> windows_core::Result<super::ProcessorArchitecture> {
        Self::ISystemDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredArchitecture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemDiagnosticInfoStatics<R, F: FnOnce(&ISystemDiagnosticInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISystemDiagnosticInfoStatics2<R, F: FnOnce(&ISystemDiagnosticInfoStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemDiagnosticInfo, ISystemDiagnosticInfoStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemDiagnosticInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemDiagnosticInfo>();
}
unsafe impl windows_core::Interface for SystemDiagnosticInfo {
    type Vtable = ISystemDiagnosticInfo_Vtbl;
    const IID: windows_core::GUID = <ISystemDiagnosticInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemDiagnosticInfo {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemDiagnosticInfo";
}
unsafe impl Send for SystemDiagnosticInfo {}
unsafe impl Sync for SystemDiagnosticInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemMemoryUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemMemoryUsage, windows_core::IUnknown, windows_core::IInspectable);
impl SystemMemoryUsage {
    pub fn GetReport(&self) -> windows_core::Result<SystemMemoryUsageReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SystemMemoryUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemMemoryUsage>();
}
unsafe impl windows_core::Interface for SystemMemoryUsage {
    type Vtable = ISystemMemoryUsage_Vtbl;
    const IID: windows_core::GUID = <ISystemMemoryUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMemoryUsage {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsage";
}
unsafe impl Send for SystemMemoryUsage {}
unsafe impl Sync for SystemMemoryUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SystemMemoryUsageReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemMemoryUsageReport, windows_core::IUnknown, windows_core::IInspectable);
impl SystemMemoryUsageReport {
    pub fn TotalPhysicalSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TotalPhysicalSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AvailableSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailableSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CommittedSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommittedSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SystemMemoryUsageReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemMemoryUsageReport>();
}
unsafe impl windows_core::Interface for SystemMemoryUsageReport {
    type Vtable = ISystemMemoryUsageReport_Vtbl;
    const IID: windows_core::GUID = <ISystemMemoryUsageReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMemoryUsageReport {
    const NAME: &'static str = "Windows.System.Diagnostics.SystemMemoryUsageReport";
}
unsafe impl Send for SystemMemoryUsageReport {}
unsafe impl Sync for SystemMemoryUsageReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DiagnosticActionState(pub i32);
impl DiagnosticActionState {
    pub const Initializing: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const VerifyingTrust: Self = Self(2i32);
    pub const Detecting: Self = Self(3i32);
    pub const Resolving: Self = Self(4i32);
    pub const VerifyingResolution: Self = Self(5i32);
    pub const Executing: Self = Self(6i32);
}
impl windows_core::TypeKind for DiagnosticActionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DiagnosticActionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DiagnosticActionState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DiagnosticActionState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DiagnosticActionState;i4)");
}
