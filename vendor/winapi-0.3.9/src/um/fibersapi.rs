// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, DWORD};
use um::winnt::{PFLS_CALLBACK_FUNCTION, PVOID};
extern "system" {
    pub fn FlsAlloc(
        lpCallback: PFLS_CALLBACK_FUNCTION,
    ) -> DWORD;
    pub fn FlsGetValue(
        dwFlsIndex: DWORD,
    ) -> PVOID;
    pub fn FlsSetValue(
        dwFlsIndex: DWORD,
        lpFlsData: PVOID,
    ) -> BOOL;
    pub fn FlsFree(
        dwFlsIndex: DWORD,
    ) -> BOOL;
    pub fn IsThreadAFiber() -> BOOL;
}
