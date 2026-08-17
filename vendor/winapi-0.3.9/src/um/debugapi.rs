// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, DWORD, PBOOL};
use um::minwinbase::LPDEBUG_EVENT;
use um::winnt::{HANDLE, LPCSTR, LPCWSTR};
extern "system" {
    pub fn IsDebuggerPresent() -> BOOL;
    pub fn DebugBreak();
    pub fn OutputDebugStringA(
        lpOutputString: LPCSTR,
    );
    pub fn OutputDebugStringW(
        lpOutputString: LPCWSTR,
    );
    pub fn ContinueDebugEvent(
        dwProcessId: DWORD,
        dwThreadId: DWORD,
        dwContinueStatus: DWORD,
    ) -> BOOL;
    pub fn WaitForDebugEvent(
        lpDebugEvent: LPDEBUG_EVENT,
        dwMilliseconds: DWORD,
    ) -> BOOL;
    pub fn DebugActiveProcess(
        dwProcessId: DWORD,
    ) -> BOOL;
    pub fn DebugActiveProcessStop(
        dwProcessId: DWORD,
    ) -> BOOL;
    pub fn CheckRemoteDebuggerPresent(
        hProcess: HANDLE,
        pbDebuggerPresent: PBOOL,
    ) -> BOOL;
    pub fn WaitForDebugEventEx(
        lpDebugEvent: LPDEBUG_EVENT,
        dwMilliseconds: DWORD,
    ) -> BOOL;
}
