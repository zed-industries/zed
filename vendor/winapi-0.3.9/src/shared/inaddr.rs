// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! IPv4 Internet address, 'on-wire' format structure.
use shared::minwindef::{UCHAR, ULONG, USHORT};
STRUCT!{struct in_addr_S_un_b {
    s_b1: UCHAR,
    s_b2: UCHAR,
    s_b3: UCHAR,
    s_b4: UCHAR,
}}
STRUCT!{struct in_addr_S_un_w {
    s_w1: USHORT,
    s_w2: USHORT,
}}
UNION!{union in_addr_S_un {
    [u32; 1],
    S_un_b S_un_b_mut: in_addr_S_un_b,
    S_un_w S_un_w_mut: in_addr_S_un_w,
    S_addr S_addr_mut: ULONG,
}}
STRUCT!{struct in_addr {
    S_un: in_addr_S_un,
}}
pub type IN_ADDR = in_addr;
pub type PIN_ADDR = *mut in_addr;
pub type LPIN_ADDR = *mut in_addr;
