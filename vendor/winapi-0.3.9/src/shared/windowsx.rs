// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Macro APIs, window message crackers, and control APIs
use ctypes::{c_int, c_short};
use shared::minwindef::{DWORD, HIWORD, LOWORD, LPARAM};
//1233
#[inline]
pub fn GET_X_LPARAM(lp: LPARAM) -> c_int {
    LOWORD(lp as DWORD) as c_short as c_int
}
#[inline]
pub fn GET_Y_LPARAM(lp: LPARAM) -> c_int {
    HIWORD(lp as DWORD) as c_short as c_int
}
