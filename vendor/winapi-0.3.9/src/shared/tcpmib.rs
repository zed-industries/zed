// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// #include <winapifamily.h>
use shared::basetsd::DWORD64;
use shared::in6addr::IN6_ADDR;
use shared::minwindef::DWORD;
use shared::ntdef::{LARGE_INTEGER, UCHAR, ULONGLONG};
const ANY_SIZE: usize = 1;
pub const TCPIP_OWNING_MODULE_SIZE: usize = 16;
ENUM!{enum MIB_TCP_STATE {
    MIB_TCP_STATE_CLOSED = 1,
    MIB_TCP_STATE_LISTEN = 2,
    MIB_TCP_STATE_SYN_SENT = 3,
    MIB_TCP_STATE_SYN_RCVD = 4,
    MIB_TCP_STATE_ESTAB = 5,
    MIB_TCP_STATE_FIN_WAIT1 = 6,
    MIB_TCP_STATE_FIN_WAIT2 = 7,
    MIB_TCP_STATE_CLOSE_WAIT = 8,
    MIB_TCP_STATE_CLOSING = 9,
    MIB_TCP_STATE_LAST_ACK = 10,
    MIB_TCP_STATE_TIME_WAIT = 11,
    MIB_TCP_STATE_DELETE_TCB = 12,
    MIB_TCP_STATE_RESERVED = 100,
}}
ENUM!{enum TCP_CONNECTION_OFFLOAD_STATE {
    TcpConnectionOffloadStateInHost = 0,
    TcpConnectionOffloadStateOffloading = 1,
    TcpConnectionOffloadStateOffloaded = 2,
    TcpConnectionOffloadStateUploading = 3,
    TcpConnectionOffloadStateMax = 4,
}}
pub type PTCP_CONNECTION_OFFLOAD_STATE = *mut TCP_CONNECTION_OFFLOAD_STATE;
STRUCT!{struct MIB_TCPROW_LH {
    State: MIB_TCP_STATE,
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwRemoteAddr: DWORD,
    dwRemotePort: DWORD,
}}
pub type PMIB_TCPROW_LH = *mut MIB_TCPROW_LH;
STRUCT!{struct MIB_TCPROW_W2K {
    dwState: DWORD,
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwRemoteAddr: DWORD,
    dwRemotePort: DWORD,
}}
pub type PMIB_TCPROW_W2K = *mut MIB_TCPROW_W2K;
pub type MIB_TCPROW = MIB_TCPROW_LH;
pub type PMIB_TCPROW = *mut MIB_TCPROW;
STRUCT!{struct MIB_TCPTABLE {
    dwNumEntries: DWORD,
    table: [MIB_TCPROW; ANY_SIZE],
}}
pub type PMIB_TCPTABLE = *mut MIB_TCPTABLE;
// FIXME: SIZEOF_TCPTABLE(x)
STRUCT!{struct MIB_TCPROW2 {
    dwState: DWORD,
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwRemoteAddr: DWORD,
    dwRemotePort: DWORD,
    dwOwningPid: DWORD,
    dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}}
