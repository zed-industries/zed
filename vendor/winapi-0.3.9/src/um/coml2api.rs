// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! Structured storage, property sets, and related APIs.
use shared::minwindef::DWORD;
pub const STGM_READ: DWORD = 0x00000000;
pub const STGM_WRITE: DWORD = 0x00000001;
pub const STGM_READWRITE: DWORD = 0x00000002;
