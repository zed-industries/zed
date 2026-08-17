use crate::winapi_local::um::winnt::PMEM_EXTENDED_PARAMETER;
use winapi::shared::basetsd::{PSIZE_T, PULONG_PTR, SIZE_T, ULONG_PTR};
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, LARGE_INTEGER, NTSTATUS, PHANDLE, PLARGE_INTEGER, POBJECT_ATTRIBUTES, PULONG,
    PUNICODE_STRING, PVOID, UCHAR, ULONG, ULONGLONG, UNICODE_STRING, USHORT,
};
use winapi::um::winnt::{
    ACCESS_MASK, PCFG_CALL_TARGET_INFO, STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE,
};
ENUM!{enum MEMORY_INFORMATION_CLASS {
    MemoryBasicInformation = 0,
    MemoryWorkingSetInformation = 1,
    MemoryMappedFilenameInformation = 2,
    MemoryRegionInformation = 3,
    MemoryWorkingSetExInformation = 4,
    MemorySharedCommitInformation = 5,
    MemoryImageInformation = 6,
    MemoryRegionInformationEx = 7,
    MemoryPrivilegedBasicInformation = 8,
    MemoryEnclaveImageInformation = 9,
    MemoryBasicInformationCapped = 10,
}}
STRUCT!{struct MEMORY_WORKING_SET_BLOCK {
    Bitfields: ULONG_PTR,
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
BITFIELD!{MEMORY_WORKING_SET_BLOCK Bitfields: ULONG_PTR [
    Protection set_Protection[0..5],
    ShareCount set_ShareCount[5..8],
    Shared set_Shared[8..9],
    Node set_Node[9..12],
    VirtualPage set_VirtualPage[12..64],
]}
#[cfg(target_arch = "x86")]
BITFIELD!{MEMORY_WORKING_SET_BLOCK Bitfields: ULONG_PTR [
    Protection set_Protection[0..5],
    ShareCount set_ShareCount[5..8],
    Shared set_Shared[8..9],
    Node set_Node[9..12],
    VirtualPage set_VirtualPage[12..32],
]}
pub type PMEMORY_WORKING_SET_BLOCK = *mut MEMORY_WORKING_SET_BLOCK;
STRUCT!{struct MEMORY_WORKING_SET_INFORMATION {
    NumberOfEntries: ULONG_PTR,
    WorkingSetInfo: [MEMORY_WORKING_SET_BLOCK; 1],
}}
pub type PMEMORY_WORKING_SET_INFORMATION = *mut MEMORY_WORKING_SET_INFORMATION;
STRUCT!{struct MEMORY_REGION_INFORMATION {
    AllocationBase: PVOID,
    AllocationProtect: ULONG,
    RegionType: ULONG,
    RegionSize: SIZE_T,
    CommitSize: SIZE_T,
}}
BITFIELD!{MEMORY_REGION_INFORMATION RegionType: ULONG [
    Private set_Private[0..1],
    MappedDataFile set_MappedDataFile[1..2],
    MappedImage set_MappedImage[2..3],
    MappedPageFile set_MappedPageFile[3..4],
    MappedPhysical set_MappedPhysical[4..5],
    DirectMapped set_DirectMapped[5..6],
    SoftwareEnclave set_SoftwareEnclave[6..7],
    PageSize64K set_PageSize64K[7..8],
    PlaceholderReservation set_PlaceholderReservation[8..9],
    Reserved set_Reserved[9..32],
]}
pub type PMEMORY_REGION_INFORMATION = *mut MEMORY_REGION_INFORMATION;
ENUM!{enum MEMORY_WORKING_SET_EX_LOCATION {
    MemoryLocationInvalid = 0,
    MemoryLocationResident = 1,
    MemoryLocationPagefile = 2,
    MemoryLocationReserved = 3,
}}
UNION!{union MEMORY_WORKING_SET_EX_BLOCK_u {
    Bitfields: ULONG_PTR,
    Invalid: ULONG_PTR,
}}
STRUCT!{struct MEMORY_WORKING_SET_EX_BLOCK {
    u: MEMORY_WORKING_SET_EX_BLOCK_u,
}}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
BITFIELD!{unsafe MEMORY_WORKING_SET_EX_BLOCK_u Bitfields: ULONG_PTR [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Priority set_Priority[24..27],
    Reserved set_Reserved[27..30],
    SharedOriginal set_SharedOriginal[30..31],
    Bad set_Bad[31..32],
    ReservedUlong set_ReservedUlong[32..64],
]}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
BITFIELD!{unsafe MEMORY_WORKING_SET_EX_BLOCK_u Invalid: ULONG_PTR [
    Invalid_Valid set_Invalid_Valid[0..1],
    Invalid_Reserved0 set_Invalid_Reserved0[1..15],
    Invalid_Shared set_Invalid_Shared[15..16],
    Invalid_Reserved1 set_Invalid_Reserved1[16..21],
    Invalid_PageTable set_Invalid_PageTable[21..22],
    Invalid_Location set_Invalid_Location[22..24],
    Invalid_Priority set_Invalid_Priority[24..27],
    Invalid_ModifiedList set_Invalid_ModifiedList[27..28],
    Invalid_Reserved2 set_Invalid_Reserved2[28..30],
    Invalid_SharedOriginal set_Invalid_SharedOriginal[30..31],
    Invalid_Bad set_Invalid_Bad[31..32],
    Invalid_ReservedUlong set_Invalid_ReservedUlong[32..64],
]}
#[cfg(target_arch = "x86")]
BITFIELD!{unsafe MEMORY_WORKING_SET_EX_BLOCK_u Bitfields: ULONG_PTR [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Priority set_Priority[24..27],
    Reserved set_Reserved[27..30],
    SharedOriginal set_SharedOriginal[30..31],
    Bad set_Bad[31..32],
]}
#[cfg(target_arch = "x86")]
BITFIELD!{unsafe MEMORY_WORKING_SET_EX_BLOCK_u Invalid: ULONG_PTR [
    Invalid_Valid set_Invalid_Valid[0..1],
    Invalid_Reserved0 set_Invalid_Reserved0[1..15],
    Invalid_Shared set_Invalid_Shared[15..16],
    Invalid_Reserved1 set_Invalid_Reserved1[16..21],
    Invalid_PageTable set_Invalid_PageTable[21..22],
    Invalid_Location set_Invalid_Location[22..24],
    Invalid_Priority set_Invalid_Priority[24..27],
    Invalid_ModifiedList set_Invalid_ModifiedList[27..28],
    Invalid_Reserved2 set_Invalid_Reserved2[28..30],
    Invalid_SharedOriginal set_Invalid_SharedOriginal[30..31],
    Invalid_Bad set_Invalid_Bad[31..32],
]}
pub type PMEMORY_WORKING_SET_EX_BLOCK = *mut MEMORY_WORKING_SET_EX_BLOCK;
STRUCT!{struct MEMORY_WORKING_SET_EX_INFORMATION {
    VirtualAddress: PVOID,
    VirtualAttributes: MEMORY_WORKING_SET_EX_BLOCK,
}}
pub type PMEMORY_WORKING_SET_EX_INFORMATION = *mut MEMORY_WORKING_SET_EX_INFORMATION;
STRUCT!{struct MEMORY_SHARED_COMMIT_INFORMATION {
    CommitSize: SIZE_T,
}}
pub type PMEMORY_SHARED_COMMIT_INFORMATION = *mut MEMORY_SHARED_COMMIT_INFORMATION;
STRUCT!{struct MEMORY_IMAGE_INFORMATION {
    ImageBase: PVOID,
    SizeOfImage: SIZE_T,
    ImageFlags: ULONG,
}}
BITFIELD!{MEMORY_IMAGE_INFORMATION ImageFlags: ULONG [
    ImagePartialMap set_ImagePartialMap[0..1],
    ImageNotExecutable set_ImageNotExecutable[1..2],
    ImageSigningLevel set_ImageSigningLevel[2..6],
    Reserved set_Reserved[6..32],
]}
pub type PMEMORY_IMAGE_INFORMATION = *mut MEMORY_IMAGE_INFORMATION;
STRUCT!{struct MEMORY_ENCLAVE_IMAGE_INFORMATION {
    ImageInfo: MEMORY_IMAGE_INFORMATION,
    UniqueID: [UCHAR; 32],
    AuthorID: [UCHAR; 32],
}}
pub type PMEMORY_ENCLAVE_IMAGE_INFORMATION = *mut MEMORY_ENCLAVE_IMAGE_INFORMATION;
pub const MMPFNLIST_ZERO: u32 = 0;
pub const MMPFNLIST_FREE: u32 = 1;
pub const MMPFNLIST_STANDBY: u32 = 2;
pub const MMPFNLIST_MODIFIED: u32 = 3;
pub const MMPFNLIST_MODIFIEDNOWRITE: u32 = 4;
pub const MMPFNLIST_BAD: u32 = 5;
pub const MMPFNLIST_ACTIVE: u32 = 6;
pub const MMPFNLIST_TRANSITION: u32 = 7;
pub const MMPFNUSE_PROCESSPRIVATE: u32 = 0;
pub const MMPFNUSE_FILE: u32 = 1;
pub const MMPFNUSE_PAGEFILEMAPPED: u32 = 2;
pub const MMPFNUSE_PAGETABLE: u32 = 3;
pub const MMPFNUSE_PAGEDPOOL: u32 = 4;
pub const MMPFNUSE_NONPAGEDPOOL: u32 = 5;
pub const MMPFNUSE_SYSTEMPTE: u32 = 6;
pub const MMPFNUSE_SESSIONPRIVATE: u32 = 7;
pub const MMPFNUSE_METAFILE: u32 = 8;
pub const MMPFNUSE_AWEPAGE: u32 = 9;
pub const MMPFNUSE_DRIVERLOCKPAGE: u32 = 10;
pub const MMPFNUSE_KERNELSTACK: u32 = 11;
STRUCT!{struct MEMORY_FRAME_INFORMATION {
    Bitfields: ULONGLONG,
}}
BITFIELD!{MEMORY_FRAME_INFORMATION Bitfields: ULONGLONG [
    UseDescription set_UseDescription[0..4],
    ListDescription set_ListDescription[4..7],
    Reserved0 set_Reserved0[7..8],
    Pinned set_Pinned[8..9],
    DontUse set_DontUse[9..57],
    Priority set_Priority[57..60],
    Reserved set_Reserved[60..64],
]}
STRUCT!{struct FILEOFFSET_INFORMATION {
    Bitfields: ULONGLONG,
}}
BITFIELD!{FILEOFFSET_INFORMATION Bitfields: ULONGLONG [
    DontUse set_DontUse[0..9],
    Offset set_Offset[9..57],
    Reserved set_Reserved[57..64],
]}
STRUCT!{struct PAGEDIR_INFORMATION {
    Bitfields: ULONGLONG,
}}
BITFIELD!{PAGEDIR_INFORMATION Bitfields: ULONGLONG [
    DontUse set_DontUse[0..9],
    PageDirectoryBase set_PageDirectoryBase[9..57],
    Reserved set_Reserved[57..64],
]}
STRUCT!{struct UNIQUE_PROCESS_INFORMATION {
    Bitfields: ULONGLONG,
}}
BITFIELD!{UNIQUE_PROCESS_INFORMATION Bitfields: ULONGLONG [
    DontUse set_DontUse[0..9],
    UniqueProcessKey set_UniqueProcessKey[9..57],
    Reserved set_Reserved[57..64],
]}
pub type PUNIQUE_PROCESS_INFORMATION = *mut UNIQUE_PROCESS_INFORMATION;
UNION!{union MMPFN_IDENTITY_u1 {
    e1: MEMORY_FRAME_INFORMATION,
    e2: FILEOFFSET_INFORMATION,
    e3: PAGEDIR_INFORMATION,
    e4: UNIQUE_PROCESS_INFORMATION,
}}
UNION!{union MMPFN_IDENTITY_u2 {
    e1: ULONG_PTR,
    e2_CombinedPage: ULONG_PTR,
    FileObject: ULONG_PTR,
    UniqueFileObjectKey: ULONG_PTR,
    ProtoPteAddress: ULONG_PTR,
    VirtualAddress: ULONG_PTR,
}}
STRUCT!{struct MMPFN_IDENTITY {
    u1: MMPFN_IDENTITY_u1,
    PageFrameIndex: ULONG_PTR,
    u2: MMPFN_IDENTITY_u2,
}}
BITFIELD!{unsafe MMPFN_IDENTITY_u2 e1: ULONG_PTR [
    Image set_Image[0..1],
    Mismatch set_Mismatch[1..2],
]}
pub type PMMPFN_IDENTITY = *mut MMPFN_IDENTITY;
STRUCT!{struct MMPFN_MEMSNAP_INFORMATION {
    InitialPageFrameIndex: ULONG_PTR,
    Count: ULONG_PTR,
}}
pub type PMMPFN_MEMSNAP_INFORMATION = *mut MMPFN_MEMSNAP_INFORMATION;
ENUM!{enum SECTION_INFORMATION_CLASS {
    SectionBasicInformation = 0,
    SectionImageInformation = 1,
    SectionRelocationInformation = 2,
    SectionOriginalBaseInformation = 3,
    SectionInternalImageInformation = 4,
    MaxSectionInfoClass = 5,
}}
STRUCT!{struct SECTION_BASIC_INFORMATION {
    BaseAddress: PVOID,
    AllocationAttributes: ULONG,
    MaximumSize: LARGE_INTEGER,
}}
pub type PSECTION_BASIC_INFORMATION = *mut SECTION_BASIC_INFORMATION;
STRUCT!{struct SECTION_IMAGE_INFORMATION_u1_s {
    SubSystemMinorVersion: USHORT,
    SubSystemMajorVersion: USHORT,
}}
UNION!{union SECTION_IMAGE_INFORMATION_u1 {
    s: SECTION_IMAGE_INFORMATION_u1_s,
    SubSystemVersion: ULONG,
}}
STRUCT!{struct SECTION_IMAGE_INFORMATION_u2_s {
    MajorOperatingSystemVersion: USHORT,
    MinorOperatingSystemVersion: USHORT,
}}
UNION!{union SECTION_IMAGE_INFORMATION_u2 {
    s: SECTION_IMAGE_INFORMATION_u2_s,
    OperatingSystemVersion: ULONG,
}}
STRUCT!{struct SECTION_IMAGE_INFORMATION {
    TransferAddress: PVOID,
    ZeroBits: ULONG,
    MaximumStackSize: SIZE_T,
    CommittedStackSize: SIZE_T,
    SubSystemType: ULONG,
    u1: SECTION_IMAGE_INFORMATION_u1,
    u2: SECTION_IMAGE_INFORMATION_u2,
    ImageCharacteristics: USHORT,
    DllCharacteristics: USHORT,
    Machine: USHORT,
    ImageContainsCode: BOOLEAN,
    ImageFlags: UCHAR,
    LoaderFlags: ULONG,
    ImageFileSize: ULONG,
    CheckSum: ULONG,
}}
BITFIELD!{SECTION_IMAGE_INFORMATION ImageFlags: UCHAR [
    ComPlusNativeReady set_ComPlusNativeReady[0..1],
    ComPlusILOnly set_ComPlusILOnly[1..2],
    ImageDynamicallyRelocated set_ImageDynamicallyRelocated[2..3],
    ImageMappedFlat set_ImageMappedFlat[3..4],
    BaseBelow4gb set_BaseBelow4gb[4..5],
    ComPlusPrefer32bit set_ComPlusPrefer32bit[5..6],
    Reserved set_Reserved[6..8],
]}
pub type PSECTION_IMAGE_INFORMATION = *mut SECTION_IMAGE_INFORMATION;
STRUCT!{struct SECTION_INTERNAL_IMAGE_INFORMATION {
    SectionInformation: SECTION_IMAGE_INFORMATION,
    ExtendedFlags: ULONG,
}}
BITFIELD!{SECTION_INTERNAL_IMAGE_INFORMATION ExtendedFlags: ULONG [
    ImageExportSuppressionEnabled set_ImageExportSuppressionEnabled[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PSECTION_INTERNAL_IMAGE_INFORMATION = *mut SECTION_INTERNAL_IMAGE_INFORMATION;
ENUM!{enum SECTION_INHERIT {
    ViewShare = 1,
    ViewUnmap = 2,
}}
pub const SEC_BASED: u32 = 0x200000;
pub const SEC_NO_CHANGE: u32 = 0x400000;
pub const SEC_GLOBAL: u32 = 0x20000000;
pub const MEM_EXECUTE_OPTION_DISABLE: u32 = 0x1;
pub const MEM_EXECUTE_OPTION_ENABLE: u32 = 0x2;
pub const MEM_EXECUTE_OPTION_DISABLE_THUNK_EMULATION: u32 = 0x4;
pub const MEM_EXECUTE_OPTION_PERMANENT: u32 = 0x8;
pub const MEM_EXECUTE_OPTION_EXECUTE_DISPATCH_ENABLE: u32 = 0x10;
pub const MEM_EXECUTE_OPTION_IMAGE_DISPATCH_ENABLE: u32 = 0x20;
pub const MEM_EXECUTE_OPTION_VALID_FLAGS: u32 = 0x3f;
EXTERN!{extern "system" {
    fn NtAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        RegionSize: PSIZE_T,
        AllocationType: ULONG,
        Protect: ULONG,
    ) -> NTSTATUS;
    fn NtFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        FreeType: ULONG,
    ) -> NTSTATUS;
    fn NtReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesRead: PSIZE_T,
    ) -> NTSTATUS;
    fn NtWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesWritten: PSIZE_T,
    ) -> NTSTATUS;
    fn NtProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        NewProtect: ULONG,
        OldProtect: PULONG,
    ) -> NTSTATUS;
    fn NtQueryVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: PVOID,
        MemoryInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
}}
ENUM!{enum VIRTUAL_MEMORY_INFORMATION_CLASS {
    VmPrefetchInformation = 0,
    VmPagePriorityInformation = 1,
    VmCfgCallTargetInformation = 2,
    VmPageDirtyStateInformation = 3,
}}
STRUCT!{struct MEMORY_RANGE_ENTRY {
    VirtualAddress: PVOID,
    NumberOfBytes: SIZE_T,
}}
pub type PMEMORY_RANGE_ENTRY = *mut MEMORY_RANGE_ENTRY;
STRUCT!{struct CFG_CALL_TARGET_LIST_INFORMATION {
    NumberOfEntries: ULONG,
    Reserved: ULONG,
    NumberOfEntriesProcessed: PULONG,
    CallTargetInfo: PCFG_CALL_TARGET_INFO,
    Section: PVOID,
    FileOffset: ULONGLONG,
}}
pub type PCFG_CALL_TARGET_LIST_INFORMATION = *mut CFG_CALL_TARGET_LIST_INFORMATION;
EXTERN!{extern "system" {
    fn NtSetInformationVirtualMemory(
        ProcessHandle: HANDLE,
        VmInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS,
        NumberOfEntries: ULONG_PTR,
        VirtualAddresses: PMEMORY_RANGE_ENTRY,
        VmInformation: PVOID,
        VmInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;
    fn NtUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;
    fn NtCreateSection(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaximumSize: PLARGE_INTEGER,
        SectionPageProtection: ULONG,
        AllocationAttributes: ULONG,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCreateSectionEx(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaximumSize: PLARGE_INTEGER,
        SectionPageProtection: ULONG,
        AllocationAttributes: ULONG,
        FileHandle: HANDLE,
        ExtendedParameters: PMEM_EXTENDED_PARAMETER,
        ExtendedParameterCount: ULONG,
    ) -> NTSTATUS;
    fn NtOpenSection(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtMapViewOfSection(
        SectionHandle: HANDLE,
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        CommitSize: SIZE_T,
        SectionOffset: PLARGE_INTEGER,
        ViewSize: PSIZE_T,
        InheritDisposition: SECTION_INHERIT,
        AllocationType: ULONG,
        Win32Protect: ULONG,
    ) -> NTSTATUS;
    fn NtUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
    ) -> NTSTATUS;
    fn NtUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtExtendSection(
        SectionHandle: HANDLE,
        NewSectionSize: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: PVOID,
        SectionInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
    fn NtAreMappedFilesTheSame(
        File1MappedAsAnImage: PVOID,
        File2MappedAsFile: PVOID,
    ) -> NTSTATUS;
}}
pub const MEMORY_PARTITION_QUERY_ACCESS: u32 = 0x0001;
pub const MEMORY_PARTITION_MODIFY_ACCESS: u32 = 0x0002;
pub const MEMORY_PARTITION_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE
    | MEMORY_PARTITION_QUERY_ACCESS | MEMORY_PARTITION_MODIFY_ACCESS;
ENUM!{enum MEMORY_PARTITION_INFORMATION_CLASS {
    SystemMemoryPartitionInformation = 0,
    SystemMemoryPartitionMoveMemory = 1,
    SystemMemoryPartitionAddPagefile = 2,
    SystemMemoryPartitionCombineMemory = 3,
    SystemMemoryPartitionInitialAddMemory = 4,
    SystemMemoryPartitionGetMemoryEvents = 5,
    SystemMemoryPartitionMax = 6,
}}
STRUCT!{struct MEMORY_PARTITION_CONFIGURATION_INFORMATION {
    Flags: ULONG,
    NumaNode: ULONG,
    Channel: ULONG,
    NumberOfNumaNodes: ULONG,
    ResidentAvailablePages: ULONG_PTR,
    CommittedPages: ULONG_PTR,
    CommitLimit: ULONG_PTR,
    PeakCommitment: ULONG_PTR,
    TotalNumberOfPages: ULONG_PTR,
    AvailablePages: ULONG_PTR,
    ZeroPages: ULONG_PTR,
    FreePages: ULONG_PTR,
    StandbyPages: ULONG_PTR,
    StandbyPageCountByPriority: [ULONG_PTR; 8],
    RepurposedPagesByPriority: [ULONG_PTR; 8],
    MaximumCommitLimit: ULONG_PTR,
    DonatedPagesToPartitions: ULONG_PTR,
    PartitionId: ULONG,
}}
pub type PMEMORY_PARTITION_CONFIGURATION_INFORMATION =
    *mut MEMORY_PARTITION_CONFIGURATION_INFORMATION;
STRUCT!{struct MEMORY_PARTITION_TRANSFER_INFORMATION {
    NumberOfPages: ULONG_PTR,
    NumaNode: ULONG,
    Flags: ULONG,
}}
pub type PMEMORY_PARTITION_TRANSFER_INFORMATION = *mut MEMORY_PARTITION_TRANSFER_INFORMATION;
STRUCT!{struct MEMORY_PARTITION_PAGEFILE_INFORMATION {
    PageFileName: UNICODE_STRING,
    MinimumSize: LARGE_INTEGER,
    MaximumSize: LARGE_INTEGER,
    Flags: ULONG,
}}
pub type PMEMORY_PARTITION_PAGEFILE_INFORMATION = *mut MEMORY_PARTITION_PAGEFILE_INFORMATION;
STRUCT!{struct MEMORY_PARTITION_PAGE_COMBINE_INFORMATION {
    StopHandle: HANDLE,
    Flags: ULONG,
    TotalNumberOfPages: ULONG_PTR,
}}
pub type PMEMORY_PARTITION_PAGE_COMBINE_INFORMATION =
    *mut MEMORY_PARTITION_PAGE_COMBINE_INFORMATION;
STRUCT!{struct MEMORY_PARTITION_PAGE_RANGE {
    StartPage: ULONG_PTR,
    NumberOfPages: ULONG_PTR,
}}
pub type PMEMORY_PARTITION_PAGE_RANGE = *mut MEMORY_PARTITION_PAGE_RANGE;
STRUCT!{struct MEMORY_PARTITION_INITIAL_ADD_INFORMATION {
    Flags: ULONG,
    NumberOfRanges: ULONG,
    NumberOfPagesAdded: ULONG_PTR,
    PartitionRanges: [MEMORY_PARTITION_PAGE_RANGE; 1],
}}
pub type PMEMORY_PARTITION_INITIAL_ADD_INFORMATION = *mut MEMORY_PARTITION_INITIAL_ADD_INFORMATION;
STRUCT!{struct MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION {
    Flags: ULONG,
    HandleAttributes: ULONG,
    DesiredAccess: ULONG,
    LowCommitCondition: HANDLE,
    HighCommitCondition: HANDLE,
    MaximumCommitCondition: HANDLE,
}}
BITFIELD!{MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION Flags: ULONG [
    CommitEvents set_CommitEvents[0..1],
    Spare set_Spare[1..32],
]}
pub type PMEMORY_PARTITION_MEMORY_EVENTS_INFORMATION =
    *mut MEMORY_PARTITION_MEMORY_EVENTS_INFORMATION;
EXTERN!{extern "system" {
    fn NtCreatePartition(
        PartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PreferredNode: ULONG,
    ) -> NTSTATUS;
    fn NtOpenPartition(
        PartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtManagePartition(
        PartitionInformationClass: MEMORY_PARTITION_INFORMATION_CLASS,
        PartitionInformation: PVOID,
        PartitionInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtMapUserPhysicalPages(
        VirtualAddress: PVOID,
        NumberOfPages: ULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn NtMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut PVOID,
        NumberOfPages: ULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn NtAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: PULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn NtFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: PULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn NtOpenSession(
        SessionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: ULONG,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
        UserAddressArray: *mut PVOID,
        EntriesInUserAddressArray: PULONG_PTR,
        Granularity: PULONG,
    ) -> NTSTATUS;
    fn NtResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
    ) -> NTSTATUS;
    fn NtCreatePagingFile(
        PageFileName: PUNICODE_STRING,
        MinimumSize: PLARGE_INTEGER,
        MaximumSize: PLARGE_INTEGER,
        Priority: ULONG,
    ) -> NTSTATUS;
    fn NtFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Length: SIZE_T,
    ) -> NTSTATUS;
    fn NtFlushWriteBuffer() -> NTSTATUS;
}}
