// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! RestartManager include file
use shared::minwindef::{BOOL, DWORD, FILETIME, LPDWORD, PBYTE, UINT, ULONG};
use um::winnt::{LPCWSTR, LPWSTR, WCHAR};
pub const RM_SESSION_KEY_LEN: usize = 16; // mem::size_of::<GUID>()
pub const CCH_RM_SESSION_KEY: usize = RM_SESSION_KEY_LEN * 2;
pub const CCH_RM_MAX_APP_NAME: usize = 255;
pub const CCH_RM_MAX_SVC_NAME: usize = 63;
pub const RM_INVALID_TS_SESSION: DWORD = -1i32 as u32;
pub const RM_INVALID_PROCESS: DWORD = -1i32 as u32;
ENUM!{enum RM_APP_TYPE {
    RmUnknownApp = 0,
    RmMainWindow = 1,
    RmOtherWindow = 2,
    RmService = 3,
    RmExplorer = 4,
    RmConsole = 5,
    RmCritical = 1000,
}}
ENUM!{enum RM_SHUTDOWN_TYPE {
    RmForceShutdown = 0x1,
    RmShutdownOnlyRegistered = 0x10,
}}
ENUM!{enum RM_APP_STATUS {
    RmStatusUnknown = 0x0,
    RmStatusRunning = 0x1,
    RmStatusStopped = 0x2,
    RmStatusStoppedOther = 0x4,
    RmStatusRestarted = 0x8,
    RmStatusErrorOnStop = 0x10,
    RmStatusErrorOnRestart = 0x20,
    RmStatusShutdownMasked = 0x40,
    RmStatusRestartMasked = 0x80,
}}
ENUM!{enum RM_REBOOT_REASON {
    RmRebootReasonNone = 0x0,
    RmRebootReasonPermissionDenied = 0x1,
    RmRebootReasonSessionMismatch = 0x2,
    RmRebootReasonCriticalProcess = 0x4,
    RmRebootReasonCriticalService = 0x8,
    RmRebootReasonDetectedSelf = 0x10,
}}
STRUCT!{struct RM_UNIQUE_PROCESS {
    dwProcessId: DWORD,
    ProcessStartTime: FILETIME,
}}
pub type PRM_UNIQUE_PROCESS = *mut RM_UNIQUE_PROCESS;
STRUCT!{struct RM_PROCESS_INFO {
    Process: RM_UNIQUE_PROCESS,
    strAppName: [WCHAR; CCH_RM_MAX_APP_NAME + 1],
    strServiceShortName: [WCHAR; CCH_RM_MAX_SVC_NAME + 1],
    ApplicationType: RM_APP_TYPE,
    AppStatus: ULONG,
    TSSessionId: DWORD,
    bRestartable: BOOL,
}}
pub type PRM_PROCESS_INFO = *mut RM_PROCESS_INFO;
ENUM!{enum RM_FILTER_TRIGGER {
    RmFilterTriggerInvalid = 0,
    RmFilterTriggerFile,
    RmFilterTriggerProcess,
    RmFilterTriggerService,
}}
ENUM!{enum RM_FILTER_ACTION {
    RmInvalidFilterAction = 0,
    RmNoRestart = 1,
    RmNoShutdown = 2,
}}
UNION!{union RM_FILTER_INFO_u {
    [u32; 3] [u64; 2],
    strFilename strFilename_mut: LPWSTR,
    Process Process_mut: RM_UNIQUE_PROCESS,
    strServiceShortName strServiceShortName_mut: LPWSTR,
}}
STRUCT!{struct RM_FILTER_INFO {
    FilterAction: RM_FILTER_ACTION,
    FilterTrigger: RM_FILTER_TRIGGER,
    cbNextOffset: DWORD,
    u: RM_FILTER_INFO_u,
}}
pub type PRM_FILTER_INFO = *mut RM_FILTER_INFO;
FN!{cdecl RM_WRITE_STATUS_CALLBACK(
    nPercentComplete: u32,
) -> ()}
extern "system" {
    pub fn RmStartSession(
        pSessionHandle: *mut DWORD,
        dwSessionFlags: DWORD,
        strSessionKey: *mut WCHAR,
    ) -> DWORD;
    pub fn RmJoinSession(
        pSessionHandle: *mut DWORD,
        strSessionKey: *const WCHAR,
    ) -> DWORD;
    pub fn RmEndSession(
        dwSessionHandle: DWORD,
    ) -> DWORD;
    pub fn RmRegisterResources(
        dwSessionHandle: DWORD,
        nFiles: UINT,
        rgsFileNames: *mut LPCWSTR,
        nApplications: UINT,
        rgApplications: *mut RM_UNIQUE_PROCESS,
        nServices: UINT,
        rgsServiceNames: *mut LPCWSTR,
    ) -> DWORD;
    pub fn RmGetList(
        dwSessionHandle: DWORD,
        pnProcInfoNeeded: *mut UINT,
        pnProcInfo: *mut UINT,
        rgAffectedApps: *mut RM_PROCESS_INFO,
        lpdwRebootReasons: LPDWORD,
    ) -> DWORD;
    pub fn RmShutdown(
        dwSessionHandle: DWORD,
        lActionFlags: ULONG,
        fnStatus: RM_WRITE_STATUS_CALLBACK,
    ) -> DWORD;
    pub fn RmRestart(
        dwSessionHandle: DWORD,
        dwRestartFlags: DWORD,
        fnStatus: RM_WRITE_STATUS_CALLBACK,
    ) -> DWORD;
    pub fn RmCancelCurrentTask(
        dwSessionHandle: DWORD,
    ) -> DWORD;
    pub fn RmAddFilter(
        dwSessionHandle: DWORD,
        strModuleName: LPCWSTR,
        pProcess: *mut RM_UNIQUE_PROCESS,
        strServiceShortName: LPCWSTR,
        FilterAction: RM_FILTER_ACTION,
    ) -> DWORD;
    pub fn RmRemoveFilter(
        dwSessionHandle: DWORD,
        strModuleName: LPCWSTR,
        pProcess: *mut RM_UNIQUE_PROCESS,
        strServiceShortName: LPCWSTR,
    ) -> DWORD;
    pub fn RmGetFilterList(
        dwSessionHandle: DWORD,
        pbFilterBuf: PBYTE,
        cbFilterBuf: DWORD,
        cbFilterBufNeeded: LPDWORD,
    ) -> DWORD;
}
