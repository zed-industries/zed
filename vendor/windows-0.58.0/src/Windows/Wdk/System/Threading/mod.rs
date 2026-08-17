#[inline]
pub unsafe fn NtCancelTimer<P0>(timerhandle: P0, currentstate: Option<*mut super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtCancelTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, currentstate : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCancelTimer(timerhandle.param().abi(), core::mem::transmute(currentstate.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NtCreateTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, timertype: super::super::super::Win32::System::Kernel::TIMER_TYPE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtCreateTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, timertype : super::super::super::Win32::System::Kernel:: TIMER_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtCreateTimer(timerhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), timertype)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenEvent(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenEvent(eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenEvent(eventhandle, desiredaccess, objectattributes)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn NtOpenProcess(processhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid: Option<*const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenProcess(processhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, clientid : *const super::super::super::Win32::System::WindowsProgramming:: CLIENT_ID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenProcess(processhandle, desiredaccess, objectattributes, core::mem::transmute(clientid.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NtOpenTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtOpenTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtOpenTimer(timerhandle, desiredaccess, objectattributes)
}
#[inline]
pub unsafe fn NtQueryInformationProcess<P0>(processhandle: P0, processinformationclass: PROCESSINFOCLASS, processinformation: *mut core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryInformationProcess(processhandle.param().abi(), processinformationclass, processinformation, processinformationlength, returnlength)
}
#[inline]
pub unsafe fn NtQueryInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *mut core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryInformationThread(threadhandle.param().abi(), threadinformationclass, threadinformation, threadinformationlength, returnlength)
}
#[inline]
pub unsafe fn NtSetInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *const core::ffi::c_void, threadinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetInformationThread(threadhandle.param().abi(), threadinformationclass, threadinformation, threadinformationlength)
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn NtSetTimer<P0, P1>(timerhandle: P0, duetime: *const i64, timerapcroutine: super::SystemServices::PTIMER_APC_ROUTINE, timercontext: Option<*const core::ffi::c_void>, resumetimer: P1, period: i32, previousstate: Option<*mut super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, duetime : *const i64, timerapcroutine : super::SystemServices:: PTIMER_APC_ROUTINE, timercontext : *const core::ffi::c_void, resumetimer : super::super::super::Win32::Foundation:: BOOLEAN, period : i32, previousstate : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetTimer(timerhandle.param().abi(), duetime, timerapcroutine, core::mem::transmute(timercontext.unwrap_or(std::ptr::null())), resumetimer.param().abi(), period, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NtSetTimerEx<P0>(timerhandle: P0, timersetinformationclass: TIMER_SET_INFORMATION_CLASS, timersetinformation: Option<*mut core::ffi::c_void>, timersetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtSetTimerEx(timerhandle : super::super::super::Win32::Foundation:: HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : *mut core::ffi::c_void, timersetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetTimerEx(timerhandle.param().abi(), timersetinformationclass, core::mem::transmute(timersetinformation.unwrap_or(std::ptr::null_mut())), timersetinformationlength)
}
#[inline]
pub unsafe fn NtTerminateProcess<P0, P1>(processhandle: P0, exitstatus: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtTerminateProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, exitstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtTerminateProcess(processhandle.param().abi(), exitstatus.param().abi())
}
#[inline]
pub unsafe fn NtWaitForSingleObject<P0, P1>(handle: P0, alertable: P1, timeout: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : super::super::super::Win32::Foundation:: HANDLE, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtWaitForSingleObject(handle.param().abi(), alertable.param().abi(), timeout)
}
#[inline]
pub unsafe fn ZwCancelTimer<P0>(timerhandle: P0, currentstate: Option<*mut super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwCancelTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, currentstate : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCancelTimer(timerhandle.param().abi(), core::mem::transmute(currentstate.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ZwCreateTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, timertype: super::super::super::Win32::System::Kernel::TIMER_TYPE) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwCreateTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, timertype : super::super::super::Win32::System::Kernel:: TIMER_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwCreateTimer(timerhandle, desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), timertype)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenEvent(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenEvent(eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenEvent(eventhandle, desiredaccess, objectattributes)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn ZwOpenProcess(processhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid: Option<*const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID>) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenProcess(processhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, clientid : *const super::super::super::Win32::System::WindowsProgramming:: CLIENT_ID) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenProcess(processhandle, desiredaccess, objectattributes, core::mem::transmute(clientid.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn ZwOpenTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwOpenTimer(timerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwOpenTimer(timerhandle, desiredaccess, objectattributes)
}
#[inline]
pub unsafe fn ZwQueryInformationProcess<P0>(processhandle: P0, processinformationclass: PROCESSINFOCLASS, processinformation: *mut core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryInformationProcess(processhandle.param().abi(), processinformationclass, processinformation, processinformationlength, returnlength)
}
#[inline]
pub unsafe fn ZwQueryInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *mut core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwQueryInformationThread(threadhandle.param().abi(), threadinformationclass, threadinformation, threadinformationlength, returnlength)
}
#[inline]
pub unsafe fn ZwSetInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *const core::ffi::c_void, threadinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetInformationThread(threadhandle.param().abi(), threadinformationclass, threadinformation, threadinformationlength)
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn ZwSetTimer<P0, P1>(timerhandle: P0, duetime: *const i64, timerapcroutine: super::SystemServices::PTIMER_APC_ROUTINE, timercontext: Option<*const core::ffi::c_void>, resumetimer: P1, period: i32, previousstate: Option<*mut super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetTimer(timerhandle : super::super::super::Win32::Foundation:: HANDLE, duetime : *const i64, timerapcroutine : super::SystemServices:: PTIMER_APC_ROUTINE, timercontext : *const core::ffi::c_void, resumetimer : super::super::super::Win32::Foundation:: BOOLEAN, period : i32, previousstate : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetTimer(timerhandle.param().abi(), duetime, timerapcroutine, core::mem::transmute(timercontext.unwrap_or(std::ptr::null())), resumetimer.param().abi(), period, core::mem::transmute(previousstate.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ZwSetTimerEx<P0>(timerhandle: P0, timersetinformationclass: TIMER_SET_INFORMATION_CLASS, timersetinformation: Option<*mut core::ffi::c_void>, timersetinformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwSetTimerEx(timerhandle : super::super::super::Win32::Foundation:: HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : *mut core::ffi::c_void, timersetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetTimerEx(timerhandle.param().abi(), timersetinformationclass, core::mem::transmute(timersetinformation.unwrap_or(std::ptr::null_mut())), timersetinformationlength)
}
#[inline]
pub unsafe fn ZwTerminateProcess<P0, P1>(processhandle: P0, exitstatus: P1) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwTerminateProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, exitstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwTerminateProcess(processhandle.param().abi(), exitstatus.param().abi())
}
#[inline]
pub unsafe fn ZwWaitForSingleObject<P0, P1>(handle: P0, alertable: P1, timeout: Option<*const i64>) -> super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Win32::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("ntdll.dll" "system" fn ZwWaitForSingleObject(handle : super::super::super::Win32::Foundation:: HANDLE, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwWaitForSingleObject(handle.param().abi(), alertable.param().abi(), core::mem::transmute(timeout.unwrap_or(std::ptr::null())))
}
pub const MaxProcessInfoClass: PROCESSINFOCLASS = PROCESSINFOCLASS(83i32);
pub const MaxThreadInfoClass: THREADINFOCLASS = THREADINFOCLASS(56i32);
pub const MaxTimerInfoClass: TIMER_SET_INFORMATION_CLASS = TIMER_SET_INFORMATION_CLASS(1i32);
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROCESSINFOCLASS(pub i32);
impl windows_core::TypeKind for PROCESSINFOCLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROCESSINFOCLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROCESSINFOCLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct THREADINFOCLASS(pub i32);
impl windows_core::TypeKind for THREADINFOCLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for THREADINFOCLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("THREADINFOCLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TIMER_SET_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for TIMER_SET_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TIMER_SET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TIMER_SET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