pub type PMIB_TCPROW2 = *mut MIB_TCPROW2;
STRUCT!{struct MIB_TCPTABLE2 {
    dwNumEntries: DWORD,
    table: [MIB_TCPROW2; ANY_SIZE],
}}
pub type PMIB_TCPTABLE2 = *mut MIB_TCPTABLE2;
// FIXME: SIZEOF_TCPTABLE2(x)
STRUCT!{struct MIB_TCPROW_OWNER_PID {
    dwState: DWORD,
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwRemoteAddr: DWORD,
    dwRemotePort: DWORD,
    dwOwningPid: DWORD,
}}
pub type PMIB_TCPROW_OWNER_PID = *mut MIB_TCPROW_OWNER_PID;
STRUCT!{struct MIB_TCPTABLE_OWNER_PID {
    dwNumEntries: DWORD,
    table: [MIB_TCPROW_OWNER_PID; ANY_SIZE],
}}
pub type PMIB_TCPTABLE_OWNER_PID = *mut MIB_TCPTABLE_OWNER_PID;
// FIXME: SIZEOF_TCPTABLE_OWNER_PID(x)
STRUCT!{struct MIB_TCPROW_OWNER_MODULE {
    dwState: DWORD,
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwRemoteAddr: DWORD,
    dwRemotePort: DWORD,
    dwOwningPid: DWORD,
    liCreateTimestamp: LARGE_INTEGER,
    OwningModuleInfo: [ULONGLONG; TCPIP_OWNING_MODULE_SIZE],
}}
pub type PMIB_TCPROW_OWNER_MODULE = *mut MIB_TCPROW_OWNER_MODULE;
STRUCT!{struct MIB_TCPTABLE_OWNER_MODULE {
    dwNumEntries: DWORD,
    table: [MIB_TCPROW_OWNER_MODULE; ANY_SIZE],
}}
pub type PMIB_TCPTABLE_OWNER_MODULE = *mut MIB_TCPTABLE_OWNER_MODULE;
// FIXME: SIZEOF_TCPTABLE_OWNER_MODULE(x)
STRUCT!{struct MIB_TCP6ROW {
    State: MIB_TCP_STATE,
    LocalAddr: IN6_ADDR,
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
    RemoteAddr: IN6_ADDR,
    dwRemoteScopeId: DWORD,
    dwRemotePort: DWORD,
}}
pub type PMIB_TCP6ROW = *mut MIB_TCP6ROW;
STRUCT!{struct MIB_TCP6TABLE {
    dwNumEntries: DWORD,
    table: [MIB_TCP6ROW; ANY_SIZE],
}}
pub type PMIB_TCP6TABLE = *mut MIB_TCP6TABLE;
// FIXME: SIZEOF_TCP6TABLE(x)
STRUCT!{struct MIB_TCP6ROW2 {
    LocalAddr: IN6_ADDR,
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
    RemoteAddr: IN6_ADDR,
    dwRemoteScopeId: DWORD,
    dwRemotePort: DWORD,
    State: MIB_TCP_STATE,
    dwOwningPid: DWORD,
    dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}}
