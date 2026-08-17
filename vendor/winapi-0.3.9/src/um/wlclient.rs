// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, PUCHAR, UCHAR, ULONG, USHORT};
use shared::windot11::{DOT11_CURRENT_OPERATION_MODE, DOT11_MAC_ADDRESS};
use um::winnt::LPWSTR;
STRUCT!{struct DOT11_ADAPTER {
    gAdapterId: GUID,
    pszDescription: LPWSTR,
    Dot11CurrentOpMode: DOT11_CURRENT_OPERATION_MODE,
}}
pub type PDOT11_ADAPTER = *mut DOT11_ADAPTER;
STRUCT!{struct DOT11_BSS_LIST {
    uNumOfBytes: ULONG,
    pucBuffer: PUCHAR,
}}
pub type PDOT11_BSS_LIST = *mut DOT11_BSS_LIST;
STRUCT!{struct DOT11_PORT_STATE {
    PeerMacAddress: DOT11_MAC_ADDRESS,
    uSessionId: ULONG,
    bPortControlled: BOOL,
    bPortAuthorized: BOOL,
}}
pub type PDOT11_PORT_STATE = *mut DOT11_PORT_STATE;
STRUCT!{#[repr(packed)] struct DOT11_SECURITY_PACKET_HEADER {
    PeerMac: DOT11_MAC_ADDRESS,
    usEtherType: USHORT,
    Data: [UCHAR; 1],
}}
pub type PDOT11_SECURITY_PACKET_HEADER = *mut DOT11_SECURITY_PACKET_HEADER;
