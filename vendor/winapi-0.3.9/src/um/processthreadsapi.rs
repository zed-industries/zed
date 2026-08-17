// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! ApiSet Contract for api-ms-win-core-processthreads-l1
use ctypes::{c_int, c_void};
use shared::basetsd::{DWORD_PTR, PSIZE_T, PULONG_PTR, SIZE_T, ULONG_PTR};
use shared::guiddef::LPCGUID;
use shared::minwindef::{
    BOOL, DWORD, LPBYTE, LPCVOID, LPDWORD, LPFILETIME, LPVOID, PBOOL, PDWORD, PULONG, UINT, WORD
};
use um::minwinbase::{LPCONTEXT, LPSECURITY_ATTRIBUTES, LPTHREAD_START_ROUTINE};
use um::winnt::{
    CONTEXT, HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PAPCFUNC, PHANDLE, PPROCESSOR_NUMBER,
    PROCESS_MITIGATION_POLICY, PVOID
};
STRUCT!{struct PROCESS_INFORMATION {
    hProcess: HANDLE,
    hThread: HANDLE,
    dwProcessId: DWORD,
    dwThreadId: DWORD,
}}
pub type PPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
pub type LPPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
STRUCT!{struct STARTUPINFOA {
    cb: DWORD,
    lpReserved: LPSTR,
    lpDesktop: LPSTR,
    lpTitle: LPSTR,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: WORD,
    cbReserved2: WORD,
    lpReserved2: LPBYTE,
    hStdInput: HANDLE,
    hStdOutput: HANDLE,
    hStdError: HANDLE,
}}
pub type LPSTARTUPINFOA = *mut STARTUPINFOA;
STRUCT!{struct STARTUPINFOW {
    cb: DWORD,
    lpReserved: LPWSTR,
    lpDesktop: LPWSTR,
    lpTitle: LPWSTR,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: WORD,
    cbReserved2: WORD,
    lpReserved2: LPBYTE,
    hStdInput: HANDLE,
    hStdOutput: HANDLE,
    hStdError: HANDLE,
}}
pub type LPSTARTUPINFOW = *mut STARTUPINFOW;
extern "system" {
    pub fn QueueUserAPC(
        pfnAPC: PAPCFUNC,
        hThread: HANDLE,
        dwData: ULONG_PTR,
    ) -> DWORD;
    pub fn GetProcessTimes(
        hProcess: HANDLE,
        lpCreationTime: LPFILETIME,
        lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME,
        lpUserTime: LPFILETIME,
    ) -> BOOL;
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn GetCurrentProcessId() -> DWORD;
    pub fn ExitProcess(
        uExitCode: UINT,
    );
    pub fn TerminateProcess(
        hProcess: HANDLE,
        uExitCode: UINT,
    ) -> BOOL;
    pub fn GetExitCodeProcess(
        hProcess: HANDLE,
        lpExitCode: LPDWORD,
    ) -> BOOL;
    pub fn SwitchToThread() -> BOOL;
    pub fn CreateThread(
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        dwStackSize: SIZE_T,
        lpStartAddress: LPTHREAD_START_ROUTINE,
        lpParameter: LPVOID,
        dwCreationFlags: DWORD,
        lpThreadId: LPDWORD,
    ) -> HANDLE;
    pub fn CreateRemoteThread(
        hProcess: HANDLE,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        dwStackSize: SIZE_T,
        lpStartAddress: LPTHREAD_START_ROUTINE,
        lpParameter: LPVOID,
        dwCreationFlags: DWORD,
        lpThreadId: LPDWORD,
    ) -> HANDLE;
    pub fn GetCurrentThread() -> HANDLE;
    pub fn GetCurrentThreadId() -> DWORD;
    pub fn OpenThread(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwThreadId: DWORD,
    ) -> HANDLE;
    pub fn SetThreadPriority(
        hThread: HANDLE,
        nPriority: c_int,
    ) -> BOOL;
    pub fn SetThreadPriorityBoost(
        hThread: HANDLE,
        bDisablePriorityBoost: BOOL,
    ) -> BOOL;
    pub fn GetThreadPriorityBoost(
        hThread: HANDLE,
        pDisablePriorityBoost: PBOOL,
    ) -> BOOL;
    pub fn GetThreadPriority(
        hThread: HANDLE,
    ) -> c_int;
    pub fn ExitThread(
        dwExitCode: DWORD,
    );
    pub fn TerminateThread(
        hThread: HANDLE,
        dwExitCode: DWORD,
    ) -> BOOL;
    pub fn GetExitCodeThread(
        hThread: HANDLE,
        lpExitCode: LPDWORD,
    ) -> BOOL;
    pub fn SuspendThread(
        hThread: HANDLE,
    ) -> DWORD;
    pub fn ResumeThread(
        hThread: HANDLE,
    ) -> DWORD;
}
pub const TLS_OUT_OF_INDEXES: DWORD = 0xFFFFFFFF;
extern "system" {
    pub fn TlsAlloc() -> DWORD;
    pub fn TlsGetValue(
        dwTlsIndex: DWORD,
    ) -> LPVOID;
    pub fn TlsSetValue(
        dwTlsIndex: DWORD,
        lpTlsValue: LPVOID,
    ) -> BOOL;
    pub fn TlsFree(
        dwTlsIndex: DWORD,
    ) -> BOOL;
    pub fn CreateProcessA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL;
    pub fn CreateProcessW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL;
    pub fn SetProcessShutdownParameters(
        dwLevel: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn GetProcessVersion(
        ProcessId: DWORD,
    ) -> DWORD;
    pub fn GetStartupInfoW(
        lpStartupInfo: LPSTARTUPINFOW,
    );
    pub fn CreateProcessAsUserW(
        hToken: HANDLE,
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL;
    // pub fn GetCurrentProcessToken();
    // pub fn GetCurrentThreadToken();
    // pub fn GetCurrentThreadEffectiveToken();
    pub fn SetThreadToken(
        Thread: PHANDLE,
        Token: HANDLE,
    ) -> BOOL;
    pub fn OpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: DWORD,
        TokenHandle: PHANDLE,
    ) -> BOOL;
    pub fn OpenThreadToken(
        ThreadHandle: HANDLE,
        DesiredAccess: DWORD,
        OpenAsSelf: BOOL,
        TokenHandle: PHANDLE,
    ) -> BOOL;
    pub fn SetPriorityClass(
        hProcess: HANDLE,
        dwPriorityClass: DWORD,
    ) -> BOOL;
    pub fn SetThreadStackGuarantee(
        StackSizeInBytes: PULONG,
    ) -> BOOL;
    pub fn GetPriorityClass(
        hProcess: HANDLE,
    ) -> DWORD;
    pub fn ProcessIdToSessionId(
        dwProcessId: DWORD,
        pSessionId: *mut DWORD,
    ) -> BOOL;
    pub fn GetProcessId(
        Process: HANDLE,
    ) -> DWORD;
}
STRUCT!{struct PROC_THREAD_ATTRIBUTE_LIST {
    dummy: *mut c_void,
}}
pub type PPROC_THREAD_ATTRIBUTE_LIST = *mut PROC_THREAD_ATTRIBUTE_LIST;
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut PROC_THREAD_ATTRIBUTE_LIST;
extern "system" {
    pub fn GetThreadId(
        Thread: HANDLE,
    ) -> DWORD;
    pub fn FlushProcessWriteBuffers();
    pub fn GetProcessIdOfThread(
        Thread: HANDLE,
    ) -> DWORD;
    pub fn InitializeProcThreadAttributeList(
        lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
        dwAttributeCount: DWORD,
        dwFlags: DWORD,
        lpSize: PSIZE_T,
    ) -> BOOL;
    pub fn DeleteProcThreadAttributeList(
        lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
    );
    pub fn SetProcessAffinityUpdateMode(
        hProcess: HANDLE,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn QueryProcessAffinityUpdateMode(
        hProcess: HANDLE,
        lpdwFlags: LPDWORD,
    ) -> BOOL;
    pub fn UpdateProcThreadAttribute(
        lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
        dwFlags: DWORD,
        Attribute: DWORD_PTR,
        lpValue: PVOID,
        cbSize: SIZE_T,
        lpPreviousValue: PVOID,
        lpReturnSize: PSIZE_T,
    ) -> BOOL;
    pub fn CreateRemoteThreadEx(
        hProcess: HANDLE,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        dwStackSize: SIZE_T,
        lpStartAddress: LPTHREAD_START_ROUTINE,
        lpParameter: LPVOID,
        dwCreationFlags: DWORD,
        lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
        lpThreadId: LPDWORD,
    ) -> HANDLE;
    pub fn GetCurrentThreadStackLimits(
        LowLimit: PULONG_PTR,
        HighLimit: PULONG_PTR,
    );
    pub fn GetThreadContext(
        hThread: HANDLE,
        lpContext: LPCONTEXT,
    ) -> BOOL;
    pub fn SetThreadContext(
        hThread: HANDLE,
        lpContext: *const CONTEXT,
    ) -> BOOL;
    pub fn SetProcessMitigationPolicy(
        MitigationPolicy: PROCESS_MITIGATION_POLICY,
        lpBuffer: PVOID,
        dwLength: SIZE_T,
    ) -> BOOL;
    pub fn GetProcessMitigationPolicy(
        hProcess: HANDLE,
        MitigationPolicy: PROCESS_MITIGATION_POLICY,
        lpBuffer: PVOID,
        dwLength: SIZE_T,
    ) -> BOOL;
    pub fn FlushInstructionCache(
        hProcess: HANDLE,
        lpBaseAddress: LPCVOID,
        dwSize: SIZE_T,
    ) -> BOOL;
    pub fn GetThreadTimes(
        hThread: HANDLE,
        lpCreationTime: LPFILETIME,
        lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME,
        lpUserTime: LPFILETIME,
    ) -> BOOL;
    pub fn OpenProcess(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwProcessId: DWORD,
    ) -> HANDLE;
    pub fn IsProcessorFeaturePresent(
        ProcessorFeature: DWORD,
    ) -> BOOL;
    pub fn GetProcessHandleCount(
        hProcess: HANDLE,
        pdwHandleCount: PDWORD,
    ) -> BOOL;
    pub fn GetCurrentProcessorNumber() -> DWORD;
    pub fn SetThreadIdealProcessorEx(
        hThread: HANDLE,
        lpIdealProcessor: PPROCESSOR_NUMBER,
        lpPreviousIdealProcessor: PPROCESSOR_NUMBER,
    ) -> BOOL;
    pub fn GetThreadIdealProcessorEx(
        hThread: HANDLE,
        lpIdealProcessor: PPROCESSOR_NUMBER,
    ) -> BOOL;
    pub fn GetCurrentProcessorNumberEx(
        ProcNumber: PPROCESSOR_NUMBER,
    );
    pub fn GetProcessPriorityBoost(
        hProcess: HANDLE,
        pDisablePriorityBoost: PBOOL,
    ) -> BOOL;
    pub fn SetProcessPriorityBoost(
        hProcess: HANDLE,
        bDisablePriorityBoost: BOOL,
    ) -> BOOL;
    pub fn GetThreadIOPendingFlag(
        hThread: HANDLE,
        lpIOIsPending: PBOOL,
    ) -> BOOL;
    pub fn GetSystemTimes(
        lpIdleTime: LPFILETIME,
        lpKernelTime: LPFILETIME,
        lpUserTime: LPFILETIME,
    ) -> BOOL;
}
ENUM!{enum THREAD_INFORMATION_CLASS {
    ThreadMemoryPriority,
    ThreadAbsoluteCpuPriority,
    ThreadInformationClassMax,
}}
// MEMORY_PRIORITY_INFORMATION
extern "system" {
    pub fn GetThreadInformation(
        hThread: HANDLE,
        ThreadInformationClass: THREAD_INFORMATION_CLASS,
        ThreadInformation: LPVOID,
        ThreadInformationSize: DWORD,
    ) -> BOOL;
    pub fn SetThreadInformation(
        hThread: HANDLE,
        ThreadInformationClass: THREAD_INFORMATION_CLASS,
        ThreadInformation: LPVOID,
        ThreadInformationSize: DWORD,
    ) -> BOOL;
    pub fn IsProcessCritical(
        hProcess: HANDLE,
        Critical: PBOOL,
    ) -> BOOL;
    pub fn SetProtectedPolicy(
        PolicyGuid: LPCGUID,
        PolicyValue: ULONG_PTR,
        OldPolicyValue: PULONG_PTR,
    ) -> BOOL;
    pub fn QueryProtectedPolicy(
        PolicyGuid: LPCGUID,
        PolicyValue: PULONG_PTR,
    ) -> BOOL;
    pub fn SetThreadIdealProcessor(
        hThread: HANDLE,
        dwIdealProcessor: DWORD,
    ) -> DWORD;
}
ENUM!{enum PROCESS_INFORMATION_CLASS {
    ProcessMemoryPriority,
    ProcessInformationClassMax,
}}
extern "system" {
    pub fn SetProcessInformation(
        hProcess: HANDLE,
        ProcessInformationClass: PROCESS_INFORMATION_CLASS,
        ProcessInformation: LPVOID,
        ProcessInformationSize: DWORD,
    ) -> BOOL;
    pub fn GetProcessInformation(
        hProcess: HANDLE,
        ProcessInformationClass: PROCESS_INFORMATION_CLASS,
        ProcessInformation: LPVOID,
        ProcessInformationSize: DWORD,
    ) -> BOOL;
    // pub fn GetSystemCpuSetInformation();
    // pub fn GetProcessDefaultCpuSets();
    // pub fn SetProcessDefaultCpuSets();
    // pub fn GetThreadSelectedCpuSets();
    // pub fn SetThreadSelectedCpuSets();
    // pub fn CreateProcessAsUserA();
    pub fn GetProcessShutdownParameters(
        lpdwLevel: LPDWORD,
        lpdwFlags: LPDWORD,
    ) -> BOOL;
    // pub fn SetThreadDescription();
    // pub fn GetThreadDescription();
}
