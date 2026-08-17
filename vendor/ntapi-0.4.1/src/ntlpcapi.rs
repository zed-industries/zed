use core::mem::size_of;
use crate::ntapi_base::{CLIENT_ID, CLIENT_ID64};
use winapi::ctypes::c_double;
use winapi::shared::basetsd::{PSIZE_T, SIZE_T, ULONG64, ULONG_PTR};
use winapi::shared::ntdef::{
    BOOLEAN, CSHORT, HANDLE, LARGE_INTEGER, NTSTATUS, OBJ_CASE_INSENSITIVE, PHANDLE,
    PLARGE_INTEGER, POBJECT_ATTRIBUTES, PULONG, PUNICODE_STRING, PVOID, ULONG, ULONGLONG,
    UNICODE_STRING,
};
use winapi::um::winnt::{
    ACCESS_MASK, PSECURITY_DESCRIPTOR, PSECURITY_QUALITY_OF_SERVICE, PSID, RTL_SRWLOCK,
    SECURITY_QUALITY_OF_SERVICE, STANDARD_RIGHTS_REQUIRED, SYNCHRONIZE,
};
pub const PORT_CONNECT: u32 = 0x0001;
pub const PORT_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x1;
STRUCT!{struct PORT_MESSAGE_u1_s {
    DataLength: CSHORT,
    TotalLength: CSHORT,
}}
STRUCT!{struct PORT_MESSAGE_u2_s {
    Type: CSHORT,
    DataInfoOffset: CSHORT,
}}
UNION!{union PORT_MESSAGE_u1 {
    s: PORT_MESSAGE_u1_s,
    Length: ULONG,
}}
UNION!{union PORT_MESSAGE_u2 {
    s: PORT_MESSAGE_u2_s,
    ZeroInit: ULONG,
}}
UNION!{union PORT_MESSAGE_u3 {
    ClientId: CLIENT_ID,
    DoNotUseThisField: c_double,
}}
UNION!{union PORT_MESSAGE_u4 {
    ClientViewSize: SIZE_T,
    CallbackId: ULONG,
}}
STRUCT!{struct PORT_MESSAGE {
    u1: PORT_MESSAGE_u1,
    u2: PORT_MESSAGE_u2,
    u3: PORT_MESSAGE_u3,
    MessageId: ULONG,
    u4: PORT_MESSAGE_u4,
}}
pub type PPORT_MESSAGE = *mut PORT_MESSAGE;
STRUCT!{struct PORT_DATA_ENTRY {
    Base: PVOID,
    Size: ULONG,
}}
pub type PPORT_DATA_ENTRY = *mut PORT_DATA_ENTRY;
STRUCT!{struct PORT_DATA_INFORMATION {
    CountDataEntries: ULONG,
    DataEntries: [PORT_DATA_ENTRY; 1],
}}
pub type PPORT_DATA_INFORMATION = *mut PORT_DATA_INFORMATION;
pub const LPC_REQUEST: ULONG = 1;
pub const LPC_REPLY: ULONG = 2;
pub const LPC_DATAGRAM: ULONG = 3;
pub const LPC_LOST_REPLY: ULONG = 4;
pub const LPC_PORT_CLOSED: ULONG = 5;
pub const LPC_CLIENT_DIED: ULONG = 6;
pub const LPC_EXCEPTION: ULONG = 7;
pub const LPC_DEBUG_EVENT: ULONG = 8;
pub const LPC_ERROR_EVENT: ULONG = 9;
pub const LPC_CONNECTION_REQUEST: ULONG = 10;
pub const LPC_KERNELMODE_MESSAGE: CSHORT = 0x8000;
pub const LPC_NO_IMPERSONATE: CSHORT = 0x4000;
pub const PORT_VALID_OBJECT_ATTRIBUTES: u32 = OBJ_CASE_INSENSITIVE;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 512;
#[cfg(target_arch = "x86")]
pub const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 256;
pub const LPC_MAX_CONNECTION_INFO_SIZE: u32 = 16 * size_of::<ULONG_PTR>() as u32;
pub const PORT_TOTAL_MAXIMUM_MESSAGE_LENGTH: u32 = (PORT_MAXIMUM_MESSAGE_LENGTH
    + size_of::<PORT_MESSAGE>() as u32
    + LPC_MAX_CONNECTION_INFO_SIZE
    + 0xf) & !0xf;
