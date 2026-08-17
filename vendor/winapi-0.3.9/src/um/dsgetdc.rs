// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains structures, function prototypes, and definitions for the DsGetDcName API.
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, PULONG, ULONG};
use shared::ws2def::{LPSOCKET_ADDRESS, PSOCKET_ADDRESS};
use um::ntsecapi::PLSA_FOREST_TRUST_INFORMATION;
use um::winnt::{HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PHANDLE, PSID};
pub const DS_FORCE_REDISCOVERY: ULONG = 0x00000001;
pub const DS_DIRECTORY_SERVICE_REQUIRED: ULONG = 0x00000010;
pub const DS_DIRECTORY_SERVICE_PREFERRED: ULONG = 0x00000020;
pub const DS_GC_SERVER_REQUIRED: ULONG = 0x00000040;
pub const DS_PDC_REQUIRED: ULONG = 0x00000080;
pub const DS_BACKGROUND_ONLY: ULONG = 0x00000100;
pub const DS_IP_REQUIRED: ULONG = 0x00000200;
pub const DS_KDC_REQUIRED: ULONG = 0x00000400;
pub const DS_TIMESERV_REQUIRED: ULONG = 0x00000800;
pub const DS_WRITABLE_REQUIRED: ULONG = 0x00001000;
pub const DS_GOOD_TIMESERV_PREFERRED: ULONG = 0x00002000;
pub const DS_AVOID_SELF: ULONG = 0x00004000;
pub const DS_ONLY_LDAP_NEEDED: ULONG = 0x00008000;
pub const DS_IS_FLAT_NAME: ULONG = 0x00010000;
pub const DS_IS_DNS_NAME: ULONG = 0x00020000;
pub const DS_TRY_NEXTCLOSEST_SITE: ULONG = 0x00040000;
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: ULONG = 0x00080000;
pub const DS_WEB_SERVICE_REQUIRED: ULONG = 0x00100000;
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: ULONG = 0x00200000;
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: ULONG = 0x00400000;
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: ULONG = 0x00800000;
pub const DS_RETURN_DNS_NAME: ULONG = 0x40000000;
pub const DS_RETURN_FLAT_NAME: ULONG = 0x80000000;
pub const DSGETDC_VALID_FLAGS: ULONG = DS_FORCE_REDISCOVERY | DS_DIRECTORY_SERVICE_REQUIRED
    | DS_DIRECTORY_SERVICE_PREFERRED | DS_GC_SERVER_REQUIRED | DS_PDC_REQUIRED | DS_BACKGROUND_ONLY
    | DS_IP_REQUIRED | DS_KDC_REQUIRED | DS_TIMESERV_REQUIRED | DS_WRITABLE_REQUIRED
    | DS_GOOD_TIMESERV_PREFERRED | DS_AVOID_SELF | DS_ONLY_LDAP_NEEDED | DS_IS_FLAT_NAME
    | DS_IS_DNS_NAME | DS_TRY_NEXTCLOSEST_SITE | DS_DIRECTORY_SERVICE_6_REQUIRED
    | DS_DIRECTORY_SERVICE_8_REQUIRED | DS_DIRECTORY_SERVICE_9_REQUIRED
    | DS_DIRECTORY_SERVICE_10_REQUIRED | DS_WEB_SERVICE_REQUIRED | DS_RETURN_FLAT_NAME
    | DS_RETURN_DNS_NAME;
