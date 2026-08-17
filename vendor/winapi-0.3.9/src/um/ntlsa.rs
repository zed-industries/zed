// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{SIZE_T, ULONG_PTR};
use shared::guiddef::GUID;
use shared::minwindef::{PUCHAR, PULONG, UCHAR, ULONG, USHORT};
use shared::ntdef::{NTSTATUS, PNTSTATUS};
use um::lsalookup::{
    LSA_TRUST_INFORMATION, LSA_UNICODE_STRING, PLSA_OBJECT_ATTRIBUTES,
    PLSA_REFERENCED_DOMAIN_LIST, PLSA_STRING, PLSA_TRANSLATED_NAME, PLSA_TRANSLATED_SID2,
    PLSA_TRUST_INFORMATION, PLSA_UNICODE_STRING,
};
use um::ntsecapi::PLSA_HANDLE;
use um::subauth::{PUNICODE_STRING, UNICODE_STRING};
use um::winnt::{
    ACCESS_MASK, ANYSIZE_ARRAY, BOOLEAN, HANDLE, LARGE_INTEGER, LONG, LUID, PBOOLEAN,
    PCLAIMS_BLOB, PHANDLE, PLARGE_INTEGER, PLUID, PPRIVILEGE_SET, PQUOTA_LIMITS,
    PSECURITY_DESCRIPTOR, PSHORT, PSID, PTOKEN_GROUPS, PTOKEN_PRIVILEGES, PTOKEN_SOURCE, PVOID,
    PWSTR, QUOTA_LIMITS, SECURITY_INFORMATION, SID_NAME_USE, STANDARD_RIGHTS_EXECUTE,
    STANDARD_RIGHTS_READ, STANDARD_RIGHTS_REQUIRED, STANDARD_RIGHTS_WRITE, TOKEN_DEFAULT_DACL,
    TOKEN_DEVICE_CLAIMS, TOKEN_OWNER, TOKEN_PRIMARY_GROUP, TOKEN_USER, TOKEN_USER_CLAIMS,
};
pub type LSA_OPERATIONAL_MODE = ULONG;
pub type PLSA_OPERATIONAL_MODE = *mut LSA_OPERATIONAL_MODE;
pub const LSA_MODE_PASSWORD_PROTECTED: ULONG = 0x00000001;
pub const LSA_MODE_INDIVIDUAL_ACCOUNTS: ULONG = 0x00000002;
pub const LSA_MODE_MANDATORY_ACCESS: ULONG = 0x00000004;
pub const LSA_MODE_LOG_FULL: ULONG = 0x00000008;
pub const LSA_MAXIMUM_SID_COUNT: SIZE_T = 0x00000100;
pub const LSA_MAXIMUM_ENUMERATION_LENGTH: SIZE_T = 32000;
pub const LSA_CALL_LICENSE_SERVER: ULONG = 0x80000000;
ENUM!{enum SECURITY_LOGON_TYPE {
    UndefinedLogonType = 0,
    Interactive = 2,
    Network,
    Batch,
    Service,
    Proxy,
    Unlock,
    NetworkCleartext,
    NewCredentials,
    RemoteInteractive,
    CachedInteractive,
    CachedRemoteInteractive,
    CachedUnlock,
}}
pub type PSECURITY_LOGON_TYPE = *mut SECURITY_LOGON_TYPE;
pub const SECURITY_ACCESS_INTERACTIVE_LOGON: ULONG = 0x00000001;
pub const SECURITY_ACCESS_NETWORK_LOGON: ULONG = 0x00000002;
pub const SECURITY_ACCESS_BATCH_LOGON: ULONG = 0x00000004;
pub const SECURITY_ACCESS_SERVICE_LOGON: ULONG = 0x00000010;
pub const SECURITY_ACCESS_PROXY_LOGON: ULONG = 0x00000020;
pub const SECURITY_ACCESS_DENY_INTERACTIVE_LOGON: ULONG = 0x00000040;
pub const SECURITY_ACCESS_DENY_NETWORK_LOGON: ULONG = 0x00000080;
pub const SECURITY_ACCESS_DENY_BATCH_LOGON: ULONG = 0x00000100;
pub const SECURITY_ACCESS_DENY_SERVICE_LOGON: ULONG = 0x00000200;
pub const SECURITY_ACCESS_REMOTE_INTERACTIVE_LOGON: ULONG = 0x00000400;
pub const SECURITY_ACCESS_DENY_REMOTE_INTERACTIVE_LOGON: ULONG = 0x00000800;
ENUM!{enum SE_ADT_PARAMETER_TYPE {
    SeAdtParmTypeNone = 0,
    SeAdtParmTypeString,
    SeAdtParmTypeFileSpec,
    SeAdtParmTypeUlong,
    SeAdtParmTypeSid,
    SeAdtParmTypeLogonId,
    SeAdtParmTypeNoLogonId,
    SeAdtParmTypeAccessMask,
    SeAdtParmTypePrivs,
    SeAdtParmTypeObjectTypes,
    SeAdtParmTypeHexUlong,
    SeAdtParmTypePtr,
    SeAdtParmTypeTime,
    SeAdtParmTypeGuid,
    SeAdtParmTypeLuid,
    SeAdtParmTypeHexInt64,
    SeAdtParmTypeStringList,
    SeAdtParmTypeSidList,
    SeAdtParmTypeDuration,
    SeAdtParmTypeUserAccountControl,
    SeAdtParmTypeNoUac,
    SeAdtParmTypeMessage,
    SeAdtParmTypeDateTime,
    SeAdtParmTypeSockAddr,
    SeAdtParmTypeSD,
    SeAdtParmTypeLogonHours,
    SeAdtParmTypeLogonIdNoSid,
    SeAdtParmTypeUlongNoConv,
    SeAdtParmTypeSockAddrNoPort,
    SeAdtParmTypeAccessReason,
    SeAdtParmTypeStagingReason,
    SeAdtParmTypeResourceAttribute,
    SeAdtParmTypeClaims,
    SeAdtParmTypeLogonIdAsSid,
    SeAdtParmTypeMultiSzString,
    SeAdtParmTypeLogonIdEx,
}}
pub type PSE_ADT_PARAMETER_TYPE = *mut SE_ADT_PARAMETER_TYPE;
pub const SE_ADT_OBJECT_ONLY: USHORT = 0x1;
STRUCT!{struct SE_ADT_OBJECT_TYPE {
    ObjectType: GUID,
    Flags: USHORT,
    Level: USHORT,
    AccessMask: ACCESS_MASK,
}}
pub type PSE_ADT_OBJECT_TYPE = *mut SE_ADT_OBJECT_TYPE;
STRUCT!{struct SE_ADT_PARAMETER_ARRAY_ENTRY {
    Type: SE_ADT_PARAMETER_TYPE,
    Length: ULONG,
    Data: [ULONG_PTR; 2],
    Address: PVOID,
}}
pub type PSE_ADT_PARAMETER_ARRAY_ENTRY = *mut SE_ADT_PARAMETER_ARRAY_ENTRY;
STRUCT!{struct SE_ADT_ACCESS_REASON {
    AccessMask: ACCESS_MASK,
    AccessReasons: [ULONG; 32],
    ObjectTypeIndex: ULONG,
    AccessGranted: ULONG,
    SecurityDescriptor: PSECURITY_DESCRIPTOR,
}}
pub type PSE_ADT_ACCESS_REASON = *mut SE_ADT_ACCESS_REASON;
STRUCT!{struct SE_ADT_CLAIMS {
    Length: ULONG,
    Claims: PCLAIMS_BLOB,
}}
pub type PSE_ADT_CLAIMS = *mut SE_ADT_CLAIMS;
pub const SE_MAX_AUDIT_PARAMETERS: SIZE_T = 32;
pub const SE_MAX_GENERIC_AUDIT_PARAMETERS: SIZE_T = 28;
STRUCT!{struct SE_ADT_PARAMETER_ARRAY {
    CategoryId: ULONG,
    AuditId: ULONG,
    ParameterCount: ULONG,
    Length: ULONG,
    FlatSubCategoryId: USHORT,
    Type: USHORT,
    Flags: ULONG,
    Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; SE_MAX_AUDIT_PARAMETERS],
}}
pub type PSE_ADT_PARAMETER_ARRAY = *mut SE_ADT_PARAMETER_ARRAY;
STRUCT!{struct SE_ADT_PARAMETER_ARRAY_EX {
    CategoryId: ULONG,
    AuditId: ULONG,
    Version: ULONG,
    ParameterCount: ULONG,
    Length: ULONG,
    FlatSubCategoryId: USHORT,
    Type: USHORT,
    Flags: ULONG,
    Parameters: [SE_ADT_PARAMETER_ARRAY_ENTRY; SE_MAX_AUDIT_PARAMETERS],
}}
pub type PSE_ADT_PARAMETER_ARRAY_EX = *mut SE_ADT_PARAMETER_ARRAY_EX;
pub const SE_ADT_PARAMETERS_SELF_RELATIVE: ULONG = 0x00000001;
pub const SE_ADT_PARAMETERS_SEND_TO_LSA: ULONG = 0x00000002;
pub const SE_ADT_PARAMETER_EXTENSIBLE_AUDIT: ULONG = 0x00000004;
pub const SE_ADT_PARAMETER_GENERIC_AUDIT: ULONG = 0x00000008;
pub const SE_ADT_PARAMETER_WRITE_SYNCHRONOUS: ULONG = 0x00000010;
#[cfg(target_pointer_width = "32")]
#[inline]
pub fn LSAP_SE_ADT_PARAMETER_ARRAY_TRUE_SIZE(
    AuditParameters: SE_ADT_PARAMETER_ARRAY,
) -> SIZE_T {
    664  // FIXME: sizeof::<SE_ADT_PARAMETER_ARRAY>()
        - (20 // FIXME: sizeof::<SE_ADT_PARAMETER_ARRAY_ENTRY>()
        * (SE_MAX_AUDIT_PARAMETERS - AuditParameters.ParameterCount as SIZE_T))
}
#[cfg(target_pointer_width = "64")]
#[inline]
pub fn LSAP_SE_ADT_PARAMETER_ARRAY_TRUE_SIZE(
    AuditParameters: SE_ADT_PARAMETER_ARRAY,
) -> SIZE_T {
    1048  // FIXME: sizeof::<SE_ADT_PARAMETER_ARRAY>()
        - (32 // FIXME: sizeof::<SE_ADT_PARAMETER_ARRAY_ENTRY>()
        * (SE_MAX_AUDIT_PARAMETERS - AuditParameters.ParameterCount as SIZE_T))
}
STRUCT!{struct LSA_ADT_STRING_LIST_ENTRY {
    Flags: ULONG,
    String: UNICODE_STRING,
}}
pub type PLSA_ADT_STRING_LIST_ENTRY = *mut LSA_ADT_STRING_LIST_ENTRY;
STRUCT!{struct LSA_ADT_STRING_LIST {
    cStrings: ULONG,
    String: PLSA_ADT_STRING_LIST_ENTRY,
}}
pub type PLSA_ADT_STRING_LIST = *mut LSA_ADT_STRING_LIST;
STRUCT!{struct LSA_ADT_SID_LIST_ENTRY {
    Flags: ULONG,
    Sid: PSID,
}}
pub type PLSA_ADT_SID_LIST_ENTRY = *mut LSA_ADT_SID_LIST_ENTRY;
STRUCT!{struct LSA_ADT_SID_LIST {
    cSids: ULONG,
    Sid: PLSA_ADT_SID_LIST_ENTRY,
}}
pub type PLSA_ADT_SID_LIST = *mut LSA_ADT_SID_LIST;
pub const LSA_ADT_SECURITY_SOURCE_NAME: &'static str = "Microsoft-Windows-Security-Auditing";
pub const LSA_ADT_LEGACY_SECURITY_SOURCE_NAME: &'static str = "Security";
pub const SE_ADT_POLICY_AUDIT_EVENT_TYPE_EX_BEGIN: ULONG = 100;
ENUM!{enum POLICY_AUDIT_EVENT_TYPE_EX {
    iSystem_SecurityStateChange = SE_ADT_POLICY_AUDIT_EVENT_TYPE_EX_BEGIN,
    iSystem_SecuritySubsystemExtension,
    iSystem_Integrity,
    iSystem_IPSecDriverEvents,
    iSystem_Others,
    iLogon_Logon,
    iLogon_Logoff,
    iLogon_AccountLockout,
    iLogon_IPSecMainMode,
    iLogon_SpecialLogon,
    iLogon_IPSecQuickMode,
    iLogon_IPSecUsermode,
    iLogon_Others,
    iLogon_NPS,
    iLogon_Claims,
    iLogon_Groups,
    iObjectAccess_FileSystem,
    iObjectAccess_Registry,
    iObjectAccess_Kernel,
    iObjectAccess_Sam,
    iObjectAccess_Other,
    iObjectAccess_CertificationAuthority,
    iObjectAccess_ApplicationGenerated,
    iObjectAccess_HandleBasedAudits,
    iObjectAccess_Share,
    iObjectAccess_FirewallPacketDrops,
    iObjectAccess_FirewallConnection,
    iObjectAccess_DetailedFileShare,
    iObjectAccess_RemovableStorage,
    iObjectAccess_CbacStaging,
    iPrivilegeUse_Sensitive,
    iPrivilegeUse_NonSensitive,
    iPrivilegeUse_Others,
    iDetailedTracking_ProcessCreation,
    iDetailedTracking_ProcessTermination,
    iDetailedTracking_DpapiActivity,
    iDetailedTracking_RpcCall,
    iDetailedTracking_PnpActivity,
    iDetailedTracking_TokenRightAdjusted,
    iPolicyChange_AuditPolicy,
    iPolicyChange_AuthenticationPolicy,
    iPolicyChange_AuthorizationPolicy,
    iPolicyChange_MpsscvRulePolicy,
    iPolicyChange_WfpIPSecPolicy,
    iPolicyChange_Others,
    iAccountManagement_UserAccount,
    iAccountManagement_ComputerAccount,
    iAccountManagement_SecurityGroup,
    iAccountManagement_DistributionGroup,
    iAccountManagement_ApplicationGroup,
    iAccountManagement_Others,
    iDSAccess_DSAccess,
    iDSAccess_AdAuditChanges,
    iDS_Replication,
    iDS_DetailedReplication,
    iAccountLogon_CredentialValidation,
    iAccountLogon_Kerberos,
    iAccountLogon_Others,
    iAccountLogon_KerbCredentialValidation,
    iUnknownSubCategory = 999,
}}
pub type PPOLICY_AUDIT_EVENT_TYPE_EX = *mut POLICY_AUDIT_EVENT_TYPE_EX;
ENUM!{enum POLICY_AUDIT_EVENT_TYPE {
    AuditCategorySystem = 0,
    AuditCategoryLogon,
    AuditCategoryObjectAccess,
    AuditCategoryPrivilegeUse,
    AuditCategoryDetailedTracking,
    AuditCategoryPolicyChange,
    AuditCategoryAccountManagement,
    AuditCategoryDirectoryServiceAccess,
    AuditCategoryAccountLogon,
}}
pub type PPOLICY_AUDIT_EVENT_TYPE = *mut POLICY_AUDIT_EVENT_TYPE;
pub const POLICY_AUDIT_EVENT_UNCHANGED: ULONG = 0x00000000;
pub const POLICY_AUDIT_EVENT_SUCCESS: ULONG = 0x00000001;
pub const POLICY_AUDIT_EVENT_FAILURE: ULONG = 0x00000002;
pub const POLICY_AUDIT_EVENT_NONE: ULONG = 0x00000004;
pub const POLICY_AUDIT_EVENT_MASK: ULONG = POLICY_AUDIT_EVENT_SUCCESS | POLICY_AUDIT_EVENT_FAILURE
    | POLICY_AUDIT_EVENT_UNCHANGED | POLICY_AUDIT_EVENT_NONE;
