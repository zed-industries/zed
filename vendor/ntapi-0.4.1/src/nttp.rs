use crate::ntioapi::PIO_STATUS_BLOCK;
use winapi::shared::ntdef::{HANDLE, LOGICAL, LONG, NTSTATUS, PLARGE_INTEGER, PVOID};
use winapi::um::winnt::{
    PRTL_CRITICAL_SECTION, PTP_CALLBACK_ENVIRON, PTP_CALLBACK_INSTANCE, PTP_CLEANUP_GROUP, PTP_IO,
    PTP_POOL, PTP_POOL_STACK_INFORMATION, PTP_SIMPLE_CALLBACK, PTP_TIMER, PTP_TIMER_CALLBACK,
    PTP_WAIT, PTP_WAIT_CALLBACK, PTP_WORK, PTP_WORK_CALLBACK,
};
#[repr(C)]
pub struct TP_ALPC([u8; 0]);
pub type PTP_ALPC = *mut TP_ALPC;
FN!{stdcall PTP_ALPC_CALLBACK(
    Instance: PTP_CALLBACK_INSTANCE,
    Context: PVOID,
    Alpc: PTP_ALPC,
) -> ()}
FN!{stdcall PTP_ALPC_CALLBACK_EX(
    Instanc: PTP_CALLBACK_INSTANCE,
    Contex: PVOID,
    Alp: PTP_ALPC,
    ApcContext: PVOID,
) -> ()}
EXTERN!{extern "system" {
    fn TpAllocPool(
        PoolReturn: *mut PTP_POOL,
        Reserved: PVOID,
    ) -> NTSTATUS;
    fn TpReleasePool(
        Pool: PTP_POOL,
    );
    fn TpSetPoolMaxThreads(
        Pool: PTP_POOL,
        MaxThreads: LONG,
    );
    fn TpSetPoolMinThreads(
        Pool: PTP_POOL,
        MinThreads: LONG,
    ) -> NTSTATUS;
    fn TpQueryPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: PTP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
    fn TpSetPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: PTP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
    fn TpAllocCleanupGroup(
        CleanupGroupReturn: *mut PTP_CLEANUP_GROUP,
    ) -> NTSTATUS;
    fn TpReleaseCleanupGroup(
        CleanupGroup: PTP_CLEANUP_GROUP,
    );
    fn TpReleaseCleanupGroupMembers(
        CleanupGroup: PTP_CLEANUP_GROUP,
        CancelPendingCallbacks: LOGICAL,
        CleanupParameter: PVOID,
    );
    fn TpCallbackSetEventOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Event: HANDLE,
    );
    fn TpCallbackReleaseSemaphoreOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Semaphore: HANDLE,
        ReleaseCount: LONG,
    );
    fn TpCallbackReleaseMutexOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Mutex: HANDLE,
    );
    fn TpCallbackLeaveCriticalSectionOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        CriticalSection: PRTL_CRITICAL_SECTION,
    );
    fn TpCallbackUnloadDllOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        DllHandle: PVOID,
    );
    fn TpCallbackMayRunLong(
        Instance: PTP_CALLBACK_INSTANCE,
    ) -> NTSTATUS;
    fn TpDisassociateCallback(
        Instance: PTP_CALLBACK_INSTANCE,
    );
    fn TpSimpleTryPost(
        Callback: PTP_SIMPLE_CALLBACK,
        Context: PVOID,
        CallbackEnviron: PTP_CALLBACK_ENVIRON,
    ) -> NTSTATUS;
    fn TpAllocWork(
        WorkReturn: *mut PTP_WORK,
        Callback: PTP_WORK_CALLBACK,
        Context: PVOID,
        CallbackEnviron: PTP_CALLBACK_ENVIRON,
    ) -> NTSTATUS;
    fn TpReleaseWork(
        Work: PTP_WORK,
    );
    fn TpPostWork(
        Work: PTP_WORK,
    );
    fn TpWaitForWork(
        Work: PTP_WORK,
        CancelPendingCallbacks: LOGICAL,
    );
    fn TpAllocTimer(
        Timer: *mut PTP_TIMER,
        Callback: PTP_TIMER_CALLBACK,
        Context: PVOID,
        CallbackEnviron: PTP_CALLBACK_ENVIRON,
    ) -> NTSTATUS;
    fn TpReleaseTimer(
        Timer: PTP_TIMER,
    );
    fn TpSetTimer(
        Timer: PTP_TIMER,
        DueTime: PLARGE_INTEGER,
        Period: LONG,
        WindowLength: LONG,
    );
    fn TpIsTimerSet(
        Timer: PTP_TIMER,
    ) -> LOGICAL;
    fn TpWaitForTimer(
        Timer: PTP_TIMER,
        CancelPendingCallbacks: LOGICAL,
    );
    fn TpAllocWait(
        WaitReturn: *mut PTP_WAIT,
        Callback: PTP_WAIT_CALLBACK,
        Context: PVOID,
        CallbackEnviron: PTP_CALLBACK_ENVIRON,
    ) -> NTSTATUS;
    fn TpReleaseWait(
        Wait: PTP_WAIT,
    );
    fn TpSetWait(
        Wait: PTP_WAIT,
        Handle: HANDLE,
        Timeout: PLARGE_INTEGER,
    );
    fn TpWaitForWait(
        Wait: PTP_WAIT,
        CancelPendingCallbacks: LOGICAL,
    );
}}
FN!{stdcall PTP_IO_CALLBACK(
    Instance: PTP_CALLBACK_INSTANCE,
    Context: PVOID,
    ApcContext: PVOID,
    IoSB: PIO_STATUS_BLOCK,
    Io: PTP_IO,
) -> ()}
EXTERN!{extern "system" {
    fn TpAllocIoCompletion(
        IoReturn: *mut PTP_IO,
        File: HANDLE,
        Callback: PTP_IO_CALLBACK,
        Context: PVOID,
        CallbackEnviron: PTP_CALLBACK_ENVIRON,
    ) -> NTSTATUS;
    fn TpReleaseIoCompletion(
        Io: PTP_IO,
    );
    fn TpStartAsyncIoOperation(
        Io: PTP_IO,
    );
    fn TpCancelAsyncIoOperation(
        Io: PTP_IO,
    );
    fn TpWaitForIoCompletion(
        Io: PTP_IO,
        CancelPendingCallbacks: LOGICAL,
    );
    fn TpAllocAlpcCompletion(
        AlpcReturn: *mut PTP_ALPC,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK,
        Context: PVOID,
        CallbackEnviron: PTP_CALLBACK_ENVIRON,
    ) -> NTSTATUS;
    fn TpAllocAlpcCompletionEx(
        AlpcReturn: *mut PTP_ALPC,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK_EX,
        Context: PVOID,
        CallbackEnviron: PTP_CALLBACK_ENVIRON,
    ) -> NTSTATUS;
    fn TpReleaseAlpcCompletion(
        Alpc: PTP_ALPC,
    );
    fn TpWaitForAlpcCompletion(
        Alpc: PTP_ALPC,
    );
}}
ENUM!{enum TP_TRACE_TYPE {
    TpTraceThreadPriority = 1,
    TpTraceThreadAffinity = 2,
    MaxTpTraceType = 3,
}}
EXTERN!{extern "system" {
    fn TpCaptureCaller(
        Type: TP_TRACE_TYPE,
    );
    fn TpCheckTerminateWorker(
        Thread: HANDLE,
    );
}}
