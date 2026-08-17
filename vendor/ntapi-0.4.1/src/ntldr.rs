use winapi::shared::basetsd::{LONG_PTR, PSIZE_T, SIZE_T, ULONG_PTR};
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, LARGE_INTEGER, LIST_ENTRY, LONG, LONGLONG, NTSTATUS, PANSI_STRING, PCSTR,
    PCUNICODE_STRING, PCWSTR, PHANDLE, POBJECT_ATTRIBUTES, PSINGLE_LIST_ENTRY, PSTR, PULONG,
    PUNICODE_STRING, PUSHORT, PVOID, PWSTR, RTL_BALANCED_NODE, SINGLE_LIST_ENTRY, UCHAR, ULONG,
    UNICODE_STRING, USHORT,
};
use winapi::um::winnt::{
    ACCESS_MASK, ACTIVATION_CONTEXT, IMAGE_RESOURCE_DIRECTORY_ENTRY, PCIMAGE_DELAYLOAD_DESCRIPTOR,
    PIMAGE_BASE_RELOCATION, PIMAGE_IMPORT_DESCRIPTOR, PIMAGE_RESOURCE_DATA_ENTRY,
    PIMAGE_RESOURCE_DIRECTORY, PIMAGE_RESOURCE_DIRECTORY_STRING, PIMAGE_THUNK_DATA,
};
FN!{stdcall PLDR_INIT_ROUTINE(
    DllHandle: PVOID,
    Reason: ULONG,
    Context: PVOID,
) -> BOOLEAN}
STRUCT!{struct LDR_SERVICE_TAG_RECORD {
    Next: *mut LDR_SERVICE_TAG_RECORD,
    ServiceTag: ULONG,
}}
pub type PLDR_SERVICE_TAG_RECORD = *mut LDR_SERVICE_TAG_RECORD;
STRUCT!{struct LDRP_CSLIST {
    Tail: PSINGLE_LIST_ENTRY,
}}
pub type PLDRP_CSLIST = *mut LDRP_CSLIST;
ENUM!{enum LDR_DDAG_STATE {
    LdrModulesMerged = -5i32 as u32,
    LdrModulesInitError = -4i32 as u32,
    LdrModulesSnapError = -3i32 as u32,
    LdrModulesUnloaded = -2i32 as u32,
    LdrModulesUnloading = -1i32 as u32,
    LdrModulesPlaceHolder = 0,
    LdrModulesMapping = 1,
    LdrModulesMapped = 2,
    LdrModulesWaitingForDependencies = 3,
    LdrModulesSnapping = 4,
    LdrModulesSnapped = 5,
    LdrModulesCondensed = 6,
    LdrModulesReadyToInit = 7,
    LdrModulesInitializing = 8,
    LdrModulesReadyToRun = 9,
}}
UNION!{union LDR_DDAG_NODE_u {
    Dependencies: LDRP_CSLIST,
    RemovalLink: SINGLE_LIST_ENTRY,
}}
STRUCT!{struct LDR_DDAG_NODE {
    Modules: LIST_ENTRY,
    ServiceTagList: PLDR_SERVICE_TAG_RECORD,
    LoadCount: ULONG,
    LoadWhileUnloadingCount: ULONG,
    LowestLink: ULONG,
    u: LDR_DDAG_NODE_u,
    IncomingDependencies: LDRP_CSLIST,
    State: LDR_DDAG_STATE,
    CondenseLink: SINGLE_LIST_ENTRY,
    PreorderNumber: ULONG,
}}
pub type PLDR_DDAG_NODE = *mut LDR_DDAG_NODE;
STRUCT!{struct LDR_DEPENDENCY_RECORD {
    DependencyLink: SINGLE_LIST_ENTRY,
    DependencyNode: PLDR_DDAG_NODE,
    IncomingDependencyLink: SINGLE_LIST_ENTRY,
    IncomingDependencyNode: PLDR_DDAG_NODE,
}}
pub type PLDR_DEPENDENCY_RECORD = *mut LDR_DEPENDENCY_RECORD;
ENUM!{enum LDR_DLL_LOAD_REASON {
    LoadReasonStaticDependency = 0,
    LoadReasonStaticForwarderDependency = 1,
    LoadReasonDynamicForwarderDependency = 2,
    LoadReasonDelayloadDependency = 3,
    LoadReasonDynamicLoad = 4,
    LoadReasonAsImageLoad = 5,
    LoadReasonAsDataLoad = 6,
    LoadReasonEnclavePrimary = 7,
    LoadReasonEnclaveDependency = 8,
    LoadReasonUnknown = -1i32 as u32,
}}
pub type PLDR_DLL_LOAD_REASON = *mut LDR_DLL_LOAD_REASON;
pub const LDRP_PACKAGED_BINARY: ULONG = 0x00000001;
pub const LDRP_STATIC_LINK: ULONG = 0x00000002;
pub const LDRP_IMAGE_DLL: ULONG = 0x00000004;
pub const LDRP_LOAD_IN_PROGRESS: ULONG = 0x00001000;
pub const LDRP_UNLOAD_IN_PROGRESS: ULONG = 0x00002000;
pub const LDRP_ENTRY_PROCESSED: ULONG = 0x00004000;
pub const LDRP_ENTRY_INSERTED: ULONG = 0x00008000;
pub const LDRP_CURRENT_LOAD: ULONG = 0x00010000;
pub const LDRP_FAILED_BUILTIN_LOAD: ULONG = 0x00020000;
pub const LDRP_DONT_CALL_FOR_THREADS: ULONG = 0x00040000;
pub const LDRP_PROCESS_ATTACH_CALLED: ULONG = 0x00080000;
pub const LDRP_DEBUG_SYMBOLS_LOADED: ULONG = 0x00100000;
pub const LDRP_IMAGE_NOT_AT_BASE: ULONG = 0x00200000;
pub const LDRP_COR_IMAGE: ULONG = 0x00400000;
pub const LDRP_DONT_RELOCATE: ULONG = 0x00800000;
pub const LDRP_SYSTEM_MAPPED: ULONG = 0x01000000;
pub const LDRP_IMAGE_VERIFYING: ULONG = 0x02000000;
pub const LDRP_DRIVER_DEPENDENT_DLL: ULONG = 0x04000000;
pub const LDRP_ENTRY_NATIVE: ULONG = 0x08000000;
pub const LDRP_REDIRECTED: ULONG = 0x10000000;
pub const LDRP_NON_PAGED_DEBUG_INFO: ULONG = 0x20000000;
pub const LDRP_MM_LOADED: ULONG = 0x40000000;
pub const LDRP_COMPAT_DATABASE_PROCESSED: ULONG = 0x80000000;
STRUCT!{struct LDRP_LOAD_CONTEXT {
    BaseDllName: UNICODE_STRING,
    somestruct: PVOID,
    Flags: ULONG,
    pstatus: *mut NTSTATUS,
    ParentEntry: *mut LDR_DATA_TABLE_ENTRY,
    Entry: *mut LDR_DATA_TABLE_ENTRY,
    WorkQueueListEntry: LIST_ENTRY,
    ReplacedEntry: *mut LDR_DATA_TABLE_ENTRY,
    pvImports: *mut *mut LDR_DATA_TABLE_ENTRY,
    ImportDllCount: ULONG,
    TaskCount: LONG,
    pvIAT: PVOID,
    SizeOfIAT: ULONG,
    CurrentDll: ULONG,
    piid: PIMAGE_IMPORT_DESCRIPTOR,
    OriginalIATProtect: ULONG,
    GuardCFCheckFunctionPointer: PVOID,
    pGuardCFCheckFunctionPointer: *mut PVOID,
}}
UNION!{union LDR_DATA_TABLE_ENTRY_u1 {
    InInitializationOrderLinks: LIST_ENTRY,
    InProgressLinks: LIST_ENTRY,
}}
UNION!{union LDR_DATA_TABLE_ENTRY_u2 {
    FlagGroup: [UCHAR; 4],
    Flags: ULONG,
}}
STRUCT!{struct LDR_DATA_TABLE_ENTRY {
    InLoadOrderLinks: LIST_ENTRY,
    InMemoryOrderLinks: LIST_ENTRY,
    u1: LDR_DATA_TABLE_ENTRY_u1,
    DllBase: PVOID,
    EntryPoint: PLDR_INIT_ROUTINE,
    SizeOfImage: ULONG,
    FullDllName: UNICODE_STRING,
    BaseDllName: UNICODE_STRING,
    u2: LDR_DATA_TABLE_ENTRY_u2,
    ObsoleteLoadCount: USHORT,
    TlsIndex: USHORT,
    HashLinks: LIST_ENTRY,
    TimeDateStamp: ULONG,
    EntryPointActivationContext: *mut ACTIVATION_CONTEXT,
    Lock: PVOID,
    DdagNode: PLDR_DDAG_NODE,
    NodeModuleLink: LIST_ENTRY,
    LoadContext: *mut LDRP_LOAD_CONTEXT,
    ParentDllBase: PVOID,
    SwitchBackContext: PVOID,
    BaseAddressIndexNode: RTL_BALANCED_NODE,
    MappingInfoIndexNode: RTL_BALANCED_NODE,
    OriginalBase: ULONG_PTR,
    LoadTime: LARGE_INTEGER,
    BaseNameHashValue: ULONG,
    LoadReason: LDR_DLL_LOAD_REASON,
    ImplicitPathOptions: ULONG,
    ReferenceCount: ULONG,
    DependentLoadFlags: ULONG,
    SigningLevel: UCHAR,
}}
BITFIELD!{unsafe LDR_DATA_TABLE_ENTRY_u2 Flags: ULONG [
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
pub type PLDR_DATA_TABLE_ENTRY = *mut LDR_DATA_TABLE_ENTRY;
#[inline]
pub const fn LDR_IS_DATAFILE(DllHandle: ULONG_PTR) -> bool {
    DllHandle & 1 != 0
}
#[inline]
pub const fn LDR_IS_IMAGEMAPPING(DllHandle: ULONG_PTR) -> bool {
    DllHandle & 2 != 0
}
#[inline]
pub const fn LDR_IS_RESOURCE(DllHandle: ULONG_PTR) -> bool {
    LDR_IS_IMAGEMAPPING(DllHandle) || LDR_IS_DATAFILE(DllHandle)
}
EXTERN!{extern "system" {
    fn LdrLoadDll(
        DllPath: PWSTR,
        DllCharacteristics: PULONG,
        DllName: PUNICODE_STRING,
        DllHandle: *mut PVOID,
    ) -> NTSTATUS;
    fn LdrUnloadDll(
        DllHandle: PVOID,
    ) -> NTSTATUS;
    fn LdrGetDllHandle(
        DllPath: PWSTR,
        DllCharacteristics: PULONG,
        DllName: PUNICODE_STRING,
        DllHandle: *mut PVOID,
    ) -> NTSTATUS;
}}
pub const LDR_GET_DLL_HANDLE_EX_UNCHANGED_REFCOUNT: ULONG = 0x00000001;
pub const LDR_GET_DLL_HANDLE_EX_PIN: ULONG = 0x00000002;
EXTERN!{extern "system" {
    fn LdrGetDllHandleEx(
        Flags: ULONG,
        DllPath: PWSTR,
        DllCharacteristics: PULONG,
        DllName: PUNICODE_STRING,
        DllHandle: *mut PVOID,
    ) -> NTSTATUS;
    fn LdrGetDllHandleByMapping(
        BaseAddress: PVOID,
        DllHandle: *mut PVOID,
    ) -> NTSTATUS;
    fn LdrGetDllHandleByName(
        BaseDllName: PUNICODE_STRING,
        FullDllName: PUNICODE_STRING,
        DllHandle: *mut PVOID,
    ) -> NTSTATUS;
    fn LdrGetDllFullName(
        DllHandle: PVOID,
        FullDllName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn LdrGetDllDirectory(
        DllDirectory: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn LdrSetDllDirectory(
        DllDirectory: PUNICODE_STRING,
    ) -> NTSTATUS;
}}
pub const LDR_ADDREF_DLL_PIN: ULONG = 0x00000001;
EXTERN!{extern "system" {
    fn LdrAddRefDll(
        Flags: ULONG,
        DllHandle: PVOID,
    ) -> NTSTATUS;
    fn LdrGetProcedureAddress(
        DllHandle: PVOID,
        ProcedureName: PANSI_STRING,
        ProcedureNumber: ULONG,
        ProcedureAddress: *mut PVOID,
    ) -> NTSTATUS;
}}
pub const LDR_GET_PROCEDURE_ADDRESS_DONT_RECORD_FORWARDER: ULONG = 0x00000001;
EXTERN!{extern "system" {
    fn LdrGetProcedureAddressEx(
        DllHandle: PVOID,
        ProcedureName: PANSI_STRING,
        ProcedureNumber: ULONG,
        ProcedureAddress: *mut PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn LdrGetKnownDllSectionHandle(
        DllName: PCWSTR,
        KnownDlls32: BOOLEAN,
        Section: PHANDLE,
    ) -> NTSTATUS;
    fn LdrGetProcedureAddressForCaller(
        DllHandle: PVOID,
        ProcedureName: PANSI_STRING,
        ProcedureNumber: ULONG,
        ProcedureAddress: *mut PVOID,
        Flags: ULONG,
        Callback: *mut PVOID,
    ) -> NTSTATUS;
}}
pub const LDR_LOCK_LOADER_LOCK_FLAG_RAISE_ON_ERRORS: ULONG = 0x00000001;
pub const LDR_LOCK_LOADER_LOCK_FLAG_TRY_ONLY: ULONG = 0x00000002;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_INVALID: ULONG = 0;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_LOCK_ACQUIRED: ULONG = 1;
pub const LDR_LOCK_LOADER_LOCK_DISPOSITION_LOCK_NOT_ACQUIRED: ULONG = 2;
EXTERN!{extern "system" {
    fn LdrLockLoaderLock(
        Flags: ULONG,
        Disposition: *mut ULONG,
        Cookie: *mut PVOID,
    ) -> NTSTATUS;
}}
pub const LDR_UNLOCK_LOADER_LOCK_FLAG_RAISE_ON_ERRORS: ULONG = 0x00000001;
EXTERN!{extern "system" {
    fn LdrUnlockLoaderLock(
        Flags: ULONG,
        Cookie: PVOID,
    ) -> NTSTATUS;
    fn LdrRelocateImage(
        NewBase: PVOID,
        LoaderName: PSTR,
        Success: NTSTATUS,
        Conflict: NTSTATUS,
        Invalid: NTSTATUS,
    ) -> NTSTATUS;
    fn LdrRelocateImageWithBias(
        NewBase: PVOID,
        Bias: LONGLONG,
        LoaderName: PSTR,
        Success: NTSTATUS,
        Conflict: NTSTATUS,
        Invalid: NTSTATUS,
    ) -> NTSTATUS;
    fn LdrProcessRelocationBlock(
        VA: ULONG_PTR,
        SizeOfBlock: ULONG,
        NextOffset: PUSHORT,
        Diff: LONG_PTR,
    ) -> PIMAGE_BASE_RELOCATION;
    fn LdrVerifyMappedImageMatchesChecksum(
        BaseAddress: PVOID,
        NumberOfBytes: SIZE_T,
        FileLength: ULONG,
    ) -> BOOLEAN;
}}
FN!{stdcall PLDR_IMPORT_MODULE_CALLBACK(
    Parameter: PVOID,
    ModuleName: PSTR,
) -> ()}
EXTERN!{extern "system" {
    fn LdrVerifyImageMatchesChecksum(
        ImageFileHandle: HANDLE,
        ImportCallbackRoutine: PLDR_IMPORT_MODULE_CALLBACK,
        ImportCallbackParameter: PVOID,
        ImageCharacteristics: PUSHORT,
    ) -> NTSTATUS;
}}
STRUCT!{struct LDR_IMPORT_CALLBACK_INFO {
    ImportCallbackRoutine: PLDR_IMPORT_MODULE_CALLBACK,
    ImportCallbackParameter: PVOID,
}}
pub type PLDR_IMPORT_CALLBACK_INFO = *mut LDR_IMPORT_CALLBACK_INFO;
STRUCT!{struct LDR_SECTION_INFO {
    SectionHandle: HANDLE,
    DesiredAccess: ACCESS_MASK,
    ObjA: POBJECT_ATTRIBUTES,
    SectionPageProtection: ULONG,
    AllocationAttributes: ULONG,
}}
pub type PLDR_SECTION_INFO = *mut LDR_SECTION_INFO;
STRUCT!{struct LDR_VERIFY_IMAGE_INFO {
    Size: ULONG,
    Flags: ULONG,
    CallbackInfo: LDR_IMPORT_CALLBACK_INFO,
    SectionInfo: LDR_SECTION_INFO,
    ImageCharacteristics: USHORT,
}}
pub type PLDR_VERIFY_IMAGE_INFO = *mut LDR_VERIFY_IMAGE_INFO;
EXTERN!{extern "system" {
    fn LdrVerifyImageMatchesChecksumEx(
        ImageFileHandle: HANDLE,
        VerifyInfo: PLDR_VERIFY_IMAGE_INFO,
    ) -> NTSTATUS;
    fn LdrQueryModuleServiceTags(
        DllHandle: PVOID,
        ServiceTagBuffer: PULONG,
        BufferSize: PULONG,
    ) -> NTSTATUS;
}}
pub const LDR_DLL_NOTIFICATION_REASON_LOADED: ULONG = 1;
pub const LDR_DLL_NOTIFICATION_REASON_UNLOADED: ULONG = 2;
STRUCT!{struct LDR_DLL_LOADED_NOTIFICATION_DATA {
    Flags: ULONG,
    FullDllName: PUNICODE_STRING,
    BaseDllName: PUNICODE_STRING,
    DllBase: PVOID,
    SizeOfImage: ULONG,
}}
pub type PLDR_DLL_LOADED_NOTIFICATION_DATA = *mut LDR_DLL_LOADED_NOTIFICATION_DATA;
STRUCT!{struct LDR_DLL_UNLOADED_NOTIFICATION_DATA {
    Flags: ULONG,
    FullDllName: PCUNICODE_STRING,
    BaseDllName: PCUNICODE_STRING,
    DllBase: PVOID,
    SizeOfImage: ULONG,
}}
pub type PLDR_DLL_UNLOADED_NOTIFICATION_DATA = *mut LDR_DLL_UNLOADED_NOTIFICATION_DATA;
UNION!{union LDR_DLL_NOTIFICATION_DATA {
    Loaded: LDR_DLL_LOADED_NOTIFICATION_DATA,
    Unloaded: LDR_DLL_UNLOADED_NOTIFICATION_DATA,
}}
pub type PLDR_DLL_NOTIFICATION_DATA = *mut LDR_DLL_NOTIFICATION_DATA;
FN!{stdcall PLDR_DLL_NOTIFICATION_FUNCTION(
    NotificationReason: ULONG,
    NotificationData: PLDR_DLL_NOTIFICATION_DATA,
    Context: PVOID,
) -> ()}
EXTERN!{extern "system" {
    fn LdrRegisterDllNotification(
        Flags: ULONG,
        NotificationFunction: PLDR_DLL_NOTIFICATION_FUNCTION,
        Context: PVOID,
        Cookie: *mut PVOID,
    ) -> NTSTATUS;
    fn LdrUnregisterDllNotification(
        Cookie: PVOID,
    ) -> NTSTATUS;
}}
STRUCT!{struct PS_MITIGATION_OPTIONS_MAP {
    Map: [ULONG_PTR; 2],
}}
pub type PPS_MITIGATION_OPTIONS_MAP = *mut PS_MITIGATION_OPTIONS_MAP;
STRUCT!{struct PS_MITIGATION_AUDIT_OPTIONS_MAP {
    Map: [ULONG_PTR; 2],
}}
pub type PPS_MITIGATION_AUDIT_OPTIONS_MAP = *mut PS_MITIGATION_AUDIT_OPTIONS_MAP;
STRUCT!{struct PS_SYSTEM_DLL_INIT_BLOCK {
    Size: ULONG,
    SystemDllWowRelocation: ULONG_PTR,
    SystemDllNativeRelocation: ULONG_PTR,
    Wow64SharedInformation: [ULONG_PTR; 16],
    RngData: ULONG,
    Flags: ULONG,
    MitigationOptionsMap: PS_MITIGATION_OPTIONS_MAP,
    CfgBitMap: ULONG_PTR,
    CfgBitMapSize: ULONG_PTR,
    Wow64CfgBitMap: ULONG_PTR,
    Wow64CfgBitMapSize: ULONG_PTR,
    MitigationAuditOptionsMap: PS_MITIGATION_AUDIT_OPTIONS_MAP,
}}
BITFIELD!{PS_SYSTEM_DLL_INIT_BLOCK Flags: ULONG [
    CfgOverride set_CfgOverride[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PPS_SYSTEM_DLL_INIT_BLOCK = *mut PS_SYSTEM_DLL_INIT_BLOCK;
EXTERN!{extern "system" {
    fn LdrSystemDllInitBlock() -> PPS_SYSTEM_DLL_INIT_BLOCK;
    fn LdrAddLoadAsDataTable(
        Module: PVOID,
        FilePath: PWSTR,
        Size: SIZE_T,
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn LdrRemoveLoadAsDataTable(
        InitModule: PVOID,
        BaseModule: *mut PVOID,
        Size: PSIZE_T,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn LdrGetFileNameFromLoadAsDataTable(
        Module: PVOID,
        pFileNamePrt: *mut PVOID,
    ) -> NTSTATUS;
    fn LdrDisableThreadCalloutsForDll(
        DllImageBase: PVOID,
    ) -> NTSTATUS;
    fn LdrAccessResource(
        DllHandle: PVOID,
        ResourceDataEntry: PIMAGE_RESOURCE_DATA_ENTRY,
        ResourceBuffer: *mut PVOID,
        ResourceLength: *mut ULONG,
    ) -> NTSTATUS;
}}
STRUCT!{struct LDR_RESOURCE_INFO {
    Type: ULONG_PTR,
    Name: ULONG_PTR,
    Language: ULONG_PTR,
}}
pub type PLDR_RESOURCE_INFO = *mut LDR_RESOURCE_INFO;
pub const RESOURCE_TYPE_LEVEL: ULONG = 0;
pub const RESOURCE_NAME_LEVEL: ULONG = 1;
pub const RESOURCE_LANGUAGE_LEVEL: ULONG = 2;
pub const RESOURCE_DATA_LEVEL: ULONG = 3;
EXTERN!{extern "system" {
    fn LdrFindResource_U(
        DllHandle: PVOID,
        ResourceInfo: PLDR_RESOURCE_INFO,
        Level: ULONG,
        ResourceDataEntry: *mut PIMAGE_RESOURCE_DATA_ENTRY,
    ) -> NTSTATUS;
    fn LdrFindResourceDirectory_U(
        DllHandle: PVOID,
        ResourceInfo: PLDR_RESOURCE_INFO,
        Level: ULONG,
        ResourceDirectory: *mut PIMAGE_RESOURCE_DIRECTORY,
    ) -> NTSTATUS;
}}
STRUCT!{struct LDR_ENUM_RESOURCE_ENTRY_Path_s {
    Id: USHORT,
    NameIsPresent: USHORT,
}}
UNION!{union LDR_ENUM_RESOURCE_ENTRY_Path {
    NameOrId: ULONG_PTR,
    Name: PIMAGE_RESOURCE_DIRECTORY_STRING,
    s: LDR_ENUM_RESOURCE_ENTRY_Path_s,
}}
STRUCT!{struct LDR_ENUM_RESOURCE_ENTRY {
    Path: [LDR_ENUM_RESOURCE_ENTRY_Path; 3],
    Data: PVOID,
    Size: ULONG,
    Reserved: ULONG,
}}
pub type PLDR_ENUM_RESOURCE_ENTRY = *mut LDR_ENUM_RESOURCE_ENTRY;
#[inline]
pub unsafe fn NAME_FROM_RESOURCE_ENTRY(
    RootDirectory: PIMAGE_RESOURCE_DIRECTORY,
    Entry: &IMAGE_RESOURCE_DIRECTORY_ENTRY,
) -> usize {
    if Entry.u.s().NameIsString() != 0 {
        return RootDirectory as usize + Entry.u.s().NameOffset() as usize;
    }
    *Entry.u.Id() as usize
}
EXTERN!{extern "system" {
    fn LdrEnumResources(
        DllHandle: PVOID,
        ResourceInfo: PLDR_RESOURCE_INFO,
        Level: ULONG,
        ResourceCount: *mut ULONG,
        Resources: PLDR_ENUM_RESOURCE_ENTRY,
    ) -> NTSTATUS;
    fn LdrFindEntryForAddress(
        DllHandle: PVOID,
        Entry: *mut PLDR_DATA_TABLE_ENTRY,
    ) -> NTSTATUS;
}}
STRUCT!{struct RTL_PROCESS_MODULE_INFORMATION {
    Section: HANDLE,
    MappedBase: PVOID,
    ImageBase: PVOID,
    ImageSize: ULONG,
    Flags: ULONG,
    LoadOrderIndex: USHORT,
    InitOrderIndex: USHORT,
    LoadCount: USHORT,
    OffsetToFileName: USHORT,
    FullPathName: [UCHAR; 256],
}}
pub type PRTL_PROCESS_MODULE_INFORMATION = *mut RTL_PROCESS_MODULE_INFORMATION;
STRUCT!{struct RTL_PROCESS_MODULES {
    NumberOfModules: ULONG,
    Modules: [RTL_PROCESS_MODULE_INFORMATION; 1],
}}
pub type PRTL_PROCESS_MODULES = *mut RTL_PROCESS_MODULES;
STRUCT!{struct RTL_PROCESS_MODULE_INFORMATION_EX {
    NextOffset: USHORT,
    BaseInfo: RTL_PROCESS_MODULE_INFORMATION,
    ImageChecksum: ULONG,
    TimeDateStamp: ULONG,
    DefaultBase: PVOID,
}}
pub type PRTL_PROCESS_MODULE_INFORMATION_EX = *mut RTL_PROCESS_MODULE_INFORMATION_EX;
EXTERN!{extern "system" {
    fn LdrQueryProcessModuleInformation(
        ModuleInformation: PRTL_PROCESS_MODULES,
        Size: ULONG,
        ReturnedSize: PULONG,
    ) -> NTSTATUS;
}}
FN!{stdcall PLDR_ENUM_CALLBACK(
    ModuleInformation: PLDR_DATA_TABLE_ENTRY,
    Parameter: PVOID,
    Stop: *mut BOOLEAN,
) -> ()}
EXTERN!{extern "system" {
    fn LdrEnumerateLoadedModules(
        ReservedFlag: BOOLEAN,
        EnumProc: PLDR_ENUM_CALLBACK,
        Context: PVOID,
    ) -> NTSTATUS;
    fn LdrOpenImageFileOptionsKey(
        SubKey: PUNICODE_STRING,
        Wow64: BOOLEAN,
        NewKeyHandle: PHANDLE,
    ) -> NTSTATUS;
    fn LdrQueryImageFileKeyOption(
        KeyHandle: HANDLE,
        ValueName: PCWSTR,
        Type: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;
    fn LdrQueryImageFileExecutionOptions(
        SubKey: PUNICODE_STRING,
        ValueName: PCWSTR,
        ValueSize: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;
    fn LdrQueryImageFileExecutionOptionsEx(
        SubKey: PUNICODE_STRING,
        ValueName: PCWSTR,
        Type: ULONG,
        Buffer: PVOID,
        BufferSize: ULONG,
        ReturnedLength: PULONG,
        Wow64: BOOLEAN,
    ) -> NTSTATUS;
}}
UNION!{union DELAYLOAD_PROC_DESCRIPTOR_Description {
    Name: PCSTR,
    Ordinal: ULONG,
}}
STRUCT!{struct DELAYLOAD_PROC_DESCRIPTOR {
    ImportDescribedByName: ULONG,
    Description: DELAYLOAD_PROC_DESCRIPTOR_Description,
}}
pub type PDELAYLOAD_PROC_DESCRIPTOR = *mut DELAYLOAD_PROC_DESCRIPTOR;
STRUCT!{struct DELAYLOAD_INFO {
    Size: ULONG,
    DelayloadDescriptor: PCIMAGE_DELAYLOAD_DESCRIPTOR,
    ThunkAddress: PIMAGE_THUNK_DATA,
    TargetDllName: PCSTR,
    TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    TargetModuleBase: PVOID,
    Unused: PVOID,
    LastError: ULONG,
}}
pub type PDELAYLOAD_INFO = *mut DELAYLOAD_INFO;
FN!{stdcall PDELAYLOAD_FAILURE_DLL_CALLBACK(
    NotificationReason: ULONG,
    DelayloadInfo: PDELAYLOAD_INFO,
) -> PVOID}
FN!{stdcall PDELAYLOAD_FAILURE_SYSTEM_ROUTINE(
    DllName: PCSTR,
    ProcName: PCSTR,
) -> PVOID}
EXTERN!{extern "system" {
    fn LdrResolveDelayLoadedAPI(
        ParentModuleBase: PVOID,
        DelayloadDescriptor: PCIMAGE_DELAYLOAD_DESCRIPTOR,
        FailureDllHook: PDELAYLOAD_FAILURE_DLL_CALLBACK,
        FailureSystemHook: PDELAYLOAD_FAILURE_SYSTEM_ROUTINE,
        ThunkAddress: PIMAGE_THUNK_DATA,
        Flags: ULONG,
    ) -> PVOID;
    fn LdrResolveDelayLoadsFromDll(
        ParentBase: PVOID,
        TargetDllName: PCSTR,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn LdrSetDefaultDllDirectories(
        DirectoryFlags: ULONG,
    ) -> NTSTATUS;
    fn LdrShutdownProcess() -> NTSTATUS;
    fn LdrShutdownThread() -> NTSTATUS;
    fn LdrSetImplicitPathOptions(
        ImplicitPathOptions: ULONG,
    ) -> NTSTATUS;
    fn LdrControlFlowGuardEnforced() -> BOOLEAN;
}}
