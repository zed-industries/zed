// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module defines the Local Security Authority APIs.
use shared::basetsd::{ULONG64, ULONG_PTR};
use shared::guiddef::GUID;
use shared::minwindef::{PUCHAR, PULONG, UCHAR, ULONG, USHORT};
use shared::ntdef::NTSTATUS;
use shared::sspi::SecHandle;
use um::lsalookup::{
    LSA_TRUST_INFORMATION, LSA_UNICODE_STRING, PLSA_TRUST_INFORMATION, PLSA_UNICODE_STRING
};
use um::subauth::{PUNICODE_STRING, STRING, UNICODE_STRING};
use um::winnt::{
    ACCESS_MASK, ANYSIZE_ARRAY, BOOLEAN, HANDLE, LARGE_INTEGER, LONG, LUID, PACL, PCSTR, PCWSTR,
    PSECURITY_DESCRIPTOR, PSID, PSTR, PVOID, PWSTR, QUOTA_LIMITS, SECURITY_INFORMATION, SHORT, SID,
    SID_NAME_USE, STANDARD_RIGHTS_EXECUTE, STANDARD_RIGHTS_READ, STANDARD_RIGHTS_REQUIRED,
    STANDARD_RIGHTS_WRITE, ULONGLONG
};
DEFINE_GUID!{Audit_System_SecurityStateChange,
    0x0cce9210, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_System_SecuritySubsystemExtension,
    0x0cce9211, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_System_Integrity,
    0x0cce9212, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_System_IPSecDriverEvents,
    0x0cce9213, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_System_Others,
    0x0cce9214, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_Logon,
    0x0cce9215, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_Logoff,
    0x0cce9216, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_AccountLockout,
    0x0cce9217, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_IPSecMainMode,
    0x0cce9218, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_IPSecQuickMode,
    0x0cce9219, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_IPSecUserMode,
    0x0cce921a, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_SpecialLogon,
    0x0cce921b, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_Others,
    0x0cce921c, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_FileSystem,
    0x0cce921d, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_Registry,
    0x0cce921e, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_Kernel,
    0x0cce921f, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_Sam,
    0x0cce9220, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_CertificationServices,
    0x0cce9221, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_ApplicationGenerated,
    0x0cce9222, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_Handle,
    0x0cce9223, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_Share,
    0x0cce9224, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_FirewallPacketDrops,
    0x0cce9225, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_FirewallConnection,
    0x0cce9226, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_Other,
    0x0cce9227, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PrivilegeUse_Sensitive,
    0x0cce9228, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PrivilegeUse_NonSensitive,
    0x0cce9229, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PrivilegeUse_Others,
    0x0cce922a, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DetailedTracking_ProcessCreation,
    0x0cce922b, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DetailedTracking_ProcessTermination,
    0x0cce922c, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DetailedTracking_DpapiActivity,
    0x0cce922d, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DetailedTracking_RpcCall,
    0x0cce922e, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PolicyChange_AuditPolicy,
    0x0cce922f, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PolicyChange_AuthenticationPolicy,
    0x0cce9230, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PolicyChange_AuthorizationPolicy,
    0x0cce9231, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PolicyChange_MpsscvRulePolicy,
    0x0cce9232, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PolicyChange_WfpIPSecPolicy,
    0x0cce9233, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PolicyChange_Others,
    0x0cce9234, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountManagement_UserAccount,
    0x0cce9235, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountManagement_ComputerAccount,
    0x0cce9236, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountManagement_SecurityGroup,
    0x0cce9237, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountManagement_DistributionGroup,
    0x0cce9238, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountManagement_ApplicationGroup,
    0x0cce9239, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountManagement_Others,
    0x0cce923a, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DSAccess_DSAccess,
    0x0cce923b, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DsAccess_AdAuditChanges,
    0x0cce923c, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Ds_Replication,
    0x0cce923d, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Ds_DetailedReplication,
    0x0cce923e, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountLogon_CredentialValidation,
    0x0cce923f, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountLogon_Kerberos,
    0x0cce9240, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountLogon_Others,
    0x0cce9241, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountLogon_KerbCredentialValidation,
    0x0cce9242, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_NPS,
    0x0cce9243, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_DetailedFileShare,
    0x0cce9244, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_RemovableStorage,
    0x0cce9245, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess_CbacStaging,
    0x0cce9246, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon_Claims,
    0x0cce9247, 0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_System,
    0x69979848, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_Logon,
    0x69979849, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_ObjectAccess,
    0x6997984a, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PrivilegeUse,
    0x6997984b, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DetailedTracking,
    0x6997984c, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_PolicyChange,
    0x6997984d, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountManagement,
    0x6997984e, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_DirectoryServiceAccess,
    0x6997984f, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
DEFINE_GUID!{Audit_AccountLogon,
    0x69979850, 0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30}
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
pub const POLICY_AUDIT_EVENT_UNCHANGED: POLICY_AUDIT_EVENT_OPTIONS = 0x00000000;
pub const POLICY_AUDIT_EVENT_SUCCESS: POLICY_AUDIT_EVENT_OPTIONS = 0x00000001;
pub const POLICY_AUDIT_EVENT_FAILURE: POLICY_AUDIT_EVENT_OPTIONS = 0x00000002;
pub const POLICY_AUDIT_EVENT_NONE: POLICY_AUDIT_EVENT_OPTIONS = 0x00000004;
pub const POLICY_AUDIT_EVENT_MASK: POLICY_AUDIT_EVENT_OPTIONS = POLICY_AUDIT_EVENT_SUCCESS
    | POLICY_AUDIT_EVENT_FAILURE | POLICY_AUDIT_EVENT_UNCHANGED | POLICY_AUDIT_EVENT_NONE;
pub const POLICY_VIEW_LOCAL_INFORMATION: ACCESS_MASK = 0x00000001;
pub const POLICY_VIEW_AUDIT_INFORMATION: ACCESS_MASK = 0x00000002;
pub const POLICY_GET_PRIVATE_INFORMATION: ACCESS_MASK = 0x00000004;
pub const POLICY_TRUST_ADMIN: ACCESS_MASK = 0x00000008;
pub const POLICY_CREATE_ACCOUNT: ACCESS_MASK = 0x00000010;
pub const POLICY_CREATE_SECRET: ACCESS_MASK = 0x00000020;
pub const POLICY_CREATE_PRIVILEGE: ACCESS_MASK = 0x00000040;
pub const POLICY_SET_DEFAULT_QUOTA_LIMITS: ACCESS_MASK = 0x00000080;
pub const POLICY_SET_AUDIT_REQUIREMENTS: ACCESS_MASK = 0x00000100;
pub const POLICY_AUDIT_LOG_ADMIN: ACCESS_MASK = 0x00000200;
pub const POLICY_SERVER_ADMIN: ACCESS_MASK = 0x00000400;
pub const POLICY_LOOKUP_NAMES: ACCESS_MASK = 0x00000800;
pub const POLICY_NOTIFICATION: ACCESS_MASK = 0x00001000;
pub const POLICY_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED
    | POLICY_VIEW_LOCAL_INFORMATION | POLICY_VIEW_AUDIT_INFORMATION
    | POLICY_GET_PRIVATE_INFORMATION | POLICY_TRUST_ADMIN | POLICY_CREATE_ACCOUNT
    | POLICY_CREATE_SECRET | POLICY_CREATE_PRIVILEGE | POLICY_SET_DEFAULT_QUOTA_LIMITS
    | POLICY_SET_AUDIT_REQUIREMENTS | POLICY_AUDIT_LOG_ADMIN | POLICY_SERVER_ADMIN
    | POLICY_LOOKUP_NAMES;
pub const POLICY_READ: ACCESS_MASK = STANDARD_RIGHTS_READ | POLICY_VIEW_AUDIT_INFORMATION
    | POLICY_GET_PRIVATE_INFORMATION;
pub const POLICY_WRITE: ACCESS_MASK = STANDARD_RIGHTS_WRITE | POLICY_TRUST_ADMIN
    | POLICY_CREATE_ACCOUNT | POLICY_CREATE_SECRET | POLICY_CREATE_PRIVILEGE
    | POLICY_SET_DEFAULT_QUOTA_LIMITS | POLICY_SET_AUDIT_REQUIREMENTS | POLICY_AUDIT_LOG_ADMIN
    | POLICY_SERVER_ADMIN;
pub const POLICY_EXECUTE: ACCESS_MASK = STANDARD_RIGHTS_EXECUTE
    | POLICY_VIEW_LOCAL_INFORMATION | POLICY_LOOKUP_NAMES;
STRUCT!{struct LSA_TRANSLATED_SID {
    Use: SID_NAME_USE,
    RelativeId: ULONG,
    DomainIndex: LONG,
}}
pub type PLSA_TRANSLATED_SID = *mut LSA_TRANSLATED_SID;
ENUM!{enum POLICY_LSA_SERVER_ROLE {
    PolicyServerRoleBackup = 2,
    PolicyServerRolePrimary,
}}
pub type PPOLICY_LSA_SERVER_ROLE = *mut POLICY_LSA_SERVER_ROLE;
pub type POLICY_AUDIT_EVENT_OPTIONS = ULONG;
pub type PPOLICY_AUDIT_EVENT_OPTIONS = *mut ULONG;
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
    MaximumSubCategoryCount: ULONG,
    SubCategoriesInfo: PPOLICY_AUDIT_SUBCATEGORIES_INFO,
}}
pub type PPOLICY_AUDIT_CATEGORIES_INFO = *mut POLICY_AUDIT_CATEGORIES_INFO;
pub const PER_USER_POLICY_UNCHANGED: ULONG = 0x00;
pub const PER_USER_AUDIT_SUCCESS_INCLUDE: ULONG = 0x01;
pub const PER_USER_AUDIT_SUCCESS_EXCLUDE: ULONG = 0x02;
pub const PER_USER_AUDIT_FAILURE_INCLUDE: ULONG = 0x04;
pub const PER_USER_AUDIT_FAILURE_EXCLUDE: ULONG = 0x08;
pub const PER_USER_AUDIT_NONE: ULONG = 0x10;
pub const VALID_PER_USER_AUDIT_POLICY_FLAG: ULONG = PER_USER_AUDIT_SUCCESS_INCLUDE
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
STRUCT!{struct POLICY_DOMAIN_EFS_INFO {
    InfoLength: ULONG,
    EfsBlob: PUCHAR,
}}
pub type PPOLICY_DOMAIN_EFS_INFO = *mut POLICY_DOMAIN_EFS_INFO;
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
pub type LSA_HANDLE = PVOID;
pub type PLSA_HANDLE = *mut PVOID;
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
    ForestTrustRecordTypeLast, // = ForestTrustDomainInfo,
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
pub const MAX_FOREST_TRUST_BINARY_DATA_SIZE: ULONG = 128 * 1024;
STRUCT!{struct LSA_FOREST_TRUST_BINARY_DATA {
    Length: ULONG,
    Buffer: PUCHAR,
}}
pub type PLSA_FOREST_TRUST_BINARY_DATA = *mut LSA_FOREST_TRUST_BINARY_DATA;
UNION!{union LSA_FOREST_TRUST_RECORD_ForestTrustData {
    [usize; 5],
    TopLevelName TopLevelName_mut: LSA_UNICODE_STRING,
    DomainInfo DomainInfo_mut: LSA_FOREST_TRUST_DOMAIN_INFO,
    Data Data_mut: LSA_FOREST_TRUST_BINARY_DATA,
}}
STRUCT!{struct LSA_FOREST_TRUST_RECORD {
    Flags: ULONG,
    ForestTrustType: LSA_FOREST_TRUST_RECORD_TYPE,
    Time: LARGE_INTEGER,
    ForestTrustData: LSA_FOREST_TRUST_RECORD_ForestTrustData,
}}
pub type PLSA_FOREST_TRUST_RECORD = *mut LSA_FOREST_TRUST_RECORD;
pub const MAX_RECORDS_IN_FOREST_TRUST_INFO: ULONG = 4000;
STRUCT!{struct LSA_FOREST_TRUST_INFORMATION {
    RecordCount: ULONG,
    Entries: *mut PLSA_FOREST_TRUST_RECORD,
}}
pub type PLSA_FOREST_TRUST_INFORMATION = *mut LSA_FOREST_TRUST_INFORMATION;
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
pub type LSA_ENUMERATION_HANDLE = ULONG;
pub type PLSA_ENUMERATION_HANDLE = *mut ULONG;
STRUCT!{struct LSA_ENUMERATION_INFORMATION {
    Sid: PSID,
}}
pub type PLSA_ENUMERATION_INFORMATION = *mut LSA_ENUMERATION_INFORMATION;
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
pub const CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG: ULONG = 0x00000001;
pub const CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG: ULONG = 0x00000100;
pub const CENTRAL_ACCESS_POLICY_STAGED_FLAG: ULONG = 0x00010000;
pub const CENTRAL_ACCESS_POLICY_VALID_FLAG_MASK: ULONG =
    CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG
    | CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG
    | CENTRAL_ACCESS_POLICY_STAGED_FLAG;
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
ENUM!{enum NEGOTIATE_MESSAGES {
    NegEnumPackagePrefixes = 0,
    NegGetCallerName = 1,
    NegTransferCredentials = 2,
    NegCallPackageMax,
}}
pub const NEGOTIATE_MAX_PREFIX: usize = 32;
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
    MessageType: ULONG,
    CallerName: PWSTR,
}}
pub type PNEGOTIATE_CALLER_NAME_RESPONSE = *mut NEGOTIATE_CALLER_NAME_RESPONSE;
STRUCT!{struct DOMAIN_PASSWORD_INFORMATION {
    MinPasswordLength: USHORT,
    PasswordHistoryLength: USHORT,
    PasswordProperties: ULONG,
    MaxPasswordAge: LARGE_INTEGER,
    MinPasswordAge: LARGE_INTEGER,
}}
pub type PDOMAIN_PASSWORD_INFORMATION = *mut DOMAIN_PASSWORD_INFORMATION;
pub const DOMAIN_PASSWORD_COMPLEX: ULONG = 0x00000001;
pub const DOMAIN_PASSWORD_NO_ANON_CHANGE: ULONG = 0x00000002;
pub const DOMAIN_PASSWORD_NO_CLEAR_CHANGE: ULONG = 0x00000004;
pub const DOMAIN_LOCKOUT_ADMINS: ULONG = 0x00000008;
pub const DOMAIN_PASSWORD_STORE_CLEARTEXT: ULONG = 0x00000010;
pub const DOMAIN_REFUSE_PASSWORD_CHANGE: ULONG = 0x00000020;
pub const DOMAIN_NO_LM_OWF_CHANGE: ULONG = 0x00000040;
FN!{stdcall PSAM_PASSWORD_NOTIFICATION_ROUTINE(
    UserName: PUNICODE_STRING,
    RelativeId: ULONG,
    NewPassword: PUNICODE_STRING,
) -> NTSTATUS}
FN!{stdcall PSAM_INIT_NOTIFICATION_ROUTINE() -> BOOLEAN}
FN!{stdcall PSAM_PASSWORD_FILTER_ROUTINE(
    AccountName: PUNICODE_STRING,
    FullName: PUNICODE_STRING,
    Password: PUNICODE_STRING,
    SetOperation: BOOLEAN,
) -> BOOLEAN}
ENUM!{enum MSV1_0_LOGON_SUBMIT_TYPE {
    MsV1_0InteractiveLogon = 2,
    MsV1_0Lm20Logon,
    MsV1_0NetworkLogon,
    MsV1_0SubAuthLogon,
    MsV1_0WorkstationUnlockLogon = 7,
    MsV1_0S4ULogon = 12,
    MsV1_0VirtualLogon = 82,
    MsV1_0NoElevationLogon = 83,
    MsV1_0LuidLogon = 84,
}}
pub type PMSV1_0_LOGON_SUBMIT_TYPE = *mut MSV1_0_LOGON_SUBMIT_TYPE;
ENUM!{enum MSV1_0_PROFILE_BUFFER_TYPE {
    MsV1_0InteractiveProfile = 2,
    MsV1_0Lm20LogonProfile,
    MsV1_0SmartCardProfile,
}}
pub type PMSV1_0_PROFILE_BUFFER_TYPE = *mut MSV1_0_PROFILE_BUFFER_TYPE;
STRUCT!{struct MSV1_0_INTERACTIVE_LOGON {
    MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    LogonDomainName: UNICODE_STRING,
    UserName: UNICODE_STRING,
    Password: UNICODE_STRING,
}}
pub type PMSV1_0_INTERACTIVE_LOGON = *mut MSV1_0_INTERACTIVE_LOGON;
STRUCT!{struct MSV1_0_INTERACTIVE_PROFILE {
    MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    LogonCount: USHORT,
    BadPasswordCount: USHORT,
    LogonTime: LARGE_INTEGER,
    LogoffTime: LARGE_INTEGER,
    KickOffTime: LARGE_INTEGER,
    PasswordLastSet: LARGE_INTEGER,
    PasswordCanChange: LARGE_INTEGER,
    PasswordMustChange: LARGE_INTEGER,
    LogonScript: UNICODE_STRING,
    HomeDirectory: UNICODE_STRING,
    FullName: UNICODE_STRING,
    ProfilePath: UNICODE_STRING,
    HomeDirectoryDrive: UNICODE_STRING,
    LogonServer: UNICODE_STRING,
    UserFlags: ULONG,
}}
pub type PMSV1_0_INTERACTIVE_PROFILE = *mut MSV1_0_INTERACTIVE_PROFILE;
pub const MSV1_0_CHALLENGE_LENGTH: usize = 8;
pub const MSV1_0_USER_SESSION_KEY_LENGTH: usize = 16;
pub const MSV1_0_LANMAN_SESSION_KEY_LENGTH: usize = 8;
pub const MSV1_0_CLEARTEXT_PASSWORD_ALLOWED: ULONG = 0x02;
pub const MSV1_0_UPDATE_LOGON_STATISTICS: ULONG = 0x04;
pub const MSV1_0_RETURN_USER_PARAMETERS: ULONG = 0x08;
pub const MSV1_0_DONT_TRY_GUEST_ACCOUNT: ULONG = 0x10;
pub const MSV1_0_ALLOW_SERVER_TRUST_ACCOUNT: ULONG = 0x20;
pub const MSV1_0_RETURN_PASSWORD_EXPIRY: ULONG = 0x40;
pub const MSV1_0_USE_CLIENT_CHALLENGE: ULONG = 0x80;
pub const MSV1_0_TRY_GUEST_ACCOUNT_ONLY: ULONG = 0x100;
pub const MSV1_0_RETURN_PROFILE_PATH: ULONG = 0x200;
pub const MSV1_0_TRY_SPECIFIED_DOMAIN_ONLY: ULONG = 0x400;
pub const MSV1_0_ALLOW_WORKSTATION_TRUST_ACCOUNT: ULONG = 0x800;
pub const MSV1_0_DISABLE_PERSONAL_FALLBACK: ULONG = 0x00001000;
pub const MSV1_0_ALLOW_FORCE_GUEST: ULONG = 0x00002000;
pub const MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED: ULONG = 0x00004000;
pub const MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY: ULONG = 0x00008000;
pub const MSV1_0_SUBAUTHENTICATION_DLL_EX: ULONG = 0x00100000;
pub const MSV1_0_ALLOW_MSVCHAPV2: ULONG = 0x00010000;
pub const MSV1_0_S4U2SELF: ULONG = 0x00020000;
pub const MSV1_0_CHECK_LOGONHOURS_FOR_S4U: ULONG = 0x00040000;
pub const MSV1_0_INTERNET_DOMAIN: ULONG = 0x00080000;
pub const MSV1_0_SUBAUTHENTICATION_DLL: ULONG = 0xFF000000;
pub const MSV1_0_SUBAUTHENTICATION_DLL_SHIFT: ULONG = 24;
pub const MSV1_0_MNS_LOGON: ULONG = 0x01000000;
pub const MSV1_0_SUBAUTHENTICATION_DLL_RAS: ULONG = 2;
pub const MSV1_0_SUBAUTHENTICATION_DLL_IIS: ULONG = 132;
STRUCT!{struct MSV1_0_LM20_LOGON {
    MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    LogonDomainName: UNICODE_STRING,
    UserName: UNICODE_STRING,
    Workstation: UNICODE_STRING,
    ChallengeToClient: [UCHAR; MSV1_0_CHALLENGE_LENGTH],
    CaseSensitiveChallengeResponse: STRING,
    CaseInsensitiveChallengeResponse: STRING,
    ParameterControl: ULONG,
}}
pub type PMSV1_0_LM20_LOGON = *mut MSV1_0_LM20_LOGON;
STRUCT!{struct MSV1_0_SUBAUTH_LOGON {
    MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    LogonDomainName: UNICODE_STRING,
    UserName: UNICODE_STRING,
    Workstation: UNICODE_STRING,
    ChallengeToClient: [UCHAR; MSV1_0_CHALLENGE_LENGTH],
    AuthenticationInfo1: STRING,
    AuthenticationInfo2: STRING,
    ParameterControl: ULONG,
    SubAuthPackageId: ULONG,
}}
pub type PMSV1_0_SUBAUTH_LOGON = *mut MSV1_0_SUBAUTH_LOGON;
STRUCT!{struct MSV1_0_S4U_LOGON {
    MessageType: MSV1_0_LOGON_SUBMIT_TYPE,
    MSV1_0_LOGON_SUBMIT_TYPE: ULONG,
    UserPrincipalName: UNICODE_STRING,
    DomainName: UNICODE_STRING,
}}
pub type PMSV1_0_S4U_LOGON = *mut MSV1_0_S4U_LOGON;
pub const LOGON_GUEST: ULONG = 0x01;
pub const LOGON_NOENCRYPTION: ULONG = 0x02;
pub const LOGON_CACHED_ACCOUNT: ULONG = 0x04;
pub const LOGON_USED_LM_PASSWORD: ULONG = 0x08;
pub const LOGON_EXTRA_SIDS: ULONG = 0x20;
pub const LOGON_SUBAUTH_SESSION_KEY: ULONG = 0x40;
pub const LOGON_SERVER_TRUST_ACCOUNT: ULONG = 0x80;
pub const LOGON_NTLMV2_ENABLED: ULONG = 0x100;
pub const LOGON_RESOURCE_GROUPS: ULONG = 0x200;
pub const LOGON_PROFILE_PATH_RETURNED: ULONG = 0x400;
pub const LOGON_NT_V2: ULONG = 0x800;
pub const LOGON_LM_V2: ULONG = 0x1000;
pub const LOGON_NTLM_V2: ULONG = 0x2000;
pub const LOGON_OPTIMIZED: ULONG = 0x4000;
pub const LOGON_WINLOGON: ULONG = 0x8000;
pub const LOGON_PKINIT: ULONG = 0x10000;
pub const LOGON_NO_OPTIMIZED: ULONG = 0x20000;
pub const LOGON_NO_ELEVATION: ULONG = 0x40000;
pub const LOGON_MANAGED_SERVICE: ULONG = 0x80000;
pub const LOGON_GRACE_LOGON: ULONG = 0x01000000;
STRUCT!{struct MSV1_0_LM20_LOGON_PROFILE {
    MessageType: MSV1_0_PROFILE_BUFFER_TYPE,
    KickOffTime: LARGE_INTEGER,
    LogoffTime: LARGE_INTEGER,
    UserFlags: ULONG,
    UserSessionKey: [UCHAR; MSV1_0_USER_SESSION_KEY_LENGTH],
    LogonDomainName: UNICODE_STRING,
    LanmanSessionKey: [UCHAR; MSV1_0_LANMAN_SESSION_KEY_LENGTH],
    LogonServer: UNICODE_STRING,
    UserParameters: UNICODE_STRING,
}}
pub type PMSV1_0_LM20_LOGON_PROFILE = *mut MSV1_0_LM20_LOGON_PROFILE;
pub const MSV1_0_OWF_PASSWORD_LENGTH: usize = 16;
STRUCT!{struct MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    Version: ULONG,
    Flags: ULONG,
    LmPassword: [UCHAR; MSV1_0_OWF_PASSWORD_LENGTH],
    NtPassword: [UCHAR; MSV1_0_OWF_PASSWORD_LENGTH],
}}
pub type PMSV1_0_SUPPLEMENTAL_CREDENTIAL = *mut MSV1_0_SUPPLEMENTAL_CREDENTIAL;
pub const MSV1_0_NTLM3_RESPONSE_LENGTH: usize = 16;
pub const MSV1_0_NTLM3_OWF_LENGTH: usize = 16;
STRUCT!{struct MSV1_0_NTLM3_RESPONSE {
    Response: [UCHAR; MSV1_0_NTLM3_RESPONSE_LENGTH],
    RespType: UCHAR,
    HiRespType: UCHAR,
    Flags: USHORT,
    MsgWord: ULONG,
    TimeStamp: ULONGLONG,
    ChallengeFromClient: [UCHAR; MSV1_0_CHALLENGE_LENGTH],
    AvPairsOff: ULONG,
    Buffer: [UCHAR; 1],
}}
pub type PMSV1_0_NTLM3_RESPONSE = *mut MSV1_0_NTLM3_RESPONSE;
ENUM!{enum MSV1_0_AVID {
    MsvAvEOL,
    MsvAvNbComputerName,
    MsvAvNbDomainName,
    MsvAvDnsComputerName,
    MsvAvDnsDomainName,
    MsvAvDnsTreeName,
    MsvAvFlags,
    MsvAvTimestamp,
    MsvAvRestrictions,
    MsvAvTargetName,
    MsvAvChannelBindings,
}}
STRUCT!{struct MSV1_0_AV_PAIR {
    AvId: USHORT,
    AvLen: USHORT,
}}
pub type PMSV1_0_AV_PAIR = *mut MSV1_0_AV_PAIR;
ENUM!{enum MSV1_0_PROTOCOL_MESSAGE_TYPE {
    MsV1_0Lm20ChallengeRequest = 0,
    MsV1_0Lm20GetChallengeResponse,
    MsV1_0EnumerateUsers,
    MsV1_0GetUserInfo,
    MsV1_0ReLogonUsers,
    MsV1_0ChangePassword,
    MsV1_0ChangeCachedPassword,
    MsV1_0GenericPassthrough,
    MsV1_0CacheLogon,
    MsV1_0SubAuth,
    MsV1_0DeriveCredential,
    MsV1_0CacheLookup,
    MsV1_0SetProcessOption,
    MsV1_0ConfigLocalAliases,
    MsV1_0ClearCachedCredentials,
    MsV1_0LookupToken,
    MsV1_0ValidateAuth,
    MsV1_0CacheLookupEx,
    MsV1_0GetCredentialKey,
    MsV1_0SetThreadOption,
}}
pub type PMSV1_0_PROTOCOL_MESSAGE_TYPE = *mut MSV1_0_PROTOCOL_MESSAGE_TYPE;
STRUCT!{struct MSV1_0_CHANGEPASSWORD_REQUEST {
    MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    DomainName: UNICODE_STRING,
    AccountName: UNICODE_STRING,
    OldPassword: UNICODE_STRING,
    NewPassword: UNICODE_STRING,
    Impersonating: BOOLEAN,
}}
pub type PMSV1_0_CHANGEPASSWORD_REQUEST = *mut MSV1_0_CHANGEPASSWORD_REQUEST;
STRUCT!{struct MSV1_0_CHANGEPASSWORD_RESPONSE {
    MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    PasswordInfoValid: BOOLEAN,
    DomainPasswordInfo: DOMAIN_PASSWORD_INFORMATION,
}}
pub type PMSV1_0_CHANGEPASSWORD_RESPONSE = *mut MSV1_0_CHANGEPASSWORD_RESPONSE;
STRUCT!{struct MSV1_0_PASSTHROUGH_REQUEST {
    MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    DomainName: UNICODE_STRING,
    PackageName: UNICODE_STRING,
    DataLength: ULONG,
    LogonData: PUCHAR,
    Pad: ULONG,
}}
pub type PMSV1_0_PASSTHROUGH_REQUEST = *mut MSV1_0_PASSTHROUGH_REQUEST;
STRUCT!{struct MSV1_0_PASSTHROUGH_RESPONSE {
    MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    Pad: ULONG,
    DataLength: ULONG,
    ValidationData: PUCHAR,
}}
pub type PMSV1_0_PASSTHROUGH_RESPONSE = *mut MSV1_0_PASSTHROUGH_RESPONSE;
STRUCT!{struct MSV1_0_SUBAUTH_REQUEST {
    MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    SubAuthPackageId: ULONG,
    SubAuthInfoLength: ULONG,
    SubAuthSubmitBuffer: PUCHAR,
}}
pub type PMSV1_0_SUBAUTH_REQUEST = *mut MSV1_0_SUBAUTH_REQUEST;
STRUCT!{struct MSV1_0_SUBAUTH_RESPONSE {
    MessageType: MSV1_0_PROTOCOL_MESSAGE_TYPE,
    SubAuthInfoLength: ULONG,
    SubAuthReturnBuffer: PUCHAR,
}}
pub type PMSV1_0_SUBAUTH_RESPONSE = *mut MSV1_0_SUBAUTH_RESPONSE;
pub use self::SystemFunction036 as RtlGenRandom;
pub use self::SystemFunction040 as RtlEncryptMemory;
pub use self::SystemFunction041 as RtlDecryptMemory;
extern "system" {
    pub fn SystemFunction036(
        RandomBuffer: PVOID,
        RandomBufferLength: ULONG,
    ) -> BOOLEAN;
}
pub const RTL_ENCRYPT_MEMORY_SIZE: ULONG = 8;
pub const RTL_ENCRYPT_OPTION_CROSS_PROCESS: ULONG = 0x01;
pub const RTL_ENCRYPT_OPTION_SAME_LOGON: ULONG = 0x02;
extern "system" {
    pub fn SystemFunction040(
        Memory: PVOID,
        MemorySize: ULONG,
        OptionFlags: ULONG,
    ) -> NTSTATUS;
    pub fn SystemFunction041(
        Memory: PVOID,
        MemorySize: ULONG,
        OptionFlags: ULONG,
    ) -> NTSTATUS;
}
pub const KERBEROS_VERSION: ULONG = 5;
pub const KERBEROS_REVISION: ULONG = 6;
pub const KERB_ETYPE_NULL: LONG = 0;
pub const KERB_ETYPE_DES_CBC_CRC: LONG = 1;
pub const KERB_ETYPE_DES_CBC_MD4: LONG = 2;
pub const KERB_ETYPE_DES_CBC_MD5: LONG = 3;
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96: LONG = 17;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96: LONG = 18;
pub const KERB_ETYPE_RC4_MD4: LONG = -128;
pub const KERB_ETYPE_RC4_PLAIN2: LONG = -129;
pub const KERB_ETYPE_RC4_LM: LONG = -130;
pub const KERB_ETYPE_RC4_SHA: LONG = -131;
pub const KERB_ETYPE_DES_PLAIN: LONG = -132;
pub const KERB_ETYPE_RC4_HMAC_OLD: LONG = -133;
pub const KERB_ETYPE_RC4_PLAIN_OLD: LONG = -134;
pub const KERB_ETYPE_RC4_HMAC_OLD_EXP: LONG = -135;
pub const KERB_ETYPE_RC4_PLAIN_OLD_EXP: LONG = -136;
pub const KERB_ETYPE_RC4_PLAIN: LONG = -140;
pub const KERB_ETYPE_RC4_PLAIN_EXP: LONG = -141;
pub const KERB_ETYPE_AES128_CTS_HMAC_SHA1_96_PLAIN: LONG = -148;
pub const KERB_ETYPE_AES256_CTS_HMAC_SHA1_96_PLAIN: LONG = -149;
pub const KERB_ETYPE_DSA_SHA1_CMS: LONG = 9;
pub const KERB_ETYPE_RSA_MD5_CMS: LONG = 10;
pub const KERB_ETYPE_RSA_SHA1_CMS: LONG = 11;
pub const KERB_ETYPE_RC2_CBC_ENV: LONG = 12;
pub const KERB_ETYPE_RSA_ENV: LONG = 13;
pub const KERB_ETYPE_RSA_ES_OEAP_ENV: LONG = 14;
pub const KERB_ETYPE_DES_EDE3_CBC_ENV: LONG = 15;
pub const KERB_ETYPE_DSA_SIGN: LONG = 8;
pub const KERB_ETYPE_RSA_PRIV: LONG = 9;
pub const KERB_ETYPE_RSA_PUB: LONG = 10;
pub const KERB_ETYPE_RSA_PUB_MD5: LONG = 11;
pub const KERB_ETYPE_RSA_PUB_SHA1: LONG = 12;
pub const KERB_ETYPE_PKCS7_PUB: LONG = 13;
pub const KERB_ETYPE_DES3_CBC_MD5: LONG = 5;
pub const KERB_ETYPE_DES3_CBC_SHA1: LONG = 7;
pub const KERB_ETYPE_DES3_CBC_SHA1_KD: LONG = 16;
pub const KERB_ETYPE_DES_CBC_MD5_NT: LONG = 20;
pub const KERB_ETYPE_RC4_HMAC_NT: LONG = 23;
pub const KERB_ETYPE_RC4_HMAC_NT_EXP: LONG = 24;
pub const KERB_CHECKSUM_NONE: LONG = 0;
pub const KERB_CHECKSUM_CRC32: LONG = 1;
pub const KERB_CHECKSUM_MD4: LONG = 2;
pub const KERB_CHECKSUM_KRB_DES_MAC: LONG = 4;
pub const KERB_CHECKSUM_KRB_DES_MAC_K: LONG = 5;
pub const KERB_CHECKSUM_MD5: LONG = 7;
pub const KERB_CHECKSUM_MD5_DES: LONG = 8;
pub const KERB_CHECKSUM_SHA1_NEW: LONG = 14;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128: LONG = 15;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256: LONG = 16;
pub const KERB_CHECKSUM_LM: LONG = -130;
pub const KERB_CHECKSUM_SHA1: LONG = -131;
pub const KERB_CHECKSUM_REAL_CRC32: LONG = -132;
pub const KERB_CHECKSUM_DES_MAC: LONG = -133;
pub const KERB_CHECKSUM_DES_MAC_MD5: LONG = -134;
pub const KERB_CHECKSUM_MD25: LONG = -135;
pub const KERB_CHECKSUM_RC4_MD5: LONG = -136;
pub const KERB_CHECKSUM_MD5_HMAC: LONG = -137;
pub const KERB_CHECKSUM_HMAC_MD5: LONG = -138;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES128_Ki: LONG = -150;
pub const KERB_CHECKSUM_HMAC_SHA1_96_AES256_Ki: LONG = -151;
pub const KERB_TICKET_FLAGS_reserved: ULONG = 0x80000000;
pub const KERB_TICKET_FLAGS_forwardable: ULONG = 0x40000000;
pub const KERB_TICKET_FLAGS_forwarded: ULONG = 0x20000000;
pub const KERB_TICKET_FLAGS_proxiable: ULONG = 0x10000000;
pub const KERB_TICKET_FLAGS_proxy: ULONG = 0x08000000;
pub const KERB_TICKET_FLAGS_may_postdate: ULONG = 0x04000000;
pub const KERB_TICKET_FLAGS_postdated: ULONG = 0x02000000;
pub const KERB_TICKET_FLAGS_invalid: ULONG = 0x01000000;
pub const KERB_TICKET_FLAGS_renewable: ULONG = 0x00800000;
pub const KERB_TICKET_FLAGS_initial: ULONG = 0x00400000;
pub const KERB_TICKET_FLAGS_pre_authent: ULONG = 0x00200000;
pub const KERB_TICKET_FLAGS_hw_authent: ULONG = 0x00100000;
pub const KERB_TICKET_FLAGS_ok_as_delegate: ULONG = 0x00040000;
pub const KERB_TICKET_FLAGS_name_canonicalize: ULONG = 0x00010000;
pub const KERB_TICKET_FLAGS_cname_in_pa_data: ULONG = 0x00040000;
pub const KERB_TICKET_FLAGS_enc_pa_rep: ULONG = 0x00010000;
pub const KERB_TICKET_FLAGS_reserved1: ULONG = 0x00000001;
pub const KRB_NT_UNKNOWN: LONG = 0;
pub const KRB_NT_PRINCIPAL: LONG = 1;
pub const KRB_NT_PRINCIPAL_AND_ID: LONG = -131;
pub const KRB_NT_SRV_INST: LONG = 2;
pub const KRB_NT_SRV_INST_AND_ID: LONG = -132;
pub const KRB_NT_SRV_HST: LONG = 3;
pub const KRB_NT_SRV_XHST: LONG = 4;
pub const KRB_NT_UID: LONG = 5;
pub const KRB_NT_ENTERPRISE_PRINCIPAL: LONG = 10;
pub const KRB_NT_WELLKNOWN: LONG = 11;
pub const KRB_NT_ENT_PRINCIPAL_AND_ID: LONG = -130;
pub const KRB_NT_MS_PRINCIPAL: LONG = -128;
pub const KRB_NT_MS_PRINCIPAL_AND_ID: LONG = -129;
pub const KRB_NT_MS_BRANCH_ID: LONG = -133;
pub const KRB_NT_X500_PRINCIPAL: LONG = 6;
pub const KERB_WRAP_NO_ENCRYPT: ULONG = 0x80000001;
ENUM!{enum KERB_LOGON_SUBMIT_TYPE {
    KerbInteractiveLogon = 2,
    KerbSmartCardLogon = 6,
    KerbWorkstationUnlockLogon = 7,
    KerbSmartCardUnlockLogon = 8,
    KerbProxyLogon = 9,
    KerbTicketLogon = 10,
    KerbTicketUnlockLogon = 11,
    KerbS4ULogon = 12,
    KerbCertificateLogon = 13,
    KerbCertificateS4ULogon = 14,
    KerbCertificateUnlockLogon = 15,
    KerbNoElevationLogon = 83,
    KerbLuidLogon = 84,
}}
pub type PKERB_LOGON_SUBMIT_TYPE = *mut KERB_LOGON_SUBMIT_TYPE;
STRUCT!{struct KERB_INTERACTIVE_LOGON {
    MessageType: KERB_LOGON_SUBMIT_TYPE,
    LogonDomainName: UNICODE_STRING,
    UserName: UNICODE_STRING,
    Password: UNICODE_STRING,
}}
pub type PKERB_INTERACTIVE_LOGON = *mut KERB_INTERACTIVE_LOGON;
STRUCT!{struct KERB_INTERACTIVE_UNLOCK_LOGON {
    Logon: KERB_INTERACTIVE_LOGON,
    LogonId: LUID,
}}
pub type PKERB_INTERACTIVE_UNLOCK_LOGON = *mut KERB_INTERACTIVE_UNLOCK_LOGON;
STRUCT!{struct KERB_SMART_CARD_LOGON {
    MessageType: KERB_LOGON_SUBMIT_TYPE,
    Pin: UNICODE_STRING,
    CspDataLength: ULONG,
    CspData: PUCHAR,
}}
pub type PKERB_SMART_CARD_LOGON = *mut KERB_SMART_CARD_LOGON;
STRUCT!{struct KERB_SMART_CARD_UNLOCK_LOGON {
    Logon: KERB_SMART_CARD_LOGON,
    LogonId: LUID,
}}
pub type PKERB_SMART_CARD_UNLOCK_LOGON = *mut KERB_SMART_CARD_UNLOCK_LOGON;
pub const KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES: ULONG = 0x1;
pub const KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO: ULONG = 0x2;
STRUCT!{struct KERB_CERTIFICATE_LOGON {
    MessageType: KERB_LOGON_SUBMIT_TYPE,
    DomainName: UNICODE_STRING,
    UserName: UNICODE_STRING,
    Pin: UNICODE_STRING,
    Flags: ULONG,
    CspDataLength: ULONG,
    CspData: PUCHAR,
}}
pub type PKERB_CERTIFICATE_LOGON = *mut KERB_CERTIFICATE_LOGON;
STRUCT!{struct KERB_CERTIFICATE_UNLOCK_LOGON {
    Logon: KERB_CERTIFICATE_LOGON,
    LogonId: LUID,
}}
pub type PKERB_CERTIFICATE_UNLOCK_LOGON = *mut KERB_CERTIFICATE_UNLOCK_LOGON;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_DUPLICATES: ULONG = 0x1;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_LOGONHOURS: ULONG = 0x2;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_FAIL_IF_NT_AUTH_POLICY_REQUIRED: ULONG = 0x4;
pub const KERB_CERTIFICATE_S4U_LOGON_FLAG_IDENTIFY: ULONG = 0x8;
STRUCT!{struct KERB_CERTIFICATE_S4U_LOGON {
    MessageType: KERB_LOGON_SUBMIT_TYPE,
    Flags: ULONG,
    UserPrincipalName: UNICODE_STRING,
    DomainName: UNICODE_STRING,
    CertificateLength: ULONG,
    Certificate: PUCHAR,
}}
pub type PKERB_CERTIFICATE_S4U_LOGON = *mut KERB_CERTIFICATE_S4U_LOGON;
STRUCT!{struct KERB_TICKET_LOGON {
    MessageType: KERB_LOGON_SUBMIT_TYPE,
    Flags: ULONG,
    ServiceTicketLength: ULONG,
    TicketGrantingTicketLength: ULONG,
    ServiceTicket: PUCHAR,
    TicketGrantingTicket: PUCHAR,
}}
pub type PKERB_TICKET_LOGON = *mut KERB_TICKET_LOGON;
STRUCT!{struct KERB_TICKET_UNLOCK_LOGON {
    Logon: KERB_TICKET_LOGON,
    LogonId: LUID,
}}
pub type PKERB_TICKET_UNLOCK_LOGON = *mut KERB_TICKET_UNLOCK_LOGON;
pub const KERB_S4U_LOGON_FLAG_CHECK_LOGONHOURS: ULONG = 0x2;
pub const KERB_S4U_LOGON_FLAG_IDENTIFY: ULONG = 0x8;
STRUCT!{struct KERB_S4U_LOGON {
    MessageType: KERB_LOGON_SUBMIT_TYPE,
    Flags: ULONG,
    ClientUpn: UNICODE_STRING,
    ClientRealm: UNICODE_STRING,
}}
pub type PKERB_S4U_LOGON = *mut KERB_S4U_LOGON;
ENUM!{enum KERB_PROFILE_BUFFER_TYPE {
    KerbInteractiveProfile = 2,
    KerbSmartCardProfile = 4,
    KerbTicketProfile = 6,
}}
pub type PKERB_PROFILE_BUFFER_TYPE = *mut KERB_PROFILE_BUFFER_TYPE;
STRUCT!{struct KERB_INTERACTIVE_PROFILE {
    MessageType: KERB_PROFILE_BUFFER_TYPE,
    LogonCount: USHORT,
    BadPasswordCount: USHORT,
    LogonTime: LARGE_INTEGER,
    LogoffTime: LARGE_INTEGER,
    KickOffTime: LARGE_INTEGER,
    PasswordLastSet: LARGE_INTEGER,
    PasswordCanChange: LARGE_INTEGER,
    PasswordMustChange: LARGE_INTEGER,
    LogonScript: UNICODE_STRING,
    HomeDirectory: UNICODE_STRING,
    FullName: UNICODE_STRING,
    ProfilePath: UNICODE_STRING,
    HomeDirectoryDrive: UNICODE_STRING,
    LogonServer: UNICODE_STRING,
    UserFlags: ULONG,
}}
pub type PKERB_INTERACTIVE_PROFILE = *mut KERB_INTERACTIVE_PROFILE;
STRUCT!{struct KERB_SMART_CARD_PROFILE {
    Profile: KERB_INTERACTIVE_PROFILE,
    CertificateSize: ULONG,
    CertificateData: PUCHAR,
}}
pub type PKERB_SMART_CARD_PROFILE = *mut KERB_SMART_CARD_PROFILE;
STRUCT!{struct KERB_CRYPTO_KEY {
    KeyType: LONG,
    Length: ULONG,
    Value: PUCHAR,
}}
pub type PKERB_CRYPTO_KEY = *mut KERB_CRYPTO_KEY;
STRUCT!{struct KERB_CRYPTO_KEY32 {
    KeyType: LONG,
    Length: ULONG,
    Offset: ULONG,
}}
pub type PKERB_CRYPTO_KEY32 = *mut KERB_CRYPTO_KEY32;
STRUCT!{struct KERB_TICKET_PROFILE {
    Profile: KERB_INTERACTIVE_PROFILE,
    SessionKey: KERB_CRYPTO_KEY,
}}
pub type PKERB_TICKET_PROFILE = *mut KERB_TICKET_PROFILE;
ENUM!{enum KERB_PROTOCOL_MESSAGE_TYPE {
    KerbDebugRequestMessage = 0,
    KerbQueryTicketCacheMessage,
    KerbChangeMachinePasswordMessage,
    KerbVerifyPacMessage,
    KerbRetrieveTicketMessage,
    KerbUpdateAddressesMessage,
    KerbPurgeTicketCacheMessage,
    KerbChangePasswordMessage,
    KerbRetrieveEncodedTicketMessage,
    KerbDecryptDataMessage,
    KerbAddBindingCacheEntryMessage,
    KerbSetPasswordMessage,
    KerbSetPasswordExMessage,
    KerbVerifyCredentialsMessage,
    KerbQueryTicketCacheExMessage,
    KerbPurgeTicketCacheExMessage,
    KerbRefreshSmartcardCredentialsMessage,
    KerbAddExtraCredentialsMessage,
    KerbQuerySupplementalCredentialsMessage,
    KerbTransferCredentialsMessage,
    KerbQueryTicketCacheEx2Message,
    KerbSubmitTicketMessage,
    KerbAddExtraCredentialsExMessage,
    KerbQueryKdcProxyCacheMessage,
    KerbPurgeKdcProxyCacheMessage,
    KerbQueryTicketCacheEx3Message,
    KerbCleanupMachinePkinitCredsMessage,
    KerbAddBindingCacheEntryExMessage,
    KerbQueryBindingCacheMessage,
    KerbPurgeBindingCacheMessage,
    KerbPinKdcMessage,
    KerbUnpinAllKdcsMessage,
    KerbQueryDomainExtendedPoliciesMessage,
    KerbQueryS4U2ProxyCacheMessage,
}}
pub type PKERB_PROTOCOL_MESSAGE_TYPE = *mut KERB_PROTOCOL_MESSAGE_TYPE;
STRUCT!{struct KERB_QUERY_TKT_CACHE_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
}}
pub type PKERB_QUERY_TKT_CACHE_REQUEST = *mut KERB_QUERY_TKT_CACHE_REQUEST;
STRUCT!{struct KERB_TICKET_CACHE_INFO {
    ServerName: UNICODE_STRING,
    RealmName: UNICODE_STRING,
    StartTime: LARGE_INTEGER,
    EndTime: LARGE_INTEGER,
    RenewTime: LARGE_INTEGER,
    EncryptionType: LONG,
    TicketFlags: ULONG,
}}
pub type PKERB_TICKET_CACHE_INFO = *mut KERB_TICKET_CACHE_INFO;
STRUCT!{struct KERB_TICKET_CACHE_INFO_EX {
    ClientName: UNICODE_STRING,
    ClientRealm: UNICODE_STRING,
    ServerName: UNICODE_STRING,
    ServerRealm: UNICODE_STRING,
    StartTime: LARGE_INTEGER,
    EndTime: LARGE_INTEGER,
    RenewTime: LARGE_INTEGER,
    EncryptionType: LONG,
    TicketFlags: ULONG,
}}
pub type PKERB_TICKET_CACHE_INFO_EX = *mut KERB_TICKET_CACHE_INFO_EX;
STRUCT!{struct KERB_TICKET_CACHE_INFO_EX2 {
    ClientName: UNICODE_STRING,
    ClientRealm: UNICODE_STRING,
    ServerName: UNICODE_STRING,
    ServerRealm: UNICODE_STRING,
    StartTime: LARGE_INTEGER,
    EndTime: LARGE_INTEGER,
    RenewTime: LARGE_INTEGER,
    EncryptionType: LONG,
    TicketFlags: ULONG,
    SessionKeyType: ULONG,
    BranchId: ULONG,
}}
pub type PKERB_TICKET_CACHE_INFO_EX2 = *mut KERB_TICKET_CACHE_INFO_EX2;
STRUCT!{struct KERB_TICKET_CACHE_INFO_EX3 {
    ClientName: UNICODE_STRING,
    ClientRealm: UNICODE_STRING,
    ServerName: UNICODE_STRING,
    ServerRealm: UNICODE_STRING,
    StartTime: LARGE_INTEGER,
    EndTime: LARGE_INTEGER,
    RenewTime: LARGE_INTEGER,
    EncryptionType: LONG,
    TicketFlags: ULONG,
    SessionKeyType: ULONG,
    BranchId: ULONG,
    CacheFlags: ULONG,
    KdcCalled: UNICODE_STRING,
}}
pub type PKERB_TICKET_CACHE_INFO_EX3 = *mut KERB_TICKET_CACHE_INFO_EX3;
STRUCT!{struct KERB_QUERY_TKT_CACHE_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfTickets: ULONG,
    Tickets: [KERB_TICKET_CACHE_INFO; ANYSIZE_ARRAY],
}}
pub type PKERB_QUERY_TKT_CACHE_RESPONSE = *mut KERB_QUERY_TKT_CACHE_RESPONSE;
STRUCT!{struct KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfTickets: ULONG,
    Tickets: [KERB_TICKET_CACHE_INFO_EX; ANYSIZE_ARRAY],
}}
pub type PKERB_QUERY_TKT_CACHE_EX_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX_RESPONSE;
STRUCT!{struct KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfTickets: ULONG,
    Tickets: [KERB_TICKET_CACHE_INFO_EX2; ANYSIZE_ARRAY],
}}
pub type PKERB_QUERY_TKT_CACHE_EX2_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX2_RESPONSE;
STRUCT!{struct KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfTickets: ULONG,
    Tickets: [KERB_TICKET_CACHE_INFO_EX3; ANYSIZE_ARRAY],
}}
pub type PKERB_QUERY_TKT_CACHE_EX3_RESPONSE = *mut KERB_QUERY_TKT_CACHE_EX3_RESPONSE;
pub const KERB_USE_DEFAULT_TICKET_FLAGS: ULONG = 0x0;
pub const KERB_RETRIEVE_TICKET_DEFAULT: ULONG = 0x0;
pub const KERB_RETRIEVE_TICKET_DONT_USE_CACHE: ULONG = 0x1;
pub const KERB_RETRIEVE_TICKET_USE_CACHE_ONLY: ULONG = 0x2;
pub const KERB_RETRIEVE_TICKET_USE_CREDHANDLE: ULONG = 0x4;
pub const KERB_RETRIEVE_TICKET_AS_KERB_CRED: ULONG = 0x8;
pub const KERB_RETRIEVE_TICKET_WITH_SEC_CRED: ULONG = 0x10;
pub const KERB_RETRIEVE_TICKET_CACHE_TICKET: ULONG = 0x20;
pub const KERB_RETRIEVE_TICKET_MAX_LIFETIME: ULONG = 0x40;
STRUCT!{struct KERB_AUTH_DATA {
    Type: ULONG,
    Length: ULONG,
    Data: PUCHAR,
}}
pub type PKERB_AUTH_DATA = *mut KERB_AUTH_DATA;
STRUCT!{struct KERB_NET_ADDRESS {
    Family: ULONG,
    Length: ULONG,
    Address: PUCHAR,
}}
pub type PKERB_NET_ADDRESS = *mut KERB_NET_ADDRESS;
STRUCT!{struct KERB_NET_ADDRESSES {
    Number: ULONG,
    Addresses: [KERB_NET_ADDRESS; ANYSIZE_ARRAY],
}}
pub type PKERB_NET_ADDRESSES = *mut KERB_NET_ADDRESSES;
STRUCT!{struct KERB_EXTERNAL_NAME {
    NameType: SHORT,
    NameCount: USHORT,
    Names: [UNICODE_STRING; ANYSIZE_ARRAY],
}}
pub type PKERB_EXTERNAL_NAME = *mut KERB_EXTERNAL_NAME;
STRUCT!{struct KERB_EXTERNAL_TICKET {
    ServiceName: PKERB_EXTERNAL_NAME,
    TargetName: PKERB_EXTERNAL_NAME,
    ClientName: PKERB_EXTERNAL_NAME,
    DomainName: UNICODE_STRING,
    TargetDomainName: UNICODE_STRING,
    AltTargetDomainName: UNICODE_STRING,
    SessionKey: KERB_CRYPTO_KEY,
    TicketFlags: ULONG,
    Flags: ULONG,
    KeyExpirationTime: LARGE_INTEGER,
    StartTime: LARGE_INTEGER,
    EndTime: LARGE_INTEGER,
    RenewUntil: LARGE_INTEGER,
    TimeSkew: LARGE_INTEGER,
    EncodedTicketSize: ULONG,
    EncodedTicket: PUCHAR,
}}
pub type PKERB_EXTERNAL_TICKET = *mut KERB_EXTERNAL_TICKET;
STRUCT!{struct KERB_RETRIEVE_TKT_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
    TargetName: UNICODE_STRING,
    TicketFlags: ULONG,
    CacheOptions: ULONG,
    EncryptionType: LONG,
    CredentialsHandle: SecHandle,
}}
pub type PKERB_RETRIEVE_TKT_REQUEST = *mut KERB_RETRIEVE_TKT_REQUEST;
STRUCT!{struct KERB_RETRIEVE_TKT_RESPONSE {
    Ticket: KERB_EXTERNAL_TICKET,
}}
pub type PKERB_RETRIEVE_TKT_RESPONSE = *mut KERB_RETRIEVE_TKT_RESPONSE;
STRUCT!{struct KERB_PURGE_TKT_CACHE_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
    ServerName: UNICODE_STRING,
    RealmName: UNICODE_STRING,
}}
pub type PKERB_PURGE_TKT_CACHE_REQUEST = *mut KERB_PURGE_TKT_CACHE_REQUEST;
pub const KERB_PURGE_ALL_TICKETS: ULONG = 1;
STRUCT!{struct KERB_PURGE_TKT_CACHE_EX_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
    Flags: ULONG,
    TicketTemplate: KERB_TICKET_CACHE_INFO_EX,
}}
pub type PKERB_PURGE_TKT_CACHE_EX_REQUEST = *mut KERB_PURGE_TKT_CACHE_EX_REQUEST;
STRUCT!{struct KERB_SUBMIT_TKT_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
    Flags: ULONG,
    Key: KERB_CRYPTO_KEY32,
    KerbCredSize: ULONG,
    KerbCredOffset: ULONG,
}}
pub type PKERB_SUBMIT_TKT_REQUEST = *mut KERB_SUBMIT_TKT_REQUEST;
STRUCT!{struct KERB_QUERY_KDC_PROXY_CACHE_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    Flags: ULONG,
    LogonId: LUID,
}}
pub type PKERB_QUERY_KDC_PROXY_CACHE_REQUEST = *mut KERB_QUERY_KDC_PROXY_CACHE_REQUEST;
STRUCT!{struct KDC_PROXY_CACHE_ENTRY_DATA {
    SinceLastUsed: ULONG64,
    DomainName: UNICODE_STRING,
    ProxyServerName: UNICODE_STRING,
    ProxyServerVdir: UNICODE_STRING,
    ProxyServerPort: USHORT,
    LogonId: LUID,
    CredUserName: UNICODE_STRING,
    CredDomainName: UNICODE_STRING,
    GlobalCache: BOOLEAN,
}}
pub type PKDC_PROXY_CACHE_ENTRY_DATA = *mut KDC_PROXY_CACHE_ENTRY_DATA;
STRUCT!{struct KERB_QUERY_KDC_PROXY_CACHE_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfEntries: ULONG,
    Entries: PKDC_PROXY_CACHE_ENTRY_DATA,
}}
pub type PKERB_QUERY_KDC_PROXY_CACHE_RESPONSE = *mut KERB_QUERY_KDC_PROXY_CACHE_RESPONSE;
STRUCT!{struct KERB_PURGE_KDC_PROXY_CACHE_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    Flags: ULONG,
    LogonId: LUID,
}}
pub type PKERB_PURGE_KDC_PROXY_CACHE_REQUEST = *mut KERB_PURGE_KDC_PROXY_CACHE_REQUEST;
STRUCT!{struct KERB_PURGE_KDC_PROXY_CACHE_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfPurged: ULONG,
}}
pub type PKERB_PURGE_KDC_PROXY_CACHE_RESPONSE = *mut KERB_PURGE_KDC_PROXY_CACHE_RESPONSE;
pub const KERB_S4U2PROXY_CACHE_ENTRY_INFO_FLAG_NEGATIVE: ULONG = 0x1;
STRUCT!{struct KERB_S4U2PROXY_CACHE_ENTRY_INFO {
    ServerName: UNICODE_STRING,
    Flags: ULONG,
    LastStatus: NTSTATUS,
    Expiry: LARGE_INTEGER,
}}
pub type PKERB_S4U2PROXY_CACHE_ENTRY_INFO = *mut KERB_S4U2PROXY_CACHE_ENTRY_INFO;
pub const KERB_S4U2PROXY_CRED_FLAG_NEGATIVE: ULONG = 0x1;
STRUCT!{struct KERB_S4U2PROXY_CRED {
    UserName: UNICODE_STRING,
    DomainName: UNICODE_STRING,
    Flags: ULONG,
    LastStatus: NTSTATUS,
    Expiry: LARGE_INTEGER,
    CountOfEntries: ULONG,
    Entries: PKERB_S4U2PROXY_CACHE_ENTRY_INFO,
}}
pub type PKERB_S4U2PROXY_CRED = *mut KERB_S4U2PROXY_CRED;
STRUCT!{struct KERB_QUERY_S4U2PROXY_CACHE_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    Flags: ULONG,
    LogonId: LUID,
}}
pub type PKERB_QUERY_S4U2PROXY_CACHE_REQUEST = *mut KERB_QUERY_S4U2PROXY_CACHE_REQUEST;
STRUCT!{struct KERB_QUERY_S4U2PROXY_CACHE_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfCreds: ULONG,
    Creds: PKERB_S4U2PROXY_CRED,
}}
pub type PKERB_QUERY_S4U2PROXY_CACHE_RESPONSE = *mut KERB_QUERY_S4U2PROXY_CACHE_RESPONSE;
STRUCT!{struct KERB_CHANGEPASSWORD_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    DomainName: UNICODE_STRING,
    AccountName: UNICODE_STRING,
    OldPassword: UNICODE_STRING,
    NewPassword: UNICODE_STRING,
    Impersonating: BOOLEAN,
}}
pub type PKERB_CHANGEPASSWORD_REQUEST = *mut KERB_CHANGEPASSWORD_REQUEST;
STRUCT!{struct KERB_SETPASSWORD_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
    CredentialsHandle: SecHandle,
    Flags: ULONG,
    DomainName: UNICODE_STRING,
    AccountName: UNICODE_STRING,
    Password: UNICODE_STRING,
}}
pub type PKERB_SETPASSWORD_REQUEST = *mut KERB_SETPASSWORD_REQUEST;
STRUCT!{struct KERB_SETPASSWORD_EX_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
    CredentialsHandle: SecHandle,
    Flags: ULONG,
    AccountRealm: UNICODE_STRING,
    AccountName: UNICODE_STRING,
    Password: UNICODE_STRING,
    ClientRealm: UNICODE_STRING,
    ClientName: UNICODE_STRING,
    Impersonating: BOOLEAN,
    KdcAddress: UNICODE_STRING,
    KdcAddressType: ULONG,
}}
pub type PKERB_SETPASSWORD_EX_REQUEST = *mut KERB_SETPASSWORD_EX_REQUEST;
pub const DS_UNKNOWN_ADDRESS_TYPE: ULONG = 0;
pub const KERB_SETPASS_USE_LOGONID: ULONG = 1;
pub const KERB_SETPASS_USE_CREDHANDLE: ULONG = 2;
STRUCT!{struct KERB_DECRYPT_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
    Flags: ULONG,
    CryptoType: LONG,
    KeyUsage: LONG,
    Key: KERB_CRYPTO_KEY,
    EncryptedDataSize: ULONG,
    InitialVectorSize: ULONG,
    InitialVector: PUCHAR,
    EncryptedData: PUCHAR,
}}
pub type PKERB_DECRYPT_REQUEST = *mut KERB_DECRYPT_REQUEST;
pub const KERB_DECRYPT_FLAG_DEFAULT_KEY: ULONG = 0x00000001;
STRUCT!{struct KERB_DECRYPT_RESPONSE {
    DecryptedData: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PKERB_DECRYPT_RESPONSE = *mut KERB_DECRYPT_RESPONSE;
STRUCT!{struct KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    RealmName: UNICODE_STRING,
    KdcAddress: UNICODE_STRING,
    AddressType: ULONG,
}}
pub type PKERB_ADD_BINDING_CACHE_ENTRY_REQUEST = *mut KERB_ADD_BINDING_CACHE_ENTRY_REQUEST;
STRUCT!{struct KERB_REFRESH_SCCRED_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CredentialBlob: UNICODE_STRING,
    LogonId: LUID,
    Flags: ULONG,
}}
pub type PKERB_REFRESH_SCCRED_REQUEST = *mut KERB_REFRESH_SCCRED_REQUEST;
pub const KERB_REFRESH_SCCRED_RELEASE: ULONG = 0x0;
pub const KERB_REFRESH_SCCRED_GETTGT: ULONG = 0x1;
STRUCT!{struct KERB_ADD_CREDENTIALS_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    UserName: UNICODE_STRING,
    DomainName: UNICODE_STRING,
    Password: UNICODE_STRING,
    LogonId: LUID,
    Flags: ULONG,
}}
pub type PKERB_ADD_CREDENTIALS_REQUEST = *mut KERB_ADD_CREDENTIALS_REQUEST;
pub const KERB_REQUEST_ADD_CREDENTIAL: ULONG = 1;
pub const KERB_REQUEST_REPLACE_CREDENTIAL: ULONG = 2;
pub const KERB_REQUEST_REMOVE_CREDENTIAL: ULONG = 4;
STRUCT!{struct KERB_ADD_CREDENTIALS_REQUEST_EX {
    Credentials: KERB_ADD_CREDENTIALS_REQUEST,
    PrincipalNameCount: ULONG,
    PrincipalNames: [UNICODE_STRING; ANYSIZE_ARRAY],
}}
pub type PKERB_ADD_CREDENTIALS_REQUEST_EX = *mut KERB_ADD_CREDENTIALS_REQUEST_EX;
STRUCT!{struct KERB_TRANSFER_CRED_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    OriginLogonId: LUID,
    DestinationLogonId: LUID,
    Flags: ULONG,
}}
pub type PKERB_TRANSFER_CRED_REQUEST = *mut KERB_TRANSFER_CRED_REQUEST;
pub const KERB_TRANSFER_CRED_WITH_TICKETS: ULONG = 0x1;
pub const KERB_TRANSFER_CRED_CLEANUP_CREDENTIALS: ULONG = 0x2;
STRUCT!{struct KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    LogonId: LUID,
}}
pub type PKERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST =
    *mut KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST;
