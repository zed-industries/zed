windows_link::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : PTP_POOL));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : PTP_CLEANUP_GROUP));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : PTP_CLEANUP_GROUP, fcancelpendingcallbacks : BOOL, pvcleanupcontext : *mut core::ffi::c_void));
windows_link::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *const core::ffi::c_void) -> PTP_POOL);
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> PTP_CLEANUP_GROUP);
windows_link::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : PTP_POOL, cthrdmost : u32));
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : PTP_POOL, cthrdmic : u32) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
windows_link::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> BOOL);
pub type BOOL = i32;
pub type PTP_CALLBACK_INSTANCE = isize;
pub type PTP_CLEANUP_GROUP = isize;
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = Option<
    unsafe extern "system" fn(
        objectcontext: *mut core::ffi::c_void,
        cleanupcontext: *mut core::ffi::c_void,
    ),
>;
pub type PTP_POOL = isize;
pub type PTP_SIMPLE_CALLBACK = Option<
    unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void),
>;
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
impl Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TP_CALLBACK_ENVIRON_V3_0 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_0_0,
}
impl Default for TP_CALLBACK_ENVIRON_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_CALLBACK_ENVIRON_V3_0_0 {
    pub _bitfield: u32,
}
pub type TP_CALLBACK_PRIORITY = i32;
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1i32;
