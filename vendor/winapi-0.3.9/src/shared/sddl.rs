// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::SIZE_T;
use shared::minwindef::{BOOL, DWORD, PULONG, UCHAR};
use um::winnt::{LPCSTR, LPCWSTR, LPSTR, LPWSTR, PSECURITY_DESCRIPTOR, PSID, SECURITY_INFORMATION};
pub const SDDL_REVISION_1: UCHAR = 1;
pub const SDDL_REVISION: UCHAR = SDDL_REVISION_1;
pub const SDDL_OWNER: &'static str = "O";
pub const SDDL_GROUP: &'static str = "G";
pub const SDDL_DACL: &'static str = "D";
pub const SDDL_SACL: &'static str = "S";
pub const SDDL_PROTECTED: &'static str = "P";
pub const SDDL_AUTO_INHERIT_REQ: &'static str = "AR";
pub const SDDL_AUTO_INHERITED: &'static str = "AI";
pub const SDDL_NULL_ACL: &'static str = "NO_ACCESS_CONTROL";
pub const SDDL_ACCESS_ALLOWED: &'static str = "A";
pub const SDDL_ACCESS_DENIED: &'static str = "D";
pub const SDDL_OBJECT_ACCESS_ALLOWED: &'static str = "OA";
pub const SDDL_OBJECT_ACCESS_DENIED: &'static str = "OD";
pub const SDDL_AUDIT: &'static str = "AU";
pub const SDDL_ALARM: &'static str = "AL";
pub const SDDL_OBJECT_AUDIT: &'static str = "OU";
pub const SDDL_OBJECT_ALARM: &'static str = "OL";
pub const SDDL_MANDATORY_LABEL: &'static str = "ML";
pub const SDDL_PROCESS_TRUST_LABEL: &'static str = "TL";
pub const SDDL_CALLBACK_ACCESS_ALLOWED: &'static str = "XA";
pub const SDDL_CALLBACK_ACCESS_DENIED: &'static str = "XD";
pub const SDDL_RESOURCE_ATTRIBUTE: &'static str = "RA";
pub const SDDL_SCOPED_POLICY_ID: &'static str = "SP";
pub const SDDL_CALLBACK_AUDIT: &'static str = "XU";
pub const SDDL_CALLBACK_OBJECT_ACCESS_ALLOWED: &'static str = "ZA";
pub const SDDL_ACCESS_FILTER: &'static str = "FL";
pub const SDDL_INT: &'static str = "TI";
pub const SDDL_UINT: &'static str = "TU";
pub const SDDL_WSTRING: &'static str = "TS";
pub const SDDL_SID: &'static str = "TD";
pub const SDDL_BLOB: &'static str = "TX";
pub const SDDL_BOOLEAN: &'static str = "TB";
pub const SDDL_CONTAINER_INHERIT: &'static str = "CI";
pub const SDDL_OBJECT_INHERIT: &'static str = "OI";
pub const SDDL_NO_PROPAGATE: &'static str = "NP";
pub const SDDL_INHERIT_ONLY: &'static str = "IO";
pub const SDDL_INHERITED: &'static str = "ID";
pub const SDDL_TRUST_PROTECTED_FILTER: &'static str = "TP";
pub const SDDL_AUDIT_SUCCESS: &'static str = "SA";
pub const SDDL_AUDIT_FAILURE: &'static str = "FA";
pub const SDDL_READ_PROPERTY: &'static str = "RP";
pub const SDDL_WRITE_PROPERTY: &'static str = "WP";
pub const SDDL_CREATE_CHILD: &'static str = "CC";
pub const SDDL_DELETE_CHILD: &'static str = "DC";
pub const SDDL_LIST_CHILDREN: &'static str = "LC";
pub const SDDL_SELF_WRITE: &'static str = "SW";
pub const SDDL_LIST_OBJECT: &'static str = "LO";
pub const SDDL_DELETE_TREE: &'static str = "DT";
pub const SDDL_CONTROL_ACCESS: &'static str = "CR";
pub const SDDL_READ_CONTROL: &'static str = "RC";
pub const SDDL_WRITE_DAC: &'static str = "WD";
pub const SDDL_WRITE_OWNER: &'static str = "WO";
pub const SDDL_STANDARD_DELETE: &'static str = "SD";
pub const SDDL_GENERIC_ALL: &'static str = "GA";
pub const SDDL_GENERIC_READ: &'static str = "GR";
pub const SDDL_GENERIC_WRITE: &'static str = "GW";
pub const SDDL_GENERIC_EXECUTE: &'static str = "GX";
pub const SDDL_FILE_ALL: &'static str = "FA";
pub const SDDL_FILE_READ: &'static str = "FR";
pub const SDDL_FILE_WRITE: &'static str = "FW";
pub const SDDL_FILE_EXECUTE: &'static str = "FX";
pub const SDDL_KEY_ALL: &'static str = "KA";
pub const SDDL_KEY_READ: &'static str = "KR";
pub const SDDL_KEY_WRITE: &'static str = "KW";
pub const SDDL_KEY_EXECUTE: &'static str = "KX";
pub const SDDL_NO_WRITE_UP: &'static str = "NW";
pub const SDDL_NO_READ_UP: &'static str = "NR";
pub const SDDL_NO_EXECUTE_UP: &'static str = "NX";
pub const SDDL_ALIAS_SIZE: SIZE_T = 2;
pub const SDDL_DOMAIN_ADMINISTRATORS: &'static str = "DA";
pub const SDDL_DOMAIN_GUESTS: &'static str = "DG";
pub const SDDL_DOMAIN_USERS: &'static str = "DU";
pub const SDDL_ENTERPRISE_DOMAIN_CONTROLLERS: &'static str = "ED";
pub const SDDL_DOMAIN_DOMAIN_CONTROLLERS: &'static str = "DD";
pub const SDDL_DOMAIN_COMPUTERS: &'static str = "DC";
pub const SDDL_BUILTIN_ADMINISTRATORS: &'static str = "BA";
pub const SDDL_BUILTIN_GUESTS: &'static str = "BG";
pub const SDDL_BUILTIN_USERS: &'static str = "BU";
pub const SDDL_LOCAL_ADMIN: &'static str = "LA";
pub const SDDL_LOCAL_GUEST: &'static str = "LG";
pub const SDDL_ACCOUNT_OPERATORS: &'static str = "AO";
pub const SDDL_BACKUP_OPERATORS: &'static str = "BO";
pub const SDDL_PRINTER_OPERATORS: &'static str = "PO";
pub const SDDL_SERVER_OPERATORS: &'static str = "SO";
pub const SDDL_AUTHENTICATED_USERS: &'static str = "AU";
pub const SDDL_PERSONAL_SELF: &'static str = "PS";
pub const SDDL_CREATOR_OWNER: &'static str = "CO";
pub const SDDL_CREATOR_GROUP: &'static str = "CG";
pub const SDDL_LOCAL_SYSTEM: &'static str = "SY";
pub const SDDL_POWER_USERS: &'static str = "PU";
pub const SDDL_EVERYONE: &'static str = "WD";
pub const SDDL_REPLICATOR: &'static str = "RE";
pub const SDDL_INTERACTIVE: &'static str = "IU";
pub const SDDL_NETWORK: &'static str = "NU";
pub const SDDL_SERVICE: &'static str = "SU";
pub const SDDL_RESTRICTED_CODE: &'static str = "RC";
pub const SDDL_WRITE_RESTRICTED_CODE: &'static str = "WR";
pub const SDDL_ANONYMOUS: &'static str = "AN";
pub const SDDL_SCHEMA_ADMINISTRATORS: &'static str = "SA";
pub const SDDL_CERT_SERV_ADMINISTRATORS: &'static str = "CA";
pub const SDDL_RAS_SERVERS: &'static str = "RS";
pub const SDDL_ENTERPRISE_ADMINS: &'static str = "EA";
pub const SDDL_GROUP_POLICY_ADMINS: &'static str = "PA";
pub const SDDL_ALIAS_PREW2KCOMPACC: &'static str = "RU";
pub const SDDL_LOCAL_SERVICE: &'static str = "LS";
pub const SDDL_NETWORK_SERVICE: &'static str = "NS";
pub const SDDL_REMOTE_DESKTOP: &'static str = "RD";
pub const SDDL_NETWORK_CONFIGURATION_OPS: &'static str = "NO";
pub const SDDL_PERFMON_USERS: &'static str = "MU";
pub const SDDL_PERFLOG_USERS: &'static str = "LU";
pub const SDDL_IIS_USERS: &'static str = "IS";
pub const SDDL_CRYPTO_OPERATORS: &'static str = "CY";
pub const SDDL_OWNER_RIGHTS: &'static str = "OW";
pub const SDDL_EVENT_LOG_READERS: &'static str = "ER";
pub const SDDL_ENTERPRISE_RO_DCs: &'static str = "RO";
pub const SDDL_CERTSVC_DCOM_ACCESS: &'static str = "CD";
pub const SDDL_ALL_APP_PACKAGES: &'static str = "AC";
pub const SDDL_RDS_REMOTE_ACCESS_SERVERS: &'static str = "RA";
pub const SDDL_RDS_ENDPOINT_SERVERS: &'static str = "ES";
pub const SDDL_RDS_MANAGEMENT_SERVERS: &'static str = "MS";
pub const SDDL_USER_MODE_DRIVERS: &'static str = "UD";
pub const SDDL_HYPER_V_ADMINS: &'static str = "HA";
pub const SDDL_CLONEABLE_CONTROLLERS: &'static str = "CN";
pub const SDDL_ACCESS_CONTROL_ASSISTANCE_OPS: &'static str = "AA";
pub const SDDL_REMOTE_MANAGEMENT_USERS: &'static str = "RM";
pub const SDDL_AUTHORITY_ASSERTED: &'static str = "AS";
pub const SDDL_SERVICE_ASSERTED: &'static str = "SS";
pub const SDDL_PROTECTED_USERS: &'static str = "AP";
pub const SDDL_KEY_ADMINS: &'static str = "KA";
pub const SDDL_ENTERPRISE_KEY_ADMINS: &'static str = "EK";
pub const SDDL_ML_LOW: &'static str = "LW";
pub const SDDL_ML_MEDIUM: &'static str = "ME";
pub const SDDL_ML_MEDIUM_PLUS: &'static str = "MP";
pub const SDDL_ML_HIGH: &'static str = "HI";
pub const SDDL_ML_SYSTEM: &'static str = "SI";
pub const SDDL_SEPERATORC: char = ';';
pub const SDDL_DELIMINATORC: char = ':';
pub const SDDL_ACE_BEGINC: char = '(';
pub const SDDL_ACE_ENDC: char = ')';
pub const SDDL_SPACEC: char = ' ';
pub const SDDL_ACE_COND_BEGINC: char = '(';
pub const SDDL_ACE_COND_ENDC: char = ')';
pub const SDDL_ACE_COND_STRING_BEGINC: char = '"';
pub const SDDL_ACE_COND_STRING_ENDC: char = '"';
pub const SDDL_ACE_COND_COMPOSITEVALUE_BEGINC: char = '{';
pub const SDDL_ACE_COND_COMPOSITEVALUE_ENDC: char = '}';
pub const SDDL_ACE_COND_COMPOSITEVALUE_SEPERATORC: char = ',';
pub const SDDL_ACE_COND_BLOB_PREFIXC: char = '#';
pub const SDDL_ACE_COND_SID_BEGINC: char = '(';
pub const SDDL_ACE_COND_SID_ENDC: char = ')';
pub const SDDL_SEPERATOR: &'static str = ";";
pub const SDDL_DELIMINATOR: &'static str = ":";
pub const SDDL_ACE_BEGIN: &'static str = "(";
pub const SDDL_ACE_END: &'static str = ")";
pub const SDDL_ACE_COND_BEGIN: &'static str = "(";
pub const SDDL_ACE_COND_END: &'static str = ")";
pub const SDDL_SPACE: &'static str = " ";
pub const SDDL_ACE_COND_BLOB_PREFIX: &'static str = "#";
pub const SDDL_ACE_COND_SID_PREFIX: &'static str = "SID";
pub const SDDL_ACE_COND_ATTRIBUTE_PREFIX: &'static str = "@";
pub const SDDL_ACE_COND_USER_ATTRIBUTE_PREFIX: &'static str = "@USER.";
pub const SDDL_ACE_COND_RESOURCE_ATTRIBUTE_PREFIX: &'static str = "@RESOURCE.";
pub const SDDL_ACE_COND_DEVICE_ATTRIBUTE_PREFIX: &'static str = "@DEVICE.";
pub const SDDL_ACE_COND_TOKEN_ATTRIBUTE_PREFIX: &'static str = "@TOKEN.";
extern "system" {
    pub fn ConvertSidToStringSidA(
        Sid: PSID,
        StringSid: *mut LPSTR,
    ) -> BOOL;
    pub fn ConvertSidToStringSidW(
        Sid: PSID,
        StringSid: *mut LPWSTR,
    ) -> BOOL;
    pub fn ConvertStringSidToSidA(
        StringSid: LPCSTR,
        Sid: *mut PSID,
    ) -> BOOL;
    pub fn ConvertStringSidToSidW(
        StringSid: LPCWSTR,
        Sid: *mut PSID,
    ) -> BOOL;
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorA(
        StringSecurityDescriptor: LPCSTR,
        StringSDRevision: DWORD,
        SecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        SecurityDescriptorSize: PULONG,
    ) -> BOOL;
    pub fn ConvertStringSecurityDescriptorToSecurityDescriptorW(
        StringSecurityDescriptor: LPCWSTR,
        StringSDRevision: DWORD,
        SecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        SecurityDescriptorSize: PULONG,
    ) -> BOOL;
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorA(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        RequestedStringSDRevision: DWORD,
        SecurityInformation: SECURITY_INFORMATION,
        StringSecurityDescriptor: *mut LPSTR,
        StringSecurityDescriptorLen: PULONG,
    ) -> BOOL;
    pub fn ConvertSecurityDescriptorToStringSecurityDescriptorW(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        RequestedStringSDRevision: DWORD,
        SecurityInformation: SECURITY_INFORMATION,
        StringSecurityDescriptor: *mut LPWSTR,
        StringSecurityDescriptorLen: PULONG,
    ) -> BOOL;
}