STRUCT!{struct KERB_BINDING_CACHE_ENTRY_DATA {
    DiscoveryTime: ULONG64,
    RealmName: UNICODE_STRING,
    KdcAddress: UNICODE_STRING,
    AddressType: ULONG,
    Flags: ULONG,
    DcFlags: ULONG,
    CacheFlags: ULONG,
    KdcName: UNICODE_STRING,
}}
pub type PKERB_BINDING_CACHE_ENTRY_DATA = *mut KERB_BINDING_CACHE_ENTRY_DATA;
STRUCT!{struct KERB_QUERY_BINDING_CACHE_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    CountOfEntries: ULONG,
    Entries: PKERB_BINDING_CACHE_ENTRY_DATA,
}}
pub type PKERB_QUERY_BINDING_CACHE_RESPONSE = *mut KERB_QUERY_BINDING_CACHE_RESPONSE;
STRUCT!{struct KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    RealmName: UNICODE_STRING,
    KdcAddress: UNICODE_STRING,
    AddressType: ULONG,
    DcFlags: ULONG,
}}
pub type PKERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST = *mut KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST;
STRUCT!{struct KERB_QUERY_BINDING_CACHE_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}}
pub type PKERB_QUERY_BINDING_CACHE_REQUEST = *mut KERB_QUERY_BINDING_CACHE_REQUEST;
STRUCT!{struct KERB_PURGE_BINDING_CACHE_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
}}
pub type PKERB_PURGE_BINDING_CACHE_REQUEST = *mut KERB_PURGE_BINDING_CACHE_REQUEST;
STRUCT!{struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    Flags: ULONG,
    DomainName: UNICODE_STRING,
}}
pub type PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST =
    *mut KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST;
