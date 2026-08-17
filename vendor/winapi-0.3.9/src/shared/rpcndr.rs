// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{__int64, __uint64, c_char, c_uchar, c_ulong};
pub const NDR_CHAR_REP_MASK: c_ulong = 0x0000000F;
pub const NDR_INT_REP_MASK: c_ulong = 0x000000F0;
pub const NDR_FLOAT_REP_MASK: c_ulong = 0x0000FF00;
pub const NDR_LITTLE_ENDIAN: c_ulong = 0x00000010;
pub const NDR_BIG_ENDIAN: c_ulong = 0x00000000;
pub const NDR_IEEE_FLOAT: c_ulong = 0x00000000;
pub const NDR_VAX_FLOAT: c_ulong = 0x00000100;
pub const NDR_IBM_FLOAT: c_ulong = 0x00000300;
pub const NDR_ASCII_CHAR: c_ulong = 0x00000000;
pub const NDR_EBCDIC_CHAR: c_ulong = 0x00000001;
pub const NDR_LOCAL_DATA_REPRESENTATION: c_ulong = 0x00000010;
pub const NDR_LOCAL_ENDIAN: c_ulong = NDR_LITTLE_ENDIAN;
pub type small = c_char;
pub type byte = c_uchar;
pub type cs_byte = byte;
pub type boolean = c_uchar;
pub type hyper = __int64;
pub type MIDL_uhyper = __uint64;
// TODO Finish the rest
