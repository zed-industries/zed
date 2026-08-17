// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-threadpool-l1.
use shared::basetsd::ULONG_PTR;
use shared::minwindef::{BOOL, DWORD, HMODULE, PFILETIME, ULONG};
use um::minwinbase::PCRITICAL_SECTION;
use um::winnt::{
    HANDLE, PTP_CALLBACK_ENVIRON, PTP_CALLBACK_INSTANCE, PTP_CLEANUP_GROUP, PTP_IO, PTP_POOL,
    PTP_POOL_STACK_INFORMATION, PTP_SIMPLE_CALLBACK, PTP_TIMER, PTP_TIMER_CALLBACK, PTP_WAIT,
    PTP_WAIT_CALLBACK, PTP_WORK, PTP_WORK_CALLBACK, PVOID,
};
FN!{stdcall PTP_WIN32_IO_CALLBACK(
    Instance: PTP_CALLBACK_INSTANCE,
    Context: PVOID,
    Overlapped: PVOID,
    IoResult: ULONG,
    NumberOfBytesTransferred: ULONG_PTR,
    Io: PTP_IO,
) -> ()}
extern "system" {
    pub fn CreateThreadpool(
        reserved: PVOID,
    ) -> PTP_POOL;
    pub fn SetThreadpoolThreadMaximum(
        ptpp: PTP_POOL,
        cthrdMost: DWORD,
    ) -> ();
    pub fn SetThreadpoolThreadMinimum(
        ptpp: PTP_POOL,
        cthrdMic: DWORD,
    ) -> BOOL;
    pub fn SetThreadpoolStackInformation(
        ptpp: PTP_POOL,
        ptpsi: PTP_POOL_STACK_INFORMATION,
    ) -> BOOL;
    pub fn QueryThreadpoolStackInformation(
        ptpp: PTP_POOL,
        ptpsi: PTP_POOL_STACK_INFORMATION,
    ) -> BOOL;
    pub fn CloseThreadpool(
        ptpp: PTP_POOL,
    ) -> ();
    pub fn CreateThreadpoolCleanupGroup() -> PTP_CLEANUP_GROUP;
    pub fn CloseThreadpoolCleanupGroupMembers(
        ptpcg: PTP_CLEANUP_GROUP,
        fCancelPendingCallbacks: BOOL,
        pvCleanupContext: PVOID,
    ) -> ();
    pub fn CloseThreadpoolCleanupGroup(
        ptpcg: PTP_CLEANUP_GROUP,
    ) -> ();
    pub fn SetEventWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE,
        evt: HANDLE,
    ) -> ();
    pub fn ReleaseSemaphoreWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE,
        sem: HANDLE,
        crel: DWORD,
    ) -> ();
    pub fn ReleaseMutexWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE,
        mut_: HANDLE,
    ) -> ();
    pub fn LeaveCriticalSectionWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE,
        pcs: PCRITICAL_SECTION,
    ) -> ();
    pub fn FreeLibraryWhenCallbackReturns(
        pci: PTP_CALLBACK_INSTANCE,
        mod_: HMODULE,
    ) -> ();
    pub fn CallbackMayRunLong(
        pci: PTP_CALLBACK_INSTANCE,
    ) -> BOOL;
    pub fn DisassociateCurrentThreadFromCallback(
        pci: PTP_CALLBACK_INSTANCE,
    ) -> ();
    pub fn TrySubmitThreadpoolCallback(
        pfns: PTP_SIMPLE_CALLBACK,
        pv: PVOID,
        pcbe: PTP_CALLBACK_ENVIRON,
    ) -> BOOL;
    pub fn CreateThreadpoolWork(
        pfnwk: PTP_WORK_CALLBACK,
        pv: PVOID,
        pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_WORK;
    pub fn SubmitThreadpoolWork(
        pwk: PTP_WORK,
    ) -> ();
    pub fn WaitForThreadpoolWorkCallbacks(
        pwk: PTP_WORK,
        fCancelPendingCallbacks: BOOL,
    ) -> ();
    pub fn CloseThreadpoolWork(
        pwk: PTP_WORK,
    ) -> ();
    pub fn CreateThreadpoolTimer(
        pfnti: PTP_TIMER_CALLBACK,
        pv: PVOID,
        pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_TIMER;
    pub fn SetThreadpoolTimer(
        pti: PTP_TIMER,
        pftDueTime: PFILETIME,
        msPeriod: DWORD,
        msWindowLength: DWORD,
    ) -> ();
    pub fn IsThreadpoolTimerSet(
        pti: PTP_TIMER,
    ) -> BOOL;
    pub fn WaitForThreadpoolTimerCallbacks(
        pti: PTP_TIMER,
        fCancelPendingCallbacks: BOOL,
    ) -> ();
    pub fn CloseThreadpoolTimer(
        pti: PTP_TIMER,
    ) -> ();
    pub fn CreateThreadpoolWait(
        pfnwa: PTP_WAIT_CALLBACK,
        pv: PVOID,
        pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_WAIT;
    pub fn SetThreadpoolWait(
        pwa: PTP_WAIT,
        h: HANDLE,
        pftTimeout: PFILETIME,
    ) -> ();
    pub fn WaitForThreadpoolWaitCallbacks(
        pwa: PTP_WAIT,
        fCancelPendingCallbacks: BOOL,
    ) -> ();
    pub fn CloseThreadpoolWait(
        pwa: PTP_WAIT,
    ) -> ();
    pub fn CreateThreadpoolIo(
        fl: HANDLE,
        pfnio: PTP_WIN32_IO_CALLBACK,
        pv: PVOID,
        pcbe: PTP_CALLBACK_ENVIRON,
    ) -> PTP_IO;
    pub fn StartThreadpoolIo(
        pio: PTP_IO,
    ) -> ();
    pub fn CancelThreadpoolIo(
        pio: PTP_IO,
    ) -> ();
    pub fn WaitForThreadpoolIoCallbacks(
        pio: PTP_IO,
        fCancelPendingCallbacks: BOOL,
    ) -> ();
    pub fn CloseThreadpoolIo(
        pio: PTP_IO,
    ) -> ();
    pub fn SetThreadpoolTimerEx(
        pti: PTP_TIMER,
        pftDueTime: PFILETIME,
        msPeriod: DWORD,
        msWindowLength: DWORD,
    ) -> BOOL;
    pub fn SetThreadpoolWaitEx(
        pwa: PTP_WAIT,
        h: HANDLE,
        pftTimeout: PFILETIME,
        Reserved: PVOID,
    ) -> BOOL;
}
