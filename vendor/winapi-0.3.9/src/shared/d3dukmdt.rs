// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Longhorn Display Driver Model (LDDM) user/kernel mode shared data type definitions.
use shared::basetsd::{UINT64, ULONG_PTR};
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, UINT, ULONG};
use shared::ntdef::{HANDLE, LUID, ULONGLONG, VOID};
pub const DXGKDDI_INTERFACE_VERSION_VISTA: ULONG = 0x1052;
pub const DXGKDDI_INTERFACE_VERSION_VISTA_SP1: ULONG = 0x1053;
pub const DXGKDDI_INTERFACE_VERSION_WIN7: ULONG = 0x2005;
pub const DXGKDDI_INTERFACE_VERSION_WIN8: ULONG = 0x300E;
pub const DXGKDDI_INTERFACE_VERSION_WDDM1_3: ULONG = 0x4002;
pub const DXGKDDI_INTERFACE_VERSION_WDDM1_3_PATH_INDEPENDENT_ROTATION: ULONG = 0x4003;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_0: ULONG = 0x5023;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1: ULONG = 0x6003;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1_5: ULONG = 0x6010;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_2: ULONG = 0x700A;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_3: ULONG = 0x8001;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_4: ULONG = 0x9006;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_5: ULONG = 0xA00B;
#[inline]
pub fn IS_OFFICIAL_DDI_INTERFACE_VERSION(version: ULONG) -> bool {
    (version == DXGKDDI_INTERFACE_VERSION_VISTA) ||
    (version == DXGKDDI_INTERFACE_VERSION_VISTA_SP1) ||
    (version == DXGKDDI_INTERFACE_VERSION_WIN7) ||
    (version == DXGKDDI_INTERFACE_VERSION_WIN8) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM1_3) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM1_3_PATH_INDEPENDENT_ROTATION) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM2_0) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM2_1) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM2_1_5) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM2_2) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM2_3) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM2_4) ||
    (version == DXGKDDI_INTERFACE_VERSION_WDDM2_5)
}
pub const DXGKDDI_INTERFACE_VERSION: ULONG = DXGKDDI_INTERFACE_VERSION_WDDM2_5;
pub const D3D_UMD_INTERFACE_VERSION_VISTA: ULONG = 0x000C;
pub const D3D_UMD_INTERFACE_VERSION_WIN7: ULONG = 0x2003;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_M3: ULONG = 0x3001;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_CP: ULONG = 0x3002;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_RC: ULONG = 0x3003;
pub const D3D_UMD_INTERFACE_VERSION_WIN8: ULONG = 0x3004;
pub const D3D_UMD_INTERFACE_VERSION_WDDM1_3: ULONG = 0x4002;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M1: ULONG = 0x5000;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M1_3: ULONG = 0x5001;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M2_2: ULONG = 0x5002;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0: ULONG = 0x5002;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_1: ULONG = 0x6000;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_2: ULONG = 0x6001;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_3: ULONG = 0x6002;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_4: ULONG = 0x6003;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1: ULONG = D3D_UMD_INTERFACE_VERSION_WDDM2_1_4;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2_1: ULONG = 0x7000;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2_2: ULONG = 0x7001;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2: ULONG = D3D_UMD_INTERFACE_VERSION_WDDM2_2_2;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3_1: ULONG = 0x8000;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3_2: ULONG = 0x8001;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3: ULONG = D3D_UMD_INTERFACE_VERSION_WDDM2_3_2;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4_1: ULONG = 0x9000;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4_2: ULONG = 0x9001;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4: ULONG = D3D_UMD_INTERFACE_VERSION_WDDM2_4_2;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_1: ULONG = 0xA000;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_2: ULONG = 0xA001;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_3: ULONG = 0xA002;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5: ULONG = D3D_UMD_INTERFACE_VERSION_WDDM2_5_3;
pub const D3D_UMD_INTERFACE_VERSION: ULONG = D3D_UMD_INTERFACE_VERSION_WDDM2_5;
pub type D3DGPU_VIRTUAL_ADDRESS = ULONGLONG;
pub type D3DGPU_SIZE_T = ULONGLONG;
pub const D3DGPU_UNIQUE_DRIVER_PROTECTION: ULONGLONG = 0x8000000000000000;
pub const DXGK_MAX_PAGE_TABLE_LEVEL_COUNT: UINT = 6;
pub const DXGK_MIN_PAGE_TABLE_LEVEL_COUNT: UINT = 2;
STRUCT!{struct GPUP_DRIVER_ESCAPE_INPUT {
    vfLUID: LUID,
}}
pub type PGPUP_DRIVER_ESCAPE_INPUT = *mut GPUP_DRIVER_ESCAPE_INPUT;
ENUM!{enum DXGKVGPU_ESCAPE_TYPE {
    DXGKVGPU_ESCAPE_TYPE_READ_PCI_CONFIG = 0,
    DXGKVGPU_ESCAPE_TYPE_WRITE_PCI_CONFIG = 1,
    DXGKVGPU_ESCAPE_TYPE_INITIALIZE = 2,
    DXGKVGPU_ESCAPE_TYPE_RELEASE = 3,
    DXGKVGPU_ESCAPE_TYPE_GET_VGPU_TYPE = 4,
    DXGKVGPU_ESCAPE_TYPE_POWERTRANSITIONCOMPLETE = 5,
}}
STRUCT!{struct DXGKVGPU_ESCAPE_HEAD {
    Luid: GPUP_DRIVER_ESCAPE_INPUT,
    Type: DXGKVGPU_ESCAPE_TYPE,
}}
STRUCT!{struct DXGKVGPU_ESCAPE_READ_PCI_CONFIG {
    Header: DXGKVGPU_ESCAPE_HEAD,
    Offset: UINT,
    Size: UINT,
}}
STRUCT!{struct DXGKVGPU_ESCAPE_WRITE_PCI_CONFIG {
    Header: DXGKVGPU_ESCAPE_HEAD,
    Offset: UINT,
    Size: UINT,
}}
STRUCT!{struct DXGKVGPU_ESCAPE_READ_VGPU_TYPE {
    Header: DXGKVGPU_ESCAPE_HEAD,
}}
STRUCT!{struct DXGKVGPU_ESCAPE_POWERTRANSITIONCOMPLETE {
    Header: DXGKVGPU_ESCAPE_HEAD,
    PowerState: UINT,
}}
STRUCT!{struct DXGKVGPU_ESCAPE_INITIALIZE {
    Header: DXGKVGPU_ESCAPE_HEAD,
    VmGuid: GUID,
}}
STRUCT!{struct DXGKVGPU_ESCAPE_RELEASE {
    Header: DXGKVGPU_ESCAPE_HEAD,
}}
ENUM!{enum DXGK_PTE_PAGE_SIZE {
    DXGK_PTE_PAGE_TABLE_PAGE_4KB = 0,
    DXGK_PTE_PAGE_TABLE_PAGE_64KB = 1,
}}
UNION!{union DXGK_PTE_u {
    [u64; 1],
    PageAddress PageAddress_mut: ULONGLONG,
    PageTableAddress PageTableAddress_mut: ULONGLONG,
}}
STRUCT!{struct DXGK_PTE {
    Flags: ULONGLONG,
    u: DXGK_PTE_u,
}}
BITFIELD!{DXGK_PTE Flags: ULONGLONG [
    Valid set_Valid[0..1],
    Zero set_Zero[1..2],
    CacheCoherent set_CacheCoherent[2..3],
    ReadOnly set_ReadOnly[3..4],
    NoExecute set_NoExecute[4..5],
    Segment set_Segment[5..10],
    LargePage set_LargePage[10..11],
    PhysicalAdapterIndex set_PhysicalAdapterIndex[11..17],
    PageTablePageSize set_PageTablePageSize[17..19],
    SystemReserved0 set_SystemReserved0[19..20],
    Reserved set_Reserved[20..64],
]}
pub const D3DGPU_NULL: D3DGPU_VIRTUAL_ADDRESS = 0;
pub const D3DDDI_MAX_WRITTEN_PRIMARIES: usize = 16;
pub const D3DDDI_MAX_MPO_PRESENT_DIRTY_RECTS: usize = 0xFFF;
STRUCT!{struct D3DGPU_PHYSICAL_ADDRESS {
    SegmentId: UINT,
    SegmentOffset: UINT64,
}}
pub type D3DDDI_VIDEO_PRESENT_SOURCE_ID = UINT;
pub type D3DDDI_VIDEO_PRESENT_TARGET_ID = UINT;
pub type D3DKMT_HANDLE = UINT;
STRUCT!{struct D3DDDI_RATIONAL {
    Numerator: UINT,
    Denominator: UINT,
}}
STRUCT!{struct D3DDDI_ALLOCATIONINFO {
    hAllocation: D3DKMT_HANDLE,
    pSystemMem: *const VOID,
    pPrivateDriverData: *mut VOID,
    PrivateDriverDataSize: UINT,
    VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    Flags: UINT,
}}
BITFIELD!{D3DDDI_ALLOCATIONINFO Flags: UINT [
    Primary set_Primary[0..1],
    Stereo set_Stereo[1..2],
    Reserved set_Reserved[2..32],
]}
UNION!{union D3DDDI_ALLOCATIONINFO2_u1 {
    [usize; 1],
    hSection hSection_mut: HANDLE,
    pSystemMem pSystemMem_mut: *const VOID,
}}
UNION!{union D3DDDI_ALLOCATIONINFO2_u2 {
    [usize; 1],
    Priority Priority_mut: UINT,
    Unused Unused_mut: ULONG_PTR,
}}
STRUCT!{struct D3DDDI_ALLOCATIONINFO2 {
    hAllocation: D3DKMT_HANDLE,
    u1: D3DDDI_ALLOCATIONINFO2_u1,
    pPrivateDriverData: *mut VOID,
    PrivateDriverDataSize: UINT,
    VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    Flags: UINT,
    GpuVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    u2: D3DDDI_ALLOCATIONINFO2_u2,
    Reserved: [ULONG_PTR; 5],
}}
BITFIELD!{D3DDDI_ALLOCATIONINFO2 Flags: UINT [
    Primary set_Primary[0..1],
    Stereo set_Stereo[1..2],
    OverridePriority set_OverridePriority[2..3],
    Reserved set_Reserved[3..32],
]}
STRUCT!{struct D3DDDI_OPENALLOCATIONINFO {
    hAllocation: D3DKMT_HANDLE,
    pPrivateDriverData: *const VOID,
    PrivateDriverDataSize: UINT,
}}
STRUCT!{struct D3DDDI_OPENALLOCATIONINFO2 {
    hAllocation: D3DKMT_HANDLE,
    pPrivateDriverData: *const VOID,
    PrivateDriverDataSize: UINT,
    GpuVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    Reserved: [ULONG_PTR; 6],
}}
ENUM!{enum D3DDDI_OFFER_PRIORITY {
    D3DDDI_OFFER_PRIORITY_NONE = 0,
    D3DDDI_OFFER_PRIORITY_LOW = 1,
    D3DDDI_OFFER_PRIORITY_NORMAL,
    D3DDDI_OFFER_PRIORITY_HIGH,
    D3DDDI_OFFER_PRIORITY_AUTO,
}}
STRUCT!{struct D3DDDI_ALLOCATIONLIST {
    hAllocation: D3DKMT_HANDLE,
    Value: UINT,
}}
BITFIELD!{D3DDDI_ALLOCATIONLIST Value: UINT [
    WriteOperation set_WriteOperation[0..1],
    DoNotRetireInstance set_DoNotRetireInstance[1..2],
    OfferPriority set_OfferPriority[2..5],
    Reserved set_Reserved[5..32],
]}
STRUCT!{struct D3DDDI_PATCHLOCATIONLIST {
    AllocationIndex: UINT,
    Value: UINT,
    DriverId: UINT,
    AllocationOffset: UINT,
    PatchOffset: UINT,
    SplitOffset: UINT,
}}
BITFIELD!{D3DDDI_PATCHLOCATIONLIST Value: UINT [
    SlotId set_SlotId[0..24],
    Reserved set_Reserved[24..32],
]}
STRUCT!{struct D3DDDICB_LOCKFLAGS {
    Value: UINT,
}}
BITFIELD!{D3DDDICB_LOCKFLAGS Value: UINT [
    ReadOnly set_ReadOnly[0..1],
    WriteOnly set_WriteOnly[1..2],
    DonotWait set_DonotWait[2..3],
    IgnoreSync set_IgnoreSync[3..4],
    LockEntire set_LockEntire[4..5],
    DonotEvict set_DonotEvict[5..6],
    AcquireAperture set_AcquireAperture[6..7],
    Discard set_Discard[7..8],
    NoExistingReference set_NoExistingReference[8..9],
    UseAlternateVA set_UseAlternateVA[9..10],
    IgnoreReadSync set_IgnoreReadSync[10..11],
    Reserved set_Reserved[11..32],
]}
STRUCT!{struct D3DDDICB_LOCK2FLAGS {
    Value: UINT,
}}
STRUCT!{struct D3DDDICB_DESTROYALLOCATION2FLAGS {
    Value: UINT,
}}
BITFIELD!{D3DDDICB_DESTROYALLOCATION2FLAGS Value: UINT [
    AssumeNotInUse set_AssumeNotInUse[0..1],
    SynchronousDestroy set_SynchronousDestroy[1..2],
    Reserved set_Reserved[2..31],
    SystemUseOnly set_SystemUseOnly[31..32],
]}
STRUCT!{struct D3DDDI_ESCAPEFLAGS {
    Value: UINT,
}}
BITFIELD!{D3DDDI_ESCAPEFLAGS Value: UINT [
    HardwareAccess set_HardwareAccess[0..1],
    DeviceStatusQuery set_DeviceStatusQuery[1..2],
    ChangeFrameLatency set_ChangeFrameLatency[2..3],
    NoAdapterSynchronization set_NoAdapterSynchronization[3..4],
    Reserved set_Reserved[4..5],
    VirtualMachineData set_VirtualMachineData[5..6],
    DriverKnownEscape set_DriverKnownEscape[6..7],
    DriverCommonEscape set_DriverCommonEscape[7..8],
    Reserved2 set_Reserved2[8..24],
]}
ENUM!{enum D3DDDI_DRIVERESCAPETYPE {
    D3DDDI_DRIVERESCAPETYPE_TRANSLATEALLOCATIONHANDLE = 0,
    D3DDDI_DRIVERESCAPETYPE_TRANSLATERESOURCEHANDLE = 1,
    D3DDDI_DRIVERESCAPETYPE_MAX,
}}
STRUCT!{struct D3DDDI_DRIVERESCAPE_TRANSLATEALLOCATIONEHANDLE {
    EscapeType: D3DDDI_DRIVERESCAPETYPE,
    hAllocation: D3DKMT_HANDLE,
}}
STRUCT!{struct D3DDDI_DRIVERESCAPE_TRANSLATERESOURCEHANDLE {
    EscapeType: D3DDDI_DRIVERESCAPETYPE,
    hResource: D3DKMT_HANDLE,
}}
STRUCT!{struct D3DDDI_CREATECONTEXTFLAGS {
    Value: UINT,
}}
BITFIELD!{D3DDDI_CREATECONTEXTFLAGS Value: UINT [
    NullRendering set_NullRendering[0..1],
    InitialData set_InitialData[1..2],
    DisableGpuTimeout set_DisableGpuTimeout[2..3],
    SynchronizationOnly set_SynchronizationOnly[3..4],
    HwQueueSupported set_HwQueueSupported[4..5],
    Reserved set_Reserved[5..32],
]}
//1188
STRUCT!{struct D3DDDICB_SIGNALFLAGS {
    Value: UINT,
}}
BITFIELD!{D3DDDICB_SIGNALFLAGS Value: UINT [
    SignalAtSubmission set_SignalAtSubmission[0..1],
    EnqueueCpuEvent set_EnqueueCpuEvent[1..2],
    AllowFenceRewind set_AllowFenceRewind[2..3],
    Reserved set_Reserved[3..31],
    DXGK_SIGNAL_FLAG_INTERNAL0 set_DXGK_SIGNAL_FLAG_INTERNAL0[31..32],
]}
pub const D3DDDI_MAX_OBJECT_WAITED_ON: usize = 32;
pub const D3DDDI_MAX_OBJECT_SIGNALED: usize = 32;
ENUM!{enum D3DDDI_SYNCHRONIZATIONOBJECT_TYPE {
    D3DDDI_SYNCHRONIZATION_MUTEX = 1,
    D3DDDI_SEMAPHORE = 2,
    D3DDDI_FENCE = 3,
    D3DDDI_CPU_NOTIFICATION = 4,
    D3DDDI_MONITORED_FENCE = 5,
    D3DDDI_PERIODIC_MONITORED_FENCE = 6,
    D3DDDI_SYNCHRONIZATION_TYPE_LIMIT,
}}
//1553
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_u_SynchronizationMutex {
    InitialState: BOOL,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_u_Semaphore {
    MaxCount: UINT,
    InitialCount: UINT,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_u_Reserved {
    Reserved: [UINT; 16],
}}
UNION!{union D3DDDI_SYNCHRONIZATIONOBJECTINFO_u {
    [u32; 16],
    SynchronizationMutex SynchronizationMutex_mut:
        D3DDDI_SYNCHRONIZATIONOBJECTINFO_u_SynchronizationMutex,
    Semaphore Semaphore_mut: D3DDDI_SYNCHRONIZATIONOBJECTINFO_u_Semaphore,
    Reserved Reserved_mut: D3DDDI_SYNCHRONIZATIONOBJECTINFO_u_Reserved,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO {
    Type: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE,
    u: D3DDDI_SYNCHRONIZATIONOBJECTINFO_u,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS {
    Value: UINT,
}}
BITFIELD!{D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS Value: UINT [
    Shared set_Shared[0..1],
    NtSecuritySharing set_NtSecuritySharing[1..2],
    CrossAdapter set_CrossAdapter[2..3],
    TopOfPipeline set_TopOfPipeline[3..4],
    NoSignal set_NoSignal[4..5],
    NoWait set_NoWait[5..6],
    NoSignalMaxValueOnTdr set_NoSignalMaxValueOnTdr[6..7],
    NoGPUAccess set_NoGPUAccess[7..8],
    Reserved set_Reserved[8..31],
    D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_RESERVED0
        set_D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_RESERVED0[31..32],
]}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_SynchronizationMutex {
    InitialState: BOOL,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_Semaphore {
    MaxCount: UINT,
    InitialCount: UINT,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_Fence {
    FenceValue: UINT64,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_CPUNotification {
    Event: HANDLE,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_MonitoredFence {
    InitialFenceValue: UINT64,
    FenceValueCPUVirtualAddress: *mut VOID,
    FenceValueGPUVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    EngineAffinity: UINT,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_PeriodicMonitoredFence {
    hAdapter: D3DKMT_HANDLE,
    VidPnTargetId: D3DDDI_VIDEO_PRESENT_TARGET_ID,
    Time: UINT64,
    FenceValueCPUVirtualAddress: *mut VOID,
    FenceValueGPUVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    EngineAffinity: UINT,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_Reserved {
    Reserved: [UINT64; 8],
}}
UNION!{union D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u {
    [u64; 8],
    SynchronizationMutex SynchronizationMutex_mut:
        D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_SynchronizationMutex,
    Semaphore Semaphore_mut: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_Semaphore,
    Fence Fence_mut: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_Fence,
    CPUNotification CPUNotification_mut: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_CPUNotification,
    MonitoredFence MonitoredFence_mut: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_MonitoredFence,
    PeriodicMonitoredFence PeriodicMonitoredFence_mut:
        D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_PeriodicMonitoredFence,
    Reserved Reserved_mut: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u_Reserved,
}}
STRUCT!{struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2 {
    Type: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE,
    Flags: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    u: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_u,
    SharedHandle: D3DKMT_HANDLE,
}}
//1778
pub const D3DDDI_MAX_BROADCAST_CONTEXT: usize = 64;
