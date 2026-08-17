// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, ULONG};
use shared::ws2def::IOC_VENDOR;
use um::winnt::{LONG, PVOID, ULONGLONG};
pub const SIO_SET_COMPATIBILITY_MODE: DWORD = _WSAIOW!(IOC_VENDOR, 300);
ENUM!{enum WSA_COMPATIBILITY_BEHAVIOR_ID {
    WsaBehaviorAll = 0,
    WsaBehaviorReceiveBuffering,
    WsaBehaviorAutoTuning,
}}
pub type PWSA_COMPATIBILITY_BEHAVIOR_ID = *mut WSA_COMPATIBILITY_BEHAVIOR_ID;
STRUCT!{struct WSA_COMPATIBILITY_MODE {
    BehaviorId: WSA_COMPATIBILITY_BEHAVIOR_ID,
    TargetOsVersion: ULONG,
}}
pub type PWSA_COMPATIBILITY_MODE = *mut WSA_COMPATIBILITY_MODE;
pub type RIO_BUFFERID = PVOID;
pub type PRIO_BUFFERID = *mut PVOID;
pub type RIO_CQ = PVOID;
pub type PRIO_CQ = *mut PVOID;
pub type RIO_RQ = PVOID;
pub type PRIO_RQ = *mut PVOID;
STRUCT!{struct RIORESULT {
    Status: LONG,
    BytesTransferred: ULONG,
    SocketContext: ULONGLONG,
    RequestContext: ULONGLONG,
}}
pub type PRIORESULT = *mut RIORESULT;
STRUCT!{struct RIO_BUF {
    BufferId: RIO_BUFFERID,
    Offset: ULONG,
    Length: ULONG,
}}
pub type PRIO_BUF = *mut RIO_BUF;
pub const RIO_MSG_DONT_NOTIFY: DWORD = 0x00000001;
pub const RIO_MSG_DEFER: DWORD = 0x00000002;
pub const RIO_MSG_WAITALL: DWORD = 0x00000004;
pub const RIO_MSG_COMMIT_ONLY: DWORD = 0x00000008;
pub const RIO_INVALID_BUFFERID: RIO_BUFFERID = 0xFFFFFFFF as RIO_BUFFERID;
pub const RIO_INVALID_CQ: RIO_CQ = 0 as RIO_CQ;
pub const RIO_INVALID_RQ: RIO_RQ = 0 as RIO_RQ;
pub const RIO_MAX_CQ_SIZE: DWORD = 0x8000000;
pub const RIO_CORRUPT_CQ: ULONG = 0xFFFFFFFF;
