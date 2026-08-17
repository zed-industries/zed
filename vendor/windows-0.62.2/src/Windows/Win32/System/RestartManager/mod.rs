#[inline]
pub unsafe fn RmAddFilter<P1, P3>(dwsessionhandle: u32, strmodulename: P1, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P3, filteraction: RM_FILTER_ACTION) -> super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rstrtmgr.dll" "system" fn RmAddFilter(dwsessionhandle : u32, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR, filteraction : RM_FILTER_ACTION) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmAddFilter(dwsessionhandle, strmodulename.param().abi(), pprocess.unwrap_or(core::mem::zeroed()) as _, strserviceshortname.param().abi(), filteraction) }
}
#[inline]
pub unsafe fn RmCancelCurrentTask(dwsessionhandle: u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmCancelCurrentTask(dwsessionhandle : u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmCancelCurrentTask(dwsessionhandle) }
}
#[inline]
pub unsafe fn RmEndSession(dwsessionhandle: u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmEndSession(dwsessionhandle : u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmEndSession(dwsessionhandle) }
}
#[inline]
pub unsafe fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: Option<&mut [u8]>, cbfilterbufneeded: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmGetFilterList(dwsessionhandle : u32, pbfilterbuf : *mut u8, cbfilterbuf : u32, cbfilterbufneeded : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmGetFilterList(dwsessionhandle, core::mem::transmute(pbfilterbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbfilterbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), cbfilterbufneeded as _) }
}
#[inline]
pub unsafe fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: Option<*mut RM_PROCESS_INFO>, lpdwrebootreasons: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmGetList(dwsessionhandle : u32, pnprocinfoneeded : *mut u32, pnprocinfo : *mut u32, rgaffectedapps : *mut RM_PROCESS_INFO, lpdwrebootreasons : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmGetList(dwsessionhandle, pnprocinfoneeded as _, pnprocinfo as _, rgaffectedapps.unwrap_or(core::mem::zeroed()) as _, lpdwrebootreasons as _) }
}
#[inline]
pub unsafe fn RmJoinSession<P1>(psessionhandle: *mut u32, strsessionkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rstrtmgr.dll" "system" fn RmJoinSession(psessionhandle : *mut u32, strsessionkey : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmJoinSession(psessionhandle as _, strsessionkey.param().abi()) }
}
#[inline]
pub unsafe fn RmRegisterResources(dwsessionhandle: u32, rgsfilenames: Option<&[windows_core::PCWSTR]>, rgapplications: Option<&[RM_UNIQUE_PROCESS]>, rgsservicenames: Option<&[windows_core::PCWSTR]>) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmRegisterResources(dwsessionhandle : u32, nfiles : u32, rgsfilenames : *const windows_core::PCWSTR, napplications : u32, rgapplications : *const RM_UNIQUE_PROCESS, nservices : u32, rgsservicenames : *const windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe {
        RmRegisterResources(
            dwsessionhandle,
            rgsfilenames.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(rgsfilenames.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rgapplications.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(rgapplications.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            rgsservicenames.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(rgsservicenames.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        )
    }
}
#[inline]
pub unsafe fn RmRemoveFilter<P1, P3>(dwsessionhandle: u32, strmodulename: P1, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P3) -> super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rstrtmgr.dll" "system" fn RmRemoveFilter(dwsessionhandle : u32, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmRemoveFilter(dwsessionhandle, strmodulename.param().abi(), pprocess.unwrap_or(core::mem::zeroed()) as _, strserviceshortname.param().abi()) }
}
#[inline]
pub unsafe fn RmRestart(dwsessionhandle: u32, dwrestartflags: Option<u32>, fnstatus: RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmRestart(dwsessionhandle : u32, dwrestartflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmRestart(dwsessionhandle, dwrestartflags.unwrap_or(core::mem::zeroed()) as _, fnstatus) }
}
#[inline]
pub unsafe fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmShutdown(dwsessionhandle : u32, lactionflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmShutdown(dwsessionhandle, lactionflags, fnstatus) }
}
#[inline]
pub unsafe fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: Option<u32>, strsessionkey: windows_core::PWSTR) -> super::super::Foundation::WIN32_ERROR {
    windows_core::link!("rstrtmgr.dll" "system" fn RmStartSession(psessionhandle : *mut u32, dwsessionflags : u32, strsessionkey : windows_core::PWSTR) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { RmStartSession(psessionhandle as _, dwsessionflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(strsessionkey)) }
}
pub const CCH_RM_MAX_APP_NAME: u32 = 255u32;
pub const CCH_RM_MAX_SVC_NAME: u32 = 63u32;
pub const CCH_RM_SESSION_KEY: u32 = 32u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_APP_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_APP_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_FILTER_ACTION(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RM_FILTER_INFO {
    pub FilterAction: RM_FILTER_ACTION,
    pub FilterTrigger: RM_FILTER_TRIGGER,
    pub cbNextOffset: u32,
    pub Anonymous: RM_FILTER_INFO_0,
}
impl Default for RM_FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RM_FILTER_INFO_0 {
    pub strFilename: windows_core::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: windows_core::PWSTR,
}
impl Default for RM_FILTER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_FILTER_TRIGGER(pub i32);
pub const RM_INVALID_PROCESS: i32 = -1i32;
pub const RM_INVALID_TS_SESSION: i32 = -1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [u16; 256],
    pub strServiceShortName: [u16; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: u32,
    pub TSSessionId: u32,
    pub bRestartable: windows_core::BOOL,
}
impl Default for RM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_REBOOT_REASON(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_SHUTDOWN_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RM_UNIQUE_PROCESS {
    pub dwProcessId: u32,
    pub ProcessStartTime: super::super::Foundation::FILETIME,
}
pub type RM_WRITE_STATUS_CALLBACK = Option<unsafe extern "system" fn(npercentcomplete: u32)>;
pub const RmConsole: RM_APP_TYPE = RM_APP_TYPE(5i32);
pub const RmCritical: RM_APP_TYPE = RM_APP_TYPE(1000i32);
pub const RmExplorer: RM_APP_TYPE = RM_APP_TYPE(4i32);
pub const RmFilterTriggerFile: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(1i32);
pub const RmFilterTriggerInvalid: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(0i32);
pub const RmFilterTriggerProcess: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(2i32);
pub const RmFilterTriggerService: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(3i32);
pub const RmForceShutdown: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(1i32);
pub const RmInvalidFilterAction: RM_FILTER_ACTION = RM_FILTER_ACTION(0i32);
pub const RmMainWindow: RM_APP_TYPE = RM_APP_TYPE(1i32);
pub const RmNoRestart: RM_FILTER_ACTION = RM_FILTER_ACTION(1i32);
pub const RmNoShutdown: RM_FILTER_ACTION = RM_FILTER_ACTION(2i32);
pub const RmOtherWindow: RM_APP_TYPE = RM_APP_TYPE(2i32);
pub const RmRebootReasonCriticalProcess: RM_REBOOT_REASON = RM_REBOOT_REASON(4i32);
pub const RmRebootReasonCriticalService: RM_REBOOT_REASON = RM_REBOOT_REASON(8i32);
pub const RmRebootReasonDetectedSelf: RM_REBOOT_REASON = RM_REBOOT_REASON(16i32);
pub const RmRebootReasonNone: RM_REBOOT_REASON = RM_REBOOT_REASON(0i32);
pub const RmRebootReasonPermissionDenied: RM_REBOOT_REASON = RM_REBOOT_REASON(1i32);
pub const RmRebootReasonSessionMismatch: RM_REBOOT_REASON = RM_REBOOT_REASON(2i32);
pub const RmService: RM_APP_TYPE = RM_APP_TYPE(3i32);
pub const RmShutdownOnlyRegistered: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(16i32);
pub const RmStatusErrorOnRestart: RM_APP_STATUS = RM_APP_STATUS(32i32);
pub const RmStatusErrorOnStop: RM_APP_STATUS = RM_APP_STATUS(16i32);
pub const RmStatusRestartMasked: RM_APP_STATUS = RM_APP_STATUS(128i32);
pub const RmStatusRestarted: RM_APP_STATUS = RM_APP_STATUS(8i32);
pub const RmStatusRunning: RM_APP_STATUS = RM_APP_STATUS(1i32);
pub const RmStatusShutdownMasked: RM_APP_STATUS = RM_APP_STATUS(64i32);
pub const RmStatusStopped: RM_APP_STATUS = RM_APP_STATUS(2i32);
pub const RmStatusStoppedOther: RM_APP_STATUS = RM_APP_STATUS(4i32);
pub const RmStatusUnknown: RM_APP_STATUS = RM_APP_STATUS(0i32);
pub const RmUnknownApp: RM_APP_TYPE = RM_APP_TYPE(0i32);
