windows_targets::link!("kernel32.dll" "system" fn AcquireSRWLockExclusive(srwlock : *mut SRWLOCK));
windows_targets::link!("kernel32.dll" "system" fn AcquireSRWLockShared(srwlock : *mut SRWLOCK));
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor : *mut super::super::Foundation:: HANDLE, integritylabel : super::super::Security:: PSID) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn AddSIDToBoundaryDescriptor(boundarydescriptor : *mut super::super::Foundation:: HANDLE, requiredsid : super::super::Security:: PSID) -> super::super::Foundation:: BOOL);
windows_targets::link!("user32.dll" "system" fn AttachThreadInput(idattach : u32, idattachto : u32, fattach : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvQuerySystemResponsiveness(avrthandle : super::super::Foundation:: HANDLE, systemresponsivenessvalue : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRevertMmThreadCharacteristics(avrthandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroup(context : *mut super::super::Foundation:: HANDLE, period : *const i64, threadorderingguid : *mut windows_sys::core::GUID, timeout : *const i64) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExA(context : *mut super::super::Foundation:: HANDLE, period : *const i64, threadorderingguid : *mut windows_sys::core::GUID, timeout : *const i64, taskname : windows_sys::core::PCSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExW(context : *mut super::super::Foundation:: HANDLE, period : *const i64, threadorderingguid : *mut windows_sys::core::GUID, timeout : *const i64, taskname : windows_sys::core::PCWSTR) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRtDeleteThreadOrderingGroup(context : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRtJoinThreadOrderingGroup(context : *mut super::super::Foundation:: HANDLE, threadorderingguid : *const windows_sys::core::GUID, before : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRtLeaveThreadOrderingGroup(context : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvRtWaitOnThreadOrderingGroup(context : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsA(firsttask : windows_sys::core::PCSTR, secondtask : windows_sys::core::PCSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
windows_targets::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsW(firsttask : windows_sys::core::PCWSTR, secondtask : windows_sys::core::PCWSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
windows_targets::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsA(taskname : windows_sys::core::PCSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
windows_targets::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsW(taskname : windows_sys::core::PCWSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
windows_targets::link!("avrt.dll" "system" fn AvSetMmThreadPriority(avrthandle : super::super::Foundation:: HANDLE, priority : AVRT_PRIORITY) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : PTP_CALLBACK_INSTANCE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : PTP_IO));
windows_targets::link!("kernel32.dll" "system" fn CancelTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, timer : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn CancelWaitableTimer(htimer : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ChangeTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, timer : super::super::Foundation:: HANDLE, duetime : u32, period : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ClosePrivateNamespace(handle : super::super::Foundation:: HANDLE, flags : u32) -> super::super::Foundation:: BOOLEAN);
windows_targets::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : PTP_POOL));
windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : PTP_CLEANUP_GROUP));
windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : PTP_CLEANUP_GROUP, fcancelpendingcallbacks : super::super::Foundation:: BOOL, pvcleanupcontext : *mut core::ffi::c_void));
windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : PTP_IO));
windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : PTP_TIMER));
windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : PTP_WAIT));
windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : PTP_WORK));
windows_targets::link!("kernel32.dll" "system" fn ConvertFiberToThread() -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ConvertThreadToFiber(lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn ConvertThreadToFiberEx(lpparameter : *const core::ffi::c_void, dwflags : u32) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorA(name : windows_sys::core::PCSTR, flags : u32) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorW(name : windows_sys::core::PCWSTR, flags : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateEventA(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, binitialstate : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateEventExA(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : CREATE_EVENT, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateEventExW(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : CREATE_EVENT, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, binitialstate : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn CreateFiber(dwstacksize : usize, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn CreateFiberEx(dwstackcommitsize : usize, dwstackreservesize : usize, dwflags : u32, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binitialowner : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateMutexExA(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateMutexExW(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binitialowner : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreatePrivateNamespaceA(lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreatePrivateNamespaceW(lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateProcessA(lpapplicationname : windows_sys::core::PCSTR, lpcommandline : windows_sys::core::PSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("advapi32.dll" "system" fn CreateProcessAsUserA(htoken : super::super::Foundation:: HANDLE, lpapplicationname : windows_sys::core::PCSTR, lpcommandline : windows_sys::core::PSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("advapi32.dll" "system" fn CreateProcessAsUserW(htoken : super::super::Foundation:: HANDLE, lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateProcessW(lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
windows_targets::link!("advapi32.dll" "system" fn CreateProcessWithLogonW(lpusername : windows_sys::core::PCWSTR, lpdomain : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, dwlogonflags : CREATE_PROCESS_LOGON_FLAGS, lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
windows_targets::link!("advapi32.dll" "system" fn CreateProcessWithTokenW(htoken : super::super::Foundation:: HANDLE, dwlogonflags : CREATE_PROCESS_LOGON_FLAGS, lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateRemoteThread(hprocess : super::super::Foundation:: HANDLE, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpthreadid : *mut u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateRemoteThreadEx(hprocess : super::super::Foundation:: HANDLE, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid : *mut u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreA(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreExA(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreExW(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreW(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateThread(lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : THREAD_CREATION_FLAGS, lpthreadid : *mut u32) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *const core::ffi::c_void) -> PTP_POOL);
windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> PTP_CLEANUP_GROUP);
windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolIo(fl : super::super::Foundation:: HANDLE, pfnio : PTP_WIN32_IO_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_IO);
windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolTimer(pfnti : PTP_TIMER_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_TIMER);
windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolWait(pfnwa : PTP_WAIT_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_WAIT);
windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : PTP_WORK_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_WORK);
windows_targets::link!("kernel32.dll" "system" fn CreateTimerQueue() -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn CreateTimerQueueTimer(phnewtimer : *mut super::super::Foundation:: HANDLE, timerqueue : super::super::Foundation:: HANDLE, callback : WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, flags : WORKER_THREAD_FLAGS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn CreateUmsCompletionList(umscompletionlist : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn CreateUmsThreadContext(lpumsthread : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerA(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, lptimername : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerExA(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lptimername : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lptimername : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, lptimername : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn DeleteBoundaryDescriptor(boundarydescriptor : super::super::Foundation:: HANDLE));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn DeleteCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
windows_targets::link!("kernel32.dll" "system" fn DeleteFiber(lpfiber : *const core::ffi::c_void));
windows_targets::link!("kernel32.dll" "system" fn DeleteProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST));
windows_targets::link!("kernel32.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn DeleteTimerQueue(timerqueue : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn DeleteTimerQueueEx(timerqueue : super::super::Foundation:: HANDLE, completionevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn DeleteTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, timer : super::super::Foundation:: HANDLE, completionevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn DeleteUmsCompletionList(umscompletionlist : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn DeleteUmsThreadContext(umsthread : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn DequeueUmsCompletionListItems(umscompletionlist : *const core::ffi::c_void, waittimeout : u32, umsthreadlist : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : PTP_CALLBACK_INSTANCE));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn EnterCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
windows_targets::link!("kernel32.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER, dwflags : u32) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("kernel32.dll" "system" fn EnterUmsSchedulingMode(schedulerstartupinfo : *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ExecuteUmsThread(umsthread : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : u32) -> !);
windows_targets::link!("kernel32.dll" "system" fn ExitThread(dwexitcode : u32) -> !);
windows_targets::link!("kernel32.dll" "system" fn FlsAlloc(lpcallback : PFLS_CALLBACK_FUNCTION) -> u32);
windows_targets::link!("kernel32.dll" "system" fn FlsFree(dwflsindex : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn FlsGetValue(dwflsindex : u32) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn FlsSetValue(dwflsindex : u32, lpflsdata : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn FlushProcessWriteBuffers());
windows_targets::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, r#mod : super::super::Foundation:: HMODULE));
windows_targets::link!("kernel32.dll" "system" fn GetActiveProcessorCount(groupnumber : u16) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetActiveProcessorGroupCount() -> u16);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcess() -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessorNumber() -> u32);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessorNumberEx(procnumber : *mut super::Kernel:: PROCESSOR_NUMBER));
windows_targets::link!("kernel32.dll" "system" fn GetCurrentThread() -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : *mut usize, highlimit : *mut usize));
windows_targets::link!("kernel32.dll" "system" fn GetCurrentUmsThread() -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn GetExitCodeProcess(hprocess : super::super::Foundation:: HANDLE, lpexitcode : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetExitCodeThread(hthread : super::super::Foundation:: HANDLE, lpexitcode : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("user32.dll" "system" fn GetGuiResources(hprocess : super::super::Foundation:: HANDLE, uiflags : GET_GUI_RESOURCES_FLAGS) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetMachineTypeAttributes(machine : u16, machinetypeattributes : *mut MACHINE_ATTRIBUTES) -> windows_sys::core::HRESULT);
windows_targets::link!("kernel32.dll" "system" fn GetMaximumProcessorCount(groupnumber : u16) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetMaximumProcessorGroupCount() -> u16);
windows_targets::link!("kernel32.dll" "system" fn GetNextUmsListItem(umscontext : *mut core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNode(node : u8, availablebytes : *mut u64) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNodeEx(node : u16, availablebytes : *mut u64) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNumaHighestNodeNumber(highestnodenumber : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeNumberFromHandle(hfile : super::super::Foundation:: HANDLE, nodenumber : *mut u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask(node : u8, processormask : *mut u64) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask2(nodenumber : u16, processormasks : *mut super::SystemInformation:: GROUP_AFFINITY, processormaskcount : u16, requiredmaskcount : *mut u16) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMaskEx(node : u16, processormask : *mut super::SystemInformation:: GROUP_AFFINITY) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNumaProcessorNode(processor : u8, nodenumber : *mut u8) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn GetNumaProcessorNodeEx(processor : *const super::Kernel:: PROCESSOR_NUMBER, nodenumber : *mut u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNumaProximityNode(proximityid : u32, nodenumber : *mut u8) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNumaProximityNodeEx(proximityid : u32, nodenumber : *mut u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetPriorityClass(hprocess : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProcessAffinityMask(hprocess : super::super::Foundation:: HANDLE, lpprocessaffinitymask : *mut usize, lpsystemaffinitymask : *mut usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessDEPPolicy(hprocess : super::super::Foundation:: HANDLE, lpflags : *mut u32, lppermanent : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSetMasks(process : super::super::Foundation:: HANDLE, cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSets(process : super::super::Foundation:: HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessGroupAffinity(hprocess : super::super::Foundation:: HANDLE, groupcount : *mut u16, grouparray : *mut u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessHandleCount(hprocess : super::super::Foundation:: HANDLE, pdwhandlecount : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("oleacc.dll" "system" fn GetProcessHandleFromHwnd(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn GetProcessId(process : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProcessIdOfThread(thread : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProcessInformation(hprocess : super::super::Foundation:: HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *mut core::ffi::c_void, processinformationsize : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessIoCounters(hprocess : super::super::Foundation:: HANDLE, lpiocounters : *mut IO_COUNTERS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessMitigationPolicy(hprocess : super::super::Foundation:: HANDLE, mitigationpolicy : PROCESS_MITIGATION_POLICY, lpbuffer : *mut core::ffi::c_void, dwlength : usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessPriorityBoost(hprocess : super::super::Foundation:: HANDLE, pdisablepriorityboost : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessShutdownParameters(lpdwlevel : *mut u32, lpdwflags : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessTimes(hprocess : super::super::Foundation:: HANDLE, lpcreationtime : *mut super::super::Foundation:: FILETIME, lpexittime : *mut super::super::Foundation:: FILETIME, lpkerneltime : *mut super::super::Foundation:: FILETIME, lpusertime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetProcessVersion(processid : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProcessWorkingSetSize(hprocess : super::super::Foundation:: HANDLE, lpminimumworkingsetsize : *mut usize, lpmaximumworkingsetsize : *mut usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetStartupInfoA(lpstartupinfo : *mut STARTUPINFOA));
windows_targets::link!("kernel32.dll" "system" fn GetStartupInfoW(lpstartupinfo : *mut STARTUPINFOW));
windows_targets::link!("kernel32.dll" "system" fn GetSystemTimes(lpidletime : *mut super::super::Foundation:: FILETIME, lpkerneltime : *mut super::super::Foundation:: FILETIME, lpusertime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetThreadDescription(hthread : super::super::Foundation:: HANDLE, ppszthreaddescription : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn GetThreadGroupAffinity(hthread : super::super::Foundation:: HANDLE, groupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetThreadIOPendingFlag(hthread : super::super::Foundation:: HANDLE, lpioispending : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetThreadId(thread : super::super::Foundation:: HANDLE) -> u32);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn GetThreadIdealProcessorEx(hthread : super::super::Foundation:: HANDLE, lpidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetThreadInformation(hthread : super::super::Foundation:: HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *mut core::ffi::c_void, threadinformationsize : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetThreadPriority(hthread : super::super::Foundation:: HANDLE) -> i32);
windows_targets::link!("kernel32.dll" "system" fn GetThreadPriorityBoost(hthread : super::super::Foundation:: HANDLE, pdisablepriorityboost : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSetMasks(thread : super::super::Foundation:: HANDLE, cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSets(thread : super::super::Foundation:: HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetThreadTimes(hthread : super::super::Foundation:: HANDLE, lpcreationtime : *mut super::super::Foundation:: FILETIME, lpexittime : *mut super::super::Foundation:: FILETIME, lpkerneltime : *mut super::super::Foundation:: FILETIME, lpusertime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetUmsCompletionListEvent(umscompletionlist : *const core::ffi::c_void, umscompletionevent : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetUmsSystemThreadInformation(threadhandle : super::super::Foundation:: HANDLE, systemthreadinfo : *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn InitOnceBeginInitialize(lpinitonce : *mut INIT_ONCE, dwflags : u32, fpending : *mut super::super::Foundation:: BOOL, lpcontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn InitOnceComplete(lpinitonce : *mut INIT_ONCE, dwflags : u32, lpcontext : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn InitOnceExecuteOnce(initonce : *mut INIT_ONCE, initfn : PINIT_ONCE_FN, parameter : *mut core::ffi::c_void, context : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn InitOnceInitialize(initonce : *mut INIT_ONCE));
windows_targets::link!("kernel32.dll" "system" fn InitializeConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InitializeCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32, flags : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn InitializeProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount : u32, dwflags : u32, lpsize : *mut usize) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : *mut super::Kernel:: SLIST_HEADER));
windows_targets::link!("kernel32.dll" "system" fn InitializeSRWLock(srwlock : *mut SRWLOCK));
windows_targets::link!("kernel32.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER, ltotalthreads : i32, lspincount : i32) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : *mut super::Kernel:: SLIST_HEADER) -> *mut super::Kernel:: SLIST_ENTRY);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : *mut super::Kernel:: SLIST_HEADER) -> *mut super::Kernel:: SLIST_ENTRY);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::Kernel:: SLIST_HEADER, listentry : *mut super::Kernel:: SLIST_ENTRY) -> *mut super::Kernel:: SLIST_ENTRY);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::Kernel:: SLIST_HEADER, list : *mut super::Kernel:: SLIST_ENTRY, listend : *mut super::Kernel:: SLIST_ENTRY, count : u32) -> *mut super::Kernel:: SLIST_ENTRY);
windows_targets::link!("user32.dll" "system" fn IsImmersiveProcess(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsProcessCritical(hprocess : super::super::Foundation:: HANDLE, critical : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsProcessorFeaturePresent(processorfeature : PROCESSOR_FEATURE_ID) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsThreadAFiber() -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : PTP_TIMER) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn IsWow64Process(hprocess : super::super::Foundation:: HANDLE, wow64process : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn IsWow64Process2(hprocess : super::super::Foundation:: HANDLE, pprocessmachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE, pnativemachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn LeaveCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, pcs : *mut CRITICAL_SECTION));
windows_targets::link!("kernel32.dll" "system" fn OpenEventA(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenMutexW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenPrivateNamespaceA(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenPrivateNamespaceW(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : PROCESS_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, dwprocessid : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : super::super::Foundation:: HANDLE, desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK, tokenhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn OpenSemaphoreW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenThread(dwdesiredaccess : THREAD_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, dwthreadid : u32) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("advapi32.dll" "system" fn OpenThreadToken(threadhandle : super::super::Foundation:: HANDLE, desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK, openasself : super::super::Foundation:: BOOL, tokenhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn OpenWaitableTimerA(dwdesiredaccess : u32, binherithandle : super::super::Foundation:: BOOL, lptimername : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn OpenWaitableTimerW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lptimername : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn PulseEvent(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : *const super::Kernel:: SLIST_HEADER) -> u16);
windows_targets::link!("kernel32.dll" "system" fn QueryFullProcessImageNameA(hprocess : super::super::Foundation:: HANDLE, dwflags : PROCESS_NAME_FORMAT, lpexename : windows_sys::core::PSTR, lpdwsize : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryFullProcessImageNameW(hprocess : super::super::Foundation:: HANDLE, dwflags : PROCESS_NAME_FORMAT, lpexename : windows_sys::core::PWSTR, lpdwsize : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryProcessAffinityUpdateMode(hprocess : super::super::Foundation:: HANDLE, lpdwflags : *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : *const windows_sys::core::GUID, policyvalue : *mut usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : PTP_POOL, ptpsi : *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueryUmsThreadInformation(umsthread : *const core::ffi::c_void, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : *mut core::ffi::c_void, umsthreadinformationlength : u32, returnlength : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueueUserAPC(pfnapc : super::super::Foundation:: PAPCFUNC, hthread : super::super::Foundation:: HANDLE, dwdata : usize) -> u32);
windows_targets::link!("kernel32.dll" "system" fn QueueUserAPC2(apcroutine : super::super::Foundation:: PAPCFUNC, thread : super::super::Foundation:: HANDLE, data : usize, flags : QUEUE_USER_APC_FLAGS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn QueueUserWorkItem(function : LPTHREAD_START_ROUTINE, context : *const core::ffi::c_void, flags : WORKER_THREAD_FLAGS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn RegisterWaitForSingleObject(phnewwaitobject : *mut super::super::Foundation:: HANDLE, hobject : super::super::Foundation:: HANDLE, callback : WAITORTIMERCALLBACK, context : *const core::ffi::c_void, dwmilliseconds : u32, dwflags : WORKER_THREAD_FLAGS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, r#mut : super::super::Foundation:: HANDLE));
windows_targets::link!("kernel32.dll" "system" fn ReleaseSRWLockExclusive(srwlock : *mut SRWLOCK));
windows_targets::link!("kernel32.dll" "system" fn ReleaseSRWLockShared(srwlock : *mut SRWLOCK));
windows_targets::link!("kernel32.dll" "system" fn ReleaseSemaphore(hsemaphore : super::super::Foundation:: HANDLE, lreleasecount : i32, lppreviouscount : *mut i32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, sem : super::super::Foundation:: HANDLE, crel : u32));
windows_targets::link!("kernel32.dll" "system" fn ResetEvent(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn ResumeThread(hthread : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("rtworkq.dll" "system" fn RtwqAddPeriodicCallback(callback : RTWQPERIODICCALLBACK, context : * mut core::ffi::c_void, key : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqAllocateSerialWorkQueue(workqueueidin : u32, workqueueidout : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqAllocateWorkQueue(workqueuetype : RTWQ_WORKQUEUE_TYPE, workqueueid : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqBeginRegisterWorkQueueWithMMCSS(workqueueid : u32, usageclass : windows_sys::core::PCWSTR, dwtaskid : u32, lpriority : i32, donecallback : * mut core::ffi::c_void, donestate : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqBeginUnregisterWorkQueueWithMMCSS(workqueueid : u32, donecallback : * mut core::ffi::c_void, donestate : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqCancelDeadline(prequest : super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqCancelWorkItem(key : u64) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqCreateAsyncResult(appobject : * mut core::ffi::c_void, callback : * mut core::ffi::c_void, appstate : * mut core::ffi::c_void, asyncresult : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqEndRegisterWorkQueueWithMMCSS(result : * mut core::ffi::c_void, taskid : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSClass(workqueueid : u32, usageclass : windows_sys::core::PWSTR, usageclasslength : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSPriority(workqueueid : u32, priority : *mut i32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSTaskId(workqueueid : u32, taskid : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqInvokeCallback(result : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqJoinWorkQueue(workqueueid : u32, hfile : super::super::Foundation:: HANDLE, out : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqLockPlatform() -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqLockSharedWorkQueue(usageclass : windows_sys::core::PCWSTR, basepriority : i32, taskid : *mut u32, id : *mut u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqLockWorkQueue(workqueueid : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqPutWaitingWorkItem(hevent : super::super::Foundation:: HANDLE, lpriority : i32, result : * mut core::ffi::c_void, key : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqPutWorkItem(dwqueue : u32, lpriority : i32, result : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformEvents(platformevents : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformWithMMCSS(usageclass : windows_sys::core::PCWSTR, taskid : *mut u32, lpriority : i32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqRemovePeriodicCallback(dwkey : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqScheduleWorkItem(result : * mut core::ffi::c_void, timeout : i64, key : *mut u64) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqSetDeadline(workqueueid : u32, deadlineinhns : i64, prequest : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqSetDeadline2(workqueueid : u32, deadlineinhns : i64, predeadlineinhns : i64, prequest : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqSetLongRunning(workqueueid : u32, enable : super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqShutdown() -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqStartup() -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqUnjoinWorkQueue(workqueueid : u32, hfile : super::super::Foundation:: HANDLE) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqUnlockPlatform() -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqUnlockWorkQueue(workqueueid : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformEvents(platformevents : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformFromMMCSS() -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn SetEvent(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, evt : super::super::Foundation:: HANDLE));
windows_targets::link!("kernel32.dll" "system" fn SetPriorityClass(hprocess : super::super::Foundation:: HANDLE, dwpriorityclass : PROCESS_CREATION_FLAGS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessAffinityMask(hprocess : super::super::Foundation:: HANDLE, dwprocessaffinitymask : usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessAffinityUpdateMode(hprocess : super::super::Foundation:: HANDLE, dwflags : PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessDEPPolicy(dwflags : PROCESS_DEP_FLAGS) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSetMasks(process : super::super::Foundation:: HANDLE, cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSets(process : super::super::Foundation:: HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessDynamicEHContinuationTargets(process : super::super::Foundation:: HANDLE, numberoftargets : u16, targets : *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessDynamicEnforcedCetCompatibleRanges(process : super::super::Foundation:: HANDLE, numberofranges : u16, ranges : *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessInformation(hprocess : super::super::Foundation:: HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *const core::ffi::c_void, processinformationsize : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessMitigationPolicy(mitigationpolicy : PROCESS_MITIGATION_POLICY, lpbuffer : *const core::ffi::c_void, dwlength : usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessPriorityBoost(hprocess : super::super::Foundation:: HANDLE, bdisablepriorityboost : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("user32.dll" "system" fn SetProcessRestrictionExemption(fenableexemption : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessShutdownParameters(dwlevel : u32, dwflags : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProcessWorkingSetSize(hprocess : super::super::Foundation:: HANDLE, dwminimumworkingsetsize : usize, dwmaximumworkingsetsize : usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : *const windows_sys::core::GUID, policyvalue : usize, oldpolicyvalue : *mut usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadAffinityMask(hthread : super::super::Foundation:: HANDLE, dwthreadaffinitymask : usize) -> usize);
windows_targets::link!("kernel32.dll" "system" fn SetThreadDescription(hthread : super::super::Foundation:: HANDLE, lpthreaddescription : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn SetThreadGroupAffinity(hthread : super::super::Foundation:: HANDLE, groupaffinity : *const super::SystemInformation:: GROUP_AFFINITY, previousgroupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadIdealProcessor(hthread : super::super::Foundation:: HANDLE, dwidealprocessor : u32) -> u32);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn SetThreadIdealProcessorEx(hthread : super::super::Foundation:: HANDLE, lpidealprocessor : *const super::Kernel:: PROCESSOR_NUMBER, lppreviousidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadInformation(hthread : super::super::Foundation:: HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *const core::ffi::c_void, threadinformationsize : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadPriority(hthread : super::super::Foundation:: HANDLE, npriority : THREAD_PRIORITY) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadPriorityBoost(hthread : super::super::Foundation:: HANDLE, bdisablepriorityboost : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSetMasks(thread : super::super::Foundation:: HANDLE, cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSets(thread : super::super::Foundation:: HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadStackGuarantee(stacksizeinbytes : *mut u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("advapi32.dll" "system" fn SetThreadToken(thread : *const super::super::Foundation:: HANDLE, token : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : PTP_POOL, ptpsi : *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : PTP_POOL, cthrdmost : u32));
windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : PTP_POOL, cthrdmic : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : PTP_TIMER, pftduetime : *const super::super::Foundation:: FILETIME, msperiod : u32, mswindowlength : u32));
windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : PTP_TIMER, pftduetime : *const super::super::Foundation:: FILETIME, msperiod : u32, mswindowlength : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : PTP_WAIT, h : super::super::Foundation:: HANDLE, pfttimeout : *const super::super::Foundation:: FILETIME));
windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : PTP_WAIT, h : super::super::Foundation:: HANDLE, pfttimeout : *const super::super::Foundation:: FILETIME, reserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, callback : WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, preferio : super::super::Foundation:: BOOL) -> super::super::Foundation:: HANDLE);
windows_targets::link!("kernel32.dll" "system" fn SetUmsThreadInformation(umsthread : *const core::ffi::c_void, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : *const core::ffi::c_void, umsthreadinformationlength : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetWaitableTimer(htimer : super::super::Foundation:: HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, fresume : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetWaitableTimerEx(htimer : super::super::Foundation:: HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, wakecontext : *const REASON_CONTEXT, tolerabledelay : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SignalObjectAndWait(hobjecttosignal : super::super::Foundation:: HANDLE, hobjecttowaiton : super::super::Foundation:: HANDLE, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: WAIT_EVENT);
windows_targets::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn SleepConditionVariableCS(conditionvariable : *mut CONDITION_VARIABLE, criticalsection : *mut CRITICAL_SECTION, dwmilliseconds : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SleepConditionVariableSRW(conditionvariable : *mut CONDITION_VARIABLE, srwlock : *mut SRWLOCK, dwmilliseconds : u32, flags : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn SleepEx(dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> u32);
windows_targets::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : PTP_IO));
windows_targets::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : PTP_WORK));
windows_targets::link!("kernel32.dll" "system" fn SuspendThread(hthread : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("kernel32.dll" "system" fn SwitchToFiber(lpfiber : *const core::ffi::c_void));
windows_targets::link!("kernel32.dll" "system" fn SwitchToThread() -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn TerminateProcess(hprocess : super::super::Foundation:: HANDLE, uexitcode : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn TerminateThread(hthread : super::super::Foundation:: HANDLE, dwexitcode : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn TlsAlloc() -> u32);
windows_targets::link!("kernel32.dll" "system" fn TlsFree(dwtlsindex : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn TlsGetValue(dwtlsindex : u32) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn TlsSetValue(dwtlsindex : u32, lptlsvalue : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : *mut SRWLOCK) -> super::super::Foundation:: BOOLEAN);
windows_targets::link!("kernel32.dll" "system" fn TryAcquireSRWLockShared(srwlock : *mut SRWLOCK) -> super::super::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("kernel32.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn UmsThreadYield(schedulerparam : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn UnregisterWait(waithandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle : super::super::Foundation:: HANDLE, completionevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn UpdateProcThreadAttribute(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwflags : u32, attribute : usize, lpvalue : *const core::ffi::c_void, cbsize : usize, lppreviousvalue : *mut core::ffi::c_void, lpreturnsize : *const usize) -> super::super::Foundation:: BOOL);
windows_targets::link!("user32.dll" "system" fn WaitForInputIdle(hprocess : super::super::Foundation:: HANDLE, dwmilliseconds : u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn WaitForMultipleObjects(ncount : u32, lphandles : *const super::super::Foundation:: HANDLE, bwaitall : super::super::Foundation:: BOOL, dwmilliseconds : u32) -> super::super::Foundation:: WAIT_EVENT);
windows_targets::link!("kernel32.dll" "system" fn WaitForMultipleObjectsEx(ncount : u32, lphandles : *const super::super::Foundation:: HANDLE, bwaitall : super::super::Foundation:: BOOL, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: WAIT_EVENT);
windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : super::super::Foundation:: HANDLE, dwmilliseconds : u32) -> super::super::Foundation:: WAIT_EVENT);
windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : super::super::Foundation:: HANDLE, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: WAIT_EVENT);
windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : PTP_IO, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : PTP_TIMER, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : PTP_WAIT, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : PTP_WORK, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
windows_targets::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WaitOnAddress(address : *const core::ffi::c_void, compareaddress : *const core::ffi::c_void, addresssize : usize, dwmilliseconds : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("kernel32.dll" "system" fn WakeAllConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
windows_targets::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressAll(address : *const core::ffi::c_void));
windows_targets::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressSingle(address : *const core::ffi::c_void));
windows_targets::link!("kernel32.dll" "system" fn WakeConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
windows_targets::link!("kernel32.dll" "system" fn WinExec(lpcmdline : windows_sys::core::PCSTR, ucmdshow : u32) -> u32);
windows_targets::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn Wow64SetThreadDefaultGuestMachine(machine : u16) -> u16);
windows_targets::link!("kernel32.dll" "system" fn Wow64SuspendThread(hthread : super::super::Foundation:: HANDLE) -> u32);
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32768u32;
pub const ALL_PROCESSOR_GROUPS: u16 = 65535u16;
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = 2i32;
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = 1i32;
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = -1i32;
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = 0i32;
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = -2i32;
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 16384u32;
pub const CONDITION_VARIABLE_INIT: CONDITION_VARIABLE = CONDITION_VARIABLE { Ptr: core::ptr::null_mut() };
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = 16777216u32;
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = 67108864u32;
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = 2u32;
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = 1u32;
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = 8192u32;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS = 2147483648u32;
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = 16u32;
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = 512u32;
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = 134217728u32;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS = 33554432u32;
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = 262144u32;
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = 4194304u32;
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = 2048u32;
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = 4096u32;
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = 4u32;
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = 1024u32;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = 2u32;
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = 1u32;
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = 8u32;
pub const EVENT_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031619u32;
pub const EVENT_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 2u32;
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = 524288u32;
pub const FLS_OUT_OF_INDEXES: u32 = 4294967295u32;
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = 0u32;
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 2u32;
pub const GR_GLOBAL: GET_GUI_RESOURCES_FLAGS = 4294967294u32;
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = 1u32;
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = 4u32;
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 128u32;
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 64u32;
pub const INFINITE: u32 = 4294967295u32;
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = 131072u32;
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = 65536u32;
pub const INIT_ONCE_ASYNC: u32 = 2u32;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
pub const INIT_ONCE_STATIC_INIT: INIT_ONCE = INIT_ONCE { Ptr: core::ptr::null_mut() };
pub const KernelEnabled: MACHINE_ATTRIBUTES = 2i32;
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = 2u32;
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = 1u32;
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = 4u32;
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = 2u32;
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = 3u32;
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = 5u32;
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = 1u32;
pub const MUTEX_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031617u32;
pub const MUTEX_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 1u32;
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = 20i32;
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 32u32;
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 7u32;
pub const PF_ALPHA_BYTE_INSTRUCTIONS: PROCESSOR_FEATURE_ID = 5u32;
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = 25u32;
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 24u32;
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = 26u32;
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 27u32;
pub const PF_ARM_NEON_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 19u32;
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 34u32;
pub const PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 43u32;
pub const PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 44u32;
pub const PF_ARM_V83_LRCPC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 45u32;
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 31u32;
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 30u32;
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 29u32;
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = 18u32;
pub const PF_AVX2_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 40u32;
pub const PF_AVX512F_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 41u32;
pub const PF_AVX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 39u32;
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = 16u32;
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = 15u32;
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = 14u32;
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = 2u32;
pub const PF_ERMS_AVAILABLE: PROCESSOR_FEATURE_ID = 42u32;
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = 23u32;
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = 1u32;
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = 0u32;
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 3u32;
pub const PF_MONITORX_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 35u32;
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = 12u32;
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = 9u32;
pub const PF_PPC_MOVEMEM_64BIT_OK: PROCESSOR_FEATURE_ID = 4u32;
pub const PF_RDPID_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 33u32;
pub const PF_RDRAND_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 28u32;
pub const PF_RDTSCP_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 32u32;
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = 8u32;
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = 22u32;
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = 20u32;
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 13u32;
pub const PF_SSE4_1_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 37u32;
pub const PF_SSE4_2_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 38u32;
pub const PF_SSE_DAZ_MODE_AVAILABLE: PROCESSOR_FEATURE_ID = 11u32;
pub const PF_SSSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 36u32;
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = 21u32;
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 10u32;
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = 6u32;
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = 17u32;
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = 0i32;
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = 1i32;
pub const PME_CURRENT_VERSION: u32 = 1u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS = 2u32;
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS = 1u32;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 0u32;
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = 1u32;
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = 2097151u32;
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = 128u32;
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = 2u32;
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = 65536u32;
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = 2u32;
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = 1u32;
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = 0u32;
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = 64u32;
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS = 1048576u32;
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = 2097152u32;
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = 1u32;
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = 0u32;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = 1024u32;
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 4096u32;
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = 131072u32;
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = 512u32;
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = 8192u32;
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = 256u32;
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = 4u32;
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS = 983040u32;
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = 2048u32;
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = 1048576u32;
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = 1u32;
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = 8u32;
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = 16u32;
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = 32u32;
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = 262144u32;
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = 524288u32;
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131087u32;
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131086u32;
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 131098u32;
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131090u32;
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196635u32;
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 196611u32;
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 131074u32;
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196613u32;
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 131085u32;
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 131097u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 131096u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 131079u32;
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 131072u32;
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 131076u32;
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 131083u32;
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094u32;
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 131081u32;
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 196614u32;
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 131088u32;
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = 536870912u32;
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = 1073741824u32;
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = 268435456u32;
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL = 3u32;
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = 7u32;
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = 6u32;
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = 4u32;
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = 4294967294u32;
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = 8u32;
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = 1u32;
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = 2u32;
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = 5u32;
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = 0u32;
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = 15u32;
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = 14u32;
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = 26u32;
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = 18u32;
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = 27u32;
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = 3u32;
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = 2u32;
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = 5u32;
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = 13u32;
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = 25u32;
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = 24u32;
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = 7u32;
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = 0u32;
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = 4u32;
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = 11u32;
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = 22u32;
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = 17u32;
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = 9u32;
pub const ProcThreadAttributeTrustedApp: PROC_THREAD_ATTRIBUTE_NUM = 29u32;
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = 6u32;
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = 16u32;
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = 1i32;
pub const ProcessActivationContextTrustPolicy: PROCESS_MITIGATION_POLICY = 19i32;
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = 2i32;
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = 13i32;
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = 7i32;
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = 0i32;
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = 2i32;
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = 6i32;
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = 9i32;
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = 10i32;
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = 3i32;
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = 12i32;
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = 8i32;
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = 9i32;
pub const ProcessMaxOverridePrefetchParameter: PROCESS_INFORMATION_CLASS = 11i32;
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = 1i32;
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = 0i32;
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = 5i32;
pub const ProcessOverrideSubsequentPrefetchParameter: PROCESS_INFORMATION_CLASS = 10i32;
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = 12i32;
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = 4i32;
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = 7i32;
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = 16i32;
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = 5i32;
pub const ProcessSEHOPPolicy: PROCESS_MITIGATION_POLICY = 18i32;
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = 14i32;
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = 8i32;
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = 3i32;
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = 4i32;
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = 11i32;
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = 6i32;
pub const ProcessUserPointerAuthPolicy: PROCESS_MITIGATION_POLICY = 17i32;
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = 15i32;
pub const QUEUE_USER_APC_CALLBACK_DATA_CONTEXT: QUEUE_USER_APC_FLAGS = 65536i32;
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = 0i32;
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = 1i32;
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = 256u32;
pub const RTL_CRITICAL_SECTION_ALL_FLAG_BITS: u32 = 4278190080u32;
pub const RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT: u32 = 1u32;
pub const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: u32 = 33554432u32;
pub const RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO: u32 = 268435456u32;
pub const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: u32 = 16777216u32;
pub const RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE: u32 = 134217728u32;
pub const RTL_CRITICAL_SECTION_FLAG_STATIC_INIT: u32 = 67108864u32;
pub const RTWQ_MULTITHREADED_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 2i32;
pub const RTWQ_STANDARD_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 0i32;
pub const RTWQ_WINDOW_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 1i32;
pub const SEMAPHORE_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031619u32;
pub const SEMAPHORE_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 2u32;
pub const SRWLOCK_INIT: SRWLOCK = SRWLOCK { Ptr: core::ptr::null_mut() };
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS = 65536u32;
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = 128u32;
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = 64u32;
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = 8192u32;
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = 32u32;
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = 4096u32;
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = 2048u32;
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = 32768u32;
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = 8u32;
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = 16u32;
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = 512u32;
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = 4u32;
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = 1u32;
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = 2u32;
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = 256u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
pub const SYNCHRONIZATION_DELETE: SYNCHRONIZATION_ACCESS_RIGHTS = 65536u32;
pub const SYNCHRONIZATION_READ_CONTROL: SYNCHRONIZATION_ACCESS_RIGHTS = 131072u32;
pub const SYNCHRONIZATION_SYNCHRONIZE: SYNCHRONIZATION_ACCESS_RIGHTS = 1048576u32;
pub const SYNCHRONIZATION_WRITE_DAC: SYNCHRONIZATION_ACCESS_RIGHTS = 262144u32;
pub const SYNCHRONIZATION_WRITE_OWNER: SYNCHRONIZATION_ACCESS_RIGHTS = 524288u32;
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = 2097151u32;
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = 0u32;
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = 4u32;
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = 65536u32;
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = 512u32;
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = 8u32;
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = 256u32;
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = 65536i32;
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = 131072i32;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = 1i32;
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = -1i32;
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = 2i32;
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = -15i32;
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = -2i32;
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = -2i32;
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = 0i32;
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = 15i32;
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = 64u32;
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 2048u32;
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = 131072u32;
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = 4096u32;
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = 16u32;
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = 32u32;
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = 1024u32;
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = 128u32;
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = 983040u32;
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = 2u32;
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = 1048576u32;
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = 1u32;
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = 262144u32;
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = 524288u32;
pub const TIMER_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = 2031619u32;
pub const TIMER_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 2u32;
pub const TIMER_QUERY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = 1u32;
pub const TLS_OUT_OF_INDEXES: u32 = 4294967295u32;
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = 3i32;
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = 0i32;
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = 3i32;
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = 2i32;
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1i32;
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = 1i32;
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = 2i32;
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = 4i32;
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = 0i32;
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = 3i32;
pub const UmsThreadAffinity: UMS_THREAD_INFO_CLASS = 3i32;
pub const UmsThreadInvalidInfoClass: UMS_THREAD_INFO_CLASS = 0i32;
pub const UmsThreadIsSuspended: UMS_THREAD_INFO_CLASS = 5i32;
pub const UmsThreadIsTerminated: UMS_THREAD_INFO_CLASS = 6i32;
pub const UmsThreadMaxInfoClass: UMS_THREAD_INFO_CLASS = 7i32;
pub const UmsThreadPriority: UMS_THREAD_INFO_CLASS = 2i32;
pub const UmsThreadTeb: UMS_THREAD_INFO_CLASS = 4i32;
pub const UmsThreadUserContext: UMS_THREAD_INFO_CLASS = 1i32;
pub const UserEnabled: MACHINE_ATTRIBUTES = 1i32;
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = 0u32;
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = 1u32;
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = 128u32;
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = 32u32;
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = 4u32;
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = 16u32;
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = 8u32;
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = 256u32;
pub const Wow64Container: MACHINE_ATTRIBUTES = 4i32;
pub type AVRT_PRIORITY = i32;
pub type CREATE_EVENT = u32;
pub type CREATE_PROCESS_LOGON_FLAGS = u32;
pub type GET_GUI_RESOURCES_FLAGS = u32;
pub type MACHINE_ATTRIBUTES = i32;
pub type MEMORY_PRIORITY = u32;
pub type POWER_REQUEST_CONTEXT_FLAGS = u32;
pub type PROCESSOR_FEATURE_ID = u32;
pub type PROCESS_ACCESS_RIGHTS = u32;
pub type PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = u32;
pub type PROCESS_CREATION_FLAGS = u32;
pub type PROCESS_DEP_FLAGS = u32;
pub type PROCESS_INFORMATION_CLASS = i32;
pub type PROCESS_MEMORY_EXHAUSTION_TYPE = i32;
pub type PROCESS_MITIGATION_POLICY = i32;
pub type PROCESS_NAME_FORMAT = u32;
pub type PROCESS_PROTECTION_LEVEL = u32;
pub type PROC_THREAD_ATTRIBUTE_NUM = u32;
pub type QUEUE_USER_APC_FLAGS = i32;
pub type RTWQ_WORKQUEUE_TYPE = i32;
pub type STARTUPINFOW_FLAGS = u32;
pub type SYNCHRONIZATION_ACCESS_RIGHTS = u32;
pub type THREAD_ACCESS_RIGHTS = u32;
pub type THREAD_CREATION_FLAGS = u32;
pub type THREAD_INFORMATION_CLASS = i32;
pub type THREAD_PRIORITY = i32;
pub type TP_CALLBACK_PRIORITY = i32;
pub type UMS_THREAD_INFO_CLASS = i32;
pub type WORKER_THREAD_FLAGS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONDITION_VARIABLE {
    pub Ptr: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct CRITICAL_SECTION {
    pub DebugInfo: *mut CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: super::super::Foundation::HANDLE,
    pub LockSemaphore: super::super::Foundation::HANDLE,
    pub SpinCount: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut CRITICAL_SECTION,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub Identifier: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INIT_ONCE {
    pub Ptr: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OVERRIDE_PREFETCH_PARAMETER {
    pub Value: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut core::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut core::ffi::c_void,
    pub Reserved5: *mut core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct PROCESS_BASIC_INFORMATION {
    pub ExitStatus: super::super::Foundation::NTSTATUS,
    pub PebBaseAddress: *mut PEB,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub UniqueProcessId: usize,
    pub InheritedFromUniqueProcessId: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_SystemInformation")]
#[derive(Clone, Copy)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: super::SystemInformation::IMAGE_FILE_MACHINE,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
pub type PTP_CALLBACK_INSTANCE = isize;
pub type PTP_CLEANUP_GROUP = isize;
pub type PTP_IO = isize;
pub type PTP_POOL = isize;
pub type PTP_TIMER = isize;
pub type PTP_WAIT = isize;
pub type PTP_WORK = isize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HMODULE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut core::ffi::c_void; 10],
    pub ImagePathName: super::super::Foundation::UNICODE_STRING,
    pub CommandLine: super::super::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SRWLOCK {
    pub Ptr: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: windows_sys::core::PSTR,
    pub lpDesktop: windows_sys::core::PSTR,
    pub lpTitle: windows_sys::core::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: windows_sys::core::PWSTR,
    pub lpDesktop: windows_sys::core::PWSTR,
    pub lpTitle: windows_sys::core::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYNCHRONIZATION_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct TEB {
    pub Reserved1: [*mut core::ffi::c_void; 12],
    pub ProcessEnvironmentBlock: *mut PEB,
    pub Reserved2: [*mut core::ffi::c_void; 399],
    pub Reserved3: [u8; 1952],
    pub TlsSlots: [*mut core::ffi::c_void; 64],
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut core::ffi::c_void; 26],
    pub ReservedForOle: *mut core::ffi::c_void,
    pub Reserved6: [*mut core::ffi::c_void; 4],
    pub TlsExpansionSlots: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PTP_CLEANUP_GROUP,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut core::ffi::c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_0,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TP_CALLBACK_ENVIRON_V3_0 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TP_CALLBACK_ENVIRON_V3_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[derive(Clone, Copy)]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut core::ffi::c_void,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
pub type APC_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(param0: u32, param1: *mut core::ffi::c_void, param2: *mut core::ffi::c_void)>;
pub type LPFIBER_START_ROUTINE = Option<unsafe extern "system" fn(lpfiberparameter: *mut core::ffi::c_void)>;
pub type LPTHREAD_START_ROUTINE = Option<unsafe extern "system" fn(lpthreadparameter: *mut core::ffi::c_void) -> u32>;
pub type PFLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(lpflsdata: *const core::ffi::c_void)>;
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(initonce: *mut INIT_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Win32_System_SystemServices")]
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = Option<unsafe extern "system" fn(reason: super::SystemServices::RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const core::ffi::c_void)>;
pub type PTIMERAPCROUTINE = Option<unsafe extern "system" fn(lpargtocompletionroutine: *const core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = Option<unsafe extern "system" fn(objectcontext: *mut core::ffi::c_void, cleanupcontext: *mut core::ffi::c_void)>;
pub type PTP_SIMPLE_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void)>;
pub type PTP_TIMER_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, timer: PTP_TIMER)>;
pub type PTP_WAIT_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, wait: PTP_WAIT, waitresult: u32)>;
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, overlapped: *mut core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: PTP_IO)>;
pub type PTP_WORK_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, work: PTP_WORK)>;
pub type RTWQPERIODICCALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type WAITORTIMERCALLBACK = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::super::Foundation::BOOLEAN)>;
pub type WORKERCALLBACKFUNC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
