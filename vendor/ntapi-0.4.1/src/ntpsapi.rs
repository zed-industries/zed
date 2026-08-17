use crate::ntapi_base::{CLIENT_ID, KPRIORITY, PCLIENT_ID};
use crate::ntexapi::{PROCESS_DISK_COUNTERS, PROCESS_ENERGY_VALUES};
use crate::ntpebteb::{PPEB, PTEB};
use winapi::ctypes::c_void;
use winapi::shared::basetsd::{PSIZE_T, SIZE_T, ULONG64, ULONG_PTR};
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, LARGE_INTEGER, LIST_ENTRY, LONG, LONGLONG, NTSTATUS, NT_PRODUCT_TYPE,
    PHANDLE, PLARGE_INTEGER, POBJECT_ATTRIBUTES, PROCESSOR_NUMBER, PSINGLE_LIST_ENTRY, PULONG,
    PVOID, SINGLE_LIST_ENTRY, UCHAR, ULONG, ULONGLONG, UNICODE_STRING, USHORT, WCHAR,
};
use winapi::um::winnt::{
    ACCESS_MASK, CONTEXT, HARDWARE_COUNTER_TYPE, IO_COUNTERS, JOBOBJECTINFOCLASS,
    JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, LDT_ENTRY, MAX_HW_COUNTERS, PCONTEXT, PJOB_SET_ARRAY,
    PROCESS_MITIGATION_ASLR_POLICY, PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY,
    PROCESS_MITIGATION_CHILD_PROCESS_POLICY, PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY,
    PROCESS_MITIGATION_DYNAMIC_CODE_POLICY, PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY,
    PROCESS_MITIGATION_FONT_DISABLE_POLICY, PROCESS_MITIGATION_IMAGE_LOAD_POLICY,
    PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY, PROCESS_MITIGATION_POLICY,
    PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY, PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY,
    PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY, PSECURITY_QUALITY_OF_SERVICE,
};
#[cfg(not(target_arch = "aarch64"))]
use crate::winapi_local::um::winnt::NtCurrentTeb;
pub const GDI_HANDLE_BUFFER_SIZE32: usize = 34;
pub const GDI_HANDLE_BUFFER_SIZE64: usize = 60;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub const GDI_HANDLE_BUFFER_SIZE: usize = GDI_HANDLE_BUFFER_SIZE64;
#[cfg(target_arch = "x86")]
pub const GDI_HANDLE_BUFFER_SIZE: usize = GDI_HANDLE_BUFFER_SIZE32;
pub type GDI_HANDLE_BUFFER = [ULONG; GDI_HANDLE_BUFFER_SIZE];
pub type GDI_HANDLE_BUFFER32 = [ULONG; GDI_HANDLE_BUFFER_SIZE32];
pub type GDI_HANDLE_BUFFER64 = [ULONG; GDI_HANDLE_BUFFER_SIZE];
pub const TLS_EXPANSION_SLOTS: usize = 1024;
STRUCT!{struct PEB_LDR_DATA {
    Length: ULONG,
    Initialized: BOOLEAN,
    SsHandle: HANDLE,
    InLoadOrderModuleList: LIST_ENTRY,
    InMemoryOrderModuleList: LIST_ENTRY,
    InInitializationOrderModuleList: LIST_ENTRY,
    EntryInProgress: PVOID,
    ShutdownInProgress: BOOLEAN,
    ShutdownThreadId: HANDLE,
}}
pub type PPEB_LDR_DATA = *mut PEB_LDR_DATA;
STRUCT!{struct INITIAL_TEB_OldInitialTeb {
    OldStackBase: PVOID,
    OldStackLimit: PVOID,
}}
STRUCT!{struct INITIAL_TEB {
    OldInitialTeb: INITIAL_TEB_OldInitialTeb,
    StackBase: PVOID,
    StackLimit: PVOID,
    StackAllocationBase: PVOID,
}}
pub type PINITIAL_TEB = *mut INITIAL_TEB;
STRUCT!{struct WOW64_PROCESS {
    Wow64: PVOID,
}}
pub type PWOW64_PROCESS = *mut WOW64_PROCESS;
ENUM!{enum PROCESSINFOCLASS {
    ProcessBasicInformation = 0,
    ProcessQuotaLimits = 1,
    ProcessIoCounters = 2,
    ProcessVmCounters = 3,
    ProcessTimes = 4,
    ProcessBasePriority = 5,
    ProcessRaisePriority = 6,
    ProcessDebugPort = 7,
    ProcessExceptionPort = 8,
    ProcessAccessToken = 9,
    ProcessLdtInformation = 10,
    ProcessLdtSize = 11,
    ProcessDefaultHardErrorMode = 12,
    ProcessIoPortHandlers = 13,
    ProcessPooledUsageAndLimits = 14,
    ProcessWorkingSetWatch = 15,
    ProcessUserModeIOPL = 16,
    ProcessEnableAlignmentFaultFixup = 17,
    ProcessPriorityClass = 18,
    ProcessWx86Information = 19,
    ProcessHandleCount = 20,
    ProcessAffinityMask = 21,
    ProcessPriorityBoost = 22,
    ProcessDeviceMap = 23,
    ProcessSessionInformation = 24,
    ProcessForegroundInformation = 25,
    ProcessWow64Information = 26,
    ProcessImageFileName = 27,
    ProcessLUIDDeviceMapsEnabled = 28,
    ProcessBreakOnTermination = 29,
    ProcessDebugObjectHandle = 30,
    ProcessDebugFlags = 31,
    ProcessHandleTracing = 32,
    ProcessIoPriority = 33,
    ProcessExecuteFlags = 34,
    ProcessResourceManagement = 35,
    ProcessCookie = 36,
    ProcessImageInformation = 37,
    ProcessCycleTime = 38,
    ProcessPagePriority = 39,
    ProcessInstrumentationCallback = 40,
    ProcessThreadStackAllocation = 41,
    ProcessWorkingSetWatchEx = 42,
    ProcessImageFileNameWin32 = 43,
    ProcessImageFileMapping = 44,
    ProcessAffinityUpdateMode = 45,
    ProcessMemoryAllocationMode = 46,
    ProcessGroupInformation = 47,
    ProcessTokenVirtualizationEnabled = 48,
    ProcessConsoleHostProcess = 49,
    ProcessWindowInformation = 50,
    ProcessHandleInformation = 51,
    ProcessMitigationPolicy = 52,
    ProcessDynamicFunctionTableInformation = 53,
    ProcessHandleCheckingMode = 54,
    ProcessKeepAliveCount = 55,
    ProcessRevokeFileHandles = 56,
    ProcessWorkingSetControl = 57,
    ProcessHandleTable = 58,
    ProcessCheckStackExtentsMode = 59,
    ProcessCommandLineInformation = 60,
    ProcessProtectionInformation = 61,
    ProcessMemoryExhaustion = 62,
    ProcessFaultInformation = 63,
    ProcessTelemetryIdInformation = 64,
    ProcessCommitReleaseInformation = 65,
    ProcessDefaultCpuSetsInformation = 66,
    ProcessAllowedCpuSetsInformation = 67,
    ProcessSubsystemProcess = 68,
    ProcessJobMemoryInformation = 69,
    ProcessInPrivate = 70,
    ProcessRaiseUMExceptionOnInvalidHandleClose = 71,
    ProcessIumChallengeResponse = 72,
    ProcessChildProcessInformation = 73,
    ProcessHighGraphicsPriorityInformation = 74,
    ProcessSubsystemInformation = 75,
    ProcessEnergyValues = 76,
    ProcessActivityThrottleState = 77,
    ProcessActivityThrottlePolicy = 78,
    ProcessWin32kSyscallFilterInformation = 79,
    ProcessDisableSystemAllowedCpuSets = 80,
    ProcessWakeInformation = 81,
    ProcessEnergyTrackingState = 82,
    ProcessManageWritesToExecutableMemory = 83,
    ProcessCaptureTrustletLiveDump = 84,
    ProcessTelemetryCoverage = 85,
    ProcessEnclaveInformation = 86,
    ProcessEnableReadWriteVmLogging = 87,
    ProcessUptimeInformation = 88,
    ProcessImageSection = 89,
    ProcessDebugAuthInformation = 90,
    ProcessSystemResourceManagement = 91,
    ProcessSequenceNumber = 92,
    ProcessLoaderDetour = 93,
    ProcessSecurityDomainInformation = 94,
    ProcessCombineSecurityDomainsInformation = 95,
    ProcessEnableLogging = 96,
    ProcessLeapSecondInformation = 97,
    MaxProcessInfoClass = 98,
}}
ENUM!{enum THREADINFOCLASS {
    ThreadBasicInformation = 0,
    ThreadTimes = 1,
    ThreadPriority = 2,
    ThreadBasePriority = 3,
    ThreadAffinityMask = 4,
    ThreadImpersonationToken = 5,
    ThreadDescriptorTableEntry = 6,
    ThreadEnableAlignmentFaultFixup = 7,
    ThreadEventPair = 8,
    ThreadQuerySetWin32StartAddress = 9,
    ThreadZeroTlsCell = 10,
    ThreadPerformanceCount = 11,
    ThreadAmILastThread = 12,
    ThreadIdealProcessor = 13,
    ThreadPriorityBoost = 14,
    ThreadSetTlsArrayAddress = 15,
    ThreadIsIoPending = 16,
    ThreadHideFromDebugger = 17,
    ThreadBreakOnTermination = 18,
    ThreadSwitchLegacyState = 19,
    ThreadIsTerminated = 20,
    ThreadLastSystemCall = 21,
    ThreadIoPriority = 22,
    ThreadCycleTime = 23,
    ThreadPagePriority = 24,
    ThreadActualBasePriority = 25,
    ThreadTebInformation = 26,
    ThreadCSwitchMon = 27,
    ThreadCSwitchPmu = 28,
    ThreadWow64Context = 29,
    ThreadGroupInformation = 30,
    ThreadUmsInformation = 31,
    ThreadCounterProfiling = 32,
    ThreadIdealProcessorEx = 33,
    ThreadCpuAccountingInformation = 34,
    ThreadSuspendCount = 35,
    ThreadHeterogeneousCpuPolicy = 36,
    ThreadContainerId = 37,
    ThreadNameInformation = 38,
    ThreadSelectedCpuSets = 39,
    ThreadSystemThreadInformation = 40,
    ThreadActualGroupAffinity = 41,
    ThreadDynamicCodePolicyInfo = 42,
    ThreadExplicitCaseSensitivity = 43,
    ThreadWorkOnBehalfTicket = 44,
    ThreadSubsystemInformation = 45,
    ThreadDbgkWerReportActive = 46,
    ThreadAttachContainer = 47,
    ThreadManageWritesToExecutableMemory = 48,
    ThreadPowerThrottlingState = 49,
    ThreadWorkloadClass = 50,
    MaxThreadInfoClass = 51,
}}
STRUCT!{struct PAGE_PRIORITY_INFORMATION {
    PagePriority: ULONG,
}}
pub type PPAGE_PRIORITY_INFORMATION = *mut PAGE_PRIORITY_INFORMATION;
STRUCT!{struct PROCESS_BASIC_INFORMATION {
    ExitStatus: NTSTATUS,
    PebBaseAddress: PPEB,
    AffinityMask: ULONG_PTR,
    BasePriority: KPRIORITY,
    UniqueProcessId: HANDLE,
    InheritedFromUniqueProcessId: HANDLE,
}}
pub type PPROCESS_BASIC_INFORMATION = *mut PROCESS_BASIC_INFORMATION;
STRUCT!{struct PROCESS_EXTENDED_BASIC_INFORMATION {
    Size: SIZE_T,
    BasicInfo: PROCESS_BASIC_INFORMATION,
    Flags: ULONG,
}}
BITFIELD!{PROCESS_EXTENDED_BASIC_INFORMATION Flags: ULONG [
    IsProtectedProcess set_IsProtectedProcess[0..1],
    IsWow64Process set_IsWow64Process[1..2],
    IsProcessDeleting set_IsProcessDeleting[2..3],
    IsCrossSessionCreate set_IsCrossSessionCreate[3..4],
    IsFrozen set_IsFrozen[4..5],
    IsBackground set_IsBackground[5..6],
    IsStronglyNamed set_IsStronglyNamed[6..7],
    IsSecureProcess set_IsSecureProcess[7..8],
    IsSubsystemProcess set_IsSubsystemProcess[8..9],
    SpareBits set_SpareBits[9..32],
]}
pub type PPROCESS_EXTENDED_BASIC_INFORMATION = *mut PROCESS_EXTENDED_BASIC_INFORMATION;
STRUCT!{struct VM_COUNTERS {
    PeakVirtualSize: SIZE_T,
    VirtualSize: SIZE_T,
    PageFaultCount: ULONG,
    PeakWorkingSetSize: SIZE_T,
    WorkingSetSize: SIZE_T,
    QuotaPeakPagedPoolUsage: SIZE_T,
    QuotaPagedPoolUsage: SIZE_T,
    QuotaPeakNonPagedPoolUsage: SIZE_T,
    QuotaNonPagedPoolUsage: SIZE_T,
    PagefileUsage: SIZE_T,
    PeakPagefileUsage: SIZE_T,
}}
pub type PVM_COUNTERS = *mut VM_COUNTERS;
STRUCT!{struct VM_COUNTERS_EX {
    PeakVirtualSize: SIZE_T,
    VirtualSize: SIZE_T,
    PageFaultCount: ULONG,
    PeakWorkingSetSize: SIZE_T,
    WorkingSetSize: SIZE_T,
    QuotaPeakPagedPoolUsage: SIZE_T,
    QuotaPagedPoolUsage: SIZE_T,
    QuotaPeakNonPagedPoolUsage: SIZE_T,
    QuotaNonPagedPoolUsage: SIZE_T,
    PagefileUsage: SIZE_T,
    PeakPagefileUsage: SIZE_T,
    PrivateUsage: SIZE_T,
}}
pub type PVM_COUNTERS_EX = *mut VM_COUNTERS_EX;
STRUCT!{struct VM_COUNTERS_EX2 {
    CountersEx: VM_COUNTERS_EX,
    PrivateWorkingSetSize: SIZE_T,
    SharedCommitUsage: SIZE_T,
}}
pub type PVM_COUNTERS_EX2 = *mut VM_COUNTERS_EX2;
STRUCT!{struct KERNEL_USER_TIMES {
    CreateTime: LARGE_INTEGER,
    ExitTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
}}
pub type PKERNEL_USER_TIMES = *mut KERNEL_USER_TIMES;
STRUCT!{struct POOLED_USAGE_AND_LIMITS {
    PeakPagedPoolUsage: SIZE_T,
    PagedPoolUsage: SIZE_T,
    PagedPoolLimit: SIZE_T,
    PeakNonPagedPoolUsage: SIZE_T,
    NonPagedPoolUsage: SIZE_T,
    NonPagedPoolLimit: SIZE_T,
    PeakPagefileUsage: SIZE_T,
    PagefileUsage: SIZE_T,
    PagefileLimit: SIZE_T,
}}
pub type PPOOLED_USAGE_AND_LIMITS = *mut POOLED_USAGE_AND_LIMITS;
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_BITS: ULONG_PTR = 0x00000003;
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_FLAGS: ULONG_PTR =
    (1 << PROCESS_EXCEPTION_PORT_ALL_STATE_BITS) - 1;