STRUCT!{struct KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    MessageType: KERB_PROTOCOL_MESSAGE_TYPE,
    Flags: ULONG,
    ExtendedPolicies: ULONG,
    DsFlags: ULONG,
}}
pub type PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE =
    *mut KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE;
ENUM!{enum KERB_CERTIFICATE_INFO_TYPE {
    CertHashInfo = 1,
}}
pub type PKERB_CERTIFICATE_INFO_TYPE = *mut KERB_CERTIFICATE_INFO_TYPE;
STRUCT!{struct KERB_CERTIFICATE_HASHINFO {
    StoreNameLength: USHORT,
    HashLength: USHORT,
}}
pub type PKERB_CERTIFICATE_HASHINFO = *mut KERB_CERTIFICATE_HASHINFO;
STRUCT!{struct KERB_CERTIFICATE_INFO {
    CertInfoSize: ULONG,
    InfoType: ULONG,
}}
pub type PKERB_CERTIFICATE_INFO = *mut KERB_CERTIFICATE_INFO;
STRUCT!{struct POLICY_AUDIT_SID_ARRAY {
    UsersCount: ULONG,
    UserSidArray: *mut PSID,
}}
pub type PPOLICY_AUDIT_SID_ARRAY = *mut POLICY_AUDIT_SID_ARRAY;
STRUCT!{struct AUDIT_POLICY_INFORMATION {
    AuditSubCategoryGuid: GUID,
    AuditingInformation: ULONG,
    AuditCategoryGuid: GUID,
}}
pub type PAUDIT_POLICY_INFORMATION = *mut AUDIT_POLICY_INFORMATION;
pub type LPAUDIT_POLICY_INFORMATION = PAUDIT_POLICY_INFORMATION;
pub type PCAUDIT_POLICY_INFORMATION = *const AUDIT_POLICY_INFORMATION;
pub const AUDIT_SET_SYSTEM_POLICY: ULONG = 0x0001;
pub const AUDIT_QUERY_SYSTEM_POLICY: ULONG = 0x0002;
pub const AUDIT_SET_USER_POLICY: ULONG = 0x0004;
pub const AUDIT_QUERY_USER_POLICY: ULONG = 0x0008;
pub const AUDIT_ENUMERATE_USERS: ULONG = 0x0010;
pub const AUDIT_SET_MISC_POLICY: ULONG = 0x0020;
pub const AUDIT_QUERY_MISC_POLICY: ULONG = 0x0040;
pub const AUDIT_GENERIC_ALL: ULONG = STANDARD_RIGHTS_REQUIRED | AUDIT_SET_SYSTEM_POLICY
    | AUDIT_QUERY_SYSTEM_POLICY | AUDIT_SET_USER_POLICY | AUDIT_QUERY_USER_POLICY
    | AUDIT_ENUMERATE_USERS | AUDIT_SET_MISC_POLICY | AUDIT_QUERY_MISC_POLICY;
