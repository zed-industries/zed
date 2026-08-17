use core::mem::MaybeUninit;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use core::ptr::addr_of;
use core::ptr::read_volatile;
#[cfg(target_arch = "x86")]
use core::hint::spin_loop;
use crate::ntapi_base::{CLIENT_ID, KPRIORITY, KSYSTEM_TIME, PRTL_ATOM, RTL_ATOM};
use crate::ntioapi::{BUS_DATA_TYPE, FILE_IO_COMPLETION_INFORMATION, INTERFACE_TYPE};
use crate::ntkeapi::{KPROFILE_SOURCE, KTHREAD_STATE, KWAIT_REASON};
use crate::ntldr::RTL_PROCESS_MODULE_INFORMATION_EX;
use crate::ntpebteb::PTEB;
use crate::ntpoapi::COUNTED_REASON_CONTEXT;
use winapi::shared::basetsd::{KAFFINITY, PULONG64, PULONG_PTR, SIZE_T, ULONG64, ULONG_PTR};
use winapi::shared::evntrace::PROFILE_SOURCE_INFO;
use winapi::shared::guiddef::{GUID, LPGUID};
use winapi::shared::ntdef::{
    BOOLEAN, CCHAR, EVENT_TYPE, HANDLE, LANGID, LARGE_INTEGER, LCID, LOGICAL, LONG, LONGLONG,
    NTSTATUS, NT_PRODUCT_TYPE, PBOOLEAN, PCHAR, PCWNF_STATE_NAME, PGROUP_AFFINITY, PHANDLE,
    PHYSICAL_ADDRESS, PLARGE_INTEGER, PLCID, PLONG, PLUID, POBJECT_ATTRIBUTES, PUCHAR,
    PULARGE_INTEGER, PULONG, PUNICODE_STRING, PUSHORT, PVOID, PWNF_STATE_NAME, PWSTR, TIMER_TYPE,
    UCHAR, ULARGE_INTEGER, ULONG, ULONGLONG, UNICODE_STRING, USHORT, VOID, WCHAR, WNF_STATE_NAME,
};
use winapi::um::winnt::{
    ACCESS_MASK, ANYSIZE_ARRAY, FIRMWARE_TYPE, GENERIC_MAPPING, PSECURITY_DESCRIPTOR,
    STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE, XSTATE_CONFIGURATION,
};
use crate::winapi_local::um::winnt::UInt32x32To64;
EXTERN!{extern "system" {
    fn NtDelayExecution(
        Alertable: BOOLEAN,
        DelayInterval: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtQuerySystemEnvironmentValue(
        VariableName: PUNICODE_STRING,
        VariableValue: PWSTR,
        ValueLength: USHORT,
        ReturnLength: PUSHORT,
    ) -> NTSTATUS;
    fn NtSetSystemEnvironmentValue(
        VariableName: PUNICODE_STRING,
        VariableValue: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn NtQuerySystemEnvironmentValueEx(
        VariableName: PUNICODE_STRING,
        VendorGuid: LPGUID,
        Value: PVOID,
        ValueLength: PULONG,
        Attributes: PULONG,
    ) -> NTSTATUS;
    fn NtSetSystemEnvironmentValueEx(
        VariableName: PUNICODE_STRING,
        VendorGuid: LPGUID,
        Value: PVOID,
        ValueLength: ULONG,
        Attributes: ULONG,
    ) -> NTSTATUS;
    fn NtEnumerateSystemEnvironmentValuesEx(
        InformationClass: ULONG,
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;
}}
STRUCT!{struct BOOT_ENTRY {
    Version: ULONG,
    Length: ULONG,
    Id: ULONG,
    Attributes: ULONG,
    FriendlyNameOffset: ULONG,
    BootFilePathOffset: ULONG,
    OsOptionsLength: ULONG,
    OsOptions: [UCHAR; 1],
}}
pub type PBOOT_ENTRY = *mut BOOT_ENTRY;
STRUCT!{struct BOOT_ENTRY_LIST {
    NextEntryOffset: ULONG,
    BootEntry: BOOT_ENTRY,
}}
pub type PBOOT_ENTRY_LIST = *mut BOOT_ENTRY_LIST;
STRUCT!{struct BOOT_OPTIONS {
    Version: ULONG,
    Length: ULONG,
    Timeout: ULONG,
    CurrentBootEntryId: ULONG,
    NextBootEntryId: ULONG,
    HeadlessRedirection: [WCHAR; 1],
}}
pub type PBOOT_OPTIONS = *mut BOOT_OPTIONS;
STRUCT!{struct FILE_PATH {
    Version: ULONG,
    Length: ULONG,
    Type: ULONG,
    FilePath: [UCHAR; 1],
}}
pub type PFILE_PATH = *mut FILE_PATH;
STRUCT!{struct EFI_DRIVER_ENTRY {
    Version: ULONG,
    Length: ULONG,
    Id: ULONG,
    FriendlyNameOffset: ULONG,
    DriverFilePathOffset: ULONG,
}}
pub type PEFI_DRIVER_ENTRY = *mut EFI_DRIVER_ENTRY;
STRUCT!{struct EFI_DRIVER_ENTRY_LIST {
    NextEntryOffset: ULONG,
    DriverEntry: EFI_DRIVER_ENTRY,
}}
pub type PEFI_DRIVER_ENTRY_LIST = *mut EFI_DRIVER_ENTRY_LIST;
EXTERN!{extern "system" {
    fn NtAddBootEntry(
        BootEntry: PBOOT_ENTRY,
        Id: PULONG,
    ) -> NTSTATUS;
    fn NtDeleteBootEntry(
        Id: ULONG,
    ) -> NTSTATUS;
    fn NtModifyBootEntry(
        BootEntry: PBOOT_ENTRY,
    ) -> NTSTATUS;
    fn NtEnumerateBootEntries(
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;
    fn NtQueryBootEntryOrder(
        Ids: PULONG,
        Count: PULONG,
    ) -> NTSTATUS;
    fn NtSetBootEntryOrder(
        Ids: PULONG,
        Count: ULONG,
    ) -> NTSTATUS;
    fn NtQueryBootOptions(
        BootOptions: PBOOT_OPTIONS,
        BootOptionsLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetBootOptions(
        BootOptions: PBOOT_OPTIONS,
        FieldsToChange: ULONG,
    ) -> NTSTATUS;
    fn NtTranslateFilePath(
        InputFilePath: PFILE_PATH,
        OutputType: ULONG,
        OutputFilePath: PFILE_PATH,
        OutputFilePathLength: PULONG,
    ) -> NTSTATUS;
    fn NtAddDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
        Id: PULONG,
    ) -> NTSTATUS;
    fn NtDeleteDriverEntry(
        Id: ULONG,
    ) -> NTSTATUS;
    fn NtModifyDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
    ) -> NTSTATUS;
    fn NtEnumerateDriverEntries(
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;
    fn NtQueryDriverEntryOrder(
        Ids: PULONG,
        Count: PULONG,
    ) -> NTSTATUS;
    fn NtSetDriverEntryOrder(
        Ids: PULONG,
        Count: ULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum FILTER_BOOT_OPTION_OPERATION {
    FilterBootOptionOperationOpenSystemStore = 0,
    FilterBootOptionOperationSetElement = 1,
    FilterBootOptionOperationDeleteElement = 2,
    FilterBootOptionOperationMax = 3,
}}
EXTERN!{extern "system" {
    fn NtFilterBootOption(
        FilterOperation: FILTER_BOOT_OPTION_OPERATION,
        ObjectType: ULONG,
        ElementType: ULONG,
        Data: PVOID,
        DataSize: ULONG,
    ) -> NTSTATUS;
}}
pub const EVENT_QUERY_STATE: u32 = 0x0001;
ENUM!{enum EVENT_INFORMATION_CLASS {
    EventBasicInformation = 0,
}}
STRUCT!{struct EVENT_BASIC_INFORMATION {
    EventType: EVENT_TYPE,
    EventState: LONG,
}}
pub type PEVENT_BASIC_INFORMATION = *mut EVENT_BASIC_INFORMATION;
EXTERN!{extern "system" {
    fn NtCreateEvent(
        EventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        EventType: EVENT_TYPE,
        InitialState: BOOLEAN,
    ) -> NTSTATUS;
    fn NtOpenEvent(
        EventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSetEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;
    fn NtSetEventBoostPriority(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtClearEvent(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtResetEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;
    fn NtPulseEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;
    fn NtQueryEvent(
        EventHandle: HANDLE,
        EventInformationClass: EVENT_INFORMATION_CLASS,
        EventInformation: PVOID,
        EventInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
pub const EVENT_PAIR_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE;
EXTERN!{extern "system" {
    fn NtCreateEventPair(
        EventPairHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtOpenEventPair(
        EventPairHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSetLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetLowWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSetHighWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
}}
ENUM!{enum MUTANT_INFORMATION_CLASS {
    MutantBasicInformation = 0,
    MutantOwnerInformation = 1,
}}
STRUCT!{struct MUTANT_BASIC_INFORMATION {
    CurrentCount: LONG,
    OwnedByCaller: BOOLEAN,
    AbandonedState: BOOLEAN,
}}
pub type PMUTANT_BASIC_INFORMATION = *mut MUTANT_BASIC_INFORMATION;
STRUCT!{struct MUTANT_OWNER_INFORMATION {
    ClientId: CLIENT_ID,
}}
pub type PMUTANT_OWNER_INFORMATION = *mut MUTANT_OWNER_INFORMATION;
EXTERN!{extern "system" {
    fn NtCreateMutant(
        MutantHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        InitialOwner: BOOLEAN,
    ) -> NTSTATUS;
    fn NtOpenMutant(
        MutantHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtReleaseMutant(
        MutantHandle: HANDLE,
        PreviousCount: PLONG,
    ) -> NTSTATUS;
    fn NtQueryMutant(
        MutantHandle: HANDLE,
        MutantInformationClass: MUTANT_INFORMATION_CLASS,
        MutantInformation: PVOID,
        MutantInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
pub const SEMAPHORE_QUERY_STATE: u32 = 0x0001;
ENUM!{enum SEMAPHORE_INFORMATION_CLASS {
    SemaphoreBasicInformation = 0,
}}
STRUCT!{struct SEMAPHORE_BASIC_INFORMATION {
    CurrentCount: LONG,
    MaximumCount: LONG,
}}
pub type PSEMAPHORE_BASIC_INFORMATION = *mut SEMAPHORE_BASIC_INFORMATION;
EXTERN!{extern "system" {
    fn NtCreateSemaphore(
        SemaphoreHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        InitialCount: LONG,
        MaximumCount: LONG,
    ) -> NTSTATUS;
    fn NtOpenSemaphore(
        SemaphoreHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtReleaseSemaphore(
        SemaphoreHandle: HANDLE,
        ReleaseCount: LONG,
        PreviousCount: PLONG,
    ) -> NTSTATUS;
    fn NtQuerySemaphore(
        SemaphoreHandle: HANDLE,
        SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS,
        SemaphoreInformation: PVOID,
        SemaphoreInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum TIMER_INFORMATION_CLASS {
    TimerBasicInformation = 0,
}}
STRUCT!{struct TIMER_BASIC_INFORMATION {
    RemainingTime: LARGE_INTEGER,
    TimerState: BOOLEAN,
}}
pub type PTIMER_BASIC_INFORMATION = *mut TIMER_BASIC_INFORMATION;
FN!{stdcall PTIMER_APC_ROUTINE(
    TimerContext: PVOID,
    TimerLowValue: ULONG,
    TimerHighValue: LONG,
) -> ()}
ENUM!{enum TIMER_SET_INFORMATION_CLASS {
    TimerSetCoalescableTimer = 0,
    MaxTimerInfoClass = 1,
}}
STRUCT!{struct TIMER_SET_COALESCABLE_TIMER_INFO {
    DueTime: LARGE_INTEGER,
    TimerApcRoutine: PTIMER_APC_ROUTINE,
    TimerContext: PVOID,
    WakeContext: *mut COUNTED_REASON_CONTEXT,
    Period: ULONG,
    TolerableDelay: ULONG,
    PreviousState: PBOOLEAN,
}}
pub type PTIMER_SET_COALESCABLE_TIMER_INFO = *mut TIMER_SET_COALESCABLE_TIMER_INFO;
EXTERN!{extern "system" {
    fn NtCreateTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TimerType: TIMER_TYPE,
    ) -> NTSTATUS;
    fn NtOpenTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSetTimer(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
        TimerApcRoutine: PTIMER_APC_ROUTINE,
        TimerContext: PVOID,
        ResumeTimer: BOOLEAN,
        Period: LONG,
        PreviousState: PBOOLEAN,
    ) -> NTSTATUS;
    fn NtSetTimerEx(
        TimerHandle: HANDLE,
        TimerSetInformationClass: TIMER_SET_INFORMATION_CLASS,
        TimerSetInformation: PVOID,
        TimerSetInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtCancelTimer(
        TimerHandle: HANDLE,
        CurrentState: PBOOLEAN,
    ) -> NTSTATUS;
    fn NtQueryTimer(
        TimerHandle: HANDLE,
        TimerInformationClass: TIMER_INFORMATION_CLASS,
        TimerInformation: PVOID,
        TimerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtCreateIRTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
    ) -> NTSTATUS;
    fn NtSetIRTimer(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
}}
STRUCT!{struct T2_SET_PARAMETERS {
    Version: ULONG,
    Reserved: ULONG,
    NoWakeTolerance: LONGLONG,
}}
pub type PT2_SET_PARAMETERS = *mut T2_SET_PARAMETERS;
pub type PT2_CANCEL_PARAMETERS = PVOID;
EXTERN!{extern "system" {
    fn NtCreateTimer2(
        TimerHandle: PHANDLE,
        Reserved1: PVOID,
        Reserved2: PVOID,
        Attributes: ULONG,
        DesiredAccess: ACCESS_MASK,
    ) -> NTSTATUS;
    fn NtSetTimer2(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
        Period: PLARGE_INTEGER,
        Parameters: PT2_SET_PARAMETERS,
    ) -> NTSTATUS;
    fn NtCancelTimer2(
        TimerHandle: HANDLE,
        Parameters: PT2_CANCEL_PARAMETERS,
    ) -> NTSTATUS;
}}
pub const PROFILE_CONTROL: u32 = 0x0001;
pub const PROFILE_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | PROFILE_CONTROL;
EXTERN!{extern "system" {
    fn NtCreateProfile(
        ProfileHandle: PHANDLE,
        Process: HANDLE,
        ProfileBase: PVOID,
        ProfileSize: SIZE_T,
        BucketSize: ULONG,
        Buffer: PULONG,
        BufferSize: ULONG,
        ProfileSource: KPROFILE_SOURCE,
        Affinity: KAFFINITY,
    ) -> NTSTATUS;
    fn NtCreateProfileEx(
        ProfileHandle: PHANDLE,
        Process: HANDLE,
        ProfileBase: PVOID,
        ProfileSize: SIZE_T,
        BucketSize: ULONG,
        Buffer: PULONG,
        BufferSize: ULONG,
        ProfileSource: KPROFILE_SOURCE,
        GroupCount: USHORT,
        GroupAffinity: PGROUP_AFFINITY,
    ) -> NTSTATUS;
    fn NtStartProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtStopProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQueryIntervalProfile(
        ProfileSource: KPROFILE_SOURCE,
        Interval: PULONG,
    ) -> NTSTATUS;
    fn NtSetIntervalProfile(
        Interval: ULONG,
        Source: KPROFILE_SOURCE,
    ) -> NTSTATUS;
}}
pub const KEYEDEVENT_WAIT: ULONG = 0x0001;
pub const KEYEDEVENT_WAKE: ULONG = 0x0002;
pub const KEYEDEVENT_ALL_ACCESS: ACCESS_MASK =
    STANDARD_RIGHTS_REQUIRED | KEYEDEVENT_WAIT | KEYEDEVENT_WAKE;
EXTERN!{extern "system" {
    fn NtCreateKeyedEvent(
        KeyedEventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtOpenKeyedEvent(
        KeyedEventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtReleaseKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: PVOID,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: PVOID,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtUmsThreadYield(
        SchedulerParam: PVOID,
    ) -> NTSTATUS;
}}
ENUM!{enum WNF_STATE_NAME_LIFETIME {
    WnfWellKnownStateName = 0,
    WnfPermanentStateName = 1,
    WnfPersistentStateName = 2,
    WnfTemporaryStateName = 3,
}}
ENUM!{enum WNF_STATE_NAME_INFORMATION {
    WnfInfoStateNameExist = 0,
    WnfInfoSubscribersPresent = 1,
    WnfInfoIsQuiescent = 2,
}}
ENUM!{enum WNF_DATA_SCOPE {
    WnfDataScopeSystem = 0,
    WnfDataScopeSession = 1,
    WnfDataScopeUser = 2,
    WnfDataScopeProcess = 3,
    WnfDataScopeMachine = 4,
}}
STRUCT!{struct WNF_TYPE_ID {
    TypeId: GUID,
}}
pub type PWNF_TYPE_ID = *mut WNF_TYPE_ID;
pub type PCWNF_TYPE_ID = *const WNF_TYPE_ID;
pub type PWNF_CHANGE_STAMP = *mut ULONG;
pub type WNF_CHANGE_STAMP = ULONG;
STRUCT!{struct WNF_DELIVERY_DESCRIPTOR {
    SubscriptionId: ULONGLONG,
    StateName: WNF_STATE_NAME,
    ChangeStamp: WNF_CHANGE_STAMP,
    StateDataSize: ULONG,
    EventMask: ULONG,
    TypeId: WNF_TYPE_ID,
    StateDataOffset: ULONG,
}}
pub type PWNF_DELIVERY_DESCRIPTOR = *mut WNF_DELIVERY_DESCRIPTOR;
EXTERN!{extern "system" {
    fn NtCreateWnfStateName(
        StateName: PWNF_STATE_NAME,
        NameLifetime: WNF_STATE_NAME_LIFETIME,
        DataScope: WNF_DATA_SCOPE,
        PersistData: BOOLEAN,
        TypeId: PCWNF_TYPE_ID,
        MaximumStateSize: ULONG,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn NtDeleteWnfStateName(
        StateName: PCWNF_STATE_NAME,
    ) -> NTSTATUS;
    fn NtUpdateWnfStateData(
        StateName: PCWNF_STATE_NAME,
        Buffer: *const VOID,
        Length: ULONG,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const VOID,
        MatchingChangeStamp: WNF_CHANGE_STAMP,
        CheckStamp: LOGICAL,
    ) -> NTSTATUS;
    fn NtDeleteWnfStateData(
        StateName: PCWNF_STATE_NAME,
        ExplicitScope: *const VOID,
    ) -> NTSTATUS;
    fn NtQueryWnfStateData(
        StateName: PCWNF_STATE_NAME,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const VOID,
        ChangeStamp: PWNF_CHANGE_STAMP,
        Buffer: PVOID,
        BufferSize: PULONG,
    ) -> NTSTATUS;
    fn NtQueryWnfStateNameInformation(
        StateName: PCWNF_STATE_NAME,
        NameInfoClass: WNF_STATE_NAME_INFORMATION,
        ExplicitScope: *const VOID,
        InfoBuffer: PVOID,
        InfoBufferSize: ULONG,
    ) -> NTSTATUS;
    fn NtSubscribeWnfStateChange(
        StateName: PCWNF_STATE_NAME,
        ChangeStamp: WNF_CHANGE_STAMP,
        EventMask: ULONG,
        SubscriptionId: PULONG64,
    ) -> NTSTATUS;
    fn NtUnsubscribeWnfStateChange(
        StateName: PCWNF_STATE_NAME,
    ) -> NTSTATUS;
    fn NtGetCompleteWnfStateSubscription(
        OldDescriptorStateName: PWNF_STATE_NAME,
        OldSubscriptionId: *mut ULONG64,
        OldDescriptorEventMask: ULONG,
        OldDescriptorStatus: ULONG,
        NewDeliveryDescriptor: PWNF_DELIVERY_DESCRIPTOR,
        DescriptorSize: ULONG,
    ) -> NTSTATUS;
    fn NtSetWnfProcessNotificationEvent(
        NotificationEvent: HANDLE,
    ) -> NTSTATUS;
}}
pub const WORKER_FACTORY_RELEASE_WORKER: u32 = 0x0001;
pub const WORKER_FACTORY_WAIT: u32 = 0x0002;
pub const WORKER_FACTORY_SET_INFORMATION: u32 = 0x0004;
pub const WORKER_FACTORY_QUERY_INFORMATION: u32 = 0x0008;
pub const WORKER_FACTORY_READY_WORKER: u32 = 0x0010;
pub const WORKER_FACTORY_SHUTDOWN: u32 = 0x0020;
pub const WORKER_FACTORY_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED
    | WORKER_FACTORY_RELEASE_WORKER | WORKER_FACTORY_WAIT | WORKER_FACTORY_SET_INFORMATION
    | WORKER_FACTORY_QUERY_INFORMATION | WORKER_FACTORY_READY_WORKER | WORKER_FACTORY_SHUTDOWN;
ENUM!{enum WORKERFACTORYINFOCLASS {
    WorkerFactoryTimeout = 0,
    WorkerFactoryRetryTimeout = 1,
    WorkerFactoryIdleTimeout = 2,
    WorkerFactoryBindingCount = 3,
    WorkerFactoryThreadMinimum = 4,
    WorkerFactoryThreadMaximum = 5,
    WorkerFactoryPaused = 6,
    WorkerFactoryBasicInformation = 7,
    WorkerFactoryAdjustThreadGoal = 8,
    WorkerFactoryCallbackType = 9,
    WorkerFactoryStackInformation = 10,
    WorkerFactoryThreadBasePriority = 11,
    WorkerFactoryTimeoutWaiters = 12,
    WorkerFactoryFlags = 13,
    WorkerFactoryThreadSoftMaximum = 14,
    MaxWorkerFactoryInfoClass = 15,
}}
pub type PWORKERFACTORYINFOCLASS = *mut WORKERFACTORYINFOCLASS;
STRUCT!{struct WORKER_FACTORY_BASIC_INFORMATION {
    Timeout: LARGE_INTEGER,
    RetryTimeout: LARGE_INTEGER,
    IdleTimeout: LARGE_INTEGER,
    Paused: BOOLEAN,
    TimerSet: BOOLEAN,
    QueuedToExWorker: BOOLEAN,
    MayCreate: BOOLEAN,
    CreateInProgress: BOOLEAN,
    InsertedIntoQueue: BOOLEAN,
    Shutdown: BOOLEAN,
    BindingCount: ULONG,
    ThreadMinimum: ULONG,
    ThreadMaximum: ULONG,
    PendingWorkerCount: ULONG,
    WaitingWorkerCount: ULONG,
    TotalWorkerCount: ULONG,
    ReleaseCount: ULONG,
    InfiniteWaitGoal: LONGLONG,
    StartRoutine: PVOID,
    StartParameter: PVOID,
    ProcessId: HANDLE,
    StackReserve: SIZE_T,
    StackCommit: SIZE_T,
    LastThreadCreationStatus: NTSTATUS,
}}
pub type PWORKER_FACTORY_BASIC_INFORMATION = *mut WORKER_FACTORY_BASIC_INFORMATION;
EXTERN!{extern "system" {
    fn NtCreateWorkerFactory(
        WorkerFactoryHandleReturn: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CompletionPortHandle: HANDLE,
        WorkerProcessHandle: HANDLE,
        StartRoutine: PVOID,
        StartParameter: PVOID,
        MaxThreadCount: ULONG,
        StackReserve: SIZE_T,
        StackCommit: SIZE_T,
    ) -> NTSTATUS;
    fn NtQueryInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: PVOID,
        WorkerFactoryInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: PVOID,
        WorkerFactoryInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtShutdownWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        PendingWorkerCount: *mut LONG,
    ) -> NTSTATUS;
    fn NtReleaseWorkerFactoryWorker(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWorkerFactoryWorkerReady(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtWaitForWorkViaWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        MiniPacket: *mut FILE_IO_COMPLETION_INFORMATION,
    ) -> NTSTATUS;
    fn NtQuerySystemTime(
        SystemTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtSetSystemTime(
        SystemTime: PLARGE_INTEGER,
        PreviousTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtQueryTimerResolution(
        MaximumTime: PULONG,
        MinimumTime: PULONG,
        CurrentTime: PULONG,
    ) -> NTSTATUS;
    fn NtSetTimerResolution(
        DesiredTime: ULONG,
        SetResolution: BOOLEAN,
        ActualTime: PULONG,
    ) -> NTSTATUS;
    fn NtQueryPerformanceCounter(
        PerformanceCounter: PLARGE_INTEGER,
        PerformanceFrequency: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtAllocateLocallyUniqueId(
        Luid: PLUID,
    ) -> NTSTATUS;
    fn NtSetUuidSeed(
        Seed: PCHAR,
    ) -> NTSTATUS;
    fn NtAllocateUuids(
        Time: PULARGE_INTEGER,
        Range: PULONG,
        Sequence: PULONG,
        Seed: PCHAR,
    ) -> NTSTATUS;
}}
ENUM!{enum SYSTEM_INFORMATION_CLASS {
    SystemBasicInformation = 0,
    SystemProcessorInformation = 1,
    SystemPerformanceInformation = 2,
    SystemTimeOfDayInformation = 3,
    SystemPathInformation = 4,
    SystemProcessInformation = 5,
    SystemCallCountInformation = 6,
    SystemDeviceInformation = 7,
    SystemProcessorPerformanceInformation = 8,
    SystemFlagsInformation = 9,
    SystemCallTimeInformation = 10,
    SystemModuleInformation = 11,
    SystemLocksInformation = 12,
    SystemStackTraceInformation = 13,
    SystemPagedPoolInformation = 14,
    SystemNonPagedPoolInformation = 15,
    SystemHandleInformation = 16,
    SystemObjectInformation = 17,
    SystemPageFileInformation = 18,
    SystemVdmInstemulInformation = 19,
    SystemVdmBopInformation = 20,
    SystemFileCacheInformation = 21,
    SystemPoolTagInformation = 22,
    SystemInterruptInformation = 23,
    SystemDpcBehaviorInformation = 24,
    SystemFullMemoryInformation = 25,
    SystemLoadGdiDriverInformation = 26,
    SystemUnloadGdiDriverInformation = 27,
    SystemTimeAdjustmentInformation = 28,
    SystemSummaryMemoryInformation = 29,
    SystemMirrorMemoryInformation = 30,
    SystemPerformanceTraceInformation = 31,
    SystemObsolete0 = 32,
    SystemExceptionInformation = 33,
    SystemCrashDumpStateInformation = 34,
    SystemKernelDebuggerInformation = 35,
    SystemContextSwitchInformation = 36,
    SystemRegistryQuotaInformation = 37,
    SystemExtendServiceTableInformation = 38,
    SystemPrioritySeperation = 39,
    SystemVerifierAddDriverInformation = 40,
    SystemVerifierRemoveDriverInformation = 41,
    SystemProcessorIdleInformation = 42,
    SystemLegacyDriverInformation = 43,
    SystemCurrentTimeZoneInformation = 44,
    SystemLookasideInformation = 45,
    SystemTimeSlipNotification = 46,
    SystemSessionCreate = 47,
    SystemSessionDetach = 48,
    SystemSessionInformation = 49,
    SystemRangeStartInformation = 50,
    SystemVerifierInformation = 51,
    SystemVerifierThunkExtend = 52,
    SystemSessionProcessInformation = 53,
    SystemLoadGdiDriverInSystemSpace = 54,
    SystemNumaProcessorMap = 55,
    SystemPrefetcherInformation = 56,
    SystemExtendedProcessInformation = 57,
    SystemRecommendedSharedDataAlignment = 58,
    SystemComPlusPackage = 59,
    SystemNumaAvailableMemory = 60,
    SystemProcessorPowerInformation = 61,
    SystemEmulationBasicInformation = 62,
    SystemEmulationProcessorInformation = 63,
    SystemExtendedHandleInformation = 64,
    SystemLostDelayedWriteInformation = 65,
    SystemBigPoolInformation = 66,
    SystemSessionPoolTagInformation = 67,
    SystemSessionMappedViewInformation = 68,
    SystemHotpatchInformation = 69,
    SystemObjectSecurityMode = 70,
    SystemWatchdogTimerHandler = 71,
    SystemWatchdogTimerInformation = 72,
    SystemLogicalProcessorInformation = 73,
    SystemWow64SharedInformationObsolete = 74,
    SystemRegisterFirmwareTableInformationHandler = 75,
    SystemFirmwareTableInformation = 76,
    SystemModuleInformationEx = 77,
    SystemVerifierTriageInformation = 78,
    SystemSuperfetchInformation = 79,
    SystemMemoryListInformation = 80,
    SystemFileCacheInformationEx = 81,
    SystemThreadPriorityClientIdInformation = 82,
    SystemProcessorIdleCycleTimeInformation = 83,
    SystemVerifierCancellationInformation = 84,
    SystemProcessorPowerInformationEx = 85,
    SystemRefTraceInformation = 86,
    SystemSpecialPoolInformation = 87,
    SystemProcessIdInformation = 88,
    SystemErrorPortInformation = 89,
    SystemBootEnvironmentInformation = 90,
    SystemHypervisorInformation = 91,
    SystemVerifierInformationEx = 92,
    SystemTimeZoneInformation = 93,
    SystemImageFileExecutionOptionsInformation = 94,
    SystemCoverageInformation = 95,
    SystemPrefetchPatchInformation = 96,
    SystemVerifierFaultsInformation = 97,
    SystemSystemPartitionInformation = 98,
    SystemSystemDiskInformation = 99,
    SystemProcessorPerformanceDistribution = 100,
    SystemNumaProximityNodeInformation = 101,
    SystemDynamicTimeZoneInformation = 102,
    SystemCodeIntegrityInformation = 103,
    SystemProcessorMicrocodeUpdateInformation = 104,
    SystemProcessorBrandString = 105,
    SystemVirtualAddressInformation = 106,
    SystemLogicalProcessorAndGroupInformation = 107,
    SystemProcessorCycleTimeInformation = 108,
    SystemStoreInformation = 109,
    SystemRegistryAppendString = 110,
    SystemAitSamplingValue = 111,
    SystemVhdBootInformation = 112,
    SystemCpuQuotaInformation = 113,
    SystemNativeBasicInformation = 114,
    SystemSpare1 = 115,
    SystemLowPriorityIoInformation = 116,
    SystemTpmBootEntropyInformation = 117,
    SystemVerifierCountersInformation = 118,
    SystemPagedPoolInformationEx = 119,
    SystemSystemPtesInformationEx = 120,
    SystemNodeDistanceInformation = 121,
    SystemAcpiAuditInformation = 122,
    SystemBasicPerformanceInformation = 123,
    SystemQueryPerformanceCounterInformation = 124,
    SystemSessionBigPoolInformation = 125,
    SystemBootGraphicsInformation = 126,
    SystemScrubPhysicalMemoryInformation = 127,
    SystemBadPageInformation = 128,
    SystemProcessorProfileControlArea = 129,
    SystemCombinePhysicalMemoryInformation = 130,
    SystemEntropyInterruptTimingCallback = 131,
    SystemConsoleInformation = 132,
    SystemPlatformBinaryInformation = 133,
    SystemThrottleNotificationInformation = 134,
    SystemHypervisorProcessorCountInformation = 135,
    SystemDeviceDataInformation = 136,
    SystemDeviceDataEnumerationInformation = 137,
    SystemMemoryTopologyInformation = 138,
    SystemMemoryChannelInformation = 139,
    SystemBootLogoInformation = 140,
    SystemProcessorPerformanceInformationEx = 141,
    SystemSpare0 = 142,
    SystemSecureBootPolicyInformation = 143,
    SystemPageFileInformationEx = 144,
    SystemSecureBootInformation = 145,
    SystemEntropyInterruptTimingRawInformation = 146,
    SystemPortableWorkspaceEfiLauncherInformation = 147,
    SystemFullProcessInformation = 148,
    SystemKernelDebuggerInformationEx = 149,
    SystemBootMetadataInformation = 150,
    SystemSoftRebootInformation = 151,
    SystemElamCertificateInformation = 152,
    SystemOfflineDumpConfigInformation = 153,
    SystemProcessorFeaturesInformation = 154,
    SystemRegistryReconciliationInformation = 155,
    SystemEdidInformation = 156,
    SystemManufacturingInformation = 157,
    SystemEnergyEstimationConfigInformation = 158,
    SystemHypervisorDetailInformation = 159,
    SystemProcessorCycleStatsInformation = 160,
    SystemVmGenerationCountInformation = 161,
    SystemTrustedPlatformModuleInformation = 162,
    SystemKernelDebuggerFlags = 163,
    SystemCodeIntegrityPolicyInformation = 164,
    SystemIsolatedUserModeInformation = 165,
    SystemHardwareSecurityTestInterfaceResultsInformation = 166,
    SystemSingleModuleInformation = 167,
    SystemAllowedCpuSetsInformation = 168,
    SystemVsmProtectionInformation = 169,
    SystemInterruptCpuSetsInformation = 170,
    SystemSecureBootPolicyFullInformation = 171,
    SystemCodeIntegrityPolicyFullInformation = 172,
    SystemAffinitizedInterruptProcessorInformation = 173,
    SystemRootSiloInformation = 174,
    SystemCpuSetInformation = 175,
    SystemCpuSetTagInformation = 176,
    SystemWin32WerStartCallout = 177,
    SystemSecureKernelProfileInformation = 178,
    SystemCodeIntegrityPlatformManifestInformation = 179,
    SystemInterruptSteeringInformation = 180,
    SystemSupportedProcessorArchitectures = 181,
    SystemMemoryUsageInformation = 182,
    SystemCodeIntegrityCertificateInformation = 183,
    SystemPhysicalMemoryInformation = 184,
    SystemControlFlowTransition = 185,
    SystemKernelDebuggingAllowed = 186,
    SystemActivityModerationExeState = 187,
    SystemActivityModerationUserSettings = 188,
    SystemCodeIntegrityPoliciesFullInformation = 189,
    SystemCodeIntegrityUnlockInformation = 190,
    SystemIntegrityQuotaInformation = 191,
    SystemFlushInformation = 192,
    SystemProcessorIdleMaskInformation = 193,
    SystemSecureDumpEncryptionInformation = 194,
    SystemWriteConstraintInformation = 195,
    SystemKernelVaShadowInformation = 196,
    SystemHypervisorSharedPageInformation = 197,
    SystemFirmwareBootPerformanceInformation = 198,
    SystemCodeIntegrityVerificationInformation = 199,
    SystemFirmwarePartitionInformation = 200,
    SystemSpeculationControlInformation = 201,
    SystemDmaGuardPolicyInformation = 202,
    SystemEnclaveLaunchControlInformation = 203,
    SystemWorkloadAllowedCpuSetsInformation = 204,
    SystemCodeIntegrityUnlockModeInformation = 205,
    SystemLeapSecondInformation = 206,
    SystemFlags2Information = 207,
    MaxSystemInfoClass = 208,
}}
STRUCT!{struct SYSTEM_BASIC_INFORMATION {
    Reserved: ULONG,
    TimerResolution: ULONG,
    PageSize: ULONG,
    NumberOfPhysicalPages: ULONG,
    LowestPhysicalPageNumber: ULONG,
    HighestPhysicalPageNumber: ULONG,
    AllocationGranularity: ULONG,
    MinimumUserModeAddress: ULONG_PTR,
    MaximumUserModeAddress: ULONG_PTR,
    ActiveProcessorsAffinityMask: ULONG_PTR,
    NumberOfProcessors: CCHAR,
}}
pub type PSYSTEM_BASIC_INFORMATION = *mut SYSTEM_BASIC_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_INFORMATION {
    ProcessorArchitecture: USHORT,
    ProcessorLevel: USHORT,
    ProcessorRevision: USHORT,
    MaximumProcessors: USHORT,
    ProcessorFeatureBits: ULONG,
}}
pub type PSYSTEM_PROCESSOR_INFORMATION = *mut SYSTEM_PROCESSOR_INFORMATION;
STRUCT!{struct SYSTEM_PERFORMANCE_INFORMATION {
    IdleProcessTime: LARGE_INTEGER,
    IoReadTransferCount: LARGE_INTEGER,
    IoWriteTransferCount: LARGE_INTEGER,
    IoOtherTransferCount: LARGE_INTEGER,
    IoReadOperationCount: ULONG,
    IoWriteOperationCount: ULONG,
    IoOtherOperationCount: ULONG,
    AvailablePages: ULONG,
    CommittedPages: ULONG,
    CommitLimit: ULONG,
    PeakCommitment: ULONG,
    PageFaultCount: ULONG,
    CopyOnWriteCount: ULONG,
    TransitionCount: ULONG,
    CacheTransitionCount: ULONG,
    DemandZeroCount: ULONG,
    PageReadCount: ULONG,
    PageReadIoCount: ULONG,
    CacheReadCount: ULONG,
    CacheIoCount: ULONG,
    DirtyPagesWriteCount: ULONG,
    DirtyWriteIoCount: ULONG,
    MappedPagesWriteCount: ULONG,
    MappedWriteIoCount: ULONG,
    PagedPoolPages: ULONG,
    NonPagedPoolPages: ULONG,
    PagedPoolAllocs: ULONG,
    PagedPoolFrees: ULONG,
    NonPagedPoolAllocs: ULONG,
    NonPagedPoolFrees: ULONG,
    FreeSystemPtes: ULONG,
    ResidentSystemCodePage: ULONG,
    TotalSystemDriverPages: ULONG,
    TotalSystemCodePages: ULONG,
    NonPagedPoolLookasideHits: ULONG,
    PagedPoolLookasideHits: ULONG,
    AvailablePagedPoolPages: ULONG,
    ResidentSystemCachePage: ULONG,
    ResidentPagedPoolPage: ULONG,
    ResidentSystemDriverPage: ULONG,
    CcFastReadNoWait: ULONG,
    CcFastReadWait: ULONG,
    CcFastReadResourceMiss: ULONG,
    CcFastReadNotPossible: ULONG,
    CcFastMdlReadNoWait: ULONG,
    CcFastMdlReadWait: ULONG,
    CcFastMdlReadResourceMiss: ULONG,
    CcFastMdlReadNotPossible: ULONG,
    CcMapDataNoWait: ULONG,
    CcMapDataWait: ULONG,
    CcMapDataNoWaitMiss: ULONG,
    CcMapDataWaitMiss: ULONG,
    CcPinMappedDataCount: ULONG,
    CcPinReadNoWait: ULONG,
    CcPinReadWait: ULONG,
    CcPinReadNoWaitMiss: ULONG,
    CcPinReadWaitMiss: ULONG,
    CcCopyReadNoWait: ULONG,
    CcCopyReadWait: ULONG,
    CcCopyReadNoWaitMiss: ULONG,
    CcCopyReadWaitMiss: ULONG,
    CcMdlReadNoWait: ULONG,
    CcMdlReadWait: ULONG,
    CcMdlReadNoWaitMiss: ULONG,
    CcMdlReadWaitMiss: ULONG,
    CcReadAheadIos: ULONG,
    CcLazyWriteIos: ULONG,
    CcLazyWritePages: ULONG,
    CcDataFlushes: ULONG,
    CcDataPages: ULONG,
    ContextSwitches: ULONG,
    FirstLevelTbFills: ULONG,
    SecondLevelTbFills: ULONG,
    SystemCalls: ULONG,
    CcTotalDirtyPages: ULONGLONG,
    CcDirtyPageThreshold: ULONGLONG,
    ResidentAvailablePages: LONGLONG,
    SharedCommittedPages: ULONGLONG,
}}
pub type PSYSTEM_PERFORMANCE_INFORMATION = *mut SYSTEM_PERFORMANCE_INFORMATION;
STRUCT!{struct SYSTEM_TIMEOFDAY_INFORMATION {
    BootTime: LARGE_INTEGER,
    CurrentTime: LARGE_INTEGER,
    TimeZoneBias: LARGE_INTEGER,
    TimeZoneId: ULONG,
    Reserved: ULONG,
    BootTimeBias: ULONGLONG,
    SleepTimeBias: ULONGLONG,
}}
pub type PSYSTEM_TIMEOFDAY_INFORMATION = *mut SYSTEM_TIMEOFDAY_INFORMATION;
STRUCT!{struct SYSTEM_THREAD_INFORMATION {
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    CreateTime: LARGE_INTEGER,
    WaitTime: ULONG,
    StartAddress: PVOID,
    ClientId: CLIENT_ID,
    Priority: KPRIORITY,
    BasePriority: LONG,
    ContextSwitches: ULONG,
    ThreadState: KTHREAD_STATE,
    WaitReason: KWAIT_REASON,
}}
pub type PSYSTEM_THREAD_INFORMATION = *mut SYSTEM_THREAD_INFORMATION;
STRUCT!{struct SYSTEM_EXTENDED_THREAD_INFORMATION {
    ThreadInfo: SYSTEM_THREAD_INFORMATION,
    StackBase: PVOID,
    StackLimit: PVOID,
    Win32StartAddress: PVOID,
    TebBase: PTEB,
    Reserved2: ULONG_PTR,
    Reserved3: ULONG_PTR,
    Reserved4: ULONG_PTR,
}}
pub type PSYSTEM_EXTENDED_THREAD_INFORMATION = *mut SYSTEM_EXTENDED_THREAD_INFORMATION;
STRUCT!{struct SYSTEM_PROCESS_INFORMATION {
    NextEntryOffset: ULONG,
    NumberOfThreads: ULONG,
    WorkingSetPrivateSize: LARGE_INTEGER,
    HardFaultCount: ULONG,
    NumberOfThreadsHighWatermark: ULONG,
    CycleTime: ULONGLONG,
    CreateTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    ImageName: UNICODE_STRING,
    BasePriority: KPRIORITY,
    UniqueProcessId: HANDLE,
    InheritedFromUniqueProcessId: HANDLE,
    HandleCount: ULONG,
    SessionId: ULONG,
    UniqueProcessKey: ULONG_PTR,
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
    PrivatePageCount: SIZE_T,
    ReadOperationCount: LARGE_INTEGER,
    WriteOperationCount: LARGE_INTEGER,
    OtherOperationCount: LARGE_INTEGER,
    ReadTransferCount: LARGE_INTEGER,
    WriteTransferCount: LARGE_INTEGER,
    OtherTransferCount: LARGE_INTEGER,
    Threads: [SYSTEM_THREAD_INFORMATION; 1],
}}
pub type PSYSTEM_PROCESS_INFORMATION = *mut SYSTEM_PROCESS_INFORMATION;
STRUCT!{struct SYSTEM_CALL_COUNT_INFORMATION {
    Length: ULONG,
    NumberOfTables: ULONG,
}}
pub type PSYSTEM_CALL_COUNT_INFORMATION = *mut SYSTEM_CALL_COUNT_INFORMATION;
STRUCT!{struct SYSTEM_DEVICE_INFORMATION {
    NumberOfDisks: ULONG,
    NumberOfFloppies: ULONG,
    NumberOfCdRoms: ULONG,
    NumberOfTapes: ULONG,
    NumberOfSerialPorts: ULONG,
    NumberOfParallelPorts: ULONG,
}}
pub type PSYSTEM_DEVICE_INFORMATION = *mut SYSTEM_DEVICE_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    IdleTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    DpcTime: LARGE_INTEGER,
    InterruptTime: LARGE_INTEGER,
    InterruptCount: ULONG,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_INFORMATION = *mut SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION;
STRUCT!{struct SYSTEM_FLAGS_INFORMATION {
    Flags: ULONG,
}}
pub type PSYSTEM_FLAGS_INFORMATION = *mut SYSTEM_FLAGS_INFORMATION;
STRUCT!{struct SYSTEM_CALL_TIME_INFORMATION {
    Length: ULONG,
    TotalCalls: ULONG,
    TimeOfCalls: [LARGE_INTEGER; 1],
}}
pub type PSYSTEM_CALL_TIME_INFORMATION = *mut SYSTEM_CALL_TIME_INFORMATION;
STRUCT!{struct RTL_PROCESS_LOCK_INFORMATION {
    Address: PVOID,
    Type: USHORT,
    CreatorBackTraceIndex: USHORT,
    OwningThread: HANDLE,
    LockCount: LONG,
    ContentionCount: ULONG,
    EntryCount: ULONG,
    RecursionCount: LONG,
    NumberOfWaitingShared: ULONG,
    NumberOfWaitingExclusive: ULONG,
}}
pub type PRTL_PROCESS_LOCK_INFORMATION = *mut RTL_PROCESS_LOCK_INFORMATION;
STRUCT!{struct RTL_PROCESS_LOCKS {
    NumberOfLocks: ULONG,
    Locks: [RTL_PROCESS_LOCK_INFORMATION; 1],
}}
pub type PRTL_PROCESS_LOCKS = *mut RTL_PROCESS_LOCKS;
STRUCT!{struct RTL_PROCESS_BACKTRACE_INFORMATION {
    SymbolicBackTrace: PCHAR,
    TraceCount: ULONG,
    Index: USHORT,
    Depth: USHORT,
    BackTrace: [PVOID; 32],
}}
pub type PRTL_PROCESS_BACKTRACE_INFORMATION = *mut RTL_PROCESS_BACKTRACE_INFORMATION;
STRUCT!{struct RTL_PROCESS_BACKTRACES {
    CommittedMemory: ULONG,
    ReservedMemory: ULONG,
    NumberOfBackTraceLookups: ULONG,
    NumberOfBackTraces: ULONG,
    BackTraces: [RTL_PROCESS_BACKTRACE_INFORMATION; 1],
}}
pub type PRTL_PROCESS_BACKTRACES = *mut RTL_PROCESS_BACKTRACES;
STRUCT!{struct SYSTEM_HANDLE_TABLE_ENTRY_INFO {
    UniqueProcessId: USHORT,
    CreatorBackTraceIndex: USHORT,
    ObjectTypeIndex: UCHAR,
    HandleAttributes: UCHAR,
    HandleValue: USHORT,
    Object: PVOID,
    GrantedAccess: ULONG,
}}
pub type PSYSTEM_HANDLE_TABLE_ENTRY_INFO = *mut SYSTEM_HANDLE_TABLE_ENTRY_INFO;
STRUCT!{struct SYSTEM_HANDLE_INFORMATION {
    NumberOfHandles: ULONG,
    Handles: [SYSTEM_HANDLE_TABLE_ENTRY_INFO; 1],
}}
pub type PSYSTEM_HANDLE_INFORMATION = *mut SYSTEM_HANDLE_INFORMATION;
STRUCT!{struct SYSTEM_OBJECTTYPE_INFORMATION {
    NextEntryOffset: ULONG,
    NumberOfObjects: ULONG,
    NumberOfHandles: ULONG,
    TypeIndex: ULONG,
    InvalidAttributes: ULONG,
    GenericMapping: GENERIC_MAPPING,
    ValidAccessMask: ULONG,
    PoolType: ULONG,
    SecurityRequired: BOOLEAN,
    WaitableObject: BOOLEAN,
    TypeName: UNICODE_STRING,
}}
pub type PSYSTEM_OBJECTTYPE_INFORMATION = *mut SYSTEM_OBJECTTYPE_INFORMATION;
STRUCT!{struct SYSTEM_OBJECT_INFORMATION {
    NextEntryOffset: ULONG,
    Object: PVOID,
    CreatorUniqueProcess: HANDLE,
    CreatorBackTraceIndex: USHORT,
    Flags: USHORT,
    PointerCount: LONG,
    HandleCount: LONG,
    PagedPoolCharge: ULONG,
    NonPagedPoolCharge: ULONG,
    ExclusiveProcessId: HANDLE,
    SecurityDescriptor: PVOID,
    NameInfo: UNICODE_STRING,
}}
pub type PSYSTEM_OBJECT_INFORMATION = *mut SYSTEM_OBJECT_INFORMATION;
STRUCT!{struct SYSTEM_PAGEFILE_INFORMATION {
    NextEntryOffset: ULONG,
    TotalSize: ULONG,
    TotalInUse: ULONG,
    PeakUsage: ULONG,
    PageFileName: UNICODE_STRING,
}}
pub type PSYSTEM_PAGEFILE_INFORMATION = *mut SYSTEM_PAGEFILE_INFORMATION;
pub const MM_WORKING_SET_MAX_HARD_ENABLE: ULONG = 0x1;
pub const MM_WORKING_SET_MAX_HARD_DISABLE: ULONG = 0x2;
pub const MM_WORKING_SET_MIN_HARD_ENABLE: ULONG = 0x4;
pub const MM_WORKING_SET_MIN_HARD_DISABLE: ULONG = 0x8;
STRUCT!{struct SYSTEM_FILECACHE_INFORMATION {
    CurrentSize: SIZE_T,
    PeakSize: SIZE_T,
    PageFaultCount: ULONG,
    MinimumWorkingSet: SIZE_T,
    MaximumWorkingSet: SIZE_T,
    CurrentSizeIncludingTransitionInPages: SIZE_T,
    PeakSizeIncludingTransitionInPages: SIZE_T,
    TransitionRePurposeCount: ULONG,
    Flags: ULONG,
}}
pub type PSYSTEM_FILECACHE_INFORMATION = *mut SYSTEM_FILECACHE_INFORMATION;
STRUCT!{struct SYSTEM_BASIC_WORKING_SET_INFORMATION {
    CurrentSize: SIZE_T,
    PeakSize: SIZE_T,
    PageFaultCount: ULONG,
}}
pub type PSYSTEM_BASIC_WORKING_SET_INFORMATION = *mut SYSTEM_BASIC_WORKING_SET_INFORMATION;
UNION!{union SYSTEM_POOLTAG_u {
    Tag: [UCHAR; 4],
    TagUlong: ULONG,
}}
STRUCT!{struct SYSTEM_POOLTAG {
    u: SYSTEM_POOLTAG_u,
    PagedAllocs: ULONG,
    PagedFrees: ULONG,
    PagedUsed: SIZE_T,
    NonPagedAllocs: ULONG,
    NonPagedFrees: ULONG,
    NonPagedUsed: SIZE_T,
}}
pub type PSYSTEM_POOLTAG = *mut SYSTEM_POOLTAG;
STRUCT!{struct SYSTEM_POOLTAG_INFORMATION {
    Count: ULONG,
    TagInfo: [SYSTEM_POOLTAG; 1],
}}
pub type PSYSTEM_POOLTAG_INFORMATION = *mut SYSTEM_POOLTAG_INFORMATION;
STRUCT!{struct SYSTEM_INTERRUPT_INFORMATION {
    ContextSwitches: ULONG,
    DpcCount: ULONG,
    DpcRate: ULONG,
    TimeIncrement: ULONG,
    DpcBypassCount: ULONG,
    ApcBypassCount: ULONG,
}}
pub type PSYSTEM_INTERRUPT_INFORMATION = *mut SYSTEM_INTERRUPT_INFORMATION;
STRUCT!{struct SYSTEM_DPC_BEHAVIOR_INFORMATION {
    Spare: ULONG,
    DpcQueueDepth: ULONG,
    MinimumDpcRate: ULONG,
    AdjustDpcThreshold: ULONG,
    IdealDpcRate: ULONG,
}}
pub type PSYSTEM_DPC_BEHAVIOR_INFORMATION = *mut SYSTEM_DPC_BEHAVIOR_INFORMATION;
STRUCT!{struct SYSTEM_QUERY_TIME_ADJUST_INFORMATION {
    TimeAdjustment: ULONG,
    TimeIncrement: ULONG,
    Enable: BOOLEAN,
}}
pub type PSYSTEM_QUERY_TIME_ADJUST_INFORMATION = *mut SYSTEM_QUERY_TIME_ADJUST_INFORMATION;
STRUCT!{struct SYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE {
    TimeAdjustment: ULONGLONG,
    TimeIncrement: ULONGLONG,
    Enable: BOOLEAN,
}}
pub type PSYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE =
    *mut SYSTEM_QUERY_TIME_ADJUST_INFORMATION_PRECISE;
STRUCT!{struct SYSTEM_SET_TIME_ADJUST_INFORMATION {
    TimeAdjustment: ULONG,
    Enable: BOOLEAN,
}}
pub type PSYSTEM_SET_TIME_ADJUST_INFORMATION = *mut SYSTEM_SET_TIME_ADJUST_INFORMATION;
STRUCT!{struct SYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE {
    TimeAdjustment: ULONGLONG,
    Enable: BOOLEAN,
}}
pub type PSYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE =
    *mut SYSTEM_SET_TIME_ADJUST_INFORMATION_PRECISE;
ENUM!{enum EVENT_TRACE_INFORMATION_CLASS {
    EventTraceKernelVersionInformation = 0,
    EventTraceGroupMaskInformation = 1,
    EventTracePerformanceInformation = 2,
    EventTraceTimeProfileInformation = 3,
    EventTraceSessionSecurityInformation = 4,
    EventTraceSpinlockInformation = 5,
    EventTraceStackTracingInformation = 6,
    EventTraceExecutiveResourceInformation = 7,
    EventTraceHeapTracingInformation = 8,
    EventTraceHeapSummaryTracingInformation = 9,
    EventTracePoolTagFilterInformation = 10,
    EventTracePebsTracingInformation = 11,
    EventTraceProfileConfigInformation = 12,
    EventTraceProfileSourceListInformation = 13,
    EventTraceProfileEventListInformation = 14,
    EventTraceProfileCounterListInformation = 15,
    EventTraceStackCachingInformation = 16,
    EventTraceObjectTypeFilterInformation = 17,
    EventTraceSoftRestartInformation = 18,
    EventTraceLastBranchConfigurationInformation = 19,
    EventTraceLastBranchEventListInformation = 20,
    EventTraceProfileSourceAddInformation = 21,
    EventTraceProfileSourceRemoveInformation = 22,
    EventTraceProcessorTraceConfigurationInformation = 23,
    EventTraceProcessorTraceEventListInformation = 24,
    EventTraceCoverageSamplerInformation = 25,
    MaxEventTraceInfoClass = 26,
}}
STRUCT!{struct EVENT_TRACE_VERSION_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    EventTraceKernelVersion: ULONG,
}}
pub type PEVENT_TRACE_VERSION_INFORMATION = *mut EVENT_TRACE_VERSION_INFORMATION;
STRUCT!{struct PERFINFO_GROUPMASK {
    Masks: [ULONG; 8],
}}
pub type PPERFINFO_GROUPMASK = *mut PERFINFO_GROUPMASK;
STRUCT!{struct EVENT_TRACE_GROUPMASK_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    EventTraceGroupMasks: PERFINFO_GROUPMASK,
}}
pub type PEVENT_TRACE_GROUPMASK_INFORMATION = *mut EVENT_TRACE_GROUPMASK_INFORMATION;
STRUCT!{struct EVENT_TRACE_PERFORMANCE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    LogfileBytesWritten: LARGE_INTEGER,
}}
pub type PEVENT_TRACE_PERFORMANCE_INFORMATION = *mut EVENT_TRACE_PERFORMANCE_INFORMATION;
STRUCT!{struct EVENT_TRACE_TIME_PROFILE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ProfileInterval: ULONG,
}}
pub type PEVENT_TRACE_TIME_PROFILE_INFORMATION = *mut EVENT_TRACE_TIME_PROFILE_INFORMATION;
STRUCT!{struct EVENT_TRACE_SESSION_SECURITY_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    SecurityInformation: ULONG,
    TraceHandle: HANDLE,
    SecurityDescriptor: [UCHAR; 1],
}}
pub type PEVENT_TRACE_SESSION_SECURITY_INFORMATION = *mut EVENT_TRACE_SESSION_SECURITY_INFORMATION;
STRUCT!{struct EVENT_TRACE_SPINLOCK_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    SpinLockSpinThreshold: ULONG,
    SpinLockAcquireSampleRate: ULONG,
    SpinLockContentionSampleRate: ULONG,
    SpinLockHoldThreshold: ULONG,
}}
pub type PEVENT_TRACE_SPINLOCK_INFORMATION = *mut EVENT_TRACE_SPINLOCK_INFORMATION;
STRUCT!{struct EVENT_TRACE_SYSTEM_EVENT_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    HookId: [ULONG; 1],
}}
pub type PEVENT_TRACE_SYSTEM_EVENT_INFORMATION = *mut EVENT_TRACE_SYSTEM_EVENT_INFORMATION;
STRUCT!{struct EVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ReleaseSamplingRate: ULONG,
    ContentionSamplingRate: ULONG,
    NumberOfExcessiveTimeouts: ULONG,
}}
pub type PEVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION =
    *mut EVENT_TRACE_EXECUTIVE_RESOURCE_INFORMATION;
STRUCT!{struct EVENT_TRACE_HEAP_TRACING_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ProcessId: ULONG,
}}
pub type PEVENT_TRACE_HEAP_TRACING_INFORMATION = *mut EVENT_TRACE_HEAP_TRACING_INFORMATION;
STRUCT!{struct EVENT_TRACE_TAG_FILTER_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    Filter: [ULONG; 1],
}}
pub type PEVENT_TRACE_TAG_FILTER_INFORMATION = *mut EVENT_TRACE_TAG_FILTER_INFORMATION;
STRUCT!{struct EVENT_TRACE_PROFILE_COUNTER_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    ProfileSource: [ULONG; 1],
}}
pub type PEVENT_TRACE_PROFILE_COUNTER_INFORMATION = *mut EVENT_TRACE_PROFILE_COUNTER_INFORMATION;
STRUCT!{struct EVENT_TRACE_PROFILE_LIST_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    Spare: ULONG,
    Profile: [*mut PROFILE_SOURCE_INFO; 1],
}}
pub type PEVENT_TRACE_PROFILE_LIST_INFORMATION = *mut EVENT_TRACE_PROFILE_LIST_INFORMATION;
STRUCT!{struct EVENT_TRACE_STACK_CACHING_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    Enabled: BOOLEAN,
    Reserved: [UCHAR; 3],
    CacheSize: ULONG,
    BucketCount: ULONG,
}}
pub type PEVENT_TRACE_STACK_CACHING_INFORMATION = *mut EVENT_TRACE_STACK_CACHING_INFORMATION;
STRUCT!{struct EVENT_TRACE_SOFT_RESTART_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    TraceHandle: HANDLE,
    PersistTraceBuffers: BOOLEAN,
    FileName: [WCHAR; 1],
}}
pub type PEVENT_TRACE_SOFT_RESTART_INFORMATION = *mut EVENT_TRACE_SOFT_RESTART_INFORMATION;
STRUCT!{struct EVENT_TRACE_PROFILE_ADD_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    PerfEvtEventSelect: BOOLEAN,
    PerfEvtUnitSelect: BOOLEAN,
    PerfEvtType: ULONG,
    CpuInfoHierarchy: [ULONG; 3],
    InitialInterval: ULONG,
    AllowsHalt: BOOLEAN,
    Persist: BOOLEAN,
    ProfileSourceDescription: [WCHAR; 1],
}}
pub type PEVENT_TRACE_PROFILE_ADD_INFORMATION = *mut EVENT_TRACE_PROFILE_ADD_INFORMATION;
STRUCT!{struct EVENT_TRACE_PROFILE_REMOVE_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    ProfileSource: KPROFILE_SOURCE,
    CpuInfoHierarchy: [ULONG; 3],
}}
pub type PEVENT_TRACE_PROFILE_REMOVE_INFORMATION = *mut EVENT_TRACE_PROFILE_REMOVE_INFORMATION;
STRUCT!{struct EVENT_TRACE_COVERAGE_SAMPLER_INFORMATION {
    EventTraceInformationClass: EVENT_TRACE_INFORMATION_CLASS,
    CoverageSamplerInformationClass: BOOLEAN,
    MajorVersion: UCHAR,
    MinorVersion: UCHAR,
    Reserved: UCHAR,
    SamplerHandle: HANDLE,
}}
pub type PEVENT_TRACE_COVERAGE_SAMPLER_INFORMATION = *mut EVENT_TRACE_COVERAGE_SAMPLER_INFORMATION;
STRUCT!{struct SYSTEM_EXCEPTION_INFORMATION {
    AlignmentFixupCount: ULONG,
    ExceptionDispatchCount: ULONG,
    FloatingEmulationCount: ULONG,
    ByteWordEmulationCount: ULONG,
}}
pub type PSYSTEM_EXCEPTION_INFORMATION = *mut SYSTEM_EXCEPTION_INFORMATION;
STRUCT!{struct SYSTEM_KERNEL_DEBUGGER_INFORMATION {
    KernelDebuggerEnabled: BOOLEAN,
    KernelDebuggerNotPresent: BOOLEAN,
}}
pub type PSYSTEM_KERNEL_DEBUGGER_INFORMATION = *mut SYSTEM_KERNEL_DEBUGGER_INFORMATION;
STRUCT!{struct SYSTEM_CONTEXT_SWITCH_INFORMATION {
    ContextSwitches: ULONG,
    FindAny: ULONG,
    FindLast: ULONG,
    FindIdeal: ULONG,
    IdleAny: ULONG,
    IdleCurrent: ULONG,
    IdleLast: ULONG,
    IdleIdeal: ULONG,
    PreemptAny: ULONG,
    PreemptCurrent: ULONG,
    PreemptLast: ULONG,
    SwitchToIdle: ULONG,
}}
pub type PSYSTEM_CONTEXT_SWITCH_INFORMATION = *mut SYSTEM_CONTEXT_SWITCH_INFORMATION;
STRUCT!{struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    RegistryQuotaAllowed: ULONG,
    RegistryQuotaUsed: ULONG,
    PagedPoolSize: SIZE_T,
}}
pub type PSYSTEM_REGISTRY_QUOTA_INFORMATION = *mut SYSTEM_REGISTRY_QUOTA_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_IDLE_INFORMATION {
    IdleTime: ULONGLONG,
    C1Time: ULONGLONG,
    C2Time: ULONGLONG,
    C3Time: ULONGLONG,
    C1Transitions: ULONG,
    C2Transitions: ULONG,
    C3Transitions: ULONG,
    Padding: ULONG,
}}
pub type PSYSTEM_PROCESSOR_IDLE_INFORMATION = *mut SYSTEM_PROCESSOR_IDLE_INFORMATION;
STRUCT!{struct SYSTEM_LEGACY_DRIVER_INFORMATION {
    VetoType: ULONG,
    VetoList: UNICODE_STRING,
}}
pub type PSYSTEM_LEGACY_DRIVER_INFORMATION = *mut SYSTEM_LEGACY_DRIVER_INFORMATION;
STRUCT!{struct SYSTEM_LOOKASIDE_INFORMATION {
    CurrentDepth: USHORT,
    MaximumDepth: USHORT,
    TotalAllocates: ULONG,
    AllocateMisses: ULONG,
    TotalFrees: ULONG,
    FreeMisses: ULONG,
    Type: ULONG,
    Tag: ULONG,
    Size: ULONG,
}}
pub type PSYSTEM_LOOKASIDE_INFORMATION = *mut SYSTEM_LOOKASIDE_INFORMATION;
STRUCT!{struct SYSTEM_RANGE_START_INFORMATION {
    SystemRangeStart: PVOID,
}}
pub type PSYSTEM_RANGE_START_INFORMATION = *mut SYSTEM_RANGE_START_INFORMATION;
STRUCT!{struct SYSTEM_VERIFIER_INFORMATION {
    NextEntryOffset: ULONG,
    Level: ULONG,
    DriverName: UNICODE_STRING,
    RaiseIrqls: ULONG,
    AcquireSpinLocks: ULONG,
    SynchronizeExecutions: ULONG,
    AllocationsAttempted: ULONG,
    AllocationsSucceeded: ULONG,
    AllocationsSucceededSpecialPool: ULONG,
    AllocationsWithNoTag: ULONG,
    TrimRequests: ULONG,
    Trims: ULONG,
    AllocationsFailed: ULONG,
    AllocationsFailedDeliberately: ULONG,
    Loads: ULONG,
    Unloads: ULONG,
    UnTrackedPool: ULONG,
    CurrentPagedPoolAllocations: ULONG,
    CurrentNonPagedPoolAllocations: ULONG,
    PeakPagedPoolAllocations: ULONG,
    PeakNonPagedPoolAllocations: ULONG,
    PagedPoolUsageInBytes: SIZE_T,
    NonPagedPoolUsageInBytes: SIZE_T,
    PeakPagedPoolUsageInBytes: SIZE_T,
    PeakNonPagedPoolUsageInBytes: SIZE_T,
}}
pub type PSYSTEM_VERIFIER_INFORMATION = *mut SYSTEM_VERIFIER_INFORMATION;
STRUCT!{struct SYSTEM_SESSION_PROCESS_INFORMATION {
    SessionId: ULONG,
    SizeOfBuf: ULONG,
    Buffer: PVOID,
}}
pub type PSYSTEM_SESSION_PROCESS_INFORMATION = *mut SYSTEM_SESSION_PROCESS_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_POWER_INFORMATION {
    CurrentFrequency: UCHAR,
    ThermalLimitFrequency: UCHAR,
    ConstantThrottleFrequency: UCHAR,
    DegradedThrottleFrequency: UCHAR,
    LastBusyFrequency: UCHAR,
    LastC3Frequency: UCHAR,
    LastAdjustedBusyFrequency: UCHAR,
    ProcessorMinThrottle: UCHAR,
    ProcessorMaxThrottle: UCHAR,
    NumberOfFrequencies: ULONG,
    PromotionCount: ULONG,
    DemotionCount: ULONG,
    ErrorCount: ULONG,
    RetryCount: ULONG,
    CurrentFrequencyTime: ULONGLONG,
    CurrentProcessorTime: ULONGLONG,
    CurrentProcessorIdleTime: ULONGLONG,
    LastProcessorTime: ULONGLONG,
    LastProcessorIdleTime: ULONGLONG,
    Energy: ULONGLONG,
}}
pub type PSYSTEM_PROCESSOR_POWER_INFORMATION = *mut SYSTEM_PROCESSOR_POWER_INFORMATION;
STRUCT!{struct SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX {
    Object: PVOID,
    UniqueProcessId: ULONG_PTR,
    HandleValue: ULONG_PTR,
    GrantedAccess: ULONG,
    CreatorBackTraceIndex: USHORT,
    ObjectTypeIndex: USHORT,
    HandleAttributes: ULONG,
    Reserved: ULONG,
}}
pub type PSYSTEM_HANDLE_TABLE_ENTRY_INFO_EX = *mut SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX;
STRUCT!{struct SYSTEM_HANDLE_INFORMATION_EX {
    NumberOfHandles: ULONG_PTR,
    Reserved: ULONG_PTR,
    Handles: [SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX; 1],
}}
pub type PSYSTEM_HANDLE_INFORMATION_EX = *mut SYSTEM_HANDLE_INFORMATION_EX;
UNION!{union SYSTEM_BIGPOOL_ENTRY_u1 {
    VirtualAddress: PVOID,
    Bitfields: ULONG_PTR,
}}
UNION!{union SYSTEM_BIGPOOL_ENTRY_u2 {
    Tag: [UCHAR; 4],
    TagUlong: ULONG,
}}
BITFIELD!{unsafe SYSTEM_BIGPOOL_ENTRY_u1 Bitfields: ULONG_PTR [
    NonPaged set_NonPaged[0..1],
]}
STRUCT!{struct SYSTEM_BIGPOOL_ENTRY {
    u1: SYSTEM_BIGPOOL_ENTRY_u1,
    SizeInBytes: SIZE_T,
    u2: SYSTEM_BIGPOOL_ENTRY_u2,
}}
pub type PSYSTEM_BIGPOOL_ENTRY = *mut SYSTEM_BIGPOOL_ENTRY;
STRUCT!{struct SYSTEM_BIGPOOL_INFORMATION {
    Count: ULONG,
    AllocatedInfo: [SYSTEM_BIGPOOL_ENTRY; 1],
}}
pub type PSYSTEM_BIGPOOL_INFORMATION = *mut SYSTEM_BIGPOOL_INFORMATION;
UNION!{union SYSTEM_POOL_ENTRY_u {
    Tag: [UCHAR; 4],
    TagUlong: ULONG,
    ProcessChargedQuota: PVOID,
}}
STRUCT!{struct SYSTEM_POOL_ENTRY {
    Allocated: BOOLEAN,
    Spare0: BOOLEAN,
    AllocatorBackTraceIndex: USHORT,
    Size: ULONG,
    u: SYSTEM_POOL_ENTRY_u,
}}
pub type PSYSTEM_POOL_ENTRY = *mut SYSTEM_POOL_ENTRY;
STRUCT!{struct SYSTEM_POOL_INFORMATION {
    TotalSize: SIZE_T,
    FirstEntry: PVOID,
    EntryOverhead: USHORT,
    PoolTagPresent: BOOLEAN,
    Spare0: BOOLEAN,
    NumberOfEntries: ULONG,
    Entries: [SYSTEM_POOL_ENTRY; 1],
}}
pub type PSYSTEM_POOL_INFORMATION = *mut SYSTEM_POOL_INFORMATION;
STRUCT!{struct SYSTEM_SESSION_POOLTAG_INFORMATION {
    NextEntryOffset: SIZE_T,
    SessionId: ULONG,
    Count: ULONG,
    TagInfo: [SYSTEM_POOLTAG; 1],
}}
pub type PSYSTEM_SESSION_POOLTAG_INFORMATION = *mut SYSTEM_SESSION_POOLTAG_INFORMATION;
STRUCT!{struct SYSTEM_SESSION_MAPPED_VIEW_INFORMATION {
    NextEntryOffset: SIZE_T,
    SessionId: ULONG,
    ViewFailures: ULONG,
    NumberOfBytesAvailable: SIZE_T,
    NumberOfBytesAvailableContiguous: SIZE_T,
}}
pub type PSYSTEM_SESSION_MAPPED_VIEW_INFORMATION = *mut SYSTEM_SESSION_MAPPED_VIEW_INFORMATION;
ENUM!{enum SYSTEM_FIRMWARE_TABLE_ACTION {
    SystemFirmwareTableEnumerate = 0,
    SystemFirmwareTableGet = 1,
    SystemFirmwareTableMax = 2,
}}
STRUCT!{struct SYSTEM_FIRMWARE_TABLE_INFORMATION {
    ProviderSignature: ULONG,
    Action: SYSTEM_FIRMWARE_TABLE_ACTION,
    TableID: ULONG,
    TableBufferLength: ULONG,
    TableBuffer: [UCHAR; 1],
}}
pub type PSYSTEM_FIRMWARE_TABLE_INFORMATION = *mut SYSTEM_FIRMWARE_TABLE_INFORMATION;
STRUCT!{struct SYSTEM_MEMORY_LIST_INFORMATION {
    ZeroPageCount: ULONG_PTR,
    FreePageCount: ULONG_PTR,
    ModifiedPageCount: ULONG_PTR,
    ModifiedNoWritePageCount: ULONG_PTR,
    BadPageCount: ULONG_PTR,
    PageCountByPriority: [ULONG_PTR; 8],
    RepurposedPagesByPriority: [ULONG_PTR; 8],
    ModifiedPageCountPageFile: ULONG_PTR,
}}
pub type PSYSTEM_MEMORY_LIST_INFORMATION = *mut SYSTEM_MEMORY_LIST_INFORMATION;
ENUM!{enum SYSTEM_MEMORY_LIST_COMMAND {
    MemoryCaptureAccessedBits = 0,
    MemoryCaptureAndResetAccessedBits = 1,
    MemoryEmptyWorkingSets = 2,
    MemoryFlushModifiedList = 3,
    MemoryPurgeStandbyList = 4,
    MemoryPurgeLowPriorityStandbyList = 5,
    MemoryCommandMax = 6,
}}
STRUCT!{struct SYSTEM_THREAD_CID_PRIORITY_INFORMATION {
    ClientId: CLIENT_ID,
    Priority: KPRIORITY,
}}
pub type PSYSTEM_THREAD_CID_PRIORITY_INFORMATION = *mut SYSTEM_THREAD_CID_PRIORITY_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION {
    CycleTime: ULONGLONG,
}}
pub type PSYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION =
    *mut SYSTEM_PROCESSOR_IDLE_CYCLE_TIME_INFORMATION;
STRUCT!{struct SYSTEM_REF_TRACE_INFORMATION {
    TraceEnable: BOOLEAN,
    TracePermanent: BOOLEAN,
    TraceProcessName: UNICODE_STRING,
    TracePoolTags: UNICODE_STRING,
}}
pub type PSYSTEM_REF_TRACE_INFORMATION = *mut SYSTEM_REF_TRACE_INFORMATION;
STRUCT!{struct SYSTEM_PROCESS_ID_INFORMATION {
    ProcessId: HANDLE,
    ImageName: UNICODE_STRING,
}}
pub type PSYSTEM_PROCESS_ID_INFORMATION = *mut SYSTEM_PROCESS_ID_INFORMATION;
STRUCT!{struct SYSTEM_BOOT_ENVIRONMENT_INFORMATION {
    BootIdentifier: GUID,
    FirmwareType: FIRMWARE_TYPE,
    BootFlags: ULONGLONG,
}}
BITFIELD!{SYSTEM_BOOT_ENVIRONMENT_INFORMATION BootFlags: ULONGLONG [
    DbgMenuOsSelection set_DbgMenuOsSelection[0..1],
    DbgHiberBoot set_DbgHiberBoot[1..2],
    DbgSoftBoot set_DbgSoftBoot[2..3],
    DbgMeasuredLaunch set_DbgMeasuredLaunch[3..4],
]}
pub type PSYSTEM_BOOT_ENVIRONMENT_INFORMATION = *mut SYSTEM_BOOT_ENVIRONMENT_INFORMATION;
STRUCT!{struct SYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION {
    FlagsToEnable: ULONG,
    FlagsToDisable: ULONG,
}}
pub type PSYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION =
    *mut SYSTEM_IMAGE_FILE_EXECUTION_OPTIONS_INFORMATION;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
STRUCT!{struct SYSTEM_VERIFIER_INFORMATION_EX {
    VerifyMode: ULONG,
    OptionChanges: ULONG,
    PreviousBucketName: UNICODE_STRING,
    IrpCancelTimeoutMsec: ULONG,
    VerifierExtensionEnabled: ULONG,
    Reserved: [ULONG; 1],
}}
#[cfg(target_arch = "x86")]
STRUCT!{struct SYSTEM_VERIFIER_INFORMATION_EX {
    VerifyMode: ULONG,
    OptionChanges: ULONG,
    PreviousBucketName: UNICODE_STRING,
    IrpCancelTimeoutMsec: ULONG,
    VerifierExtensionEnabled: ULONG,
    Reserved: [ULONG; 3],
}}
pub type PSYSTEM_VERIFIER_INFORMATION_EX = *mut SYSTEM_VERIFIER_INFORMATION_EX;
STRUCT!{struct SYSTEM_SYSTEM_PARTITION_INFORMATION {
    SystemPartition: UNICODE_STRING,
}}
pub type PSYSTEM_SYSTEM_PARTITION_INFORMATION = *mut SYSTEM_SYSTEM_PARTITION_INFORMATION;
STRUCT!{struct SYSTEM_SYSTEM_DISK_INFORMATION {
    SystemDisk: UNICODE_STRING,
}}
pub type PSYSTEM_SYSTEM_DISK_INFORMATION = *mut SYSTEM_SYSTEM_DISK_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT {
    Hits: ULONGLONG,
    PercentFrequency: UCHAR,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT = *mut SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT;
STRUCT!{struct SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8 {
    Hits: ULONG,
    PercentFrequency: UCHAR,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8 =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT_WIN8;
STRUCT!{struct SYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION {
    ProcessorNumber: ULONG,
    StateCount: ULONG,
    States: [SYSTEM_PROCESSOR_PERFORMANCE_HITCOUNT; 1],
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_STATE_DISTRIBUTION;
STRUCT!{struct SYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION {
    ProcessorCount: ULONG,
    Offsets: [ULONG; 1],
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_DISTRIBUTION;
STRUCT!{struct SYSTEM_CODEINTEGRITY_INFORMATION {
    Length: ULONG,
    CodeIntegrityOptions: ULONG,
}}
pub type PSYSTEM_CODEINTEGRITY_INFORMATION = *mut SYSTEM_CODEINTEGRITY_INFORMATION;
ENUM!{enum SYSTEM_VA_TYPE {
    SystemVaTypeAll = 0,
    SystemVaTypeNonPagedPool = 1,
    SystemVaTypePagedPool = 2,
    SystemVaTypeSystemCache = 3,
    SystemVaTypeSystemPtes = 4,
    SystemVaTypeSessionSpace = 5,
    SystemVaTypeMax = 6,
}}
pub type PSYSTEM_VA_TYPE = *mut SYSTEM_VA_TYPE;
STRUCT!{struct SYSTEM_VA_LIST_INFORMATION {
    VirtualSize: SIZE_T,
    VirtualPeak: SIZE_T,
    VirtualLimit: SIZE_T,
    AllocationFailures: SIZE_T,
}}
pub type PSYSTEM_VA_LIST_INFORMATION = *mut SYSTEM_VA_LIST_INFORMATION;
STRUCT!{struct SYSTEM_REGISTRY_APPEND_STRING_PARAMETERS {
    KeyHandle: HANDLE,
    ValueNamePointer: PUNICODE_STRING,
    RequiredLengthPointer: PULONG,
    Buffer: PUCHAR,
    BufferLength: ULONG,
    Type: ULONG,
    AppendBuffer: PUCHAR,
    AppendBufferLength: ULONG,
    CreateIfDoesntExist: BOOLEAN,
    TruncateExistingValue: BOOLEAN,
}}
pub type PSYSTEM_REGISTRY_APPEND_STRING_PARAMETERS = *mut SYSTEM_REGISTRY_APPEND_STRING_PARAMETERS;
STRUCT!{struct SYSTEM_VHD_BOOT_INFORMATION {
    OsDiskIsVhd: BOOLEAN,
    OsVhdFilePathOffset: ULONG,
    OsVhdParentVolume: [WCHAR; ANYSIZE_ARRAY],
}}
pub type PSYSTEM_VHD_BOOT_INFORMATION = *mut SYSTEM_VHD_BOOT_INFORMATION;
STRUCT!{struct SYSTEM_LOW_PRIORITY_IO_INFORMATION {
    LowPriReadOperations: ULONG,
    LowPriWriteOperations: ULONG,
    KernelBumpedToNormalOperations: ULONG,
    LowPriPagingReadOperations: ULONG,
    KernelPagingReadsBumpedToNormal: ULONG,
    LowPriPagingWriteOperations: ULONG,
    KernelPagingWritesBumpedToNormal: ULONG,
    BoostedIrpCount: ULONG,
    BoostedPagingIrpCount: ULONG,
    BlanketBoostCount: ULONG,
}}
pub type PSYSTEM_LOW_PRIORITY_IO_INFORMATION = *mut SYSTEM_LOW_PRIORITY_IO_INFORMATION;
ENUM!{enum TPM_BOOT_ENTROPY_RESULT_CODE {
    TpmBootEntropyStructureUninitialized = 0,
    TpmBootEntropyDisabledByPolicy = 1,
    TpmBootEntropyNoTpmFound = 2,
    TpmBootEntropyTpmError = 3,
    TpmBootEntropySuccess = 4,
}}
STRUCT!{struct TPM_BOOT_ENTROPY_NT_RESULT {
    Policy: ULONGLONG,
    ResultCode: TPM_BOOT_ENTROPY_RESULT_CODE,
    ResultStatus: NTSTATUS,
    Time: ULONGLONG,
    EntropyLength: ULONG,
    EntropyData: [UCHAR; 40],
}}
pub type PTPM_BOOT_ENTROPY_NT_RESULT = *mut TPM_BOOT_ENTROPY_NT_RESULT;
STRUCT!{struct SYSTEM_VERIFIER_COUNTERS_INFORMATION {
    Legacy: SYSTEM_VERIFIER_INFORMATION,
    RaiseIrqls: ULONG,
    AcquireSpinLocks: ULONG,
    SynchronizeExecutions: ULONG,
    AllocationsWithNoTag: ULONG,
    AllocationsFailed: ULONG,
    AllocationsFailedDeliberately: ULONG,
    LockedBytes: SIZE_T,
    PeakLockedBytes: SIZE_T,
    MappedLockedBytes: SIZE_T,
    PeakMappedLockedBytes: SIZE_T,
    MappedIoSpaceBytes: SIZE_T,
    PeakMappedIoSpaceBytes: SIZE_T,
    PagesForMdlBytes: SIZE_T,
    PeakPagesForMdlBytes: SIZE_T,
    ContiguousMemoryBytes: SIZE_T,
    PeakContiguousMemoryBytes: SIZE_T,
    ExecutePoolTypes: ULONG,
    ExecutePageProtections: ULONG,
    ExecutePageMappings: ULONG,
    ExecuteWriteSections: ULONG,
    SectionAlignmentFailures: ULONG,
    UnsupportedRelocs: ULONG,
    IATInExecutableSection: ULONG,
}}
pub type PSYSTEM_VERIFIER_COUNTERS_INFORMATION = *mut SYSTEM_VERIFIER_COUNTERS_INFORMATION;
STRUCT!{struct SYSTEM_ACPI_AUDIT_INFORMATION {
    RsdpCount: ULONG,
    Bitfields: ULONG,
}}
BITFIELD!{SYSTEM_ACPI_AUDIT_INFORMATION Bitfields: ULONG [
    SameRsdt set_SameRsdt[0..1],
    SlicPresent set_SlicPresent[1..2],
    SlicDifferent set_SlicDifferent[2..3],
]}
pub type PSYSTEM_ACPI_AUDIT_INFORMATION = *mut SYSTEM_ACPI_AUDIT_INFORMATION;
STRUCT!{struct SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    AvailablePages: SIZE_T,
    CommittedPages: SIZE_T,
    CommitLimit: SIZE_T,
    PeakCommitment: SIZE_T,
}}
pub type PSYSTEM_BASIC_PERFORMANCE_INFORMATION = *mut SYSTEM_BASIC_PERFORMANCE_INFORMATION;
STRUCT!{struct QUERY_PERFORMANCE_COUNTER_FLAGS {
    ul: ULONG,
}}
BITFIELD!{QUERY_PERFORMANCE_COUNTER_FLAGS ul: ULONG [
    KernelTransition set_KernelTransition[0..1],
    Reserved set_Reserved[1..32],
]}
STRUCT!{struct SYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION {
    Version: ULONG,
    Flags: QUERY_PERFORMANCE_COUNTER_FLAGS,
    ValidFlags: QUERY_PERFORMANCE_COUNTER_FLAGS,
}}
pub type PSYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION =
    *mut SYSTEM_QUERY_PERFORMANCE_COUNTER_INFORMATION;
ENUM!{enum SYSTEM_PIXEL_FORMAT {
    SystemPixelFormatUnknown = 0,
    SystemPixelFormatR8G8B8 = 1,
    SystemPixelFormatR8G8B8X8 = 2,
    SystemPixelFormatB8G8R8 = 3,
    SystemPixelFormatB8G8R8X8 = 4,
}}
STRUCT!{struct SYSTEM_BOOT_GRAPHICS_INFORMATION {
    FrameBuffer: LARGE_INTEGER,
    Width: ULONG,
    Height: ULONG,
    PixelStride: ULONG,
    Flags: ULONG,
    Format: SYSTEM_PIXEL_FORMAT,
    DisplayRotation: ULONG,
}}
pub type PSYSTEM_BOOT_GRAPHICS_INFORMATION = *mut SYSTEM_BOOT_GRAPHICS_INFORMATION;
STRUCT!{struct MEMORY_SCRUB_INFORMATION {
    Handle: HANDLE,
    PagesScrubbed: ULONG,
}}
pub type PMEMORY_SCRUB_INFORMATION = *mut MEMORY_SCRUB_INFORMATION;
STRUCT!{struct PEBS_DS_SAVE_AREA {
    BtsBufferBase: ULONGLONG,
    BtsIndex: ULONGLONG,
    BtsAbsoluteMaximum: ULONGLONG,
    BtsInterruptThreshold: ULONGLONG,
    PebsBufferBase: ULONGLONG,
    PebsIndex: ULONGLONG,
    PebsAbsoluteMaximum: ULONGLONG,
    PebsInterruptThreshold: ULONGLONG,
    PebsCounterReset0: ULONGLONG,
    PebsCounterReset1: ULONGLONG,
    PebsCounterReset2: ULONGLONG,
    PebsCounterReset3: ULONGLONG,
}}
pub type PPEBS_DS_SAVE_AREA = *mut PEBS_DS_SAVE_AREA;
STRUCT!{struct PROCESSOR_PROFILE_CONTROL_AREA {
    PebsDsSaveArea: PEBS_DS_SAVE_AREA,
}}
pub type PPROCESSOR_PROFILE_CONTROL_AREA = *mut PROCESSOR_PROFILE_CONTROL_AREA;
STRUCT!{struct SYSTEM_PROCESSOR_PROFILE_CONTROL_AREA {
    ProcessorProfileControlArea: PROCESSOR_PROFILE_CONTROL_AREA,
    Allocate: BOOLEAN,
}}
pub type PSYSTEM_PROCESSOR_PROFILE_CONTROL_AREA = *mut SYSTEM_PROCESSOR_PROFILE_CONTROL_AREA;
STRUCT!{struct MEMORY_COMBINE_INFORMATION {
    Handle: HANDLE,
    PagesCombined: ULONG_PTR,
}}
pub type PMEMORY_COMBINE_INFORMATION = *mut MEMORY_COMBINE_INFORMATION;
pub const MEMORY_COMBINE_FLAGS_COMMON_PAGES_ONLY: ULONG = 0x4;
STRUCT!{struct MEMORY_COMBINE_INFORMATION_EX {
    Handle: HANDLE,
    PagesCombined: ULONG_PTR,
    Flags: ULONG,
}}
pub type PMEMORY_COMBINE_INFORMATION_EX = *mut MEMORY_COMBINE_INFORMATION_EX;
STRUCT!{struct MEMORY_COMBINE_INFORMATION_EX2 {
    Handle: HANDLE,
    PagesCombined: ULONG_PTR,
    Flags: ULONG,
    ProcessHandle: HANDLE,
}}
pub type PMEMORY_COMBINE_INFORMATION_EX2 = *mut MEMORY_COMBINE_INFORMATION_EX2;
STRUCT!{struct SYSTEM_CONSOLE_INFORMATION {
    Bitfields: ULONG,
}}
BITFIELD!{SYSTEM_CONSOLE_INFORMATION Bitfields: ULONG [
    DriverLoaded set_DriverLoaded[0..1],
    Spare set_Spare[1..32],
]}
pub type PSYSTEM_CONSOLE_INFORMATION = *mut SYSTEM_CONSOLE_INFORMATION;
STRUCT!{struct SYSTEM_PLATFORM_BINARY_INFORMATION {
    PhysicalAddress: ULONG64,
    HandoffBuffer: PVOID,
    CommandLineBuffer: PVOID,
    HandoffBufferSize: ULONG,
    CommandLineBufferSize: ULONG,
}}
pub type PSYSTEM_PLATFORM_BINARY_INFORMATION = *mut SYSTEM_PLATFORM_BINARY_INFORMATION;
STRUCT!{struct SYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION {
    NumberOfLogicalProcessors: ULONG,
    NumberOfCores: ULONG,
}}
pub type PSYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION =
    *mut SYSTEM_HYPERVISOR_PROCESSOR_COUNT_INFORMATION;
STRUCT!{struct SYSTEM_DEVICE_DATA_INFORMATION {
    DeviceId: UNICODE_STRING,
    DataName: UNICODE_STRING,
    DataType: ULONG,
    DataBufferLength: ULONG,
    DataBuffer: PVOID,
}}
pub type PSYSTEM_DEVICE_DATA_INFORMATION = *mut SYSTEM_DEVICE_DATA_INFORMATION;
STRUCT!{struct PHYSICAL_CHANNEL_RUN {
    NodeNumber: ULONG,
    ChannelNumber: ULONG,
    BasePage: ULONGLONG,
    PageCount: ULONGLONG,
    Flags: ULONG,
}}
pub type PPHYSICAL_CHANNEL_RUN = *mut PHYSICAL_CHANNEL_RUN;
STRUCT!{struct SYSTEM_MEMORY_TOPOLOGY_INFORMATION {
    NumberOfRuns: ULONGLONG,
    NumberOfNodes: ULONG,
    NumberOfChannels: ULONG,
    Run: [PHYSICAL_CHANNEL_RUN; 1],
}}
pub type PSYSTEM_MEMORY_TOPOLOGY_INFORMATION = *mut SYSTEM_MEMORY_TOPOLOGY_INFORMATION;
STRUCT!{struct SYSTEM_MEMORY_CHANNEL_INFORMATION {
    ChannelNumber: ULONG,
    ChannelHeatIndex: ULONG,
    TotalPageCount: ULONGLONG,
    ZeroPageCount: ULONGLONG,
    FreePageCount: ULONGLONG,
    StandbyPageCount: ULONGLONG,
}}
pub type PSYSTEM_MEMORY_CHANNEL_INFORMATION = *mut SYSTEM_MEMORY_CHANNEL_INFORMATION;
STRUCT!{struct SYSTEM_BOOT_LOGO_INFORMATION {
    Flags: ULONG,
    BitmapOffset: ULONG,
}}
pub type PSYSTEM_BOOT_LOGO_INFORMATION = *mut SYSTEM_BOOT_LOGO_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX {
    IdleTime: LARGE_INTEGER,
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    DpcTime: LARGE_INTEGER,
    InterruptTime: LARGE_INTEGER,
    InterruptCount: ULONG,
    Spare0: ULONG,
    AvailableTime: LARGE_INTEGER,
    Spare1: LARGE_INTEGER,
    Spare2: LARGE_INTEGER,
}}
pub type PSYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX =
    *mut SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_EX;
STRUCT!{struct SYSTEM_SECUREBOOT_POLICY_INFORMATION {
    PolicyPublisher: GUID,
    PolicyVersion: ULONG,
    PolicyOptions: ULONG,
}}
pub type PSYSTEM_SECUREBOOT_POLICY_INFORMATION = *mut SYSTEM_SECUREBOOT_POLICY_INFORMATION;
STRUCT!{struct SYSTEM_PAGEFILE_INFORMATION_EX {
    Info: SYSTEM_PAGEFILE_INFORMATION,
    MinimumSize: ULONG,
    MaximumSize: ULONG,
}}
pub type PSYSTEM_PAGEFILE_INFORMATION_EX = *mut SYSTEM_PAGEFILE_INFORMATION_EX;
STRUCT!{struct SYSTEM_SECUREBOOT_INFORMATION {
    SecureBootEnabled: BOOLEAN,
    SecureBootCapable: BOOLEAN,
}}
pub type PSYSTEM_SECUREBOOT_INFORMATION = *mut SYSTEM_SECUREBOOT_INFORMATION;
STRUCT!{struct PROCESS_DISK_COUNTERS {
    BytesRead: ULONGLONG,
    BytesWritten: ULONGLONG,
    ReadOperationCount: ULONGLONG,
    WriteOperationCount: ULONGLONG,
    FlushOperationCount: ULONGLONG,
}}
pub type PPROCESS_DISK_COUNTERS = *mut PROCESS_DISK_COUNTERS;
UNION!{union ENERGY_STATE_DURATION_u {
    Value: ULONGLONG,
    LastChangeTime: ULONG,
}}
UNION!{union ENERGY_STATE_DURATION {
    u: ENERGY_STATE_DURATION_u,
    BitFields: ULONG,
}}
pub type PENERGY_STATE_DURATION = *mut ENERGY_STATE_DURATION;
BITFIELD!{unsafe ENERGY_STATE_DURATION BitFields: ULONG [
    Duration set_Duration[0..31],
    IsInState set_IsInState[31..32],
]}
STRUCT!{struct PROCESS_ENERGY_VALUES {
    Cycles: [[ULONGLONG; 4]; 2],
    DiskEnergy: ULONGLONG,
    NetworkTailEnergy: ULONGLONG,
    MBBTailEnergy: ULONGLONG,
    NetworkTxRxBytes: ULONGLONG,
    MBBTxRxBytes: ULONGLONG,
    ForegroundDuration: ENERGY_STATE_DURATION,
    DesktopVisibleDuration: ENERGY_STATE_DURATION,
    PSMForegroundDuration: ENERGY_STATE_DURATION,
    CompositionRendered: ULONG,
    CompositionDirtyGenerated: ULONG,
    CompositionDirtyPropagated: ULONG,
    Reserved1: ULONG,
    AttributedCycles: [[ULONGLONG; 2]; 4],
    WorkOnBehalfCycles: [[ULONGLONG; 2]; 4],
}}
pub type PPROCESS_ENERGY_VALUES = *mut PROCESS_ENERGY_VALUES;
STRUCT!{struct TIMELINE_BITMAP {
    Value: ULONGLONG,
    EndTime: ULONG,
    Bitmap: ULONG,
}}
pub type PTIMELINE_BITMAP = *mut TIMELINE_BITMAP;
STRUCT!{struct PROCESS_ENERGY_VALUES_EXTENSION_Timelines {
    CpuTimeline: TIMELINE_BITMAP,
    DiskTimeline: TIMELINE_BITMAP,
    NetworkTimeline: TIMELINE_BITMAP,
    MBBTimeline: TIMELINE_BITMAP,
    ForegroundTimeline: TIMELINE_BITMAP,
    DesktopVisibleTimeline: TIMELINE_BITMAP,
    CompositionRenderedTimeline: TIMELINE_BITMAP,
    CompositionDirtyGeneratedTimeline: TIMELINE_BITMAP,
    CompositionDirtyPropagatedTimeline: TIMELINE_BITMAP,
    InputTimeline: TIMELINE_BITMAP,
    AudioInTimeline: TIMELINE_BITMAP,
    AudioOutTimeline: TIMELINE_BITMAP,
    DisplayRequiredTimeline: TIMELINE_BITMAP,
    KeyboardInputTimeline: TIMELINE_BITMAP,
}}
STRUCT!{struct PROCESS_ENERGY_VALUES_EXTENSION_Durations {
    InputDuration: ENERGY_STATE_DURATION,
    AudioInDuration: ENERGY_STATE_DURATION,
    AudioOutDuration: ENERGY_STATE_DURATION,
    DisplayRequiredDuration: ENERGY_STATE_DURATION,
    PSMBackgroundDuration: ENERGY_STATE_DURATION,
}}
STRUCT!{struct PROCESS_ENERGY_VALUES_EXTENSION {
    Timelines: PROCESS_ENERGY_VALUES_EXTENSION_Timelines,
    Durations: PROCESS_ENERGY_VALUES_EXTENSION_Durations,
    KeyboardInput: ULONG,
    MouseInput: ULONG,
}}
pub type PPROCESS_ENERGY_VALUES_EXTENSION = *mut PROCESS_ENERGY_VALUES_EXTENSION;
STRUCT!{struct PROCESS_EXTENDED_ENERGY_VALUES {
    Base: PROCESS_ENERGY_VALUES,
    Extension: PROCESS_ENERGY_VALUES_EXTENSION,
}}
pub type PPROCESS_EXTENDED_ENERGY_VALUES = *mut PROCESS_EXTENDED_ENERGY_VALUES;
ENUM!{enum SYSTEM_PROCESS_CLASSIFICATION {
    SystemProcessClassificationNormal = 0,
    SystemProcessClassificationSystem = 1,
    SystemProcessClassificationSecureSystem = 2,
    SystemProcessClassificationMemCompression = 3,
    SystemProcessClassificationRegistry = 4,
    SystemProcessClassificationMaximum = 5,
}}
STRUCT!{struct SYSTEM_PROCESS_INFORMATION_EXTENSION {
    DiskCounters: PROCESS_DISK_COUNTERS,
    ContextSwitches: ULONGLONG,
    Flags: ULONG,
    UserSidOffset: ULONG,
    PackageFullNameOffset: ULONG,
    EnergyValues: PROCESS_ENERGY_VALUES,
    AppIdOffset: ULONG,
    SharedCommitCharge: SIZE_T,
    JobObjectId: ULONG,
    SpareUlong: ULONG,
    ProcessSequenceNumber: ULONGLONG,
}}
BITFIELD!{SYSTEM_PROCESS_INFORMATION_EXTENSION Flags: ULONG [
    HasStrongId set_HasStrongId[0..1],
    Classification set_Classification[1..5],
    BackgroundActivityModerated set_BackgroundActivityModerated[5..6],
    Spare set_Spare[6..32],
]}
pub type PSYSTEM_PROCESS_INFORMATION_EXTENSION = *mut SYSTEM_PROCESS_INFORMATION_EXTENSION;
STRUCT!{struct SYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION {
    EfiLauncherEnabled: BOOLEAN,
}}
pub type PSYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION =
    *mut SYSTEM_PORTABLE_WORKSPACE_EFI_LAUNCHER_INFORMATION;
STRUCT!{struct SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX {
    DebuggerAllowed: BOOLEAN,
    DebuggerEnabled: BOOLEAN,
    DebuggerPresent: BOOLEAN,
}}
pub type PSYSTEM_KERNEL_DEBUGGER_INFORMATION_EX = *mut SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX;
STRUCT!{struct SYSTEM_ELAM_CERTIFICATE_INFORMATION {
    ElamDriverFile: HANDLE,
}}
pub type PSYSTEM_ELAM_CERTIFICATE_INFORMATION = *mut SYSTEM_ELAM_CERTIFICATE_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_FEATURES_INFORMATION {
    ProcessorFeatureBits: ULONGLONG,
    Reserved: [ULONGLONG; 3],
}}
pub type PSYSTEM_PROCESSOR_FEATURES_INFORMATION = *mut SYSTEM_PROCESSOR_FEATURES_INFORMATION;
STRUCT!{struct SYSTEM_MANUFACTURING_INFORMATION {
    Options: ULONG,
    ProfileName: UNICODE_STRING,
}}
pub type PSYSTEM_MANUFACTURING_INFORMATION = *mut SYSTEM_MANUFACTURING_INFORMATION;
STRUCT!{struct SYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION {
    Enabled: BOOLEAN,
}}
pub type PSYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION =
    *mut SYSTEM_ENERGY_ESTIMATION_CONFIG_INFORMATION;
STRUCT!{struct HV_DETAILS {
    Data: [ULONG; 4],
}}
pub type PHV_DETAILS = *mut HV_DETAILS;
STRUCT!{struct SYSTEM_HYPERVISOR_DETAIL_INFORMATION {
    HvVendorAndMaxFunction: HV_DETAILS,
    HypervisorInterface: HV_DETAILS,
    HypervisorVersion: HV_DETAILS,
    HvFeatures: HV_DETAILS,
    HwFeatures: HV_DETAILS,
    EnlightenmentInfo: HV_DETAILS,
    ImplementationLimits: HV_DETAILS,
}}
pub type PSYSTEM_HYPERVISOR_DETAIL_INFORMATION = *mut SYSTEM_HYPERVISOR_DETAIL_INFORMATION;
STRUCT!{struct SYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION {
    Cycles: [[ULONGLONG; 4]; 2],
}}
pub type PSYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION = *mut SYSTEM_PROCESSOR_CYCLE_STATS_INFORMATION;
STRUCT!{struct SYSTEM_TPM_INFORMATION {
    Flags: ULONG,
}}
pub type PSYSTEM_TPM_INFORMATION = *mut SYSTEM_TPM_INFORMATION;
STRUCT!{struct SYSTEM_VSM_PROTECTION_INFORMATION {
    DmaProtectionsAvailable: BOOLEAN,
    DmaProtectionsInUse: BOOLEAN,
    HardwareMbecAvailable: BOOLEAN,
}}
pub type PSYSTEM_VSM_PROTECTION_INFORMATION = *mut SYSTEM_VSM_PROTECTION_INFORMATION;
STRUCT!{struct SYSTEM_CODEINTEGRITYPOLICY_INFORMATION {
    Options: ULONG,
    HVCIOptions: ULONG,
    Version: ULONGLONG,
    PolicyGuid: GUID,
}}
pub type PSYSTEM_CODEINTEGRITYPOLICY_INFORMATION = *mut SYSTEM_CODEINTEGRITYPOLICY_INFORMATION;
STRUCT!{struct SYSTEM_ISOLATED_USER_MODE_INFORMATION {
    Bitfields1: BOOLEAN,
    Bitfields2: BOOLEAN,
    Spare0: [BOOLEAN; 6],
    Spare1: ULONGLONG,
}}
BITFIELD!{SYSTEM_ISOLATED_USER_MODE_INFORMATION Bitfields1: BOOLEAN [
    SecureKernelRunning set_SecureKernelRunning[0..1],
    HvciEnabled set_HvciEnabled[1..2],
    HvciStrictMode set_HvciStrictMode[2..3],
    DebugEnabled set_DebugEnabled[3..4],
    FirmwarePageProtection set_FirmwarePageProtection[4..5],
    EncryptionKeyAvailable set_EncryptionKeyAvailable[5..6],
    SpareFlags set_SpareFlags[6..7],
    TrustletRunning set_TrustletRunning[7..8],
]}
BITFIELD!{SYSTEM_ISOLATED_USER_MODE_INFORMATION Bitfields2: BOOLEAN [
    SpareFlags2 set_SpareFlags2[0..1],
]}
pub type PSYSTEM_ISOLATED_USER_MODE_INFORMATION = *mut SYSTEM_ISOLATED_USER_MODE_INFORMATION;
STRUCT!{struct SYSTEM_SINGLE_MODULE_INFORMATION {
    TargetModuleAddress: PVOID,
    ExInfo: RTL_PROCESS_MODULE_INFORMATION_EX,
}}
pub type PSYSTEM_SINGLE_MODULE_INFORMATION = *mut SYSTEM_SINGLE_MODULE_INFORMATION;
STRUCT!{struct SYSTEM_INTERRUPT_CPU_SET_INFORMATION {
    Gsiv: ULONG,
    Group: USHORT,
    CpuSets: ULONGLONG,
}}
pub type PSYSTEM_INTERRUPT_CPU_SET_INFORMATION = *mut SYSTEM_INTERRUPT_CPU_SET_INFORMATION;
STRUCT!{struct SYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION {
    PolicyInformation: SYSTEM_SECUREBOOT_POLICY_INFORMATION,
    PolicySize: ULONG,
    Policy: [UCHAR; 1],
}}
pub type PSYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION =
    *mut SYSTEM_SECUREBOOT_POLICY_FULL_INFORMATION;
STRUCT!{struct SYSTEM_ROOT_SILO_INFORMATION {
    NumberOfSilos: ULONG,
    SiloIdList: [ULONG; 1],
}}
pub type PSYSTEM_ROOT_SILO_INFORMATION = *mut SYSTEM_ROOT_SILO_INFORMATION;
STRUCT!{struct SYSTEM_CPU_SET_TAG_INFORMATION {
    Tag: ULONGLONG,
    CpuSets: [ULONGLONG; 1],
}}
pub type PSYSTEM_CPU_SET_TAG_INFORMATION = *mut SYSTEM_CPU_SET_TAG_INFORMATION;
STRUCT!{struct SYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION {
    ExtentCount: ULONG,
    ValidStructureSize: ULONG,
    NextExtentIndex: ULONG,
    ExtentRestart: ULONG,
    CycleCount: ULONG,
    TimeoutCount: ULONG,
    CycleTime: ULONGLONG,
    CycleTimeMax: ULONGLONG,
    ExtentTime: ULONGLONG,
    ExtentTimeIndex: ULONG,
    ExtentTimeMaxIndex: ULONG,
    ExtentTimeMax: ULONGLONG,
    HyperFlushTimeMax: ULONGLONG,
    TranslateVaTimeMax: ULONGLONG,
    DebugExemptionCount: ULONGLONG,
    TbHitCount: ULONGLONG,
    TbMissCount: ULONGLONG,
    VinaPendingYield: ULONGLONG,
    HashCycles: ULONGLONG,
    HistogramOffset: ULONG,
    HistogramBuckets: ULONG,
    HistogramShift: ULONG,
    Reserved1: ULONG,
    PageNotPresentCount: ULONGLONG,
}}
pub type PSYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION =
    *mut SYSTEM_SECURE_KERNEL_HYPERGUARD_PROFILE_INFORMATION;
STRUCT!{struct SYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION {
    PlatformManifestSize: ULONG,
    PlatformManifest: [UCHAR; 1],
}}
pub type PSYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION =
    *mut SYSTEM_SECUREBOOT_PLATFORM_MANIFEST_INFORMATION;
STRUCT!{struct SYSTEM_MEMORY_USAGE_INFORMATION {
    TotalPhysicalBytes: ULONGLONG,
    AvailableBytes: ULONGLONG,
    ResidentAvailableBytes: LONGLONG,
    CommittedBytes: ULONGLONG,
    SharedCommittedBytes: ULONGLONG,
    CommitLimitBytes: ULONGLONG,
    PeakCommitmentBytes: ULONGLONG,
}}
pub type PSYSTEM_MEMORY_USAGE_INFORMATION = *mut SYSTEM_MEMORY_USAGE_INFORMATION;
STRUCT!{struct SYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION {
    ImageFile: HANDLE,
    Type: ULONG,
}}
pub type PSYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION =
    *mut SYSTEM_CODEINTEGRITY_CERTIFICATE_INFORMATION;
STRUCT!{struct SYSTEM_PHYSICAL_MEMORY_INFORMATION {
    TotalPhysicalBytes: ULONGLONG,
    LowestPhysicalAddress: ULONGLONG,
    HighestPhysicalAddress: ULONGLONG,
}}
pub type PSYSTEM_PHYSICAL_MEMORY_INFORMATION = *mut SYSTEM_PHYSICAL_MEMORY_INFORMATION;
ENUM!{enum SYSTEM_ACTIVITY_MODERATION_STATE {
    SystemActivityModerationStateSystemManaged = 0,
    SystemActivityModerationStateUserManagedAllowThrottling = 1,
    SystemActivityModerationStateUserManagedDisableThrottling = 2,
    MaxSystemActivityModerationState = 3,
}}
ENUM!{enum SYSTEM_ACTIVITY_MODERATION_APP_TYPE {
    SystemActivityModerationAppTypeClassic = 0,
    SystemActivityModerationAppTypePackaged = 1,
    MaxSystemActivityModerationAppType = 2,
}}
STRUCT!{struct SYSTEM_ACTIVITY_MODERATION_INFO {
    Identifier: UNICODE_STRING,
    ModerationState: SYSTEM_ACTIVITY_MODERATION_STATE,
    AppType: SYSTEM_ACTIVITY_MODERATION_APP_TYPE,
}}
pub type PSYSTEM_ACTIVITY_MODERATION_INFO = *mut SYSTEM_ACTIVITY_MODERATION_INFO;
STRUCT!{struct SYSTEM_ACTIVITY_MODERATION_USER_SETTINGS {
    UserKeyHandle: HANDLE,
}}
pub type PSYSTEM_ACTIVITY_MODERATION_USER_SETTINGS = *mut SYSTEM_ACTIVITY_MODERATION_USER_SETTINGS;
STRUCT!{struct SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION {
    Flags: ULONG,
    UnlockId: [UCHAR; 32],
}}
BITFIELD!{SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION Flags: ULONG [
    Locked set_Locked[0..1],
    Unlockable set_Unlockable[1..2],
    UnlockApplied set_UnlockApplied[2..3],
    UnlockIdValid set_UnlockIdValid[3..4],
    Reserved set_Reserved[4..32],
]}
pub type PSYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION = *mut SYSTEM_CODEINTEGRITY_UNLOCK_INFORMATION;
STRUCT!{struct SYSTEM_FLUSH_INFORMATION {
    SupportedFlushMethods: ULONG,
    ProcessorCacheFlushSize: ULONG,
    SystemFlushCapabilities: ULONGLONG,
    Reserved: [ULONGLONG; 2],
}}
pub type PSYSTEM_FLUSH_INFORMATION = *mut SYSTEM_FLUSH_INFORMATION;
STRUCT!{struct SYSTEM_WRITE_CONSTRAINT_INFORMATION {
    WriteConstraintPolicy: ULONG,
    Reserved: ULONG,
}}
pub type PSYSTEM_WRITE_CONSTRAINT_INFORMATION = *mut SYSTEM_WRITE_CONSTRAINT_INFORMATION;
STRUCT!{struct SYSTEM_KERNEL_VA_SHADOW_INFORMATION {
    Flags: ULONG,
}}
BITFIELD!{SYSTEM_KERNEL_VA_SHADOW_INFORMATION Flags: ULONG [
    KvaShadowEnabled set_KvaShadowEnabled[0..1],
    KvaShadowUserGlobal set_KvaShadowUserGlobal[1..2],
    KvaShadowPcid set_KvaShadowPcid[2..3],
    KvaShadowInvpcid set_KvaShadowInvpcid[3..4],
    KvaShadowRequired set_KvaShadowRequired[4..5],
    KvaShadowRequiredAvailable set_KvaShadowRequiredAvailable[5..6],
    InvalidPteBit set_InvalidPteBit[6..12],
    L1DataCacheFlushSupported set_L1DataCacheFlushSupported[12..13],
    L1TerminalFaultMitigationPresent set_L1TerminalFaultMitigationPresent[13..14],
    Reserved set_Reserved[14..32],
]}
pub type PSYSTEM_KERNEL_VA_SHADOW_INFORMATION = *mut SYSTEM_KERNEL_VA_SHADOW_INFORMATION;
STRUCT!{struct SYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION {
    FileHandle: HANDLE,
    ImageSize: ULONG,
    Image: PVOID,
}}
pub type PSYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION =
    *mut SYSTEM_CODEINTEGRITYVERIFICATION_INFORMATION;
STRUCT!{struct SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION {
    HypervisorSharedUserVa: PVOID,
}}
pub type PSYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION =
    *mut SYSTEM_HYPERVISOR_SHARED_PAGE_INFORMATION;
STRUCT!{struct SYSTEM_SPECULATION_CONTROL_INFORMATION {
    Flags: ULONG,
}}
BITFIELD!{SYSTEM_SPECULATION_CONTROL_INFORMATION Flags: ULONG [
    BpbEnabled set_BpbEnabled[0..1],
    BpbDisabledSystemPolicy set_BpbDisabledSystemPolicy[1..2],
    BpbDisabledNoHardwareSupport set_BpbDisabledNoHardwareSupport[2..3],
    SpecCtrlEnumerated set_SpecCtrlEnumerated[3..4],
    SpecCmdEnumerated set_SpecCmdEnumerated[4..5],
    IbrsPresent set_IbrsPresent[5..6],
    StibpPresent set_StibpPresent[6..7],
    SmepPresent set_SmepPresent[7..8],
    SpeculativeStoreBypassDisableAvailable set_SpeculativeStoreBypassDisableAvailable[8..9],
    SpeculativeStoreBypassDisableSupported set_SpeculativeStoreBypassDisableSupported[9..10],
    SpeculativeStoreBypassDisabledSystemWide set_SpeculativeStoreBypassDisabledSystemWide[10..11],
    SpeculativeStoreBypassDisabledKernel set_SpeculativeStoreBypassDisabledKernel[11..12],
    SpeculativeStoreBypassDisableRequired set_SpeculativeStoreBypassDisableRequired[12..13],
    BpbDisabledKernelToUser set_BpbDisabledKernelToUser[13..14],
    SpecCtrlRetpolineEnabled set_SpecCtrlRetpolineEnabled[14..15],
    SpecCtrlImportOptimizationEnabled set_SpecCtrlImportOptimizationEnabled[15..16],
    Reserved set_Reserved[16..32],
]}
pub type PSYSTEM_SPECULATION_CONTROL_INFORMATION = *mut SYSTEM_SPECULATION_CONTROL_INFORMATION;
STRUCT!{struct SYSTEM_DMA_GUARD_POLICY_INFORMATION {
    DmaGuardPolicyEnabled: BOOLEAN,
}}
pub type PSYSTEM_DMA_GUARD_POLICY_INFORMATION = *mut SYSTEM_DMA_GUARD_POLICY_INFORMATION;
STRUCT!{struct SYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION {
    EnclaveLaunchSigner: [UCHAR; 32],
}}
pub type PSYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION =
    *mut SYSTEM_ENCLAVE_LAUNCH_CONTROL_INFORMATION;
STRUCT!{struct SYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION {
    WorkloadClass: ULONGLONG,
    CpuSets: [ULONGLONG; 1],
}}
pub type PSYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION =
    *mut SYSTEM_WORKLOAD_ALLOWED_CPU_SET_INFORMATION;
EXTERN!{extern "system" {
    fn NtQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtQuerySystemInformationEx(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetSystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum SYSDBG_COMMAND {
    SysDbgQueryModuleInformation = 0,
    SysDbgQueryTraceInformation = 1,
    SysDbgSetTracepoint = 2,
    SysDbgSetSpecialCall = 3,
    SysDbgClearSpecialCalls = 4,
    SysDbgQuerySpecialCalls = 5,
    SysDbgBreakPoint = 6,
    SysDbgQueryVersion = 7,
    SysDbgReadVirtual = 8,
    SysDbgWriteVirtual = 9,
    SysDbgReadPhysical = 10,
    SysDbgWritePhysical = 11,
    SysDbgReadControlSpace = 12,
    SysDbgWriteControlSpace = 13,
    SysDbgReadIoSpace = 14,
    SysDbgWriteIoSpace = 15,
    SysDbgReadMsr = 16,
    SysDbgWriteMsr = 17,
    SysDbgReadBusData = 18,
    SysDbgWriteBusData = 19,
    SysDbgCheckLowMemory = 20,
    SysDbgEnableKernelDebugger = 21,
    SysDbgDisableKernelDebugger = 22,
    SysDbgGetAutoKdEnable = 23,
    SysDbgSetAutoKdEnable = 24,
    SysDbgGetPrintBufferSize = 25,
    SysDbgSetPrintBufferSize = 26,
    SysDbgGetKdUmExceptionEnable = 27,
    SysDbgSetKdUmExceptionEnable = 28,
    SysDbgGetTriageDump = 29,
    SysDbgGetKdBlockEnable = 30,
    SysDbgSetKdBlockEnable = 31,
    SysDbgRegisterForUmBreakInfo = 32,
    SysDbgGetUmBreakPid = 33,
    SysDbgClearUmBreakPid = 34,
    SysDbgGetUmAttachPid = 35,
    SysDbgClearUmAttachPid = 36,
    SysDbgGetLiveKernelDump = 37,
}}
pub type PSYSDBG_COMMAND = *mut SYSDBG_COMMAND;
STRUCT!{struct SYSDBG_VIRTUAL {
    Address: PVOID,
    Buffer: PVOID,
    Request: ULONG,
}}
pub type PSYSDBG_VIRTUAL = *mut SYSDBG_VIRTUAL;
STRUCT!{struct SYSDBG_PHYSICAL {
    Address: PHYSICAL_ADDRESS,
    Buffer: PVOID,
    Request: ULONG,
}}
pub type PSYSDBG_PHYSICAL = *mut SYSDBG_PHYSICAL;
STRUCT!{struct SYSDBG_CONTROL_SPACE {
    Address: ULONG64,
    Buffer: PVOID,
    Request: ULONG,
    Processor: ULONG,
}}
pub type PSYSDBG_CONTROL_SPACE = *mut SYSDBG_CONTROL_SPACE;
STRUCT!{struct SYSDBG_IO_SPACE {
    Address: ULONG64,
    Buffer: PVOID,
    Request: ULONG,
    InterfaceType: INTERFACE_TYPE,
    BusNumber: ULONG,
    AddressSpace: ULONG,
}}
pub type PSYSDBG_IO_SPACE = *mut SYSDBG_IO_SPACE;
STRUCT!{struct SYSDBG_MSR {
    Msr: ULONG,
    Data: ULONG64,
}}
pub type PSYSDBG_MSR = *mut SYSDBG_MSR;
STRUCT!{struct SYSDBG_BUS_DATA {
    Address: ULONG,
    Buffer: PVOID,
    Request: ULONG,
    BusDataType: BUS_DATA_TYPE,
    BusNumber: ULONG,
    SlotNumber: ULONG,
}}
pub type PSYSDBG_BUS_DATA = *mut SYSDBG_BUS_DATA;
STRUCT!{struct SYSDBG_TRIAGE_DUMP {
    Flags: ULONG,
    BugCheckCode: ULONG,
    BugCheckParam1: ULONG_PTR,
    BugCheckParam2: ULONG_PTR,
    BugCheckParam3: ULONG_PTR,
    BugCheckParam4: ULONG_PTR,
    ProcessHandles: ULONG,
    ThreadHandles: ULONG,
    Handles: PHANDLE,
}}
pub type PSYSDBG_TRIAGE_DUMP = *mut SYSDBG_TRIAGE_DUMP;
STRUCT!{struct SYSDBG_LIVEDUMP_CONTROL_FLAGS {
    AsUlong: ULONG,
}}
BITFIELD!{SYSDBG_LIVEDUMP_CONTROL_FLAGS AsUlong: ULONG [
    UseDumpStorageStack set_UseDumpStorageStack[0..1],
    CompressMemoryPagesData set_CompressMemoryPagesData[1..2],
    IncludeUserSpaceMemoryPages set_IncludeUserSpaceMemoryPages[2..3],
    AbortIfMemoryPressure set_AbortIfMemoryPressure[3..4],
    Reserved set_Reserved[4..32],
]}
pub type PSYSDBG_LIVEDUMP_CONTROL_FLAGS = *mut SYSDBG_LIVEDUMP_CONTROL_FLAGS;
STRUCT!{struct SYSDBG_LIVEDUMP_CONTROL_ADDPAGES {
    AsUlong: ULONG,
}}
BITFIELD!{SYSDBG_LIVEDUMP_CONTROL_ADDPAGES AsUlong: ULONG [
    HypervisorPages set_HypervisorPages[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PSYSDBG_LIVEDUMP_CONTROL_ADDPAGES = *mut SYSDBG_LIVEDUMP_CONTROL_ADDPAGES;
pub const SYSDBG_LIVEDUMP_CONTROL_VERSION: ULONG = 1;
STRUCT!{struct SYSDBG_LIVEDUMP_CONTROL {
    Version: ULONG,
    BugCheckCode: ULONG,
    BugCheckParam1: ULONG_PTR,
    BugCheckParam2: ULONG_PTR,
    BugCheckParam3: ULONG_PTR,
    BugCheckParam4: ULONG_PTR,
    DumpFileHandle: HANDLE,
    CancelEventHandle: HANDLE,
    Flags: SYSDBG_LIVEDUMP_CONTROL_FLAGS,
    AddPagesControl: SYSDBG_LIVEDUMP_CONTROL_ADDPAGES,
}}
pub type PSYSDBG_LIVEDUMP_CONTROL = *mut SYSDBG_LIVEDUMP_CONTROL;
EXTERN!{extern "system" {
    fn NtSystemDebugControl(
        Command: SYSDBG_COMMAND,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum HARDERROR_RESPONSE_OPTION {
    OptionAbortRetryIgnore = 0,
    OptionOk = 1,
    OptionOkCancel = 2,
    OptionRetryCancel = 3,
    OptionYesNo = 4,
    OptionYesNoCancel = 5,
    OptionShutdownSystem = 6,
    OptionOkNoWait = 7,
    OptionCancelTryContinue = 8,
}}
ENUM!{enum HARDERROR_RESPONSE {
    ResponseReturnToCaller = 0,
    ResponseNotHandled = 1,
    ResponseAbort = 2,
    ResponseCancel = 3,
    ResponseIgnore = 4,
    ResponseNo = 5,
    ResponseOk = 6,
    ResponseRetry = 7,
    ResponseYes = 8,
    ResponseTryAgain = 9,
    ResponseContinue = 10,
}}
pub const HARDERROR_OVERRIDE_ERRORMODE: ULONG = 0x10000000;
EXTERN!{extern "system" {
    fn NtRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: ULONG,
        UnicodeStringParameterMask: ULONG,
        Parameters: PULONG_PTR,
        ValidResponseOptions: ULONG,
        Response: PULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum ALTERNATIVE_ARCHITECTURE_TYPE {
    StandardDesign = 0,
    NEC98x86 = 1,
    EndAlternatives = 2,
}}
pub const PROCESSOR_FEATURE_MAX: usize = 64;
pub const MAX_WOW64_SHARED_ENTRIES: u32 = 16;
pub const NX_SUPPORT_POLICY_ALWAYSOFF: u32 = 0;
pub const NX_SUPPORT_POLICY_ALWAYSON: u32 = 1;
pub const NX_SUPPORT_POLICY_OPTIN: u32 = 2;
pub const NX_SUPPORT_POLICY_OPTOUT: u32 = 3;
UNION!{union KUSER_SHARED_DATA_u {
    TickCount: KSYSTEM_TIME,
    TickCountQuad: ULONG64,
    ReservedTickCountOverlay: [ULONG; 3],
}}
STRUCT!{#[repr(packed(4))] struct KUSER_SHARED_DATA {
    TickCountLowDeprecated: ULONG,
    TickCountMultiplier: ULONG,
    InterruptTime: KSYSTEM_TIME,
    SystemTime: KSYSTEM_TIME,
    TimeZoneBias: KSYSTEM_TIME,
    ImageNumberLow: USHORT,
    ImageNumberHigh: USHORT,
    NtSystemRoot: [WCHAR; 260],
    MaxStackTraceDepth: ULONG,
    CryptoExponent: ULONG,
    TimeZoneId: ULONG,
    LargePageMinimum: ULONG,
    AitSamplingValue: ULONG,
    AppCompatFlag: ULONG,
    RNGSeedVersion: ULONGLONG,
    GlobalValidationRunlevel: ULONG,
    TimeZoneBiasStamp: LONG,
    NtBuildNumber: ULONG,
    NtProductType: NT_PRODUCT_TYPE,
    ProductTypeIsValid: BOOLEAN,
    Reserved0: [UCHAR; 1],
    NativeProcessorArchitecture: USHORT,
    NtMajorVersion: ULONG,
    NtMinorVersion: ULONG,
    ProcessorFeatures: [BOOLEAN; PROCESSOR_FEATURE_MAX],
    Reserved1: ULONG,
    Reserved3: ULONG,
    TimeSlip: ULONG,
    AlternativeArchitecture: ALTERNATIVE_ARCHITECTURE_TYPE,
    BootId: ULONG,
    SystemExpirationDate: LARGE_INTEGER,
    SuiteMask: ULONG,
    KdDebuggerEnabled: BOOLEAN,
    MitigationPolicies: UCHAR,
    Reserved6: [UCHAR; 2],
    ActiveConsoleId: ULONG,
    DismountCount: ULONG,
    ComPlusPackage: ULONG,
    LastSystemRITEventTickCount: ULONG,
    NumberOfPhysicalPages: ULONG,
    SafeBootMode: BOOLEAN,
    VirtualizationFlags: UCHAR,
    Reserved12: [UCHAR; 2],
    SharedDataFlags: ULONG,
    DataFlagsPad: [ULONG; 1],
    TestRetInstruction: ULONGLONG,
    QpcFrequency: LONGLONG,
    SystemCall: ULONG,
    SystemCallPad0: ULONG,
    SystemCallPad: [ULONGLONG; 2],
    u: KUSER_SHARED_DATA_u,
    //TickCountPad: [ULONG; 1],
    Cookie: ULONG,
    CookiePad: [ULONG; 1],
    ConsoleSessionForegroundProcessId: LONGLONG,
    TimeUpdateLock: ULONGLONG,
    BaselineSystemTimeQpc: ULONGLONG,
    BaselineInterruptTimeQpc: ULONGLONG,
    QpcSystemTimeIncrement: ULONGLONG,
    QpcInterruptTimeIncrement: ULONGLONG,
    QpcSystemTimeIncrementShift: UCHAR,
    QpcInterruptTimeIncrementShift: UCHAR,
    UnparkedProcessorCount: USHORT,
    EnclaveFeatureMask: [ULONG; 4],
    TelemetryCoverageRound: ULONG,
    UserModeGlobalLogger: [USHORT; 16],
    ImageFileExecutionOptions: ULONG,
    LangGenerationCount: ULONG,
    Reserved4: ULONGLONG,
    InterruptTimeBias: ULONG64,
    QpcBias: ULONG64,
    ActiveProcessorCount: ULONG,
    ActiveGroupCount: UCHAR,
    Reserved9: UCHAR,
    QpcData: UCHAR,
    TimeZoneBiasEffectiveStart: LARGE_INTEGER,
    TimeZoneBiasEffectiveEnd: LARGE_INTEGER,
    XState: XSTATE_CONFIGURATION,
}}
BITFIELD!{KUSER_SHARED_DATA MitigationPolicies: UCHAR [
    NXSupportPolicy set_NXSupportPolicy[0..2],
    SEHValidationPolicy set_SEHValidationPolicy[2..4],
    CurDirDevicesSkippedForDlls set_CurDirDevicesSkippedForDlls[4..6],
    Reserved set_Reserved[6..8],
]}
BITFIELD!{KUSER_SHARED_DATA SharedDataFlags: ULONG [
    DbgErrorPortPresent set_DbgErrorPortPresent[0..1],
    DbgElevationEnabled set_DbgElevationEnabled[1..2],
    DbgVirtEnabled set_DbgVirtEnabled[2..3],
    DbgInstallerDetectEnabled set_DbgInstallerDetectEnabled[3..4],
    DbgLkgEnabled set_DbgLkgEnabled[4..5],
    DbgDynProcessorEnabled set_DbgDynProcessorEnabled[5..6],
    DbgConsoleBrokerEnabled set_DbgConsoleBrokerEnabled[6..7],
    DbgSecureBootEnabled set_DbgSecureBootEnabled[7..8],
    DbgMultiSessionSku set_DbgMultiSessionSku[8..9],
    DbgMultiUsersInSessionSku set_DbgMultiUsersInSessionSku[9..10],
    DbgStateSeparationEnabled set_DbgStateSeparationEnabled[10..11],
    SpareBits set_SpareBits[11..32],
]}
BITFIELD!{KUSER_SHARED_DATA QpcData: UCHAR [
    QpcBypassEnabled set_QpcBypassEnabled[0..1],
    QpcShift set_QpcShift[1..2],
]}
pub type PKUSER_SHARED_DATA = *mut KUSER_SHARED_DATA;
pub const USER_SHARED_DATA: *const KUSER_SHARED_DATA = 0x7ffe0000 as *const _;
#[inline]
pub unsafe fn NtGetTickCount64() -> ULONGLONG {
    let mut tick_count: ULARGE_INTEGER = MaybeUninit::zeroed().assume_init();
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
        *tick_count.QuadPart_mut() = read_volatile(addr_of!((*USER_SHARED_DATA).u.TickCountQuad));
    }
    #[cfg(target_arch = "x86")] {
        loop {
            tick_count.s_mut().HighPart =
                read_volatile(&(*USER_SHARED_DATA).u.TickCount.High1Time) as u32;
            tick_count.s_mut().LowPart = read_volatile(&(*USER_SHARED_DATA).u.TickCount.LowPart);
            if tick_count.s().HighPart == read_volatile(&(*USER_SHARED_DATA).u.TickCount.High2Time)
                as u32
            {
                break;
            }
            spin_loop();
        }
    }
    (UInt32x32To64(tick_count.s().LowPart, (*USER_SHARED_DATA).TickCountMultiplier) >> 24)
        + (UInt32x32To64(
        tick_count.s().HighPart as u32,
        (*USER_SHARED_DATA).TickCountMultiplier,
    ) << 8)
}
#[inline]
pub unsafe fn NtGetTickCount() -> ULONG {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
        ((read_volatile(addr_of!((*USER_SHARED_DATA).u.TickCountQuad))
            * (*USER_SHARED_DATA).TickCountMultiplier as u64) >> 24) as u32
    }
    #[cfg(target_arch = "x86")] {
        let mut tick_count: ULARGE_INTEGER = MaybeUninit::zeroed().assume_init();
        loop {
            tick_count.s_mut().HighPart = read_volatile(&(*USER_SHARED_DATA).u.TickCount.High1Time)
                as u32;
            tick_count.s_mut().LowPart = read_volatile(&(*USER_SHARED_DATA).u.TickCount.LowPart);
            if tick_count.s().HighPart == read_volatile(&(*USER_SHARED_DATA).u.TickCount.High2Time)
                as u32
            {
                break;
            }
            spin_loop();
        }
        ((UInt32x32To64(tick_count.s().LowPart, (*USER_SHARED_DATA).TickCountMultiplier) >> 24)
            + UInt32x32To64(
            (tick_count.s().HighPart as u32) << 8,
            (*USER_SHARED_DATA).TickCountMultiplier,
        )) as u32
    }
}
EXTERN!{extern "system" {
    fn NtQueryDefaultLocale(
        UserProfile: BOOLEAN,
        DefaultLocaleId: PLCID,
    ) -> NTSTATUS;
    fn NtSetDefaultLocale(
        UserProfile: BOOLEAN,
        DefaultLocaleId: LCID,
    ) -> NTSTATUS;
    fn NtQueryInstallUILanguage(
        InstallUILanguageId: *mut LANGID,
    ) -> NTSTATUS;
    fn NtFlushInstallUILanguage(
        InstallUILanguage: LANGID,
        SetComittedFlag: ULONG,
    ) -> NTSTATUS;
    fn NtQueryDefaultUILanguage(
        DefaultUILanguageId: *mut LANGID,
    ) -> NTSTATUS;
    fn NtSetDefaultUILanguage(
        DefaultUILanguageId: LANGID,
    ) -> NTSTATUS;
    fn NtIsUILanguageComitted() -> NTSTATUS;
    fn NtInitializeNlsFiles(
        BaseAddress: *mut PVOID,
        DefaultLocaleId: PLCID,
        DefaultCasingTableSize: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtGetNlsSectionPtr(
        SectionType: ULONG,
        SectionData: ULONG,
        ContextData: PVOID,
        SectionPointer: *mut PVOID,
        SectionSize: PULONG,
    ) -> NTSTATUS;
    fn NtMapCMFModule(
        What: ULONG,
        Index: ULONG,
        CacheIndexOut: PULONG,
        CacheFlagsOut: PULONG,
        ViewSizeOut: PULONG,
        BaseAddress: *mut PVOID,
    ) -> NTSTATUS;
    fn NtGetMUIRegistryInfo(
        Flags: ULONG,
        DataSize: PULONG,
        Data: PVOID,
    ) -> NTSTATUS;
    fn NtAddAtom(
        AtomName: PWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
}}
pub const ATOM_FLAG_GLOBAL: ULONG = 0x2;
EXTERN!{extern "system" {
    fn NtAddAtomEx(
        AtomName: PWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtFindAtom(
        AtomName: PWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn NtDeleteAtom(
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
}}
ENUM!{enum ATOM_INFORMATION_CLASS {
    AtomBasicInformation = 0,
    AtomTableInformation = 1,
}}
STRUCT!{struct ATOM_BASIC_INFORMATION {
    UsageCount: USHORT,
    Flags: USHORT,
    NameLength: USHORT,
    Name: [WCHAR; 1],
}}
pub type PATOM_BASIC_INFORMATION = *mut ATOM_BASIC_INFORMATION;
STRUCT!{struct ATOM_TABLE_INFORMATION {
    NumberOfAtoms: ULONG,
    Atoms: [RTL_ATOM; 1],
}}
pub type PATOM_TABLE_INFORMATION = *mut ATOM_TABLE_INFORMATION;
EXTERN!{extern "system" {
    fn NtQueryInformationAtom(
        Atom: RTL_ATOM,
        AtomInformationClass: ATOM_INFORMATION_CLASS,
        AtomInformation: PVOID,
        AtomInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
pub const FLG_STOP_ON_EXCEPTION: u32 = 0x00000001;
pub const FLG_SHOW_LDR_SNAPS: u32 = 0x00000002;
pub const FLG_DEBUG_INITIAL_COMMAND: u32 = 0x00000004;
pub const FLG_STOP_ON_HUNG_GUI: u32 = 0x00000008;
pub const FLG_HEAP_ENABLE_TAIL_CHECK: u32 = 0x00000010;
pub const FLG_HEAP_ENABLE_FREE_CHECK: u32 = 0x00000020;
pub const FLG_HEAP_VALIDATE_PARAMETERS: u32 = 0x00000040;
pub const FLG_HEAP_VALIDATE_ALL: u32 = 0x00000080;
pub const FLG_APPLICATION_VERIFIER: u32 = 0x00000100;
pub const FLG_POOL_ENABLE_TAGGING: u32 = 0x00000400;
pub const FLG_HEAP_ENABLE_TAGGING: u32 = 0x00000800;
pub const FLG_USER_STACK_TRACE_DB: u32 = 0x00001000;
pub const FLG_KERNEL_STACK_TRACE_DB: u32 = 0x00002000;
pub const FLG_MAINTAIN_OBJECT_TYPELIST: u32 = 0x00004000;
pub const FLG_HEAP_ENABLE_TAG_BY_DLL: u32 = 0x00008000;
pub const FLG_DISABLE_STACK_EXTENSION: u32 = 0x00010000;
pub const FLG_ENABLE_CSRDEBUG: u32 = 0x00020000;
pub const FLG_ENABLE_KDEBUG_SYMBOL_LOAD: u32 = 0x00040000;
pub const FLG_DISABLE_PAGE_KERNEL_STACKS: u32 = 0x00080000;
pub const FLG_ENABLE_SYSTEM_CRIT_BREAKS: u32 = 0x00100000;
pub const FLG_HEAP_DISABLE_COALESCING: u32 = 0x00200000;
pub const FLG_ENABLE_CLOSE_EXCEPTIONS: u32 = 0x00400000;
pub const FLG_ENABLE_EXCEPTION_LOGGING: u32 = 0x00800000;
pub const FLG_ENABLE_HANDLE_TYPE_TAGGING: u32 = 0x01000000;
pub const FLG_HEAP_PAGE_ALLOCS: u32 = 0x02000000;
pub const FLG_DEBUG_INITIAL_COMMAND_EX: u32 = 0x04000000;
pub const FLG_DISABLE_DBGPRINT: u32 = 0x08000000;
pub const FLG_CRITSEC_EVENT_CREATION: u32 = 0x10000000;
pub const FLG_LDR_TOP_DOWN: u32 = 0x20000000;
pub const FLG_ENABLE_HANDLE_EXCEPTIONS: u32 = 0x40000000;
pub const FLG_DISABLE_PROTDLLS: u32 = 0x80000000;
pub const FLG_VALID_BITS: u32 = 0xfffffdff;
pub const FLG_USERMODE_VALID_BITS: u32 = FLG_STOP_ON_EXCEPTION | FLG_SHOW_LDR_SNAPS
    | FLG_HEAP_ENABLE_TAIL_CHECK | FLG_HEAP_ENABLE_FREE_CHECK | FLG_HEAP_VALIDATE_PARAMETERS
    | FLG_HEAP_VALIDATE_ALL | FLG_APPLICATION_VERIFIER | FLG_HEAP_ENABLE_TAGGING
    | FLG_USER_STACK_TRACE_DB | FLG_HEAP_ENABLE_TAG_BY_DLL | FLG_DISABLE_STACK_EXTENSION
    | FLG_ENABLE_SYSTEM_CRIT_BREAKS | FLG_HEAP_DISABLE_COALESCING | FLG_DISABLE_PROTDLLS
    | FLG_HEAP_PAGE_ALLOCS | FLG_CRITSEC_EVENT_CREATION | FLG_LDR_TOP_DOWN;
pub const FLG_BOOTONLY_VALID_BITS: u32 = FLG_KERNEL_STACK_TRACE_DB | FLG_MAINTAIN_OBJECT_TYPELIST
    | FLG_ENABLE_CSRDEBUG | FLG_DEBUG_INITIAL_COMMAND | FLG_DEBUG_INITIAL_COMMAND_EX
    | FLG_DISABLE_PAGE_KERNEL_STACKS;
pub const FLG_KERNELMODE_VALID_BITS: u32 = FLG_STOP_ON_EXCEPTION | FLG_SHOW_LDR_SNAPS
    | FLG_STOP_ON_HUNG_GUI | FLG_POOL_ENABLE_TAGGING | FLG_ENABLE_KDEBUG_SYMBOL_LOAD
    | FLG_ENABLE_CLOSE_EXCEPTIONS | FLG_ENABLE_EXCEPTION_LOGGING | FLG_ENABLE_HANDLE_TYPE_TAGGING
    | FLG_DISABLE_DBGPRINT | FLG_ENABLE_HANDLE_EXCEPTIONS;
EXTERN!{extern "system" {
    fn NtQueryLicenseValue(
        ValueName: PUNICODE_STRING,
        Type: PULONG,
        Data: PVOID,
        DataSize: ULONG,
        ResultDataSize: PULONG,
    ) -> NTSTATUS;
    fn NtSetDefaultHardErrorPort(
        DefaultHardErrorPort: HANDLE,
    ) -> NTSTATUS;
}}
ENUM!{enum SHUTDOWN_ACTION {
    ShutdownNoReboot = 0,
    ShutdownReboot = 1,
    ShutdownPowerOff = 2,
}}
EXTERN!{extern "system" {
    fn NtShutdownSystem(
        Action: SHUTDOWN_ACTION,
    ) -> NTSTATUS;
    fn NtDisplayString(
        String: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn NtDrawText(
        Text: PUNICODE_STRING,
    ) -> NTSTATUS;
}}
