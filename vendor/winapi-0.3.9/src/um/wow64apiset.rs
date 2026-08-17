// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, PBOOL, PUSHORT, UINT};
use um::winnt::{HANDLE, LPSTR, LPWSTR, PVOID};
extern "system" {
    pub fn Wow64DisableWow64FsRedirection(
        OldValue: *mut PVOID,
    ) -> BOOL;
    pub fn Wow64RevertWow64FsRedirection(
        OlValue: PVOID,
    ) -> BOOL;
    pub fn IsWow64Process(
        hProcess: HANDLE,
        Wow64Process: PBOOL,
    ) -> BOOL;
    pub fn GetSystemWow64DirectoryA(
        lpBuffer: LPSTR,
        uSize: UINT,
    ) -> UINT;
    pub fn GetSystemWow64DirectoryW(
        lpBuffer: LPWSTR,
        uSize: UINT,
    ) -> UINT;
    pub fn IsWow64Process2(
        hProcess: HANDLE,
        pProcessMachine: PUSHORT,
        pNativeMachine: PUSHORT,
    ) -> BOOL;
}
