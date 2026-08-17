use crate::string::UTF8Const;
use winapi::shared::basetsd::ULONG64;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, LARGE_INTEGER, NTSTATUS, OEM_STRING, PLARGE_INTEGER, POBJECT_ATTRIBUTES,
    PUCHAR, PULONG, PUNICODE_STRING, PVOID, PWSTR, ULONG, UNICODE_STRING, USHORT,
};
use winapi::um::ntsecapi::PDOMAIN_PASSWORD_INFORMATION;
use winapi::um::subauth::LOGON_HOURS;
use winapi::um::winnt::{
    ACCESS_MASK, PSECURITY_DESCRIPTOR, PSID, PSID_NAME_USE, SECURITY_INFORMATION, SID_NAME_USE,
    STANDARD_RIGHTS_EXECUTE, STANDARD_RIGHTS_READ, STANDARD_RIGHTS_REQUIRED, STANDARD_RIGHTS_WRITE,
};
pub const SAM_MAXIMUM_LOOKUP_COUNT: u32 = 1000;
pub const SAM_MAXIMUM_LOOKUP_LENGTH: u32 = 32000;
pub const SAM_MAX_PASSWORD_LENGTH: u32 = 256;
pub const SAM_PASSWORD_ENCRYPTION_SALT_LEN: u32 = 16;
pub type PSAM_HANDLE = *mut PVOID;
pub type SAM_HANDLE = PVOID;
pub type SAM_ENUMERATE_HANDLE = ULONG;
pub type PSAM_ENUMERATE_HANDLE = *mut ULONG;
STRUCT!{struct SAM_RID_ENUMERATION {
    RelativeId: ULONG,
    Name: UNICODE_STRING,
}}
pub type PSAM_RID_ENUMERATION = *mut SAM_RID_ENUMERATION;
STRUCT!{struct SAM_SID_ENUMERATION {
    Sid: PSID,
    Name: UNICODE_STRING,
}}
pub type PSAM_SID_ENUMERATION = *mut SAM_SID_ENUMERATION;
STRUCT!{struct SAM_BYTE_ARRAY {
    Size: ULONG,
    Data: PUCHAR,
}}
pub type PSAM_BYTE_ARRAY = *mut SAM_BYTE_ARRAY;
STRUCT!{struct SAM_BYTE_ARRAY_32K {
    Size: ULONG,
    Data: PUCHAR,
}}
pub type PSAM_BYTE_ARRAY_32K = *mut SAM_BYTE_ARRAY_32K;
pub type PSAM_SHELL_OBJECT_PROPERTIES = *mut SAM_BYTE_ARRAY_32K;
pub type SAM_SHELL_OBJECT_PROPERTIES = SAM_BYTE_ARRAY_32K;
EXTERN!{extern "system" {
    fn SamFreeMemory(
        Buffer: PVOID,
    ) -> NTSTATUS;
    fn SamCloseHandle(
        SamHandle: SAM_HANDLE,
    ) -> NTSTATUS;
    fn SamSetSecurityObject(
        ObjectHandle: SAM_HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn SamQuerySecurityObject(
        ObjectHandle: SAM_HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    fn SamRidToSid(
        ObjectHandle: SAM_HANDLE,
        Rid: ULONG,
        Sid: *mut PSID,
    ) -> NTSTATUS;
}}
pub const SAM_SERVER_CONNECT: ACCESS_MASK = 0x0001;
pub const SAM_SERVER_SHUTDOWN: ACCESS_MASK = 0x0002;
pub const SAM_SERVER_INITIALIZE: ACCESS_MASK = 0x0004;
pub const SAM_SERVER_CREATE_DOMAIN: ACCESS_MASK = 0x0008;
pub const SAM_SERVER_ENUMERATE_DOMAINS: ACCESS_MASK = 0x0010;
pub const SAM_SERVER_LOOKUP_DOMAIN: ACCESS_MASK = 0x0020;
pub const SAM_SERVER_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | SAM_SERVER_CONNECT
    | SAM_SERVER_INITIALIZE | SAM_SERVER_CREATE_DOMAIN | SAM_SERVER_SHUTDOWN
    | SAM_SERVER_ENUMERATE_DOMAINS | SAM_SERVER_LOOKUP_DOMAIN;
pub const SAM_SERVER_READ: ACCESS_MASK = STANDARD_RIGHTS_READ | SAM_SERVER_ENUMERATE_DOMAINS;
pub const SAM_SERVER_WRITE: ACCESS_MASK =
    STANDARD_RIGHTS_WRITE | SAM_SERVER_INITIALIZE | SAM_SERVER_CREATE_DOMAIN | SAM_SERVER_SHUTDOWN;
pub const SAM_SERVER_EXECUTE: ACCESS_MASK =
    STANDARD_RIGHTS_EXECUTE | SAM_SERVER_CONNECT | SAM_SERVER_LOOKUP_DOMAIN;
EXTERN!{extern "system" {
    fn SamConnect(
        ServerName: PUNICODE_STRING,
        ServerHandle: PSAM_HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn SamShutdownSamServer(
        ServerHandle: SAM_HANDLE,
    ) -> NTSTATUS;
}}
pub const DOMAIN_READ_PASSWORD_PARAMETERS: u32 = 0x0001;
pub const DOMAIN_WRITE_PASSWORD_PARAMS: u32 = 0x0002;
pub const DOMAIN_READ_OTHER_PARAMETERS: u32 = 0x0004;
pub const DOMAIN_WRITE_OTHER_PARAMETERS: u32 = 0x0008;
pub const DOMAIN_CREATE_USER: u32 = 0x0010;
pub const DOMAIN_CREATE_GROUP: u32 = 0x0020;
pub const DOMAIN_CREATE_ALIAS: u32 = 0x0040;
pub const DOMAIN_GET_ALIAS_MEMBERSHIP: u32 = 0x0080;
pub const DOMAIN_LIST_ACCOUNTS: u32 = 0x0100;
pub const DOMAIN_LOOKUP: u32 = 0x0200;
pub const DOMAIN_ADMINISTER_SERVER: u32 = 0x0400;
pub const DOMAIN_ALL_ACCESS: u32 = STANDARD_RIGHTS_REQUIRED | DOMAIN_READ_OTHER_PARAMETERS
    | DOMAIN_WRITE_OTHER_PARAMETERS | DOMAIN_WRITE_PASSWORD_PARAMS | DOMAIN_CREATE_USER
    | DOMAIN_CREATE_GROUP | DOMAIN_CREATE_ALIAS | DOMAIN_GET_ALIAS_MEMBERSHIP
    | DOMAIN_LIST_ACCOUNTS | DOMAIN_READ_PASSWORD_PARAMETERS | DOMAIN_LOOKUP
    | DOMAIN_ADMINISTER_SERVER;
pub const DOMAIN_READ: u32 =
    STANDARD_RIGHTS_READ | DOMAIN_GET_ALIAS_MEMBERSHIP | DOMAIN_READ_OTHER_PARAMETERS;
pub const DOMAIN_WRITE: u32 = STANDARD_RIGHTS_WRITE | DOMAIN_WRITE_OTHER_PARAMETERS
    | DOMAIN_WRITE_PASSWORD_PARAMS | DOMAIN_CREATE_USER | DOMAIN_CREATE_GROUP | DOMAIN_CREATE_ALIAS
    | DOMAIN_ADMINISTER_SERVER;
pub const DOMAIN_EXECUTE: u32 = STANDARD_RIGHTS_EXECUTE | DOMAIN_READ_PASSWORD_PARAMETERS
    | DOMAIN_LIST_ACCOUNTS | DOMAIN_LOOKUP;
ENUM!{enum DOMAIN_INFORMATION_CLASS {
    DomainPasswordInformation = 1,
    DomainGeneralInformation = 2,
    DomainLogoffInformation = 3,
    DomainOemInformation = 4,
    DomainNameInformation = 5,
    DomainReplicationInformation = 6,
    DomainServerRoleInformation = 7,
    DomainModifiedInformation = 8,
    DomainStateInformation = 9,
    DomainUasInformation = 10,
    DomainGeneralInformation2 = 11,
    DomainLockoutInformation = 12,
    DomainModifiedInformation2 = 13,
}}
ENUM!{enum DOMAIN_SERVER_ENABLE_STATE {
    DomainServerEnabled = 1,
    DomainServerDisabled = 2,
}}
pub type PDOMAIN_SERVER_ENABLE_STATE = *mut DOMAIN_SERVER_ENABLE_STATE;
ENUM!{enum DOMAIN_SERVER_ROLE {
    DomainServerRoleBackup = 2,
    DomainServerRolePrimary = 3,
}}
pub type PDOMAIN_SERVER_ROLE = *mut DOMAIN_SERVER_ROLE;
STRUCT!{#[repr(packed(4))] struct DOMAIN_GENERAL_INFORMATION {
    ForceLogoff: LARGE_INTEGER,
    OemInformation: UNICODE_STRING,
    DomainName: UNICODE_STRING,
    ReplicaSourceNodeName: UNICODE_STRING,
    DomainModifiedCount: LARGE_INTEGER,
    DomainServerState: DOMAIN_SERVER_ENABLE_STATE,
    DomainServerRole: DOMAIN_SERVER_ROLE,
    UasCompatibilityRequired: BOOLEAN,
    UserCount: ULONG,
    GroupCount: ULONG,
    AliasCount: ULONG,
}}
pub type PDOMAIN_GENERAL_INFORMATION = *mut DOMAIN_GENERAL_INFORMATION;
STRUCT!{#[repr(packed(4))] struct DOMAIN_GENERAL_INFORMATION2 {
    I1: DOMAIN_GENERAL_INFORMATION,
    LockoutDuration: LARGE_INTEGER,
    LockoutObservationWindow: LARGE_INTEGER,
    LockoutThreshold: USHORT,
}}
pub type PDOMAIN_GENERAL_INFORMATION2 = *mut DOMAIN_GENERAL_INFORMATION2;
STRUCT!{struct DOMAIN_UAS_INFORMATION {
    UasCompatibilityRequired: BOOLEAN,
}}
ENUM!{enum DOMAIN_PASSWORD_CONSTRUCTION {
    DomainPasswordSimple = 1,
    DomainPasswordComplex = 2,
}}
STRUCT!{struct DOMAIN_LOGOFF_INFORMATION {
    ForceLogoff: LARGE_INTEGER,
}}
pub type PDOMAIN_LOGOFF_INFORMATION = *mut DOMAIN_LOGOFF_INFORMATION;
STRUCT!{struct DOMAIN_OEM_INFORMATION {
    OemInformation: UNICODE_STRING,
}}
pub type PDOMAIN_OEM_INFORMATION = *mut DOMAIN_OEM_INFORMATION;
STRUCT!{struct DOMAIN_NAME_INFORMATION {
    DomainName: UNICODE_STRING,
}}
pub type PDOMAIN_NAME_INFORMATION = *mut DOMAIN_NAME_INFORMATION;
STRUCT!{struct DOMAIN_SERVER_ROLE_INFORMATION {
    DomainServerRole: DOMAIN_SERVER_ROLE,
}}
pub type PDOMAIN_SERVER_ROLE_INFORMATION = *mut DOMAIN_SERVER_ROLE_INFORMATION;
STRUCT!{struct DOMAIN_REPLICATION_INFORMATION {
    ReplicaSourceNodeName: UNICODE_STRING,
}}
pub type PDOMAIN_REPLICATION_INFORMATION = *mut DOMAIN_REPLICATION_INFORMATION;
STRUCT!{struct DOMAIN_MODIFIED_INFORMATION {
    DomainModifiedCount: LARGE_INTEGER,
    CreationTime: LARGE_INTEGER,
}}
pub type PDOMAIN_MODIFIED_INFORMATION = *mut DOMAIN_MODIFIED_INFORMATION;
STRUCT!{struct DOMAIN_MODIFIED_INFORMATION2 {
    DomainModifiedCount: LARGE_INTEGER,
    CreationTime: LARGE_INTEGER,
    ModifiedCountAtLastPromotion: LARGE_INTEGER,
}}
pub type PDOMAIN_MODIFIED_INFORMATION2 = *mut DOMAIN_MODIFIED_INFORMATION2;
STRUCT!{struct DOMAIN_STATE_INFORMATION {
    DomainServerState: DOMAIN_SERVER_ENABLE_STATE,
}}
pub type PDOMAIN_STATE_INFORMATION = *mut DOMAIN_STATE_INFORMATION;
STRUCT!{struct DOMAIN_LOCKOUT_INFORMATION {
    LockoutDuration: LARGE_INTEGER,
    LockoutObservationWindow: LARGE_INTEGER,
    LockoutThreshold: USHORT,
}}
pub type PDOMAIN_LOCKOUT_INFORMATION = *mut DOMAIN_LOCKOUT_INFORMATION;
ENUM!{enum DOMAIN_DISPLAY_INFORMATION {
    DomainDisplayUser = 1,
    DomainDisplayMachine = 2,
    DomainDisplayGroup = 3,
    DomainDisplayOemUser = 4,
    DomainDisplayOemGroup = 5,
    DomainDisplayServer = 6,
}}
pub type PDOMAIN_DISPLAY_INFORMATION = *mut DOMAIN_DISPLAY_INFORMATION;
STRUCT!{struct DOMAIN_DISPLAY_USER {
    Index: ULONG,
    Rid: ULONG,
    AccountControl: ULONG,
    LogonName: UNICODE_STRING,
    AdminComment: UNICODE_STRING,
    FullName: UNICODE_STRING,
}}
pub type PDOMAIN_DISPLAY_USER = *mut DOMAIN_DISPLAY_USER;
STRUCT!{struct DOMAIN_DISPLAY_MACHINE {
    Index: ULONG,
    Rid: ULONG,
    AccountControl: ULONG,
    Machine: UNICODE_STRING,
    Comment: UNICODE_STRING,
}}
pub type PDOMAIN_DISPLAY_MACHINE = *mut DOMAIN_DISPLAY_MACHINE;
STRUCT!{struct DOMAIN_DISPLAY_GROUP {
    Index: ULONG,
    Rid: ULONG,
    Attributes: ULONG,
    Group: UNICODE_STRING,
    Comment: UNICODE_STRING,
}}
pub type PDOMAIN_DISPLAY_GROUP = *mut DOMAIN_DISPLAY_GROUP;
STRUCT!{struct DOMAIN_DISPLAY_OEM_USER {
    Index: ULONG,
    User: OEM_STRING,
}}
pub type PDOMAIN_DISPLAY_OEM_USER = *mut DOMAIN_DISPLAY_OEM_USER;
STRUCT!{struct DOMAIN_DISPLAY_OEM_GROUP {
    Index: ULONG,
    Group: OEM_STRING,
}}
pub type PDOMAIN_DISPLAY_OEM_GROUP = *mut DOMAIN_DISPLAY_OEM_GROUP;
ENUM!{enum DOMAIN_LOCALIZABLE_ACCOUNTS_INFORMATION {
    DomainLocalizableAccountsBasic = 1,
}}
pub type PDOMAIN_LOCALIZABLE_ACCOUNTS_INFORMATION = *mut DOMAIN_LOCALIZABLE_ACCOUNTS_INFORMATION;
STRUCT!{struct DOMAIN_LOCALIZABLE_ACCOUNT_ENTRY {
    Rid: ULONG,
    Use: SID_NAME_USE,
    Name: UNICODE_STRING,
    AdminComment: UNICODE_STRING,
}}
pub type PDOMAIN_LOCALIZABLE_ACCOUNT_ENTRY = *mut DOMAIN_LOCALIZABLE_ACCOUNT_ENTRY;
STRUCT!{struct DOMAIN_LOCALIZABLE_ACCOUNTS_BASIC {
    Count: ULONG,
    Entries: *mut DOMAIN_LOCALIZABLE_ACCOUNT_ENTRY,
}}
pub type PDOMAIN_LOCALIZABLE_ACCOUNTS_BASIC = *mut DOMAIN_LOCALIZABLE_ACCOUNTS_BASIC;
UNION!{union DOMAIN_LOCALIZABLE_ACCOUNTS_INFO_BUFFER {
    Basic: DOMAIN_LOCALIZABLE_ACCOUNTS_BASIC,
}}
pub type PDOMAIN_LOCALIZABLE_ACCOUNTS_INFO_BUFFER = *mut DOMAIN_LOCALIZABLE_ACCOUNTS_INFO_BUFFER;
EXTERN!{extern "system" {
    fn SamLookupDomainInSamServer(
        ServerHandle: SAM_HANDLE,
        Name: PUNICODE_STRING,
        DomainId: *mut PSID,
    ) -> NTSTATUS;
    fn SamEnumerateDomainsInSamServer(
        ServerHandle: SAM_HANDLE,
        EnumerationContext: PSAM_ENUMERATE_HANDLE,
        Buffer: *mut PVOID,
        PreferedMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    fn SamOpenDomain(
        ServerHandle: SAM_HANDLE,
        DesiredAccess: ACCESS_MASK,
        DomainId: PSID,
        DomainHandle: PSAM_HANDLE,
    ) -> NTSTATUS;
    fn SamQueryInformationDomain(
        DomainHandle: SAM_HANDLE,
        DomainInformationClass: DOMAIN_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    fn SamSetInformationDomain(
        DomainHandle: SAM_HANDLE,
        DomainInformationClass: DOMAIN_INFORMATION_CLASS,
        DomainInformation: PVOID,
    ) -> NTSTATUS;
    fn SamLookupNamesInDomain(
        DomainHandle: SAM_HANDLE,
        Count: ULONG,
        Names: PUNICODE_STRING,
        RelativeIds: *mut PULONG,
        Use: *mut PSID_NAME_USE,
    ) -> NTSTATUS;
    fn SamLookupIdsInDomain(
        DomainHandle: SAM_HANDLE,
        Count: ULONG,
        RelativeIds: PULONG,
        Names: *mut PUNICODE_STRING,
        Use: *mut PSID_NAME_USE,
    ) -> NTSTATUS;
    fn SamRemoveMemberFromForeignDomain(
        DomainHandle: SAM_HANDLE,
        MemberId: PSID,
    ) -> NTSTATUS;
    fn SamQueryLocalizableAccountsInDomain(
        Domain: SAM_HANDLE,
        Flags: ULONG,
        LanguageId: ULONG,
        Class: DOMAIN_LOCALIZABLE_ACCOUNTS_INFORMATION,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
}}
pub const GROUP_READ_INFORMATION: ACCESS_MASK = 0x0001;
pub const GROUP_WRITE_ACCOUNT: ACCESS_MASK = 0x0002;
pub const GROUP_ADD_MEMBER: ACCESS_MASK = 0x0004;
pub const GROUP_REMOVE_MEMBER: ACCESS_MASK = 0x0008;
pub const GROUP_LIST_MEMBERS: ACCESS_MASK = 0x0010;
pub const GROUP_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | GROUP_LIST_MEMBERS
    | GROUP_WRITE_ACCOUNT | GROUP_ADD_MEMBER | GROUP_REMOVE_MEMBER | GROUP_READ_INFORMATION;
pub const GROUP_READ: ACCESS_MASK = STANDARD_RIGHTS_READ | GROUP_LIST_MEMBERS;
pub const GROUP_WRITE: ACCESS_MASK =
    STANDARD_RIGHTS_WRITE | GROUP_WRITE_ACCOUNT | GROUP_ADD_MEMBER | GROUP_REMOVE_MEMBER;
pub const GROUP_EXECUTE: ACCESS_MASK = STANDARD_RIGHTS_EXECUTE | GROUP_READ_INFORMATION;
STRUCT!{struct GROUP_MEMBERSHIP {
    RelativeId: ULONG,
    Attributes: ULONG,
}}
pub type PGROUP_MEMBERSHIP = *mut GROUP_MEMBERSHIP;
ENUM!{enum GROUP_INFORMATION_CLASS {
    GroupGeneralInformation = 1,
    GroupNameInformation = 2,
    GroupAttributeInformation = 3,
    GroupAdminCommentInformation = 4,
    GroupReplicationInformation = 5,
}}
STRUCT!{struct GROUP_GENERAL_INFORMATION {
    Name: UNICODE_STRING,
    Attributes: ULONG,
    MemberCount: ULONG,
    AdminComment: UNICODE_STRING,
}}
pub type PGROUP_GENERAL_INFORMATION = *mut GROUP_GENERAL_INFORMATION;
STRUCT!{struct GROUP_NAME_INFORMATION {
    Name: UNICODE_STRING,
}}
pub type PGROUP_NAME_INFORMATION = *mut GROUP_NAME_INFORMATION;
STRUCT!{struct GROUP_ATTRIBUTE_INFORMATION {
    Attributes: ULONG,
}}
pub type PGROUP_ATTRIBUTE_INFORMATION = *mut GROUP_ATTRIBUTE_INFORMATION;
STRUCT!{struct GROUP_ADM_COMMENT_INFORMATION {
    AdminComment: UNICODE_STRING,
}}
pub type PGROUP_ADM_COMMENT_INFORMATION = *mut GROUP_ADM_COMMENT_INFORMATION;
EXTERN!{extern "system" {
    fn SamEnumerateGroupsInDomain(
        DomainHandle: SAM_HANDLE,
        EnumerationContext: PSAM_ENUMERATE_HANDLE,
        Buffer: *mut PVOID,
        PreferedMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    fn SamCreateGroupInDomain(
        DomainHandle: SAM_HANDLE,
        AccountName: PUNICODE_STRING,
        DesiredAccess: ACCESS_MASK,
        GroupHandle: PSAM_HANDLE,
        RelativeId: PULONG,
    ) -> NTSTATUS;
    fn SamOpenGroup(
        DomainHandle: SAM_HANDLE,
        DesiredAccess: ACCESS_MASK,
        GroupId: ULONG,
        GroupHandle: PSAM_HANDLE,
    ) -> NTSTATUS;
    fn SamDeleteGroup(
        GroupHandle: SAM_HANDLE,
    ) -> NTSTATUS;
    fn SamQueryInformationGroup(
        GroupHandle: SAM_HANDLE,
        GroupInformationClass: GROUP_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    fn SamSetInformationGroup(
        GroupHandle: SAM_HANDLE,
        GroupInformationClass: GROUP_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    fn SamAddMemberToGroup(
        GroupHandle: SAM_HANDLE,
        MemberId: ULONG,
        Attributes: ULONG,
    ) -> NTSTATUS;
    fn SamRemoveMemberFromGroup(
        GroupHandle: SAM_HANDLE,
        MemberId: ULONG,
    ) -> NTSTATUS;
    fn SamGetMembersInGroup(
        GroupHandle: SAM_HANDLE,
        MemberIds: *mut PULONG,
        Attributes: *mut PULONG,
        MemberCount: PULONG,
    ) -> NTSTATUS;
    fn SamSetMemberAttributesOfGroup(
        GroupHandle: SAM_HANDLE,
        MemberId: ULONG,
        Attributes: ULONG,
    ) -> NTSTATUS;
}}
pub const ALIAS_ADD_MEMBER: ACCESS_MASK = 0x0001;
pub const ALIAS_REMOVE_MEMBER: ACCESS_MASK = 0x0002;
pub const ALIAS_LIST_MEMBERS: ACCESS_MASK = 0x0004;
pub const ALIAS_READ_INFORMATION: ACCESS_MASK = 0x0008;
pub const ALIAS_WRITE_ACCOUNT: ACCESS_MASK = 0x0010;
pub const ALIAS_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | ALIAS_READ_INFORMATION
    | ALIAS_WRITE_ACCOUNT | ALIAS_LIST_MEMBERS | ALIAS_ADD_MEMBER | ALIAS_REMOVE_MEMBER;
pub const ALIAS_READ: ACCESS_MASK = STANDARD_RIGHTS_READ | ALIAS_LIST_MEMBERS;
pub const ALIAS_WRITE: ACCESS_MASK =
    STANDARD_RIGHTS_WRITE | ALIAS_WRITE_ACCOUNT | ALIAS_ADD_MEMBER | ALIAS_REMOVE_MEMBER;
pub const ALIAS_EXECUTE: ACCESS_MASK = STANDARD_RIGHTS_EXECUTE | ALIAS_READ_INFORMATION;
ENUM!{enum ALIAS_INFORMATION_CLASS {
    AliasGeneralInformation = 1,
    AliasNameInformation = 2,
    AliasAdminCommentInformation = 3,
    AliasReplicationInformation = 4,
    AliasExtendedInformation = 5,
}}
STRUCT!{struct ALIAS_GENERAL_INFORMATION {
    Name: UNICODE_STRING,
    MemberCount: ULONG,
    AdminComment: UNICODE_STRING,
}}
pub type PALIAS_GENERAL_INFORMATION = *mut ALIAS_GENERAL_INFORMATION;
STRUCT!{struct ALIAS_NAME_INFORMATION {
    Name: UNICODE_STRING,
}}
pub type PALIAS_NAME_INFORMATION = *mut ALIAS_NAME_INFORMATION;
STRUCT!{struct ALIAS_ADM_COMMENT_INFORMATION {
    AdminComment: UNICODE_STRING,
}}
pub type PALIAS_ADM_COMMENT_INFORMATION = *mut ALIAS_ADM_COMMENT_INFORMATION;
pub const ALIAS_ALL_NAME: ULONG = 0x00000001;
pub const ALIAS_ALL_MEMBER_COUNT: ULONG = 0x00000002;
pub const ALIAS_ALL_ADMIN_COMMENT: ULONG = 0x00000004;
pub const ALIAS_ALL_SHELL_ADMIN_OBJECT_PROPERTIES: ULONG = 0x00000008;
STRUCT!{struct ALIAS_EXTENDED_INFORMATION {
    WhichFields: ULONG,
    ShellAdminObjectProperties: SAM_SHELL_OBJECT_PROPERTIES,
}}
pub type PALIAS_EXTENDED_INFORMATION = *mut ALIAS_EXTENDED_INFORMATION;
EXTERN!{extern "system" {
    fn SamEnumerateAliasesInDomain(
        DomainHandle: SAM_HANDLE,
        EnumerationContext: PSAM_ENUMERATE_HANDLE,
        Buffer: *mut PVOID,
        PreferedMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    fn SamCreateAliasInDomain(
        DomainHandle: SAM_HANDLE,
        AccountName: PUNICODE_STRING,
        DesiredAccess: ACCESS_MASK,
        AliasHandle: PSAM_HANDLE,
        RelativeId: PULONG,
    ) -> NTSTATUS;
    fn SamOpenAlias(
        DomainHandle: SAM_HANDLE,
        DesiredAccess: ACCESS_MASK,
        AliasId: ULONG,
        AliasHandle: PSAM_HANDLE,
    ) -> NTSTATUS;
    fn SamDeleteAlias(
        AliasHandle: SAM_HANDLE,
    ) -> NTSTATUS;
    fn SamQueryInformationAlias(
        AliasHandle: SAM_HANDLE,
        AliasInformationClass: ALIAS_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    fn SamSetInformationAlias(
        AliasHandle: SAM_HANDLE,
        AliasInformationClass: ALIAS_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    fn SamAddMemberToAlias(
        AliasHandle: SAM_HANDLE,
        MemberId: PSID,
    ) -> NTSTATUS;
    fn SamAddMultipleMembersToAlias(
        AliasHandle: SAM_HANDLE,
        MemberIds: *mut PSID,
        MemberCount: ULONG,
    ) -> NTSTATUS;
    fn SamRemoveMemberFromAlias(
        AliasHandle: SAM_HANDLE,
        MemberId: PSID,
    ) -> NTSTATUS;
    fn SamRemoveMultipleMembersFromAlias(
        AliasHandle: SAM_HANDLE,
        MemberIds: *mut PSID,
        MemberCount: ULONG,
    ) -> NTSTATUS;
    fn SamGetMembersInAlias(
        AliasHandle: SAM_HANDLE,
        MemberIds: *mut *mut PSID,
        MemberCount: PULONG,
    ) -> NTSTATUS;
    fn SamGetAliasMembership(
        DomainHandle: SAM_HANDLE,
        PassedCount: ULONG,
        Sids: *mut PSID,
        MembershipCount: PULONG,
        Aliases: *mut PULONG,
    ) -> NTSTATUS;
}}
pub const GROUP_TYPE_BUILTIN_LOCAL_GROUP: u32 = 0x00000001;
pub const GROUP_TYPE_ACCOUNT_GROUP: u32 = 0x00000002;
pub const GROUP_TYPE_RESOURCE_GROUP: u32 = 0x00000004;
pub const GROUP_TYPE_UNIVERSAL_GROUP: u32 = 0x00000008;
pub const GROUP_TYPE_APP_BASIC_GROUP: u32 = 0x00000010;
pub const GROUP_TYPE_APP_QUERY_GROUP: u32 = 0x00000020;
pub const GROUP_TYPE_SECURITY_ENABLED: u32 = 0x80000000;
pub const GROUP_TYPE_RESOURCE_BEHAVOIR: u32 =
    GROUP_TYPE_RESOURCE_GROUP | GROUP_TYPE_APP_BASIC_GROUP | GROUP_TYPE_APP_QUERY_GROUP;
pub const USER_READ_GENERAL: DWORD = 0x0001;
pub const USER_READ_PREFERENCES: DWORD = 0x0002;
pub const USER_WRITE_PREFERENCES: DWORD = 0x0004;
pub const USER_READ_LOGON: DWORD = 0x0008;
pub const USER_READ_ACCOUNT: DWORD = 0x0010;
pub const USER_WRITE_ACCOUNT: DWORD = 0x0020;
pub const USER_CHANGE_PASSWORD: DWORD = 0x0040;
pub const USER_FORCE_PASSWORD_CHANGE: DWORD = 0x0080;
pub const USER_LIST_GROUPS: DWORD = 0x0100;
pub const USER_READ_GROUP_INFORMATION: DWORD = 0x0200;
pub const USER_WRITE_GROUP_INFORMATION: DWORD = 0x0400;
pub const USER_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | USER_READ_PREFERENCES
    | USER_READ_LOGON | USER_LIST_GROUPS | USER_READ_GROUP_INFORMATION | USER_WRITE_PREFERENCES
    | USER_CHANGE_PASSWORD | USER_FORCE_PASSWORD_CHANGE | USER_READ_GENERAL | USER_READ_ACCOUNT
    | USER_WRITE_ACCOUNT | USER_WRITE_GROUP_INFORMATION;
pub const USER_READ: DWORD = STANDARD_RIGHTS_READ | USER_READ_PREFERENCES | USER_READ_LOGON
    | USER_READ_ACCOUNT | USER_LIST_GROUPS | USER_READ_GROUP_INFORMATION;
pub const USER_WRITE: DWORD =
    STANDARD_RIGHTS_WRITE | USER_WRITE_PREFERENCES | USER_CHANGE_PASSWORD;
pub const USER_EXECUTE: DWORD = STANDARD_RIGHTS_EXECUTE | USER_READ_GENERAL | USER_CHANGE_PASSWORD;
ENUM!{enum USER_INFORMATION_CLASS {
    UserGeneralInformation = 1,
    UserPreferencesInformation = 2,
    UserLogonInformation = 3,
    UserLogonHoursInformation = 4,
    UserAccountInformation = 5,
    UserNameInformation = 6,
    UserAccountNameInformation = 7,
    UserFullNameInformation = 8,
    UserPrimaryGroupInformation = 9,
    UserHomeInformation = 10,
    UserScriptInformation = 11,
    UserProfileInformation = 12,
    UserAdminCommentInformation = 13,
    UserWorkStationsInformation = 14,
    UserSetPasswordInformation = 15,
    UserControlInformation = 16,
    UserExpiresInformation = 17,
    UserInternal1Information = 18,
    UserInternal2Information = 19,
    UserParametersInformation = 20,
    UserAllInformation = 21,
    UserInternal3Information = 22,
    UserInternal4Information = 23,
    UserInternal5Information = 24,
    UserInternal4InformationNew = 25,
    UserInternal5InformationNew = 26,
    UserInternal6Information = 27,
    UserExtendedInformation = 28,
    UserLogonUIInformation = 29,
}}
pub type PUSER_INFORMATION_CLASS = *mut USER_INFORMATION_CLASS;
pub const USER_ALL_USERNAME: ULONG = 0x00000001;
pub const USER_ALL_FULLNAME: ULONG = 0x00000002;
pub const USER_ALL_USERID: ULONG = 0x00000004;
pub const USER_ALL_PRIMARYGROUPID: ULONG = 0x00000008;
pub const USER_ALL_ADMINCOMMENT: ULONG = 0x00000010;
pub const USER_ALL_USERCOMMENT: ULONG = 0x00000020;
pub const USER_ALL_HOMEDIRECTORY: ULONG = 0x00000040;
pub const USER_ALL_HOMEDIRECTORYDRIVE: ULONG = 0x00000080;
pub const USER_ALL_SCRIPTPATH: ULONG = 0x00000100;
pub const USER_ALL_PROFILEPATH: ULONG = 0x00000200;
pub const USER_ALL_WORKSTATIONS: ULONG = 0x00000400;
pub const USER_ALL_LASTLOGON: ULONG = 0x00000800;
pub const USER_ALL_LASTLOGOFF: ULONG = 0x00001000;
pub const USER_ALL_LOGONHOURS: ULONG = 0x00002000;
pub const USER_ALL_BADPASSWORDCOUNT: ULONG = 0x00004000;
pub const USER_ALL_LOGONCOUNT: ULONG = 0x00008000;
pub const USER_ALL_PASSWORDCANCHANGE: ULONG = 0x00010000;
pub const USER_ALL_PASSWORDMUSTCHANGE: ULONG = 0x00020000;
pub const USER_ALL_PASSWORDLASTSET: ULONG = 0x00040000;
pub const USER_ALL_ACCOUNTEXPIRES: ULONG = 0x00080000;
pub const USER_ALL_USERACCOUNTCONTROL: ULONG = 0x00100000;
pub const USER_ALL_PARAMETERS: ULONG = 0x00200000;
pub const USER_ALL_COUNTRYCODE: ULONG = 0x00400000;
pub const USER_ALL_CODEPAGE: ULONG = 0x00800000;
pub const USER_ALL_NTPASSWORDPRESENT: ULONG = 0x01000000;
pub const USER_ALL_LMPASSWORDPRESENT: ULONG = 0x02000000;
pub const USER_ALL_PRIVATEDATA: ULONG = 0x04000000;
pub const USER_ALL_PASSWORDEXPIRED: ULONG = 0x08000000;
pub const USER_ALL_SECURITYDESCRIPTOR: ULONG = 0x10000000;
pub const USER_ALL_OWFPASSWORD: ULONG = 0x20000000;
pub const USER_ALL_UNDEFINED_MASK: ULONG = 0xc0000000;
pub const USER_ALL_READ_GENERAL_MASK: ULONG = USER_ALL_USERNAME | USER_ALL_FULLNAME
    | USER_ALL_USERID | USER_ALL_PRIMARYGROUPID | USER_ALL_ADMINCOMMENT | USER_ALL_USERCOMMENT;
pub const USER_ALL_READ_LOGON_MASK: ULONG = USER_ALL_HOMEDIRECTORY | USER_ALL_HOMEDIRECTORYDRIVE
    | USER_ALL_SCRIPTPATH | USER_ALL_PROFILEPATH | USER_ALL_WORKSTATIONS | USER_ALL_LASTLOGON
    | USER_ALL_LASTLOGOFF | USER_ALL_LOGONHOURS | USER_ALL_BADPASSWORDCOUNT | USER_ALL_LOGONCOUNT
    | USER_ALL_PASSWORDCANCHANGE | USER_ALL_PASSWORDMUSTCHANGE;
pub const USER_ALL_READ_ACCOUNT_MASK: ULONG = USER_ALL_PASSWORDLASTSET | USER_ALL_ACCOUNTEXPIRES
    | USER_ALL_USERACCOUNTCONTROL | USER_ALL_PARAMETERS;
pub const USER_ALL_READ_PREFERENCES_MASK: ULONG = USER_ALL_COUNTRYCODE | USER_ALL_CODEPAGE;
pub const USER_ALL_READ_TRUSTED_MASK: ULONG = USER_ALL_NTPASSWORDPRESENT
    | USER_ALL_LMPASSWORDPRESENT | USER_ALL_PASSWORDEXPIRED | USER_ALL_SECURITYDESCRIPTOR
    | USER_ALL_PRIVATEDATA;
pub const USER_ALL_READ_CANT_MASK: ULONG = USER_ALL_UNDEFINED_MASK;
pub const USER_ALL_WRITE_ACCOUNT_MASK: ULONG = USER_ALL_USERNAME | USER_ALL_FULLNAME
    | USER_ALL_PRIMARYGROUPID | USER_ALL_HOMEDIRECTORY | USER_ALL_HOMEDIRECTORYDRIVE
    | USER_ALL_SCRIPTPATH | USER_ALL_PROFILEPATH | USER_ALL_ADMINCOMMENT | USER_ALL_WORKSTATIONS
    | USER_ALL_LOGONHOURS | USER_ALL_ACCOUNTEXPIRES | USER_ALL_USERACCOUNTCONTROL
    | USER_ALL_PARAMETERS;
pub const USER_ALL_WRITE_PREFERENCES_MASK: ULONG =
    USER_ALL_USERCOMMENT | USER_ALL_COUNTRYCODE | USER_ALL_CODEPAGE;
pub const USER_ALL_WRITE_FORCE_PASSWORD_CHANGE_MASK: ULONG =
    USER_ALL_NTPASSWORDPRESENT | USER_ALL_LMPASSWORDPRESENT | USER_ALL_PASSWORDEXPIRED;
pub const USER_ALL_WRITE_TRUSTED_MASK: ULONG = USER_ALL_LASTLOGON | USER_ALL_LASTLOGOFF
    | USER_ALL_BADPASSWORDCOUNT | USER_ALL_LOGONCOUNT | USER_ALL_PASSWORDLASTSET
    | USER_ALL_SECURITYDESCRIPTOR | USER_ALL_PRIVATEDATA;
pub const USER_ALL_WRITE_CANT_MASK: ULONG = USER_ALL_USERID | USER_ALL_PASSWORDCANCHANGE
    | USER_ALL_PASSWORDMUSTCHANGE | USER_ALL_UNDEFINED_MASK;
STRUCT!{struct USER_GENERAL_INFORMATION {
    UserName: UNICODE_STRING,
    FullName: UNICODE_STRING,
    PrimaryGroupId: ULONG,
    AdminComment: UNICODE_STRING,
    UserComment: UNICODE_STRING,
}}
pub type PUSER_GENERAL_INFORMATION = *mut USER_GENERAL_INFORMATION;
STRUCT!{struct USER_PREFERENCES_INFORMATION {
    UserComment: UNICODE_STRING,
    Reserved1: UNICODE_STRING,
    CountryCode: USHORT,
    CodePage: USHORT,
}}
pub type PUSER_PREFERENCES_INFORMATION = *mut USER_PREFERENCES_INFORMATION;
STRUCT!{struct USER_PARAMETERS_INFORMATION {
    Parameters: UNICODE_STRING,
}}
pub type PUSER_PARAMETERS_INFORMATION = *mut USER_PARAMETERS_INFORMATION;
STRUCT!{#[repr(packed(4))] struct USER_LOGON_INFORMATION {
    UserName: UNICODE_STRING,
    FullName: UNICODE_STRING,
    UserId: ULONG,
    PrimaryGroupId: ULONG,
    HomeDirectory: UNICODE_STRING,
    HomeDirectoryDrive: UNICODE_STRING,
    ScriptPath: UNICODE_STRING,
    ProfilePath: UNICODE_STRING,
    WorkStations: UNICODE_STRING,
    LastLogon: LARGE_INTEGER,
    LastLogoff: LARGE_INTEGER,
    PasswordLastSet: LARGE_INTEGER,
    PasswordCanChange: LARGE_INTEGER,
    PasswordMustChange: LARGE_INTEGER,
    LogonHours: LOGON_HOURS,
    BadPasswordCount: USHORT,
    LogonCount: USHORT,
    UserAccountControl: ULONG,
}}
pub type PUSER_LOGON_INFORMATION = *mut USER_LOGON_INFORMATION;
STRUCT!{#[repr(packed(4))] struct USER_ACCOUNT_INFORMATION {
    UserName: UNICODE_STRING,
    FullName: UNICODE_STRING,
    UserId: ULONG,
    PrimaryGroupId: ULONG,
    HomeDirectory: UNICODE_STRING,
    HomeDirectoryDrive: UNICODE_STRING,
    ScriptPath: UNICODE_STRING,
    ProfilePath: UNICODE_STRING,
    AdminComment: UNICODE_STRING,
    WorkStations: UNICODE_STRING,
    LastLogon: LARGE_INTEGER,
    LastLogoff: LARGE_INTEGER,
    LogonHours: LOGON_HOURS,
    BadPasswordCount: USHORT,
    LogonCount: USHORT,
    PasswordLastSet: LARGE_INTEGER,
    AccountExpires: LARGE_INTEGER,
    UserAccountControl: ULONG,
}}
pub type PUSER_ACCOUNT_INFORMATION = *mut USER_ACCOUNT_INFORMATION;
STRUCT!{struct USER_ACCOUNT_NAME_INFORMATION {
    UserName: UNICODE_STRING,
}}
pub type PUSER_ACCOUNT_NAME_INFORMATION = *mut USER_ACCOUNT_NAME_INFORMATION;
STRUCT!{struct USER_FULL_NAME_INFORMATION {
    FullName: UNICODE_STRING,
}}
pub type PUSER_FULL_NAME_INFORMATION = *mut USER_FULL_NAME_INFORMATION;
STRUCT!{struct USER_NAME_INFORMATION {
    UserName: UNICODE_STRING,
    FullName: UNICODE_STRING,
}}
pub type PUSER_NAME_INFORMATION = *mut USER_NAME_INFORMATION;
STRUCT!{struct USER_PRIMARY_GROUP_INFORMATION {
    PrimaryGroupId: ULONG,
}}
pub type PUSER_PRIMARY_GROUP_INFORMATION = *mut USER_PRIMARY_GROUP_INFORMATION;
STRUCT!{struct USER_HOME_INFORMATION {
    HomeDirectory: UNICODE_STRING,
    HomeDirectoryDrive: UNICODE_STRING,
}}
pub type PUSER_HOME_INFORMATION = *mut USER_HOME_INFORMATION;
STRUCT!{struct USER_SCRIPT_INFORMATION {
    ScriptPath: UNICODE_STRING,
}}
pub type PUSER_SCRIPT_INFORMATION = *mut USER_SCRIPT_INFORMATION;
STRUCT!{struct USER_PROFILE_INFORMATION {
    ProfilePath: UNICODE_STRING,
}}
pub type PUSER_PROFILE_INFORMATION = *mut USER_PROFILE_INFORMATION;
STRUCT!{struct USER_ADMIN_COMMENT_INFORMATION {
    AdminComment: UNICODE_STRING,
}}
pub type PUSER_ADMIN_COMMENT_INFORMATION = *mut USER_ADMIN_COMMENT_INFORMATION;
STRUCT!{struct USER_WORKSTATIONS_INFORMATION {
    WorkStations: UNICODE_STRING,
}}
pub type PUSER_WORKSTATIONS_INFORMATION = *mut USER_WORKSTATIONS_INFORMATION;
STRUCT!{struct USER_SET_PASSWORD_INFORMATION {
    Password: UNICODE_STRING,
    PasswordExpired: BOOLEAN,
}}
pub type PUSER_SET_PASSWORD_INFORMATION = *mut USER_SET_PASSWORD_INFORMATION;
STRUCT!{struct USER_CONTROL_INFORMATION {
    UserAccountControl: ULONG,
}}
pub type PUSER_CONTROL_INFORMATION = *mut USER_CONTROL_INFORMATION;
STRUCT!{struct USER_EXPIRES_INFORMATION {
    AccountExpires: LARGE_INTEGER,
}}
pub type PUSER_EXPIRES_INFORMATION = *mut USER_EXPIRES_INFORMATION;
STRUCT!{struct USER_LOGON_HOURS_INFORMATION {
    LogonHours: LOGON_HOURS,
}}
pub type PUSER_LOGON_HOURS_INFORMATION = *mut USER_LOGON_HOURS_INFORMATION;
pub type SAM_USER_TILE = SAM_BYTE_ARRAY_32K;
pub type PSAM_USER_TILE = *mut SAM_BYTE_ARRAY_32K;
pub const USER_EXTENDED_FIELD_USER_TILE: ULONG = 0x00001000;
pub const USER_EXTENDED_FIELD_PASSWORD_HINT: ULONG = 0x00002000;
pub const USER_EXTENDED_FIELD_DONT_SHOW_IN_LOGON_UI: ULONG = 0x00004000;
pub const USER_EXTENDED_FIELD_SHELL_ADMIN_OBJECT_PROPERTIES: ULONG = 0x00008000;
STRUCT!{struct USER_EXTENDED_INFORMATION {
    ExtendedWhichFields: ULONG,
    UserTile: SAM_USER_TILE,
    PasswordHint: UNICODE_STRING,
    DontShowInLogonUI: BOOLEAN,
    ShellAdminObjectProperties: SAM_SHELL_OBJECT_PROPERTIES,
}}
pub type PUSER_EXTENDED_INFORMATION = *mut USER_EXTENDED_INFORMATION;
STRUCT!{struct USER_LOGON_UI_INFORMATION {
    PasswordIsBlank: BOOLEAN,
    AccountIsDisabled: BOOLEAN,
}}
pub type PUSER_LOGON_UI_INFORMATION = *mut USER_LOGON_UI_INFORMATION;
STRUCT!{struct USER_PWD_CHANGE_FAILURE_INFORMATION {
    ExtendedFailureReason: ULONG,
    FilterModuleName: UNICODE_STRING,
}}
pub type PUSER_PWD_CHANGE_FAILURE_INFORMATION = *mut USER_PWD_CHANGE_FAILURE_INFORMATION;
pub const SAM_PWD_CHANGE_NO_ERROR: u32 = 0;
pub const SAM_PWD_CHANGE_PASSWORD_TOO_SHORT: u32 = 1;
pub const SAM_PWD_CHANGE_PWD_IN_HISTORY: u32 = 2;
pub const SAM_PWD_CHANGE_USERNAME_IN_PASSWORD: u32 = 3;
pub const SAM_PWD_CHANGE_FULLNAME_IN_PASSWORD: u32 = 4;
pub const SAM_PWD_CHANGE_NOT_COMPLEX: u32 = 5;
pub const SAM_PWD_CHANGE_MACHINE_PASSWORD_NOT_DEFAULT: u32 = 6;
pub const SAM_PWD_CHANGE_FAILED_BY_FILTER: u32 = 7;
pub const SAM_PWD_CHANGE_PASSWORD_TOO_LONG: u32 = 8;
pub const SAM_PWD_CHANGE_FAILURE_REASON_MAX: u32 = 8;
EXTERN!{extern "system" {
    fn SamEnumerateUsersInDomain(
        DomainHandle: SAM_HANDLE,
        EnumerationContext: PSAM_ENUMERATE_HANDLE,
        UserAccountControl: ULONG,
        Buffer: *mut PVOID,
        PreferedMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    fn SamCreateUserInDomain(
        DomainHandle: SAM_HANDLE,
        AccountName: PUNICODE_STRING,
        DesiredAccess: ACCESS_MASK,
        UserHandle: PSAM_HANDLE,
        RelativeId: PULONG,
    ) -> NTSTATUS;
    fn SamCreateUser2InDomain(
        DomainHandle: SAM_HANDLE,
        AccountName: PUNICODE_STRING,
        AccountType: ULONG,
        DesiredAccess: ACCESS_MASK,
        UserHandle: PSAM_HANDLE,
        GrantedAccess: PULONG,
        RelativeId: PULONG,
    ) -> NTSTATUS;
    fn SamOpenUser(
        DomainHandle: SAM_HANDLE,
        DesiredAccess: ACCESS_MASK,
        UserId: ULONG,
        UserHandle: PSAM_HANDLE,
    ) -> NTSTATUS;
    fn SamDeleteUser(
        UserHandle: SAM_HANDLE,
    ) -> NTSTATUS;
    fn SamQueryInformationUser(
        UserHandle: SAM_HANDLE,
        UserInformationClass: USER_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    fn SamSetInformationUser(
        UserHandle: SAM_HANDLE,
        UserInformationClass: USER_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    fn SamGetGroupsForUser(
        UserHandle: SAM_HANDLE,
        Groups: *mut PGROUP_MEMBERSHIP,
        MembershipCount: PULONG,
    ) -> NTSTATUS;
    fn SamChangePasswordUser(
        UserHandle: SAM_HANDLE,
        OldPassword: PUNICODE_STRING,
        NewPassword: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn SamChangePasswordUser2(
        ServerName: PUNICODE_STRING,
        UserName: PUNICODE_STRING,
        OldPassword: PUNICODE_STRING,
        NewPassword: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn SamChangePasswordUser3(
        ServerName: PUNICODE_STRING,
        UserName: PUNICODE_STRING,
        OldPassword: PUNICODE_STRING,
        NewPassword: PUNICODE_STRING,
        EffectivePasswordPolicy: *mut PDOMAIN_PASSWORD_INFORMATION,
        PasswordChangeFailureInfo: *mut PUSER_PWD_CHANGE_FAILURE_INFORMATION,
    ) -> NTSTATUS;
    fn SamQueryDisplayInformation(
        DomainHandle: SAM_HANDLE,
        DisplayInformation: DOMAIN_DISPLAY_INFORMATION,
        Index: ULONG,
        EntryCount: ULONG,
        PreferredMaximumLength: ULONG,
        TotalAvailable: PULONG,
        TotalReturned: PULONG,
        ReturnedEntryCount: PULONG,
        SortedBuffer: *mut PVOID,
    ) -> NTSTATUS;
    fn SamGetDisplayEnumerationIndex(
        DomainHandle: SAM_HANDLE,
        DisplayInformation: DOMAIN_DISPLAY_INFORMATION,
        Prefix: PUNICODE_STRING,
        Index: PULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum SECURITY_DB_DELTA_TYPE {
    SecurityDbNew = 1,
    SecurityDbRename = 2,
    SecurityDbDelete = 3,
    SecurityDbChangeMemberAdd = 4,
    SecurityDbChangeMemberSet = 5,
    SecurityDbChangeMemberDel = 6,
    SecurityDbChange = 7,
    SecurityDbChangePassword = 8,
}}
pub type PSECURITY_DB_DELTA_TYPE = *mut SECURITY_DB_DELTA_TYPE;
ENUM!{enum SECURITY_DB_OBJECT_TYPE {
    SecurityDbObjectSamDomain = 1,
    SecurityDbObjectSamUser = 2,
    SecurityDbObjectSamGroup = 3,
    SecurityDbObjectSamAlias = 4,
    SecurityDbObjectLsaPolicy = 5,
    SecurityDbObjectLsaTDomain = 6,
    SecurityDbObjectLsaAccount = 7,
    SecurityDbObjectLsaSecret = 8,
}}
pub type PSECURITY_DB_OBJECT_TYPE = *mut SECURITY_DB_OBJECT_TYPE;
ENUM!{enum SAM_ACCOUNT_TYPE {
    SamObjectUser = 1,
    SamObjectGroup = 2,
    SamObjectAlias = 3,
}}
pub type PSAM_ACCOUNT_TYPE = *mut SAM_ACCOUNT_TYPE;
pub const SAM_USER_ACCOUNT: u32 = 0x00000001;
pub const SAM_GLOBAL_GROUP_ACCOUNT: u32 = 0x00000002;
pub const SAM_LOCAL_GROUP_ACCOUNT: u32 = 0x00000004;
STRUCT!{struct SAM_GROUP_MEMBER_ID {
    MemberRid: ULONG,
}}
pub type PSAM_GROUP_MEMBER_ID = *mut SAM_GROUP_MEMBER_ID;
STRUCT!{struct SAM_ALIAS_MEMBER_ID {
    MemberSid: PSID,
}}
pub type PSAM_ALIAS_MEMBER_ID = *mut SAM_ALIAS_MEMBER_ID;
UNION!{union SAM_DELTA_DATA {
    GroupMemberId: SAM_GROUP_MEMBER_ID,
    AliasMemberId: SAM_ALIAS_MEMBER_ID,
    AccountControl: ULONG,
}}
pub type PSAM_DELTA_DATA = *mut SAM_DELTA_DATA;
FN!{stdcall PSAM_DELTA_NOTIFICATION_ROUTINE(
    DomainSid: PSID,
    DeltaType: SECURITY_DB_DELTA_TYPE,
    ObjectType: SECURITY_DB_OBJECT_TYPE,
    ObjectRid: ULONG,
    ObjectName: PUNICODE_STRING,
    ModifiedCount: PLARGE_INTEGER,
    DeltaData: PSAM_DELTA_DATA,
) -> NTSTATUS}
pub const SAM_DELTA_NOTIFY_ROUTINE: UTF8Const = UTF8Const("DeltaNotify\0");
EXTERN!{extern "system" {
    fn SamRegisterObjectChangeNotification(
        ObjectType: SECURITY_DB_OBJECT_TYPE,
        NotificationEventHandle: HANDLE,
    ) -> NTSTATUS;
    fn SamUnregisterObjectChangeNotification(
        ObjectType: SECURITY_DB_OBJECT_TYPE,
        NotificationEventHandle: HANDLE,
    ) -> NTSTATUS;
}}
pub const SAM_SID_COMPATIBILITY_ALL: u32 = 0;
pub const SAM_SID_COMPATIBILITY_LAX: u32 = 1;
pub const SAM_SID_COMPATIBILITY_STRICT: u32 = 2;
EXTERN!{extern "system" {
    fn SamGetCompatibilityMode(
        ObjectHandle: SAM_HANDLE,
        Mode: *mut ULONG,
    ) -> NTSTATUS;
}}
ENUM!{enum PASSWORD_POLICY_VALIDATION_TYPE {
    SamValidateAuthentication = 1,
    SamValidatePasswordChange = 2,
    SamValidatePasswordReset = 3,
}}
STRUCT!{struct SAM_VALIDATE_PASSWORD_HASH {
    Length: ULONG,
    Hash: PUCHAR,
}}
pub type PSAM_VALIDATE_PASSWORD_HASH = *mut SAM_VALIDATE_PASSWORD_HASH;
pub const SAM_VALIDATE_PASSWORD_LAST_SET: u32 = 0x00000001;
pub const SAM_VALIDATE_BAD_PASSWORD_TIME: u32 = 0x00000002;
pub const SAM_VALIDATE_LOCKOUT_TIME: u32 = 0x00000004;
pub const SAM_VALIDATE_BAD_PASSWORD_COUNT: u32 = 0x00000008;
pub const SAM_VALIDATE_PASSWORD_HISTORY_LENGTH: u32 = 0x00000010;
pub const SAM_VALIDATE_PASSWORD_HISTORY: u32 = 0x00000020;
STRUCT!{struct SAM_VALIDATE_PERSISTED_FIELDS {
    PresentFields: ULONG,
    PasswordLastSet: LARGE_INTEGER,
    BadPasswordTime: LARGE_INTEGER,
    LockoutTime: LARGE_INTEGER,
    BadPasswordCount: ULONG,
    PasswordHistoryLength: ULONG,
    PasswordHistory: PSAM_VALIDATE_PASSWORD_HASH,
}}
pub type PSAM_VALIDATE_PERSISTED_FIELDS = *mut SAM_VALIDATE_PERSISTED_FIELDS;
ENUM!{enum SAM_VALIDATE_VALIDATION_STATUS {
    SamValidateSuccess = 0,
    SamValidatePasswordMustChange = 1,
    SamValidateAccountLockedOut = 2,
    SamValidatePasswordExpired = 3,
    SamValidatePasswordIncorrect = 4,
    SamValidatePasswordIsInHistory = 5,
    SamValidatePasswordTooShort = 6,
    SamValidatePasswordTooLong = 7,
    SamValidatePasswordNotComplexEnough = 8,
    SamValidatePasswordTooRecent = 9,
    SamValidatePasswordFilterError = 10,
}}
pub type PSAM_VALIDATE_VALIDATION_STATUS = *mut SAM_VALIDATE_VALIDATION_STATUS;
STRUCT!{struct SAM_VALIDATE_STANDARD_OUTPUT_ARG {
    ChangedPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    ValidationStatus: SAM_VALIDATE_VALIDATION_STATUS,
}}
pub type PSAM_VALIDATE_STANDARD_OUTPUT_ARG = *mut SAM_VALIDATE_STANDARD_OUTPUT_ARG;
STRUCT!{struct SAM_VALIDATE_AUTHENTICATION_INPUT_ARG {
    InputPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    PasswordMatched: BOOLEAN,
}}
pub type PSAM_VALIDATE_AUTHENTICATION_INPUT_ARG = *mut SAM_VALIDATE_AUTHENTICATION_INPUT_ARG;
STRUCT!{struct SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    InputPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    ClearPassword: UNICODE_STRING,
    UserAccountName: UNICODE_STRING,
    HashedPassword: SAM_VALIDATE_PASSWORD_HASH,
    PasswordMatch: BOOLEAN,
}}
pub type PSAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG = *mut SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG;
STRUCT!{struct SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    InputPersistedFields: SAM_VALIDATE_PERSISTED_FIELDS,
    ClearPassword: UNICODE_STRING,
    UserAccountName: UNICODE_STRING,
    HashedPassword: SAM_VALIDATE_PASSWORD_HASH,
    PasswordMustChangeAtNextLogon: BOOLEAN,
    ClearLockout: BOOLEAN,
}}
pub type PSAM_VALIDATE_PASSWORD_RESET_INPUT_ARG = *mut SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG;
UNION!{union SAM_VALIDATE_INPUT_ARG {
    ValidateAuthenticationInput: SAM_VALIDATE_AUTHENTICATION_INPUT_ARG,
    ValidatePasswordChangeInput: SAM_VALIDATE_PASSWORD_CHANGE_INPUT_ARG,
    ValidatePasswordResetInput: SAM_VALIDATE_PASSWORD_RESET_INPUT_ARG,
}}
pub type PSAM_VALIDATE_INPUT_ARG = *mut SAM_VALIDATE_INPUT_ARG;
UNION!{union SAM_VALIDATE_OUTPUT_ARG {
    ValidateAuthenticationOutput: SAM_VALIDATE_STANDARD_OUTPUT_ARG,
    ValidatePasswordChangeOutput: SAM_VALIDATE_STANDARD_OUTPUT_ARG,
    ValidatePasswordResetOutput: SAM_VALIDATE_STANDARD_OUTPUT_ARG,
}}
pub type PSAM_VALIDATE_OUTPUT_ARG = *mut SAM_VALIDATE_OUTPUT_ARG;
EXTERN!{extern "system" {
    fn SamValidatePassword(
        ServerName: PUNICODE_STRING,
        ValidationType: PASSWORD_POLICY_VALIDATION_TYPE,
        InputArg: PSAM_VALIDATE_INPUT_ARG,
        OutputArg: *mut PSAM_VALIDATE_OUTPUT_ARG,
    ) -> NTSTATUS;
}}
ENUM!{enum SAM_GENERIC_OPERATION_TYPE {
    SamObjectChangeNotificationOperation = 0,
}}
pub type PSAM_GENERIC_OPERATION_TYPE = *mut SAM_GENERIC_OPERATION_TYPE;
STRUCT!{struct SAM_OPERATION_OBJCHG_INPUT {
    Register: BOOLEAN,
    EventHandle: ULONG64,
    ObjectType: SECURITY_DB_OBJECT_TYPE,
    ProcessID: ULONG,
}}
pub type PSAM_OPERATION_OBJCHG_INPUT = *mut SAM_OPERATION_OBJCHG_INPUT;
STRUCT!{struct SAM_OPERATION_OBJCHG_OUTPUT {
    Reserved: ULONG,
}}
pub type PSAM_OPERATION_OBJCHG_OUTPUT = *mut SAM_OPERATION_OBJCHG_OUTPUT;
UNION!{union SAM_GENERIC_OPERATION_INPUT {
    ObjChangeIn: SAM_OPERATION_OBJCHG_INPUT,
}}
pub type PSAM_GENERIC_OPERATION_INPUT = *mut SAM_GENERIC_OPERATION_INPUT;
UNION!{union SAM_GENERIC_OPERATION_OUTPUT {
    ObjChangeOut: SAM_OPERATION_OBJCHG_OUTPUT,
}}
pub type PSAM_GENERIC_OPERATION_OUTPUT = *mut SAM_GENERIC_OPERATION_OUTPUT;
EXTERN!{extern "system" {
    fn SamPerformGenericOperation(
        ServerName: PWSTR,
        OperationType: SAM_GENERIC_OPERATION_TYPE,
        OperationIn: PSAM_GENERIC_OPERATION_INPUT,
        OperationOut: *mut PSAM_GENERIC_OPERATION_OUTPUT,
    ) -> NTSTATUS;
}}
