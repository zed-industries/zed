// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! handleapi include file
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPHANDLE};
use um::winnt::HANDLE;
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
extern "system" {
    pub fn CloseHandle(
        hObject: HANDLE,
    ) -> BOOL;
    pub fn DuplicateHandle(
        hSourceProcessHandle: HANDLE,
        hSourceHandle: HANDLE,
        hTargetProcessHandle: HANDLE,
        lpTargetHandle: LPHANDLE,
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwOptions: DWORD,
    ) -> BOOL;
    pub fn CompareObjectHandles(
        hFirstObjectHandle: HANDLE,
        hSecondObjectHandle: HANDLE,
    ) -> BOOL;
    pub fn GetHandleInformation(
        hObject: HANDLE,
        lpdwFlags: LPDWORD,
    ) -> BOOL;
    pub fn SetHandleInformation(
        hObject: HANDLE,
        dwMask: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
}
