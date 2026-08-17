#[inline]
pub unsafe fn RmAddFilter<P0, P1>(dwsessionhandle: u32, strmodulename: P0, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P1, filteraction: RM_FILTER_ACTION) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rstrtmgr.dll" "system" fn RmAddFilter(dwsessionhandle : u32, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR, filteraction : RM_FILTER_ACTION) -> super::super::Foundation:: WIN32_ERROR);
    RmAddFilter(dwsessionhandle, strmodulename.param().abi(), core::mem::transmute(pprocess.unwrap_or(std::ptr::null())), strserviceshortname.param().abi(), filteraction)
}
#[inline]
pub unsafe fn RmCancelCurrentTask(dwsessionhandle: u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmCancelCurrentTask(dwsessionhandle : u32) -> super::super::Foundation:: WIN32_ERROR);
    RmCancelCurrentTask(dwsessionhandle)
}
#[inline]
pub unsafe fn RmEndSession(dwsessionhandle: u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmEndSession(dwsessionhandle : u32) -> super::super::Foundation:: WIN32_ERROR);
    RmEndSession(dwsessionhandle)
}
#[inline]
pub unsafe fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: Option<&mut [u8]>, cbfilterbufneeded: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmGetFilterList(dwsessionhandle : u32, pbfilterbuf : *mut u8, cbfilterbuf : u32, cbfilterbufneeded : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RmGetFilterList(dwsessionhandle, core::mem::transmute(pbfilterbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbfilterbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), cbfilterbufneeded)
}
#[inline]
pub unsafe fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: Option<*mut RM_PROCESS_INFO>, lpdwrebootreasons: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmGetList(dwsessionhandle : u32, pnprocinfoneeded : *mut u32, pnprocinfo : *mut u32, rgaffectedapps : *mut RM_PROCESS_INFO, lpdwrebootreasons : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RmGetList(dwsessionhandle, pnprocinfoneeded, pnprocinfo, core::mem::transmute(rgaffectedapps.unwrap_or(std::ptr::null_mut())), lpdwrebootreasons)
}
#[inline]
pub unsafe fn RmJoinSession<P0>(psessionhandle: *mut u32, strsessionkey: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rstrtmgr.dll" "system" fn RmJoinSession(psessionhandle : *mut u32, strsessionkey : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RmJoinSession(psessionhandle, strsessionkey.param().abi())
}
#[inline]
pub unsafe fn RmRegisterResources(dwsessionhandle: u32, rgsfilenames: Option<&[windows_core::PCWSTR]>, rgapplications: Option<&[RM_UNIQUE_PROCESS]>, rgsservicenames: Option<&[windows_core::PCWSTR]>) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmRegisterResources(dwsessionhandle : u32, nfiles : u32, rgsfilenames : *const windows_core::PCWSTR, napplications : u32, rgapplications : *const RM_UNIQUE_PROCESS, nservices : u32, rgsservicenames : *const windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
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
#[inline]
pub unsafe fn RmRemoveFilter<P0, P1>(dwsessionhandle: u32, strmodulename: P0, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rstrtmgr.dll" "system" fn RmRemoveFilter(dwsessionhandle : u32, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RmRemoveFilter(dwsessionhandle, strmodulename.param().abi(), core::mem::transmute(pprocess.unwrap_or(std::ptr::null())), strserviceshortname.param().abi())
}
#[inline]
pub unsafe fn RmRestart(dwsessionhandle: u32, dwrestartflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmRestart(dwsessionhandle : u32, dwrestartflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation:: WIN32_ERROR);
    RmRestart(dwsessionhandle, dwrestartflags, fnstatus)
}
#[inline]
pub unsafe fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmShutdown(dwsessionhandle : u32, lactionflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation:: WIN32_ERROR);
    RmShutdown(dwsessionhandle, lactionflags, fnstatus)
}
#[inline]
pub unsafe fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: u32, strsessionkey: windows_core::PWSTR) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("rstrtmgr.dll" "system" fn RmStartSession(psessionhandle : *mut u32, dwsessionflags : u32, strsessionkey : windows_core::PWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RmStartSession(psessionhandle, dwsessionflags, core::mem::transmute(strsessionkey))
}
pub const CCH_RM_MAX_APP_NAME: u32 = 255u32;
pub const CCH_RM_MAX_SVC_NAME: u32 = 63u32;
pub const CCH_RM_SESSION_KEY: u32 = 32u32;
pub const RM_INVALID_PROCESS: i32 = -1i32;
pub const RM_INVALID_TS_SESSION: i32 = -1i32;
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RM_APP_STATUS(pub i32);
impl windows_core::TypeKind for RM_APP_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RM_APP_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RM_APP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RM_APP_TYPE(pub i32);
impl windows_core::TypeKind for RM_APP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RM_APP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RM_APP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RM_FILTER_ACTION(pub i32);
impl windows_core::TypeKind for RM_FILTER_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RM_FILTER_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RM_FILTER_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RM_FILTER_TRIGGER(pub i32);
impl windows_core::TypeKind for RM_FILTER_TRIGGER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RM_FILTER_TRIGGER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RM_FILTER_TRIGGER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RM_REBOOT_REASON(pub i32);
impl windows_core::TypeKind for RM_REBOOT_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RM_REBOOT_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RM_REBOOT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RM_SHUTDOWN_TYPE(pub i32);
impl windows_core::TypeKind for RM_SHUTDOWN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RM_SHUTDOWN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RM_SHUTDOWN_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RM_FILTER_INFO {
    pub FilterAction: RM_FILTER_ACTION,
    pub FilterTrigger: RM_FILTER_TRIGGER,
    pub cbNextOffset: u32,
    pub Anonymous: RM_FILTER_INFO_0,
}
impl windows_core::TypeKind for RM_FILTER_INFO {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for RM_FILTER_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RM_FILTER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [u16; 256],
    pub strServiceShortName: [u16; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: u32,
    pub TSSessionId: u32,
    pub bRestartable: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for RM_PROCESS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RM_UNIQUE_PROCESS {
    pub dwProcessId: u32,
    pub ProcessStartTime: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for RM_UNIQUE_PROCESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RM_UNIQUE_PROCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RM_WRITE_STATUS_CALLBACK = Option<unsafe extern "system" fn(npercentcomplete: u32)>;
