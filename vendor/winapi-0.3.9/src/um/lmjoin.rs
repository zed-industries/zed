// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// Definitions and prototypes for the Net setup apis
use shared::lmcons::NET_API_STATUS;
use shared::minwindef::{BYTE, DWORD, PBYTE, PDWORD, ULONG};
use um::wincrypt::PCCERT_CONTEXT;
use um::winnt::{HRESULT, LPCWSTR, LPWSTR, PVOID};
ENUM!{enum NETSETUP_NAME_TYPE {
    NetSetupUnknown = 0,
    NetSetupMachine,
    NetSetupWorkgroup,
    NetSetupDomain,
    NetSetupNonExistentDomain,
    NetSetupDnsMachine,
}}
pub type PNETSETUP_NAME_TYPE = *mut NETSETUP_NAME_TYPE;
ENUM!{enum DSREG_JOIN_TYPE {
    DSREG_UNKNOWN_JOIN = 0,
    DSREG_DEVICE_JOIN = 1,
    DSREG_WORKPLACE_JOIN = 2,
}}
pub type PDSREG_JOIN_TYPE = *mut DSREG_JOIN_TYPE;
STRUCT!{struct DSREG_USER_INFO {
    pszUserEmail: LPWSTR,
    pszUserKeyId: LPWSTR,
    pszUserKeyName: LPWSTR,
}}
pub type PDSREG_USER_INFO = *mut DSREG_USER_INFO;
STRUCT!{struct DSREG_JOIN_INFO {
    joinType: DSREG_JOIN_TYPE,
    pJoinCertificate: PCCERT_CONTEXT,
    pszDeviceId: LPWSTR,
    pszIdpDomain: LPWSTR,
    pszTenantId: LPWSTR,
    pszJoinUserEmail: LPWSTR,
    pszTenantDisplayName: LPWSTR,
    pszMdmEnrollmentUrl: LPWSTR,
    pszMdmTermsOfUseUrl: LPWSTR,
    pszMdmComplianceUrl: LPWSTR,
    pszUserSettingSyncUrl: LPWSTR,
    pUserInfo: *mut DSREG_USER_INFO,
}}
pub type PDSREG_JOIN_INFO = *mut DSREG_JOIN_INFO;
pub const NETSETUP_JOIN_DOMAIN: DWORD = 0x00000001;
pub const NETSETUP_ACCT_CREATE: DWORD = 0x00000002;
pub const NETSETUP_ACCT_DELETE: DWORD = 0x00000004;
pub const NETSETUP_WIN9X_UPGRADE: DWORD = 0x00000010;
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: DWORD = 0x00000020;
pub const NETSETUP_JOIN_UNSECURE: DWORD = 0x00000040;
pub const NETSETUP_MACHINE_PWD_PASSED: DWORD = 0x00000080;
pub const NETSETUP_DEFER_SPN_SET: DWORD = 0x00000100;
pub const NETSETUP_JOIN_DC_ACCOUNT: DWORD = 0x00000200;
pub const NETSETUP_JOIN_WITH_NEW_NAME: DWORD = 0x00000400;
pub const NETSETUP_JOIN_READONLY: DWORD = 0x00000800;
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: DWORD = 0x00001000;
pub const NETSETUP_INSTALL_INVOCATION: DWORD = 0x00040000;
pub const NETSETUP_AMBIGUOUS_DC: DWORD = 0x00001000;
pub const NETSETUP_NO_NETLOGON_CACHE: DWORD = 0x00002000;
pub const NETSETUP_DONT_CONTROL_SERVICES: DWORD = 0x00004000;
pub const NETSETUP_SET_MACHINE_NAME: DWORD = 0x00008000;
pub const NETSETUP_FORCE_SPN_SET: DWORD = 0x00010000;
pub const NETSETUP_NO_ACCT_REUSE: DWORD = 0x00020000;
pub const NETSETUP_ALT_SAMACCOUNTNAME: DWORD = 0x00020000;
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: DWORD = 0x10000000;
pub const NETSETUP_VALID_UNJOIN_FLAGS: DWORD = NETSETUP_ACCT_DELETE
    | NETSETUP_IGNORE_UNSUPPORTED_FLAGS | NETSETUP_JOIN_DC_ACCOUNT;
pub const NETSETUP_PROCESS_OFFLINE_FLAGS: DWORD = NETSETUP_JOIN_DOMAIN
    | NETSETUP_DOMAIN_JOIN_IF_JOINED | NETSETUP_JOIN_WITH_NEW_NAME | NETSETUP_DONT_CONTROL_SERVICES
    | NETSETUP_MACHINE_PWD_PASSED;
