// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! LSA Policy Lookup API
use shared::guiddef::GUID;
use shared::minwindef::{ULONG, USHORT};
use shared::ntdef::NTSTATUS;
use um::winnt::{ACCESS_MASK, HANDLE, LONG, PCHAR, PSID, PVOID, PWSTR, SID_NAME_USE};
STRUCT!{struct LSA_UNICODE_STRING {
    Length: USHORT,
    MaximumLength: USHORT,
    Buffer: PWSTR,
}}
pub type PLSA_UNICODE_STRING = *mut LSA_UNICODE_STRING;
STRUCT!{struct LSA_STRING {
    Length: USHORT,
    MaximumLength: USHORT,
    Buffer: PCHAR,
}}
pub type PLSA_STRING = *mut LSA_STRING;
STRUCT!{struct LSA_OBJECT_ATTRIBUTES {
    Length: ULONG,
    RootDirectory: HANDLE,
    ObjectName: PLSA_UNICODE_STRING,
    Attributes: ULONG,
    SecurityDescriptor: PVOID,
    SecurityQualityOfService: PVOID,
}}
pub type PLSA_OBJECT_ATTRIBUTES = *mut LSA_OBJECT_ATTRIBUTES;
STRUCT!{struct LSA_TRUST_INFORMATION {
    Name: LSA_UNICODE_STRING,
    Sid: PSID,
}}
pub type PLSA_TRUST_INFORMATION = *mut LSA_TRUST_INFORMATION;
STRUCT!{struct LSA_REFERENCED_DOMAIN_LIST {
    Entries: ULONG,
    Domains: PLSA_TRUST_INFORMATION,
}}
pub type PLSA_REFERENCED_DOMAIN_LIST = *mut LSA_REFERENCED_DOMAIN_LIST;
STRUCT!{struct LSA_TRANSLATED_SID2 {
    Use: SID_NAME_USE,
    Sid: PSID,
    DomainIndex: LONG,
    Flags: ULONG,
}}
pub type PLSA_TRANSLATED_SID2 = *mut LSA_TRANSLATED_SID2;
STRUCT!{struct LSA_TRANSLATED_NAME {
    Use: SID_NAME_USE,
    Name: LSA_UNICODE_STRING,
    DomainIndex: LONG,
}}
pub type PLSA_TRANSLATED_NAME = *mut LSA_TRANSLATED_NAME;
STRUCT!{struct POLICY_ACCOUNT_DOMAIN_INFO {
    DomainName: LSA_UNICODE_STRING,
    DomainSid: PSID,
}}
pub type PPOLICY_ACCOUNT_DOMAIN_INFO = *mut POLICY_ACCOUNT_DOMAIN_INFO;
STRUCT!{struct POLICY_DNS_DOMAIN_INFO {
    Name: LSA_UNICODE_STRING,
    DnsDomainName: LSA_UNICODE_STRING,
    DnsForestName: LSA_UNICODE_STRING,
    DomainGuid: GUID,
    Sid: PSID,
}}
pub type PPOLICY_DNS_DOMAIN_INFO = *mut POLICY_DNS_DOMAIN_INFO;
pub const LOOKUP_VIEW_LOCAL_INFORMATION: ACCESS_MASK = 0x00000001;
pub const LOOKUP_TRANSLATE_NAMES: ACCESS_MASK = 0x00000800;
ENUM!{enum LSA_LOOKUP_DOMAIN_INFO_CLASS {
    AccountDomainInformation = 5,
    DnsDomainInformation = 12,
}}
pub type PLSA_LOOKUP_DOMAIN_INFO_CLASS = *mut LSA_LOOKUP_DOMAIN_INFO_CLASS;
pub type LSA_LOOKUP_HANDLE = PVOID;
pub type PLSA_LOOKUP_HANDLE = *mut PVOID;
extern "C" {
    pub fn LsaLookupOpenLocalPolicy(
        ObjectAttributes: PLSA_OBJECT_ATTRIBUTES,
        AccessMask: ACCESS_MASK,
        PolicyHandle: PLSA_LOOKUP_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaLookupClose(
        ObjectHandle: LSA_LOOKUP_HANDLE,
    ) -> NTSTATUS;
    pub fn LsaLookupTranslateSids(
        PolicyHandle: LSA_LOOKUP_HANDLE,
        Count: ULONG,
        Sids: *mut PSID,
        ReferencedDomains: *mut PLSA_REFERENCED_DOMAIN_LIST,
        Names: *mut PLSA_TRANSLATED_NAME,
    ) -> NTSTATUS;
    pub fn LsaLookupTranslateNames(
        PolicyHandle: LSA_LOOKUP_HANDLE,
        Flags: ULONG,
        Count: ULONG,
        Names: PLSA_UNICODE_STRING,
        ReferencedDomains: *mut PLSA_REFERENCED_DOMAIN_LIST,
        Sids: *mut PLSA_TRANSLATED_SID2,
    ) -> NTSTATUS;
    pub fn LsaLookupGetDomainInfo(
        PolicyHandle: LSA_LOOKUP_HANDLE,
        DomainInfoClass: LSA_LOOKUP_DOMAIN_INFO_CLASS,
        DomainInfo: *mut PVOID,
    ) -> NTSTATUS;
    pub fn LsaLookupFreeMemory(
        Buffer: PVOID,
    ) -> NTSTATUS;
}
