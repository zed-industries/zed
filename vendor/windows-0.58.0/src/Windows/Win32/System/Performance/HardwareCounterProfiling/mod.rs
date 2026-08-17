#[inline]
pub unsafe fn DisableThreadProfiling<P0>(performancedatahandle: P0) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DisableThreadProfiling(performancedatahandle : super::super::super::Foundation:: HANDLE) -> u32);
    DisableThreadProfiling(performancedatahandle.param().abi())
}
#[inline]
pub unsafe fn EnableThreadProfiling<P0>(threadhandle: P0, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn EnableThreadProfiling(threadhandle : super::super::super::Foundation:: HANDLE, flags : u32, hardwarecounters : u64, performancedatahandle : *mut super::super::super::Foundation:: HANDLE) -> u32);
    EnableThreadProfiling(threadhandle.param().abi(), flags, hardwarecounters, performancedatahandle)
}
#[inline]
pub unsafe fn QueryThreadProfiling<P0>(threadhandle: P0, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryThreadProfiling(threadhandle : super::super::super::Foundation:: HANDLE, enabled : *mut super::super::super::Foundation:: BOOLEAN) -> u32);
    QueryThreadProfiling(threadhandle.param().abi(), enabled)
}
#[inline]
pub unsafe fn ReadThreadProfilingData<P0>(performancedatahandle: P0, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReadThreadProfilingData(performancedatahandle : super::super::super::Foundation:: HANDLE, flags : u32, performancedata : *mut PERFORMANCE_DATA) -> u32);
    ReadThreadProfilingData(performancedatahandle.param().abi(), flags, performancedata)
}
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(1i32);
pub const PMCCounter: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HARDWARE_COUNTER_TYPE(pub i32);
impl windows_core::TypeKind for HARDWARE_COUNTER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HARDWARE_COUNTER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HARDWARE_COUNTER_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HARDWARE_COUNTER_DATA {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl windows_core::TypeKind for HARDWARE_COUNTER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for HARDWARE_COUNTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PERFORMANCE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
