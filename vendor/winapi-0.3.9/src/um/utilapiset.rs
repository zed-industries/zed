// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, DWORD};
use shared::ntdef::PVOID;
extern "system" {
    pub fn EncodePointer(
        Ptr: PVOID,
    ) -> PVOID;
    pub fn DecodePointer(
        Ptr: PVOID,
    ) -> PVOID;
    pub fn EncodeSystemPointer(
        Ptr: PVOID,
    ) -> PVOID;
    pub fn DecodeSystemPointer(
        Ptr: PVOID,
    ) -> PVOID;
    pub fn Beep(
        dwFreq: DWORD,
        dwDuration: DWORD,
    ) -> BOOL;
}
