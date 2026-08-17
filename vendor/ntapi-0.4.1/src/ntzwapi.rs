use crate::ntapi_base::{PCLIENT_ID, PRTL_ATOM, RTL_ATOM};
use crate::ntdbg::DEBUGOBJECTINFOCLASS;
use crate::ntexapi::{
    ATOM_INFORMATION_CLASS, EVENT_INFORMATION_CLASS, MUTANT_INFORMATION_CLASS, PBOOT_ENTRY,
    PBOOT_OPTIONS, PCWNF_TYPE_ID, PEFI_DRIVER_ENTRY, PFILE_PATH, PT2_CANCEL_PARAMETERS,
    PT2_SET_PARAMETERS, PTIMER_APC_ROUTINE, PWNF_CHANGE_STAMP, PWNF_DELIVERY_DESCRIPTOR,
    SEMAPHORE_INFORMATION_CLASS, SHUTDOWN_ACTION, SYSDBG_COMMAND, SYSTEM_INFORMATION_CLASS,
    TIMER_INFORMATION_CLASS, TIMER_SET_INFORMATION_CLASS, WNF_CHANGE_STAMP, WNF_DATA_SCOPE,
    WNF_STATE_NAME_INFORMATION, WNF_STATE_NAME_LIFETIME, WORKERFACTORYINFOCLASS,
};
use crate::ntioapi::{
    FILE_INFORMATION_CLASS, FILE_IO_COMPLETION_INFORMATION, FS_INFORMATION_CLASS,
    IO_COMPLETION_INFORMATION_CLASS, IO_SESSION_EVENT, IO_SESSION_STATE, PFILE_BASIC_INFORMATION,
    PFILE_IO_COMPLETION_INFORMATION, PFILE_NETWORK_OPEN_INFORMATION, PIO_APC_ROUTINE,
    PIO_STATUS_BLOCK,
};
use crate::ntkeapi::KPROFILE_SOURCE;
use crate::ntlpcapi::{
    ALPC_HANDLE, ALPC_MESSAGE_INFORMATION_CLASS, ALPC_PORT_INFORMATION_CLASS, PALPC_CONTEXT_ATTR,
    PALPC_DATA_VIEW_ATTR, PALPC_HANDLE, PALPC_MESSAGE_ATTRIBUTES, PALPC_PORT_ATTRIBUTES,
    PALPC_SECURITY_ATTR, PORT_INFORMATION_CLASS, PPORT_MESSAGE, PPORT_VIEW, PREMOTE_PORT_VIEW,
};
use crate::ntmisc::VDMSERVICECLASS;
use crate::ntmmapi::{
    MEMORY_INFORMATION_CLASS, MEMORY_PARTITION_INFORMATION_CLASS, PMEMORY_RANGE_ENTRY,
    SECTION_INFORMATION_CLASS, SECTION_INHERIT, VIRTUAL_MEMORY_INFORMATION_CLASS,
};
use crate::ntobapi::OBJECT_INFORMATION_CLASS;
use crate::ntpnpapi::{PLUGPLAY_CONTROL_CLASS, PPLUGPLAY_EVENT_BLOCK};
use crate::ntpsapi::{
    MEMORY_RESERVE_TYPE, PINITIAL_TEB, PPS_APC_ROUTINE, PPS_ATTRIBUTE_LIST, PPS_CREATE_INFO,
    PROCESSINFOCLASS, THREADINFOCLASS,
};
use crate::ntregapi::{
    KEY_INFORMATION_CLASS, KEY_SET_INFORMATION_CLASS, KEY_VALUE_INFORMATION_CLASS,
    PKEY_VALUE_ENTRY,
};
use crate::ntseapi::PTOKEN_SECURITY_ATTRIBUTES_INFORMATION;
use winapi::shared::basetsd::{
    KAFFINITY, PSIZE_T, PULONG64, PULONG_PTR, SIZE_T, ULONG64, ULONG_PTR,
};
use winapi::shared::guiddef::LPGUID;
use winapi::shared::ktmtypes::{NOTIFICATION_MASK, PCRM_PROTOCOL_ID, PTRANSACTION_NOTIFICATION};
use winapi::shared::ntdef::{
    BOOLEAN, EVENT_TYPE, HANDLE, LANGID, LCID, LOGICAL, LONG, NTSTATUS, OBJECT_ATTRIBUTES,
    PBOOLEAN, PCHAR, PCWNF_STATE_NAME, PGROUP_AFFINITY, PHANDLE, PLARGE_INTEGER, PLCID, PLONG,
    PLUID, PNTSTATUS, POBJECT_ATTRIBUTES, PUCHAR, PULARGE_INTEGER, PULONG, PULONGLONG,
    PUNICODE_STRING, PUSHORT, PVOID, PWNF_STATE_NAME, PWSTR, TIMER_TYPE, ULONG, USHORT, VOID,
    WAIT_TYPE,
};
use winapi::um::winnt::{
    ACCESS_MASK, AUDIT_EVENT_TYPE, ENLISTMENT_INFORMATION_CLASS, EXECUTION_STATE,
    JOBOBJECTINFOCLASS, KTMOBJECT_TYPE, LATENCY_TIME, PACCESS_MASK, PCONTEXT, PDEVICE_POWER_STATE,
    PEXCEPTION_RECORD, PFILE_SEGMENT_ELEMENT, PGENERIC_MAPPING, PJOB_SET_ARRAY, PKTMOBJECT_CURSOR,
    POBJECT_TYPE_LIST, POWER_ACTION, POWER_INFORMATION_LEVEL, PPRIVILEGE_SET, PSECURITY_DESCRIPTOR,
    PSECURITY_QUALITY_OF_SERVICE, PSE_SIGNING_LEVEL, PSID, PSID_AND_ATTRIBUTES,
    PTOKEN_DEFAULT_DACL, PTOKEN_GROUPS, PTOKEN_MANDATORY_POLICY, PTOKEN_OWNER,
    PTOKEN_PRIMARY_GROUP, PTOKEN_PRIVILEGES, PTOKEN_SOURCE, PTOKEN_USER,
    RESOURCEMANAGER_INFORMATION_CLASS, SECURITY_INFORMATION, SE_SIGNING_LEVEL, SYSTEM_POWER_STATE,
    TOKEN_INFORMATION_CLASS, TOKEN_TYPE, TRANSACTIONMANAGER_INFORMATION_CLASS,
    TRANSACTION_INFORMATION_CLASS,
};
EXTERN!{extern "system" {
    fn ZwAcceptConnectPort(
        PortHandle: PHANDLE,
        PortContext: PVOID,
        ConnectionRequest: PPORT_MESSAGE,
        AcceptConnection: BOOLEAN,
        ServerView: PPORT_VIEW,
        ClientView: PREMOTE_PORT_VIEW,
    ) -> NTSTATUS;
    fn ZwAccessCheck(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: PULONG,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
    ) -> NTSTATUS;
    fn ZwAccessCheckAndAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PUNICODE_STRING,
        ObjectName: PUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DesiredAccess: ACCESS_MASK,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwAccessCheckByType(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: PULONG,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeAndAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PUNICODE_STRING,
        ObjectName: PUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: ULONG,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeResultList(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: PULONG,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeResultListAndAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PUNICODE_STRING,
        ObjectName: PUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: ULONG,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        ClientToken: HANDLE,
        ObjectTypeName: PUNICODE_STRING,
        ObjectName: PUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: ULONG,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOLEAN,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwAcquireCMFViewOwnership(
        TimeStamp: PULONGLONG,
        tokenTaken: PBOOLEAN,
        replaceExisting: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwAddAtom(
        AtomName: PWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn ZwAddAtomEx(
        AtomName: PWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwAddBootEntry(
        BootEntry: PBOOT_ENTRY,
        Id: PULONG,
    ) -> NTSTATUS;
    fn ZwAddDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
        Id: PULONG,
    ) -> NTSTATUS;
    fn ZwAdjustGroupsToken(
        TokenHandle: HANDLE,
        ResetToDefault: BOOLEAN,
        NewState: PTOKEN_GROUPS,
        BufferLength: ULONG,
        PreviousState: PTOKEN_GROUPS,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwAdjustPrivilegesToken(
        TokenHandle: HANDLE,
        DisableAllPrivileges: BOOLEAN,
        NewState: PTOKEN_PRIVILEGES,
        BufferLength: ULONG,
        PreviousState: PTOKEN_PRIVILEGES,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwAdjustTokenClaimsAndDeviceGroups(
        TokenHandle: HANDLE,
        UserResetToDefault: BOOLEAN,
        DeviceResetToDefault: BOOLEAN,
        DeviceGroupsResetToDefault: BOOLEAN,
        NewUserState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        NewDeviceGroupsState: PTOKEN_GROUPS,
        UserBufferLength: ULONG,
        PreviousUserState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceBufferLength: ULONG,
        PreviousDeviceState: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroupsBufferLength: ULONG,
        PreviousDeviceGroups: PTOKEN_GROUPS,
        UserReturnLength: PULONG,
        DeviceReturnLength: PULONG,
        DeviceGroupsReturnBufferLength: PULONG,
    ) -> NTSTATUS;
    fn ZwAlertResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;
    fn ZwAlertThread(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwAlertThreadByThreadId(
        ThreadId: HANDLE,
    ) -> NTSTATUS;
    fn ZwAllocateLocallyUniqueId(
        Luid: PLUID,
    ) -> NTSTATUS;
    fn ZwAllocateReserveObject(
        MemoryReserveHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Type: MEMORY_RESERVE_TYPE,
    ) -> NTSTATUS;
    fn ZwAllocateUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: PULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn ZwAllocateUuids(
        Time: PULARGE_INTEGER,
        Range: PULONG,
        Sequence: PULONG,
        Seed: PCHAR,
    ) -> NTSTATUS;
    fn ZwAllocateVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        ZeroBits: ULONG_PTR,
        RegionSize: PSIZE_T,
        AllocationType: ULONG,
        Protect: ULONG,
    ) -> NTSTATUS;
    fn ZwAlpcAcceptConnectPort(
        PortHandle: PHANDLE,
        ConnectionPortHandle: HANDLE,
        Flags: ULONG,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        PortContext: PVOID,
        ConnectionRequest: PPORT_MESSAGE,
        ConnectionMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        AcceptConnection: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: ULONG,
        MessageContext: PALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;
    fn ZwAlpcConnectPort(
        PortHandle: PHANDLE,
        PortName: PUNICODE_STRING,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        Flags: ULONG,
        RequiredServerSid: PSID,
        ConnectionMessage: PPORT_MESSAGE,
        BufferLength: PULONG,
        OutMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwAlpcConnectPortEx(
        PortHandle: PHANDLE,
        ConnectionPortObjectAttributes: POBJECT_ATTRIBUTES,
        ClientPortObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
        Flags: ULONG,
        ServerSecurityRequirements: PSECURITY_DESCRIPTOR,
        ConnectionMessage: PPORT_MESSAGE,
        BufferLength: PSIZE_T,
        OutMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        InMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwAlpcCreatePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwAlpcCreatePortSection(
        PortHandle: HANDLE,
        Flags: ULONG,
        SectionHandle: HANDLE,
        SectionSize: SIZE_T,
        AlpcSectionHandle: PALPC_HANDLE,
        ActualSectionSize: PSIZE_T,
    ) -> NTSTATUS;
    fn ZwAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: ULONG,
        MessageSize: SIZE_T,
        ResourceId: PALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: ULONG,
        ViewAttributes: PALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;
    fn ZwAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        SecurityAttribute: PALPC_SECURITY_ATTR,
    ) -> NTSTATUS;
    fn ZwAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: ULONG,
        SectionHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: ULONG,
        ResourceId: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: ULONG,
        ViewBase: PVOID,
    ) -> NTSTATUS;
    fn ZwAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcDisconnectPort(
        PortHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: PVOID,
    ) -> NTSTATUS;
    fn ZwAlpcOpenSenderProcess(
        ProcessHandle: PHANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: ULONG,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwAlpcOpenSenderThread(
        ThreadHandle: PHANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: ULONG,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwAlpcQueryInformationMessage(
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS,
        MessageInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn ZwAlpcSendWaitReceivePort(
        PortHandle: HANDLE,
        Flags: ULONG,
        SendMessageA: PPORT_MESSAGE,
        SendMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        ReceiveMessage: PPORT_MESSAGE,
        BufferLength: PSIZE_T,
        ReceiveMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;
    fn ZwAreMappedFilesTheSame(
        File1MappedAsAnImage: PVOID,
        File2MappedAsFile: PVOID,
    ) -> NTSTATUS;
    fn ZwAssignProcessToJobObject(
        JobHandle: HANDLE,
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwAssociateWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        IoCompletionHandle: HANDLE,
        TargetObjectHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
        AlreadySignaled: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwCallbackReturn(
        OutputBuffer: PVOID,
        OutputLength: ULONG,
        Status: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwCancelIoFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwCancelIoFileEx(
        FileHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwCancelSynchronousIoFile(
        ThreadHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwCancelTimer(
        TimerHandle: HANDLE,
        CurrentState: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwCancelTimer2(
        TimerHandle: HANDLE,
        Parameters: PT2_CANCEL_PARAMETERS,
    ) -> NTSTATUS;
    fn ZwCancelWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        RemoveSignaledPacket: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwClearEvent(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwClose(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCloseObjectAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwCommitComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCommitEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCommitTransaction(
        TransactionHandle: HANDLE,
        Wait: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwCompactKeys(
        Count: ULONG,
        KeyArray: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwCompareObjects(
        FirstObjectHandle: HANDLE,
        SecondObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwCompleteConnectPort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCompressKey(
        Key: HANDLE,
    ) -> NTSTATUS;
    fn ZwConnectPort(
        PortHandle: PHANDLE,
        PortName: PUNICODE_STRING,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: PULONG,
        ConnectionInformation: PVOID,
        ConnectionInformationLength: PULONG,
    ) -> NTSTATUS;
    fn ZwContinue(
        ContextRecord: PCONTEXT,
        TestAlert: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwCreateDebugObject(
        DebugObjectHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateDirectoryObject(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateDirectoryObjectEx(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateEnlistment(
        EnlistmentHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ResourceManagerHandle: HANDLE,
        TransactionHandle: HANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CreateOptions: ULONG,
        NotificationMask: NOTIFICATION_MASK,
        EnlistmentKey: PVOID,
    ) -> NTSTATUS;
    fn ZwCreateEvent(
        EventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        EventType: EVENT_TYPE,
        InitialState: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwCreateEventPair(
        EventPairHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        AllocationSize: PLARGE_INTEGER,
        FileAttributes: ULONG,
        ShareAccess: ULONG,
        CreateDisposition: ULONG,
        CreateOptions: ULONG,
        EaBuffer: PVOID,
        EaLength: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateIRTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
    ) -> NTSTATUS;
    fn ZwCreateIoCompletion(
        IoCompletionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Count: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateJobObject(
        JobHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateJobSet(
        NumJob: ULONG,
        UserJobSet: PJOB_SET_ARRAY,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateKey(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TitleIndex: ULONG,
        Class: PUNICODE_STRING,
        CreateOptions: ULONG,
        Disposition: PULONG,
    ) -> NTSTATUS;
    fn ZwCreateKeyTransacted(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TitleIndex: ULONG,
        Class: PUNICODE_STRING,
        CreateOptions: ULONG,
        TransactionHandle: HANDLE,
        Disposition: PULONG,
    ) -> NTSTATUS;
    fn ZwCreateKeyedEvent(
        KeyedEventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateLowBoxToken(
        TokenHandle: PHANDLE,
        ExistingTokenHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PackageSid: PSID,
        CapabilityCount: ULONG,
        Capabilities: PSID_AND_ATTRIBUTES,
        HandleCount: ULONG,
        Handles: *mut HANDLE,
    ) -> NTSTATUS;
    fn ZwCreateMailslotFile(
        FileHandle: PHANDLE,
        DesiredAccess: ULONG,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CreateOptions: ULONG,
        MailslotQuota: ULONG,
        MaximumMessageSize: ULONG,
        ReadTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCreateMutant(
        MutantHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        InitialOwner: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwCreateNamedPipeFile(
        FileHandle: PHANDLE,
        DesiredAccess: ULONG,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: ULONG,
        CreateDisposition: ULONG,
        CreateOptions: ULONG,
        NamedPipeType: ULONG,
        ReadMode: ULONG,
        CompletionMode: ULONG,
        MaximumInstances: ULONG,
        InboundQuota: ULONG,
        OutboundQuota: ULONG,
        DefaultTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwCreatePagingFile(
        PageFileName: PUNICODE_STRING,
        MinimumSize: PLARGE_INTEGER,
        MaximumSize: PLARGE_INTEGER,
        Priority: ULONG,
    ) -> NTSTATUS;
    fn ZwCreatePartition(
        PartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PreferredNode: ULONG,
    ) -> NTSTATUS;
    fn ZwCreatePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: ULONG,
        MaxMessageLength: ULONG,
        MaxPoolUsage: ULONG,
    ) -> NTSTATUS;
    fn ZwCreatePrivateNamespace(
        NamespaceHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        BoundaryDescriptor: PVOID,
    ) -> NTSTATUS;
    fn ZwCreateProcess(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ParentProcess: HANDLE,
        InheritObjectTable: BOOLEAN,
        SectionHandle: HANDLE,
        DebugPort: HANDLE,
        ExceptionPort: HANDLE,
    ) -> NTSTATUS;
    fn ZwCreateProcessEx(
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
    fn ZwCreateProfile(
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
    fn ZwCreateProfileEx(
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
    fn ZwCreateResourceManager(
        ResourceManagerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        TmHandle: HANDLE,
        ResourceManagerGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CreateOptions: ULONG,
        Description: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwCreateSection(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaximumSize: PLARGE_INTEGER,
        SectionPageProtection: ULONG,
        AllocationAttributes: ULONG,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwCreateSemaphore(
        SemaphoreHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        InitialCount: LONG,
        MaximumCount: LONG,
    ) -> NTSTATUS;
    fn ZwCreateSymbolicLinkObject(
        LinkHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LinkTarget: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwCreateThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ProcessHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ThreadContext: PCONTEXT,
        InitialTeb: PINITIAL_TEB,
        CreateSuspended: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwCreateThreadEx(
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
    fn ZwCreateTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TimerType: TIMER_TYPE,
    ) -> NTSTATUS;
    fn ZwCreateTimer2(
        TimerHandle: PHANDLE,
        Reserved1: PVOID,
        Reserved2: PVOID,
        Attributes: ULONG,
        DesiredAccess: ACCESS_MASK,
    ) -> NTSTATUS;
    fn ZwCreateToken(
        TokenHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TokenType: TOKEN_TYPE,
        AuthenticationId: PLUID,
        ExpirationTime: PLARGE_INTEGER,
        User: PTOKEN_USER,
        Groups: PTOKEN_GROUPS,
        Privileges: PTOKEN_PRIVILEGES,
        Owner: PTOKEN_OWNER,
        PrimaryGroup: PTOKEN_PRIMARY_GROUP,
        DefaultDacl: PTOKEN_DEFAULT_DACL,
        TokenSource: PTOKEN_SOURCE,
    ) -> NTSTATUS;
    fn ZwCreateTokenEx(
        TokenHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TokenType: TOKEN_TYPE,
        AuthenticationId: PLUID,
        ExpirationTime: PLARGE_INTEGER,
        User: PTOKEN_USER,
        Groups: PTOKEN_GROUPS,
        Privileges: PTOKEN_PRIVILEGES,
        UserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        DeviceGroups: PTOKEN_GROUPS,
        TokenMandatoryPolicy: PTOKEN_MANDATORY_POLICY,
        Owner: PTOKEN_OWNER,
        PrimaryGroup: PTOKEN_PRIMARY_GROUP,
        DefaultDacl: PTOKEN_DEFAULT_DACL,
        TokenSource: PTOKEN_SOURCE,
    ) -> NTSTATUS;
    fn ZwCreateTransaction(
        TransactionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Uow: LPGUID,
        TmHandle: HANDLE,
        CreateOptions: ULONG,
        IsolationLevel: ULONG,
        IsolationFlags: ULONG,
        Timeout: PLARGE_INTEGER,
        Description: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwCreateTransactionManager(
        TmHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LogFileName: PUNICODE_STRING,
        CreateOptions: ULONG,
        CommitStrength: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateUserProcess(
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
    fn ZwCreateWaitCompletionPacket(
        WaitCompletionPacketHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwCreateWaitablePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: ULONG,
        MaxMessageLength: ULONG,
        MaxPoolUsage: ULONG,
    ) -> NTSTATUS;
    fn ZwCreateWnfStateName(
        StateName: PWNF_STATE_NAME,
        NameLifetime: WNF_STATE_NAME_LIFETIME,
        DataScope: WNF_DATA_SCOPE,
        PersistData: BOOLEAN,
        TypeId: PCWNF_TYPE_ID,
        MaximumStateSize: ULONG,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn ZwCreateWorkerFactory(
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
    fn ZwDebugActiveProcess(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: PCLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwDelayExecution(
        Alertable: BOOLEAN,
        DelayInterval: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwDeleteAtom(
        Atom: RTL_ATOM,
    ) -> NTSTATUS;
    fn ZwDeleteBootEntry(
        Id: ULONG,
    ) -> NTSTATUS;
    fn ZwDeleteDriverEntry(
        Id: ULONG,
    ) -> NTSTATUS;
    fn ZwDeleteFile(
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwDeleteKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwDeleteObjectAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwDeletePrivateNamespace(
        NamespaceHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwDeleteValueKey(
        KeyHandle: HANDLE,
        ValueName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwDeleteWnfStateData(
        StateName: PCWNF_STATE_NAME,
        ExplicitScope: *const VOID,
    ) -> NTSTATUS;
    fn ZwDeleteWnfStateName(
        StateName: PCWNF_STATE_NAME,
    ) -> NTSTATUS;
    fn ZwDeviceIoControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        IoControlCode: ULONG,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;
    fn ZwDisableLastKnownGood() -> NTSTATUS;
    fn ZwDisplayString(
        String: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwDrawText(
        String: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Options: ULONG,
    ) -> NTSTATUS;
    fn ZwDuplicateToken(
        ExistingTokenHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        EffectiveOnly: BOOLEAN,
        TokenType: TOKEN_TYPE,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwEnableLastKnownGood() -> NTSTATUS;
    fn ZwEnumerateBootEntries(
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;
    fn ZwEnumerateDriverEntries(
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;
    fn ZwEnumerateKey(
        KeyHandle: HANDLE,
        Index: ULONG,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn ZwEnumerateSystemEnvironmentValuesEx(
        InformationClass: ULONG,
        Buffer: PVOID,
        BufferLength: PULONG,
    ) -> NTSTATUS;
    fn ZwEnumerateTransactionObject(
        RootObjectHandle: HANDLE,
        QueryType: KTMOBJECT_TYPE,
        ObjectCursor: PKTMOBJECT_CURSOR,
        ObjectCursorLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: ULONG,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn ZwExtendSection(
        SectionHandle: HANDLE,
        NewSectionSize: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwFilterToken(
        ExistingTokenHandle: HANDLE,
        Flags: ULONG,
        SidsToDisable: PTOKEN_GROUPS,
        PrivilegesToDelete: PTOKEN_PRIVILEGES,
        RestrictedSids: PTOKEN_GROUPS,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwFilterTokenEx(
        ExistingTokenHandle: HANDLE,
        Flags: ULONG,
        SidsToDisable: PTOKEN_GROUPS,
        PrivilegesToDelete: PTOKEN_PRIVILEGES,
        RestrictedSids: PTOKEN_GROUPS,
        DisableUserClaimsCount: ULONG,
        UserClaimsToDisable: PUNICODE_STRING,
        DisableDeviceClaimsCount: ULONG,
        DeviceClaimsToDisable: PUNICODE_STRING,
        DeviceGroupsToDisable: PTOKEN_GROUPS,
        RestrictedUserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
        RestrictedDeviceGroups: PTOKEN_GROUPS,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwFindAtom(
        AtomName: PWSTR,
        Length: ULONG,
        Atom: PRTL_ATOM,
    ) -> NTSTATUS;
    fn ZwFlushBuffersFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwFlushBuffersFileEx(
        FileHandle: HANDLE,
        Flags: ULONG,
        Parameters: PVOID,
        ParametersSize: ULONG,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwFlushInstallUILanguage(
        InstallUILanguage: LANGID,
        SetComittedFlag: ULONG,
    ) -> NTSTATUS;
    fn ZwFlushInstructionCache(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Length: SIZE_T,
    ) -> NTSTATUS;
    fn ZwFlushKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwFlushProcessWriteBuffers();
    fn ZwFlushWriteBuffer() -> NTSTATUS;
    fn ZwFreeUserPhysicalPages(
        ProcessHandle: HANDLE,
        NumberOfPages: PULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn ZwFreeVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        FreeType: ULONG,
    ) -> NTSTATUS;
    fn ZwFreezeRegistry(
        TimeOutInSeconds: ULONG,
    ) -> NTSTATUS;
    fn ZwFreezeTransactions(
        FreezeTimeout: PLARGE_INTEGER,
        ThawTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwFsControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsControlCode: ULONG,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;
    fn ZwGetCachedSigningLevel(
        File: HANDLE,
        Flags: PULONG,
        SigningLevel: PSE_SIGNING_LEVEL,
        Thumbprint: PUCHAR,
        ThumbprintSize: PULONG,
        ThumbprintAlgorithm: PULONG,
    ) -> NTSTATUS;
    fn ZwGetCompleteWnfStateSubscription(
        OldDescriptorStateName: PWNF_STATE_NAME,
        OldSubscriptionId: *mut ULONG64,
        OldDescriptorEventMask: ULONG,
        OldDescriptorStatus: ULONG,
        NewDeliveryDescriptor: PWNF_DELIVERY_DESCRIPTOR,
        DescriptorSize: ULONG,
    ) -> NTSTATUS;
    fn ZwGetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;
    fn ZwGetCurrentProcessorNumber() -> ULONG;
    fn ZwGetDevicePowerState(
        Device: HANDLE,
        State: PDEVICE_POWER_STATE,
    ) -> NTSTATUS;
    fn ZwGetMUIRegistryInfo(
        Flags: ULONG,
        DataSize: PULONG,
        Data: PVOID,
    ) -> NTSTATUS;
    fn ZwGetNextProcess(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewProcessHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwGetNextThread(
        ProcessHandle: HANDLE,
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Flags: ULONG,
        NewThreadHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwGetNlsSectionPtr(
        SectionType: ULONG,
        SectionData: ULONG,
        ContextData: PVOID,
        SectionPointer: *mut PVOID,
        SectionSize: PULONG,
    ) -> NTSTATUS;
    fn ZwGetNotificationResourceManager(
        ResourceManagerHandle: HANDLE,
        TransactionNotification: PTRANSACTION_NOTIFICATION,
        NotificationLength: ULONG,
        Timeout: PLARGE_INTEGER,
        ReturnLength: PULONG,
        Asynchronous: ULONG,
        AsynchronousContext: ULONG_PTR,
    ) -> NTSTATUS;
    fn ZwGetPlugPlayEvent(
        EventHandle: HANDLE,
        Context: PVOID,
        EventBlock: PPLUGPLAY_EVENT_BLOCK,
        EventBufferSize: ULONG,
    ) -> NTSTATUS;
    fn ZwGetWriteWatch(
        ProcessHandle: HANDLE,
        Flags: ULONG,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
        UserAddressArray: *mut PVOID,
        EntriesInUserAddressArray: PULONG_PTR,
        Granularity: PULONG,
    ) -> NTSTATUS;
    fn ZwImpersonateAnonymousToken(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwImpersonateThread(
        ServerThreadHandle: HANDLE,
        ClientThreadHandle: HANDLE,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
    ) -> NTSTATUS;
    fn ZwInitializeNlsFiles(
        BaseAddress: *mut PVOID,
        DefaultLocaleId: PLCID,
        DefaultCasingTableSize: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwInitializeRegistry(
        BootCondition: USHORT,
    ) -> NTSTATUS;
    fn ZwInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: ULONG,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwIsProcessInJob(
        ProcessHandle: HANDLE,
        JobHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwIsSystemResumeAutomatic() -> BOOLEAN;
    fn ZwIsUILanguageComitted() -> NTSTATUS;
    fn ZwListenPort(
        PortHandle: HANDLE,
        ConnectionRequest: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwLoadDriver(
        DriverServiceName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwLoadKey(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwLoadKey2(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwLoadKeyEx(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
        Flags: ULONG,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: ACCESS_MASK,
        RootHandle: PHANDLE,
        IoStatus: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn ZwLockFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: PLARGE_INTEGER,
        Length: PLARGE_INTEGER,
        Key: ULONG,
        FailImmediately: BOOLEAN,
        ExclusiveLock: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwLockProductActivationKeys(
        pPrivateVer: *mut ULONG,
        pSafeMode: *mut ULONG,
    ) -> NTSTATUS;
    fn ZwLockRegistryKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;
    fn ZwMakePermanentObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn ZwMakeTemporaryObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn ZwManagePartition(
        PartitionInformationClass: MEMORY_PARTITION_INFORMATION_CLASS,
        PartitionInformation: PVOID,
        PartitionInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwMapCMFModule(
        What: ULONG,
        Index: ULONG,
        CacheIndexOut: PULONG,
        CacheFlagsOut: PULONG,
        ViewSizeOut: PULONG,
        BaseAddress: *mut PVOID,
    ) -> NTSTATUS;
    fn ZwMapUserPhysicalPages(
        VirtualAddress: PVOID,
        NumberOfPages: ULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn ZwMapUserPhysicalPagesScatter(
        VirtualAddresses: *mut PVOID,
        NumberOfPages: ULONG_PTR,
        UserPfnArray: PULONG_PTR,
    ) -> NTSTATUS;
    fn ZwMapViewOfSection(
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
    fn ZwModifyBootEntry(
        BootEntry: PBOOT_ENTRY,
    ) -> NTSTATUS;
    fn ZwModifyDriverEntry(
        DriverEntry: PEFI_DRIVER_ENTRY,
    ) -> NTSTATUS;
    fn ZwNotifyChangeDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwNotifyChangeKey(
        KeyHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
        Buffer: PVOID,
        BufferSize: ULONG,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwNotifyChangeMultipleKeys(
        MasterKeyHandle: HANDLE,
        Count: ULONG,
        SubordinateObjects: *mut OBJECT_ATTRIBUTES,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
        Buffer: PVOID,
        BufferSize: ULONG,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwNotifyChangeSession(
        SessionHandle: HANDLE,
        ChangeSequenceNumber: ULONG,
        ChangeTimeStamp: PLARGE_INTEGER,
        Event: IO_SESSION_EVENT,
        NewState: IO_SESSION_STATE,
        PreviousState: IO_SESSION_STATE,
        Payload: PVOID,
        PayloadSize: ULONG,
    ) -> NTSTATUS;
    fn ZwOpenDirectoryObject(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenEnlistment(
        EnlistmentHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        RmHandle: HANDLE,
        EnlistmentGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenEvent(
        EventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenEventPair(
        EventPairHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: ULONG,
        OpenOptions: ULONG,
    ) -> NTSTATUS;
    fn ZwOpenIoCompletion(
        IoCompletionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenJobObject(
        JobHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenKey(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenKeyEx(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        OpenOptions: ULONG,
    ) -> NTSTATUS;
    fn ZwOpenKeyTransacted(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenKeyTransactedEx(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        OpenOptions: ULONG,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenKeyedEvent(
        KeyedEventHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenMutant(
        MutantHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenObjectAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        ObjectTypeName: PUNICODE_STRING,
        ObjectName: PUNICODE_STRING,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        GrantedAccess: ACCESS_MASK,
        Privileges: PPRIVILEGE_SET,
        ObjectCreation: BOOLEAN,
        AccessGranted: BOOLEAN,
        GenerateOnClose: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwOpenPartition(
        PartitionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenPrivateNamespace(
        NamespaceHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        BoundaryDescriptor: PVOID,
    ) -> NTSTATUS;
    fn ZwOpenProcess(
        ProcessHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn ZwOpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwOpenProcessTokenEx(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwOpenResourceManager(
        ResourceManagerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        TmHandle: HANDLE,
        ResourceManagerGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSection(
        SectionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSemaphore(
        SemaphoreHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSession(
        SessionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenSymbolicLinkObject(
        LinkHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenThread(
        ThreadHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
    fn ZwOpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        OpenAsSelf: BOOLEAN,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwOpenThreadTokenEx(
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        OpenAsSelf: BOOLEAN,
        HandleAttributes: ULONG,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn ZwOpenTimer(
        TimerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwOpenTransaction(
        TransactionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Uow: LPGUID,
        TmHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwOpenTransactionManager(
        TmHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LogFileName: PUNICODE_STRING,
        TmIdentity: LPGUID,
        OpenOptions: ULONG,
    ) -> NTSTATUS;
    fn ZwPlugPlayControl(
        PnPControlClass: PLUGPLAY_CONTROL_CLASS,
        PnPControlData: PVOID,
        PnPControlDataLength: ULONG,
    ) -> NTSTATUS;
    fn ZwPowerInformation(
        InformationLevel: POWER_INFORMATION_LEVEL,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;
    fn ZwPrePrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrePrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwPrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: PPRIVILEGE_SET,
        Result: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwPrivilegeObjectAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwPrivilegedServiceAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        ServiceName: PUNICODE_STRING,
        ClientToken: HANDLE,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: ULONG,
        BufferLength: ULONG,
        Buffer: PVOID,
    ) -> NTSTATUS;
    fn ZwPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: ULONG,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        NewProtect: ULONG,
        OldProtect: PULONG,
    ) -> NTSTATUS;
    fn ZwPulseEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;
    fn ZwQueryAttributesFile(
        ObjectAttributes: POBJECT_ATTRIBUTES,
        FileInformation: PFILE_BASIC_INFORMATION,
    ) -> NTSTATUS;
    fn ZwQueryBootEntryOrder(
        Ids: PULONG,
        Count: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryBootOptions(
        BootOptions: PBOOT_OPTIONS,
        BootOptionsLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
    ) -> NTSTATUS;
    fn ZwQueryDefaultLocale(
        UserProfile: BOOLEAN,
        DefaultLocaleId: PLCID,
    ) -> NTSTATUS;
    fn ZwQueryDefaultUILanguage(
        DefaultUILanguageId: *mut LANGID,
    ) -> NTSTATUS;
    fn ZwQueryDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
        ReturnSingleEntry: BOOLEAN,
        FileName: PUNICODE_STRING,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        RestartScan: BOOLEAN,
        Context: PULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryDriverEntryOrder(
        Ids: PULONG,
        Count: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        EaList: PVOID,
        EaListLength: ULONG,
        EaIndex: PULONG,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwQueryEvent(
        EventHandle: HANDLE,
        EventInformationClass: EVENT_INFORMATION_CLASS,
        EventInformation: PVOID,
        EventInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryFullAttributesFile(
        ObjectAttributes: POBJECT_ATTRIBUTES,
        FileInformation: PFILE_NETWORK_OPEN_INFORMATION,
    ) -> NTSTATUS;
    fn ZwQueryInformationAtom(
        Atom: RTL_ATOM,
        AtomInformationClass: ATOM_INFORMATION_CLASS,
        AtomInformation: PVOID,
        AtomInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: PVOID,
        EnlistmentInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwQueryInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: PVOID,
        JobObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: PVOID,
        ResourceManagerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: PVOID,
        TokenInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: PVOID,
        TransactionInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationTransactionManager(
        TransactionManagerHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: PVOID,
        TransactionManagerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: PVOID,
        WorkerFactoryInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryInstallUILanguage(
        InstallUILanguageId: *mut LANGID,
    ) -> NTSTATUS;
    fn ZwQueryIntervalProfile(
        ProfileSource: KPROFILE_SOURCE,
        Interval: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryIoCompletion(
        IoCompletionHandle: HANDLE,
        IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS,
        IoCompletionInformation: PVOID,
        IoCompletionInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryLicenseValue(
        ValueName: PUNICODE_STRING,
        Type: PULONG,
        Data: PVOID,
        DataSize: ULONG,
        ResultDataSize: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: PKEY_VALUE_ENTRY,
        EntryCount: ULONG,
        ValueBuffer: PVOID,
        BufferLength: PULONG,
        RequiredBufferLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryMutant(
        MutantHandle: HANDLE,
        MutantInformationClass: MUTANT_INFORMATION_CLASS,
        MutantInformation: PVOID,
        MutantInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: PVOID,
        ObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryOpenSubKeys(
        TargetKey: POBJECT_ATTRIBUTES,
        HandleCount: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryOpenSubKeysEx(
        TargetKey: POBJECT_ATTRIBUTES,
        BufferLength: ULONG,
        Buffer: PVOID,
        RequiredSize: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryPerformanceCounter(
        PerformanceCounter: PLARGE_INTEGER,
        PerformanceFrequency: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwQueryPortInformationProcess() -> NTSTATUS;
    fn ZwQueryQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        SidList: PVOID,
        SidListLength: ULONG,
        StartSid: PSID,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwQuerySection(
        SectionHandle: HANDLE,
        SectionInformationClass: SECTION_INFORMATION_CLASS,
        SectionInformation: PVOID,
        SectionInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
    fn ZwQuerySecurityAttributesToken(
        TokenHandle: HANDLE,
        Attributes: PUNICODE_STRING,
        NumberOfAttributes: ULONG,
        Buffer: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQuerySecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Length: ULONG,
        LengthNeeded: PULONG,
    ) -> NTSTATUS;
    fn ZwQuerySemaphore(
        SemaphoreHandle: HANDLE,
        SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS,
        SemaphoreInformation: PVOID,
        SemaphoreInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: PUNICODE_STRING,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQuerySystemEnvironmentValue(
        VariableName: PUNICODE_STRING,
        VariableValue: PWSTR,
        ValueLength: USHORT,
        ReturnLength: PUSHORT,
    ) -> NTSTATUS;
    fn ZwQuerySystemEnvironmentValueEx(
        VariableName: PUNICODE_STRING,
        VendorGuid: LPGUID,
        Value: PVOID,
        ValueLength: PULONG,
        Attributes: PULONG,
    ) -> NTSTATUS;
    fn ZwQuerySystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQuerySystemInformationEx(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQuerySystemTime(
        SystemTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwQueryTimer(
        TimerHandle: HANDLE,
        TimerInformationClass: TIMER_INFORMATION_CLASS,
        TimerInformation: PVOID,
        TimerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryTimerResolution(
        MaximumTime: PULONG,
        MinimumTime: PULONG,
        CurrentTime: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: PUNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        MemoryInformationClass: MEMORY_INFORMATION_CLASS,
        MemoryInformation: PVOID,
        MemoryInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> NTSTATUS;
    fn ZwQueryVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: PVOID,
        Length: ULONG,
        FsInformationClass: FS_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwQueryWnfStateData(
        StateName: PCWNF_STATE_NAME,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const VOID,
        ChangeStamp: PWNF_CHANGE_STAMP,
        Buffer: PVOID,
        BufferSize: PULONG,
    ) -> NTSTATUS;
    fn ZwQueryWnfStateNameInformation(
        StateName: PCWNF_STATE_NAME,
        NameInfoClass: WNF_STATE_NAME_INFORMATION,
        ExplicitScope: *const VOID,
        InfoBuffer: PVOID,
        InfoBufferSize: ULONG,
    ) -> NTSTATUS;
    fn ZwQueueApcThread(
        ThreadHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;
    fn ZwQueueApcThreadEx(
        ThreadHandle: HANDLE,
        UserApcReserveHandle: HANDLE,
        ApcRoutine: PPS_APC_ROUTINE,
        ApcArgument1: PVOID,
        ApcArgument2: PVOID,
        ApcArgument3: PVOID,
    ) -> NTSTATUS;
    fn ZwRaiseException(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
        FirstChance: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwRaiseHardError(
        ErrorStatus: NTSTATUS,
        NumberOfParameters: ULONG,
        UnicodeStringParameterMask: ULONG,
        Parameters: PULONG_PTR,
        ValidResponseOptions: ULONG,
        Response: PULONG,
    ) -> NTSTATUS;
    fn ZwReadFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;
    fn ZwReadFileScatter(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: PFILE_SEGMENT_ELEMENT,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;
    fn ZwReadOnlyEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwReadRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: ULONG,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesRead: PSIZE_T,
    ) -> NTSTATUS;
    fn ZwReadVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesRead: PSIZE_T,
    ) -> NTSTATUS;
    fn ZwRecoverEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentKey: PVOID,
    ) -> NTSTATUS;
    fn ZwRecoverResourceManager(
        ResourceManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRecoverTransactionManager(
        TransactionManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: PCRM_PROTOCOL_ID,
        ProtocolInformationSize: ULONG,
        ProtocolInformation: PVOID,
        CreateOptions: ULONG,
    ) -> NTSTATUS;
    fn ZwRegisterThreadTerminatePort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwReleaseCMFViewOwnership() -> NTSTATUS;
    fn ZwReleaseKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: PVOID,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwReleaseMutant(
        MutantHandle: HANDLE,
        PreviousCount: PLONG,
    ) -> NTSTATUS;
    fn ZwReleaseSemaphore(
        SemaphoreHandle: HANDLE,
        ReleaseCount: LONG,
        PreviousCount: PLONG,
    ) -> NTSTATUS;
    fn ZwReleaseWorkerFactoryWorker(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRemoveIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut PVOID,
        ApcContext: *mut PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwRemoveIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionInformation: PFILE_IO_COMPLETION_INFORMATION,
        Count: ULONG,
        NumEntriesRemoved: PULONG,
        Timeout: PLARGE_INTEGER,
        Alertable: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwRemoveProcessDebug(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwRenameKey(
        KeyHandle: HANDLE,
        NewName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwRenameTransactionManager(
        LogFileName: PUNICODE_STRING,
        ExistingTransactionManagerGuid: LPGUID,
    ) -> NTSTATUS;
    fn ZwReplaceKey(
        NewFile: POBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwReplacePartitionUnit(
        TargetInstancePath: PUNICODE_STRING,
        SpareInstancePath: PUNICODE_STRING,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut PVOID,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut PVOID,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwReplyWaitReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwRequestPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn ZwRequestWakeupLatency(
        latency: LATENCY_TIME,
    ) -> NTSTATUS;
    fn ZwResetEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;
    fn ZwResetWriteWatch(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        RegionSize: SIZE_T,
    ) -> NTSTATUS;
    fn ZwRestoreKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwResumeProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwResumeThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;
    fn ZwRevertContainerImpersonation() -> NTSTATUS;
    fn ZwRollbackComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwRollbackEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwRollbackTransaction(
        TransactionHandle: HANDLE,
        Wait: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwRollforwardTransactionManager(
        TransactionManagerHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSaveKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSaveKeyEx(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Format: ULONG,
    ) -> NTSTATUS;
    fn ZwSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSecureConnectPort(
        PortHandle: PHANDLE,
        PortName: PUNICODE_STRING,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        RequiredServerSid: PSID,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: PULONG,
        ConnectionInformation: PVOID,
        ConnectionInformationLength: PULONG,
    ) -> NTSTATUS;
    fn ZwSerializeBoot() -> NTSTATUS;
    fn ZwSetBootEntryOrder(
        Ids: PULONG,
        Count: ULONG,
    ) -> NTSTATUS;
    fn ZwSetBootOptions(
        BootOptions: PBOOT_OPTIONS,
        FieldsToChange: ULONG,
    ) -> NTSTATUS;
    fn ZwSetCachedSigningLevel(
        Flags: ULONG,
        InputSigningLevel: SE_SIGNING_LEVEL,
        SourceFiles: PHANDLE,
        SourceFileCount: ULONG,
        TargetFile: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetContextThread(
        ThreadHandle: HANDLE,
        ThreadContext: PCONTEXT,
    ) -> NTSTATUS;
    fn ZwSetDebugFilterState(
        ComponentId: ULONG,
        Level: ULONG,
        State: BOOLEAN,
    ) -> NTSTATUS;
    fn ZwSetDefaultHardErrorPort(
        DefaultHardErrorPort: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetDefaultLocale(
        UserProfile: BOOLEAN,
        DefaultLocaleId: LCID,
    ) -> NTSTATUS;
    fn ZwSetDefaultUILanguage(
        DefaultUILanguageId: LANGID,
    ) -> NTSTATUS;
    fn ZwSetDriverEntryOrder(
        Ids: PULONG,
        Count: ULONG,
    ) -> NTSTATUS;
    fn ZwSetEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;
    fn ZwSetEvent(
        EventHandle: HANDLE,
        PreviousState: PLONG,
    ) -> NTSTATUS;
    fn ZwSetEventBoostPriority(
        EventHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetHighWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetIRTimer(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: PVOID,
        DebugInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: PVOID,
        EnlistmentInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwSetInformationJobObject(
        JobHandle: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        JobObjectInformation: PVOID,
        JobObjectInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationKey(
        KeyHandle: HANDLE,
        KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
        KeySetInformation: PVOID,
        KeySetInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: PVOID,
        ObjectInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: PVOID,
        ResourceManagerInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationThread(
        ThreadHandle: HANDLE,
        ThreadInformationClass: THREADINFOCLASS,
        ThreadInformation: PVOID,
        ThreadInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: PVOID,
        TokenInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: PVOID,
        TransactionInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationTransactionManager(
        TmHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: PVOID,
        TransactionManagerInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationVirtualMemory(
        ProcessHandle: HANDLE,
        VmInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS,
        NumberOfEntries: ULONG_PTR,
        VirtualAddresses: PMEMORY_RANGE_ENTRY,
        VmInformation: PVOID,
        VmInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetInformationWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS,
        WorkerFactoryInformation: PVOID,
        WorkerFactoryInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetIntervalProfile(
        Interval: ULONG,
        Source: KPROFILE_SOURCE,
    ) -> NTSTATUS;
    fn ZwSetIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
    ) -> NTSTATUS;
    fn ZwSetIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionPacketHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
    ) -> NTSTATUS;
    fn ZwSetLdtEntries(
        Selector0: ULONG,
        Entry0Low: ULONG,
        Entry0Hi: ULONG,
        Selector1: ULONG,
        Entry1Low: ULONG,
        Entry1Hi: ULONG,
    ) -> NTSTATUS;
    fn ZwSetLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetLowWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSetQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;
    fn ZwSetSecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn ZwSetSystemEnvironmentValue(
        VariableName: PUNICODE_STRING,
        VariableValue: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwSetSystemEnvironmentValueEx(
        VariableName: PUNICODE_STRING,
        VendorGuid: LPGUID,
        Value: PVOID,
        ValueLength: ULONG,
        Attributes: ULONG,
    ) -> NTSTATUS;
    fn ZwSetSystemInformation(
        SystemInformationClass: SYSTEM_INFORMATION_CLASS,
        SystemInformation: PVOID,
        SystemInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwSetSystemTime(
        SystemTime: PLARGE_INTEGER,
        PreviousTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: *mut EXECUTION_STATE,
    ) -> NTSTATUS;
    fn ZwSetTimer(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
        TimerApcRoutine: PTIMER_APC_ROUTINE,
        TimerContext: PVOID,
        ResumeTimer: BOOLEAN,
        Period: LONG,
        PreviousState: PBOOLEAN,
    ) -> NTSTATUS;
    fn ZwSetTimer2(
        TimerHandle: HANDLE,
        DueTime: PLARGE_INTEGER,
        Period: PLARGE_INTEGER,
        Parameters: PT2_SET_PARAMETERS,
    ) -> NTSTATUS;
    fn ZwSetTimerEx(
        TimerHandle: HANDLE,
        TimerSetInformationClass: TIMER_SET_INFORMATION_CLASS,
        TimerSetInformation: PVOID,
        TimerSetInformationLength: ULONG,
    ) -> NTSTATUS;
    fn ZwSetTimerResolution(
        DesiredTime: ULONG,
        SetResolution: BOOLEAN,
        ActualTime: PULONG,
    ) -> NTSTATUS;
    fn ZwSetUuidSeed(
        Seed: PCHAR,
    ) -> NTSTATUS;
    fn ZwSetValueKey(
        KeyHandle: HANDLE,
        ValueName: PUNICODE_STRING,
        TitleIndex: ULONG,
        Type: ULONG,
        Data: PVOID,
        DataSize: ULONG,
    ) -> NTSTATUS;
    fn ZwSetVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: PVOID,
        Length: ULONG,
        FsInformationClass: FS_INFORMATION_CLASS,
    ) -> NTSTATUS;
    fn ZwSetWnfProcessNotificationEvent(
        NotificationEvent: HANDLE,
    ) -> NTSTATUS;
    fn ZwShutdownSystem(
        Action: SHUTDOWN_ACTION,
    ) -> NTSTATUS;
    fn ZwShutdownWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        PendingWorkerCount: *mut LONG,
    ) -> NTSTATUS;
    fn ZwSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwSinglePhaseReject(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwStartProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwStopProfile(
        ProfileHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSubscribeWnfStateChange(
        StateName: PCWNF_STATE_NAME,
        ChangeStamp: WNF_CHANGE_STAMP,
        EventMask: ULONG,
        SubscriptionId: PULONG64,
    ) -> NTSTATUS;
    fn ZwSuspendProcess(
        ProcessHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwSuspendThread(
        ThreadHandle: HANDLE,
        PreviousSuspendCount: PULONG,
    ) -> NTSTATUS;
    fn ZwSystemDebugControl(
        Command: SYSDBG_COMMAND,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwTerminateJobObject(
        JobHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwTerminateProcess(
        ProcessHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwTerminateThread(
        ThreadHandle: HANDLE,
        ExitStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn ZwTestAlert() -> NTSTATUS;
    fn ZwThawRegistry() -> NTSTATUS;
    fn ZwThawTransactions() -> NTSTATUS;
    fn ZwTraceControl(
        FunctionCode: ULONG,
        InBuffer: PVOID,
        InBufferLen: ULONG,
        OutBuffer: PVOID,
        OutBufferLen: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn ZwTraceEvent(
        TraceHandle: HANDLE,
        Flags: ULONG,
        FieldSize: ULONG,
        Fields: PVOID,
    ) -> NTSTATUS;
    fn ZwTranslateFilePath(
        InputFilePath: PFILE_PATH,
        OutputType: ULONG,
        OutputFilePath: PFILE_PATH,
        OutputFilePathLength: PULONG,
    ) -> NTSTATUS;
    fn ZwUmsThreadYield(
        SchedulerParam: PVOID,
    ) -> NTSTATUS;
    fn ZwUnloadDriver(
        DriverServiceName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn ZwUnloadKey(
        TargetKey: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn ZwUnloadKey2(
        TargetKey: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwUnloadKeyEx(
        TargetKey: POBJECT_ATTRIBUTES,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn ZwUnlockFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: PLARGE_INTEGER,
        Length: PLARGE_INTEGER,
        Key: ULONG,
    ) -> NTSTATUS;
    fn ZwUnlockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        MapType: ULONG,
    ) -> NTSTATUS;
    fn ZwUnmapViewOfSection(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
    ) -> NTSTATUS;
    fn ZwUnmapViewOfSectionEx(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn ZwUnsubscribeWnfStateChange(
        StateName: PCWNF_STATE_NAME,
    ) -> NTSTATUS;
    fn ZwUpdateWnfStateData(
        StateName: PCWNF_STATE_NAME,
        Buffer: *const VOID,
        Length: ULONG,
        TypeId: PCWNF_TYPE_ID,
        ExplicitScope: *const VOID,
        MatchingChangeStamp: WNF_CHANGE_STAMP,
        CheckStamp: LOGICAL,
    ) -> NTSTATUS;
    fn ZwVdmControl(
        Service: VDMSERVICECLASS,
        ServiceData: PVOID,
    ) -> NTSTATUS;
    fn ZwWaitForAlertByThreadId(
        Address: PVOID,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
        WaitStateChange: PVOID,
    ) -> NTSTATUS;
    fn ZwWaitForKeyedEvent(
        KeyedEventHandle: HANDLE,
        KeyValue: PVOID,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForMultipleObjects(
        Count: ULONG,
        Handles: *mut HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForMultipleObjects32(
        Count: ULONG,
        Handles: *mut LONG,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForSingleObject(
        Handle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn ZwWaitForWorkViaWorkerFactory(
        WorkerFactoryHandle: HANDLE,
        MiniPacket: *mut FILE_IO_COMPLETION_INFORMATION,
    ) -> NTSTATUS;
    fn ZwWaitHighEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwWaitLowEventPair(
        EventPairHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwWorkerFactoryWorkerReady(
        WorkerFactoryHandle: HANDLE,
    ) -> NTSTATUS;
    fn ZwWriteFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;
    fn ZwWriteFileGather(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: PFILE_SEGMENT_ELEMENT,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;
    fn ZwWriteRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: ULONG,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesWritten: PSIZE_T,
    ) -> NTSTATUS;
    fn ZwWriteVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesWritten: PSIZE_T,
    ) -> NTSTATUS;
    fn ZwYieldExecution() -> NTSTATUS;
}}
