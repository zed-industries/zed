#[inline]
pub unsafe fn DisableThreadProfiling(performancedatahandle: super::super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn DisableThreadProfiling(performancedatahandle : super::super::super::Foundation:: HANDLE) -> u32);
    unsafe { DisableThreadProfiling(performancedatahandle) }
}
#[inline]
pub unsafe fn EnableThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn EnableThreadProfiling(threadhandle : super::super::super::Foundation:: HANDLE, flags : u32, hardwarecounters : u64, performancedatahandle : *mut super::super::super::Foundation:: HANDLE) -> u32);
    unsafe { EnableThreadProfiling(threadhandle, flags, hardwarecounters, performancedatahandle as _) }
}
#[inline]
pub unsafe fn QueryThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, enabled: *mut bool) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn QueryThreadProfiling(threadhandle : super::super::super::Foundation:: HANDLE, enabled : *mut bool) -> u32);
    unsafe { QueryThreadProfiling(threadhandle, enabled as _) }
}
#[inline]
pub unsafe fn ReadThreadProfilingData(performancedatahandle: super::super::super::Foundation::HANDLE, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn ReadThreadProfilingData(performancedatahandle : super::super::super::Foundation:: HANDLE, flags : u32, performancedata : *mut PERFORMANCE_DATA) -> u32);
    unsafe { ReadThreadProfilingData(performancedatahandle, flags, performancedata as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HARDWARE_COUNTER_DATA {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HARDWARE_COUNTER_TYPE(pub i32);
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERFORMANCE_DATA {
    pub Size: u16,
    pub Version: u8,
    pub HwCountersCount: u8,
    pub ContextSwitchCount: u32,
    pub WaitReasonBitMap: u64,
    pub CycleTime: u64,
    pub RetryCount: u32,
    pub Reserved: u32,
    pub HwCounters: [HARDWARE_COUNTER_DATA; 16],
}
impl Default for PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PMCCounter: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(0i32);
