// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-synch-l1
use shared::basetsd::SIZE_T;
use shared::minwindef::{BOOL, DWORD, LPLONG, LPVOID, PBOOL, ULONG};
use um::minwinbase::{
    LPCRITICAL_SECTION, LPSECURITY_ATTRIBUTES, PCRITICAL_SECTION, PREASON_CONTEXT,
};
use um::winnt::{
    BOOLEAN, HANDLE, LARGE_INTEGER, LONG, LPCSTR, LPCWSTR, PRTL_BARRIER, PRTL_RUN_ONCE,
    PVOID, RTL_BARRIER, RTL_CONDITION_VARIABLE, RTL_CONDITION_VARIABLE_INIT,
    RTL_RUN_ONCE, RTL_SRWLOCK, RTL_SRWLOCK_INIT, VOID
};
pub const SRWLOCK_INIT: SRWLOCK = RTL_SRWLOCK_INIT;
pub type SRWLOCK = RTL_SRWLOCK;
pub type PSRWLOCK = *mut RTL_SRWLOCK;
extern "system" {
    pub fn InitializeSRWLock(
        SRWLock: PSRWLOCK,
    );
    pub fn ReleaseSRWLockExclusive(
        SRWLock: PSRWLOCK,
    );
    pub fn ReleaseSRWLockShared(
        SRWLock: PSRWLOCK,
    );
    pub fn AcquireSRWLockExclusive(
        SRWLock: PSRWLOCK,
    );
    pub fn AcquireSRWLockShared(
        SRWLock: PSRWLOCK,
    );
    pub fn TryAcquireSRWLockExclusive(
        SRWLock: PSRWLOCK,
    ) -> BOOLEAN;
    pub fn TryAcquireSRWLockShared(
        SRWLock: PSRWLOCK,
    ) -> BOOLEAN;
    pub fn InitializeCriticalSection(
        lpCriticalSection: LPCRITICAL_SECTION,
    );
    pub fn EnterCriticalSection(
        lpCriticalSection: LPCRITICAL_SECTION,
    );
    pub fn LeaveCriticalSection(
        lpCriticalSection: LPCRITICAL_SECTION,
    );
    pub fn InitializeCriticalSectionAndSpinCount(
        lpCriticalSection: LPCRITICAL_SECTION,
        dwSpinCount: DWORD,
    ) -> BOOL;
    pub fn InitializeCriticalSectionEx(
        lpCriticalSection: LPCRITICAL_SECTION,
        dwSpinCount: DWORD,
        Flags: DWORD,
    ) -> BOOL;
    pub fn SetCriticalSectionSpinCount(
        lpCriticalSection: LPCRITICAL_SECTION,
        dwSpinCount: DWORD,
    ) -> DWORD;
    pub fn TryEnterCriticalSection(
        lpCriticalSection: LPCRITICAL_SECTION,
    ) -> BOOL;
    pub fn DeleteCriticalSection(
        lpCriticalSection: LPCRITICAL_SECTION,
    );
}
pub type INIT_ONCE = RTL_RUN_ONCE;
pub type PINIT_ONCE = PRTL_RUN_ONCE;
pub type LPINIT_ONCE = PRTL_RUN_ONCE;
//pub const INIT_ONCE_STATIC_INIT: INIT_ONCE = RTL_RUN_ONCE_INIT;
//pub const INIT_ONCE_CHECK_ONLY: ULONG = RTL_RUN_ONCE_CHECK_ONLY;
//pub const INIT_ONCE_ASYNC: ULONG = RTL_RUN_ONCE_ASYNC;
//pub const INIT_ONCE_INIT_FAILED: ULONG = RTL_RUN_ONCE_INIT_FAILED;
//pub const INIT_ONCE_CTX_RESERVED_BITS: usize = RTL_RUN_ONCE_CTX_RESERVED_BITS;
FN!{stdcall PINIT_ONCE_FN(
    InitOnce: PINIT_ONCE,
    Parameter: PVOID,
    Context: *mut PVOID,
) -> BOOL}
extern "system" {
    pub fn InitOnceInitialize(
        InitOnce: PINIT_ONCE,
    );
    pub fn InitOnceExecuteOnce(
        InitOnce: PINIT_ONCE,
        InitFn: PINIT_ONCE_FN,
        Parameter: PVOID,
        Context: *mut LPVOID,
    ) -> BOOL;
    pub fn InitOnceBeginInitialize(
        lpInitOnce: LPINIT_ONCE,
        dwFlags: DWORD,
        fPending: PBOOL,
        lpContext: *mut LPVOID,
    ) -> BOOL;
    pub fn InitOnceComplete(
        lpInitOnce: LPINIT_ONCE,
        dwFlags: DWORD,
        lpContext: LPVOID,
    ) -> BOOL;
}
pub type CONDITION_VARIABLE = RTL_CONDITION_VARIABLE;
pub type PCONDITION_VARIABLE = *mut CONDITION_VARIABLE;
pub const CONDITION_VARIABLE_INIT: CONDITION_VARIABLE = RTL_CONDITION_VARIABLE_INIT;
//pub const CONDITION_VARIABLE_LOCKMODE_SHARED: ULONG = RTL_CONDITION_VARIABLE_LOCKMODE_SHARED;
extern "system" {
    pub fn InitializeConditionVariable(
        ConditionVariable: PCONDITION_VARIABLE,
    );
    pub fn WakeConditionVariable(
        ConditionVariable: PCONDITION_VARIABLE,
    );
    pub fn WakeAllConditionVariable(
        ConditionVariable: PCONDITION_VARIABLE,
    );
    pub fn SleepConditionVariableCS(
        ConditionVariable: PCONDITION_VARIABLE,
        CriticalSection: PCRITICAL_SECTION,
        dwMilliseconds: DWORD,
    ) -> BOOL;
    pub fn SleepConditionVariableSRW(
        ConditionVariable: PCONDITION_VARIABLE,
        SRWLock: PSRWLOCK,
        dwMilliseconds: DWORD,
        Flags: ULONG,
    ) -> BOOL;
    pub fn SetEvent(
        hEvent: HANDLE,
    ) -> BOOL;
    pub fn ResetEvent(
        hEvent: HANDLE,
    ) -> BOOL;
    pub fn ReleaseSemaphore(
        hSemaphore: HANDLE,
        lReleaseCount: LONG,
        lpPreviousCount: LPLONG,
    ) -> BOOL;
    pub fn ReleaseMutex(
        hMutex: HANDLE,
    ) -> BOOL;
    pub fn WaitForSingleObject(
        hHandle: HANDLE,
        dwMilliseconds: DWORD,
    ) -> DWORD;
    pub fn SleepEx(
        dwMilliseconds: DWORD,
        bAlertable: BOOL,
    ) -> DWORD;
    pub fn WaitForSingleObjectEx(
        hHandle: HANDLE,
        dwMilliseconds: DWORD,
        bAlertable: BOOL,
    ) -> DWORD;
    pub fn WaitForMultipleObjectsEx(
        nCount: DWORD,
        lpHandles: *const HANDLE,
        bWaitAll: BOOL,
        dwMilliseconds: DWORD,
        bAlertable: BOOL,
    ) -> DWORD;
}
//pub const MUTEX_MODIFY_STATE: DWORD = MUTANT_QUERY_STATE;
//pub const MUTEX_ALL_ACCESS: DWORD = MUTANT_ALL_ACCESS;
extern "system" {
    pub fn CreateMutexA(
        lpMutexAttributes: LPSECURITY_ATTRIBUTES,
        bInitialOwner: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateMutexW(
        lpMutexAttributes: LPSECURITY_ATTRIBUTES,
        bInitialOwner: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn OpenMutexW(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn CreateEventA(
        lpEventAttributes: LPSECURITY_ATTRIBUTES,
        bManualReset: BOOL,
        bInitialState: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn CreateEventW(
        lpEventAttributes: LPSECURITY_ATTRIBUTES,
        bManualReset: BOOL,
        bInitialState: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn OpenEventA(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCSTR,
    ) -> HANDLE;
    pub fn OpenEventW(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn OpenSemaphoreW(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
}
FN!{stdcall PTIMERAPCROUTINE(
    lpArgToCompletionRoutine: LPVOID,
    dwTimerLowValue: DWORD,
    dwTimerHighValue: DWORD,
) -> ()}
extern "system" {
    pub fn OpenWaitableTimerW(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpTimerName: LPCWSTR,
    ) -> HANDLE;
    pub fn SetWaitableTimerEx(
        hTimer: HANDLE,
        lpDueTime: *const LARGE_INTEGER,
        lPeriod: LONG,
        pfnCompletionRoutine: PTIMERAPCROUTINE,
        lpArgToCompletionRoutine: LPVOID,
        WakeContext: PREASON_CONTEXT,
        TolerableDelay: ULONG,
    ) -> BOOL;
    pub fn SetWaitableTimer(
        hTimer: HANDLE,
        lpDueTime: *const LARGE_INTEGER,
        lPeriod: LONG,
        pfnCompletionRoutine: PTIMERAPCROUTINE,
        lpArgToCompletionRoutine: LPVOID,
        fResume: BOOL,
    ) -> BOOL;
    pub fn CancelWaitableTimer(
        hTimer: HANDLE,
    ) -> BOOL;
}
pub const CREATE_MUTEX_INITIAL_OWNER: DWORD = 0x00000001;
extern "system" {
    pub fn CreateMutexExA(
        lpMutexAttributes: LPSECURITY_ATTRIBUTES,
        lpName: LPCSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateMutexExW(
        lpMutexAttributes: LPSECURITY_ATTRIBUTES,
        lpName: LPCWSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
}
pub const CREATE_EVENT_MANUAL_RESET: DWORD = 0x00000001;
pub const CREATE_EVENT_INITIAL_SET: DWORD = 0x00000002;
extern "system" {
    pub fn CreateEventExA(
        lpEventAttributes: LPSECURITY_ATTRIBUTES,
        lpName: LPCSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateEventExW(
        lpEventAttributes: LPSECURITY_ATTRIBUTES,
        lpName: LPCWSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
    pub fn CreateSemaphoreExW(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES,
        lInitialCount: LONG,
        lMaximumCount: LONG,
        lpName: LPCWSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
}
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: DWORD = 0x00000001;
extern "system" {
    pub fn CreateWaitableTimerExW(
        lpTimerAttributes: LPSECURITY_ATTRIBUTES,
        lpTimerName: LPCWSTR,
        dwFlags: DWORD,
        dwDesiredAccess: DWORD,
    ) -> HANDLE;
}
pub type SYNCHRONIZATION_BARRIER = RTL_BARRIER;
pub type PSYNCHRONIZATION_BARRIER = PRTL_BARRIER;
pub type LPSYNCHRONIZATION_BARRIER = PRTL_BARRIER;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: DWORD = 0x01;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: DWORD = 0x02;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: DWORD = 0x04;
extern "system" {
    pub fn EnterSynchronizationBarrier(
        lpBarrier: LPSYNCHRONIZATION_BARRIER,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn InitializeSynchronizationBarrier(
        lpBarrier: LPSYNCHRONIZATION_BARRIER,
        lTotalThreads: LONG,
        lSpinCount: LONG,
    ) -> BOOL;
    pub fn DeleteSynchronizationBarrier(
        lpBarrier: LPSYNCHRONIZATION_BARRIER,
    ) -> BOOL;
    pub fn Sleep(
        dwMilliseconds: DWORD,
    );
    pub fn WaitOnAddress(
        Address: *mut VOID,
        CompareAddress: PVOID,
        AddressSize: SIZE_T,
        dwMilliseconds: DWORD,
    ) -> BOOL;
    pub fn WakeByAddressSingle(
        Address: PVOID,
    );
    pub fn WakeByAddressAll(
        Address: PVOID,
    );
    pub fn SignalObjectAndWait(
        hObjectToSignal: HANDLE,
        hObjectToWaitOn: HANDLE,
        dwMilliseconds: DWORD,
        bAlertable: BOOL,
    ) -> DWORD;
    pub fn WaitForMultipleObjects(
        nCount: DWORD,
        lpHandles: *const HANDLE,
        bWaitAll: BOOL,
        dwMilliseconds: DWORD,
    ) -> DWORD;
    pub fn CreateSemaphoreW(
        lpSemaphoreAttributes: LPSECURITY_ATTRIBUTES,
        lInitialCount: LONG,
        lMaximumCount: LONG,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn CreateWaitableTimerW(
        lpTimerAttributes: LPSECURITY_ATTRIBUTES,
        bManualReset: BOOL,
        lpTimerName: LPCWSTR,
    ) -> HANDLE;
}
