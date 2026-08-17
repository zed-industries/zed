use core::mem::size_of;
use crate::ntapi_base::CLIENT_ID32;
use crate::ntldr::{LDR_DDAG_STATE, LDR_DLL_LOAD_REASON};
use crate::ntpsapi::GDI_HANDLE_BUFFER32;
use crate::ntrtl::RTL_MAX_DRIVE_LETTERS;
use crate::string::{UTF16Const, UTF8Const};
use winapi::shared::guiddef::GUID;
use winapi::shared::ntdef::{
    BOOLEAN, CHAR, LARGE_INTEGER, LCID, LIST_ENTRY32, LONG, NTSTATUS, PROCESSOR_NUMBER,
    SINGLE_LIST_ENTRY32, STRING32, UCHAR, ULARGE_INTEGER, ULONG, ULONGLONG, UNICODE_STRING,
    UNICODE_STRING32, USHORT, WCHAR,
};
use winapi::um::winnt::{FLS_MAXIMUM_AVAILABLE, NT_TIB32};
pub const WOW64_SYSTEM_DIRECTORY: UTF8Const = UTF8Const("SysWOW64\0");
/// "SysWOW64"
pub const WOW64_SYSTEM_DIRECTORY_U: UTF16Const = UTF16Const(&[
    0x0053, 0x0079, 0x0073, 0x0057, 0x004F, 0x0057, 0x0036, 0x0034, 0u16,
]);
pub const WOW64_X86_TAG: UTF8Const = UTF8Const(" (x86)\0");
/// " (x86)"
pub const WOW64_X86_TAG_U: UTF16Const = UTF16Const(&[
    0x0020, 0x0028, 0x0078, 0x0038, 0x0036, 0x0029, 0u16,
]);
ENUM!{enum WOW64_SHARED_INFORMATION {
    SharedNtdll32LdrInitializeThunk = 0,
    SharedNtdll32KiUserExceptionDispatcher = 1,
    SharedNtdll32KiUserApcDispatcher = 2,
    SharedNtdll32KiUserCallbackDispatcher = 3,
    SharedNtdll32ExpInterlockedPopEntrySListFault = 4,
    SharedNtdll32ExpInterlockedPopEntrySListResume = 5,
    SharedNtdll32ExpInterlockedPopEntrySListEnd = 6,
    SharedNtdll32RtlUserThreadStart = 7,
    SharedNtdll32pQueryProcessDebugInformationRemote = 8,
    SharedNtdll32BaseAddress = 9,
    SharedNtdll32LdrSystemDllInitBlock = 10,
    Wow64SharedPageEntriesCount = 11,
}}
STRUCT!{struct RTL_BALANCED_NODE32_u_s {
    Left: ULONG, // WOW64_POINTER
    Right: ULONG, // WOW64_POINTER
}}
UNION!{union RTL_BALANCED_NODE32_u {
    Children: [ULONG; 2], // WOW64_POINTER
    s: RTL_BALANCED_NODE32_u_s,
}}
STRUCT!{struct RTL_BALANCED_NODE32 {
    u: RTL_BALANCED_NODE32_u,
    ParentValue: ULONG,
}}
pub type PRTL_BALANCED_NODE32 = *mut RTL_BALANCED_NODE32;
STRUCT!{struct RTL_RB_TREE32 {
    Root: ULONG, // WOW64_POINTER
    Min: ULONG, // WOW64_POINTER
}}
pub type PRTL_RB_TREE32 = *mut RTL_RB_TREE32;
STRUCT!{struct PEB_LDR_DATA32 {
    Length: ULONG,
    Initialized: BOOLEAN,
    SsHandle: ULONG,
    InLoadOrderModuleList: LIST_ENTRY32,
    InMemoryOrderModuleList: LIST_ENTRY32,
    InInitializationOrderModuleList: LIST_ENTRY32,
    EntryInProgress: ULONG,
    ShutdownInProgress: BOOLEAN,
    ShutdownThreadId: ULONG,
}}
pub type PPEB_LDR_DATA32 = *mut PEB_LDR_DATA32;
STRUCT!{struct LDR_SERVICE_TAG_RECORD32 {
    Next: ULONG,
    ServiceTag: ULONG,
}}
pub type PLDR_SERVICE_TAG_RECORD32 = *mut LDR_SERVICE_TAG_RECORD32;
STRUCT!{struct LDRP_CSLIST32 {
    Tail: ULONG, // WOW64_POINTER
}}
pub type PLDRP_CSLIST32 = *mut LDRP_CSLIST32;
UNION!{union LDR_DDAG_NODE32_u {
    Dependencies: LDRP_CSLIST32,
    RemovalLink: SINGLE_LIST_ENTRY32,
}}
STRUCT!{struct LDR_DDAG_NODE32 {
    Modules: LIST_ENTRY32,
    ServiceTagList: ULONG, // WOW64_POINTER
    LoadCount: ULONG,
    LoadWhileUnloadingCount: ULONG,
    LowestLink: ULONG,
    u: LDR_DDAG_NODE32_u,
    IncomingDependencies: LDRP_CSLIST32,
    State: LDR_DDAG_STATE,
    CondenseLink: SINGLE_LIST_ENTRY32,
    PreorderNumber: ULONG,
}}
pub type PLDR_DDAG_NODE32 = *mut LDR_DDAG_NODE32;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WINXP_32: usize = 80;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN7_32: usize = 144;
pub const LDR_DATA_TABLE_ENTRY_SIZE_WIN8_32: usize = 152;
UNION!{union LDR_DATA_TABLE_ENTRY32_u1 {
    InInitializationOrderLinks: LIST_ENTRY32,
    InProgressLinks: LIST_ENTRY32,
}}
UNION!{union LDR_DATA_TABLE_ENTRY32_u2 {
    FlagGroup: [UCHAR; 4],
    Flags: ULONG,
}}
STRUCT!{struct LDR_DATA_TABLE_ENTRY32 {
    InLoadOrderLinks: LIST_ENTRY32,
    InMemoryOrderLinks: LIST_ENTRY32,
    u1: LDR_DATA_TABLE_ENTRY32_u1,
    DllBase: ULONG, // WOW64_POINTER
    EntryPoint: ULONG, // WOW64_POINTER
    SizeOfImage: ULONG,
    FullDllName: UNICODE_STRING32,
    BaseDllName: UNICODE_STRING32,
    u2: LDR_DATA_TABLE_ENTRY32_u2,
    ObsoleteLoadCount: USHORT,
    TlsIndex: USHORT,
    HashLinks: LIST_ENTRY32,
    TimeDateStamp: ULONG,
    EntryPointActivationContext: ULONG, // WOW64_POINTER
    Lock: ULONG, // WOW64_POINTER
    DdagNode: ULONG, // WOW64_POINTER
    NodeModuleLink: LIST_ENTRY32,
    LoadContext: ULONG, // WOW64_POINTER
    ParentDllBase: ULONG, // WOW64_POINTER
    SwitchBackContext: ULONG, // WOW64_POINTER
    BaseAddressIndexNode: RTL_BALANCED_NODE32,
    MappingInfoIndexNode: RTL_BALANCED_NODE32,
    OriginalBase: ULONG,
    LoadTime: LARGE_INTEGER,
    BaseNameHashValue: ULONG,
    LoadReason: LDR_DLL_LOAD_REASON,
    ImplicitPathOptions: ULONG,
    ReferenceCount: ULONG,
    DependentLoadFlags: ULONG,
    SigningLevel: UCHAR,
}}
BITFIELD!{unsafe LDR_DATA_TABLE_ENTRY32_u2 Flags: ULONG [
    PackagedBinary set_PackagedBinary[0..1],
    MarkedForRemoval set_MarkedForRemoval[1..2],
    ImageDll set_ImageDll[2..3],
    LoadNotificationsSent set_LoadNotificationsSent[3..4],
    TelemetryEntryProcessed set_TelemetryEntryProcessed[4..5],
    ProcessStaticImport set_ProcessStaticImport[5..6],
    InLegacyLists set_InLegacyLists[6..7],
    InIndexes set_InIndexes[7..8],
    ShimDll set_ShimDll[8..9],
    InExceptionTable set_InExceptionTable[9..10],
    ReservedFlags1 set_ReservedFlags1[10..12],
    LoadInProgress set_LoadInProgress[12..13],
    LoadConfigProcessed set_LoadConfigProcessed[13..14],
    EntryProcessed set_EntryProcessed[14..15],
    ProtectDelayLoad set_ProtectDelayLoad[15..16],
    ReservedFlags3 set_ReservedFlags3[16..18],
    DontCallForThreads set_DontCallForThreads[18..19],
    ProcessAttachCalled set_ProcessAttachCalled[19..20],
    ProcessAttachFailed set_ProcessAttachFailed[20..21],
    CorDeferredValidate set_CorDeferredValidate[21..22],
    CorImage set_CorImage[22..23],
    DontRelocate set_DontRelocate[23..24],
    CorILOnly set_CorILOnly[24..25],
    ReservedFlags5 set_ReservedFlags5[25..28],
    Redirected set_Redirected[28..29],
    ReservedFlags6 set_ReservedFlags6[29..31],
    CompatDatabaseProcessed set_CompatDatabaseProcessed[31..32],
]}
pub type PLDR_DATA_TABLE_ENTRY32 = *mut LDR_DATA_TABLE_ENTRY32;
STRUCT!{struct CURDIR32 {
    DosPath: UNICODE_STRING32,
    Handle: ULONG, // WOW64_POINTER
}}
pub type PCURDIR32 = *mut CURDIR32;
STRUCT!{struct RTL_DRIVE_LETTER_CURDIR32 {
    Flags: USHORT,
    Length: USHORT,
    TimeStamp: ULONG,
    DosPath: STRING32,
}}
pub type PRTL_DRIVE_LETTER_CURDIR32 = *mut RTL_DRIVE_LETTER_CURDIR32;
STRUCT!{struct RTL_USER_PROCESS_PARAMETERS32 {
    MaximumLength: ULONG,
    Length: ULONG,
    Flags: ULONG,
    DebugFlags: ULONG,
    ConsoleHandle: ULONG, // WOW64_POINTER
    ConsoleFlags: ULONG,
    StandardInput: ULONG, // WOW64_POINTER
    StandardOutput: ULONG, // WOW64_POINTER
    StandardError: ULONG, // WOW64_POINTER
    CurrentDirectory: CURDIR32,
    DllPath: UNICODE_STRING32,
    ImagePathName: UNICODE_STRING32,
    CommandLine: UNICODE_STRING32,
    Environment: ULONG, // WOW64_POINTER
    StartingX: ULONG,
    StartingY: ULONG,
    CountX: ULONG,
    CountY: ULONG,
    CountCharsX: ULONG,
    CountCharsY: ULONG,
    FillAttribute: ULONG,
    WindowFlags: ULONG,
    ShowWindowFlags: ULONG,
    WindowTitle: UNICODE_STRING32,
    DesktopInfo: UNICODE_STRING32,
    ShellInfo: UNICODE_STRING32,
    RuntimeData: UNICODE_STRING32,
    CurrentDirectories: [RTL_DRIVE_LETTER_CURDIR32; RTL_MAX_DRIVE_LETTERS],
    EnvironmentSize: ULONG,
    EnvironmentVersion: ULONG,
    PackageDependencyData: ULONG, // WOW64_POINTER
    ProcessGroupId: ULONG,
    LoaderThreads: ULONG,
}}
pub type PRTL_USER_PROCESS_PARAMETERS32 = *mut RTL_USER_PROCESS_PARAMETERS32;
UNION!{union PEB32_u {
    KernelCallbackTable: ULONG, // WOW64_POINTER
    UserSharedInfoPtr: ULONG, // WOW64_POINTER
}}
STRUCT!{struct PEB32 {
    InheritedAddressSpace: BOOLEAN,
    ReadImageFileExecOptions: BOOLEAN,
    BeingDebugged: BOOLEAN,
    BitField: BOOLEAN,
    Mutant: ULONG, // WOW64_POINTER
    ImageBaseAddress: ULONG, // WOW64_POINTER
    Ldr: ULONG, // WOW64_POINTER
    ProcessParameters: ULONG, // WOW64_POINTER
    SubSystemData: ULONG, // WOW64_POINTER
    ProcessHeap: ULONG, // WOW64_POINTER
    FastPebLock: ULONG, // WOW64_POINTER
    AtlThunkSListPtr: ULONG, // WOW64_POINTER
    IFEOKey: ULONG, // WOW64_POINTER
    CrossProcessFlags: ULONG,
    u: PEB32_u,
    SystemReserved: [ULONG; 1],
    AtlThunkSListPtr32: ULONG,
    ApiSetMap: ULONG, // WOW64_POINTER
    TlsExpansionCounter: ULONG,
    TlsBitmap: ULONG, // WOW64_POINTER
    TlsBitmapBits: [ULONG; 2],
    ReadOnlySharedMemoryBase: ULONG, // WOW64_POINTER
    HotpatchInformation: ULONG, // WOW64_POINTER
    ReadOnlyStaticServerData: ULONG, // WOW64_POINTER
    AnsiCodePageData: ULONG, // WOW64_POINTER
    OemCodePageData: ULONG, // WOW64_POINTER
    UnicodeCaseTableData: ULONG, // WOW64_POINTER
    NumberOfProcessors: ULONG,
    NtGlobalFlag: ULONG,
    CriticalSectionTimeout: LARGE_INTEGER,
    HeapSegmentReserve: ULONG,
    HeapSegmentCommit: ULONG,
    HeapDeCommitTotalFreeThreshold: ULONG,
    HeapDeCommitFreeBlockThreshold: ULONG,
    NumberOfHeaps: ULONG,
    MaximumNumberOfHeaps: ULONG,
    ProcessHeaps: ULONG, // WOW64_POINTER
    GdiSharedHandleTable: ULONG, // WOW64_POINTER
    ProcessStarterHelper: ULONG, // WOW64_POINTER
    GdiDCAttributeList: ULONG,
    LoaderLock: ULONG, // WOW64_POINTER
    OSMajorVersion: ULONG,
    OSMinorVersion: ULONG,
    OSBuildNumber: USHORT,
    OSCSDVersion: USHORT,
    OSPlatformId: ULONG,
    ImageSubsystem: ULONG,
    ImageSubsystemMajorVersion: ULONG,
    ImageSubsystemMinorVersion: ULONG,
    ActiveProcessAffinityMask: ULONG,
    GdiHandleBuffer: GDI_HANDLE_BUFFER32,
    PostProcessInitRoutine: ULONG, // WOW64_POINTER
    TlsExpansionBitmap: ULONG, // WOW64_POINTER
    TlsExpansionBitmapBits: [ULONG; 32],
    SessionId: ULONG,
    AppCompatFlags: ULARGE_INTEGER,
    AppCompatFlagsUser: ULARGE_INTEGER,
    pShimData: ULONG, // WOW64_POINTER
    AppCompatInfo: ULONG, // WOW64_POINTER
    CSDVersion: UNICODE_STRING32,
    ActivationContextData: ULONG, // WOW64_POINTER
    ProcessAssemblyStorageMap: ULONG, // WOW64_POINTER
    SystemDefaultActivationContextData: ULONG, // WOW64_POINTER
    SystemAssemblyStorageMap: ULONG, // WOW64_POINTER
    MinimumStackCommit: ULONG,
    FlsCallback: ULONG, // WOW64_POINTER
    FlsListHead: LIST_ENTRY32,
    FlsBitmap: ULONG, // WOW64_POINTER
    FlsBitmapBits: [ULONG; FLS_MAXIMUM_AVAILABLE as usize / (size_of::<ULONG>() * 8)],
    FlsHighIndex: ULONG,
    WerRegistrationData: ULONG, // WOW64_POINTER
    WerShipAssertPtr: ULONG, // WOW64_POINTER
    pContextData: ULONG, // WOW64_POINTER
    pImageHeaderHash: ULONG, // WOW64_POINTER
    TracingFlags: ULONG,
    CsrServerReadOnlySharedMemoryBase: ULONGLONG,
    TppWorkerpListLock: ULONG, // WOW64_POINTER
    TppWorkerpList: LIST_ENTRY32,
    WaitOnAddressHashTable: [ULONG; 128], // WOW64_POINTER
    TelemetryCoverageHeader: ULONG, // WOW64_POINTER
    CloudFileFlags: ULONG,
    CloudFileDiagFlags: ULONG,
    PlaceholderCompatibilityMode: CHAR,
    PlaceholderCompatibilityModeReserved: [CHAR; 7],
}}
BITFIELD!{PEB32 BitField: BOOLEAN [
    ImageUsesLargePages set_ImageUsesLargePages[0..1],
    IsProtectedProcess set_IsProtectedProcess[1..2],
    IsImageDynamicallyRelocated set_IsImageDynamicallyRelocated[2..3],
    SkipPatchingUser32Forwarders set_SkipPatchingUser32Forwarders[3..4],
    IsPackagedProcess set_IsPackagedProcess[4..5],
    IsAppContainer set_IsAppContainer[5..6],
    IsProtectedProcessLight set_IsProtectedProcessLight[6..7],
    IsLongPathAwareProcess set_IsLongPathAwareProcess[7..8],
]}
BITFIELD!{PEB32 CrossProcessFlags: ULONG [
    ProcessInJob set_ProcessInJob[0..1],
    ProcessInitializing set_ProcessInitializing[1..2],
    ProcessUsingVEH set_ProcessUsingVEH[2..3],
    ProcessUsingVCH set_ProcessUsingVCH[3..4],
    ProcessUsingFTH set_ProcessUsingFTH[4..5],
    ReservedBits0 set_ReservedBits0[5..32],
]}
BITFIELD!{PEB32 TracingFlags: ULONG [
    HeapTracingEnabled set_HeapTracingEnabled[0..1],
    CritSecTracingEnabled set_CritSecTracingEnabled[1..2],
    LibLoaderTracingEnabled set_LibLoaderTracingEnabled[2..3],
    SpareTracingBits set_SpareTracingBits[3..32],
]}
pub type PPEB32 = *mut PEB32;
pub const GDI_BATCH_BUFFER_SIZE: usize = 310;
STRUCT!{struct GDI_TEB_BATCH32 {
    Offset: ULONG,
    HDC: ULONG,
    Buffer: [ULONG; GDI_BATCH_BUFFER_SIZE],
}}
pub type PGDI_TEB_BATCH32 = *mut GDI_TEB_BATCH32;
STRUCT!{struct TEB32_u_s {
    ReservedPad0: UCHAR,
    ReservedPad1: UCHAR,
    ReservedPad2: UCHAR,
    IdealProcessor: UCHAR,
}}
UNION!{union TEB32_u {
    CurrentIdealProcessor: PROCESSOR_NUMBER,
    IdealProcessorValue: ULONG,
    s: TEB32_u_s,
}}
STRUCT!{struct TEB32 {
    NtTib: NT_TIB32,
    EnvironmentPointer: ULONG, // WOW64_POINTER
    ClientId: CLIENT_ID32,
    ActiveRpcHandle: ULONG, // WOW64_POINTER
    ThreadLocalStoragePointer: ULONG, // WOW64_POINTER
    ProcessEnvironmentBlock: ULONG, // WOW64_POINTER
    LastErrorValue: ULONG,
    CountOfOwnedCriticalSections: ULONG,
    CsrClientThread: ULONG, // WOW64_POINTER
    Win32ThreadInfo: ULONG, // WOW64_POINTER
    User32Reserved: [ULONG; 26],
    UserReserved: [ULONG; 5],
    WOW32Reserved: ULONG, // WOW64_POINTER
    CurrentLocale: LCID,
    FpSoftwareStatusRegister: ULONG,
    ReservedForDebuggerInstrumentation: [ULONG; 16], // WOW64_POINTER
    SystemReserved1: [ULONG; 36], // WOW64_POINTER
    WorkingOnBehalfTicket: [UCHAR; 8],
    ExceptionCode: NTSTATUS,
    ActivationContextStackPointer: ULONG, // WOW64_POINTER
    InstrumentationCallbackSp: ULONG,
    InstrumentationCallbackPreviousPc: ULONG,
    InstrumentationCallbackPreviousSp: ULONG,
    InstrumentationCallbackDisabled: BOOLEAN,
    SpareBytes: [UCHAR; 23],
    TxFsContext: ULONG,
    GdiTebBatch: GDI_TEB_BATCH32,
    RealClientId: CLIENT_ID32,
    GdiCachedProcessHandle: ULONG, // WOW64_POINTER
    GdiClientPID: ULONG,
    GdiClientTID: ULONG,
    GdiThreadLocalInfo: ULONG, // WOW64_POINTER
    Win32ClientInfo: [ULONG; 62],
    glDispatchTable: [ULONG; 233], // WOW64_POINTER
    glReserved1: [ULONG; 29], // WOW64_POINTER
    glReserved2: ULONG, // WOW64_POINTER
    glSectionInfo: ULONG, // WOW64_POINTER
    glSection: ULONG, // WOW64_POINTER
    glTable: ULONG, // WOW64_POINTER
    glCurrentRC: ULONG, // WOW64_POINTER
    glContext: ULONG, // WOW64_POINTER
    LastStatusValue: NTSTATUS,
    StaticUnicodeString: UNICODE_STRING32,
    StaticUnicodeBuffer: [WCHAR; 261],
    DeallocationStack: ULONG, // WOW64_POINTER
    TlsSlots: [ULONG; 64], // WOW64_POINTER
    TlsLinks: LIST_ENTRY32,
    Vdm: ULONG, // WOW64_POINTER
    ReservedForNtRpc: ULONG, // WOW64_POINTER
    DbgSsReserved: [ULONG; 2], // WOW64_POINTER
    HardErrorMode: ULONG,
    Instrumentation: [ULONG; 9], // WOW64_POINTER
    ActivityId: GUID,
    SubProcessTag: ULONG, // WOW64_POINTER
    PerflibData: ULONG, // WOW64_POINTER
    EtwTraceData: ULONG, // WOW64_POINTER
    WinSockData: ULONG, // WOW64_POINTER
    GdiBatchCount: ULONG,
    u: TEB32_u,
    GuaranteedStackBytes: ULONG,
    ReservedForPerf: ULONG, // WOW64_POINTER
    ReservedForOle: ULONG, // WOW64_POINTER
    WaitingOnLoaderLock: ULONG,
    SavedPriorityState: ULONG, // WOW64_POINTER
    ReservedForCodeCoverage: ULONG,
    ThreadPoolData: ULONG, // WOW64_POINTER
    TlsExpansionSlots: ULONG, // WOW64_POINTER
    MuiGeneration: ULONG,
    IsImpersonating: ULONG,
    NlsCache: ULONG, // WOW64_POINTER
    pShimData: ULONG, // WOW64_POINTER
    HeapVirtualAffinity: USHORT,
    LowFragHeapDataSlot: USHORT,
    CurrentTransactionHandle: ULONG, // WOW64_POINTER
    ActiveFrame: ULONG, // WOW64_POINTER
    FlsData: ULONG, // WOW64_POINTER
    PreferredLanguages: ULONG, // WOW64_POINTER
    UserPrefLanguages: ULONG, // WOW64_POINTER
    MergedPrefLanguages: ULONG, // WOW64_POINTER
    MuiImpersonation: ULONG,
    CrossTebFlags: USHORT,
    SameTebFlags: USHORT,
    TxnScopeEnterCallback: ULONG, // WOW64_POINTER
    TxnScopeExitCallback: ULONG, // WOW64_POINTER
    TxnScopeContext: ULONG, // WOW64_POINTER
    LockCount: ULONG,
    WowTebOffset: LONG,
    ResourceRetValue: ULONG, // WOW64_POINTER
    ReservedForWdf: ULONG, // WOW64_POINTER
    ReservedForCrt: ULONGLONG,
    EffectiveContainerId: GUID,
}}
BITFIELD!{TEB32 SameTebFlags: USHORT [
    SafeThunkCall set_SafeThunkCall[0..1],
    InDebugPrint set_InDebugPrint[1..2],
    HasFiberData set_HasFiberData[2..3],
    SkipThreadAttach set_SkipThreadAttach[3..4],
    WerInShipAssertCode set_WerInShipAssertCode[4..5],
    RanProcessInit set_RanProcessInit[5..6],
    ClonedThread set_ClonedThread[6..7],
    SuppressDebugMsg set_SuppressDebugMsg[7..8],
    DisableUserStackWalk set_DisableUserStackWalk[8..9],
    RtlExceptionAttached set_RtlExceptionAttached[9..10],
    InitialThread set_InitialThread[10..11],
    SessionAware set_SessionAware[11..12],
    LoadOwner set_LoadOwner[12..13],
    LoaderWorker set_LoaderWorker[13..14],
    SpareSameTebBits set_SpareSameTebBits[14..16],
]}
pub type PTEB32 = *mut TEB32;
#[inline]
pub fn UStr32ToUStr(
    Destination: &mut UNICODE_STRING,
    Source: &UNICODE_STRING32,
) {
    Destination.Length = Source.Length;
    Destination.MaximumLength = Source.MaximumLength;
    Destination.Buffer = Source.Buffer as *mut u16;
}
#[inline]
pub fn UStrToUStr32(
    Destination: &mut UNICODE_STRING32,
    Source: &UNICODE_STRING,
) {
    Destination.Length = Source.Length;
    Destination.MaximumLength = Source.MaximumLength;
    Destination.Buffer = Source.Buffer as u32;
}