extern "system" {
    pub fn NetJoinDomain(
        lpServer: LPCWSTR,
        lpDomain: LPCWSTR,
        lpMachineAccountOU: LPCWSTR,
        lpAccount: LPCWSTR,
        lpPassword: LPCWSTR,
        fJoinOptions: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetUnjoinDomain(
        lpServer: LPCWSTR,
        lpAccount: LPCWSTR,
        lpPassword: LPCWSTR,
        fUnjoinOptions: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetRenameMachineInDomain(
        lpServer: LPCWSTR,
        lpNewMachineName: LPCWSTR,
        lpAccount: LPCWSTR,
        lpPassword: LPCWSTR,
        fRenameOptions: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetValidateName(
        lpServer: LPCWSTR,
        lpName: LPCWSTR,
        lpAccount: LPCWSTR,
        lpPassword: LPCWSTR,
        NameType: NETSETUP_NAME_TYPE,
    ) -> NET_API_STATUS;
    pub fn NetGetJoinableOUs(
        lpServer: LPCWSTR,
        lpDomain: LPCWSTR,
        lpAccount: LPCWSTR,
        lpPassword: LPCWSTR,
        OUCount: *mut DWORD,
        OUs: *mut *mut LPWSTR,
    ) -> NET_API_STATUS;
}
pub const NET_IGNORE_UNSUPPORTED_FLAGS: DWORD = 0x01;
extern "system" {
    pub fn NetAddAlternateComputerName(
        Server: LPCWSTR,
        AlternateName: LPCWSTR,
        DomainAccount: LPCWSTR,
        DomainAccountPassword: LPCWSTR,
        Reserved: ULONG,
    ) -> NET_API_STATUS;
    pub fn NetRemoveAlternateComputerName(
        Server: LPCWSTR,
        AlternateName: LPCWSTR,
        DomainAccount: LPCWSTR,
        DomainAccountPassword: LPCWSTR,
        Reserved: ULONG,
    ) -> NET_API_STATUS;
    pub fn NetSetPrimaryComputerName(
        Server: LPCWSTR,
        PrimaryName: LPCWSTR,
        DomainAccount: LPCWSTR,
        DomainAccountPassword: LPCWSTR,
        Reserved: ULONG,
    ) -> NET_API_STATUS;
}
ENUM!{enum NET_COMPUTER_NAME_TYPE {
    NetPrimaryComputerName,
    NetAlternateComputerNames,
    NetAllComputerNames,
    NetComputerNameTypeMax,
}}
pub type PNET_COMPUTER_NAME_TYPE = *mut NET_COMPUTER_NAME_TYPE;
extern "system" {
    pub fn NetEnumerateComputerNames(
        Server: LPCWSTR,
        NameType: NET_COMPUTER_NAME_TYPE,
        Reserved: ULONG,
        EntryCount: PDWORD,
        ComputerNames: *mut *mut LPWSTR,
    ) -> NET_API_STATUS;
}
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: DWORD = 0x00000001;
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: DWORD = 0x00000002;
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: DWORD = 0x00000004;
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: DWORD = 0x00000008;
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: DWORD = 0x00000010;
pub const NETSETUP_PROVISION_PERSISTENTSITE: DWORD = 0x00000020;
pub const NETSETUP_PROVISION_ONLINE_CALLER: DWORD = 0x40000000;
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: DWORD = 0x80000000;
extern "system" {
    pub fn NetProvisionComputerAccount(
        lpDomain: LPCWSTR,
        lpMachineName: LPCWSTR,
        lpMachineAccountOU: LPCWSTR,
        lpDcName: LPCWSTR,
        dwOptions: DWORD,
        pProvisionBinData: *mut PBYTE,
        pdwProvisionBinDataSize: *mut DWORD,
        pProvisionTextData: *mut LPWSTR,
    ) -> NET_API_STATUS;
    pub fn NetRequestOfflineDomainJoin(
        pProvisionBinData: *mut BYTE,
        cbProvisionBinDataSize: DWORD,
        dwOptions: DWORD,
        lpWindowsPath: LPCWSTR,
    ) -> NET_API_STATUS;
}
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: DWORD = 0x00000001;
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: DWORD = 0x00000002;
STRUCT!{struct NETSETUP_PROVISIONING_PARAMS {
    dwVersion: DWORD,
    lpDomain: LPCWSTR,
    lpHostName: LPCWSTR,
    lpMachineAccountOU: LPCWSTR,
    lpDcName: LPCWSTR,
    dwProvisionOptions: DWORD,
    aCertTemplateNames: *mut LPCWSTR,
    cCertTemplateNames: DWORD,
    aMachinePolicyNames: *mut LPCWSTR,
    cMachinePolicyNames: DWORD,
    aMachinePolicyPaths: *mut LPCWSTR,
    cMachinePolicyPaths: DWORD,
    lpNetbiosName: LPWSTR,
    lpSiteName: LPWSTR,
    lpPrimaryDNSDomain: LPWSTR,
}}
pub type PNETSETUP_PROVISIONING_PARAMS = *mut NETSETUP_PROVISIONING_PARAMS;
extern "system" {
    pub fn NetCreateProvisioningPackage(
        pProvisioningParams: PNETSETUP_PROVISIONING_PARAMS,
        ppPackageBinData: *mut PBYTE,
        pdwPackageBinDataSize: *mut DWORD,
        ppPackageTextData: *mut LPWSTR,
    ) -> NET_API_STATUS;
    pub fn NetRequestProvisioningPackageInstall(
        pPackageBinData: *mut BYTE,
        dwPackageBinDataSize: DWORD,
        dwProvisionOptions: DWORD,
        lpWindowsPath: LPCWSTR,
        pvReserved: PVOID,
    ) -> NET_API_STATUS;
    pub fn NetGetAadJoinInformation(
        pcszTenantId: LPCWSTR,
        ppJoinInfo: *mut PDSREG_JOIN_INFO,
    ) -> HRESULT;
    pub fn NetFreeAadJoinInformation(
        pJoinInfo: PDSREG_JOIN_INFO,
    );
}
ENUM!{enum NETSETUP_JOIN_STATUS {
    NetSetupUnknownStatus = 0,
    NetSetupUnjoined,
    NetSetupWorkgroupName,
    NetSetupDomainName,
}}
pub type PNETSETUP_JOIN_STATUS = *mut NETSETUP_JOIN_STATUS;
extern "system" {
    pub fn NetGetJoinInformation(
        lpServer: LPCWSTR,
        lpNameBuffer: *mut LPWSTR,
        BufferType: PNETSETUP_JOIN_STATUS,
    ) -> NET_API_STATUS;
}
