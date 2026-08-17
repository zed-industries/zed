use winapi::shared::basetsd::{PLONG64, PULONG64, ULONG64};
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, LONG, NTSTATUS, PBOOLEAN, PHANDLE, PLARGE_INTEGER, PLUID, PNTSTATUS,
    POBJECT_ATTRIBUTES, PUCHAR, PULONG, PUNICODE_STRING, PVOID, ULONG, UNICODE_STRING, USHORT,
};
use winapi::um::winnt::{
    ACCESS_MASK, AUDIT_EVENT_TYPE, PACCESS_MASK, PGENERIC_MAPPING, POBJECT_TYPE_LIST,
    PPRIVILEGE_SET, PSECURITY_DESCRIPTOR, PSE_SIGNING_LEVEL, PSID, PSID_AND_ATTRIBUTES,
    PTOKEN_DEFAULT_DACL, PTOKEN_GROUPS, PTOKEN_MANDATORY_POLICY, PTOKEN_OWNER,
    PTOKEN_PRIMARY_GROUP, PTOKEN_PRIVILEGES, PTOKEN_SOURCE, PTOKEN_USER, SE_SIGNING_LEVEL,
    TOKEN_INFORMATION_CLASS, TOKEN_TYPE,
};
pub const SE_MIN_WELL_KNOWN_PRIVILEGE: LONG = 2;
pub const SE_CREATE_TOKEN_PRIVILEGE: LONG = 2;
pub const SE_ASSIGNPRIMARYTOKEN_PRIVILEGE: LONG = 3;
pub const SE_LOCK_MEMORY_PRIVILEGE: LONG = 4;
pub const SE_INCREASE_QUOTA_PRIVILEGE: LONG = 5;
pub const SE_MACHINE_ACCOUNT_PRIVILEGE: LONG = 6;
pub const SE_TCB_PRIVILEGE: LONG = 7;
pub const SE_SECURITY_PRIVILEGE: LONG = 8;
pub const SE_TAKE_OWNERSHIP_PRIVILEGE: LONG = 9;
pub const SE_LOAD_DRIVER_PRIVILEGE: LONG = 10;
pub const SE_SYSTEM_PROFILE_PRIVILEGE: LONG = 11;
pub const SE_SYSTEMTIME_PRIVILEGE: LONG = 12;
pub const SE_PROF_SINGLE_PROCESS_PRIVILEGE: LONG = 13;
pub const SE_INC_BASE_PRIORITY_PRIVILEGE: LONG = 14;
pub const SE_CREATE_PAGEFILE_PRIVILEGE: LONG = 15;
pub const SE_CREATE_PERMANENT_PRIVILEGE: LONG = 16;
pub const SE_BACKUP_PRIVILEGE: LONG = 17;
pub const SE_RESTORE_PRIVILEGE: LONG = 18;
pub const SE_SHUTDOWN_PRIVILEGE: LONG = 19;
pub const SE_DEBUG_PRIVILEGE: LONG = 20;
pub const SE_AUDIT_PRIVILEGE: LONG = 21;
pub const SE_SYSTEM_ENVIRONMENT_PRIVILEGE: LONG = 22;
pub const SE_CHANGE_NOTIFY_PRIVILEGE: LONG = 23;
pub const SE_REMOTE_SHUTDOWN_PRIVILEGE: LONG = 24;
pub const SE_UNDOCK_PRIVILEGE: LONG = 25;
pub const SE_SYNC_AGENT_PRIVILEGE: LONG = 26;
pub const SE_ENABLE_DELEGATION_PRIVILEGE: LONG = 27;
pub const SE_MANAGE_VOLUME_PRIVILEGE: LONG = 28;
pub const SE_IMPERSONATE_PRIVILEGE: LONG = 29;
pub const SE_CREATE_GLOBAL_PRIVILEGE: LONG = 30;
pub const SE_TRUSTED_CREDMAN_ACCESS_PRIVILEGE: LONG = 31;
pub const SE_RELABEL_PRIVILEGE: LONG = 32;
pub const SE_INC_WORKING_SET_PRIVILEGE: LONG = 33;
pub const SE_TIME_ZONE_PRIVILEGE: LONG = 34;
pub const SE_CREATE_SYMBOLIC_LINK_PRIVILEGE: LONG = 35;
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE: LONG = 36;
pub const SE_MAX_WELL_KNOWN_PRIVILEGE: LONG = SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INVALID: USHORT = 0x00;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INT64: USHORT = 0x01;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_UINT64: USHORT = 0x02;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_STRING: USHORT = 0x03;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_FQBN: USHORT = 0x04;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_SID: USHORT = 0x05;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: USHORT = 0x06;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: USHORT = 0x10;
pub const TOKEN_SECURITY_ATTRIBUTE_NON_INHERITABLE: USHORT = 0x0001;
pub const TOKEN_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: USHORT = 0x0002;
pub const TOKEN_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: USHORT = 0x0004;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: USHORT = 0x0008;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED: USHORT = 0x0010;
pub const TOKEN_SECURITY_ATTRIBUTE_MANDATORY: USHORT = 0x0020;
pub const TOKEN_SECURITY_ATTRIBUTE_COMPARE_IGNORE: USHORT = 0x0040;
pub const TOKEN_SECURITY_ATTRIBUTE_VALID_FLAGS: USHORT = TOKEN_SECURITY_ATTRIBUTE_NON_INHERITABLE
    | TOKEN_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE | TOKEN_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY
    | TOKEN_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT | TOKEN_SECURITY_ATTRIBUTE_DISABLED
    | TOKEN_SECURITY_ATTRIBUTE_MANDATORY;
