// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! IPv6 Internet address, 'on-wire' format structure.
use shared::minwindef::{UCHAR, USHORT};
UNION!{union in6_addr_u {
    [u16; 8],
    Byte Byte_mut: [UCHAR; 16],
    Word Word_mut: [USHORT; 8],
}}
STRUCT!{struct in6_addr {
    u: in6_addr_u,
}}
pub type IN6_ADDR = in6_addr;
pub type PIN6_ADDR = *mut IN6_ADDR;
pub type LPIN6_ADDR = *mut IN6_ADDR;