STRUCT!{struct DOMAIN_CONTROLLER_INFOA {
    DomainControllerName: LPSTR,
    DomainControllerAddress: LPSTR,
    DomainControllerAddressType: ULONG,
    DomainGuid: GUID,
    DomainName: LPSTR,
    DnsForestName: LPSTR,
    Flags: ULONG,
    DcSiteName: LPSTR,
    ClientSiteName: LPSTR,
}}
pub type PDOMAIN_CONTROLLER_INFOA = *mut DOMAIN_CONTROLLER_INFOA;
STRUCT!{struct DOMAIN_CONTROLLER_INFOW {
    DomainControllerName: LPWSTR,
    DomainControllerAddress: LPWSTR,
    DomainControllerAddressType: ULONG,
    DomainGuid: GUID,
    DomainName: LPWSTR,
    DnsForestName: LPWSTR,
    Flags: ULONG,
    DcSiteName: LPWSTR,
    ClientSiteName: LPWSTR,
}}
pub type PDOMAIN_CONTROLLER_INFOW = *mut DOMAIN_CONTROLLER_INFOW;
pub const DS_INET_ADDRESS: ULONG = 1;
pub const DS_NETBIOS_ADDRESS: ULONG = 2;
pub const DS_PDC_FLAG: ULONG = 0x00000001;
pub const DS_GC_FLAG: ULONG = 0x00000004;
pub const DS_LDAP_FLAG: ULONG = 0x00000008;
pub const DS_DS_FLAG: ULONG = 0x00000010;
pub const DS_KDC_FLAG: ULONG = 0x00000020;
pub const DS_TIMESERV_FLAG: ULONG = 0x00000040;
pub const DS_CLOSEST_FLAG: ULONG = 0x00000080;
pub const DS_WRITABLE_FLAG: ULONG = 0x00000100;
pub const DS_GOOD_TIMESERV_FLAG: ULONG = 0x00000200;
pub const DS_NDNC_FLAG: ULONG = 0x00000400;
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: ULONG = 0x00000800;
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: ULONG = 0x00001000;
pub const DS_WS_FLAG: ULONG = 0x00002000;
pub const DS_DS_8_FLAG: ULONG = 0x00004000;
pub const DS_DS_9_FLAG: ULONG = 0x00008000;
pub const DS_DS_10_FLAG: ULONG = 0x00010000;
pub const DS_PING_FLAGS: ULONG = 0x000FFFFF;
pub const DS_DNS_CONTROLLER_FLAG: ULONG = 0x20000000;
pub const DS_DNS_DOMAIN_FLAG: ULONG = 0x40000000;
pub const DS_DNS_FOREST_FLAG: ULONG = 0x80000000;
extern "system" {
    pub fn DsGetDcNameA(
        ComputerName: LPCSTR,
        DomainName: LPCSTR,
        DomainGuid: *mut GUID,
        SiteName: LPCSTR,
        Flags: ULONG,
        DomainControllerInfo: *mut PDOMAIN_CONTROLLER_INFOA,
    ) -> DWORD;
    pub fn DsGetDcNameW(
        ComputerName: LPCWSTR,
        DomainName: LPCWSTR,
        DomainGuid: *mut GUID,
        SiteName: LPCWSTR,
        Flags: ULONG,
        DomainControllerInfo: *mut PDOMAIN_CONTROLLER_INFOW,
    ) -> DWORD;
    pub fn DsGetSiteNameA(
        ComputerName: LPCSTR,
        SiteName: *mut LPSTR,
    ) -> DWORD;
    pub fn DsGetSiteNameW(
        ComputerName: LPCWSTR,
        SiteName: *mut LPWSTR,
    ) -> DWORD;
    pub fn DsValidateSubnetNameW(
        SubnetName: LPCWSTR,
    ) -> DWORD;
    pub fn DsValidateSubnetNameA(
        SubnetName: LPCSTR,
    ) -> DWORD;
    pub fn DsAddressToSiteNamesW(
        ComputerName: LPCWSTR,
        EntryCount: DWORD,
        SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPWSTR,
    ) -> DWORD;
    pub fn DsAddressToSiteNamesA(
        ComputerName: LPCSTR,
        EntryCount: DWORD,
        SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPSTR,
    ) -> DWORD;
    pub fn DsAddressToSiteNamesExW(
        ComputerName: LPCWSTR,
        EntryCount: DWORD,
        SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPWSTR,
        SubnetNames: *mut *mut LPWSTR,
    ) -> DWORD;
    pub fn DsAddressToSiteNamesExA(
        ComputerName: LPCSTR,
        EntryCount: DWORD,
        SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPSTR,
        SubnetNames: *mut *mut LPSTR,
    ) -> DWORD;
}
pub const DS_DOMAIN_IN_FOREST: ULONG = 0x0001;
pub const DS_DOMAIN_DIRECT_OUTBOUND: ULONG = 0x0002;
pub const DS_DOMAIN_TREE_ROOT: ULONG = 0x0004;
pub const DS_DOMAIN_PRIMARY: ULONG = 0x0008;
pub const DS_DOMAIN_NATIVE_MODE: ULONG = 0x0010;
pub const DS_DOMAIN_DIRECT_INBOUND: ULONG = 0x0020;
pub const DS_DOMAIN_VALID_FLAGS: ULONG = DS_DOMAIN_IN_FOREST | DS_DOMAIN_DIRECT_OUTBOUND
    | DS_DOMAIN_TREE_ROOT | DS_DOMAIN_PRIMARY | DS_DOMAIN_NATIVE_MODE | DS_DOMAIN_DIRECT_INBOUND;
