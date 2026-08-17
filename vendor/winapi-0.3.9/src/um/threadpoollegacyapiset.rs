// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, DWORD, ULONG};
use um::minwinbase::LPTHREAD_START_ROUTINE;
use um::winnt::{HANDLE, PHANDLE, PVOID, WAITORTIMERCALLBACK};
extern "system" {
    pub fn QueueUserWorkItem(
        Function: LPTHREAD_START_ROUTINE,
        Context: PVOID,
        Flags: ULONG,
    ) -> BOOL;
    pub fn UnregisterWaitEx(
        WaitHandle: HANDLE,
        CompletionEvent: HANDLE,
    ) -> BOOL;
    pub fn CreateTimerQueue() -> HANDLE;
    pub fn CreateTimerQueueTimer(
        phNewTimer: PHANDLE,
        TimerQueue: HANDLE,
        Callback: WAITORTIMERCALLBACK,
        Parameter: PVOID,
        DueTime: DWORD,
        Period: DWORD,
        Flags: ULONG,
    ) -> BOOL;
    pub fn ChangeTimerQueueTimer(
        TimerQueue: HANDLE,
        Timer: HANDLE,
        DueTime: ULONG,
        Period: ULONG,
    ) -> BOOL;
    pub fn DeleteTimerQueueTimer(
        TimerQueue: HANDLE,
        Timer: HANDLE,
        CompletionEvent: HANDLE,
    ) -> BOOL;
    pub fn DeleteTimerQueueEx(
        TimerQueue: HANDLE,
        CompletionEvent: HANDLE,
    ) -> BOOL;
}