pub const TOKEN_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 0xffff0000;
STRUCT!{struct TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE {
    Version: ULONG64,
    Name: UNICODE_STRING,
}}
pub type PTOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE = *mut TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE;
STRUCT!{struct TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pValue: PVOID,
    ValueLength: ULONG,
}}
pub type PTOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE =
    *mut TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;
UNION!{union TOKEN_SECURITY_ATTRIBUTE_V1_Values {
    pInt64: PLONG64,
    pUint64: PULONG64,
    pString: PUNICODE_STRING,
    pFqbn: PTOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pOctetString: PTOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}}
STRUCT!{struct TOKEN_SECURITY_ATTRIBUTE_V1 {
    Name: UNICODE_STRING,
    ValueType: USHORT,
    Reserved: USHORT,
    Flags: ULONG,
    ValueCount: ULONG,
    Values: TOKEN_SECURITY_ATTRIBUTE_V1_Values,
}}
pub type PTOKEN_SECURITY_ATTRIBUTE_V1 = *mut TOKEN_SECURITY_ATTRIBUTE_V1;
pub const TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: USHORT = 1;
pub const TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION: USHORT =
    TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1;
STRUCT!{struct TOKEN_SECURITY_ATTRIBUTES_INFORMATION {
    Version: USHORT,
    Reserved: USHORT,
    AttributeCount: ULONG,
    pAttributeV1: PTOKEN_SECURITY_ATTRIBUTE_V1,
}}
pub type PTOKEN_SECURITY_ATTRIBUTES_INFORMATION = *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION;
STRUCT!{struct TOKEN_PROCESS_TRUST_LEVEL {
    TrustLevelSid: PSID,
}}
pub type PTOKEN_PROCESS_TRUST_LEVEL = *mut TOKEN_PROCESS_TRUST_LEVEL;
EXTERN!{extern "system" {
    fn NtCreateToken(
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
    fn NtCreateLowBoxToken(
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
    fn NtCreateTokenEx(
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
    fn NtOpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtOpenProcessTokenEx(
        ProcessHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtOpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        OpenAsSelf: BOOLEAN,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtOpenThreadTokenEx(
        ThreadHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        OpenAsSelf: BOOLEAN,
        HandleAttributes: ULONG,
        TokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtDuplicateToken(
        ExistingTokenHandle: HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        EffectiveOnly: BOOLEAN,
        TokenType: TOKEN_TYPE,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtQueryInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: PVOID,
        TokenInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: PVOID,
        TokenInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtAdjustPrivilegesToken(
        TokenHandle: HANDLE,
        DisableAllPrivileges: BOOLEAN,
        NewState: PTOKEN_PRIVILEGES,
        BufferLength: ULONG,
        PreviousState: PTOKEN_PRIVILEGES,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtAdjustGroupsToken(
        TokenHandle: HANDLE,
        ResetToDefault: BOOLEAN,
        NewState: PTOKEN_GROUPS,
        BufferLength: ULONG,
        PreviousState: PTOKEN_GROUPS,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtAdjustTokenClaimsAndDeviceGroups(
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
    fn NtFilterToken(
        ExistingTokenHandle: HANDLE,
        Flags: ULONG,
        SidsToDisable: PTOKEN_GROUPS,
        PrivilegesToDelete: PTOKEN_PRIVILEGES,
        RestrictedSids: PTOKEN_GROUPS,
        NewTokenHandle: PHANDLE,
    ) -> NTSTATUS;
    fn NtFilterTokenEx(
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
    fn NtCompareTokens(
        FirstTokenHandle: HANDLE,
        SecondTokenHandle: HANDLE,
        Equal: PBOOLEAN,
    ) -> NTSTATUS;
    fn NtPrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: PPRIVILEGE_SET,
        Result: PBOOLEAN,
    ) -> NTSTATUS;
    fn NtImpersonateAnonymousToken(
        ThreadHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQuerySecurityAttributesToken(
        TokenHandle: HANDLE,
        Attributes: PUNICODE_STRING,
        NumberOfAttributes: ULONG,
        Buffer: PVOID,
        Length: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtAccessCheck(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: PULONG,
        GrantedAccess: PACCESS_MASK,
        AccessStatus: PNTSTATUS,
    ) -> NTSTATUS;
    fn NtAccessCheckByType(
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
    fn NtAccessCheckByTypeResultList(
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
    fn NtSetCachedSigningLevel(
        Flags: ULONG,
        InputSigningLevel: SE_SIGNING_LEVEL,
        SourceFiles: PHANDLE,
        SourceFileCount: ULONG,
        TargetFile: HANDLE,
    ) -> NTSTATUS;
    fn NtGetCachedSigningLevel(
        File: HANDLE,
        Flags: PULONG,
        SigningLevel: PSE_SIGNING_LEVEL,
        Thumbprint: PUCHAR,
        ThumbprintSize: PULONG,
        ThumbprintAlgorithm: PULONG,
    ) -> NTSTATUS;
    fn NtAccessCheckAndAuditAlarm(
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
    fn NtAccessCheckByTypeAndAuditAlarm(
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
    fn NtAccessCheckByTypeResultListAndAuditAlarm(
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
    fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(
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
    fn NtOpenObjectAuditAlarm(
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
    fn NtPrivilegeObjectAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        ClientToken: HANDLE,
        DesiredAccess: ACCESS_MASK,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;
    fn NtCloseObjectAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;
    fn NtDeleteObjectAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        HandleId: PVOID,
        GenerateOnClose: BOOLEAN,
    ) -> NTSTATUS;
    fn NtPrivilegedServiceAuditAlarm(
        SubsystemName: PUNICODE_STRING,
        ServiceName: PUNICODE_STRING,
        ClientToken: HANDLE,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOLEAN,
    ) -> NTSTATUS;
}}