STRUCT!{struct DS_DOMAIN_TRUSTSW {
    NetbiosDomainName: LPWSTR,
    DnsDomainName: LPWSTR,
    Flags: ULONG,
    ParentIndex: ULONG,
    TrustType: ULONG,
    TrustAttributes: ULONG,
    DomainSid: PSID,
    DomainGuid: GUID,
}}
pub type PDS_DOMAIN_TRUSTSW = *mut DS_DOMAIN_TRUSTSW;
STRUCT!{struct DS_DOMAIN_TRUSTSA {
    NetbiosDomainName: LPSTR,
    DnsDomainName: LPSTR,
    Flags: ULONG,
    ParentIndex: ULONG,
    TrustType: ULONG,
    TrustAttributes: ULONG,
    DomainSid: PSID,
    DomainGuid: GUID,
}}
pub type PDS_DOMAIN_TRUSTSA = *mut DS_DOMAIN_TRUSTSA;
extern "system" {
    pub fn DsEnumerateDomainTrustsW(
        ServerName: LPWSTR,
        Flags: ULONG,
        Domains: *mut PDS_DOMAIN_TRUSTSW,
        DomainCount: PULONG,
    ) -> DWORD;
    pub fn DsEnumerateDomainTrustsA(
        ServerName: LPSTR,
        Flags: ULONG,
        Domains: *mut PDS_DOMAIN_TRUSTSA,
        DomainCount: PULONG,
    ) -> DWORD;
    pub fn DsGetForestTrustInformationW(
        ServerName: LPCWSTR,
        TrustedDomainName: LPCWSTR,
        Flags: DWORD,
        ForestTrustInfo: *mut PLSA_FOREST_TRUST_INFORMATION,
    ) -> DWORD;
    pub fn DsMergeForestTrustInformationW(
        DomainName: LPCWSTR,
        NewForestTrustInfo: PLSA_FOREST_TRUST_INFORMATION,
        OldForestTrustInfo: PLSA_FOREST_TRUST_INFORMATION,
        MergedForestTrustInfo: *mut PLSA_FOREST_TRUST_INFORMATION,
    ) -> DWORD;
    pub fn DsGetDcSiteCoverageW(
        ServerName: LPCWSTR,
        EntryCount: PULONG,
        SiteNames: *mut *mut LPWSTR,
    ) -> DWORD;
    pub fn DsGetDcSiteCoverageA(
        ServerName: LPCSTR,
        EntryCount: PULONG,
        SiteNames: *mut *mut LPSTR,
    ) -> DWORD;
    pub fn DsDeregisterDnsHostRecordsW(
        ServerName: LPWSTR,
        DnsDomainName: LPWSTR,
        DomainGuid: *mut GUID,
        DsaGuid: *mut GUID,
        DnsHostName: LPWSTR,
    ) -> DWORD;
    pub fn DsDeregisterDnsHostRecordsA(
        ServerName: LPSTR,
        DnsDomainName: LPSTR,
        DomainGuid: *mut GUID,
        DsaGuid: *mut GUID,
        DnsHostName: LPSTR,
    ) -> DWORD;
}
pub const DS_ONLY_DO_SITE_NAME: ULONG = 0x01;
pub const DS_NOTIFY_AFTER_SITE_RECORDS: ULONG = 0x02;
pub const DS_OPEN_VALID_OPTION_FLAGS: ULONG = DS_ONLY_DO_SITE_NAME
    | DS_NOTIFY_AFTER_SITE_RECORDS;
pub const DS_OPEN_VALID_FLAGS: ULONG = DS_FORCE_REDISCOVERY | DS_ONLY_LDAP_NEEDED
    | DS_KDC_REQUIRED | DS_PDC_REQUIRED | DS_GC_SERVER_REQUIRED | DS_WRITABLE_REQUIRED;
extern "system" {
    pub fn DsGetDcOpenW(
        DnsName: LPCWSTR,
        OptionFlags: ULONG,
        SiteName: LPCWSTR,
        DomainGuid: *mut GUID,
        DnsForestName: LPCWSTR,
        DcFlags: ULONG,
        RetGetDcContext: PHANDLE,
    ) -> DWORD;
    pub fn DsGetDcOpenA(
        DnsName: LPCSTR,
        OptionFlags: ULONG,
        SiteName: LPCSTR,
        DomainGuid: *mut GUID,
        DnsForestName: LPCSTR,
        DcFlags: ULONG,
        RetGetDcContext: PHANDLE,
    ) -> DWORD;
    pub fn DsGetDcNextA(
        GetDcContextHandle: HANDLE,
        SockAddressCount: PULONG,
        SockAddresses: *mut LPSOCKET_ADDRESS,
        DnsHostName: *mut LPSTR,
    ) -> DWORD;
    pub fn DsGetDcNextW(
        GetDcContextHandle: HANDLE,
        SockAddressCount: PULONG,
        SockAddresses: *mut LPSOCKET_ADDRESS,
        DnsHostName: *mut LPWSTR,
    ) -> DWORD;
    pub fn DsGetDcCloseW(
        GetDcContextHandle: HANDLE,
    );
}
