use winapi::shared::ntdef::{
    BOOLEAN, CHAR, HANDLE, LARGE_INTEGER, LONG, NTSTATUS, PHANDLE, PLARGE_INTEGER,
    POBJECT_ATTRIBUTES, PULONG, PUNICODE_STRING, PVOID, UCHAR, ULONG, UNICODE_STRING, WAIT_TYPE,
};
use winapi::um::winnt::{
    ACCESS_MASK, GENERIC_MAPPING, PSECURITY_DESCRIPTOR, SECURITY_INFORMATION,
    STANDARD_RIGHTS_REQUIRED,
};
pub const OBJECT_TYPE_CREATE: u32 = 0x0001;
pub const OBJECT_TYPE_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | 0x1;
pub const DIRECTORY_QUERY: u32 = 0x0001;
pub const DIRECTORY_TRAVERSE: u32 = 0x0002;
pub const DIRECTORY_CREATE_OBJECT: u32 = 0x0004;
pub const DIRECTORY_CREATE_SUBDIRECTORY: u32 = 0x0008;
pub const DIRECTORY_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | 0xf;
pub const SYMBOLIC_LINK_QUERY: u32 = 0x0001;
pub const SYMBOLIC_LINK_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | 0x1;
pub const OBJ_PROTECT_CLOSE: u32 = 0x00000001;
pub const OBJ_INHERIT: u32 = 0x00000002;
pub const OBJ_AUDIT_OBJECT_CLOSE: u32 = 0x00000004;
ENUM!{enum OBJECT_INFORMATION_CLASS {
    ObjectBasicInformation = 0,
    ObjectNameInformation = 1,
    ObjectTypeInformation = 2,
    ObjectTypesInformation = 3,
    ObjectHandleFlagInformation = 4,
    ObjectSessionInformation = 5,
    ObjectSessionObjectInformation = 6,
    MaxObjectInfoClass = 7,
}}
STRUCT!{struct OBJECT_BASIC_INFORMATION {
    Attributes: ULONG,
    GrantedAccess: ACCESS_MASK,
    HandleCount: ULONG,
    PointerCount: ULONG,
    PagedPoolCharge: ULONG,
    NonPagedPoolCharge: ULONG,
    Reserved: [ULONG; 3],
    NameInfoSize: ULONG,
    TypeInfoSize: ULONG,
    SecurityDescriptorSize: ULONG,
    CreationTime: LARGE_INTEGER,
}}
pub type POBJECT_BASIC_INFORMATION = *mut OBJECT_BASIC_INFORMATION;
STRUCT!{struct OBJECT_NAME_INFORMATION {
    Name: UNICODE_STRING,
}}
pub type POBJECT_NAME_INFORMATION = *mut OBJECT_NAME_INFORMATION;
STRUCT!{struct OBJECT_TYPE_INFORMATION {
    TypeName: UNICODE_STRING,
    TotalNumberOfObjects: ULONG,
    TotalNumberOfHandles: ULONG,
    TotalPagedPoolUsage: ULONG,
    TotalNonPagedPoolUsage: ULONG,
    TotalNamePoolUsage: ULONG,
    TotalHandleTableUsage: ULONG,
    HighWaterNumberOfObjects: ULONG,
    HighWaterNumberOfHandles: ULONG,
    HighWaterPagedPoolUsage: ULONG,
    HighWaterNonPagedPoolUsage: ULONG,
    HighWaterNamePoolUsage: ULONG,
    HighWaterHandleTableUsage: ULONG,
    InvalidAttributes: ULONG,
    GenericMapping: GENERIC_MAPPING,
    ValidAccessMask: ULONG,
    SecurityRequired: BOOLEAN,
    MaintainHandleCount: BOOLEAN,
    TypeIndex: UCHAR,
    ReservedByte: CHAR,
    PoolType: ULONG,
    DefaultPagedPoolCharge: ULONG,
    DefaultNonPagedPoolCharge: ULONG,
}}
pub type POBJECT_TYPE_INFORMATION = *mut OBJECT_TYPE_INFORMATION;
STRUCT!{struct OBJECT_TYPES_INFORMATION {
    NumberOfTypes: ULONG,
}}
pub type POBJECT_TYPES_INFORMATION = *mut OBJECT_TYPES_INFORMATION;
STRUCT!{struct OBJECT_HANDLE_FLAG_INFORMATION {
    Inherit: BOOLEAN,
    ProtectFromClose: BOOLEAN,
}}
pub type POBJECT_HANDLE_FLAG_INFORMATION = *mut OBJECT_HANDLE_FLAG_INFORMATION;
EXTERN!{extern "system" {
    fn NtQueryObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: PVOID,
        ObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: PVOID,
        ObjectInformationLength: ULONG,
    ) -> NTSTATUS;
}}
pub const DUPLICATE_CLOSE_SOURCE: u32 = 0x00000001;
pub const DUPLICATE_SAME_ACCESS: u32 = 0x00000002;
pub const DUPLICATE_SAME_ATTRIBUTES: u32 = 0x00000004;
EXTERN!{extern "system" {
    fn NtDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Options: ULONG,
    ) -> NTSTATUS;
    fn NtMakeTemporaryObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn NtMakePermanentObject(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn NtSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForSingleObject(
        Handle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForMultipleObjects(
        Count: ULONG,
        Handles: *mut HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtWaitForMultipleObjects32(
        Count: ULONG,
        Handles: *mut LONG,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtSetSecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn NtQuerySecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Length: ULONG,
        LengthNeeded: PULONG,
    ) -> NTSTATUS;
    fn NtClose(
        Handle: HANDLE,
    ) -> NTSTATUS;
    fn NtCompareObjects(
        FirstObjectHandle: HANDLE,
        SecondObjectHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCreateDirectoryObject(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtCreateDirectoryObjectEx(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;
    fn NtOpenDirectoryObject(
        DirectoryHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}}
STRUCT!{struct OBJECT_DIRECTORY_INFORMATION {
    Name: UNICODE_STRING,
    TypeName: UNICODE_STRING,
}}
pub type POBJECT_DIRECTORY_INFORMATION = *mut OBJECT_DIRECTORY_INFORMATION;
EXTERN!{extern "system" {
    fn NtQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        RestartScan: BOOLEAN,
        Context: PULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtCreatePrivateNamespace(
        NamespaceHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        BoundaryDescriptor: PVOID,
    ) -> NTSTATUS;
    fn NtOpenPrivateNamespace(
        NamespaceHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        BoundaryDescriptor: PVOID,
    ) -> NTSTATUS;
    fn NtDeletePrivateNamespace(
        NamespaceHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtCreateSymbolicLinkObject(
        LinkHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LinkTarget: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn NtOpenSymbolicLinkObject(
        LinkHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: PUNICODE_STRING,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;
}}