pub const AUDIT_GENERIC_READ: ULONG = STANDARD_RIGHTS_READ | AUDIT_QUERY_SYSTEM_POLICY
    | AUDIT_QUERY_USER_POLICY | AUDIT_ENUMERATE_USERS | AUDIT_QUERY_MISC_POLICY;
pub const AUDIT_GENERIC_WRITE: ULONG = STANDARD_RIGHTS_WRITE | AUDIT_SET_USER_POLICY
    | AUDIT_SET_MISC_POLICY | AUDIT_SET_SYSTEM_POLICY;
pub const AUDIT_GENERIC_EXECUTE: ULONG = STANDARD_RIGHTS_EXECUTE;
extern "system" {
    pub fn AuditSetSystemPolicy(
        pAuditPolicy: PCAUDIT_POLICY_INFORMATION,
        PolicyCount: ULONG,
    ) -> BOOLEAN;
    pub fn AuditSetPerUserPolicy(
        pSid: *const SID,
        pAuditPolicy: PCAUDIT_POLICY_INFORMATION,
        PolicyCount: ULONG,
    ) -> BOOLEAN;
    pub fn AuditQuerySystemPolicy(
        pSubCategoryGuids: *const GUID,
        PolicyCount: ULONG,
        ppAuditPolicy: *mut PAUDIT_POLICY_INFORMATION,
    ) -> BOOLEAN;
    pub fn AuditQueryPerUserPolicy(
        pSid: *const SID,
        pSubCategoryGuids: *const GUID,
        PolicyCount: ULONG,
        ppAuditPolicy: *mut PAUDIT_POLICY_INFORMATION,
    ) -> BOOLEAN;
    pub fn AuditEnumeratePerUserPolicy(
        ppAuditSidArray: *mut PPOLICY_AUDIT_SID_ARRAY,
    ) -> BOOLEAN;
    pub fn AuditComputeEffectivePolicyBySid(
        pSid: *const SID,
        pSubCategoryGuids: *const GUID,
        dwPolicyCount: ULONG,
        ppAuditPolicy: *mut PAUDIT_POLICY_INFORMATION,
    ) -> BOOLEAN;
    pub fn AuditComputeEffectivePolicyByToken(
        hTokenHandle: HANDLE,
        pSubCategoryGuids: *const GUID,
        dwPolicyCount: ULONG,
        ppAuditPolicy: *mut PAUDIT_POLICY_INFORMATION,
    ) -> BOOLEAN;
    pub fn AuditEnumerateCategories(
        ppAuditCategoriesArray: *mut *mut GUID,
        pdwCountReturned: PULONG,
    ) -> BOOLEAN;
    pub fn AuditEnumerateSubCategories(
        pAuditCategoryGuid: *const GUID,
        bRetrieveAllSubCategories: BOOLEAN,
        ppAuditSubCategoriesArray: *mut *mut GUID,
        pdwCountReturned: PULONG,
    ) -> BOOLEAN;
    pub fn AuditLookupCategoryNameW(
        pAuditCategoryGuid: *const GUID,
        ppszCategoryName: *mut PWSTR,
    ) -> BOOLEAN;
    pub fn AuditLookupCategoryNameA(
        pAuditCategoryGuid: *const GUID,
        ppszCategoryName: *mut PSTR,
    ) -> BOOLEAN;
    pub fn AuditLookupSubCategoryNameW(
        pAuditSubCategoryGuid: *const GUID,
        ppszSubCategoryName: *mut PWSTR,
    ) -> BOOLEAN;
    pub fn AuditLookupSubCategoryNameA(
        pAuditSubCategoryGuid: *const GUID,
        ppszSubCategoryName: *mut PSTR,
    ) -> BOOLEAN;
    pub fn AuditLookupCategoryIdFromCategoryGuid(
        pAuditCategoryGuid: *const GUID,
        pAuditCategoryId: PPOLICY_AUDIT_EVENT_TYPE,
    ) -> BOOLEAN;
    pub fn AuditLookupCategoryGuidFromCategoryId(
        AuditCategoryId: POLICY_AUDIT_EVENT_TYPE,
        pAuditCategoryGuid: *mut GUID,
    ) -> BOOLEAN;
    pub fn AuditSetSecurity(
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOLEAN;
    pub fn AuditQuerySecurity(
        SecurityInformation: SECURITY_INFORMATION,
        ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> BOOLEAN;
    pub fn AuditSetGlobalSaclW(
        ObjectTypeName: PCWSTR,
        Acl: PACL,
    ) -> BOOLEAN;
    pub fn AuditSetGlobalSaclA(
        ObjectTypeName: PCSTR,
        Acl: PACL,
    ) -> BOOLEAN;
    pub fn AuditQueryGlobalSaclW(
        ObjectTypeName: PCWSTR,
        Acl: *mut PACL,
    ) -> BOOLEAN;
    pub fn AuditQueryGlobalSaclA(
        ObjectTypeName: PCSTR,
        Acl: *mut PACL,
    ) -> BOOLEAN;
    pub fn AuditFree(
        Buffer: PVOID,
    );
}
STRUCT!{struct PKU2U_CERT_BLOB {
    CertOffset: ULONG,
    CertLength: USHORT,
}}
pub type PPKU2U_CERT_BLOB = *mut PKU2U_CERT_BLOB;
pub const PKU2U_CREDUI_CONTEXT_VERSION: ULONG64 = 0x4154414454524543;
STRUCT!{struct PKU2U_CREDUI_CONTEXT {
    Version: ULONG64,
    cbHeaderLength: USHORT,
    cbStructureLength: ULONG,
    CertArrayCount: USHORT,
    CertArrayOffset: ULONG,
}}
pub type PPKU2U_CREDUI_CONTEXT = *mut PKU2U_CREDUI_CONTEXT;
ENUM!{enum PKU2U_LOGON_SUBMIT_TYPE {
    Pku2uCertificateS4ULogon = 14,
}}
pub type PPKU2U_LOGON_SUBMIT_TYPE = *mut PKU2U_LOGON_SUBMIT_TYPE;
STRUCT!{struct PKU2U_CERTIFICATE_S4U_LOGON {
    MessageType: PKU2U_LOGON_SUBMIT_TYPE,
    Flags: ULONG,
    UserPrincipalName: UNICODE_STRING,
    DomainName: UNICODE_STRING,
    CertificateLength: ULONG,
    Certificate: PUCHAR,
}}
pub type PPKU2U_CERTIFICATE_S4U_LOGON = *mut PKU2U_CERTIFICATE_S4U_LOGON;
