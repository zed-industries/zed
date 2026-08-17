// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Contains public interfaces to query the network roles of workstations, servers, and DCs
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, PBYTE, ULONG};
use um::winnt::{LPCWSTR, LPWSTR, PVOID};
ENUM!{enum DSROLE_MACHINE_ROLE {
    DsRole_RoleStandaloneWorkstation,
    DsRole_RoleMemberWorkstation,
    DsRole_RoleStandaloneServer,
    DsRole_RoleMemberServer,
    DsRole_RoleBackupDomainController,
    DsRole_RolePrimaryDomainController,
}}
ENUM!{enum DSROLE_SERVER_STATE {
    DsRoleServerUnknown = 0,
    DsRoleServerPrimary,
    DsRoleServerBackup,
}}
pub type PDSROLE_SERVER_STATE = *mut DSROLE_SERVER_STATE;
ENUM!{enum DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    DsRolePrimaryDomainInfoBasic = 1,
    DsRoleUpgradeStatus,
    DsRoleOperationState,
}}
pub const DSROLE_PRIMARY_DS_RUNNING: ULONG = 0x00000001;
pub const DSROLE_PRIMARY_DS_MIXED_MODE: ULONG = 0x00000002;
pub const DSROLE_UPGRADE_IN_PROGRESS: ULONG = 0x00000004;
pub const DSROLE_PRIMARY_DS_READONLY: ULONG = 0x00000008;
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: ULONG = 0x01000000;
STRUCT!{struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    MachineRole: DSROLE_MACHINE_ROLE,
    Flags: ULONG,
    DomainNameFlat: LPWSTR,
    DomainNameDns: LPWSTR,
    DomainForestName: LPWSTR,
    DomainGuid: GUID,
}}
pub type PDSROLE_PRIMARY_DOMAIN_INFO_BASIC = *mut DSROLE_PRIMARY_DOMAIN_INFO_BASIC;
STRUCT!{struct DSROLE_UPGRADE_STATUS_INFO {
    OperationState: ULONG,
    PreviousServerState: DSROLE_SERVER_STATE,
}}
pub type PDSROLE_UPGRADE_STATUS_INFO = *mut DSROLE_UPGRADE_STATUS_INFO;
ENUM!{enum DSROLE_OPERATION_STATE {
    DsRoleOperationIdle = 0,
    DsRoleOperationActive,
    DsRoleOperationNeedReboot,
}}
STRUCT!{struct DSROLE_OPERATION_STATE_INFO {
    OperationState: DSROLE_OPERATION_STATE,
}}
pub type PDSROLE_OPERATION_STATE_INFO = *mut DSROLE_OPERATION_STATE_INFO;
extern "system" {
    pub fn DsRoleGetPrimaryDomainInformation(
        lpServer: LPCWSTR,
        InfoLevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL,
        Buffer: *mut PBYTE,
    ) -> DWORD;
    pub fn DsRoleFreeMemory(
        Buffer: PVOID,
    );
}