pub type PMIB_TCP6ROW2 = *mut MIB_TCP6ROW2;
STRUCT!{struct MIB_TCP6TABLE2 {
    dwNumEntries: DWORD,
    table: [MIB_TCP6ROW2; ANY_SIZE],
}}
pub type PMIB_TCP6TABLE2 = *mut MIB_TCP6TABLE2;
// FIXME: SIZEOF_TCP6TABLE2(x)
STRUCT!{struct MIB_TCP6ROW_OWNER_PID {
    ucLocalAddr: [UCHAR; 16],
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
    ucRemoteAddr: [UCHAR; 16],
    dwRemoteScopeId: DWORD,
    dwRemotePort: DWORD,
    dwState: DWORD,
    dwOwningPid: DWORD,
}}
pub type PMIB_TCP6ROW_OWNER_PID = *mut MIB_TCP6ROW_OWNER_PID;
STRUCT!{struct MIB_TCP6TABLE_OWNER_PID {
    dwNumEntries: DWORD,
    table: [MIB_TCP6ROW_OWNER_PID; ANY_SIZE],
}}
pub type PMIB_TCP6TABLE_OWNER_PID = *mut MIB_TCP6TABLE_OWNER_PID;
// FIXME: SIZEOF_TCP6TABLE_OWNER_PID(x)
STRUCT!{struct MIB_TCP6ROW_OWNER_MODULE {
    ucLocalAddr: [UCHAR; 16],
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
    ucRemoteAddr: [UCHAR; 16],
    dwRemoteScopeId: DWORD,
    dwRemotePort: DWORD,
    dwState: DWORD,
    dwOwningPid: DWORD,
    liCreateTimestamp: LARGE_INTEGER,
    OwningModuleInfo: [ULONGLONG; TCPIP_OWNING_MODULE_SIZE],
}}
pub type PMIB_TCP6ROW_OWNER_MODULE = *mut MIB_TCP6ROW_OWNER_MODULE;
STRUCT!{struct MIB_TCP6TABLE_OWNER_MODULE {
    dwNumEntries: DWORD,
    table: [MIB_TCP6ROW_OWNER_MODULE; ANY_SIZE],
}}
pub type PMIB_TCP6TABLE_OWNER_MODULE = *mut MIB_TCP6TABLE_OWNER_MODULE;
// FIXME: SIZEOF_TCP6TABLE_OWNER_MODULE(x)
ENUM!{enum TCP_RTO_ALGORITHM {
    TcpRtoAlgorithmOther = 1,
    TcpRtoAlgorithmConstant = 2,
    TcpRtoAlgorithmRsre = 3,
    TcpRtoAlgorithmVanj = 4,
    MIB_TCP_RTO_OTHER = 1,
    MIB_TCP_RTO_CONSTANT = 2,
    MIB_TCP_RTO_RSRE = 3,
    MIB_TCP_RTO_VANJ = 4,
}}
pub type PTCP_RTO_ALGORITHM = *mut TCP_RTO_ALGORITHM;
STRUCT!{struct MIB_TCPSTATS_LH {
    RtoAlgorithm: TCP_RTO_ALGORITHM,
    dwRtoMin: DWORD,
    dwRtoMax: DWORD,
    dwMaxConn: DWORD,
    dwActiveOpens: DWORD,
    dwPassiveOpens: DWORD,
    dwAttemptFails: DWORD,
    dwEstabResets: DWORD,
    dwCurrEstab: DWORD,
    dwInSegs: DWORD,
    dwOutSegs: DWORD,
    dwRetransSegs: DWORD,
    dwInErrs: DWORD,
    dwOutRsts: DWORD,
    dwNumConns: DWORD,
}}
pub type PMIB_TCPSTATS_LH = *mut MIB_TCPSTATS_LH;
STRUCT!{struct MIB_TCPSTATS_W2K {
    dwRtoAlgorithm: DWORD,
    dwRtoMin: DWORD,
    dwRtoMax: DWORD,
    dwMaxConn: DWORD,
    dwActiveOpens: DWORD,
    dwPassiveOpens: DWORD,
    dwAttemptFails: DWORD,
    dwEstabResets: DWORD,
    dwCurrEstab: DWORD,
    dwInSegs: DWORD,
    dwOutSegs: DWORD,
    dwRetransSegs: DWORD,
    dwInErrs: DWORD,
    dwOutRsts: DWORD,
    dwNumConns: DWORD,
}}
pub type PMIB_TCPSTATS_W2K = *mut MIB_TCPSTATS_W2K;
pub type MIB_TCPSTATS = MIB_TCPSTATS_LH;
pub type PMIB_TCPSTATS = *mut MIB_TCPSTATS;
STRUCT!{struct MIB_TCPSTATS2 {
    RtoAlgorithm: TCP_RTO_ALGORITHM,
    dwRtoMin: DWORD,
    dwRtoMax: DWORD,
    dwMaxConn: DWORD,
    dwActiveOpens: DWORD,
    dwPassiveOpens: DWORD,
    dwAttemptFails: DWORD,
    dwEstabResets: DWORD,
    dwCurrEstab: DWORD,
    dw64InSegs: DWORD64,
    dw64OutSegs: DWORD64,
    dwRetransSegs: DWORD,
    dwInErrs: DWORD,
    dwOutRsts: DWORD,
    dwNumConns: DWORD,
}}
pub type PMIB_TCPSTATS2 = *mut MIB_TCPSTATS2;