STRUCT!{struct PROCESS_EXCEPTION_PORT {
    ExceptionPortHandle: HANDLE,
    StateFlags: ULONG,
}}
pub type PPROCESS_EXCEPTION_PORT = *mut PROCESS_EXCEPTION_PORT;
STRUCT!{struct PROCESS_ACCESS_TOKEN {
    Token: HANDLE,
    Thread: HANDLE,
}}
pub type PPROCESS_ACCESS_TOKEN = *mut PROCESS_ACCESS_TOKEN;
STRUCT!{struct PROCESS_LDT_INFORMATION {
    Start: ULONG,
    Length: ULONG,
    LdtEntries: [LDT_ENTRY; 1],
}}
pub type PPROCESS_LDT_INFORMATION = *mut PROCESS_LDT_INFORMATION;
STRUCT!{struct PROCESS_LDT_SIZE {
    Length: ULONG,
}}
pub type PPROCESS_LDT_SIZE = *mut PROCESS_LDT_SIZE;
STRUCT!{struct PROCESS_WS_WATCH_INFORMATION {
    FaultingPc: PVOID,
    FaultingVa: PVOID,
}}
pub type PPROCESS_WS_WATCH_INFORMATION = *mut PROCESS_WS_WATCH_INFORMATION;
STRUCT!{struct PROCESS_WS_WATCH_INFORMATION_EX {
    BasicInfo: PROCESS_WS_WATCH_INFORMATION,
    FaultingThreadId: ULONG_PTR,
    Flags: ULONG_PTR,
}}
pub type PPROCESS_WS_WATCH_INFORMATION_EX = *mut PROCESS_WS_WATCH_INFORMATION_EX;
pub const PROCESS_PRIORITY_CLASS_UNKNOWN: u32 = 0;
pub const PROCESS_PRIORITY_CLASS_IDLE: u32 = 1;
pub const PROCESS_PRIORITY_CLASS_NORMAL: u32 = 2;
pub const PROCESS_PRIORITY_CLASS_HIGH: u32 = 3;
pub const PROCESS_PRIORITY_CLASS_REALTIME: u32 = 4;
pub const PROCESS_PRIORITY_CLASS_BELOW_NORMAL: u32 = 5;
pub const PROCESS_PRIORITY_CLASS_ABOVE_NORMAL: u32 = 6;
STRUCT!{struct PROCESS_PRIORITY_CLASS {
    Foreground: BOOLEAN,
    PriorityClass: UCHAR,
}}
pub type PPROCESS_PRIORITY_CLASS = *mut PROCESS_PRIORITY_CLASS;
STRUCT!{struct PROCESS_FOREGROUND_BACKGROUND {
    Foreground: BOOLEAN,
}}
pub type PPROCESS_FOREGROUND_BACKGROUND = *mut PROCESS_FOREGROUND_BACKGROUND;
STRUCT!{struct PROCESS_DEVICEMAP_INFORMATION_Set {
    DirectoryHandle: HANDLE,
}}
STRUCT!{struct PROCESS_DEVICEMAP_INFORMATION_Query {
    DriveMap: ULONG,
    DriveType: [UCHAR; 32],
}}
UNION!{union PROCESS_DEVICEMAP_INFORMATION {
    Set: PROCESS_DEVICEMAP_INFORMATION_Set,
    Query: PROCESS_DEVICEMAP_INFORMATION_Query,
}}
pub type PPROCESS_DEVICEMAP_INFORMATION = *mut PROCESS_DEVICEMAP_INFORMATION;
pub const PROCESS_LUID_DOSDEVICES_ONLY: ULONG = 0x00000001;
STRUCT!{struct PROCESS_DEVICEMAP_INFORMATION_EX_u_Set {
    DirectoryHandle: HANDLE,
}}
STRUCT!{struct PROCESS_DEVICEMAP_INFORMATION_EX_u_Query {
    DriveMap: ULONG,
    DriveType: [UCHAR; 32],
}}
UNION!{union PROCESS_DEVICEMAP_INFORMATION_EX_u {
    Set: PROCESS_DEVICEMAP_INFORMATION_EX_u_Set,
    Query: PROCESS_DEVICEMAP_INFORMATION_EX_u_Query,
}}
STRUCT!{struct PROCESS_DEVICEMAP_INFORMATION_EX {
    u: PROCESS_DEVICEMAP_INFORMATION_EX_u,
    Flags: ULONG,
}}
pub type PPROCESS_DEVICEMAP_INFORMATION_EX = *mut PROCESS_DEVICEMAP_INFORMATION_EX;
STRUCT!{struct PROCESS_SESSION_INFORMATION {
    SessionId: ULONG,
}}
pub type PPROCESS_SESSION_INFORMATION = *mut PROCESS_SESSION_INFORMATION;
pub const PROCESS_HANDLE_EXCEPTIONS_ENABLED: ULONG = 0x00000001;
pub const PROCESS_HANDLE_RAISE_EXCEPTION_ON_INVALID_HANDLE_CLOSE_DISABLED: ULONG = 0x00000000;
pub const PROCESS_HANDLE_RAISE_EXCEPTION_ON_INVALID_HANDLE_CLOSE_ENABLED: ULONG = 0x00000001;
STRUCT!{struct PROCESS_HANDLE_TRACING_ENABLE {
    Flags: ULONG,
}}
pub type PPROCESS_HANDLE_TRACING_ENABLE = *mut PROCESS_HANDLE_TRACING_ENABLE;
pub const PROCESS_HANDLE_TRACING_MAX_SLOTS: ULONG = 0x20000;
STRUCT!{struct PROCESS_HANDLE_TRACING_ENABLE_EX {
    Flags: ULONG,
    TotalSlots: ULONG,
}}
pub type PPROCESS_HANDLE_TRACING_ENABLE_EX = *mut PROCESS_HANDLE_TRACING_ENABLE_EX;
pub const PROCESS_HANDLE_TRACING_MAX_STACKS: usize = 16;
pub const PROCESS_HANDLE_TRACE_TYPE_OPEN: ULONG = 1;
pub const PROCESS_HANDLE_TRACE_TYPE_CLOSE: ULONG = 2;
pub const PROCESS_HANDLE_TRACE_TYPE_BADREF: ULONG = 3;
STRUCT!{struct PROCESS_HANDLE_TRACING_ENTRY {
    Handle: HANDLE,
    ClientId: CLIENT_ID,
    Type: ULONG,
    Stacks: [PVOID; PROCESS_HANDLE_TRACING_MAX_STACKS],
}}
pub type PPROCESS_HANDLE_TRACING_ENTRY = *mut PROCESS_HANDLE_TRACING_ENTRY;
STRUCT!{struct PROCESS_HANDLE_TRACING_QUERY {
    Handle: HANDLE,
    TotalTraces: ULONG,
    HandleTrace: [PROCESS_HANDLE_TRACING_ENTRY; 1],
}}
pub type PPROCESS_HANDLE_TRACING_QUERY = *mut PROCESS_HANDLE_TRACING_QUERY;
STRUCT!{struct THREAD_TLS_INFORMATION {
    Flags: ULONG,
    NewTlsData: PVOID,
    OldTlsData: PVOID,
    ThreadId: HANDLE,
}}
pub type PTHREAD_TLS_INFORMATION = *mut THREAD_TLS_INFORMATION;
ENUM!{enum PROCESS_TLS_INFORMATION_TYPE {
    ProcessTlsReplaceIndex = 0,
    ProcessTlsReplaceVector = 1,
    MaxProcessTlsOperation = 2,
}}
pub type PPROCESS_TLS_INFORMATION_TYPE = *mut PROCESS_TLS_INFORMATION_TYPE;
STRUCT!{struct PROCESS_TLS_INFORMATION {
    Flags: ULONG,
    OperationType: ULONG,
    ThreadDataCount: ULONG,
    TlsIndex: ULONG,
    PreviousCount: ULONG,
    ThreadData: [THREAD_TLS_INFORMATION; 1],
}}
pub type PPROCESS_TLS_INFORMATION = *mut PROCESS_TLS_INFORMATION;
STRUCT!{struct PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION {
    Version: ULONG,
    Reserved: ULONG,
    Callback: PVOID,
}}
pub type PPROCESS_INSTRUMENTATION_CALLBACK_INFORMATION =
    *mut PROCESS_INSTRUMENTATION_CALLBACK_INFORMATION;
