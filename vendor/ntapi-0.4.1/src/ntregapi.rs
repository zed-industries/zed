use crate::ntioapi::{PIO_APC_ROUTINE, PIO_STATUS_BLOCK};
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, LARGE_INTEGER, NTSTATUS, OBJECT_ATTRIBUTES, PHANDLE, POBJECT_ATTRIBUTES,
    PULONG, PUNICODE_STRING, PVOID, UCHAR, ULONG, UNICODE_STRING, USHORT, WCHAR,
};
use winapi::um::winnt::ACCESS_MASK;
pub const REG_INIT_BOOT_SM: USHORT = 0x0000;
pub const REG_INIT_BOOT_SETUP: USHORT = 0x0001;
pub const REG_INIT_BOOT_ACCEPTED_BASE: USHORT = 0x0002;
pub const REG_INIT_BOOT_ACCEPTED_MAX: USHORT = REG_INIT_BOOT_ACCEPTED_BASE;
pub const REG_MAX_KEY_VALUE_NAME_LENGTH: u32 = 32767;
pub const REG_MAX_KEY_NAME_LENGTH: u32 = 512;
ENUM!{enum KEY_INFORMATION_CLASS {
    KeyBasicInformation = 0,
    KeyNodeInformation = 1,
    KeyFullInformation = 2,
    KeyNameInformation = 3,
    KeyCachedInformation = 4,
    KeyFlagsInformation = 5,
    KeyVirtualizationInformation = 6,
    KeyHandleTagsInformation = 7,
    KeyTrustInformation = 8,
    KeyLayerInformation = 9,
    MaxKeyInfoClass = 10,
}}
STRUCT!{struct KEY_BASIC_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: ULONG,
    NameLength: ULONG,
    Name: [WCHAR; 1],
}}
pub type PKEY_BASIC_INFORMATION = *mut KEY_BASIC_INFORMATION;
STRUCT!{struct KEY_NODE_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: ULONG,
    ClassOffset: ULONG,
    ClassLength: ULONG,
    NameLength: ULONG,
    Name: [WCHAR; 1],
}}
pub type PKEY_NODE_INFORMATION = *mut KEY_NODE_INFORMATION;
STRUCT!{struct KEY_FULL_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: ULONG,
    ClassOffset: ULONG,
    ClassLength: ULONG,
    SubKeys: ULONG,
    MaxNameLen: ULONG,
    MaxClassLen: ULONG,
    Values: ULONG,
    MaxValueNameLen: ULONG,
    MaxValueDataLen: ULONG,
    Class: [WCHAR; 1],
}}
pub type PKEY_FULL_INFORMATION = *mut KEY_FULL_INFORMATION;
STRUCT!{struct KEY_NAME_INFORMATION {
    NameLength: ULONG,
    Name: [WCHAR; 1],
}}
pub type PKEY_NAME_INFORMATION = *mut KEY_NAME_INFORMATION;
STRUCT!{struct KEY_CACHED_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
    TitleIndex: ULONG,
    SubKeys: ULONG,
    MaxNameLen: ULONG,
    Values: ULONG,
    MaxValueNameLen: ULONG,
    MaxValueDataLen: ULONG,
    NameLength: ULONG,
    Name: [WCHAR; 1],
}}
pub type PKEY_CACHED_INFORMATION = *mut KEY_CACHED_INFORMATION;
STRUCT!{struct KEY_FLAGS_INFORMATION {
    UserFlags: ULONG,
}}
pub type PKEY_FLAGS_INFORMATION = *mut KEY_FLAGS_INFORMATION;
STRUCT!{struct KEY_VIRTUALIZATION_INFORMATION {
    Bitfields: ULONG,
}}
BITFIELD!{KEY_VIRTUALIZATION_INFORMATION Bitfields: ULONG [
    VirtualizationCandidate set_VirtualizationCandidate[0..1],
    VirtualizationEnabled set_VirtualizationEnabled[1..2],
    VirtualTarget set_VirtualTarget[2..3],
    VirtualStore set_VirtualStore[3..4],
    VirtualSource set_VirtualSource[4..5],
    Reserved set_Reserved[5..32],
]}
pub type PKEY_VIRTUALIZATION_INFORMATION = *mut KEY_VIRTUALIZATION_INFORMATION;
STRUCT!{struct KEY_TRUST_INFORMATION {
    Bitfields: ULONG,
}}
BITFIELD!{KEY_TRUST_INFORMATION Bitfields: ULONG [
    TrustedKey set_TrustedKey[0..1],
    Reserved set_Reserved[1..32],
]}
pub type PKEY_TRUST_INFORMATION = *mut KEY_TRUST_INFORMATION;
STRUCT!{struct KEY_LAYER_INFORMATION {
    IsTombstone: ULONG,
    IsSupersedeLocal: ULONG,
    IsSupersedeTree: ULONG,
    ClassIsInherited: ULONG,
    Reserved: ULONG,
}}
pub type PKEY_LAYER_INFORMATION = *mut KEY_LAYER_INFORMATION;
ENUM!{enum KEY_SET_INFORMATION_CLASS {
    KeyWriteTimeInformation = 0,
    KeyWow64FlagsInformation = 1,
    KeyControlFlagsInformation = 2,
    KeySetVirtualizationInformation = 3,
    KeySetDebugInformation = 4,
    KeySetHandleTagsInformation = 5,
    KeySetLayerInformation = 6,
    MaxKeySetInfoClass = 7,
}}
STRUCT!{struct KEY_WRITE_TIME_INFORMATION {
    LastWriteTime: LARGE_INTEGER,
}}
pub type PKEY_WRITE_TIME_INFORMATION = *mut KEY_WRITE_TIME_INFORMATION;
STRUCT!{struct KEY_WOW64_FLAGS_INFORMATION {
    UserFlags: ULONG,
}}
pub type PKEY_WOW64_FLAGS_INFORMATION = *mut KEY_WOW64_FLAGS_INFORMATION;
STRUCT!{struct KEY_HANDLE_TAGS_INFORMATION {
    HandleTags: ULONG,
}}
pub type PKEY_HANDLE_TAGS_INFORMATION = *mut KEY_HANDLE_TAGS_INFORMATION;
STRUCT!{struct KEY_SET_LAYER_INFORMATION {
    Bitfields: ULONG,
}}
BITFIELD!{KEY_SET_LAYER_INFORMATION Bitfields: ULONG [
    IsTombstone set_IsTombstone[0..1],
    IsSupersedeLocal set_IsSupersedeLocal[1..2],
    IsSupersedeTree set_IsSupersedeTree[2..3],
    ClassIsInherited set_ClassIsInherited[3..4],
    Reserved set_Reserved[4..32],
]}
pub type PKEY_SET_LAYER_INFORMATION = *mut KEY_SET_LAYER_INFORMATION;
STRUCT!{struct KEY_CONTROL_FLAGS_INFORMATION {
    ControlFlags: ULONG,
}}
pub type PKEY_CONTROL_FLAGS_INFORMATION = *mut KEY_CONTROL_FLAGS_INFORMATION;
STRUCT!{struct KEY_SET_VIRTUALIZATION_INFORMATION {
    HandleTags: ULONG,
}}
BITFIELD!{KEY_SET_VIRTUALIZATION_INFORMATION HandleTags: ULONG [
    VirtualTarget set_VirtualTarget[0..1],
    VirtualStore set_VirtualStore[1..2],
    VirtualSource set_VirtualSource[2..3],
    Reserved set_Reserved[3..32],
]}
pub type PKEY_SET_VIRTUALIZATION_INFORMATION = *mut KEY_SET_VIRTUALIZATION_INFORMATION;
ENUM!{enum KEY_VALUE_INFORMATION_CLASS {
    KeyValueBasicInformation = 0,
    KeyValueFullInformation = 1,
    KeyValuePartialInformation = 2,
    KeyValueFullInformationAlign64 = 3,
    KeyValuePartialInformationAlign64 = 4,
    KeyValueLayerInformation = 5,
    MaxKeyValueInfoClass = 6,
}}
STRUCT!{struct KEY_VALUE_BASIC_INFORMATION {
    TitleIndex: ULONG,
    Type: ULONG,
    NameLength: ULONG,
    Name: [WCHAR; 1],
}}
pub type PKEY_VALUE_BASIC_INFORMATION = *mut KEY_VALUE_BASIC_INFORMATION;
STRUCT!{struct KEY_VALUE_FULL_INFORMATION {
    TitleIndex: ULONG,
    Type: ULONG,
    DataOffset: ULONG,
    DataLength: ULONG,
    NameLength: ULONG,
    Name: [WCHAR; 1],
}}
pub type PKEY_VALUE_FULL_INFORMATION = *mut KEY_VALUE_FULL_INFORMATION;
STRUCT!{struct KEY_VALUE_PARTIAL_INFORMATION {
    TitleIndex: ULONG,
    Type: ULONG,
    DataLength: ULONG,
    Data: [UCHAR; 1],
}}
pub type PKEY_VALUE_PARTIAL_INFORMATION = *mut KEY_VALUE_PARTIAL_INFORMATION;
STRUCT!{struct KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    Type: ULONG,
    DataLength: ULONG,
    Data: [UCHAR; 1],
}}
pub type PKEY_VALUE_PARTIAL_INFORMATION_ALIGN64 = *mut KEY_VALUE_PARTIAL_INFORMATION_ALIGN64;
STRUCT!{struct KEY_VALUE_LAYER_INFORMATION {
    IsTombstone: ULONG,
    Reserved: ULONG,
}}
pub type PKEY_VALUE_LAYER_INFORMATION = *mut KEY_VALUE_LAYER_INFORMATION;
STRUCT!{struct KEY_VALUE_ENTRY {
    ValueName: PUNICODE_STRING,
    DataLength: ULONG,
    DataOffset: ULONG,
    Type: ULONG,
}}
pub type PKEY_VALUE_ENTRY = *mut KEY_VALUE_ENTRY;
ENUM!{enum REG_ACTION {
    KeyAdded = 0,
    KeyRemoved = 1,
    KeyModified = 2,
}}
STRUCT!{struct REG_NOTIFY_INFORMATION {
    NextEntryOffset: ULONG,
    Action: REG_ACTION,
    KeyLength: ULONG,
    Key: [WCHAR; 1],
}}
pub type PREG_NOTIFY_INFORMATION = *mut REG_NOTIFY_INFORMATION;
STRUCT!{struct KEY_PID_ARRAY {
    PID: HANDLE,
    KeyName: UNICODE_STRING,
}}
pub type PKEY_PID_ARRAY = *mut KEY_PID_ARRAY;
STRUCT!{struct KEY_OPEN_SUBKEYS_INFORMATION {
    Count: ULONG,
    KeyArray: [KEY_PID_ARRAY; 1],
}}
pub type PKEY_OPEN_SUBKEYS_INFORMATION = *mut KEY_OPEN_SUBKEYS_INFORMATION;
EXTERN!{extern "system" {
    fn NtCreateKey(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TitleIndex: ULONG,
        Class: PUNICODE_STRING,
        CreateOptions: ULONG,
        Disposition: PULONG,
    ) -> NTSTATUS;
    fn NtCreateKeyTransacted(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TitleIndex: ULONG,
        Class: PUNICODE_STRING,
        CreateOptions: ULONG,
        TransactionHandle: HANDLE,
        Disposition: PULONG,
    ) -> NTSTATUS;
    fn NtOpenKey(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtOpenKeyTransacted(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtOpenKeyEx(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        OpenOptions: ULONG,
    ) -> NTSTATUS;
    fn NtOpenKeyTransactedEx(
        KeyHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        OpenOptions: ULONG,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtDeleteKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtRenameKey(
        KeyHandle: HANDLE,
        NewName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn NtDeleteValueKey(
        KeyHandle: HANDLE,
        ValueName: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn NtQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationKey(
        KeyHandle: HANDLE,
        KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
        KeySetInformation: PVOID,
        KeySetInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: PUNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetValueKey(
        KeyHandle: HANDLE,
        ValueName: PUNICODE_STRING,
        TitleIndex: ULONG,
        Type: ULONG,
        Data: PVOID,
        DataSize: ULONG,
    ) -> NTSTATUS;
    fn NtQueryMultipleValueKey(
        KeyHandle: HANDLE,
        ValueEntries: PKEY_VALUE_ENTRY,
        EntryCount: ULONG,
        ValueBuffer: PVOID,
        BufferLength: PULONG,
        RequiredBufferLength: PULONG,
    ) -> NTSTATUS;
    fn NtEnumerateKey(
        KeyHandle: HANDLE,
        Index: ULONG,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn NtEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: ULONG,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: PVOID,
        Length: ULONG,
        ResultLength: PULONG,
    ) -> NTSTATUS;
    fn NtFlushKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCompactKeys(
        Count: ULONG,
        KeyArray: *mut HANDLE,
    ) -> NTSTATUS;
    fn NtCompressKey(
        Key: HANDLE,
    ) -> NTSTATUS;
    fn NtLoadKey(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtLoadKey2(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtLoadKeyEx(
        TargetKey: POBJECT_ATTRIBUTES,
        SourceFile: POBJECT_ATTRIBUTES,
        Flags: ULONG,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: ACCESS_MASK,
        RootHandle: PHANDLE,
        IoStatus: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;
    fn NtReplaceKey(
        NewFile: POBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtSaveKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtSaveKeyEx(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Format: ULONG,
    ) -> NTSTATUS;
    fn NtSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtRestoreKey(
        KeyHandle: HANDLE,
        FileHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtUnloadKey(
        TargetKey: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}}
pub const REG_FORCE_UNLOAD: ULONG = 1;
pub const REG_UNLOAD_LEGAL_FLAGS: ULONG = REG_FORCE_UNLOAD;
EXTERN!{extern "system" {
    fn NtUnloadKey2(
        TargetKey: POBJECT_ATTRIBUTES,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtUnloadKeyEx(
        TargetKey: POBJECT_ATTRIBUTES,
        Event: HANDLE,
    ) -> NTSTATUS;
    fn NtNotifyChangeKey(
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
    fn NtNotifyChangeMultipleKeys(
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
    fn NtQueryOpenSubKeys(
        TargetKey: POBJECT_ATTRIBUTES,
        HandleCount: PULONG,
    ) -> NTSTATUS;
    fn NtQueryOpenSubKeysEx(
        TargetKey: POBJECT_ATTRIBUTES,
        BufferLength: ULONG,
        Buffer: PVOID,
        RequiredSize: PULONG,
    ) -> NTSTATUS;
    fn NtInitializeRegistry(
        BootCondition: USHORT,
    ) -> NTSTATUS;
    fn NtLockRegistryKey(
        KeyHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtLockProductActivationKeys(
        pPrivateVer: *mut ULONG,
        pSafeMode: *mut ULONG,
    ) -> NTSTATUS;
    fn NtFreezeRegistry(
        TimeOutInSeconds: ULONG,
    ) -> NTSTATUS;
    fn NtThawRegistry() -> NTSTATUS;
}}