#[inline]
pub fn LSA_SUCCESS(Error: NTSTATUS) -> bool {
    (Error as LONG) >= 0
}
extern "system" {
    pub fn LsaRegisterLogonProcess(
        LogonProcessName: PLSA_STRING,
        LsaHandle: PHANDLE,
        SecurityMode: PLSA_OPERATIONAL_MODE,
    ) -> NTSTATUS;
    pub fn LsaLogonUser(
        LsaHandle: HANDLE,
        OriginName: PLSA_STRING,
        LogonType: SECURITY_LOGON_TYPE,
        AuthenticationPackage: ULONG,
        AuthenticationInformation: PVOID,
        AuthenticationInformationLength: ULONG,
        LocalGroups: PTOKEN_GROUPS,
        SourceContext: PTOKEN_SOURCE,
        ProfileBuffer: *mut PVOID,
        ProfileBufferLength: PULONG,
        LogonId: PLUID,
        Token: PHANDLE,
        Quotas: PQUOTA_LIMITS,
        SubStatus: PNTSTATUS,
    ) -> NTSTATUS;
    pub fn LsaLookupAuthenticationPackage(
        LsaHandle: HANDLE,
        PackageName: PLSA_STRING,
        AuthenticationPackage: PULONG,
    ) -> NTSTATUS;
    pub fn LsaFreeReturnBuffer(
        Buffer: PVOID,
    ) -> NTSTATUS;
    pub fn LsaCallAuthenticationPackage(
        LsaHandle: HANDLE,
        AuthenticationPackage: ULONG,
        ProtocolSubmitBuffer: PVOID,
        SubmitBufferLength: ULONG,
        ProtocolReturnBuffer: *mut PVOID,
        ReturnBufferLength: PULONG,
        ProtocolStatus: PNTSTATUS,
    ) -> NTSTATUS;
    pub fn LsaDeregisterLogonProcess(
        LsaHandle: HANDLE,
    ) -> NTSTATUS;
    pub fn LsaConnectUntrusted(
        LsaHandle: PHANDLE,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn LsaInsertProtectedProcessAddress(
        BufferAddress: PVOID,
        BufferSize: ULONG,
    ) -> NTSTATUS;
    pub fn LsaRemoveProtectedProcessAddress(
        BufferAddress: PVOID,
        BufferSize: ULONG,
    ) -> NTSTATUS;
}
FN!{stdcall PFN_LSA_CALL_AUTH_PKG(
    LsaHandle: HANDLE,
    AuthenticationPackage: ULONG,
    ProtocolSubmitBuffer: PVOID,
    SubmitBufferLength: ULONG,
    ProtocolReturnBuffer: *mut PVOID,
    ReturnBufferLength: PULONG,
    ProtocolStatus: PNTSTATUS,
) -> NTSTATUS}
FN!{stdcall PFN_LSA_DEREGISTER_PROC(
    LsaHandle: HANDLE,
) -> NTSTATUS}
FN!{stdcall PFN_LSA_FREE_BUFFER(
    Buffer: PVOID,
) -> NTSTATUS}
FN!{stdcall PFN_LSA_LOGON_USER(
    LsaHandle: HANDLE,
    OriginName: PLSA_STRING,
    LogonType: SECURITY_LOGON_TYPE,
    AuthenticationPackage: ULONG,
    AuthenticationInformation: PVOID,
    AuthenticationInformationLength: ULONG,
    LocalGroups: PTOKEN_GROUPS,
    SourceContext: PTOKEN_SOURCE,
    ProfileBuffer: *mut PVOID,
    ProfileBufferLength: PULONG,
    LogonId: PLUID,
    Token: PHANDLE,
    Quotas: PQUOTA_LIMITS,
    SubStatus: PNTSTATUS,
) -> NTSTATUS}
FN!{stdcall PFN_LOOKUP_AUTH_PKG(
    LsaHandle: HANDLE,
    PackageName: PLSA_STRING,
    AuthenticationPackage: PULONG,
) -> NTSTATUS}
FN!{stdcall PFN_LSA_REGISTER_PROC(
    LogonProcessName: PLSA_STRING,
    LsaHandle: PHANDLE,
    SecurityMode: PLSA_OPERATIONAL_MODE,
) -> NTSTATUS}
STRUCT!{struct LSA_AUTH_CALLBACKS {
    LsaCallAuthPkgFn: PFN_LSA_CALL_AUTH_PKG,
    LsaDeregisterProcFn: PFN_LSA_DEREGISTER_PROC,
    LsaFreeReturnBufferFn: PFN_LSA_FREE_BUFFER,
    LsaLogonUserFn: PFN_LSA_LOGON_USER,
    LsaLookupAuthPkgFn: PFN_LOOKUP_AUTH_PKG,
    LsaRegisterProcFn: PFN_LSA_REGISTER_PROC,
}}
pub type PLSA_AUTH_CALLBACKS = *mut LSA_AUTH_CALLBACKS;
pub type PCLSA_AUTH_CALLBACKS = *const LSA_AUTH_CALLBACKS;
pub type PLSA_CLIENT_REQUEST = *mut PVOID;
ENUM!{enum LSA_TOKEN_INFORMATION_TYPE {
    LsaTokenInformationNull,
    LsaTokenInformationV1,
    LsaTokenInformationV2,
    LsaTokenInformationV3,
}}
pub type PLSA_TOKEN_INFORMATION_TYPE = *mut LSA_TOKEN_INFORMATION_TYPE;
STRUCT!{struct LSA_TOKEN_INFORMATION_NULL {
    ExpirationTime: LARGE_INTEGER,
    Groups: PTOKEN_GROUPS,
}}
pub type PLSA_TOKEN_INFORMATION_NULL = *mut LSA_TOKEN_INFORMATION_NULL;
STRUCT!{struct LSA_TOKEN_INFORMATION_V1 {
    ExpirationTime: LARGE_INTEGER,
    User: TOKEN_USER,
    Groups: PTOKEN_GROUPS,
    PrimaryGroup: TOKEN_PRIMARY_GROUP,
    Privileges: PTOKEN_PRIVILEGES,
    Owner: TOKEN_OWNER,
    DefaultDacl: TOKEN_DEFAULT_DACL,
}}
pub type PLSA_TOKEN_INFORMATION_V1 = *mut LSA_TOKEN_INFORMATION_V1;
pub type LSA_TOKEN_INFORMATION_V2 = LSA_TOKEN_INFORMATION_V1;
pub type PLSA_TOKEN_INFORMATION_V2 = *mut LSA_TOKEN_INFORMATION_V2;
STRUCT!{struct LSA_TOKEN_INFORMATION_V3 {
    ExpirationTime: LARGE_INTEGER,
    User: TOKEN_USER,
    Groups: PTOKEN_GROUPS,
    PrimaryGroup: TOKEN_PRIMARY_GROUP,
    Privileges: PTOKEN_PRIVILEGES,
    Owner: TOKEN_OWNER,
    DefaultDacl: TOKEN_DEFAULT_DACL,
    UserClaims: TOKEN_USER_CLAIMS,
    DeviceClaims: TOKEN_DEVICE_CLAIMS,
    DeviceGroups: PTOKEN_GROUPS,
}}
pub type PLSA_TOKEN_INFORMATION_V3 = *mut LSA_TOKEN_INFORMATION_V3;
FN!{stdcall PLSA_CREATE_LOGON_SESSION(
    LogonId: PLUID,
) -> NTSTATUS}
FN!{stdcall PLSA_DELETE_LOGON_SESSION(
    LogonId: PLUID,
) -> NTSTATUS}
FN!{stdcall PLSA_ADD_CREDENTIAL(
    LogonId: PLUID,
    AuthenticationPackage: ULONG,
    PrimaryKeyValue: PLSA_STRING,
    Credentials: PLSA_STRING,
) -> NTSTATUS}
FN!{stdcall PLSA_GET_CREDENTIALS(
    LogonId: PLUID,
    AuthenticationPackage: ULONG,
    QueryContext: PULONG,
    RetrieveAllCredentials: BOOLEAN,
    PrimaryKeyValue: PLSA_STRING,
    PrimaryKeyLength: PULONG,
    Credentials: PLSA_STRING,
) -> NTSTATUS}
FN!{stdcall PLSA_DELETE_CREDENTIAL(
    LogonId: PLUID,
    AuthenticationPackage: ULONG,
    PrimaryKeyValue: PLSA_STRING,
) -> NTSTATUS}
FN!{stdcall PLSA_ALLOCATE_LSA_HEAP(
    Length: ULONG,
) -> PVOID}
FN!{stdcall PLSA_FREE_LSA_HEAP(
    Base: PVOID,
) -> ()}
FN!{stdcall PLSA_ALLOCATE_PRIVATE_HEAP(
    Length: SIZE_T,
) -> PVOID}
FN!{stdcall PLSA_FREE_PRIVATE_HEAP(
    Base: PVOID,
) -> ()}
FN!{stdcall PLSA_ALLOCATE_CLIENT_BUFFER(
    ClientRequest: PLSA_CLIENT_REQUEST,
    LengthRequired: ULONG,
    ClientBaseAddress: *mut PVOID,
) -> NTSTATUS}
FN!{stdcall PLSA_FREE_CLIENT_BUFFER(
    ClientRequest: PLSA_CLIENT_REQUEST,
    ClientBaseAddress: PVOID,
) -> NTSTATUS}
FN!{stdcall PLSA_COPY_TO_CLIENT_BUFFER(
    ClientRequest: PLSA_CLIENT_REQUEST,
    Length: ULONG,
    ClientBaseAddress: PVOID,
    BufferToCopy: PVOID,
) -> NTSTATUS}
FN!{stdcall PLSA_COPY_FROM_CLIENT_BUFFER(
    ClientRequest: PLSA_CLIENT_REQUEST,
    Length: ULONG,
    BufferToCopy: PVOID,
    ClientBaseAddress: PVOID,
) -> NTSTATUS}
STRUCT!{struct LSA_DISPATCH_TABLE {
    CreateLogonSession: PLSA_CREATE_LOGON_SESSION,
    DeleteLogonSession: PLSA_DELETE_LOGON_SESSION,
    AddCredential: PLSA_ADD_CREDENTIAL,
    GetCredentials: PLSA_GET_CREDENTIALS,
    DeleteCredential: PLSA_DELETE_CREDENTIAL,
    AllocateLsaHeap: PLSA_ALLOCATE_LSA_HEAP,
    FreeLsaHeap: PLSA_FREE_LSA_HEAP,
    AllocateClientBuffer: PLSA_ALLOCATE_CLIENT_BUFFER,
    FreeClientBuffer: PLSA_FREE_CLIENT_BUFFER,
    CopyToClientBuffer: PLSA_COPY_TO_CLIENT_BUFFER,
    CopyFromClientBuffer: PLSA_COPY_FROM_CLIENT_BUFFER,
}}
pub type PLSA_DISPATCH_TABLE = *mut LSA_DISPATCH_TABLE;
pub const LSA_AP_NAME_INITIALIZE_PACKAGE: &'static str = "LsaApInitializePackage";
pub const LSA_AP_NAME_LOGON_USER: &'static str = "LsaApLogonUser";
pub const LSA_AP_NAME_LOGON_USER_EX: &'static str = "LsaApLogonUserEx";
pub const LSA_AP_NAME_CALL_PACKAGE: &'static str = "LsaApCallPackage";
pub const LSA_AP_NAME_LOGON_TERMINATED: &'static str = "LsaApLogonTerminated";
pub const LSA_AP_NAME_CALL_PACKAGE_UNTRUSTED: &'static str = "LsaApCallPackageUntrusted";
pub const LSA_AP_NAME_CALL_PACKAGE_PASSTHROUGH: &'static str = "LsaApCallPackagePassthrough";
FN!{stdcall PLSA_AP_INITIALIZE_PACKAGE(
    AuthenticationPackageId: ULONG,
    LsaDispatchTable: PLSA_DISPATCH_TABLE,
    Database: PLSA_STRING,
    Confidentiality: PLSA_STRING,
    AuthenticationPackageName: *mut PLSA_STRING,
) -> NTSTATUS}
FN!{stdcall PLSA_AP_LOGON_USER(
    ClientRequest: PLSA_CLIENT_REQUEST,
    LogonType: SECURITY_LOGON_TYPE,
    AuthenticationInformation: PVOID,
    ClientAuthentication: PVOID,
    AuthenticationInformationLength: ULONG,
    ProfileBuffer: *mut PVOID,
    ProfileBufferLength: PULONG,
    LogonId: PLUID,
    SubStatus: PNTSTATUS,
    TokenInformationType: PLSA_TOKEN_INFORMATION_TYPE,
    TokenInformation: *mut PVOID,
    AccountName: *mut PLSA_UNICODE_STRING,
    AuthenticatingAutority: *mut PLSA_UNICODE_STRING,
) -> NTSTATUS}
FN!{stdcall PLSA_AP_LOGON_USER_EX(
    ClientRequest: PLSA_CLIENT_REQUEST,
    LogonType: SECURITY_LOGON_TYPE,
    AuthenticationInformation: PVOID,
    ClientAuthentication: PVOID,
    AuthenticationInformationLength: ULONG,
    ProfileBuffer: *mut PVOID,
    ProfileBufferLength: PULONG,
    LogonId: PLUID,
    SubStatus: PNTSTATUS,
    TokenInformationType: PLSA_TOKEN_INFORMATION_TYPE,
    TokenInformation: *mut PVOID,
    AccountName: *mut PLSA_UNICODE_STRING,
    AuthenticatingAutority: *mut PLSA_UNICODE_STRING,
    MachineName: *mut PUNICODE_STRING,
) -> NTSTATUS}
FN!{stdcall PLSA_AP_CALL_PACKAGE(
    ClientRequest: PLSA_CLIENT_REQUEST,
    ProtocolSubmitBuffer: PVOID,
    ClientBufferBase: PVOID,
    SubmitBufferLength: ULONG,
    ProtocolReturnBuffer: *mut PVOID,
    ReturnBufferLength: PULONG,
    ProtocolStatus: PNTSTATUS,
) -> NTSTATUS}
FN!{stdcall PLSA_AP_CALL_PACKAGE_PASSTHROUGH(
    ClientRequest: PLSA_CLIENT_REQUEST,
    ProtocolSubmitBuffer: PVOID,
    ClientBufferBase: PVOID,
    SubmitBufferLength: ULONG,
    ProtocolReturnBuffer: *mut PVOID,
    ReturnBufferLength: PULONG,
    ProtocolStatus: PNTSTATUS,
) -> NTSTATUS}
FN!{stdcall PLSA_AP_LOGON_TERMINATED(
    LogonId: PLUID,
) -> ()}
pub const POLICY_VIEW_LOCAL_INFORMATION: ULONG = 0x00000001;
pub const POLICY_VIEW_AUDIT_INFORMATION: ULONG = 0x00000002;
pub const POLICY_GET_PRIVATE_INFORMATION: ULONG = 0x00000004;
pub const POLICY_TRUST_ADMIN: ULONG = 0x00000008;
pub const POLICY_CREATE_ACCOUNT: ULONG = 0x00000010;
pub const POLICY_CREATE_SECRET: ULONG = 0x00000020;
pub const POLICY_CREATE_PRIVILEGE: ULONG = 0x00000040;
pub const POLICY_SET_DEFAULT_QUOTA_LIMITS: ULONG = 0x00000080;
pub const POLICY_SET_AUDIT_REQUIREMENTS: ULONG = 0x00000100;
pub const POLICY_AUDIT_LOG_ADMIN: ULONG = 0x00000200;
pub const POLICY_SERVER_ADMIN: ULONG = 0x00000400;
pub const POLICY_LOOKUP_NAMES: ULONG = 0x00000800;
pub const POLICY_NOTIFICATION: ULONG = 0x00001000;
pub const POLICY_ALL_ACCESS: ULONG = STANDARD_RIGHTS_REQUIRED | POLICY_VIEW_LOCAL_INFORMATION
    | POLICY_VIEW_AUDIT_INFORMATION | POLICY_GET_PRIVATE_INFORMATION | POLICY_TRUST_ADMIN
    | POLICY_CREATE_ACCOUNT | POLICY_CREATE_SECRET | POLICY_CREATE_PRIVILEGE
    | POLICY_SET_DEFAULT_QUOTA_LIMITS | POLICY_SET_AUDIT_REQUIREMENTS | POLICY_AUDIT_LOG_ADMIN
    | POLICY_SERVER_ADMIN | POLICY_LOOKUP_NAMES;
pub const POLICY_READ: ULONG = STANDARD_RIGHTS_READ | POLICY_VIEW_AUDIT_INFORMATION
    | POLICY_GET_PRIVATE_INFORMATION;
pub const POLICY_WRITE: ULONG = STANDARD_RIGHTS_WRITE | POLICY_TRUST_ADMIN | POLICY_CREATE_ACCOUNT
    | POLICY_CREATE_SECRET | POLICY_CREATE_PRIVILEGE | POLICY_SET_DEFAULT_QUOTA_LIMITS
    | POLICY_SET_AUDIT_REQUIREMENTS | POLICY_AUDIT_LOG_ADMIN | POLICY_SERVER_ADMIN;
pub const POLICY_EXECUTE: ULONG = STANDARD_RIGHTS_EXECUTE | POLICY_VIEW_LOCAL_INFORMATION
    | POLICY_LOOKUP_NAMES;
STRUCT!{struct LSA_TRANSLATED_SID {
    Use: SID_NAME_USE,
    RelativeId: ULONG,
    DomainIndex: LONG,
}}
pub type PLSA_TRANSLATED_SID = *mut LSA_TRANSLATED_SID;
pub type POLICY_SYSTEM_ACCESS_MODE = ULONG;
pub type PPOLICY_SYSTEM_ACCESS_MODE = *mut POLICY_SYSTEM_ACCESS_MODE;
pub const POLICY_MODE_INTERACTIVE: ULONG = SECURITY_ACCESS_INTERACTIVE_LOGON;
pub const POLICY_MODE_NETWORK: ULONG = SECURITY_ACCESS_NETWORK_LOGON;
pub const POLICY_MODE_BATCH: ULONG = SECURITY_ACCESS_BATCH_LOGON;
pub const POLICY_MODE_SERVICE: ULONG = SECURITY_ACCESS_SERVICE_LOGON;
pub const POLICY_MODE_PROXY: ULONG = SECURITY_ACCESS_PROXY_LOGON;
pub const POLICY_MODE_DENY_INTERACTIVE: ULONG = SECURITY_ACCESS_DENY_INTERACTIVE_LOGON;
pub const POLICY_MODE_DENY_NETWORK: ULONG = SECURITY_ACCESS_DENY_NETWORK_LOGON;
pub const POLICY_MODE_DENY_BATCH: ULONG = SECURITY_ACCESS_DENY_BATCH_LOGON;
pub const POLICY_MODE_DENY_SERVICE: ULONG = SECURITY_ACCESS_DENY_SERVICE_LOGON;
pub const POLICY_MODE_REMOTE_INTERACTIVE: ULONG = SECURITY_ACCESS_REMOTE_INTERACTIVE_LOGON;
pub const POLICY_MODE_DENY_REMOTE_INTERACTIVE: ULONG =
    SECURITY_ACCESS_DENY_REMOTE_INTERACTIVE_LOGON;
pub const POLICY_MODE_ALL: ULONG = POLICY_MODE_INTERACTIVE | POLICY_MODE_NETWORK
    | POLICY_MODE_BATCH | POLICY_MODE_SERVICE | POLICY_MODE_PROXY | POLICY_MODE_DENY_INTERACTIVE
    | POLICY_MODE_DENY_NETWORK | SECURITY_ACCESS_DENY_BATCH_LOGON
    | SECURITY_ACCESS_DENY_SERVICE_LOGON | POLICY_MODE_REMOTE_INTERACTIVE
    | POLICY_MODE_DENY_REMOTE_INTERACTIVE ;
pub const POLICY_MODE_ALL_NT4: ULONG = POLICY_MODE_INTERACTIVE | POLICY_MODE_NETWORK
    | POLICY_MODE_BATCH | POLICY_MODE_SERVICE;
ENUM!{enum POLICY_LSA_SERVER_ROLE {
    PolicyServerRoleBackup = 2,
    PolicyServerRolePrimary,
}}
pub type PPOLICY_LSA_SERVER_ROLE = *mut POLICY_LSA_SERVER_ROLE;
ENUM!{enum POLICY_SERVER_ENABLE_STATE {
    PolicyServerEnabled = 2,
    PolicyServerDisabled,
}}
pub type PPOLICY_SERVER_ENABLE_STATE = *mut POLICY_SERVER_ENABLE_STATE;
pub type POLICY_AUDIT_EVENT_OPTIONS = ULONG;
pub type PPOLICY_AUDIT_EVENT_OPTIONS = *mut POLICY_AUDIT_EVENT_OPTIONS;
STRUCT!{struct POLICY_PRIVILEGE_DEFINITION {
    Name: LSA_UNICODE_STRING,
    LocalValue: LUID,
}}
pub type PPOLICY_PRIVILEGE_DEFINITION = *mut POLICY_PRIVILEGE_DEFINITION;
pub const LSA_LOOKUP_ISOLATED_AS_LOCAL: ULONG = 0x80000000;
pub const LSA_LOOKUP_DISALLOW_CONNECTED_ACCOUNT_INTERNET_SID: ULONG = 0x80000000;
pub const LSA_LOOKUP_PREFER_INTERNET_NAMES: ULONG = 0x40000000;
ENUM!{enum POLICY_INFORMATION_CLASS {
    PolicyAuditLogInformation = 1,
    PolicyAuditEventsInformation,
    PolicyPrimaryDomainInformation,
    PolicyPdAccountInformation,
    PolicyAccountDomainInformation,
    PolicyLsaServerRoleInformation,
    PolicyReplicaSourceInformation,
    PolicyDefaultQuotaInformation,
    PolicyModificationInformation,
    PolicyAuditFullSetInformation,
    PolicyAuditFullQueryInformation,
    PolicyDnsDomainInformation,
    PolicyDnsDomainInformationInt,
    PolicyLocalAccountDomainInformation,
    PolicyLastEntry,
}}
pub type PPOLICY_INFORMATION_CLASS = *mut POLICY_INFORMATION_CLASS;
STRUCT!{struct POLICY_AUDIT_LOG_INFO {
    AuditLogPercentFull: ULONG,
    MaximumLogSize: ULONG,
    AuditRetentionPeriod: LARGE_INTEGER,
    AuditLogFullShutdownInProgress: BOOLEAN,
    TimeToShutdown: LARGE_INTEGER,
    NextAuditRecordId: ULONG,
}}
pub type PPOLICY_AUDIT_LOG_INFO = *mut POLICY_AUDIT_LOG_INFO;
STRUCT!{struct POLICY_AUDIT_EVENTS_INFO {
    AuditingMode: BOOLEAN,
    EventAuditingOptions: PPOLICY_AUDIT_EVENT_OPTIONS,
    MaximumAuditEventCount: ULONG,
}}
pub type PPOLICY_AUDIT_EVENTS_INFO = *mut POLICY_AUDIT_EVENTS_INFO;
STRUCT!{struct POLICY_AUDIT_SUBCATEGORIES_INFO {
    MaximumSubCategoryCount: ULONG,
    EventAuditingOptions: PPOLICY_AUDIT_EVENT_OPTIONS,
}}
pub type PPOLICY_AUDIT_SUBCATEGORIES_INFO = *mut POLICY_AUDIT_SUBCATEGORIES_INFO;
STRUCT!{struct POLICY_AUDIT_CATEGORIES_INFO {
    MaximumCategoryCount: ULONG,
    SubCategoriesInfo: PPOLICY_AUDIT_SUBCATEGORIES_INFO,
}}
pub type PPOLICY_AUDIT_CATEGORIES_INFO = *mut POLICY_AUDIT_CATEGORIES_INFO;
pub const PER_USER_POLICY_UNCHANGED: UCHAR = 0x00;
pub const PER_USER_AUDIT_SUCCESS_INCLUDE: UCHAR = 0x01;
pub const PER_USER_AUDIT_SUCCESS_EXCLUDE: UCHAR = 0x02;
pub const PER_USER_AUDIT_FAILURE_INCLUDE: UCHAR = 0x04;
pub const PER_USER_AUDIT_FAILURE_EXCLUDE: UCHAR = 0x08;
pub const PER_USER_AUDIT_NONE: UCHAR = 0x10;
pub const VALID_PER_USER_AUDIT_POLICY_FLAG: UCHAR = PER_USER_AUDIT_SUCCESS_INCLUDE
    | PER_USER_AUDIT_SUCCESS_EXCLUDE | PER_USER_AUDIT_FAILURE_INCLUDE
    | PER_USER_AUDIT_FAILURE_EXCLUDE | PER_USER_AUDIT_NONE;
STRUCT!{struct POLICY_PRIMARY_DOMAIN_INFO {
    Name: LSA_UNICODE_STRING,
    Sid: PSID,
}}
pub type PPOLICY_PRIMARY_DOMAIN_INFO = *mut POLICY_PRIMARY_DOMAIN_INFO;
STRUCT!{struct POLICY_PD_ACCOUNT_INFO {
    Name: LSA_UNICODE_STRING,
}}
pub type PPOLICY_PD_ACCOUNT_INFO = *mut POLICY_PD_ACCOUNT_INFO;
STRUCT!{struct POLICY_LSA_SERVER_ROLE_INFO {
    LsaServerRole: POLICY_LSA_SERVER_ROLE,
}}
pub type PPOLICY_LSA_SERVER_ROLE_INFO = *mut POLICY_LSA_SERVER_ROLE_INFO;
STRUCT!{struct POLICY_REPLICA_SOURCE_INFO {
    ReplicaSource: LSA_UNICODE_STRING,
    ReplicaAccountName: LSA_UNICODE_STRING,
}}
pub type PPOLICY_REPLICA_SOURCE_INFO = *mut POLICY_REPLICA_SOURCE_INFO;
STRUCT!{struct POLICY_DEFAULT_QUOTA_INFO {
    QuotaLimits: QUOTA_LIMITS,
}}
pub type PPOLICY_DEFAULT_QUOTA_INFO = *mut POLICY_DEFAULT_QUOTA_INFO;
STRUCT!{struct POLICY_MODIFICATION_INFO {
    ModifiedId: LARGE_INTEGER,
    DatabaseCreationTime: LARGE_INTEGER,
}}
pub type PPOLICY_MODIFICATION_INFO = *mut POLICY_MODIFICATION_INFO;
STRUCT!{struct POLICY_AUDIT_FULL_SET_INFO {
    ShutDownOnFull: BOOLEAN,
}}
pub type PPOLICY_AUDIT_FULL_SET_INFO = *mut POLICY_AUDIT_FULL_SET_INFO;
STRUCT!{struct POLICY_AUDIT_FULL_QUERY_INFO {
    ShutDownOnFull: BOOLEAN,
    LogIsFull: BOOLEAN,
}}
pub type PPOLICY_AUDIT_FULL_QUERY_INFO = *mut POLICY_AUDIT_FULL_QUERY_INFO;
ENUM!{enum POLICY_DOMAIN_INFORMATION_CLASS {
    PolicyDomainEfsInformation = 2,
    PolicyDomainKerberosTicketInformation,
}}
pub type PPOLICY_DOMAIN_INFORMATION_CLASS = *mut POLICY_DOMAIN_INFORMATION_CLASS;
pub const POLICY_QOS_SCHANNEL_REQUIRED: ULONG = 0x00000001;
pub const POLICY_QOS_OUTBOUND_INTEGRITY: ULONG = 0x00000002;
pub const POLICY_QOS_OUTBOUND_CONFIDENTIALITY: ULONG = 0x00000004;
pub const POLICY_QOS_INBOUND_INTEGRITY: ULONG = 0x00000008;
pub const POLICY_QOS_INBOUND_CONFIDENTIALITY: ULONG = 0x00000010;
pub const POLICY_QOS_ALLOW_LOCAL_ROOT_CERT_STORE: ULONG = 0x00000020;
pub const POLICY_QOS_RAS_SERVER_ALLOWED: ULONG = 0x00000040;
pub const POLICY_QOS_DHCP_SERVER_ALLOWED: ULONG = 0x00000080;
STRUCT!{struct POLICY_DOMAIN_EFS_INFO {
    InfoLength: ULONG,
    EfsBlob: PUCHAR,
}}
pub type PPOLICY_DOMAIN_EFS_INFO = *mut POLICY_DOMAIN_EFS_INFO;
pub const POLICY_KERBEROS_VALIDATE_CLIENT: ULONG = 0x00000080;
STRUCT!{struct POLICY_DOMAIN_KERBEROS_TICKET_INFO {
    AuthenticationOptions: ULONG,
    MaxServiceTicketAge: LARGE_INTEGER,
    MaxTicketAge: LARGE_INTEGER,
    MaxRenewAge: LARGE_INTEGER,
    MaxClockSkew: LARGE_INTEGER,
    Reserved: LARGE_INTEGER,
}}
pub type PPOLICY_DOMAIN_KERBEROS_TICKET_INFO = *mut POLICY_DOMAIN_KERBEROS_TICKET_INFO;
ENUM!{enum POLICY_NOTIFICATION_INFORMATION_CLASS {
    PolicyNotifyAuditEventsInformation = 1,
    PolicyNotifyAccountDomainInformation,
    PolicyNotifyServerRoleInformation,
    PolicyNotifyDnsDomainInformation,
    PolicyNotifyDomainEfsInformation,
    PolicyNotifyDomainKerberosTicketInformation,
    PolicyNotifyMachineAccountPasswordInformation,
    PolicyNotifyGlobalSaclInformation,
    PolicyNotifyMax,
}}
pub type PPOLICY_NOTIFICATION_INFORMATION_CLASS = *mut POLICY_NOTIFICATION_INFORMATION_CLASS;
pub const ACCOUNT_VIEW: ULONG = 0x00000001;
pub const ACCOUNT_ADJUST_PRIVILEGES: ULONG = 0x00000002;
pub const ACCOUNT_ADJUST_QUOTAS: ULONG = 0x00000004;
pub const ACCOUNT_ADJUST_SYSTEM_ACCESS: ULONG = 0x00000008;
pub const ACCOUNT_ALL_ACCESS: ULONG = STANDARD_RIGHTS_REQUIRED | ACCOUNT_VIEW
    | ACCOUNT_ADJUST_PRIVILEGES | ACCOUNT_ADJUST_QUOTAS | ACCOUNT_ADJUST_SYSTEM_ACCESS;
pub const ACCOUNT_READ: ULONG = STANDARD_RIGHTS_READ | ACCOUNT_VIEW;
pub const ACCOUNT_WRITE: ULONG = STANDARD_RIGHTS_WRITE | ACCOUNT_ADJUST_PRIVILEGES
    | ACCOUNT_ADJUST_QUOTAS | ACCOUNT_ADJUST_SYSTEM_ACCESS;
pub const ACCOUNT_EXECUTE: ULONG = STANDARD_RIGHTS_EXECUTE;
DECLARE_HANDLE!{LSA_HANDLE, __LSA_HANDLE}
pub const TRUSTED_QUERY_DOMAIN_NAME: ULONG = 0x00000001;
pub const TRUSTED_QUERY_CONTROLLERS: ULONG = 0x00000002;
pub const TRUSTED_SET_CONTROLLERS: ULONG = 0x00000004;
pub const TRUSTED_QUERY_POSIX: ULONG = 0x00000008;
pub const TRUSTED_SET_POSIX: ULONG = 0x00000010;
pub const TRUSTED_SET_AUTH: ULONG = 0x00000020;
pub const TRUSTED_QUERY_AUTH: ULONG = 0x00000040;
pub const TRUSTED_ALL_ACCESS: ULONG = STANDARD_RIGHTS_REQUIRED | TRUSTED_QUERY_DOMAIN_NAME
    | TRUSTED_QUERY_CONTROLLERS | TRUSTED_SET_CONTROLLERS | TRUSTED_QUERY_POSIX | TRUSTED_SET_POSIX
    | TRUSTED_SET_AUTH | TRUSTED_QUERY_AUTH;
pub const TRUSTED_READ: ULONG = STANDARD_RIGHTS_READ | TRUSTED_QUERY_DOMAIN_NAME;
pub const TRUSTED_WRITE: ULONG = STANDARD_RIGHTS_WRITE | TRUSTED_SET_CONTROLLERS
    | TRUSTED_SET_POSIX | TRUSTED_SET_AUTH;
pub const TRUSTED_EXECUTE: ULONG = STANDARD_RIGHTS_EXECUTE | TRUSTED_QUERY_CONTROLLERS
    | TRUSTED_QUERY_POSIX;
ENUM!{enum TRUSTED_INFORMATION_CLASS {
    TrustedDomainNameInformation = 1,
    TrustedControllersInformation,
    TrustedPosixOffsetInformation,
    TrustedPasswordInformation,
    TrustedDomainInformationBasic,
    TrustedDomainInformationEx,
    TrustedDomainAuthInformation,
    TrustedDomainFullInformation,
    TrustedDomainAuthInformationInternal,
    TrustedDomainFullInformationInternal,
    TrustedDomainInformationEx2Internal,
    TrustedDomainFullInformation2Internal,
    TrustedDomainSupportedEncryptionTypes,
}}
pub type PTRUSTED_INFORMATION_CLASS = *mut TRUSTED_INFORMATION_CLASS;
STRUCT!{struct TRUSTED_DOMAIN_NAME_INFO {
    Name: LSA_UNICODE_STRING,
}}
pub type PTRUSTED_DOMAIN_NAME_INFO = *mut TRUSTED_DOMAIN_NAME_INFO;
STRUCT!{struct TRUSTED_CONTROLLERS_INFO {
    Entries: ULONG,
    Names: PLSA_UNICODE_STRING,
}}
pub type PTRUSTED_CONTROLLERS_INFO = *mut TRUSTED_CONTROLLERS_INFO;
STRUCT!{struct TRUSTED_POSIX_OFFSET_INFO {
    Offset: ULONG,
}}
pub type PTRUSTED_POSIX_OFFSET_INFO = *mut TRUSTED_POSIX_OFFSET_INFO;
STRUCT!{struct TRUSTED_PASSWORD_INFO {
    Password: LSA_UNICODE_STRING,
    OldPassword: LSA_UNICODE_STRING,
}}
pub type PTRUSTED_PASSWORD_INFO = *mut TRUSTED_PASSWORD_INFO;
pub type TRUSTED_DOMAIN_INFORMATION_BASIC = LSA_TRUST_INFORMATION;
pub type PTRUSTED_DOMAIN_INFORMATION_BASIC = PLSA_TRUST_INFORMATION;
// NOTE: Ignoring Win XP constants
pub const TRUST_DIRECTION_DISABLED: ULONG = 0x00000000;
pub const TRUST_DIRECTION_INBOUND: ULONG = 0x00000001;
pub const TRUST_DIRECTION_OUTBOUND: ULONG = 0x00000002;
pub const TRUST_DIRECTION_BIDIRECTIONAL: ULONG = TRUST_DIRECTION_INBOUND
    | TRUST_DIRECTION_OUTBOUND;
pub const TRUST_TYPE_DOWNLEVEL: ULONG = 0x00000001;
pub const TRUST_TYPE_UPLEVEL: ULONG = 0x00000002;
pub const TRUST_TYPE_MIT: ULONG = 0x00000003;
pub const TRUST_ATTRIBUTE_NON_TRANSITIVE: ULONG = 0x00000001;
pub const TRUST_ATTRIBUTE_UPLEVEL_ONLY: ULONG = 0x00000002;
pub const TRUST_ATTRIBUTE_QUARANTINED_DOMAIN: ULONG = 0x00000004;
pub const TRUST_ATTRIBUTE_FOREST_TRANSITIVE: ULONG = 0x00000008;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION: ULONG = 0x00000010;
pub const TRUST_ATTRIBUTE_WITHIN_FOREST: ULONG = 0x00000020;
pub const TRUST_ATTRIBUTE_TREAT_AS_EXTERNAL: ULONG = 0x00000040;
pub const TRUST_ATTRIBUTE_TRUST_USES_RC4_ENCRYPTION: ULONG = 0x00000080;
pub const TRUST_ATTRIBUTE_TRUST_USES_AES_KEYS: ULONG = 0x00000100;
pub const TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION: ULONG = 0x00000200;
pub const TRUST_ATTRIBUTE_PIM_TRUST: ULONG = 0x00000400;
pub const TRUST_ATTRIBUTES_VALID: ULONG = 0xFF03FFFF;
pub const TRUST_ATTRIBUTES_USER: ULONG = 0xFF000000;
STRUCT!{struct TRUSTED_DOMAIN_INFORMATION_EX {
    Name: LSA_UNICODE_STRING,
    FlatName: LSA_UNICODE_STRING,
    Sid: PSID,
    TrustDirection: ULONG,
    TrustType: ULONG,
    TrustAttributes: ULONG,
}}
pub type PTRUSTED_DOMAIN_INFORMATION_EX = *mut TRUSTED_DOMAIN_INFORMATION_EX;
STRUCT!{struct TRUSTED_DOMAIN_INFORMATION_EX2 {
    Name: LSA_UNICODE_STRING,
    FlatName: LSA_UNICODE_STRING,
    Sid: PSID,
    TrustDirection: ULONG,
    TrustType: ULONG,
    TrustAttributes: ULONG,
    ForestTrustLength: ULONG,
    ForestTrustInfo: PUCHAR,
}}
pub type PTRUSTED_DOMAIN_INFORMATION_EX2 = *mut TRUSTED_DOMAIN_INFORMATION_EX2;
pub const TRUST_AUTH_TYPE_NONE: ULONG = 0;
pub const TRUST_AUTH_TYPE_NT4OWF: ULONG = 1;
pub const TRUST_AUTH_TYPE_CLEAR: ULONG = 2;
pub const TRUST_AUTH_TYPE_VERSION: ULONG = 3;
STRUCT!{struct LSA_AUTH_INFORMATION {
    LastUpdateTime: LARGE_INTEGER,
    AuthType: ULONG,
    AuthInfoLength: ULONG,
    AuthInfo: PUCHAR,
}}
pub type PLSA_AUTH_INFORMATION = *mut LSA_AUTH_INFORMATION;
STRUCT!{struct TRUSTED_DOMAIN_AUTH_INFORMATION {
    IncomingAuthInfos: ULONG,
    IncomingAuthenticationInformation: PLSA_AUTH_INFORMATION,
    IncomingPreviousAuthenticationInformation: PLSA_AUTH_INFORMATION,
    OutgoingAuthInfos: ULONG,
    OutgoingAuthenticationInformation: PLSA_AUTH_INFORMATION,
    OutgoingPreviousAuthenticationInformation: PLSA_AUTH_INFORMATION,
}}
pub type PTRUSTED_DOMAIN_AUTH_INFORMATION = *mut TRUSTED_DOMAIN_AUTH_INFORMATION;
STRUCT!{struct TRUSTED_DOMAIN_FULL_INFORMATION {
    Information: TRUSTED_DOMAIN_INFORMATION_EX,
    PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}}
pub type PTRUSTED_DOMAIN_FULL_INFORMATION = *mut TRUSTED_DOMAIN_FULL_INFORMATION;
STRUCT!{struct TRUSTED_DOMAIN_FULL_INFORMATION2 {
    Information: TRUSTED_DOMAIN_INFORMATION_EX2,
    PosixOffset: TRUSTED_POSIX_OFFSET_INFO,
    AuthInformation: TRUSTED_DOMAIN_AUTH_INFORMATION,
}}
pub type PTRUSTED_DOMAIN_FULL_INFORMATION2 = *mut TRUSTED_DOMAIN_FULL_INFORMATION2;
STRUCT!{struct TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {
    SupportedEncryptionTypes: ULONG,
}}
pub type PTRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES =
    *mut TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES;
ENUM!{enum LSA_FOREST_TRUST_RECORD_TYPE {
    ForestTrustTopLevelName,
    ForestTrustTopLevelNameEx,
    ForestTrustDomainInfo,
    ForestTrustRecordTypeLast = ForestTrustDomainInfo,
}}
pub const LSA_FTRECORD_DISABLED_REASONS: ULONG = 0x0000FFFF;
pub const LSA_TLN_DISABLED_NEW: ULONG = 0x00000001;
pub const LSA_TLN_DISABLED_ADMIN: ULONG = 0x00000002;
pub const LSA_TLN_DISABLED_CONFLICT: ULONG = 0x00000004;
pub const LSA_SID_DISABLED_ADMIN: ULONG = 0x00000001;
pub const LSA_SID_DISABLED_CONFLICT: ULONG = 0x00000002;
pub const LSA_NB_DISABLED_ADMIN: ULONG = 0x00000004;
pub const LSA_NB_DISABLED_CONFLICT: ULONG = 0x00000008;
STRUCT!{struct LSA_FOREST_TRUST_DOMAIN_INFO {
    Sid: PSID,
    DnsName: LSA_UNICODE_STRING,
    NetbiosName: LSA_UNICODE_STRING,
}}
pub type PLSA_FOREST_TRUST_DOMAIN_INFO = *mut LSA_FOREST_TRUST_DOMAIN_INFO;
pub const MAX_FOREST_TRUST_BINARY_DATA_SIZE: SIZE_T = 128 * 1024;
STRUCT!{struct LSA_FOREST_TRUST_BINARY_DATA {
    Length: ULONG,
    Buffer: PUCHAR,
}}
pub type PLSA_FOREST_TRUST_BINARY_DATA = *mut LSA_FOREST_TRUST_BINARY_DATA;
UNION!{union LSA_FOREST_TRUST_RECORD_FORESTTRUSTDATA {
    [u32; 5] [u64; 5],
    TopLevelName TopLevelName_mut: LSA_UNICODE_STRING,
    DomainInfo DomainInfo_mut: LSA_FOREST_TRUST_DOMAIN_INFO,
    Data Data_mut: LSA_FOREST_TRUST_BINARY_DATA,
}}
STRUCT!{struct LSA_FOREST_TRUST_RECORD {
    Flags: ULONG,
    ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    Time: LARGE_INTEGER,
    ForestTrustData: LSA_FOREST_TRUST_RECORD_FORESTTRUSTDATA,
}}
pub type PLSA_FOREST_TRUST_RECORD = *mut LSA_FOREST_TRUST_RECORD;
pub const MAX_RECORDS_IN_FOREST_TRUST_INFO: SIZE_T = 4000;
STRUCT!{struct LSA_FOREST_TRUST_INFORMATION {
    RecordCount: ULONG,
    Entries: *mut PLSA_FOREST_TRUST_RECORD,
}}
pub type PLSA_FOREST_TRUST_INFORMATION = LSA_FOREST_TRUST_INFORMATION;
ENUM!{enum LSA_FOREST_TRUST_COLLISION_RECORD_TYPE {
    CollisionTdo,
    CollisionXref,
    CollisionOther,
}}
STRUCT!{struct LSA_FOREST_TRUST_COLLISION_RECORD {
    Index: ULONG,
    Type: LSA_FOREST_TRUST_COLLISION_RECORD_TYPE,
    Flags: ULONG,
    Name: LSA_UNICODE_STRING,
}}
pub type PLSA_FOREST_TRUST_COLLISION_RECORD = *mut LSA_FOREST_TRUST_COLLISION_RECORD;
STRUCT!{struct LSA_FOREST_TRUST_COLLISION_INFORMATION {
    RecordCount: ULONG,
    Entries: *mut PLSA_FOREST_TRUST_COLLISION_RECORD,
}}
pub type PLSA_FOREST_TRUST_COLLISION_INFORMATION = *mut LSA_FOREST_TRUST_COLLISION_INFORMATION;
pub const SECRET_SET_VALUE: ULONG = 0x00000001;
pub const SECRET_QUERY_VALUE: ULONG = 0x00000002;
pub const SECRET_ALL_ACCESS: ULONG = STANDARD_RIGHTS_REQUIRED | SECRET_SET_VALUE
    | SECRET_QUERY_VALUE;
pub const SECRET_READ: ULONG = STANDARD_RIGHTS_READ | SECRET_QUERY_VALUE;
pub const SECRET_WRITE: ULONG = STANDARD_RIGHTS_WRITE | SECRET_SET_VALUE;
pub const SECRET_EXECUTE: ULONG = STANDARD_RIGHTS_EXECUTE;
pub const LSA_GLOBAL_SECRET_PREFIX: &'static str = "G$";
pub const LSA_GLOBAL_SECRET_PREFIX_LENGTH: SIZE_T = 2;
pub const LSA_LOCAL_SECRET_PREFIX: &'static str = "L$";
pub const LSA_LOCAL_SECRET_PREFIX_LENGTH: SIZE_T = 2;
pub const LSA_MACHINE_SECRET_PREFIX: &'static str = "M$";
pub const LSA_MACHINE_SECRET_PREFIX_LENGTH: SIZE_T = 2;
pub const LSA_SECRET_MAXIMUM_COUNT: SIZE_T = 0x00001000;
pub const LSA_SECRET_MAXIMUM_LENGTH: SIZE_T = 0x00000200;
DECLARE_HANDLE!{LSA_ENUMERATION_HANDLE, __LSA_ENUMERATION_HANDLE}
pub type PLSA_ENUMERATION_HANDLE = *mut LSA_ENUMERATION_HANDLE;
STRUCT!{struct LSA_ENUMERATION_INFORMATION {
    Sid: PSID,
}}
pub type PLSA_ENUMERATION_INFORMATION = *mut LSA_ENUMERATION_INFORMATION;
extern "system" {
    pub fn LsaFreeMemory(
        Buffer: PVOID,
    ) -> NTSTATUS;
    pub fn LsaClose(
        ObjectHandle: LSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaDelete(
        ObjectHandle: LSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaQuerySecurityObject(
        ObjectHandle: LSA_HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    pub fn LsaSetSecurityObject(
        ObjectHandle: LSA_HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;
    pub fn LsaChangePassword(
        ServerName: PLSA_UNICODE_STRING,
        DomainName: PLSA_UNICODE_STRING,
        AccountName: PLSA_UNICODE_STRING,
        OldPassword: PLSA_UNICODE_STRING,
        NewPassword: PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
}
STRUCT!{struct LSA_LAST_INTER_LOGON_INFO {
    LastSuccessfulLogon: LARGE_INTEGER,
    LastFailedLogon: LARGE_INTEGER,
    FailedAttemptCountSinceLastSuccessfulLogon: ULONG,
}}
pub type PLSA_LAST_INTER_LOGON_INFO = *mut LSA_LAST_INTER_LOGON_INFO;
STRUCT!{struct SECURITY_LOGON_SESSION_DATA {
    Size: ULONG,
    LogonId: LUID,
    UserName: LSA_UNICODE_STRING,
    LogonDomain: LSA_UNICODE_STRING,
    AuthenticationPackage: LSA_UNICODE_STRING,
    LogonType: ULONG,
    Session: ULONG,
    Sid: PSID,
    LogonTime: LARGE_INTEGER,
    LogonServer: LSA_UNICODE_STRING,
    DnsDomainName: LSA_UNICODE_STRING,
    Upn: LSA_UNICODE_STRING,
    UserFlags: ULONG,
    LastLogonInfo: LSA_LAST_INTER_LOGON_INFO,
    LogonScript: LSA_UNICODE_STRING,
    ProfilePath: LSA_UNICODE_STRING,
    HomeDirectory: LSA_UNICODE_STRING,
    HomeDirectoryDrive: LSA_UNICODE_STRING,
    LogoffTime: LARGE_INTEGER,
    KickOffTime: LARGE_INTEGER,
    PasswordLastSet: LARGE_INTEGER,
    PasswordCanChange: LARGE_INTEGER,
    PasswordMustChange: LARGE_INTEGER,
}}
pub type PSECURITY_LOGON_SESSION_DATA = *mut SECURITY_LOGON_SESSION_DATA;
extern "system" {
    pub fn LsaEnumerateLogonSessions(
        LogonSessionCount: PULONG,
        LogonSessionList: *mut PLUID,
    ) -> NTSTATUS;
    pub fn LsaGetLogonSessionData(
        LogonId: PLUID,
        ppLogonSessionData: *mut PSECURITY_LOGON_SESSION_DATA,
    ) -> NTSTATUS;
    pub fn LsaOpenPolicy(
        SystemName: PLSA_UNICODE_STRING,
        ObjectAttributes: PLSA_OBJECT_ATTRIBUTES,
        DesiredAccess: ACCESS_MASK,
        PolicyHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaOpenPolicySce(
        SystemName: PLSA_UNICODE_STRING,
        ObjectAttributes: PLSA_OBJECT_ATTRIBUTES,
        DesiredAccess: ACCESS_MASK,
        PolicyHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
}
pub const MAXIMUM_CAPES_PER_CAP: SIZE_T = 0x7F;
pub const CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG: ULONG = 0x00000001;
pub const CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG: ULONG = 0x00000100;
#[inline]
pub fn STAGING_FLAG(Effective: ULONG) -> ULONG {
    (Effective & 0xF) << 8
}
pub const CENTRAL_ACCESS_POLICY_STAGED_FLAG: ULONG = 0x00010000;
pub const CENTRAL_ACCESS_POLICY_VALID_FLAG_MASK: ULONG =
    CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG
    | CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG | CENTRAL_ACCESS_POLICY_STAGED_FLAG;
pub const LSASETCAPS_RELOAD_FLAG: ULONG = 0x00000001;
pub const LSASETCAPS_VALID_FLAG_MASK: ULONG = LSASETCAPS_RELOAD_FLAG;
STRUCT!{struct CENTRAL_ACCESS_POLICY_ENTRY {
    Name: LSA_UNICODE_STRING,
    Description: LSA_UNICODE_STRING,
    ChangeId: LSA_UNICODE_STRING,
    LengthAppliesTo: ULONG,
    AppliesTo: PUCHAR,
    LengthSD: ULONG,
    SD: PSECURITY_DESCRIPTOR,
    LengthStagedSD: ULONG,
    StagedSD: PSECURITY_DESCRIPTOR,
    Flags: ULONG,
}}
pub type PCENTRAL_ACCESS_POLICY_ENTRY = *mut CENTRAL_ACCESS_POLICY_ENTRY;
pub type PCCENTRAL_ACCESS_POLICY_ENTRY = *const CENTRAL_ACCESS_POLICY_ENTRY;
STRUCT!{struct CENTRAL_ACCESS_POLICY {
    CAPID: PSID,
    Name: LSA_UNICODE_STRING,
    Description: LSA_UNICODE_STRING,
    ChangeId: LSA_UNICODE_STRING,
    Flags: ULONG,
    CAPECount: ULONG,
    CAPEs: *mut PCENTRAL_ACCESS_POLICY_ENTRY,
}}
pub type PCENTRAL_ACCESS_POLICY = *mut CENTRAL_ACCESS_POLICY;
pub type PCCENTRAL_ACCESS_POLICY = *const CENTRAL_ACCESS_POLICY;
extern "system" {
    pub fn LsaSetCAPs(
        CAPDNs: PLSA_UNICODE_STRING,
        CAPDNCount: ULONG,
        Flags: ULONG,
    ) -> NTSTATUS;
    pub fn LsaGetAppliedCAPIDs(
        SystemName: PLSA_UNICODE_STRING,
        CAPIDs: *mut *mut PSID,
        CAPIDCount: PULONG,
    ) -> NTSTATUS;
    pub fn LsaQueryCAPs(
        CAPIDs: *mut PSID,
        CAPIDCount: ULONG,
        CAPs: *mut PCENTRAL_ACCESS_POLICY,
        CAPCount: PULONG,
    ) -> NTSTATUS;
    pub fn LsaQueryInformationPolicy(
        PolicyHandle: LSA_HANDLE,
        InformationClass: POLICY_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    pub fn LsaSetInformationPolicy(
        PolicyHandle: LSA_HANDLE,
        InformationClass: POLICY_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    pub fn LsaQueryDomainInformationPolicy(
        PolicyHandle: LSA_HANDLE,
        InformationClass: POLICY_DOMAIN_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    pub fn LsaSetDomainInformationPolicy(
        PolicyHandle: LSA_HANDLE,
        InformationClass: POLICY_DOMAIN_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    pub fn LsaRegisterPolicyChangeNotification(
        InformationClass: POLICY_NOTIFICATION_INFORMATION_CLASS,
        NotifcationEventHandle: HANDLE,
    ) -> NTSTATUS;
    pub fn LsaUnregisterPolicyChangeNotification(
        InformationClass: POLICY_NOTIFICATION_INFORMATION_CLASS,
        NotifcationEventHandle: HANDLE,
    ) -> NTSTATUS;
    pub fn LsaClearAuditLog(
        PolicyHandle: LSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaCreateAccount(
        PolicyHandle: LSA_HANDLE,
        AccountSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AccountHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaEnumerateAccounts(
        PolicyHandle: LSA_HANDLE,
        EnumerationContext: PLSA_ENUMERATION_HANDLE,
        Buffer: *mut PVOID,
        PreferredMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    pub fn LsaCreateTrustedDomain(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainInformation: PLSA_TRUST_INFORMATION,
        DesiredAccess: ACCESS_MASK,
        TrustedDomainHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaEnumerateTrustedDomains(
        PolicyHandle: LSA_HANDLE,
        EnumerationContext: PLSA_ENUMERATION_HANDLE,
        Buffer: *mut PVOID,
        PreferredMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    pub fn LsaEnumeratePrivileges(
        PolicyHandle: LSA_HANDLE,
        EnumerationContext: PLSA_ENUMERATION_HANDLE,
        Buffer: *mut PVOID,
        PreferredMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    pub fn LsaLookupNames(
        PolicyHandle: LSA_HANDLE,
        Count: ULONG,
        Names: PLSA_UNICODE_STRING,
        ReferencedDomains: *mut PLSA_REFERENCED_DOMAIN_LIST,
        Sids: *mut PLSA_TRANSLATED_SID,
    ) -> NTSTATUS;
    pub fn LsaLookupNames2(
        PolicyHandle: LSA_HANDLE,
        Flags: ULONG,
        Count: ULONG,
        Names: PLSA_UNICODE_STRING,
        ReferencedDomains: *mut PLSA_REFERENCED_DOMAIN_LIST,
        Sids: *mut PLSA_TRANSLATED_SID2,
    ) -> NTSTATUS;
    pub fn LsaLookupSids(
        PolicyHandle: LSA_HANDLE,
        Count: ULONG,
        Sids: *mut PSID,
        ReferencedDomains: *mut PLSA_REFERENCED_DOMAIN_LIST,
        Names: *mut PLSA_TRANSLATED_NAME,
    ) -> NTSTATUS;
    pub fn LsaLookupSids2(
        PolicyHandle: LSA_HANDLE,
        LookupOptions: ULONG,
        Count: ULONG,
        Sids: *mut PSID,
        ReferencedDomains: *mut PLSA_REFERENCED_DOMAIN_LIST,
        Names: *mut PLSA_TRANSLATED_NAME,
    ) -> NTSTATUS;
    pub fn LsaCreateSecret(
        PolicyHandle: LSA_HANDLE,
        SecretName: PLSA_UNICODE_STRING,
        DesiredAccess: ACCESS_MASK,
        SecretHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaOpenAccount(
        PolicyHandle: LSA_HANDLE,
        AccountSid: PSID,
        DesiredAccess: ACCESS_MASK,
        AccountHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaEnumeratePrivilegesOfAccount(
        AccountHandle: LSA_HANDLE,
        Privileges: *mut PPRIVILEGE_SET,
    ) -> NTSTATUS;
    pub fn LsaAddPrivilegesToAccount(
        AccountHandle: LSA_HANDLE,
        Privileges: PPRIVILEGE_SET,
    ) -> NTSTATUS;
    pub fn LsaRemovePrivilegesFromAccount(
        AccountHandle: LSA_HANDLE,
        AllPrivileges: BOOLEAN,
        Privileges: PPRIVILEGE_SET,
    ) -> NTSTATUS;
    pub fn LsaGetQuotasForAccount(
        AccountHandle: LSA_HANDLE,
        QuotaLimits: PQUOTA_LIMITS,
    ) -> NTSTATUS;
    pub fn LsaSetQuotasForAccount(
        AccountHandle: LSA_HANDLE,
        QuotaLimits: PQUOTA_LIMITS,
    ) -> NTSTATUS;
    pub fn LsaGetSystemAccessAccount(
        AccountHandle: LSA_HANDLE,
        SystemAccess: PULONG,
    ) -> NTSTATUS;
    pub fn LsaSetSystemAccessAccount(
        AccountHandle: LSA_HANDLE,
        SystemAccess: ULONG,
    ) -> NTSTATUS;
    pub fn LsaOpenTrustedDomain(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainSid: PSID,
        DesiredAccess: ACCESS_MASK,
        TrustedDomainHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaQueryInfoTrustedDomain(
        TrustedDomainHandle: LSA_HANDLE,
        InformationClass: TRUSTED_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    pub fn LsaSetInformationTrustedDomain(
        TrustedDomainHandle: LSA_HANDLE,
        InformationClass: TRUSTED_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    pub fn LsaOpenSecret(
        PolicyHandle: LSA_HANDLE,
        SecretName: PLSA_UNICODE_STRING,
        DesiredAccess: ACCESS_MASK,
        SecretHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaSetSecret(
        SecretHandle: LSA_HANDLE,
        CurrentValue: PLSA_UNICODE_STRING,
        OldValue: PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
    pub fn LsaQuerySecret(
        SecretHandle: LSA_HANDLE,
        CurrentValue: *mut PLSA_UNICODE_STRING,
        CurrentValueSetTime: PLARGE_INTEGER,
        OldValue: *mut PLSA_UNICODE_STRING,
        OldValueSetTime: PLARGE_INTEGER,
    ) -> NTSTATUS;
    pub fn LsaLookupPrivilegeValue(
        PolicyHandle: LSA_HANDLE,
        Name: PLSA_UNICODE_STRING,
        Value: PLUID,
    ) -> NTSTATUS;
    pub fn LsaLookupPrivilegeName(
        PolicyHandle: LSA_HANDLE,
        Value: PLUID,
        Name: *mut PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
    pub fn LsaLookupPrivilegeDisplayName(
        PolicyHandle: LSA_HANDLE,
        Name: PLSA_UNICODE_STRING,
        DisplayName: *mut PLSA_UNICODE_STRING,
        LanguageReturned: PSHORT,
    ) -> NTSTATUS;
}
extern "C" {
    pub fn LsaGetUserName(
        UserName: *mut PLSA_UNICODE_STRING,
        DomainName: *mut PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
    pub fn LsaGetRemoteUserName(
        SystemName: PLSA_UNICODE_STRING,
        UserName: *mut PLSA_UNICODE_STRING,
        DomainName: *mut PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
}
pub const SE_INTERACTIVE_LOGON_NAME: &'static str = "SeInteractiveLogonRight";
pub const SE_NETWORK_LOGON_NAME: &'static str = "SeNetworkLogonRight";
pub const SE_BATCH_LOGON_NAME: &'static str = "SeBatchLogonRight";
pub const SE_SERVICE_LOGON_NAME: &'static str = "SeServiceLogonRight";
pub const SE_DENY_INTERACTIVE_LOGON_NAME: &'static str = "SeDenyInteractiveLogonRight";
pub const SE_DENY_NETWORK_LOGON_NAME: &'static str = "SeDenyNetworkLogonRight";
pub const SE_DENY_BATCH_LOGON_NAME: &'static str = "SeDenyBatchLogonRight";
pub const SE_DENY_SERVICE_LOGON_NAME: &'static str = "SeDenyServiceLogonRight";
pub const SE_REMOTE_INTERACTIVE_LOGON_NAME: &'static str = "SeRemoteInteractiveLogonRight";
pub const SE_DENY_REMOTE_INTERACTIVE_LOGON_NAME: &'static str =
    "SeDenyRemoteInteractiveLogonRight";
extern "system" {
    pub fn LsaEnumerateAccountsWithUserRight(
        PolictHandle: LSA_HANDLE,
        UserRights: PLSA_UNICODE_STRING,
        EnumerationBuffer: *mut PVOID,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    pub fn LsaEnumerateAccountRights(
        PolicyHandle: LSA_HANDLE,
        AccountSid: PSID,
        UserRights: *mut PLSA_UNICODE_STRING,
        CountOfRights: PULONG,
    ) -> NTSTATUS;
    pub fn LsaAddAccountRights(
        PolicyHandle: LSA_HANDLE,
        AccountSid: PSID,
        UserRights: PLSA_UNICODE_STRING,
        CountOfRights: ULONG,
    ) -> NTSTATUS;
    pub fn LsaRemoveAccountRights(
        PolicyHandle: LSA_HANDLE,
        AccountSid: PSID,
        AllRights: BOOLEAN,
        UserRights: PLSA_UNICODE_STRING,
        CountOfRights: ULONG,
    ) -> NTSTATUS;
    pub fn LsaOpenTrustedDomainByName(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainName: PLSA_UNICODE_STRING,
        DesiredAccess: ACCESS_MASK,
        TrustedDomainHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaQueryTrustedDomainInfo(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainSid: PSID,
        InformationClass: TRUSTED_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    pub fn LsaSetTrustedDomainInformation(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainSid: PSID,
        InformationClass: TRUSTED_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    pub fn LsaDeleteTrustedDomain(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainSid: PSID,
    ) -> NTSTATUS;
    pub fn LsaQueryTrustedDomainInfoByName(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainName: PLSA_UNICODE_STRING,
        InformationClass: TRUSTED_INFORMATION_CLASS,
        Buffer: *mut PVOID,
    ) -> NTSTATUS;
    pub fn LsaSetTrustedDomainInfoByName(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainName: PLSA_UNICODE_STRING,
        InformationClass: TRUSTED_INFORMATION_CLASS,
        Buffer: PVOID,
    ) -> NTSTATUS;
    pub fn LsaEnumerateTrustedDomainsEx(
        PolicyHandle: LSA_HANDLE,
        EnumerationContext: PLSA_ENUMERATION_HANDLE,
        Buffer: *mut PVOID,
        PreferredMaximumLength: ULONG,
        CountReturned: PULONG,
    ) -> NTSTATUS;
    pub fn LsaCreateTrustedDomainEx(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainInformation: PTRUSTED_DOMAIN_INFORMATION_EX,
        AuthenticationInformation: PTRUSTED_DOMAIN_AUTH_INFORMATION,
        DesiredAccess: ACCESS_MASK,
        TrustedDomainHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaQueryForestTrustInformation(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainName: PLSA_UNICODE_STRING,
        ForestTrustInfo: *mut PLSA_FOREST_TRUST_INFORMATION,
    ) -> NTSTATUS;
    pub fn LsaSetForestTrustInformation(
        PolicyHandle: LSA_HANDLE,
        TrustedDomainName: PLSA_UNICODE_STRING,
        ForestTrustInfo: PLSA_FOREST_TRUST_INFORMATION,
        CheckOnly: BOOLEAN,
        CollisionInfo: *mut PLSA_FOREST_TRUST_COLLISION_INFORMATION,
    ) -> NTSTATUS;
    pub fn LsaForestTrustFindMatch(
        PolicyHandle: LSA_HANDLE,
        Type: ULONG,
        Name: PLSA_UNICODE_STRING,
        Match: *mut PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
    pub fn LsaStorePrivateData(
        PolicyHandle: LSA_HANDLE,
        KeyName: PLSA_UNICODE_STRING,
        PrivateData: PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
    pub fn LsaRetrievePrivateData(
        PolicyHandle: LSA_HANDLE,
        KeyName: PLSA_UNICODE_STRING,
        PrivateData: *mut PLSA_UNICODE_STRING,
    ) -> NTSTATUS;
    pub fn LsaNtStatusToWinError(
        Status: NTSTATUS,
    ) -> ULONG;
}
ENUM!{enum NEGOTIATE_MESSAGES {
    NegEnumPackagePrefixes = 0,
    NegGetCallerName = 1,
    NegTransferCredentials = 2,
    NegEnumPackageNames = 3,
    NegCallPackageMax,
}}
pub const NEGOTIATE_MAX_PREFIX: SIZE_T = 32;
STRUCT!{struct NEGOTIATE_PACKAGE_PREFIX {
    PackageId: ULONG_PTR,
    PackageDataA: PVOID,
    PackageDataW: PVOID,
    PrefixLen: ULONG_PTR,
    Prefix: [UCHAR; NEGOTIATE_MAX_PREFIX],
}}
pub type PNEGOTIATE_PACKAGE_PREFIX = *mut NEGOTIATE_PACKAGE_PREFIX;
STRUCT!{struct NEGOTIATE_PACKAGE_PREFIXES {
    MessageType: ULONG,
    PrefixCount: ULONG,
    Offset: ULONG,
    Pad: ULONG,
}}
pub type PNEGOTIATE_PACKAGE_PREFIXES = *mut NEGOTIATE_PACKAGE_PREFIXES;
STRUCT!{struct NEGOTIATE_CALLER_NAME_REQUEST {
    MessageType: ULONG,
    LogonId: LUID,
}}
pub type PNEGOTIATE_CALLER_NAME_REQUEST = *mut NEGOTIATE_CALLER_NAME_REQUEST;
STRUCT!{struct NEGOTIATE_CALLER_NAME_RESPONSE {
    Messagetype: ULONG,
    CallerName: PWSTR,
}}
pub type PNEGOTIATE_CALLER_NAME_RESPONSE = *mut NEGOTIATE_CALLER_NAME_RESPONSE;
STRUCT!{struct NEGOTIATE_PACKAGE_NAMES {
    NamesCount: ULONG,
    Names: [UNICODE_STRING; ANYSIZE_ARRAY],
}}
pub type PNEGOTIATE_PACKAGE_NAMES = *mut NEGOTIATE_PACKAGE_NAMES;
pub const NEGOTIATE_ALLOW_NTLM: ULONG = 0x10000000;
pub const NEGOTIATE_NEG_NTLM: ULONG = 0x20000000;
STRUCT!{struct NEGOTIATE_PACKAGE_PREFIX_WOW {
    PackageId: ULONG,
    PackageDataA: ULONG,
    PackageDataW: ULONG,
    PrefixLen: ULONG,
    Prefix: [UCHAR; NEGOTIATE_MAX_PREFIX],
}}
pub type PNEGOTIATE_PACKAGE_PREFIX_WOW = *mut NEGOTIATE_PACKAGE_PREFIX_WOW;
STRUCT!{struct NEGOTIATE_CALLER_NAME_RESPONSE_WOW {
    MessageType: ULONG,
    CallerName: ULONG,
}}
pub type PNEGOTIATE_CALLER_NAME_RESPONSE_WOW = *mut NEGOTIATE_CALLER_NAME_RESPONSE_WOW;
extern "system" {
    pub fn LsaSetPolicyReplicationHandle(
        PolicyHandle: PLSA_HANDLE,
    ) -> NTSTATUS;
}
pub const MAX_USER_RECORDS: SIZE_T = 1000;
STRUCT!{struct LSA_USER_REGISTRATION_INFO {
    Sid: LSA_UNICODE_STRING,
    DeviceId: LSA_UNICODE_STRING,
    Username: LSA_UNICODE_STRING,
    Thumbprint: LSA_UNICODE_STRING,
    RegistrationTime: LARGE_INTEGER,
}}
pub type PLSA_USER_REGISTRATION_INFO = *mut LSA_USER_REGISTRATION_INFO;
STRUCT!{struct LSA_REGISTRATION_INFO {
    RegisteredCount: ULONG,
    UserRegistrationInfo: *mut PLSA_USER_REGISTRATION_INFO,
}}
pub type PLSA_REGISTRATION_INFO = *mut LSA_REGISTRATION_INFO;
extern "system" {
    pub fn LsaGetDeviceRegistrationInfo(
        RegistrationInfo: *mut PLSA_REGISTRATION_INFO,
    ) -> NTSTATUS;
}
ENUM!{enum LSA_CREDENTIAL_KEY_SOURCE_TYPE {
    eFromPrecomputed = 1,
    eFromClearPassword,
    eFromNtOwf,
}}
pub type PLSA_CREDENTIAL_KEY_SOURCE_TYPE = *mut LSA_CREDENTIAL_KEY_SOURCE_TYPE;
extern "C" {
    pub fn SeciIsProtectedUser(
        ProtectedUser: PBOOLEAN,
    ) -> NTSTATUS;
}