STRUCT!{struct PROCESS_STACK_ALLOCATION_INFORMATION {
    ReserveSize: SIZE_T,
    ZeroBits: SIZE_T,
    StackBase: PVOID,
}}
pub type PPROCESS_STACK_ALLOCATION_INFORMATION = *mut PROCESS_STACK_ALLOCATION_INFORMATION;
STRUCT!{struct PROCESS_STACK_ALLOCATION_INFORMATION_EX {
    PreferredNode: ULONG,
    Reserved0: ULONG,
    Reserved1: ULONG,
    Reserved2: ULONG,
    AllocInfo: PROCESS_STACK_ALLOCATION_INFORMATION,
}}
pub type PPROCESS_STACK_ALLOCATION_INFORMATION_EX = *mut PROCESS_STACK_ALLOCATION_INFORMATION_EX;
STRUCT!{struct PROCESS_AFFINITY_UPDATE_MODE {
    Flags: ULONG,
}}
BITFIELD!{PROCESS_AFFINITY_UPDATE_MODE Flags: ULONG [
    EnableAutoUpdate set_EnableAutoUpdate[0..1],
    Permanent set_Permanent[1..2],
    Reserved set_Reserved[2..32],
]}
pub type PPROCESS_AFFINITY_UPDATE_MODE = *mut PROCESS_AFFINITY_UPDATE_MODE;
STRUCT!{struct PROCESS_MEMORY_ALLOCATION_MODE {
    Flags: ULONG,
}}
BITFIELD!{PROCESS_MEMORY_ALLOCATION_MODE Flags: ULONG [
    TopDown set_TopDown[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PPROCESS_MEMORY_ALLOCATION_MODE = *mut PROCESS_MEMORY_ALLOCATION_MODE;
STRUCT!{struct PROCESS_HANDLE_INFORMATION {
    HandleCount: ULONG,
    HandleCountHighWatermark: ULONG,
}}
pub type PPROCESS_HANDLE_INFORMATION = *mut PROCESS_HANDLE_INFORMATION;
STRUCT!{struct PROCESS_CYCLE_TIME_INFORMATION {
    AccumulatedCycles: ULONGLONG,
    CurrentCycleCount: ULONGLONG,
}}
pub type PPROCESS_CYCLE_TIME_INFORMATION = *mut PROCESS_CYCLE_TIME_INFORMATION;
STRUCT!{struct PROCESS_WINDOW_INFORMATION {
    WindowFlags: ULONG,
    WindowTitleLength: USHORT,
    WindowTitle: [WCHAR; 1],
}}
pub type PPROCESS_WINDOW_INFORMATION = *mut PROCESS_WINDOW_INFORMATION;
STRUCT!{struct PROCESS_HANDLE_TABLE_ENTRY_INFO {
    HandleValue: HANDLE,
    HandleCount: ULONG_PTR,
    PointerCount: ULONG_PTR,
    GrantedAccess: ULONG,
    ObjectTypeIndex: ULONG,
    HandleAttributes: ULONG,
    Reserved: ULONG,
}}
pub type PPROCESS_HANDLE_TABLE_ENTRY_INFO = *mut PROCESS_HANDLE_TABLE_ENTRY_INFO;
STRUCT!{struct PROCESS_HANDLE_SNAPSHOT_INFORMATION {
    NumberOfHandles: ULONG_PTR,
    Reserved: ULONG_PTR,
    Handles: [PROCESS_HANDLE_TABLE_ENTRY_INFO; 1],
}}
pub type PPROCESS_HANDLE_SNAPSHOT_INFORMATION = *mut PROCESS_HANDLE_SNAPSHOT_INFORMATION;
UNION!{union PROCESS_MITIGATION_POLICY_INFORMATION_u {
    ASLRPolicy: PROCESS_MITIGATION_ASLR_POLICY,
    StrictHandleCheckPolicy: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY,
    SystemCallDisablePolicy: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY,
    ExtensionPointDisablePolicy: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY,
    DynamicCodePolicy: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY,
    ControlFlowGuardPolicy: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY,
    SignaturePolicy: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY,
    FontDisablePolicy: PROCESS_MITIGATION_FONT_DISABLE_POLICY,
    ImageLoadPolicy: PROCESS_MITIGATION_IMAGE_LOAD_POLICY,
    SystemCallFilterPolicy: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY,
    PayloadRestrictionPolicy: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY,
    ChildProcessPolicy: PROCESS_MITIGATION_CHILD_PROCESS_POLICY,
    // SideChannelIsolationPolicy: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY, //TODO
}}
STRUCT!{struct PROCESS_MITIGATION_POLICY_INFORMATION {
    Policy: PROCESS_MITIGATION_POLICY,
    u: PROCESS_MITIGATION_POLICY_INFORMATION_u,
}}
pub type PPROCESS_MITIGATION_POLICY_INFORMATION = *mut PROCESS_MITIGATION_POLICY_INFORMATION;
STRUCT!{struct PROCESS_KEEPALIVE_COUNT_INFORMATION {
    WakeCount: ULONG,
    NoWakeCount: ULONG,
}}
pub type PPROCESS_KEEPALIVE_COUNT_INFORMATION = *mut PROCESS_KEEPALIVE_COUNT_INFORMATION;
STRUCT!{struct PROCESS_REVOKE_FILE_HANDLES_INFORMATION {
    TargetDevicePath: UNICODE_STRING,
}}
pub type PPROCESS_REVOKE_FILE_HANDLES_INFORMATION = *mut PROCESS_REVOKE_FILE_HANDLES_INFORMATION;
ENUM!{enum PROCESS_WORKING_SET_OPERATION {
    ProcessWorkingSetSwap = 0,
    ProcessWorkingSetEmpty = 1,
    ProcessWorkingSetOperationMax = 2,
}}
STRUCT!{struct PROCESS_WORKING_SET_CONTROL {
    Version: ULONG,
    Operation: PROCESS_WORKING_SET_OPERATION,
    Flags: ULONG,
}}
pub type PPROCESS_WORKING_SET_CONTROL = *mut PROCESS_WORKING_SET_CONTROL;
ENUM!{enum PS_PROTECTED_TYPE {
    PsProtectedTypeNone = 0,
    PsProtectedTypeProtectedLight = 1,
    PsProtectedTypeProtected = 2,
    PsProtectedTypeMax = 3,
}}
ENUM!{enum PS_PROTECTED_SIGNER {
    PsProtectedSignerNone = 0,
    PsProtectedSignerAuthenticode = 1,
    PsProtectedSignerCodeGen = 2,
    PsProtectedSignerAntimalware = 3,
    PsProtectedSignerLsa = 4,
    PsProtectedSignerWindows = 5,
    PsProtectedSignerWinTcb = 6,
    PsProtectedSignerWinSystem = 7,
    PsProtectedSignerApp = 8,
    PsProtectedSignerMax = 9,
}}
pub const PS_PROTECTED_SIGNER_MASK: UCHAR = 0xFF;
pub const PS_PROTECTED_AUDIT_MASK: UCHAR = 0x08;
pub const PS_PROTECTED_TYPE_MASK: UCHAR = 0x07;
#[inline]
pub const fn PsProtectedValue(
    aSigner: PS_PROTECTED_SIGNER,
    aAudit: u8,
    aType: PS_PROTECTED_TYPE,
) -> UCHAR {
    (aSigner as u8 & PS_PROTECTED_SIGNER_MASK) << 4 | (aAudit & PS_PROTECTED_AUDIT_MASK) << 3
        | (aType as u8 & PS_PROTECTED_TYPE_MASK)
}
#[inline]
pub fn InitializePsProtection(
    aProtectionLevelPtr: &mut PS_PROTECTION,
    aSigner: PS_PROTECTED_SIGNER,
    aAudit: u8,
    aType: PS_PROTECTED_TYPE,
) {
    aProtectionLevelPtr.set_Signer(aSigner as u8);
    aProtectionLevelPtr.set_Audit(aAudit);
    aProtectionLevelPtr.set_Type(aType as u8);
}
STRUCT!{struct PS_PROTECTION {
    Level: UCHAR,
}}
pub type PPS_PROTECTION = *mut PS_PROTECTION;
BITFIELD!{PS_PROTECTION Level: UCHAR [
    Type set_Type[0..3],
    Audit set_Audit[3..4],
    Signer set_Signer[4..8],
]}
STRUCT!{struct PROCESS_FAULT_INFORMATION {
    FaultFlags: ULONG,
    AdditionalInfo: ULONG,
}}
pub type PPROCESS_FAULT_INFORMATION = *mut PROCESS_FAULT_INFORMATION;
STRUCT!{struct PROCESS_TELEMETRY_ID_INFORMATION {
    HeaderSize: ULONG,
    ProcessId: ULONG,
    ProcessStartKey: ULONGLONG,
    CreateTime: ULONGLONG,
    CreateInterruptTime: ULONGLONG,
    CreateUnbiasedInterruptTime: ULONGLONG,
    ProcessSequenceNumber: ULONGLONG,
    SessionCreateTime: ULONGLONG,
    SessionId: ULONG,
    BootId: ULONG,
    ImageChecksum: ULONG,
    ImageTimeDateStamp: ULONG,
    UserSidOffset: ULONG,
    ImagePathOffset: ULONG,
    PackageNameOffset: ULONG,
    RelativeAppNameOffset: ULONG,
    CommandLineOffset: ULONG,
}}
pub type PPROCESS_TELEMETRY_ID_INFORMATION = *mut PROCESS_TELEMETRY_ID_INFORMATION;
STRUCT!{struct PROCESS_COMMIT_RELEASE_INFORMATION {
    Version: ULONG,
    s: ULONG,
    CommitDebt: SIZE_T,
    CommittedMemResetSize: SIZE_T,
    RepurposedMemResetSize: SIZE_T,
}}
BITFIELD!{PROCESS_COMMIT_RELEASE_INFORMATION s: ULONG [
    Eligible set_Eligible[0..1],
    ReleaseRepurposedMemResetCommit set_ReleaseRepurposedMemResetCommit[1..2],
    ForceReleaseMemResetCommit set_ForceReleaseMemResetCommit[2..3],
    Spare set_Spare[3..32],
]}
pub type PPROCESS_COMMIT_RELEASE_INFORMATION = *mut PROCESS_COMMIT_RELEASE_INFORMATION;
STRUCT!{struct PROCESS_JOB_MEMORY_INFO {
    SharedCommitUsage: ULONGLONG,
    PrivateCommitUsage: ULONGLONG,
    PeakPrivateCommitUsage: ULONGLONG,
    PrivateCommitLimit: ULONGLONG,
    TotalCommitLimit: ULONGLONG,
}}
pub type PPROCESS_JOB_MEMORY_INFO = *mut PROCESS_JOB_MEMORY_INFO;
STRUCT!{struct PROCESS_CHILD_PROCESS_INFORMATION {
    ProhibitChildProcesses: BOOLEAN,
    AlwaysAllowSecureChildProcess: BOOLEAN,
    AuditProhibitChildProcesses: BOOLEAN,
}}
pub type PPROCESS_CHILD_PROCESS_INFORMATION = *mut PROCESS_CHILD_PROCESS_INFORMATION;
STRUCT!{struct PROCESS_WAKE_INFORMATION {
    NotificationChannel: ULONGLONG,
    WakeCounters: [ULONG; 7],
    WakeFilter: *mut JOBOBJECT_WAKE_FILTER,
}}
pub type PPROCESS_WAKE_INFORMATION = *mut PROCESS_WAKE_INFORMATION;
STRUCT!{struct PROCESS_ENERGY_TRACKING_STATE {
    StateUpdateMask: ULONG,
    StateDesiredValue: ULONG,
    StateSequence: ULONG,
    UpdateTag: ULONG,
    Tag: [WCHAR; 64],
}}
pub type PPROCESS_ENERGY_TRACKING_STATE = *mut PROCESS_ENERGY_TRACKING_STATE;
BITFIELD!{PROCESS_ENERGY_TRACKING_STATE UpdateTag: ULONG [
    UpdateTag set_UpdateTag[0..1],
]}
STRUCT!{struct MANAGE_WRITES_TO_EXECUTABLE_MEMORY {
    BitFields: ULONG,
}}
BITFIELD!{MANAGE_WRITES_TO_EXECUTABLE_MEMORY BitFields: ULONG [
    Machine set_Machine[0..16],
    KernelMode set_KernelMode[16..17],
    UserMode set_UserMode[17..18],
    Native set_Native[18..19],
    Process set_Process[19..20],
    ReservedZero0 set_ReservedZero0[20..32],
]}
pub type PMANAGE_WRITES_TO_EXECUTABLE_MEMORY = *mut MANAGE_WRITES_TO_EXECUTABLE_MEMORY;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_READVM: UCHAR = 1;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_WRITEVM: UCHAR = 2;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_READVM_V: UCHAR = 1;
pub const PROCESS_READWRITEVM_LOGGING_ENABLE_WRITEVM_V: UCHAR = 2;
STRUCT!{struct PROCESS_READWRITEVM_LOGGING_INFORMATION {
    Flags: UCHAR,
}}
BITFIELD!{PROCESS_READWRITEVM_LOGGING_INFORMATION Flags: UCHAR [
    EnableReadVmLogging set_EnableReadVmLogging[0..1],
    EnableWriteVmLogging set_EnableWriteVmLogging[1..2],
    Unused set_Unused[2..8],
]}
UNION!{union PROCESS_UPTIME_INFORMATION_u {
    HangCount: ULONG,
    GhostCount: ULONG,
    Crashed: ULONG,
    Terminated: ULONG,
}}
pub type PPROCESS_READWRITEVM_LOGGING_INFORMATION = *mut PROCESS_READWRITEVM_LOGGING_INFORMATION;
STRUCT!{struct PROCESS_UPTIME_INFORMATION {
    QueryInterruptTime: ULONGLONG,
    QueryUnbiasedTime: ULONGLONG,
    EndInterruptTime: ULONGLONG,
    TimeSinceCreation: ULONGLONG,
    Uptime: ULONGLONG,
    SuspendedTime: ULONGLONG,
    u: PROCESS_UPTIME_INFORMATION_u,
}}
pub type PPROCESS_UPTIME_INFORMATION = *mut PROCESS_UPTIME_INFORMATION;
STRUCT!{struct PROCESS_SYSTEM_RESOURCE_MANAGEMENT {
    Flags: ULONG,
}}
pub type PPROCESS_SYSTEM_RESOURCE_MANAGEMENT = *mut PROCESS_SYSTEM_RESOURCE_MANAGEMENT;
BITFIELD!{PROCESS_SYSTEM_RESOURCE_MANAGEMENT Flags: ULONG [
    Foreground set_Foreground[0..1],
    Reserved set_Reserved[1..32],
]}
STRUCT!{struct PROCESS_SECURITY_DOMAIN_INFORMATION {
    SecurityDomain: ULONGLONG,
}}
pub type PPROCESS_SECURITY_DOMAIN_INFORMATION = *mut PROCESS_SECURITY_DOMAIN_INFORMATION;
STRUCT!{struct PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION {
    ProcessHandle: HANDLE,
}}
pub type PPROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION =
    *mut PROCESS_COMBINE_SECURITY_DOMAINS_INFORMATION;
STRUCT!{struct PROCESS_LOGGING_INFORMATION {
    Flags: ULONG,
    BitFields: ULONG,
}}
BITFIELD!{PROCESS_LOGGING_INFORMATION BitFields: ULONG [
    EnableReadVmLogging set_EnableReadVmLogging[0..1],
    EnableWriteVmLogging set_EnableWriteVmLogging[1..2],
    EnableProcessSuspendResumeLogging set_EnableProcessSuspendResumeLogging[2..3],
    EnableThreadSuspendResumeLogging set_EnableThreadSuspendResumeLogging[3..4],
    Reserved set_Reserved[4..32],
]}
pub type PPROCESS_LOGGING_INFORMATION = *mut PROCESS_LOGGING_INFORMATION;
STRUCT!{struct PROCESS_LEAP_SECOND_INFORMATION {
    Flags: ULONG,
    Reserved: ULONG,
}}
pub type PPROCESS_LEAP_SECOND_INFORMATION = *mut PROCESS_LEAP_SECOND_INFORMATION;
STRUCT!{struct THREAD_BASIC_INFORMATION {
    ExitStatus: NTSTATUS,
    TebBaseAddress: PTEB,
    ClientId: CLIENT_ID,
    AffinityMask: ULONG_PTR,
    Priority: KPRIORITY,
    BasePriority: LONG,
}}
pub type PTHREAD_BASIC_INFORMATION = *mut THREAD_BASIC_INFORMATION;
STRUCT!{struct THREAD_LAST_SYSCALL_INFORMATION {
    FirstArgument: PVOID,
    SystemCallNumber: USHORT,
    Pad: [USHORT; 1],
    WaitTime: ULONG64,
}}
pub type PTHREAD_LAST_SYSCALL_INFORMATION = *mut THREAD_LAST_SYSCALL_INFORMATION;
STRUCT!{struct THREAD_CYCLE_TIME_INFORMATION {
    AccumulatedCycles: ULONGLONG,
    CurrentCycleCount: ULONGLONG,
}}
pub type PTHREAD_CYCLE_TIME_INFORMATION = *mut THREAD_CYCLE_TIME_INFORMATION;
STRUCT!{struct THREAD_TEB_INFORMATION {
    TebInformation: PVOID,
    TebOffset: ULONG,
    BytesToRead: ULONG,
}}
pub type PTHREAD_TEB_INFORMATION = *mut THREAD_TEB_INFORMATION;
STRUCT!{struct COUNTER_READING {
    Type: HARDWARE_COUNTER_TYPE,
    Index: ULONG,
    Start: ULONG64,
    Total: ULONG64,
}}
pub type PCOUNTER_READING = *mut COUNTER_READING;
STRUCT!{struct THREAD_PERFORMANCE_DATA {
    Size: USHORT,
    Version: USHORT,
    ProcessorNumber: PROCESSOR_NUMBER,
    ContextSwitches: ULONG,
    HwCountersCount: ULONG,
    UpdateCount: ULONG64,
    WaitReasonBitMap: ULONG64,
    HardwareCounters: ULONG64,
    CycleTime: COUNTER_READING,
    HwCounters: [COUNTER_READING; MAX_HW_COUNTERS],
}}
pub type PTHREAD_PERFORMANCE_DATA = *mut THREAD_PERFORMANCE_DATA;
STRUCT!{struct THREAD_PROFILING_INFORMATION {
    HardwareCounters: ULONG64,
    Flags: ULONG,
    Enable: ULONG,
    PerformanceData: PTHREAD_PERFORMANCE_DATA,
}}
pub type PTHREAD_PROFILING_INFORMATION = *mut THREAD_PROFILING_INFORMATION;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
STRUCT!{#[repr(align(16))] struct RTL_UMS_CONTEXT {
    Link: SINGLE_LIST_ENTRY,
    __padding: u64,
    Context: CONTEXT,
    Teb: PVOID,
    UserContext: PVOID,
    ScheduledThread: ULONG,
    Suspended: ULONG,
    VolatileContext: ULONG,
    Terminated: ULONG,
    DebugActive: ULONG,
    RunningOnSelfThread: ULONG,
    DenyRunningOnSelfThread: ULONG,
    Flags: LONG,
    KernelUpdateLock: ULONG64,
    PrimaryClientID: ULONG64,
    ContextLock: ULONG64,
    PrimaryUmsContext: *mut RTL_UMS_CONTEXT,
    SwitchCount: ULONG,
    KernelYieldCount: ULONG,
    MixedYieldCount: ULONG,
    YieldCount: ULONG,
}}
#[cfg(target_arch = "x86")]
STRUCT!{struct RTL_UMS_CONTEXT {
    Link: SINGLE_LIST_ENTRY,
    Context: CONTEXT,
    Teb: PVOID,
    UserContext: PVOID,
    ScheduledThread: ULONG,
    Suspended: ULONG,
    VolatileContext: ULONG,
    Terminated: ULONG,
    DebugActive: ULONG,
    RunningOnSelfThread: ULONG,
    DenyRunningOnSelfThread: ULONG,
    Flags: LONG,
    KernelUpdateLock: ULONG64,
    PrimaryClientID: ULONG64,
    ContextLock: ULONG64,
    PrimaryUmsContext: *mut RTL_UMS_CONTEXT,
    SwitchCount: ULONG,
    KernelYieldCount: ULONG,
    MixedYieldCount: ULONG,
    YieldCount: ULONG,
    __padding: u32,
}}
pub type PRTL_UMS_CONTEXT = *mut RTL_UMS_CONTEXT;
ENUM!{enum THREAD_UMS_INFORMATION_COMMAND {
    UmsInformationCommandInvalid = 0,
    UmsInformationCommandAttach = 1,
    UmsInformationCommandDetach = 2,
    UmsInformationCommandQuery = 3,
}}
STRUCT!{struct RTL_UMS_COMPLETION_LIST {
    ThreadListHead: PSINGLE_LIST_ENTRY,
    CompletionEvent: PVOID,
    CompletionFlags: ULONG,
    InternalListHead: SINGLE_LIST_ENTRY,
}}
pub type PRTL_UMS_COMPLETION_LIST = *mut RTL_UMS_COMPLETION_LIST;
STRUCT!{struct THREAD_UMS_INFORMATION {
    Command: THREAD_UMS_INFORMATION_COMMAND,
    CompletionList: PRTL_UMS_COMPLETION_LIST,
    UmsContext: PRTL_UMS_CONTEXT,
    Flags: ULONG,
}}
BITFIELD!{THREAD_UMS_INFORMATION Flags: ULONG [
    IsUmsSchedulerThread set_IsUmsSchedulerThread[0..1],
    IsUmsWorkerThread set_IsUmsWorkerThread[1..2],
    SpareBits set_SpareBits[2..32],
]}
pub type PTHREAD_UMS_INFORMATION = *mut THREAD_UMS_INFORMATION;
STRUCT!{struct THREAD_NAME_INFORMATION {
    ThreadName: UNICODE_STRING,
}}
pub type PTHREAD_NAME_INFORMATION = *mut THREAD_NAME_INFORMATION;
ENUM!{enum SUBSYSTEM_INFORMATION_TYPE {
    SubsystemInformationTypeWin32 = 0,
    SubsystemInformationTypeWSL = 1,
    MaxSubsystemInformationType = 2,
}}
ENUM!{enum THREAD_WORKLOAD_CLASS {
    ThreadWorkloadClassDefault = 0,
    ThreadWorkloadClassGraphics = 1,
    MaxThreadWorkloadClass = 2,
}}
EXTERN!{extern "system" {
    fn NtCreateProcess(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        InheritObjectTable: BOOLEAN,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        ExceptionPort: HANDLE,
    ) -> NTSTATUS;
}}
pub const PROCESS_CREATE_FLAGS_BREAKAWAY: ULONG = 0x00000001;
pub const PROCESS_CREATE_FLAGS_NO_DEBUG_INHERIT: ULONG = 0x00000002;
pub const PROCESS_CREATE_FLAGS_INHERIT_HANDLES: ULONG = 0x00000004;
pub const PROCESS_CREATE_FLAGS_OVERRIDE_ADDRESS_SPACE: ULONG = 0x00000008;
pub const PROCESS_CREATE_FLAGS_LARGE_PAGES: ULONG = 0x00000010;
EXTERN!{extern "system" {
    fn NtCreateProcessEx(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        Flags: ULONG,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        ExceptionPort: HANDLE,
        JobMemberLevel: ULONG,
    ) -> NTSTATUS;
    fn NtOpenProcess(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn NtTerminateProcess(
        ProcessHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn NtSuspendProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtResumeProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
}}
pub const NtCurrentProcess: HANDLE = -1isize as *mut c_void;
pub const ZwCurrentProcess: HANDLE = NtCurrentProcess;
pub const NtCurrentThread: HANDLE = -2isize as *mut c_void;
pub const ZwCurrentThread: HANDLE = NtCurrentThread;
pub const NtCurrentSession: HANDLE = -3isize as *mut c_void;
pub const ZwCurrentSession: HANDLE = NtCurrentSession;
#[inline] #[cfg(not(target_arch = "aarch64"))]
pub unsafe fn NtCurrentPeb() -> PPEB {
    (*NtCurrentTeb()).ProcessEnvironmentBlock
}
pub const NtCurrentProcessToken: HANDLE = -4isize as *mut c_void;
pub const NtCurrentThreadToken: HANDLE = -5isize as *mut c_void;
pub const NtCurrentEffectiveToken: HANDLE = -6isize as *mut c_void;
pub const NtCurrentSilo: HANDLE = -1isize as *mut c_void;
#[inline] #[cfg(not(target_arch = "aarch64"))]
pub unsafe fn NtCurrentProcessId() -> HANDLE {
    (*NtCurrentTeb()).ClientId.UniqueProcess
}
#[inline] #[cfg(not(target_arch = "aarch64"))]
pub unsafe fn NtCurrentThreadId() -> HANDLE {
    (*NtCurrentTeb()).ClientId.UniqueThread
}
EXTERN!{extern "system" {
    fn NtQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewProcessHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtGetNextThread(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewThreadHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtQueryPortInformationProcess() -> NTSTATUS;
    fn NtCreateThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ThreadContext: PCONTEXT,
        InitialTeb: PINITIAL_TEB,
        CreateSuspended: BOOLEAN,
    ) -> NTSTATUS;
    fn NtOpenThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn NtTerminateThread(
        ThreadHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn NtSuspendThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;
    fn NtResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;
    fn NtGetCurrentProcessorNumber() -> ULONG;
    fn NtGetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;
    fn NtSetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;
    fn NtQueryInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtAlertThread(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtAlertResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;
    fn NtTestAlert() -> NTSTATUS;
    fn NtImpersonateThread(
        ServerThreadHandle: HANDLE,
        ClientThreadHandle: HANDLE,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
    ) -> NTSTATUS;
    fn NtRegisterThreadTerminatePort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetLdtEntries(
        Selector0: ULONG,
        Entry0Low: ULONG,
        Entry0Hi: ULONG,
        Selector1: ULONG,
        Entry1Low: ULONG,
        Entry1Hi: ULONG,
    ) -> NTSTATUS;
}}
FN!{cdecl PPS_APC_ROUTINE(
    ApcArgument1: PVOID,
    ApcArgument2: PVOID,
    ApcArgument3: PVOID,
) -> ()}
EXTERN!{extern "system" {
    fn NtQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;
}}
pub const APC_FORCE_THREAD_SIGNAL: HANDLE = 1 as *mut c_void;
EXTERN!{extern "system" {
    fn NtQueueApcThreadEx(
        ThreadHandle: HANDLE,
        UserApcReserveHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;
    fn NtAlertThreadByThreadId(
        ThreadId: HANDLE,
    ) -> NTSTATUS;
    fn NtWaitForAlertByThreadId(
        Address: PVOID,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
}}
pub const PS_ATTRIBUTE_NUMBER_MASK: u32 = 0x0000ffff;
pub const PS_ATTRIBUTE_THREAD: u32 = 0x00010000;
pub const PS_ATTRIBUTE_INPUT: u32 = 0x00020000;
pub const PS_ATTRIBUTE_ADDITIVE: u32 = 0x00040000;
ENUM!{enum PS_ATTRIBUTE_NUM {
    PsAttributeParentProcess = 0,
    PsAttributeDebugPort = 1,
    PsAttributeToken = 2,
    PsAttributeClientId = 3,
    PsAttributeTebAddress = 4,
    PsAttributeImageName = 5,
    PsAttributeImageInfo = 6,
    PsAttributeMemoryReserve = 7,
    PsAttributePriorityClass = 8,
    PsAttributeErrorMode = 9,
    PsAttributeStdHandleInfo = 10,
    PsAttributeHandleList = 11,
    PsAttributeGroupAffinity = 12,
    PsAttributePreferredNode = 13,
    PsAttributeIdealProcessor = 14,
    PsAttributeUmsThread = 15,
    PsAttributeMitigationOptions = 16,
    PsAttributeProtectionLevel = 17,
    PsAttributeSecureProcess = 18,
    PsAttributeJobList = 19,
    PsAttributeChildProcessPolicy = 20,
    PsAttributeAllApplicationPackagesPolicy = 21,
    PsAttributeWin32kFilter = 22,
    PsAttributeSafeOpenPromptOriginClaim = 23,
    PsAttributeBnoIsolation = 24,
    PsAttributeDesktopAppPolicy = 25,
    PsAttributeChpe = 26,
    PsAttributeMax = 27,
}}
#[inline]
pub const fn PsAttributeValue(
    mut Number: PS_ATTRIBUTE_NUM,
    Thread: bool,
    Input: bool,
    Additive: bool,
) -> ULONG_PTR {
    Number &= PS_ATTRIBUTE_NUMBER_MASK;
    if Thread {
        Number |= PS_ATTRIBUTE_THREAD;
    }
    if Input {
        Number |= PS_ATTRIBUTE_INPUT;
    }
    if Additive {
        Number |= PS_ATTRIBUTE_ADDITIVE;
    }
    Number as _
}
pub const PS_ATTRIBUTE_PARENT_PROCESS: ULONG_PTR = 0x00060000;
pub const PS_ATTRIBUTE_DEBUG_PORT: ULONG_PTR = 0x00060001;
pub const PS_ATTRIBUTE_TOKEN: ULONG_PTR = 0x00060002;
pub const PS_ATTRIBUTE_CLIENT_ID: ULONG_PTR = 0x00010003;
pub const PS_ATTRIBUTE_TEB_ADDRESS: ULONG_PTR = 0x00010004;
pub const PS_ATTRIBUTE_IMAGE_NAME: ULONG_PTR = 0x00020005;
pub const PS_ATTRIBUTE_IMAGE_INFO: ULONG_PTR = 0x00000006;
pub const PS_ATTRIBUTE_MEMORY_RESERVE: ULONG_PTR = 0x00020007;
pub const PS_ATTRIBUTE_PRIORITY_CLASS: ULONG_PTR = 0x00020008;
pub const PS_ATTRIBUTE_ERROR_MODE: ULONG_PTR = 0x00020009;
pub const PS_ATTRIBUTE_STD_HANDLE_INFO: ULONG_PTR = 0x0002000a;
pub const PS_ATTRIBUTE_HANDLE_LIST: ULONG_PTR = 0x0002000b;
pub const PS_ATTRIBUTE_GROUP_AFFINITY: ULONG_PTR = 0x0003000c;
pub const PS_ATTRIBUTE_PREFERRED_NODE: ULONG_PTR = 0x0002000d;
pub const PS_ATTRIBUTE_IDEAL_PROCESSOR: ULONG_PTR = 0x0003000e;
pub const PS_ATTRIBUTE_UMS_THREAD: ULONG_PTR = 0x0003000f;
pub const PS_ATTRIBUTE_MITIGATION_OPTIONS: ULONG_PTR = 0x00060010;
pub const PS_ATTRIBUTE_PROTECTION_LEVEL: ULONG_PTR = 0x00060011;
pub const PS_ATTRIBUTE_SECURE_PROCESS: ULONG_PTR = 0x00020012;
pub const PS_ATTRIBUTE_JOB_LIST: ULONG_PTR = 0x00020013;
pub const PS_ATTRIBUTE_CHILD_PROCESS_POLICY: ULONG_PTR = 0x00020014;
pub const PS_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: ULONG_PTR = 0x00020015;
pub const PS_ATTRIBUTE_WIN32K_FILTER: ULONG_PTR = 0x00020016;
pub const PS_ATTRIBUTE_SAFE_OPEN_PROMPT_ORIGIN_CLAIM: ULONG_PTR = 0x00020017;
pub const PS_ATTRIBUTE_BNO_ISOLATION: ULONG_PTR = 0x00020018;
pub const PS_ATTRIBUTE_DESKTOP_APP_POLICY: ULONG_PTR = 0x00020019;
UNION!{union PS_ATTRIBUTE_u {
    Value: ULONG_PTR,
    ValuePtr: PVOID,
}}
STRUCT!{struct PS_ATTRIBUTE {
    Attribute: ULONG_PTR,
    Size: SIZE_T,
    u: PS_ATTRIBUTE_u,
    ReturnLength: PSIZE_T,
}}
pub type PPS_ATTRIBUTE = *mut PS_ATTRIBUTE;
STRUCT!{struct PS_ATTRIBUTE_LIST {
    TotalLength: SIZE_T,
    Attributes: [PS_ATTRIBUTE; 1],
}}
pub type PPS_ATTRIBUTE_LIST = *mut PS_ATTRIBUTE_LIST;
STRUCT!{struct PS_MEMORY_RESERVE {
    ReserveAddress: PVOID,
    ReserveSize: SIZE_T,
}}
pub type PPS_MEMORY_RESERVE = *mut PS_MEMORY_RESERVE;
ENUM!{enum PS_STD_HANDLE_STATE {
    PsNeverDuplicate = 0,
    PsRequestDuplicate = 1,
    PsAlwaysDuplicate = 2,
    PsMaxStdHandleStates = 3,
}}
pub const PS_STD_INPUT_HANDLE: u32 = 0x1;
pub const PS_STD_OUTPUT_HANDLE: u32 = 0x2;
pub const PS_STD_ERROR_HANDLE: u32 = 0x4;
STRUCT!{struct PS_STD_HANDLE_INFO {
    Flags: ULONG,
    StdHandleSubsystemType: ULONG,
}}
pub type PPS_STD_HANDLE_INFO = *mut PS_STD_HANDLE_INFO;
BITFIELD!{PS_STD_HANDLE_INFO Flags: ULONG [
    StdHandleState set_StdHandleState[0..2],
    PseudoHandleMask set_PseudoHandleMask[2..5],
]}
STRUCT!{struct PS_BNO_ISOLATION_PARAMETERS {
    IsolationPrefix: UNICODE_STRING,
    HandleCount: ULONG,
    Handles: *mut PVOID,
    IsolationEnabled: BOOLEAN,
}}
pub type PPS_BNO_ISOLATION_PARAMETERS = *mut PS_BNO_ISOLATION_PARAMETERS;
ENUM!{enum PS_MITIGATION_OPTION {
    PS_MITIGATION_OPTION_NX = 0,
    PS_MITIGATION_OPTION_SEHOP = 1,
    PS_MITIGATION_OPTION_FORCE_RELOCATE_IMAGES = 2,
    PS_MITIGATION_OPTION_HEAP_TERMINATE = 3,
    PS_MITIGATION_OPTION_BOTTOM_UP_ASLR = 4,
    PS_MITIGATION_OPTION_HIGH_ENTROPY_ASLR = 5,
    PS_MITIGATION_OPTION_STRICT_HANDLE_CHECKS = 6,
    PS_MITIGATION_OPTION_WIN32K_SYSTEM_CALL_DISABLE = 7,
    PS_MITIGATION_OPTION_EXTENSION_POINT_DISABLE = 8,
    PS_MITIGATION_OPTION_PROHIBIT_DYNAMIC_CODE = 9,
    PS_MITIGATION_OPTION_CONTROL_FLOW_GUARD = 10,
    PS_MITIGATION_OPTION_BLOCK_NON_MICROSOFT_BINARIES = 11,
    PS_MITIGATION_OPTION_FONT_DISABLE = 12,
    PS_MITIGATION_OPTION_IMAGE_LOAD_NO_REMOTE = 13,
    PS_MITIGATION_OPTION_IMAGE_LOAD_NO_LOW_LABEL = 14,
    PS_MITIGATION_OPTION_IMAGE_LOAD_PREFER_SYSTEM32 = 15,
    PS_MITIGATION_OPTION_RETURN_FLOW_GUARD = 16,
    PS_MITIGATION_OPTION_LOADER_INTEGRITY_CONTINUITY = 17,
    PS_MITIGATION_OPTION_STRICT_CONTROL_FLOW_GUARD = 18,
    PS_MITIGATION_OPTION_RESTRICT_SET_THREAD_CONTEXT = 19,
    PS_MITIGATION_OPTION_ROP_STACKPIVOT = 20,
    PS_MITIGATION_OPTION_ROP_CALLER_CHECK = 21,
    PS_MITIGATION_OPTION_ROP_SIMEXEC = 22,
    PS_MITIGATION_OPTION_EXPORT_ADDRESS_FILTER = 23,
    PS_MITIGATION_OPTION_EXPORT_ADDRESS_FILTER_PLUS = 24,
    PS_MITIGATION_OPTION_RESTRICT_CHILD_PROCESS_CREATION = 25,
    PS_MITIGATION_OPTION_IMPORT_ADDRESS_FILTER = 26,
    PS_MITIGATION_OPTION_MODULE_TAMPERING_PROTECTION = 27,
    PS_MITIGATION_OPTION_RESTRICT_INDIRECT_BRANCH_PREDICTION = 28,
    PS_MITIGATION_OPTION_SPECULATIVE_STORE_BYPASS_DISABLE = 29,
    PS_MITIGATION_OPTION_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY = 30,
    PS_MITIGATION_OPTION_CET_SHADOW_STACKS = 31,
}}
ENUM!{enum PS_CREATE_STATE {
    PsCreateInitialState = 0,
    PsCreateFailOnFileOpen = 1,
    PsCreateFailOnSectionCreate = 2,
    PsCreateFailExeFormat = 3,
    PsCreateFailMachineMismatch = 4,
    PsCreateFailExeName = 5,
    PsCreateSuccess = 6,
    PsCreateMaximumStates = 7,
}}
STRUCT!{struct PS_CREATE_INFO_u_InitState {
    InitFlags: ULONG,
    AdditionalFileAccess: ACCESS_MASK,
}}
BITFIELD!{PS_CREATE_INFO_u_InitState InitFlags: ULONG [
    WriteOutputOnExit set_WriteOutputOnExit[0..1],
    DetectManifest set_DetectManifest[1..2],
    IFEOSkipDebugger set_IFEOSkipDebugger[2..3],
    IFEODoNotPropagateKeyState set_IFEODoNotPropagateKeyState[3..4],
    SpareBits1 set_SpareBits1[4..8],
    SpareBits2 set_SpareBits2[8..16],
    ProhibitedImageCharacteristics set_ProhibitedImageCharacteristics[16..32],
]}
STRUCT!{struct PS_CREATE_INFO_u_SuccessState {
    OutputFlags: ULONG,
    FileHandle: HANDLE,
    SectionHandle: HANDLE,
    UserProcessParametersNative: ULONGLONG,
    UserProcessParametersWow64: ULONG,
    CurrentParameterFlags: ULONG,
    PebAddressNative: ULONGLONG,
    PebAddressWow64: ULONG,
    ManifestAddress: ULONGLONG,
    ManifestSize: ULONG,
}}
BITFIELD!{PS_CREATE_INFO_u_SuccessState OutputFlags: ULONG [
    ProtectedProcess set_ProtectedProcess[0..1],
    AddressSpaceOverride set_AddressSpaceOverride[1..2],
    DevOverrideEnabled set_DevOverrideEnabled[2..3],
    ManifestDetected set_ManifestDetected[3..4],
    ProtectedProcessLight set_ProtectedProcessLight[4..5],
    SpareBits1 set_SpareBits1[5..8],
    SpareBits2 set_SpareBits2[8..16],
    SpareBits3 set_SpareBits3[16..32],
]}
UNION!{union PS_CREATE_INFO_u {
    InitState: PS_CREATE_INFO_u_InitState,
    FileHandle: HANDLE,
    DllCharacteristics: USHORT,
    IFEOKey: HANDLE,
    SuccessState: PS_CREATE_INFO_u_SuccessState,
}}
STRUCT!{struct PS_CREATE_INFO {
    Size: SIZE_T,
    State: PS_CREATE_STATE,
    u: PS_CREATE_INFO_u,
}}
pub type PPS_CREATE_INFO = *mut PS_CREATE_INFO;
pub const PROCESS_CREATE_FLAGS_LARGE_PAGE_SYSTEM_DLL: ULONG = 0x00000020;
pub const PROCESS_CREATE_FLAGS_PROTECTED_PROCESS: ULONG = 0x00000040;
pub const PROCESS_CREATE_FLAGS_CREATE_SESSION: ULONG = 0x00000080;
pub const PROCESS_CREATE_FLAGS_INHERIT_FROM_PARENT: ULONG = 0x00000100;
pub const PROCESS_CREATE_FLAGS_SUSPENDED: ULONG = 0x00000200;
pub const PROCESS_CREATE_FLAGS_EXTENDED_UNKNOWN: ULONG = 0x00000400;
EXTERN!{extern "system" {
    fn NtCreateUserProcess(
        ProcessHandle: PHANDLE,
        ThreadHandle: PHANDLE,
        ProcessDesiredAccess: ACCESS_MASK,
        ThreadDesiredAccess: ACCESS_MASK,
        ProcessObjectAttributes: POBJECT_ATTRIBUTES,
        ThreadObjectAttributes: POBJECT_ATTRIBUTES,
        ProcessFlags: ULONG,
        ThreadFlags: ULONG,
        ProcessParameters: PVOID,
        CreateInfo: PPS_CREATE_INFO,
        AttributeList: PPS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;
}}
pub const THREAD_CREATE_FLAGS_CREATE_SUSPENDED: ULONG = 0x00000001;
pub const THREAD_CREATE_FLAGS_SKIP_THREAD_ATTACH: ULONG = 0x00000002;
pub const THREAD_CREATE_FLAGS_HIDE_FROM_DEBUGGER: ULONG = 0x00000004;
pub const THREAD_CREATE_FLAGS_HAS_SECURITY_DESCRIPTOR: ULONG = 0x00000010;
pub const THREAD_CREATE_FLAGS_ACCESS_CHECK_IN_TARGET: ULONG = 0x00000020;
pub const THREAD_CREATE_FLAGS_INITIAL_THREAD: ULONG = 0x00000080;
EXTERN!{extern "system" {
    fn NtCreateThreadEx(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        StartRoutine: PVOID,
        Argument: PVOID,
        CreateFlags: ULONG,
        ZeroBits: SIZE_T,
        StackSize: SIZE_T,
        MaximumStackSize: SIZE_T,
        AttributeList: PPS_ATTRIBUTE_LIST,
    ) -> NTSTATUS;
}}
STRUCT!{struct JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION {
    BasicInfo: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
    IoInfo: IO_COUNTERS,
    DiskIoInfo: PROCESS_DISK_COUNTERS,
    ContextSwitches: ULONG64,
    TotalCycleTime: LARGE_INTEGER,
    ReadyTime: ULONG64,
    EnergyValues: PROCESS_ENERGY_VALUES,
}}
pub type PJOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION =
    *mut JOBOBJECT_EXTENDED_ACCOUNTING_INFORMATION;
STRUCT!{struct JOBOBJECT_WAKE_INFORMATION {
    NotificationChannel: HANDLE,
    WakeCounters: [ULONG64; 7],
}}
pub type PJOBOBJECT_WAKE_INFORMATION = *mut JOBOBJECT_WAKE_INFORMATION;
STRUCT!{struct JOBOBJECT_WAKE_INFORMATION_V1 {
    NotificationChannel: HANDLE,
    WakeCounters: [ULONG64; 4],
}}
pub type PJOBOBJECT_WAKE_INFORMATION_V1 = *mut JOBOBJECT_WAKE_INFORMATION_V1;
STRUCT!{struct JOBOBJECT_INTERFERENCE_INFORMATION {
    Count: ULONG64,
}}
pub type PJOBOBJECT_INTERFERENCE_INFORMATION = *mut JOBOBJECT_INTERFERENCE_INFORMATION;
STRUCT!{struct JOBOBJECT_WAKE_FILTER {
    HighEdgeFilter: ULONG,
    LowEdgeFilter: ULONG,
}}
pub type PJOBOBJECT_WAKE_FILTER = *mut JOBOBJECT_WAKE_FILTER;
STRUCT!{struct JOBOBJECT_FREEZE_INFORMATION {
    Flags: ULONG,
    Freeze: BOOLEAN,
    Swap: BOOLEAN,
    Reserved0: [UCHAR; 2],
    WakeFilter: JOBOBJECT_WAKE_FILTER,
}}
pub type PJOBOBJECT_FREEZE_INFORMATION = *mut JOBOBJECT_FREEZE_INFORMATION;
BITFIELD!{JOBOBJECT_FREEZE_INFORMATION Flags: ULONG [
    FreezeOperation set_FreezeOperation[0..1],
    FilterOperation set_FilterOperation[1..2],
    SwapOperation set_SwapOperation[2..3],
    Reserved set_Reserved[3..32],
]}
STRUCT!{struct JOBOBJECT_MEMORY_USAGE_INFORMATION {
    JobMemory: ULONG64,
    PeakJobMemoryUsed: ULONG64,
}}
pub type PJOBOBJECT_MEMORY_USAGE_INFORMATION = *mut JOBOBJECT_MEMORY_USAGE_INFORMATION;
STRUCT!{struct JOBOBJECT_MEMORY_USAGE_INFORMATION_V2 {
    BasicInfo: JOBOBJECT_MEMORY_USAGE_INFORMATION,
    JobSharedMemory: ULONG64,
    Reserved: [ULONG64; 2],
}}
pub type PJOBOBJECT_MEMORY_USAGE_INFORMATION_V2 = *mut JOBOBJECT_MEMORY_USAGE_INFORMATION_V2;
STRUCT!{struct SILO_USER_SHARED_DATA {
    ServiceSessionId: ULONG64,
    ActiveConsoleId: ULONG,
    ConsoleSessionForegroundProcessId: LONGLONG,
    NtProductType: NT_PRODUCT_TYPE,
    SuiteMask: ULONG,
    SharedUserSessionId: ULONG,
    IsMultiSessionSku: BOOLEAN,
    NtSystemRoot: [WCHAR; 260],
    UserModeGlobalLogger: [USHORT; 16],
}}
pub type PSILO_USER_SHARED_DATA = *mut SILO_USER_SHARED_DATA;
STRUCT!{struct SILOOBJECT_ROOT_DIRECTORY {
    ControlFlags: ULONG,
    Path: UNICODE_STRING,
}}
pub type PSILOOBJECT_ROOT_DIRECTORY = *mut SILOOBJECT_ROOT_DIRECTORY;
STRUCT!{struct JOBOBJECT_ENERGY_TRACKING_STATE {
    Value: ULONG64,
    UpdateMask: ULONG,
    DesiredState: ULONG,
}}
pub type PJOBOBJECT_ENERGY_TRACKING_STATE = *mut JOBOBJECT_ENERGY_TRACKING_STATE;
EXTERN!{extern "system" {
    fn NtCreateJobObject(
        JobHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtOpenJobObject(
        JobHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtAssignProcessToJobObject(
        JobHandle: HANDLE,
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtTerminateJobObject(
        JobHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn NtIsProcessInJob(
        ProcessHandle: HANDLE,
        JobHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: PVOID,
        JobObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: PVOID,
        JobObjectInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtCreateJobSet(
        NumJob: ULONG,
        UserJobSet: PJOB_SET_ARRAY,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtRevertContainerImpersonation() -> NTSTATUS;
}}
ENUM!{enum MEMORY_RESERVE_TYPE {
    MemoryReserveUserApc = 0,
    MemoryReserveIoCompletion = 1,
    MemoryReserveTypeMax = 2,
}}
EXTERN!{extern "system" {
    fn NtAllocateReserveObject(
        MemoryReserveHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Type: MEMORY_RESERVE_TYPE,
    ) -> NTSTATUS;
}}
