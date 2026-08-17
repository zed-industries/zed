// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This interface definition contains typedefs for Windows Runtime data types.
use ctypes::c_char;
use um::winnt::PVOID;
DECLARE_HANDLE!{HSTRING, HSTRING__}
#[cfg(target_pointer_width = "32")]
UNION!{union HSTRING_HEADER_Reserved {
    [u32; 5],
    Reserved1 Reserved1_mut: PVOID,
    Reserved2 Reserved2_mut: [c_char; 20],
}}
#[cfg(target_pointer_width = "64")]
UNION!{union HSTRING_HEADER_Reserved {
    [u64; 3],
    Reserved1 Reserved1_mut: PVOID,
    Reserved2 Reserved2_mut: [c_char; 24],
}}
STRUCT!{struct HSTRING_HEADER {
    Reserved: HSTRING_HEADER_Reserved,
}}
DECLARE_HANDLE!{HSTRING_BUFFER, HSTRING_BUFFER__}
