// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// #include <winapifamily.h>
// #include <mprapidef.h>
// #include <ipifcons.h>
// #include <ipmib.h>
// #include <tcpmib.h>
// #include <udpmib.h>
use shared::ipmib::MIB_IPFORWARDROW;
use shared::minwindef::{BOOL, BYTE, DWORD};
use shared::ntdef::{PWCHAR, ULONGLONG, WCHAR};
pub const MAX_SCOPE_NAME_LEN: usize = 255;
pub const MAX_MIB_OFFSET: usize = 8;
const ANY_SIZE: usize = 1;
STRUCT!{struct MIB_OPAQUE_QUERY {
    dwVarId: DWORD,
    rgdwVarIndex: [DWORD; ANY_SIZE],
}}
pub type PMIB_OPAQUE_QUERY = *mut MIB_OPAQUE_QUERY;
ENUM!{enum TCP_TABLE_CLASS {
    TCP_TABLE_BASIC_LISTENER = 0,
    TCP_TABLE_BASIC_CONNECTIONS = 1,
    TCP_TABLE_BASIC_ALL = 2,
    TCP_TABLE_OWNER_PID_LISTENER = 3,
    TCP_TABLE_OWNER_PID_CONNECTIONS = 4,
    TCP_TABLE_OWNER_PID_ALL = 5,
    TCP_TABLE_OWNER_MODULE_LISTENER = 6,
    TCP_TABLE_OWNER_MODULE_CONNECTIONS = 7,
    TCP_TABLE_OWNER_MODULE_ALL = 8,
}}
pub type PTCP_TABLE_CLASS = *mut TCP_TABLE_CLASS;
ENUM!{enum UDP_TABLE_CLASS {
    UDP_TABLE_BASIC = 0,
    UDP_TABLE_OWNER_PID = 1,
    UDP_TABLE_OWNER_MODULE = 2,
}}
pub type PUDP_TABLE_CLASS = *mut UDP_TABLE_CLASS;
ENUM!{enum TCPIP_OWNER_MODULE_INFO_CLASS {
    TCPIP_OWNER_MODULE_INFO_BASIC = 0,
}}
pub type PTCPIP_OWNER_MODULE_INFO_CLASS = *mut TCPIP_OWNER_MODULE_INFO_CLASS;
STRUCT!{struct TCPIP_OWNER_MODULE_BASIC_INFO {
    pModuleName: PWCHAR,
    pModulePath: PWCHAR,
}}
pub type PTCPIP_OWNER_MODULE_BASIC_INFO = *mut TCPIP_OWNER_MODULE_BASIC_INFO;
STRUCT!{struct MIB_IPMCAST_BOUNDARY {
    dwIfIndex: DWORD,
    dwGroupAddress: DWORD,
    dwGroupMask: DWORD,
    dwStatus: DWORD,
}}
pub type PMIB_IPMCAST_BOUNDARY = *mut MIB_IPMCAST_BOUNDARY;
STRUCT!{struct MIB_IPMCAST_BOUNDARY_TABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPMCAST_BOUNDARY; ANY_SIZE],
}}
pub type PMIB_IPMCAST_BOUNDARY_TABLE = *mut MIB_IPMCAST_BOUNDARY_TABLE;
STRUCT!{struct MIB_BOUNDARYROW {
    dwGroupAddress: DWORD,
    dwGroupMask: DWORD,
}}
pub type PMIB_BOUNDARYROW = *mut MIB_BOUNDARYROW;
STRUCT!{struct MIB_MCAST_LIMIT_ROW {
    dwTtl: DWORD,
    dwRateLimit: DWORD,
}}
pub type PMIB_MCAST_LIMIT_ROW = *mut MIB_MCAST_LIMIT_ROW;
pub type SN_CHAR = WCHAR;
pub type SCOPE_NAME_BUFFER = [SN_CHAR; MAX_SCOPE_NAME_LEN + 1];
pub type SCOPE_NAME = *mut SCOPE_NAME_BUFFER;
STRUCT!{struct MIB_IPMCAST_SCOPE {
    dwGroupAddress: DWORD,
    dwGroupMask: DWORD,
    snNameBuffer: SCOPE_NAME_BUFFER,
    dwStatus: DWORD,
}}
pub type PMIB_IPMCAST_SCOPE = *mut MIB_IPMCAST_SCOPE;
STRUCT!{struct MIB_IPDESTROW {
    ForwardRow: MIB_IPFORWARDROW,
    dwForwardPreference: DWORD,
    dwForwardViewSet: DWORD,
}}
pub type PMIB_IPDESTROW = *mut MIB_IPDESTROW;
STRUCT!{struct MIB_IPDESTTABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPDESTROW; ANY_SIZE],
}}
pub type PMIB_IPDESTTABLE = *mut MIB_IPDESTTABLE;
STRUCT!{struct MIB_BEST_IF {
    dwDestAddr: DWORD,
    dwIfIndex: DWORD,
}}
pub type PMIB_BEST_IF = *mut MIB_BEST_IF;
STRUCT!{struct MIB_PROXYARP {
    dwAddress: DWORD,
    dwMask: DWORD,
    dwIfIndex: DWORD,
}}
pub type PMIB_PROXYARP = *mut MIB_PROXYARP;
STRUCT!{struct MIB_IFSTATUS {
    dwIfIndex: DWORD,
    dwAdminStatus: DWORD,
    dwOperationalStatus: DWORD,
    bMHbeatActive: BOOL,
    bMHbeatAlive: BOOL,
}}
pub type PMIB_IFSTATUS = *mut MIB_IFSTATUS;
STRUCT!{struct MIB_ROUTESTATE {
    bRoutesSetToStack: BOOL,
}}
pub type PMIB_ROUTESTATE = *mut MIB_ROUTESTATE;
UNION!{union MIB_OPAQUE_INFO_u {
    [u64; 1],
    ullAlign ullAlign_mut: ULONGLONG,
    rgbyData rgbyData_mut: [BYTE; 1],
}}
STRUCT!{struct MIB_OPAQUE_INFO {
    dwId: DWORD,
    u: MIB_OPAQUE_INFO_u,
}}
pub type PMIB_OPAQUE_INFO = *mut MIB_OPAQUE_INFO;
