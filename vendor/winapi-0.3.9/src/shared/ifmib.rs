// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Contains the public definitions and structures for the non-TCP/IP specific parts of MIB-II.
// #include <winapifamily.h>
// #include <ifdef.h>
use shared::ifdef::IF_INDEX;
use shared::ipifcons::{IFTYPE, INTERNAL_IF_OPER_STATUS};
use shared::minwindef::DWORD;
use shared::ntdef::{UCHAR, WCHAR};
const ANY_SIZE: usize = 1;
STRUCT!{struct MIB_IFNUMBER {
    dwValue: DWORD,
}}
pub type PMIB_IFNUMBER = *mut MIB_IFNUMBER;
pub const MAXLEN_PHYSADDR: usize = 8;
pub const MAXLEN_IFDESCR: usize = 256;
pub const MAX_INTERFACE_NAME_LEN: usize = 256;
STRUCT!{struct MIB_IFROW {
    wszName: [WCHAR; MAX_INTERFACE_NAME_LEN],
    dwIndex: IF_INDEX,
    dwType: IFTYPE,
    dwMtu: DWORD,
    dwSpeed: DWORD,
    dwPhysAddrLen: DWORD,
    bPhysAddr: [UCHAR; MAXLEN_PHYSADDR],
    dwAdminStatus: DWORD,
    dwOperStatus: INTERNAL_IF_OPER_STATUS,
    dwLastChange: DWORD,
    dwInOctets: DWORD,
    dwInUcastPkts: DWORD,
    dwInNUcastPkts: DWORD,
    dwInDiscards: DWORD,
    dwInErrors: DWORD,
    dwInUnknownProtos: DWORD,
    dwOutOctets: DWORD,
    dwOutUcastPkts: DWORD,
    dwOutNUcastPkts: DWORD,
    dwOutDiscards: DWORD,
    dwOutErrors: DWORD,
    dwOutQLen: DWORD,
    dwDescrLen: DWORD,
    bDescr: [UCHAR; MAXLEN_IFDESCR],
}}
pub type PMIB_IFROW = *mut MIB_IFROW;
STRUCT!{struct MIB_IFTABLE {
    dwNumEntries: DWORD,
    table: [MIB_IFROW; ANY_SIZE],
}}
pub type PMIB_IFTABLE = *mut MIB_IFTABLE;
// FIXME: SIZEOF_IFTABLE(x)
