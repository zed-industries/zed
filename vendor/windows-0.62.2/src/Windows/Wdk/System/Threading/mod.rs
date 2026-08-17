#[inline]
pub unsafe fn NtCancelTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, currentstate: Option<*mut bool>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCancelTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, currentstate : *mut bool) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtCancelTimer(timerhandle, currentstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NtCreateTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, timertype: super::super::super::Win32::System::Kernel::TIMER_TYPE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, timertype : super::super::super::Win32::System::Kernel:: TIMER_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtCreateTimer(timerhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, timertype) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenEvent(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenEvent(eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenEvent(eventhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn NtOpenProcess(processhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid: Option<*const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenProcess(processhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, clientid : *const super::super::super::Win32::System::WindowsProgramming:: CLIENT_ID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenProcess(processhandle as _, desiredaccess, objectattributes, clientid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtOpenTimer(timerhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn NtQueryInformationProcess(processhandle: super::super::super::Win32::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQueryInformationProcess(processhandle, processinformationclass, processinformation as _, processinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn NtQueryInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQueryInformationThread(threadhandle, threadinformationclass, threadinformation as _, threadinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn NtSetInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const core::ffi::c_void, threadinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtSetInformationThread(threadhandle, threadinformationclass, threadinformation, threadinformationlength) }
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn NtSetTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, duetime: *const i64, timerapcroutine: super::SystemServices::PTIMER_APC_ROUTINE, timercontext: Option<*const core::ffi::c_void>, resumetimer: bool, period: Option<i32>, previousstate: Option<*mut bool>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, duetime : *const i64, timerapcroutine : super::SystemServices:: PTIMER_APC_ROUTINE, timercontext : *const core::ffi::c_void, resumetimer : bool, period : i32, previousstate : *mut bool) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtSetTimer(timerhandle, duetime, timerapcroutine, timercontext.unwrap_or(core::mem::zeroed()) as _, resumetimer, period.unwrap_or(core::mem::zeroed()) as _, previousstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NtSetTimerEx(timerhandle: super::super::super::Win32::Foundation::HANDLE, timersetinformationclass: TIMER_SET_INFORMATION_CLASS, timersetinformation: Option<*mut core::ffi::c_void>, timersetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetTimerEx(timerhandle : super::super::super::Win32::Foundation:: HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : *mut core::ffi::c_void, timersetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtSetTimerEx(timerhandle, timersetinformationclass, timersetinformation.unwrap_or(core::mem::zeroed()) as _, timersetinformationlength) }
}
#[inline]
pub unsafe fn NtTerminateProcess(processhandle: Option<super::super::super::Win32::Foundation::HANDLE>, exitstatus: super::super::super::Win32::Foundation::NTSTATUS) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtTerminateProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, exitstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtTerminateProcess(processhandle.unwrap_or(core::mem::zeroed()) as _, exitstatus) }
}
#[inline]
pub unsafe fn NtWaitForSingleObject(handle: super::super::super::Win32::Foundation::HANDLE, alertable: bool, timeout: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : super::super::super::Win32::Foundation:: HANDLE, alertable : bool, timeout : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtWaitForSingleObject(handle, alertable, timeout as _) }
}
#[inline]
pub unsafe fn ZwCancelTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, currentstate: Option<*mut bool>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCancelTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, currentstate : *mut bool) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwCancelTimer(timerhandle, currentstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ZwCreateTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, timertype: super::super::super::Win32::System::Kernel::TIMER_TYPE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, timertype : super::super::super::Win32::System::Kernel:: TIMER_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwCreateTimer(timerhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, timertype) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenEvent(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenEvent(eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenEvent(eventhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn ZwOpenProcess(processhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid: Option<*const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenProcess(processhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, clientid : *const super::super::super::Win32::System::WindowsProgramming:: CLIENT_ID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenProcess(processhandle as _, desiredaccess, objectattributes, clientid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwOpenTimer(timerhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn ZwQueryInformationProcess(processhandle: super::super::super::Win32::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQueryInformationProcess(processhandle, processinformationclass, processinformation as _, processinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn ZwQueryInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQueryInformationThread(threadhandle, threadinformationclass, threadinformation as _, threadinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn ZwSetInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const core::ffi::c_void, threadinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwSetInformationThread(threadhandle, threadinformationclass, threadinformation, threadinformationlength) }
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn ZwSetTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, duetime: *const i64, timerapcroutine: super::SystemServices::PTIMER_APC_ROUTINE, timercontext: Option<*const core::ffi::c_void>, resumetimer: bool, period: Option<i32>, previousstate: Option<*mut bool>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, duetime : *const i64, timerapcroutine : super::SystemServices:: PTIMER_APC_ROUTINE, timercontext : *const core::ffi::c_void, resumetimer : bool, period : i32, previousstate : *mut bool) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwSetTimer(timerhandle, duetime, timerapcroutine, timercontext.unwrap_or(core::mem::zeroed()) as _, resumetimer, period.unwrap_or(core::mem::zeroed()) as _, previousstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ZwSetTimerEx(timerhandle: super::super::super::Win32::Foundation::HANDLE, timersetinformationclass: TIMER_SET_INFORMATION_CLASS, timersetinformation: Option<*mut core::ffi::c_void>, timersetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetTimerEx(timerhandle : super::super::super::Win32::Foundation:: HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : *mut core::ffi::c_void, timersetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwSetTimerEx(timerhandle, timersetinformationclass, timersetinformation.unwrap_or(core::mem::zeroed()) as _, timersetinformationlength) }
}
#[inline]
pub unsafe fn ZwTerminateProcess(processhandle: Option<super::super::super::Win32::Foundation::HANDLE>, exitstatus: super::super::super::Win32::Foundation::NTSTATUS) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwTerminateProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, exitstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwTerminateProcess(processhandle.unwrap_or(core::mem::zeroed()) as _, exitstatus) }
}
#[inline]
pub unsafe fn ZwWaitForSingleObject(handle: super::super::super::Win32::Foundation::HANDLE, alertable: bool, timeout: Option<*const i64>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwWaitForSingleObject(handle : super::super::super::Win32::Foundation:: HANDLE, alertable : bool, timeout : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwWaitForSingleObject(handle, alertable, timeout.unwrap_or(core::mem::zeroed()) as _) }
}
pub const MaxProcessInfoClass: PROCESSINFOCLASS = PROCESSINFOCLASS(83i32);
pub const MaxThreadInfoClass: THREADINFOCLASS = THREADINFOCLASS(56i32);
pub const MaxTimerInfoClass: TIMER_SET_INFORMATION_CLASS = TIMER_SET_INFORMATION_CLASS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESSINFOCLASS(pub i32);
pub const ProcessAccessToken: PROCESSINFOCLASS = PROCESSINFOCLASS(9i32);
pub const ProcessAffinityMask: PROCESSINFOCLASS = PROCESSINFOCLASS(21i32);
pub const ProcessAffinityUpdateMode: PROCESSINFOCLASS = PROCESSINFOCLASS(45i32);
pub const ProcessBasePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(5i32);
pub const ProcessBasicInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(0i32);
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = PROCESSINFOCLASS(29i32);
pub const ProcessCheckStackExtentsMode: PROCESSINFOCLASS = PROCESSINFOCLASS(59i32);
pub const ProcessCommandLineInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(60i32);
pub const ProcessCommitReleaseInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(65i32);
pub const ProcessCookie: PROCESSINFOCLASS = PROCESSINFOCLASS(36i32);
pub const ProcessCycleTime: PROCESSINFOCLASS = PROCESSINFOCLASS(38i32);
pub const ProcessDebugFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(31i32);
pub const ProcessDebugObjectHandle: PROCESSINFOCLASS = PROCESSINFOCLASS(30i32);
pub const ProcessDebugPort: PROCESSINFOCLASS = PROCESSINFOCLASS(7i32);
pub const ProcessDefaultHardErrorMode: PROCESSINFOCLASS = PROCESSINFOCLASS(12i32);
pub const ProcessDeviceMap: PROCESSINFOCLASS = PROCESSINFOCLASS(23i32);
pub const ProcessDynamicFunctionTableInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(53i32);
pub const ProcessEnableAlignmentFaultFixup: PROCESSINFOCLASS = PROCESSINFOCLASS(17i32);
pub const ProcessEnergyTrackingState: PROCESSINFOCLASS = PROCESSINFOCLASS(82i32);
pub const ProcessExceptionPort: PROCESSINFOCLASS = PROCESSINFOCLASS(8i32);
pub const ProcessExecuteFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(34i32);
pub const ProcessFaultInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(63i32);
pub const ProcessForegroundInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(25i32);
pub const ProcessGroupInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(47i32);
pub const ProcessHandleCheckingMode: PROCESSINFOCLASS = PROCESSINFOCLASS(54i32);
pub const ProcessHandleCount: PROCESSINFOCLASS = PROCESSINFOCLASS(20i32);
pub const ProcessHandleInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(51i32);
pub const ProcessHandleTable: PROCESSINFOCLASS = PROCESSINFOCLASS(58i32);
pub const ProcessHandleTracing: PROCESSINFOCLASS = PROCESSINFOCLASS(32i32);
pub const ProcessImageFileMapping: PROCESSINFOCLASS = PROCESSINFOCLASS(44i32);
pub const ProcessImageFileName: PROCESSINFOCLASS = PROCESSINFOCLASS(27i32);
pub const ProcessImageFileNameWin32: PROCESSINFOCLASS = PROCESSINFOCLASS(43i32);
pub const ProcessImageInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(37i32);
pub const ProcessInPrivate: PROCESSINFOCLASS = PROCESSINFOCLASS(70i32);
pub const ProcessInstrumentationCallback: PROCESSINFOCLASS = PROCESSINFOCLASS(40i32);
pub const ProcessIoCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(2i32);
pub const ProcessIoPortHandlers: PROCESSINFOCLASS = PROCESSINFOCLASS(13i32);
pub const ProcessIoPriority: PROCESSINFOCLASS = PROCESSINFOCLASS(33i32);
pub const ProcessKeepAliveCount: PROCESSINFOCLASS = PROCESSINFOCLASS(55i32);
pub const ProcessLUIDDeviceMapsEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(28i32);
pub const ProcessLdtInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(10i32);
pub const ProcessLdtSize: PROCESSINFOCLASS = PROCESSINFOCLASS(11i32);
pub const ProcessMemoryAllocationMode: PROCESSINFOCLASS = PROCESSINFOCLASS(46i32);
pub const ProcessMemoryExhaustion: PROCESSINFOCLASS = PROCESSINFOCLASS(62i32);
pub const ProcessMitigationPolicy: PROCESSINFOCLASS = PROCESSINFOCLASS(52i32);
pub const ProcessOwnerInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(49i32);
pub const ProcessPagePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(39i32);
pub const ProcessPooledUsageAndLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(14i32);
pub const ProcessPriorityBoost: PROCESSINFOCLASS = PROCESSINFOCLASS(22i32);
pub const ProcessPriorityClass: PROCESSINFOCLASS = PROCESSINFOCLASS(18i32);
pub const ProcessProtectionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(61i32);
pub const ProcessQuotaLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(1i32);
pub const ProcessRaisePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(6i32);
pub const ProcessRaiseUMExceptionOnInvalidHandleClose: PROCESSINFOCLASS = PROCESSINFOCLASS(71i32);
pub const ProcessReserved1Information: PROCESSINFOCLASS = PROCESSINFOCLASS(66i32);
pub const ProcessReserved2Information: PROCESSINFOCLASS = PROCESSINFOCLASS(67i32);
pub const ProcessRevokeFileHandles: PROCESSINFOCLASS = PROCESSINFOCLASS(56i32);
pub const ProcessSessionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(24i32);
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(75i32);
pub const ProcessSubsystemProcess: PROCESSINFOCLASS = PROCESSINFOCLASS(68i32);
pub const ProcessTelemetryIdInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(64i32);
pub const ProcessThreadStackAllocation: PROCESSINFOCLASS = PROCESSINFOCLASS(41i32);
pub const ProcessTimes: PROCESSINFOCLASS = PROCESSINFOCLASS(4i32);
pub const ProcessTlsInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(35i32);
pub const ProcessTokenVirtualizationEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(48i32);
pub const ProcessUserModeIOPL: PROCESSINFOCLASS = PROCESSINFOCLASS(16i32);
pub const ProcessVmCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(3i32);
pub const ProcessWin32kSyscallFilterInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(79i32);
pub const ProcessWindowInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(50i32);
pub const ProcessWorkingSetControl: PROCESSINFOCLASS = PROCESSINFOCLASS(57i32);
pub const ProcessWorkingSetWatch: PROCESSINFOCLASS = PROCESSINFOCLASS(15i32);
pub const ProcessWorkingSetWatchEx: PROCESSINFOCLASS = PROCESSINFOCLASS(42i32);
pub const ProcessWow64Information: PROCESSINFOCLASS = PROCESSINFOCLASS(26i32);
pub const ProcessWx86Information: PROCESSINFOCLASS = PROCESSINFOCLASS(19i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREADINFOCLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIMER_SET_INFORMATION_CLASS(pub i32);
pub const ThreadActualBasePriority: THREADINFOCLASS = THREADINFOCLASS(25i32);
pub const ThreadActualGroupAffinity: THREADINFOCLASS = THREADINFOCLASS(41i32);
pub const ThreadAffinityMask: THREADINFOCLASS = THREADINFOCLASS(4i32);
pub const ThreadAmILastThread: THREADINFOCLASS = THREADINFOCLASS(12i32);
pub const ThreadBasePriority: THREADINFOCLASS = THREADINFOCLASS(3i32);
pub const ThreadBasicInformation: THREADINFOCLASS = THREADINFOCLASS(0i32);
pub const ThreadBreakOnTermination: THREADINFOCLASS = THREADINFOCLASS(18i32);
pub const ThreadCSwitchMon: THREADINFOCLASS = THREADINFOCLASS(27i32);
pub const ThreadCSwitchPmu: THREADINFOCLASS = THREADINFOCLASS(28i32);
pub const ThreadCounterProfiling: THREADINFOCLASS = THREADINFOCLASS(32i32);
pub const ThreadCpuAccountingInformation: THREADINFOCLASS = THREADINFOCLASS(34i32);
pub const ThreadCycleTime: THREADINFOCLASS = THREADINFOCLASS(23i32);
pub const ThreadDescriptorTableEntry: THREADINFOCLASS = THREADINFOCLASS(6i32);
pub const ThreadDynamicCodePolicyInfo: THREADINFOCLASS = THREADINFOCLASS(42i32);
pub const ThreadEnableAlignmentFaultFixup: THREADINFOCLASS = THREADINFOCLASS(7i32);
pub const ThreadEventPair_Reusable: THREADINFOCLASS = THREADINFOCLASS(8i32);
pub const ThreadGroupInformation: THREADINFOCLASS = THREADINFOCLASS(30i32);
pub const ThreadHideFromDebugger: THREADINFOCLASS = THREADINFOCLASS(17i32);
pub const ThreadIdealProcessor: THREADINFOCLASS = THREADINFOCLASS(13i32);
pub const ThreadIdealProcessorEx: THREADINFOCLASS = THREADINFOCLASS(33i32);
pub const ThreadImpersonationToken: THREADINFOCLASS = THREADINFOCLASS(5i32);
pub const ThreadIoPriority: THREADINFOCLASS = THREADINFOCLASS(22i32);
pub const ThreadIsIoPending: THREADINFOCLASS = THREADINFOCLASS(16i32);
pub const ThreadIsTerminated: THREADINFOCLASS = THREADINFOCLASS(20i32);
pub const ThreadLastSystemCall: THREADINFOCLASS = THREADINFOCLASS(21i32);
pub const ThreadNameInformation: THREADINFOCLASS = THREADINFOCLASS(38i32);
pub const ThreadPagePriority: THREADINFOCLASS = THREADINFOCLASS(24i32);
pub const ThreadPerformanceCount: THREADINFOCLASS = THREADINFOCLASS(11i32);
pub const ThreadPriority: THREADINFOCLASS = THREADINFOCLASS(2i32);
pub const ThreadPriorityBoost: THREADINFOCLASS = THREADINFOCLASS(14i32);
pub const ThreadQuerySetWin32StartAddress: THREADINFOCLASS = THREADINFOCLASS(9i32);
pub const ThreadSetTlsArrayAddress: THREADINFOCLASS = THREADINFOCLASS(15i32);
pub const ThreadSubsystemInformation: THREADINFOCLASS = THREADINFOCLASS(45i32);
pub const ThreadSuspendCount: THREADINFOCLASS = THREADINFOCLASS(35i32);
pub const ThreadSwitchLegacyState: THREADINFOCLASS = THREADINFOCLASS(19i32);
pub const ThreadTebInformation: THREADINFOCLASS = THREADINFOCLASS(26i32);
pub const ThreadTimes: THREADINFOCLASS = THREADINFOCLASS(1i32);
pub const ThreadUmsInformation: THREADINFOCLASS = THREADINFOCLASS(31i32);
pub const ThreadWow64Context: THREADINFOCLASS = THREADINFOCLASS(29i32);
pub const ThreadZeroTlsCell: THREADINFOCLASS = THREADINFOCLASS(10i32);
pub const TimerSetCoalescableTimer: TIMER_SET_INFORMATION_CLASS = TIMER_SET_INFORMATION_CLASS(0i32);