STRUCT!{struct LPC_CLIENT_DIED_MSG {
    PortMsg: PORT_MESSAGE,
    CreateTime: LARGE_INTEGER,
}}
pub type PLPC_CLIENT_DIED_MSG = *mut LPC_CLIENT_DIED_MSG;
STRUCT!{struct PORT_VIEW {
    Length: ULONG,
    SectionHandle: HANDLE,
    SectionOffset: ULONG,
    ViewSize: SIZE_T,
    ViewBase: PVOID,
    ViewRemoteBase: PVOID,
}}
pub type PPORT_VIEW = *mut PORT_VIEW;
STRUCT!{struct REMOTE_PORT_VIEW {
    Length: ULONG,
    ViewSize: SIZE_T,
    ViewBase: PVOID,
}}
pub type PREMOTE_PORT_VIEW = *mut REMOTE_PORT_VIEW;
STRUCT!{struct PORT_MESSAGE64_u1_s {
    DataLength: CSHORT,
    TotalLength: CSHORT,
}}
STRUCT!{struct PORT_MESSAGE64_u2_s {
    Type: CSHORT,
    DataInfoOffset: CSHORT,
}}
UNION!{union PORT_MESSAGE64_u1 {
    s: PORT_MESSAGE64_u1_s,
    Length: ULONG,
}}
UNION!{union PORT_MESSAGE64_u2 {
    s: PORT_MESSAGE64_u2_s,
    ZeroInit: ULONG,
}}
UNION!{union PORT_MESSAGE64_u3 {
    ClientId: CLIENT_ID64,
    DoNotUseThisField: c_double,
}}
UNION!{union PORT_MESSAGE64_u4 {
    ClientViewSize: ULONGLONG,
    CallbackId: ULONG,
}}
STRUCT!{struct PORT_MESSAGE64 {
    u1: PORT_MESSAGE64_u1,
    u2: PORT_MESSAGE64_u2,
    u3: PORT_MESSAGE64_u3,
    MessageId: ULONG,
    u4: PORT_MESSAGE64_u4,
}}
pub type PPORT_MESSAGE64 = *mut PORT_MESSAGE64;
STRUCT!{struct LPC_CLIENT_DIED_MSG64 {
    PortMsg: PORT_MESSAGE64,
    CreateTime: LARGE_INTEGER,
}}
pub type PLPC_CLIENT_DIED_MSG64 = *mut LPC_CLIENT_DIED_MSG64;
STRUCT!{struct PORT_VIEW64 {
    Length: ULONG,
    SectionHandle: ULONGLONG,
    SectionOffset: ULONG,
    ViewSize: ULONGLONG,
    ViewBase: ULONGLONG,
    ViewRemoteBase: ULONGLONG,
}}
pub type PPORT_VIEW64 = *mut PORT_VIEW64;
STRUCT!{struct REMOTE_PORT_VIEW64 {
    Length: ULONG,
    ViewSize: ULONGLONG,
    ViewBase: ULONGLONG,
}}
pub type PREMOTE_PORT_VIEW64 = *mut REMOTE_PORT_VIEW64;
EXTERN!{extern "system" {
    fn NtCreatePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: ULONG,
        MaxMessageLength: ULONG,
        MaxPoolUsage: ULONG,
    ) -> NTSTATUS;
    fn NtCreateWaitablePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        MaxConnectionInfoLength: ULONG,
        MaxMessageLength: ULONG,
        MaxPoolUsage: ULONG,
    ) -> NTSTATUS;
    fn NtConnectPort(
        PortHandle: PHANDLE,
        PortName: PUNICODE_STRING,
        SecurityQos: PSECURITY_QUALITY_OF_SERVICE,
        ClientView: PPORT_VIEW,
        ServerView: PREMOTE_PORT_VIEW,
        MaxMessageLength: PULONG,
        ConnectionInformation: PVOID,
        ConnectionInformationLength: PULONG,
    ) -> NTSTATUS;
    fn NtSecureConnectPort(
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
    fn NtListenPort(
        PortHandle: HANDLE,
        ConnectionRequest: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtAcceptConnectPort(
        PortHandle: PHANDLE,
        PortContext: PVOID,
        ConnectionRequest: PPORT_MESSAGE,
        AcceptConnection: BOOLEAN,
        ServerView: PPORT_VIEW,
        ClientView: PREMOTE_PORT_VIEW,
    ) -> NTSTATUS;
    fn NtCompleteConnectPort(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtRequestPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtRequestWaitReplyPort(
        PortHandle: HANDLE,
        RequestMessage: PPORT_MESSAGE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyWaitReplyPort(
        PortHandle: HANDLE,
        ReplyMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyWaitReceivePort(
        PortHandle: HANDLE,
        PortContext: *mut PVOID,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReplyWaitReceivePortEx(
        PortHandle: HANDLE,
        PortContext: *mut PVOID,
        ReplyMessage: PPORT_MESSAGE,
        ReceiveMessage: PPORT_MESSAGE,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
    ) -> NTSTATUS;
    fn NtReadRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: ULONG,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesRead: PSIZE_T,
    ) -> NTSTATUS;
    fn NtWriteRequestData(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        DataEntryIndex: ULONG,
        Buffer: PVOID,
        BufferSize: SIZE_T,
        NumberOfBytesWritten: PSIZE_T,
    ) -> NTSTATUS;
}}
ENUM!{enum PORT_INFORMATION_CLASS {
    PortBasicInformation = 0,
    PortDumpInformation = 1,
}}
EXTERN!{extern "system" {
    fn NtQueryInformationPort(
        PortHandle: HANDLE,
        PortInformationClass: PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
pub type PALPC_HANDLE = *mut HANDLE;
pub type ALPC_HANDLE = HANDLE;
pub const ALPC_PORFLG_ALLOW_LPC_REQUESTS: ULONG = 0x20000;
pub const ALPC_PORFLG_WAITABLE_PORT: ULONG = 0x40000;
pub const ALPC_PORFLG_SYSTEM_PROCESS: ULONG = 0x100000;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
STRUCT!{struct ALPC_PORT_ATTRIBUTES {
    Flags: ULONG,
    SecurityQos: SECURITY_QUALITY_OF_SERVICE,
    MaxMessageLength: SIZE_T,
    MemoryBandwidth: SIZE_T,
    MaxPoolUsage: SIZE_T,
    MaxSectionSize: SIZE_T,
    MaxViewSize: SIZE_T,
    MaxTotalSectionSize: SIZE_T,
    DupObjectTypes: ULONG,
    Reserved: ULONG,
}}
#[cfg(target_arch = "x86")]
STRUCT!{struct ALPC_PORT_ATTRIBUTES {
    Flags: ULONG,
    SecurityQos: SECURITY_QUALITY_OF_SERVICE,
    MaxMessageLength: SIZE_T,
    MemoryBandwidth: SIZE_T,
    MaxPoolUsage: SIZE_T,
    MaxSectionSize: SIZE_T,
    MaxViewSize: SIZE_T,
    MaxTotalSectionSize: SIZE_T,
    DupObjectTypes: ULONG,
}}
pub type PALPC_PORT_ATTRIBUTES = *mut ALPC_PORT_ATTRIBUTES;
pub const ALPC_MESSAGE_SECURITY_ATTRIBUTE: ULONG = 0x80000000;
pub const ALPC_MESSAGE_VIEW_ATTRIBUTE: ULONG = 0x40000000;
pub const ALPC_MESSAGE_CONTEXT_ATTRIBUTE: ULONG = 0x20000000;
pub const ALPC_MESSAGE_HANDLE_ATTRIBUTE: ULONG = 0x10000000;
STRUCT!{struct ALPC_MESSAGE_ATTRIBUTES {
    AllocatedAttributes: ULONG,
    ValidAttributes: ULONG,
}}
pub type PALPC_MESSAGE_ATTRIBUTES = *mut ALPC_MESSAGE_ATTRIBUTES;
STRUCT!{struct ALPC_COMPLETION_LIST_STATE {
    Value: ULONG64,
}}
BITFIELD!{ALPC_COMPLETION_LIST_STATE Value: ULONG64 [
    Head set_Head[0..24],
    Tail set_Tail[24..48],
    ActiveThreadCount set_ActiveThreadCount[48..64],
]}
pub type PALPC_COMPLETION_LIST_STATE = *mut ALPC_COMPLETION_LIST_STATE;
pub const ALPC_COMPLETION_LIST_BUFFER_GRANULARITY_MASK: ULONG = 0x3f;
STRUCT!{#[repr(align(128))] struct ALPC_COMPLETION_LIST_HEADER {
    StartMagic: ULONG64,
    TotalSize: ULONG,
    ListOffset: ULONG,
    ListSize: ULONG,
    BitmapOffset: ULONG,
    BitmapSize: ULONG,
    DataOffset: ULONG,
    DataSize: ULONG,
    AttributeFlags: ULONG,
    AttributeSize: ULONG,
    __padding0: [u64; 10],
    State: ALPC_COMPLETION_LIST_STATE,
    LastMessageId: ULONG,
    LastCallbackId: ULONG,
    __padding1: [u32; 28],
    PostCount: ULONG,
    __padding2: [u32; 31],
    ReturnCount: ULONG,
    __padding3: [u32; 31],
    LogSequenceNumber: ULONG,
    __padding4: [u64; 15],
    UserLock: RTL_SRWLOCK,
    EndMagic: ULONG64,
    __padding5: [u64; 14],
}}
pub type PALPC_COMPLETION_LIST_HEADER = *mut ALPC_COMPLETION_LIST_HEADER;
STRUCT!{struct ALPC_CONTEXT_ATTR {
    PortContext: PVOID,
    MessageContext: PVOID,
    Sequence: ULONG,
    MessageId: ULONG,
    CallbackId: ULONG,
}}
pub type PALPC_CONTEXT_ATTR = *mut ALPC_CONTEXT_ATTR;
pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ACCESS: ULONG = 0x10000;
pub const ALPC_HANDLEFLG_DUPLICATE_SAME_ATTRIBUTES: ULONG = 0x20000;
pub const ALPC_HANDLEFLG_DUPLICATE_INHERIT: ULONG = 0x80000;
STRUCT!{struct ALPC_HANDLE_ATTR32 {
    Flags: ULONG,
    Reserved0: ULONG,
    SameAccess: ULONG,
    SameAttributes: ULONG,
    Indirect: ULONG,
    Inherit: ULONG,
    Reserved1: ULONG,
    Handle: ULONG,
    ObjectType: ULONG,
    DesiredAccess: ULONG,
    GrantedAccess: ULONG,
}}
pub type PALPC_HANDLE_ATTR32 = *mut ALPC_HANDLE_ATTR32;
STRUCT!{struct ALPC_HANDLE_ATTR {
    Flags: ULONG,
    Reserved0: ULONG,
    SameAccess: ULONG,
    SameAttributes: ULONG,
    Indirect: ULONG,
    Inherit: ULONG,
    Reserved1: ULONG,
    Handle: HANDLE,
    HandleAttrArray: PALPC_HANDLE_ATTR32,
    ObjectType: ULONG,
    HandleCount: ULONG,
    DesiredAccess: ACCESS_MASK,
    GrantedAccess: ACCESS_MASK,
}}
pub type PALPC_HANDLE_ATTR = *mut ALPC_HANDLE_ATTR;
pub const ALPC_SECFLG_CREATE_HANDLE: ULONG = 0x20000;
STRUCT!{struct ALPC_SECURITY_ATTR {
    Flags: ULONG,
    QoS: PSECURITY_QUALITY_OF_SERVICE,
    ContextHandle: ALPC_HANDLE,
}}
pub type PALPC_SECURITY_ATTR = *mut ALPC_SECURITY_ATTR;
pub const ALPC_VIEWFLG_NOT_SECURE: ULONG = 0x40000;
STRUCT!{struct ALPC_DATA_VIEW_ATTR {
    Flags: ULONG,
    SectionHandle: ALPC_HANDLE,
    ViewBase: PVOID,
    ViewSize: SIZE_T,
}}
pub type PALPC_DATA_VIEW_ATTR = *mut ALPC_DATA_VIEW_ATTR;
ENUM!{enum ALPC_PORT_INFORMATION_CLASS {
    AlpcBasicInformation = 0,
    AlpcPortInformation = 1,
    AlpcAssociateCompletionPortInformation = 2,
    AlpcConnectedSIDInformation = 3,
    AlpcServerInformation = 4,
    AlpcMessageZoneInformation = 5,
    AlpcRegisterCompletionListInformation = 6,
    AlpcUnregisterCompletionListInformation = 7,
    AlpcAdjustCompletionListConcurrencyCountInformation = 8,
    AlpcRegisterCallbackInformation = 9,
    AlpcCompletionListRundownInformation = 10,
    AlpcWaitForPortReferences = 11,
}}
STRUCT!{struct ALPC_BASIC_INFORMATION {
    Flags: ULONG,
    SequenceNo: ULONG,
    PortContext: PVOID,
}}
pub type PALPC_BASIC_INFORMATION = *mut ALPC_BASIC_INFORMATION;
STRUCT!{struct ALPC_PORT_ASSOCIATE_COMPLETION_PORT {
    CompletionKey: PVOID,
    CompletionPort: HANDLE,
}}
pub type PALPC_PORT_ASSOCIATE_COMPLETION_PORT = *mut ALPC_PORT_ASSOCIATE_COMPLETION_PORT;
STRUCT!{struct ALPC_SERVER_INFORMATION_Out {
    ThreadBlocked: BOOLEAN,
    ConnectedProcessId: HANDLE,
    ConnectionPortName: UNICODE_STRING,
}}
UNION!{union ALPC_SERVER_INFORMATION {
    ThreadHandle: HANDLE,
    Out: ALPC_SERVER_INFORMATION_Out,
}}
pub type PALPC_SERVER_INFORMATION = *mut ALPC_SERVER_INFORMATION;
STRUCT!{struct ALPC_PORT_MESSAGE_ZONE_INFORMATION {
    Buffer: PVOID,
    Size: ULONG,
}}
pub type PALPC_PORT_MESSAGE_ZONE_INFORMATION = *mut ALPC_PORT_MESSAGE_ZONE_INFORMATION;
STRUCT!{struct ALPC_PORT_COMPLETION_LIST_INFORMATION {
    Buffer: PVOID,
    Size: ULONG,
    ConcurrencyCount: ULONG,
    AttributeFlags: ULONG,
}}
pub type PALPC_PORT_COMPLETION_LIST_INFORMATION = *mut ALPC_PORT_COMPLETION_LIST_INFORMATION;
ENUM!{enum ALPC_MESSAGE_INFORMATION_CLASS {
    AlpcMessageSidInformation = 0,
    AlpcMessageTokenModifiedIdInformation = 1,
    AlpcMessageDirectStatusInformation = 2,
    AlpcMessageHandleInformation = 3,
    MaxAlpcMessageInfoClass = 4,
}}
pub type PALPC_MESSAGE_INFORMATION_CLASS = *mut ALPC_MESSAGE_INFORMATION_CLASS;
STRUCT!{struct ALPC_MESSAGE_HANDLE_INFORMATION {
    Index: ULONG,
    Flags: ULONG,
    Handle: ULONG,
    ObjectType: ULONG,
    GrantedAccess: ACCESS_MASK,
}}
pub type PALPC_MESSAGE_HANDLE_INFORMATION = *mut ALPC_MESSAGE_HANDLE_INFORMATION;
EXTERN!{extern "system" {
    fn NtAlpcCreatePort(
        PortHandle: PHANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        PortAttributes: PALPC_PORT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtAlpcDisconnectPort(
        PortHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtAlpcQueryInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtAlpcSetInformation(
        PortHandle: HANDLE,
        PortInformationClass: ALPC_PORT_INFORMATION_CLASS,
        PortInformation: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;
    fn NtAlpcCreatePortSection(
        PortHandle: HANDLE,
        Flags: ULONG,
        SectionHandle: HANDLE,
        SectionSize: SIZE_T,
        AlpcSectionHandle: PALPC_HANDLE,
        ActualSectionSize: PSIZE_T,
    ) -> NTSTATUS;
    fn NtAlpcDeletePortSection(
        PortHandle: HANDLE,
        Flags: ULONG,
        SectionHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcCreateResourceReserve(
        PortHandle: HANDLE,
        Flags: ULONG,
        MessageSize: SIZE_T,
        ResourceId: PALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcDeleteResourceReserve(
        PortHandle: HANDLE,
        Flags: ULONG,
        ResourceId: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcCreateSectionView(
        PortHandle: HANDLE,
        Flags: ULONG,
        ViewAttributes: PALPC_DATA_VIEW_ATTR,
    ) -> NTSTATUS;
    fn NtAlpcDeleteSectionView(
        PortHandle: HANDLE,
        Flags: ULONG,
        ViewBase: PVOID,
    ) -> NTSTATUS;
    fn NtAlpcCreateSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        SecurityAttribute: PALPC_SECURITY_ATTR,
    ) -> NTSTATUS;
    fn NtAlpcDeleteSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcRevokeSecurityContext(
        PortHandle: HANDLE,
        Flags: ULONG,
        ContextHandle: ALPC_HANDLE,
    ) -> NTSTATUS;
    fn NtAlpcQueryInformationMessage(
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS,
        MessageInformation: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}}
pub const ALPC_MSGFLG_REPLY_MESSAGE: ULONG = 0x1;
pub const ALPC_MSGFLG_LPC_MODE: ULONG = 0x2;
pub const ALPC_MSGFLG_RELEASE_MESSAGE: ULONG = 0x10000;
pub const ALPC_MSGFLG_SYNC_REQUEST: ULONG = 0x20000;
pub const ALPC_MSGFLG_WAIT_USER_MODE: ULONG = 0x100000;
pub const ALPC_MSGFLG_WAIT_ALERTABLE: ULONG = 0x200000;
pub const ALPC_MSGFLG_WOW64_CALL: ULONG = 0x80000000;
EXTERN!{extern "system" {
    fn NtAlpcConnectPort(
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
    fn NtAlpcConnectPortEx(
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
    fn NtAlpcAcceptConnectPort(
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
    fn NtAlpcSendWaitReceivePort(
        PortHandle: HANDLE,
        Flags: ULONG,
        SendMessageA: PPORT_MESSAGE,
        SendMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        ReceiveMessage: PPORT_MESSAGE,
        BufferLength: PSIZE_T,
        ReceiveMessageAttributes: PALPC_MESSAGE_ATTRIBUTES,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
}}
pub const ALPC_CANCELFLG_TRY_CANCEL: ULONG = 0x1;
pub const ALPC_CANCELFLG_NO_CONTEXT_CHECK: ULONG = 0x8;
pub const ALPC_CANCELFLGP_FLUSH: ULONG = 0x10000;
EXTERN!{extern "system" {
    fn NtAlpcCancelMessage(
        PortHandle: HANDLE,
        Flags: ULONG,
        MessageContext: PALPC_CONTEXT_ATTR,
    ) -> NTSTATUS;
    fn NtAlpcImpersonateClientOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: PVOID,
    ) -> NTSTATUS;
    fn NtAlpcImpersonateClientContainerOfPort(
        PortHandle: HANDLE,
        Message: PPORT_MESSAGE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtAlpcOpenSenderProcess(
        ProcessHandle: PHANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: ULONG,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtAlpcOpenSenderThread(
        ThreadHandle: PHANDLE,
        PortHandle: HANDLE,
        PortMessage: PPORT_MESSAGE,
        Flags: ULONG,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn AlpcMaxAllowedMessageLength() -> ULONG;
    fn AlpcGetHeaderSize(
        Flags: ULONG,
    ) -> ULONG;
    fn AlpcInitializeMessageAttribute(
        AttributeFlags: ULONG,
        Buffer: PALPC_MESSAGE_ATTRIBUTES,
        BufferSize: ULONG,
        RequiredBufferSize: PULONG,
    ) -> NTSTATUS;
    fn AlpcGetMessageAttribute(
        Buffer: PALPC_MESSAGE_ATTRIBUTES,
        AttributeFlag: ULONG,
    ) -> PVOID;
    fn AlpcRegisterCompletionList(
        PortHandle: HANDLE,
        Buffer: PALPC_COMPLETION_LIST_HEADER,
        Size: ULONG,
        ConcurrencyCount: ULONG,
        AttributeFlags: ULONG,
    ) -> NTSTATUS;
    fn AlpcUnregisterCompletionList(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn AlpcRundownCompletionList(
        PortHandle: HANDLE,
    ) -> NTSTATUS;
    fn AlpcAdjustCompletionListConcurrencyCount(
        PortHandle: HANDLE,
        ConcurrencyCount: ULONG,
    ) -> NTSTATUS;
    fn AlpcRegisterCompletionListWorkerThread(
        CompletionList: PVOID,
    ) -> BOOLEAN;
    fn AlpcUnregisterCompletionListWorkerThread(
        CompletionList: PVOID,
    ) -> BOOLEAN;
    fn AlpcGetCompletionListLastMessageInformation(
        CompletionList: PVOID,
        LastMessageId: PULONG,
        LastCallbackId: PULONG,
    );
    fn AlpcGetOutstandingCompletionListMessageCount(
        CompletionList: PVOID,
    ) -> ULONG;
    fn AlpcGetMessageFromCompletionList(
        CompletionList: PVOID,
        MessageAttributes: *mut PALPC_MESSAGE_ATTRIBUTES,
    ) -> PPORT_MESSAGE;
    fn AlpcFreeCompletionListMessage(
        CompletionList: PVOID,
        Message: PPORT_MESSAGE,
    );
    fn AlpcGetCompletionListMessageAttributes(
        CompletionList: PVOID,
        Message: PPORT_MESSAGE,
    ) -> PALPC_MESSAGE_ATTRIBUTES;
}}
