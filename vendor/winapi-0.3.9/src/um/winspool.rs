// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Winspool header file
use shared::guiddef::GUID;
use shared::minwindef::{
    BOOL, BYTE, DWORD, FILETIME, FLOAT, LPBYTE, LPDWORD, LPHANDLE, LPVOID, MAX_PATH, PBYTE, PDWORD,
    PULONG, PWORD, UINT, ULONG, WORD,
};
use shared::windef::{HWND, RECTL, SIZEL};
use shared::winerror::ERROR_NOT_SUPPORTED;
use um::minwinbase::SYSTEMTIME;
use um::wingdi::{LPDEVMODEA, LPDEVMODEW, PDEVMODEA, PDEVMODEW};
use um::winnt::{
    ACCESS_MASK, CHAR, DWORDLONG, HANDLE, HRESULT, LANGID, LONG, LONGLONG, LPCSTR, LPCWSTR, LPSTR,
    LPWSTR, PCWSTR, PSECURITY_DESCRIPTOR, PVOID, PWSTR, STANDARD_RIGHTS_EXECUTE,
    STANDARD_RIGHTS_READ, STANDARD_RIGHTS_REQUIRED, STANDARD_RIGHTS_WRITE, WCHAR,
};
use vc::vcruntime::size_t;
STRUCT!{struct PRINTER_INFO_1A {
    Flags: DWORD,
    pDescription: LPSTR,
    pName: LPSTR,
    pComment: LPSTR,
}}
pub type PPRINTER_INFO_1A = *mut PRINTER_INFO_1A;
pub type LPPRINTER_INFO_1A = *mut PRINTER_INFO_1A;
STRUCT!{struct PRINTER_INFO_1W {
    Flags: DWORD,
    pDescription: LPWSTR,
    pName: LPWSTR,
    pComment: LPWSTR,
}}
pub type PPRINTER_INFO_1W = *mut PRINTER_INFO_1W;
pub type LPPRINTER_INFO_1W = *mut PRINTER_INFO_1W;
STRUCT!{struct PRINTER_INFO_2A {
    pServerName: LPSTR,
    pPrinterName: LPSTR,
    pShareName: LPSTR,
    pPortName: LPSTR,
    pDriverName: LPSTR,
    pComment: LPSTR,
    pLocation: LPSTR,
    pDevMode: LPDEVMODEA,
    pSepFile: LPSTR,
    pPrintProcessor: LPSTR,
    pDatatype: LPSTR,
    pParameters: LPSTR,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    Attributes: DWORD,
    Priority: DWORD,
    DefaultPriority: DWORD,
    StartTime: DWORD,
    UntilTime: DWORD,
    Status: DWORD,
    cJobs: DWORD,
    AveragePPM: DWORD,
}}
pub type PPRINTER_INFO_2A = *mut PRINTER_INFO_2A;
pub type LPPRINTER_INFO_2A = *mut PRINTER_INFO_2A;
STRUCT!{struct PRINTER_INFO_2W {
    pServerName: LPWSTR,
    pPrinterName: LPWSTR,
    pShareName: LPWSTR,
    pPortName: LPWSTR,
    pDriverName: LPWSTR,
    pComment: LPWSTR,
    pLocation: LPWSTR,
    pDevMode: LPDEVMODEW,
    pSepFile: LPWSTR,
    pPrintProcessor: LPWSTR,
    pDatatype: LPWSTR,
    pParameters: LPWSTR,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    Attributes: DWORD,
    Priority: DWORD,
    DefaultPriority: DWORD,
    StartTime: DWORD,
    UntilTime: DWORD,
    Status: DWORD,
    cJobs: DWORD,
    AveragePPM: DWORD,
}}
pub type PPRINTER_INFO_2W = *mut PRINTER_INFO_2W;
pub type LPPRINTER_INFO_2W = *mut PRINTER_INFO_2W;
STRUCT!{struct PRINTER_INFO_3 {
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
}}
pub type PPRINTER_INFO_3 = *mut PRINTER_INFO_3;
pub type LPPRINTER_INFO_3 = *mut PRINTER_INFO_3;
STRUCT!{struct PRINTER_INFO_4A {
    pPrinterName: LPSTR,
    pServerName: LPSTR,
    Attributes: DWORD,
}}
pub type PPRINTER_INFO_4A = *mut PRINTER_INFO_4A;
pub type LPPRINTER_INFO_4A = *mut PRINTER_INFO_4A;
STRUCT!{struct PRINTER_INFO_4W {
    pPrinterName: LPWSTR,
    pServerName: LPWSTR,
    Attributes: DWORD,
}}
pub type PPRINTER_INFO_4W = *mut PRINTER_INFO_4W;
pub type LPPRINTER_INFO_4W = *mut PRINTER_INFO_4W;
STRUCT!{struct PRINTER_INFO_5A {
    pPrinterName: LPSTR,
    pPortName: LPSTR,
    Attributes: DWORD,
    DeviceNotSelectedTimeout: DWORD,
    TransmissionRetryTimeout: DWORD,
}}
pub type PPRINTER_INFO_5A = *mut PRINTER_INFO_5A;
pub type LPPRINTER_INFO_5A = *mut PRINTER_INFO_5A;
STRUCT!{struct PRINTER_INFO_5W {
    pPrinterName: LPWSTR,
    pPortName: LPWSTR,
    Attributes: DWORD,
    DeviceNotSelectedTimeout: DWORD,
    TransmissionRetryTimeout: DWORD,
}}
pub type PPRINTER_INFO_5W = *mut PRINTER_INFO_5W;
pub type LPPRINTER_INFO_5W = *mut PRINTER_INFO_5W;
STRUCT!{struct PRINTER_INFO_6 {
    dwStatus: DWORD,
}}
pub type PPRINTER_INFO_6 = *mut PRINTER_INFO_6;
pub type LPPRINTER_INFO_6 = *mut PRINTER_INFO_6;
STRUCT!{struct PRINTER_INFO_7A {
    pszObjectGUID: LPSTR,
    dwAction: DWORD,
}}
pub type PPRINTER_INFO_7A = *mut PRINTER_INFO_7A;
pub type LPPRINTER_INFO_7A = *mut PRINTER_INFO_7A;
STRUCT!{struct PRINTER_INFO_7W {
    pszObjectGUID: LPWSTR,
    dwAction: DWORD,
}}
pub type PPRINTER_INFO_7W = *mut PRINTER_INFO_7W;
pub type LPPRINTER_INFO_7W = *mut PRINTER_INFO_7W;
pub const DSPRINT_PUBLISH: DWORD = 0x00000001;
pub const DSPRINT_UPDATE: DWORD = 0x00000002;
pub const DSPRINT_UNPUBLISH: DWORD = 0x00000004;
pub const DSPRINT_REPUBLISH: DWORD = 0x00000008;
pub const DSPRINT_PENDING: DWORD = 0x80000000;
STRUCT!{struct PRINTER_INFO_8A {
    pDevMode: LPDEVMODEA,
}}
pub type PPRINTER_INFO_8A = *mut PRINTER_INFO_8A;
pub type LPPRINTER_INFO_8A = *mut PRINTER_INFO_8A;
STRUCT!{struct PRINTER_INFO_8W {
    pDevMode: LPDEVMODEW,
}}
pub type PPRINTER_INFO_8W = *mut PRINTER_INFO_8W;
pub type LPPRINTER_INFO_8W = *mut PRINTER_INFO_8W;
STRUCT!{struct PRINTER_INFO_9A {
    pDevMode: LPDEVMODEA,
}}
pub type PPRINTER_INFO_9A = *mut PRINTER_INFO_9A;
pub type LPPRINTER_INFO_9A = *mut PRINTER_INFO_9A;
STRUCT!{struct PRINTER_INFO_9W {
    pDevMode: LPDEVMODEW,
}}
pub type PPRINTER_INFO_9W = *mut PRINTER_INFO_9W;
pub type LPPRINTER_INFO_9W = *mut PRINTER_INFO_9W;
pub const PRINTER_CONTROL_PAUSE: DWORD = 1;
pub const PRINTER_CONTROL_RESUME: DWORD = 2;
pub const PRINTER_CONTROL_PURGE: DWORD = 3;
pub const PRINTER_CONTROL_SET_STATUS: DWORD = 4;
pub const PRINTER_STATUS_PAUSED: DWORD = 0x00000001;
pub const PRINTER_STATUS_ERROR: DWORD = 0x00000002;
pub const PRINTER_STATUS_PENDING_DELETION: DWORD = 0x00000004;
pub const PRINTER_STATUS_PAPER_JAM: DWORD = 0x00000008;
pub const PRINTER_STATUS_PAPER_OUT: DWORD = 0x00000010;
pub const PRINTER_STATUS_MANUAL_FEED: DWORD = 0x00000020;
pub const PRINTER_STATUS_PAPER_PROBLEM: DWORD = 0x00000040;
pub const PRINTER_STATUS_OFFLINE: DWORD = 0x00000080;
pub const PRINTER_STATUS_IO_ACTIVE: DWORD = 0x00000100;
pub const PRINTER_STATUS_BUSY: DWORD = 0x00000200;
pub const PRINTER_STATUS_PRINTING: DWORD = 0x00000400;
pub const PRINTER_STATUS_OUTPUT_BIN_FULL: DWORD = 0x00000800;
pub const PRINTER_STATUS_NOT_AVAILABLE: DWORD = 0x00001000;
pub const PRINTER_STATUS_WAITING: DWORD = 0x00002000;
pub const PRINTER_STATUS_PROCESSING: DWORD = 0x00004000;
pub const PRINTER_STATUS_INITIALIZING: DWORD = 0x00008000;
pub const PRINTER_STATUS_WARMING_UP: DWORD = 0x00010000;
pub const PRINTER_STATUS_TONER_LOW: DWORD = 0x00020000;
pub const PRINTER_STATUS_NO_TONER: DWORD = 0x00040000;
pub const PRINTER_STATUS_PAGE_PUNT: DWORD = 0x00080000;
pub const PRINTER_STATUS_USER_INTERVENTION: DWORD = 0x00100000;
pub const PRINTER_STATUS_OUT_OF_MEMORY: DWORD = 0x00200000;
pub const PRINTER_STATUS_DOOR_OPEN: DWORD = 0x00400000;
pub const PRINTER_STATUS_SERVER_UNKNOWN: DWORD = 0x00800000;
pub const PRINTER_STATUS_POWER_SAVE: DWORD = 0x01000000;
pub const PRINTER_STATUS_SERVER_OFFLINE: DWORD = 0x02000000;
pub const PRINTER_STATUS_DRIVER_UPDATE_NEEDED: DWORD = 0x04000000;
pub const PRINTER_ATTRIBUTE_QUEUED: DWORD = 0x00000001;
pub const PRINTER_ATTRIBUTE_DIRECT: DWORD = 0x00000002;
pub const PRINTER_ATTRIBUTE_DEFAULT: DWORD = 0x00000004;
pub const PRINTER_ATTRIBUTE_SHARED: DWORD = 0x00000008;
pub const PRINTER_ATTRIBUTE_NETWORK: DWORD = 0x00000010;
pub const PRINTER_ATTRIBUTE_HIDDEN: DWORD = 0x00000020;
pub const PRINTER_ATTRIBUTE_LOCAL: DWORD = 0x00000040;
pub const PRINTER_ATTRIBUTE_ENABLE_DEVQ: DWORD = 0x00000080;
pub const PRINTER_ATTRIBUTE_KEEPPRINTEDJOBS: DWORD = 0x00000100;
pub const PRINTER_ATTRIBUTE_DO_COMPLETE_FIRST: DWORD = 0x00000200;
pub const PRINTER_ATTRIBUTE_WORK_OFFLINE: DWORD = 0x00000400;
pub const PRINTER_ATTRIBUTE_ENABLE_BIDI: DWORD = 0x00000800;
pub const PRINTER_ATTRIBUTE_RAW_ONLY: DWORD = 0x00001000;
pub const PRINTER_ATTRIBUTE_PUBLISHED: DWORD = 0x00002000;
pub const PRINTER_ATTRIBUTE_FAX: DWORD = 0x00004000;
pub const PRINTER_ATTRIBUTE_TS: DWORD = 0x00008000;
pub const PRINTER_ATTRIBUTE_PUSHED_USER: DWORD = 0x00020000;
pub const PRINTER_ATTRIBUTE_PUSHED_MACHINE: DWORD = 0x00040000;
pub const PRINTER_ATTRIBUTE_MACHINE: DWORD = 0x00080000;
pub const PRINTER_ATTRIBUTE_FRIENDLY_NAME: DWORD = 0x00100000;
pub const PRINTER_ATTRIBUTE_TS_GENERIC_DRIVER: DWORD = 0x00200000;
pub const PRINTER_ATTRIBUTE_PER_USER: DWORD = 0x00400000;
pub const PRINTER_ATTRIBUTE_ENTERPRISE_CLOUD: DWORD = 0x00800000;
pub const NO_PRIORITY: DWORD = 0;
pub const MAX_PRIORITY: DWORD = 99;
pub const MIN_PRIORITY: DWORD = 1;
pub const DEF_PRIORITY: DWORD = 1;
STRUCT!{struct JOB_INFO_1A {
    JobId: DWORD,
    pPrinterName: LPSTR,
    pMachineName: LPSTR,
    pUserName: LPSTR,
    pDocument: LPSTR,
    pDatatype: LPSTR,
    pStatus: LPSTR,
    Status: DWORD,
    Priority: DWORD,
    Position: DWORD,
    TotalPages: DWORD,
    PagesPrinted: DWORD,
    Submitted: SYSTEMTIME,
}}
pub type PJOB_INFO_1A = *mut JOB_INFO_1A;
pub type LPJOB_INFO_1A = *mut JOB_INFO_1A;
STRUCT!{struct JOB_INFO_1W {
    JobId: DWORD,
    pPrinterName: LPWSTR,
    pMachineName: LPWSTR,
    pUserName: LPWSTR,
    pDocument: LPWSTR,
    pDatatype: LPWSTR,
    pStatus: LPWSTR,
    Status: DWORD,
    Priority: DWORD,
    Position: DWORD,
    TotalPages: DWORD,
    PagesPrinted: DWORD,
    Submitted: SYSTEMTIME,
}}
pub type PJOB_INFO_1W = *mut JOB_INFO_1W;
pub type LPJOB_INFO_1W = *mut JOB_INFO_1W;
STRUCT!{struct JOB_INFO_2A {
    JobId: DWORD,
    pPrinterName: LPSTR,
    pMachineName: LPSTR,
    pUserName: LPSTR,
    pDocument: LPSTR,
    pNotifyName: LPSTR,
    pDatatype: LPSTR,
    pPrintProcessor: LPSTR,
    pParameters: LPSTR,
    pDriverName: LPSTR,
    pDevMode: LPDEVMODEA,
    pStatus: LPSTR,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    Status: DWORD,
    Priority: DWORD,
    Position: DWORD,
    StartTime: DWORD,
    UntilTime: DWORD,
    TotalPages: DWORD,
    Size: DWORD,
    Submitted: SYSTEMTIME,
    Time: DWORD,
    PagesPrinted: DWORD,
}}
pub type PJOB_INFO_2A = *mut JOB_INFO_2A;
pub type LPJOB_INFO_2A = *mut JOB_INFO_2A;
STRUCT!{struct JOB_INFO_2W {
    JobId: DWORD,
    pPrinterName: LPWSTR,
    pMachineName: LPWSTR,
    pUserName: LPWSTR,
    pDocument: LPWSTR,
    pNotifyName: LPWSTR,
    pDatatype: LPWSTR,
    pPrintProcessor: LPWSTR,
    pParameters: LPWSTR,
    pDriverName: LPWSTR,
    pDevMode: LPDEVMODEW,
    pStatus: LPWSTR,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    Status: DWORD,
    Priority: DWORD,
    Position: DWORD,
    StartTime: DWORD,
    UntilTime: DWORD,
    TotalPages: DWORD,
    Size: DWORD,
    Submitted: SYSTEMTIME,
    Time: DWORD,
    PagesPrinted: DWORD,
}}
pub type PJOB_INFO_2W = *mut JOB_INFO_2W;
pub type LPJOB_INFO_2W = *mut JOB_INFO_2W;
STRUCT!{struct JOB_INFO_3 {
    JobId: DWORD,
    NextJobId: DWORD,
    Reserved: DWORD,
}}
pub type PJOB_INFO_3 = *mut JOB_INFO_3;
pub type LPJOB_INFO_3 = *mut JOB_INFO_3;
STRUCT!{struct JOB_INFO_4A {
    JobId: DWORD,
    pPrinterName: LPSTR,
    pMachineName: LPSTR,
    pUserName: LPSTR,
    pDocument: LPSTR,
    pNotifyName: LPSTR,
    pDatatype: LPSTR,
    pPrintProcessor: LPSTR,
    pParameters: LPSTR,
    pDriverName: LPSTR,
    pDevMode: LPDEVMODEA,
    pStatus: LPSTR,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    Status: DWORD,
    Priority: DWORD,
    Position: DWORD,
    StartTime: DWORD,
    UntilTime: DWORD,
    TotalPages: DWORD,
    Size: DWORD,
    Submitted: SYSTEMTIME,
    Time: DWORD,
    PagesPrinted: DWORD,
    SizeHigh: LONG,
}}
pub type PJOB_INFO_4A = *mut JOB_INFO_4A;
pub type LPJOB_INFO_4A = *mut JOB_INFO_4A;
STRUCT!{struct JOB_INFO_4W {
    JobId: DWORD,
    pPrinterName: LPWSTR,
    pMachineName: LPWSTR,
    pUserName: LPWSTR,
    pDocument: LPWSTR,
    pNotifyName: LPWSTR,
    pDatatype: LPWSTR,
    pPrintProcessor: LPWSTR,
    pParameters: LPWSTR,
    pDriverName: LPWSTR,
    pDevMode: LPDEVMODEW,
    pStatus: LPWSTR,
    pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    Status: DWORD,
    Priority: DWORD,
    Position: DWORD,
    StartTime: DWORD,
    UntilTime: DWORD,
    TotalPages: DWORD,
    Size: DWORD,
    Submitted: SYSTEMTIME,
    Time: DWORD,
    PagesPrinted: DWORD,
    SizeHigh: LONG,
}}
pub type PJOB_INFO_4W = *mut JOB_INFO_4W;
pub type LPJOB_INFO_4W = *mut JOB_INFO_4W;
pub const JOB_CONTROL_PAUSE: DWORD = 1;
pub const JOB_CONTROL_RESUME: DWORD = 2;
pub const JOB_CONTROL_CANCEL: DWORD = 3;
pub const JOB_CONTROL_RESTART: DWORD = 4;
pub const JOB_CONTROL_DELETE: DWORD = 5;
pub const JOB_CONTROL_SENT_TO_PRINTER: DWORD = 6;
pub const JOB_CONTROL_LAST_PAGE_EJECTED: DWORD = 7;
pub const JOB_STATUS_PAUSED: DWORD = 0x00000001;
pub const JOB_STATUS_ERROR: DWORD = 0x00000002;
pub const JOB_STATUS_DELETING: DWORD = 0x00000004;
pub const JOB_STATUS_SPOOLING: DWORD = 0x00000008;
pub const JOB_STATUS_PRINTING: DWORD = 0x00000010;
pub const JOB_STATUS_OFFLINE: DWORD = 0x00000020;
pub const JOB_STATUS_PAPEROUT: DWORD = 0x00000040;
pub const JOB_STATUS_PRINTED: DWORD = 0x00000080;
pub const JOB_STATUS_DELETED: DWORD = 0x00000100;
pub const JOB_STATUS_BLOCKED_DEVQ: DWORD = 0x00000200;
pub const JOB_STATUS_USER_INTERVENTION: DWORD = 0x00000400;
pub const JOB_STATUS_RESTART: DWORD = 0x00000800;
pub const JOB_POSITION_UNSPECIFIED: DWORD = 0;
STRUCT!{struct ADDJOB_INFO_1A {
    Path: LPSTR,
    JobId: DWORD,
}}
pub type PADDJOB_INFO_1A = *mut ADDJOB_INFO_1A;
pub type LPADDJOB_INFO_1A = *mut ADDJOB_INFO_1A;
STRUCT!{struct ADDJOB_INFO_1W {
    Path: LPWSTR,
    JobId: DWORD,
}}
pub type PADDJOB_INFO_1W = *mut ADDJOB_INFO_1W;
pub type LPADDJOB_INFO_1W = *mut ADDJOB_INFO_1W;
STRUCT!{struct DRIVER_INFO_1A {
    pName: LPSTR,
}}
pub type PDRIVER_INFO_1A = *mut DRIVER_INFO_1A;
pub type LPDRIVER_INFO_1A = *mut DRIVER_INFO_1A;
STRUCT!{struct DRIVER_INFO_1W {
    pName: LPWSTR,
}}
pub type PDRIVER_INFO_1W = *mut DRIVER_INFO_1W;
pub type LPDRIVER_INFO_1W = *mut DRIVER_INFO_1W;
STRUCT!{struct DRIVER_INFO_2A {
    cVersion: DWORD,
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDriverPath: LPSTR,
    pDataFile: LPSTR,
    pConfigFile: LPSTR,
}}
pub type PDRIVER_INFO_2A = *mut DRIVER_INFO_2A;
pub type LPDRIVER_INFO_2A = *mut DRIVER_INFO_2A;
STRUCT!{struct DRIVER_INFO_2W {
    cVersion: DWORD,
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDriverPath: LPWSTR,
    pDataFile: LPWSTR,
    pConfigFile: LPWSTR,
}}
pub type PDRIVER_INFO_2W = *mut DRIVER_INFO_2W;
pub type LPDRIVER_INFO_2W = *mut DRIVER_INFO_2W;
STRUCT!{struct DRIVER_INFO_3A {
    cVersion: DWORD,
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDriverPath: LPSTR,
    pDataFile: LPSTR,
    pConfigFile: LPSTR,
    pHelpFile: LPSTR,
    pDependentFiles: LPSTR,
    pMonitorName: LPSTR,
    pDefaultDataType: LPSTR,
}}
pub type PDRIVER_INFO_3A = *mut DRIVER_INFO_3A;
pub type LPDRIVER_INFO_3A = *mut DRIVER_INFO_3A;
STRUCT!{struct DRIVER_INFO_3W {
    cVersion: DWORD,
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDriverPath: LPWSTR,
    pDataFile: LPWSTR,
    pConfigFile: LPWSTR,
    pHelpFile: LPWSTR,
    pDependentFiles: LPWSTR,
    pMonitorName: LPWSTR,
    pDefaultDataType: LPWSTR,
}}
pub type PDRIVER_INFO_3W = *mut DRIVER_INFO_3W;
pub type LPDRIVER_INFO_3W = *mut DRIVER_INFO_3W;
STRUCT!{struct DRIVER_INFO_4A {
    cVersion: DWORD,
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDriverPath: LPSTR,
    pDataFile: LPSTR,
    pConfigFile: LPSTR,
    pHelpFile: LPSTR,
    pDependentFiles: LPSTR,
    pMonitorName: LPSTR,
    pDefaultDataType: LPSTR,
    pszzPreviousNames: LPSTR,
}}
pub type PDRIVER_INFO_4A = *mut DRIVER_INFO_4A;
pub type LPDRIVER_INFO_4A = *mut DRIVER_INFO_4A;
STRUCT!{struct DRIVER_INFO_4W {
    cVersion: DWORD,
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDriverPath: LPWSTR,
    pDataFile: LPWSTR,
    pConfigFile: LPWSTR,
    pHelpFile: LPWSTR,
    pDependentFiles: LPWSTR,
    pMonitorName: LPWSTR,
    pDefaultDataType: LPWSTR,
    pszzPreviousNames: LPWSTR,
}}
pub type PDRIVER_INFO_4W = *mut DRIVER_INFO_4W;
pub type LPDRIVER_INFO_4W = *mut DRIVER_INFO_4W;
STRUCT!{struct DRIVER_INFO_5A {
    cVersion: DWORD,
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDriverPath: LPSTR,
    pDataFile: LPSTR,
    pConfigFile: LPSTR,
    dwDriverAttributes: DWORD,
    dwConfigVersion: DWORD,
    dwDriverVersion: DWORD,
}}
pub type PDRIVER_INFO_5A = *mut DRIVER_INFO_5A;
pub type LPDRIVER_INFO_5A = *mut DRIVER_INFO_5A;
STRUCT!{struct DRIVER_INFO_5W {
    cVersion: DWORD,
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDriverPath: LPWSTR,
    pDataFile: LPWSTR,
    pConfigFile: LPWSTR,
    dwDriverAttributes: DWORD,
    dwConfigVersion: DWORD,
    dwDriverVersion: DWORD,
}}
pub type PDRIVER_INFO_5W = *mut DRIVER_INFO_5W;
pub type LPDRIVER_INFO_5W = *mut DRIVER_INFO_5W;
STRUCT!{struct DRIVER_INFO_6A {
    cVersion: DWORD,
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDriverPath: LPSTR,
    pDataFile: LPSTR,
    pConfigFile: LPSTR,
    pHelpFile: LPSTR,
    pDependentFiles: LPSTR,
    pMonitorName: LPSTR,
    pDefaultDataType: LPSTR,
    pszzPreviousNames: LPSTR,
    ftDriverDate: FILETIME,
    dwlDriverVersion: DWORDLONG,
    pszMfgName: LPSTR,
    pszOEMUrl: LPSTR,
    pszHardwareID: LPSTR,
    pszProvider: LPSTR,
}}
pub type PDRIVER_INFO_6A = *mut DRIVER_INFO_6A;
pub type LPDRIVER_INFO_6A = *mut DRIVER_INFO_6A;
STRUCT!{struct DRIVER_INFO_6W {
    cVersion: DWORD,
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDriverPath: LPWSTR,
    pDataFile: LPWSTR,
    pConfigFile: LPWSTR,
    pHelpFile: LPWSTR,
    pDependentFiles: LPWSTR,
    pMonitorName: LPWSTR,
    pDefaultDataType: LPWSTR,
    pszzPreviousNames: LPWSTR,
    ftDriverDate: FILETIME,
    dwlDriverVersion: DWORDLONG,
    pszMfgName: LPWSTR,
    pszOEMUrl: LPWSTR,
    pszHardwareID: LPWSTR,
    pszProvider: LPWSTR,
}}
pub type PDRIVER_INFO_6W = *mut DRIVER_INFO_6W;
pub type LPDRIVER_INFO_6W = *mut DRIVER_INFO_6W;
pub const PRINTER_DRIVER_PACKAGE_AWARE: DWORD = 0x00000001;
pub const PRINTER_DRIVER_XPS: DWORD = 0x00000002;
pub const PRINTER_DRIVER_SANDBOX_ENABLED: DWORD = 0x00000004;
pub const PRINTER_DRIVER_CLASS: DWORD = 0x00000008;
pub const PRINTER_DRIVER_DERIVED: DWORD = 0x00000010;
pub const PRINTER_DRIVER_NOT_SHAREABLE: DWORD = 0x00000020;
pub const PRINTER_DRIVER_CATEGORY_FAX: DWORD = 0x00000040;
pub const PRINTER_DRIVER_CATEGORY_FILE: DWORD = 0x00000080;
pub const PRINTER_DRIVER_CATEGORY_VIRTUAL: DWORD = 0x00000100;
pub const PRINTER_DRIVER_CATEGORY_SERVICE: DWORD = 0x00000200;
pub const PRINTER_DRIVER_SOFT_RESET_REQUIRED: DWORD = 0x00000400;
pub const PRINTER_DRIVER_SANDBOX_DISABLED: DWORD = 0x00000800;
pub const PRINTER_DRIVER_CATEGORY_3D: DWORD = 0x00001000;
pub const PRINTER_DRIVER_CATEGORY_CLOUD: DWORD = 0x00002000;
STRUCT!{struct DRIVER_INFO_8A {
    cVersion: DWORD,
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDriverPath: LPSTR,
    pDataFile: LPSTR,
    pConfigFile: LPSTR,
    pHelpFile: LPSTR,
    pDependentFiles: LPSTR,
    pMonitorName: LPSTR,
    pDefaultDataType: LPSTR,
    pszzPreviousNames: LPSTR,
    ftDriverDate: FILETIME,
    dwlDriverVersion: DWORDLONG,
    pszMfgName: LPSTR,
    pszOEMUrl: LPSTR,
    pszHardwareID: LPSTR,
    pszProvider: LPSTR,
    pszPrintProcessor: LPSTR,
    pszVendorSetup: LPSTR,
    pszzColorProfiles: LPSTR,
    pszInfPath: LPSTR,
    dwPrinterDriverAttributes: DWORD,
    pszzCoreDriverDependencies: LPSTR,
    ftMinInboxDriverVerDate: FILETIME,
    dwlMinInboxDriverVerVersion: DWORDLONG,
}}
pub type PDRIVER_INFO_8A = *mut DRIVER_INFO_8A;
pub type LPDRIVER_INFO_8A = *mut DRIVER_INFO_8A;
STRUCT!{struct DRIVER_INFO_8W {
    cVersion: DWORD,
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDriverPath: LPWSTR,
    pDataFile: LPWSTR,
    pConfigFile: LPWSTR,
    pHelpFile: LPWSTR,
    pDependentFiles: LPWSTR,
    pMonitorName: LPWSTR,
    pDefaultDataType: LPWSTR,
    pszzPreviousNames: LPWSTR,
    ftDriverDate: FILETIME,
    dwlDriverVersion: DWORDLONG,
    pszMfgName: LPWSTR,
    pszOEMUrl: LPWSTR,
    pszHardwareID: LPWSTR,
    pszProvider: LPWSTR,
    pszPrintProcessor: LPWSTR,
    pszVendorSetup: LPWSTR,
    pszzColorProfiles: LPWSTR,
    pszInfPath: LPWSTR,
    dwPrinterDriverAttributes: DWORD,
    pszzCoreDriverDependencies: LPWSTR,
    ftMinInboxDriverVerDate: FILETIME,
    dwlMinInboxDriverVerVersion: DWORDLONG,
}}
pub type PDRIVER_INFO_8W = *mut DRIVER_INFO_8W;
pub type LPDRIVER_INFO_8W = *mut DRIVER_INFO_8W;
pub const DRIVER_KERNELMODE: DWORD = 0x00000001;
pub const DRIVER_USERMODE: DWORD = 0x00000002;
pub const DPD_DELETE_UNUSED_FILES: DWORD = 0x00000001;
pub const DPD_DELETE_SPECIFIC_VERSION: DWORD = 0x00000002;
pub const DPD_DELETE_ALL_FILES: DWORD = 0x00000004;
pub const APD_STRICT_UPGRADE: DWORD = 0x00000001;
pub const APD_STRICT_DOWNGRADE: DWORD = 0x00000002;
pub const APD_COPY_ALL_FILES: DWORD = 0x00000004;
pub const APD_COPY_NEW_FILES: DWORD = 0x00000008;
pub const APD_COPY_FROM_DIRECTORY: DWORD = 0x00000010;
STRUCT!{struct DOC_INFO_1A {
    pDocName: LPSTR,
    pOutputFile: LPSTR,
    pDatatype: LPSTR,
}}
pub type PDOC_INFO_1A = *mut DOC_INFO_1A;
pub type LPDOC_INFO_1A = *mut DOC_INFO_1A;
STRUCT!{struct DOC_INFO_1W {
    pDocName: LPWSTR,
    pOutputFile: LPWSTR,
    pDatatype: LPWSTR,
}}
pub type PDOC_INFO_1W = *mut DOC_INFO_1W;
pub type LPDOC_INFO_1W = *mut DOC_INFO_1W;
STRUCT!{struct FORM_INFO_1A {
    Flags: DWORD,
    pName: LPSTR,
    Size: SIZEL,
    ImageableArea: RECTL,
}}
pub type PFORM_INFO_1A = *mut FORM_INFO_1A;
pub type LPFORM_INFO_1A = *mut FORM_INFO_1A;
STRUCT!{struct FORM_INFO_1W {
    Flags: DWORD,
    pName: LPWSTR,
    Size: SIZEL,
    ImageableArea: RECTL,
}}
pub type PFORM_INFO_1W = *mut FORM_INFO_1W;
pub type LPFORM_INFO_1W = *mut FORM_INFO_1W;
pub const STRING_NONE: DWORD = 0x00000001;
pub const STRING_MUIDLL: DWORD = 0x00000002;
pub const STRING_LANGPAIR: DWORD = 0x00000004;
pub const MAX_FORM_KEYWORD_LENGTH: usize = 63 + 1;
STRUCT!{struct FORM_INFO_2A {
    Flags: DWORD,
    pName: LPCSTR,
    Size: SIZEL,
    ImageableArea: RECTL,
    pKeyword: LPCSTR,
    StringType: DWORD,
    pMuiDll: LPCSTR,
    dwResourceId: DWORD,
    pDisplayName: LPCSTR,
    wLangId: LANGID,
}}
pub type PFORM_INFO_2A = *mut FORM_INFO_2A;
pub type LPFORM_INFO_2A = *mut FORM_INFO_2A;
STRUCT!{struct FORM_INFO_2W {
    Flags: DWORD,
    pName: LPCWSTR,
    Size: SIZEL,
    ImageableArea: RECTL,
    pKeyword: LPCSTR,
    StringType: DWORD,
    pMuiDll: LPCWSTR,
    dwResourceId: DWORD,
    pDisplayName: LPCWSTR,
    wLangId: LANGID,
}}
pub type PFORM_INFO_2W = *mut FORM_INFO_2W;
pub type LPFORM_INFO_2W = *mut FORM_INFO_2W;
STRUCT!{struct DOC_INFO_2A {
    pDocName: LPSTR,
    pOutputFile: LPSTR,
    pDatatype: LPSTR,
    dwMode: DWORD,
    JobId: DWORD,
}}
pub type PDOC_INFO_2A = *mut DOC_INFO_2A;
pub type LPDOC_INFO_2A = *mut DOC_INFO_2A;
STRUCT!{struct DOC_INFO_2W {
    pDocName: LPWSTR,
    pOutputFile: LPWSTR,
    pDatatype: LPWSTR,
    dwMode: DWORD,
    JobId: DWORD,
}}
pub type PDOC_INFO_2W = *mut DOC_INFO_2W;
pub type LPDOC_INFO_2W = *mut DOC_INFO_2W;
pub const DI_CHANNEL: DWORD = 1;
pub const DI_READ_SPOOL_JOB: DWORD = 3;
STRUCT!{struct DOC_INFO_3A {
    pDocName: LPSTR,
    pOutputFile: LPSTR,
    pDatatype: LPSTR,
    dwFlags: DWORD,
}}
pub type PDOC_INFO_3A = *mut DOC_INFO_3A;
pub type LPDOC_INFO_3A = *mut DOC_INFO_3A;
STRUCT!{struct DOC_INFO_3W {
    pDocName: LPWSTR,
    pOutputFile: LPWSTR,
    pDatatype: LPWSTR,
    dwFlags: DWORD,
}}
pub type PDOC_INFO_3W = *mut DOC_INFO_3W;
pub type LPDOC_INFO_3W = *mut DOC_INFO_3W;
pub const DI_MEMORYMAP_WRITE: DWORD = 0x00000001;
pub const FORM_USER: DWORD = 0x00000000;
pub const FORM_BUILTIN: DWORD = 0x00000001;
pub const FORM_PRINTER: DWORD = 0x00000002;
STRUCT!{struct PRINTPROCESSOR_INFO_1A {
    pName: LPSTR,
}}
pub type PPRINTPROCESSOR_INFO_1A = *mut PRINTPROCESSOR_INFO_1A;
pub type LPPRINTPROCESSOR_INFO_1A = *mut PRINTPROCESSOR_INFO_1A;
STRUCT!{struct PRINTPROCESSOR_INFO_1W {
    pName: LPWSTR,
}}
pub type PPRINTPROCESSOR_INFO_1W = *mut PRINTPROCESSOR_INFO_1W;
pub type LPPRINTPROCESSOR_INFO_1W = *mut PRINTPROCESSOR_INFO_1W;
STRUCT!{struct PRINTPROCESSOR_CAPS_1 {
    dwLevel: DWORD,
    dwNupOptions: DWORD,
    dwPageOrderFlags: DWORD,
    dwNumberOfCopies: DWORD,
}}
pub type PPRINTPROCESSOR_CAPS_1 = *mut PRINTPROCESSOR_CAPS_1;
STRUCT!{struct PRINTPROCESSOR_CAPS_2 {
    dwLevel: DWORD,
    dwNupOptions: DWORD,
    dwPageOrderFlags: DWORD,
    dwNumberOfCopies: DWORD,
    dwDuplexHandlingCaps: DWORD,
    dwNupDirectionCaps: DWORD,
    dwNupBorderCaps: DWORD,
    dwBookletHandlingCaps: DWORD,
    dwScalingCaps: DWORD,
}}
pub type PPRINTPROCESSOR_CAPS_2 = *mut PRINTPROCESSOR_CAPS_2;
pub const PPCAPS_RIGHT_THEN_DOWN: DWORD = 0x00000001;
pub const PPCAPS_DOWN_THEN_RIGHT: DWORD = 0x00000001 << 1;
pub const PPCAPS_LEFT_THEN_DOWN: DWORD = 0x00000001 << 2;
pub const PPCAPS_DOWN_THEN_LEFT: DWORD = 0x00000001 << 3;
pub const PPCAPS_BORDER_PRINT: DWORD = 0x00000001;
pub const PPCAPS_BOOKLET_EDGE: DWORD = 0x00000001;
pub const PPCAPS_REVERSE_PAGES_FOR_REVERSE_DUPLEX: DWORD = 0x00000001;
pub const PPCAPS_DONT_SEND_EXTRA_PAGES_FOR_DUPLEX: DWORD = 0x00000001 << 1;
pub const PPCAPS_SQUARE_SCALING: DWORD = 0x00000001;
STRUCT!{struct PORT_INFO_1A {
    pName: LPSTR,
}}
pub type PPORT_INFO_1A = *mut PORT_INFO_1A;
pub type LPPORT_INFO_1A = *mut PORT_INFO_1A;
STRUCT!{struct PORT_INFO_1W {
    pName: LPWSTR,
}}
pub type PPORT_INFO_1W = *mut PORT_INFO_1W;
pub type LPPORT_INFO_1W = *mut PORT_INFO_1W;
STRUCT!{struct PORT_INFO_2A {
    pPortName: LPSTR,
    pMonitorName: LPSTR,
    pDescription: LPSTR,
    fPortType: DWORD,
    Reserved: DWORD,
}}
pub type PPORT_INFO_2A = *mut PORT_INFO_2A;
pub type LPPORT_INFO_2A = *mut PORT_INFO_2A;
STRUCT!{struct PORT_INFO_2W {
    pPortName: LPWSTR,
    pMonitorName: LPWSTR,
    pDescription: LPWSTR,
    fPortType: DWORD,
    Reserved: DWORD,
}}
pub type PPORT_INFO_2W = *mut PORT_INFO_2W;
pub type LPPORT_INFO_2W = *mut PORT_INFO_2W;
pub const PORT_TYPE_WRITE: DWORD = 0x0001;
pub const PORT_TYPE_READ: DWORD = 0x0002;
pub const PORT_TYPE_REDIRECTED: DWORD = 0x0004;
pub const PORT_TYPE_NET_ATTACHED: DWORD = 0x0008;
STRUCT!{struct PORT_INFO_3A {
    dwStatus: DWORD,
    pszStatus: LPSTR,
    dwSeverity: DWORD,
}}
pub type PPORT_INFO_3A = *mut PORT_INFO_3A;
pub type LPPORT_INFO_3A = *mut PORT_INFO_3A;
STRUCT!{struct PORT_INFO_3W {
    dwStatus: DWORD,
    pszStatus: LPWSTR,
    dwSeverity: DWORD,
}}
pub type PPORT_INFO_3W = *mut PORT_INFO_3W;
pub type LPPORT_INFO_3W = *mut PORT_INFO_3W;
pub const PORT_STATUS_TYPE_ERROR: DWORD = 1;
pub const PORT_STATUS_TYPE_WARNING: DWORD = 2;
pub const PORT_STATUS_TYPE_INFO: DWORD = 3;
pub const PORT_STATUS_OFFLINE: DWORD = 1;
pub const PORT_STATUS_PAPER_JAM: DWORD = 2;
pub const PORT_STATUS_PAPER_OUT: DWORD = 3;
pub const PORT_STATUS_OUTPUT_BIN_FULL: DWORD = 4;
pub const PORT_STATUS_PAPER_PROBLEM: DWORD = 5;
pub const PORT_STATUS_NO_TONER: DWORD = 6;
pub const PORT_STATUS_DOOR_OPEN: DWORD = 7;
pub const PORT_STATUS_USER_INTERVENTION: DWORD = 8;
pub const PORT_STATUS_OUT_OF_MEMORY: DWORD = 9;
pub const PORT_STATUS_TONER_LOW: DWORD = 10;
pub const PORT_STATUS_WARMING_UP: DWORD = 11;
pub const PORT_STATUS_POWER_SAVE: DWORD = 12;
STRUCT!{struct MONITOR_INFO_1A {
    pName: LPSTR,
}}
pub type PMONITOR_INFO_1A = *mut MONITOR_INFO_1A;
pub type LPMONITOR_INFO_1A = *mut MONITOR_INFO_1A;
STRUCT!{struct MONITOR_INFO_1W {
    pName: LPWSTR,
}}
pub type PMONITOR_INFO_1W = *mut MONITOR_INFO_1W;
pub type LPMONITOR_INFO_1W = *mut MONITOR_INFO_1W;
STRUCT!{struct MONITOR_INFO_2A {
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDLLName: LPSTR,
}}
pub type PMONITOR_INFO_2A = *mut MONITOR_INFO_2A;
pub type LPMONITOR_INFO_2A = *mut MONITOR_INFO_2A;
STRUCT!{struct MONITOR_INFO_2W {
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDLLName: LPWSTR,
}}
pub type PMONITOR_INFO_2W = *mut MONITOR_INFO_2W;
pub type LPMONITOR_INFO_2W = *mut MONITOR_INFO_2W;
STRUCT!{struct DATATYPES_INFO_1A {
    pName: LPSTR,
}}
pub type PDATATYPES_INFO_1A = *mut DATATYPES_INFO_1A;
pub type LPDATATYPES_INFO_1A = *mut DATATYPES_INFO_1A;
STRUCT!{struct DATATYPES_INFO_1W {
    pName: LPWSTR,
}}
pub type PDATATYPES_INFO_1W = *mut DATATYPES_INFO_1W;
pub type LPDATATYPES_INFO_1W = *mut DATATYPES_INFO_1W;
STRUCT!{struct PRINTER_DEFAULTSA {
    pDataType: LPSTR,
    pDevMode: LPDEVMODEA,
    DesiredAccess: ACCESS_MASK,
}}
pub type PPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
pub type LPPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
STRUCT!{struct PRINTER_DEFAULTSW {
    pDataType: LPWSTR,
    pDevMode: LPDEVMODEW,
    DesiredAccess: ACCESS_MASK,
}}
pub type PPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
pub type LPPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
STRUCT!{struct PRINTER_ENUM_VALUESA {
    pValueName: LPSTR,
    cbValueName: DWORD,
    dwType: DWORD,
    pData: LPBYTE,
    cbData: DWORD,
}}
pub type PPRINTER_ENUM_VALUESA = *mut PRINTER_ENUM_VALUESA;
pub type LPPRINTER_ENUM_VALUESA = *mut PRINTER_ENUM_VALUESA;
STRUCT!{struct PRINTER_ENUM_VALUESW {
    pValueName: LPWSTR,
    cbValueName: DWORD,
    dwType: DWORD,
    pData: LPBYTE,
    cbData: DWORD,
}}
pub type PPRINTER_ENUM_VALUESW = *mut PRINTER_ENUM_VALUESW;
pub type LPPRINTER_ENUM_VALUESW = *mut PRINTER_ENUM_VALUESW;
extern "system" {
    pub fn EnumPrintersA(
        Flags: DWORD,
        Name: LPSTR,
        Level: DWORD,
        pPrinterEnum: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintersW(
        Flags: DWORD,
        Name: LPWSTR,
        Level: DWORD,
        pPrinterEnum: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
}
pub const PRINTER_ENUM_DEFAULT: DWORD = 0x00000001;
pub const PRINTER_ENUM_LOCAL: DWORD = 0x00000002;
pub const PRINTER_ENUM_CONNECTIONS: DWORD = 0x00000004;
pub const PRINTER_ENUM_FAVORITE: DWORD = 0x00000004;
pub const PRINTER_ENUM_NAME: DWORD = 0x00000008;
pub const PRINTER_ENUM_REMOTE: DWORD = 0x00000010;
pub const PRINTER_ENUM_SHARED: DWORD = 0x00000020;
pub const PRINTER_ENUM_NETWORK: DWORD = 0x00000040;
pub const PRINTER_ENUM_EXPAND: DWORD = 0x00004000;
pub const PRINTER_ENUM_CONTAINER: DWORD = 0x00008000;
pub const PRINTER_ENUM_ICONMASK: DWORD = 0x00ff0000;
pub const PRINTER_ENUM_ICON1: DWORD = 0x00010000;
pub const PRINTER_ENUM_ICON2: DWORD = 0x00020000;
pub const PRINTER_ENUM_ICON3: DWORD = 0x00040000;
pub const PRINTER_ENUM_ICON4: DWORD = 0x00080000;
pub const PRINTER_ENUM_ICON5: DWORD = 0x00100000;
pub const PRINTER_ENUM_ICON6: DWORD = 0x00200000;
pub const PRINTER_ENUM_ICON7: DWORD = 0x00400000;
pub const PRINTER_ENUM_ICON8: DWORD = 0x00800000;
pub const PRINTER_ENUM_HIDE: DWORD = 0x01000000;
pub const PRINTER_ENUM_CATEGORY_ALL: DWORD = 0x02000000;
pub const PRINTER_ENUM_CATEGORY_3D: DWORD = 0x04000000;
pub const SPOOL_FILE_PERSISTENT: DWORD = 0x00000001;
pub const SPOOL_FILE_TEMPORARY: DWORD = 0x00000002;
extern "system" {
    pub fn GetSpoolFileHandle(
        hPrinter: HANDLE,
    ) -> HANDLE;
    pub fn CommitSpoolData(
        hPrinter: HANDLE,
        hSpoolFile: HANDLE,
        cbCommit: DWORD,
    ) -> HANDLE;
    pub fn CloseSpoolFileHandle(
        hPrinter: HANDLE,
        hSpoolFile: HANDLE,
    ) -> BOOL;
    pub fn OpenPrinterA(
        pPrinterName: LPSTR,
        phPrinter: LPHANDLE,
        pDefault: LPPRINTER_DEFAULTSA,
    ) -> BOOL;
    pub fn OpenPrinterW(
        pPrinterName: LPWSTR,
        phPrinter: LPHANDLE,
        pDefault: LPPRINTER_DEFAULTSW,
    ) -> BOOL;
    pub fn ResetPrinterA(
        hPrinter: HANDLE,
        pDefault: LPPRINTER_DEFAULTSA,
    ) -> BOOL;
    pub fn ResetPrinterW(
        hPrinter: HANDLE,
        pDefault: LPPRINTER_DEFAULTSW,
    ) -> BOOL;
    pub fn SetJobA(
        hPrinter: HANDLE,
        JobId: DWORD,
        Level: DWORD,
        pJob: LPBYTE,
        Command: DWORD,
    ) -> BOOL;
    pub fn SetJobW(
        hPrinter: HANDLE,
        JobId: DWORD,
        Level: DWORD,
        pJob: LPBYTE,
        Command: DWORD,
    ) -> BOOL;
    pub fn GetJobA(
        hPrinter: HANDLE,
        JobId: DWORD,
        Level: DWORD,
        pJob: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetJobW(
        hPrinter: HANDLE,
        JobId: DWORD,
        Level: DWORD,
        pJob: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn EnumJobsA(
        hPrinter: HANDLE,
        FirstJob: DWORD,
        NoJobs: DWORD,
        Level: DWORD,
        pJob: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumJobsW(
        hPrinter: HANDLE,
        FirstJob: DWORD,
        NoJobs: DWORD,
        Level: DWORD,
        pJob: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn AddPrinterA(
        pName: LPSTR,
        Level: DWORD,
        pPrinter: LPBYTE,
    ) -> HANDLE;
    pub fn AddPrinterW(
        pName: LPWSTR,
        Level: DWORD,
        pPrinter: LPBYTE,
    ) -> HANDLE;
    pub fn DeletePrinter(
        hPrinter: HANDLE,
    ) -> BOOL;
    pub fn SetPrinterA(
        hPrinter: HANDLE,
        Level: DWORD,
        pPrinter: LPBYTE,
        Command: DWORD,
    ) -> BOOL;
    pub fn SetPrinterW(
        hPrinter: HANDLE,
        Level: DWORD,
        pPrinter: LPBYTE,
        Command: DWORD,
    ) -> BOOL;
    pub fn GetPrinterA(
        hPrinter: HANDLE,
        Level: DWORD,
        pPrinter: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterW(
        hPrinter: HANDLE,
        Level: DWORD,
        pPrinter: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn AddPrinterDriverA(
        pName: LPSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
    ) -> BOOL;
    pub fn AddPrinterDriverW(
        pName: LPWSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
    ) -> BOOL;
    pub fn AddPrinterDriverExA(
        pName: LPSTR,
        Level: DWORD,
        pDriverInfo: PBYTE,
        dwFileCopyFlags: DWORD,
    ) -> BOOL;
    pub fn AddPrinterDriverExW(
        pName: LPWSTR,
        Level: DWORD,
        pDriverInfo: PBYTE,
        dwFileCopyFlags: DWORD,
    ) -> BOOL;
    pub fn EnumPrinterDriversA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrinterDriversW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriverA(
        hPrinter: HANDLE,
        pEnvironment: LPSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriverW(
        hPrinter: HANDLE,
        pEnvironment: LPWSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriverDirectoryA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        Level: DWORD,
        pDriverDirectory: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriverDirectoryW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        Level: DWORD,
        pDriverDirectory: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn DeletePrinterDriverA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        pDriverName: LPSTR,
    ) -> BOOL;
    pub fn DeletePrinterDriverW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        pDriverName: LPWSTR,
    ) -> BOOL;
    pub fn DeletePrinterDriverExA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        pDriverName: LPSTR,
        dwDeleteFlag: DWORD,
        dwVersionFlag: DWORD,
    ) -> BOOL;
    pub fn DeletePrinterDriverExW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        pDriverName: LPWSTR,
        dwDeleteFlag: DWORD,
        dwVersionFlag: DWORD,
    ) -> BOOL;
    pub fn AddPrintProcessorA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        pPathName: LPSTR,
        pPrintProcessorName: LPSTR,
    ) -> BOOL;
    pub fn AddPrintProcessorW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        pPathName: LPWSTR,
        pPrintProcessorName: LPWSTR,
    ) -> BOOL;
    pub fn EnumPrintProcessorsA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        Level: DWORD,
        pPrintProcessorInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintProcessorsW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        Level: DWORD,
        pPrintProcessorInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn GetPrintProcessorDirectoryA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        Level: DWORD,
        pPrintProcessorInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrintProcessorDirectoryW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        Level: DWORD,
        pPrintProcessorInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintProcessorDatatypesA(
        pName: LPSTR,
        pPrintProcessorName: LPSTR,
        Level: DWORD,
        pDatatypes: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPrintProcessorDatatypesW(
        pName: LPWSTR,
        pPrintProcessorName: LPWSTR,
        Level: DWORD,
        pDatatypes: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn DeletePrintProcessorA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        pPrintProcessorName: LPSTR,
    ) -> BOOL;
    pub fn DeletePrintProcessorW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        pPrintProcessorName: LPWSTR,
    ) -> BOOL;
    pub fn StartDocPrinterA(
        hPrinter: HANDLE,
        Level: DWORD,
        pDocInfo: LPBYTE,
    ) -> DWORD;
    pub fn StartDocPrinterW(
        hPrinter: HANDLE,
        Level: DWORD,
        pDocInfo: LPBYTE,
    ) -> DWORD;
    pub fn StartPagePrinter(
        hPrinter: HANDLE,
    ) -> BOOL;
    pub fn WritePrinter(
        hPrinter: HANDLE,
        pBuf: LPVOID,
        cbBuf: DWORD,
        pcWritten: LPDWORD,
    ) -> BOOL;
    pub fn FlushPrinter(
        hPrinter: HANDLE,
        pBuf: LPVOID,
        cbBuf: DWORD,
        pcWritten: LPDWORD,
        cSleep: DWORD,
    ) -> BOOL;
    pub fn EndPagePrinter(
        hPrinter: HANDLE,
    ) -> BOOL;
    pub fn AbortPrinter(
        hPrinter: HANDLE,
    ) -> BOOL;
    pub fn ReadPrinter(
        hPrinter: HANDLE,
        pBuf: LPVOID,
        cbBuf: DWORD,
        pNoBytesRead: LPDWORD,
    ) -> BOOL;
    pub fn EndDocPrinter(
        hPrinter: HANDLE,
    ) -> BOOL;
    pub fn AddJobA(
        hPrinter: HANDLE,
        Level: DWORD,
        pData: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn AddJobW(
        hPrinter: HANDLE,
        Level: DWORD,
        pData: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn ScheduleJob(
        hPrinter: HANDLE,
        JobId: DWORD,
    ) -> BOOL;
    pub fn PrinterProperties(
        hWnd: HWND,
        hPrinter: HANDLE,
    ) -> BOOL;
    pub fn DocumentPropertiesA(
        hWnd: HWND,
        hPrinter: HANDLE,
        pDeviceName: LPSTR,
        pDevModeOutput: PDEVMODEA,
        pDevModeInput: PDEVMODEA,
        fMode: DWORD,
    ) -> LONG;
    pub fn DocumentPropertiesW(
        hWnd: HWND,
        hPrinter: HANDLE,
        pDeviceName: LPWSTR,
        pDevModeOutput: PDEVMODEW,
        pDevModeInput: PDEVMODEW,
        fMode: DWORD,
    ) -> LONG;
    pub fn AdvancedDocumentPropertiesA(
        hWnd: HWND,
        hPrinter: HANDLE,
        pDeviceName: LPSTR,
        pDevModeOutput: PDEVMODEA,
        pDevModeInput: PDEVMODEA,
    ) -> LONG;
    pub fn AdvancedDocumentPropertiesW(
        hWnd: HWND,
        hPrinter: HANDLE,
        pDeviceName: LPWSTR,
        pDevModeOutput: PDEVMODEW,
        pDevModeInput: PDEVMODEW,
    ) -> LONG;
    pub fn ExtDeviceMode(
        hWnd: HWND,
        hInst: HANDLE,
        pDevModeOutput: LPDEVMODEA,
        pDeviceName: LPSTR,
        pPort: LPSTR,
        pDevModeInput: LPDEVMODEA,
        pProfile: LPSTR,
        fMode: DWORD,
    ) -> LONG;
    pub fn GetPrinterDataA(
        hPrinter: HANDLE,
        pValueName: LPSTR,
        pType: LPDWORD,
        pData: LPBYTE,
        nSize: DWORD,
        pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn GetPrinterDataW(
        hPrinter: HANDLE,
        pValueName: LPWSTR,
        pType: LPDWORD,
        pData: LPBYTE,
        nSize: DWORD,
        pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn GetPrinterDataExA(
        hPrinter: HANDLE,
        pKeyName: LPCSTR,
        pValueName: LPCSTR,
        pType: LPDWORD,
        pData: LPBYTE,
        nSize: DWORD,
        pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn GetPrinterDataExW(
        hPrinter: HANDLE,
        pKeyName: LPCWSTR,
        pValueName: LPCWSTR,
        pType: LPDWORD,
        pData: LPBYTE,
        nSize: DWORD,
        pcbNeeded: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDataA(
        hPrinter: HANDLE,
        dwIndex: DWORD,
        pValueName: LPSTR,
        cbValueName: DWORD,
        pcbValueName: LPDWORD,
        pType: LPDWORD,
        pData: LPBYTE,
        cbData: DWORD,
        pcbData: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDataW(
        hPrinter: HANDLE,
        dwIndex: DWORD,
        pValueName: LPWSTR,
        cbValueName: DWORD,
        pcbValueName: LPDWORD,
        pType: LPDWORD,
        pData: LPBYTE,
        cbData: DWORD,
        pcbData: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDataExA(
        hPrinter: HANDLE,
        pKeyName: LPCSTR,
        pEnumValues: LPBYTE,
        cbEnumValues: DWORD,
        pcbEnumValues: LPDWORD,
        pnEnumValues: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterDataExW(
        hPrinter: HANDLE,
        pKeyName: LPCWSTR,
        pEnumValues: LPBYTE,
        cbEnumValues: DWORD,
        pcbEnumValues: LPDWORD,
        pnEnumValues: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterKeyA(
        hPrinter: HANDLE,
        pKeyName: LPCSTR,
        pSubKey: LPSTR,
        cbSubkey: DWORD,
        pcbSubkey: LPDWORD,
    ) -> DWORD;
    pub fn EnumPrinterKeyW(
        hPrinter: HANDLE,
        pKeyName: LPCWSTR,
        pSubKey: LPWSTR,
        cbSubkey: DWORD,
        pcbSubkey: LPDWORD,
    ) -> DWORD;
    pub fn SetPrinterDataA(
        hPrinter: HANDLE,
        pValueName: LPSTR,
        Type: DWORD,
        pData: LPBYTE,
        cbData: DWORD,
    ) -> DWORD;
    pub fn SetPrinterDataW(
        hPrinter: HANDLE,
        pValueName: LPWSTR,
        Type: DWORD,
        pData: LPBYTE,
        cbData: DWORD,
    ) -> DWORD;
    pub fn SetPrinterDataExA(
        hPrinter: HANDLE,
        pKeyName: LPCSTR,
        pValueName: LPCSTR,
        Type: DWORD,
        pData: LPBYTE,
        cbData: DWORD,
    ) -> DWORD;
    pub fn SetPrinterDataExW(
        hPrinter: HANDLE,
        pKeyName: LPCWSTR,
        pValueName: LPCWSTR,
        Type: DWORD,
        pData: LPBYTE,
        cbData: DWORD,
    ) -> DWORD;
    pub fn DeletePrinterDataA(
        hPrinter: HANDLE,
        pValueName: LPSTR,
    ) -> DWORD;
    pub fn DeletePrinterDataW(
        hPrinter: HANDLE,
        pValueName: LPWSTR,
    ) -> DWORD;
    pub fn DeletePrinterDataExA(
        hPrinter: HANDLE,
        pKeyName: LPCSTR,
        pValueName: LPCSTR,
    ) -> DWORD;
    pub fn DeletePrinterDataExW(
        hPrinter: HANDLE,
        pKeyName: LPCWSTR,
        pValueName: LPCWSTR,
    ) -> DWORD;
    pub fn DeletePrinterKeyA(
        hPrinter: HANDLE,
        pKeyName: LPCSTR,
    ) -> DWORD;
    pub fn DeletePrinterKeyW(
        hPrinter: HANDLE,
        pKeyName: LPCWSTR,
    ) -> DWORD;
}
pub const PRINTER_NOTIFY_TYPE: DWORD = 0x00;
pub const JOB_NOTIFY_TYPE: DWORD = 0x01;
pub const SERVER_NOTIFY_TYPE: DWORD = 0x02;
pub const PRINTER_NOTIFY_FIELD_SERVER_NAME: DWORD = 0x00;
pub const PRINTER_NOTIFY_FIELD_PRINTER_NAME: DWORD = 0x01;
pub const PRINTER_NOTIFY_FIELD_SHARE_NAME: DWORD = 0x02;
pub const PRINTER_NOTIFY_FIELD_PORT_NAME: DWORD = 0x03;
pub const PRINTER_NOTIFY_FIELD_DRIVER_NAME: DWORD = 0x04;
pub const PRINTER_NOTIFY_FIELD_COMMENT: DWORD = 0x05;
pub const PRINTER_NOTIFY_FIELD_LOCATION: DWORD = 0x06;
pub const PRINTER_NOTIFY_FIELD_DEVMODE: DWORD = 0x07;
pub const PRINTER_NOTIFY_FIELD_SEPFILE: DWORD = 0x08;
pub const PRINTER_NOTIFY_FIELD_PRINT_PROCESSOR: DWORD = 0x09;
pub const PRINTER_NOTIFY_FIELD_PARAMETERS: DWORD = 0x0A;
pub const PRINTER_NOTIFY_FIELD_DATATYPE: DWORD = 0x0B;
pub const PRINTER_NOTIFY_FIELD_SECURITY_DESCRIPTOR: DWORD = 0x0C;
pub const PRINTER_NOTIFY_FIELD_ATTRIBUTES: DWORD = 0x0D;
pub const PRINTER_NOTIFY_FIELD_PRIORITY: DWORD = 0x0E;
pub const PRINTER_NOTIFY_FIELD_DEFAULT_PRIORITY: DWORD = 0x0F;
pub const PRINTER_NOTIFY_FIELD_START_TIME: DWORD = 0x10;
pub const PRINTER_NOTIFY_FIELD_UNTIL_TIME: DWORD = 0x11;
pub const PRINTER_NOTIFY_FIELD_STATUS: DWORD = 0x12;
pub const PRINTER_NOTIFY_FIELD_STATUS_STRING: DWORD = 0x13;
pub const PRINTER_NOTIFY_FIELD_CJOBS: DWORD = 0x14;
pub const PRINTER_NOTIFY_FIELD_AVERAGE_PPM: DWORD = 0x15;
pub const PRINTER_NOTIFY_FIELD_TOTAL_PAGES: DWORD = 0x16;
pub const PRINTER_NOTIFY_FIELD_PAGES_PRINTED: DWORD = 0x17;
pub const PRINTER_NOTIFY_FIELD_TOTAL_BYTES: DWORD = 0x18;
pub const PRINTER_NOTIFY_FIELD_BYTES_PRINTED: DWORD = 0x19;
pub const PRINTER_NOTIFY_FIELD_OBJECT_GUID: DWORD = 0x1A;
pub const PRINTER_NOTIFY_FIELD_FRIENDLY_NAME: DWORD = 0x1B;
pub const PRINTER_NOTIFY_FIELD_BRANCH_OFFICE_PRINTING: DWORD = 0x1C;
pub const JOB_NOTIFY_FIELD_PRINTER_NAME: DWORD = 0x00;
pub const JOB_NOTIFY_FIELD_MACHINE_NAME: DWORD = 0x01;
pub const JOB_NOTIFY_FIELD_PORT_NAME: DWORD = 0x02;
pub const JOB_NOTIFY_FIELD_USER_NAME: DWORD = 0x03;
pub const JOB_NOTIFY_FIELD_NOTIFY_NAME: DWORD = 0x04;
pub const JOB_NOTIFY_FIELD_DATATYPE: DWORD = 0x05;
pub const JOB_NOTIFY_FIELD_PRINT_PROCESSOR: DWORD = 0x06;
pub const JOB_NOTIFY_FIELD_PARAMETERS: DWORD = 0x07;
pub const JOB_NOTIFY_FIELD_DRIVER_NAME: DWORD = 0x08;
pub const JOB_NOTIFY_FIELD_DEVMODE: DWORD = 0x09;
pub const JOB_NOTIFY_FIELD_STATUS: DWORD = 0x0A;
pub const JOB_NOTIFY_FIELD_STATUS_STRING: DWORD = 0x0B;
pub const JOB_NOTIFY_FIELD_SECURITY_DESCRIPTOR: DWORD = 0x0C;
pub const JOB_NOTIFY_FIELD_DOCUMENT: DWORD = 0x0D;
pub const JOB_NOTIFY_FIELD_PRIORITY: DWORD = 0x0E;
pub const JOB_NOTIFY_FIELD_POSITION: DWORD = 0x0F;
pub const JOB_NOTIFY_FIELD_SUBMITTED: DWORD = 0x10;
pub const JOB_NOTIFY_FIELD_START_TIME: DWORD = 0x11;
pub const JOB_NOTIFY_FIELD_UNTIL_TIME: DWORD = 0x12;
pub const JOB_NOTIFY_FIELD_TIME: DWORD = 0x13;
pub const JOB_NOTIFY_FIELD_TOTAL_PAGES: DWORD = 0x14;
pub const JOB_NOTIFY_FIELD_PAGES_PRINTED: DWORD = 0x15;
pub const JOB_NOTIFY_FIELD_TOTAL_BYTES: DWORD = 0x16;
pub const JOB_NOTIFY_FIELD_BYTES_PRINTED: DWORD = 0x17;
pub const JOB_NOTIFY_FIELD_REMOTE_JOB_ID: DWORD = 0x18;
pub const SERVER_NOTIFY_FIELD_PRINT_DRIVER_ISOLATION_GROUP: DWORD = 0x00;
pub const PRINTER_NOTIFY_CATEGORY_ALL: DWORD = 0x001000;
pub const PRINTER_NOTIFY_CATEGORY_3D: DWORD = 0x002000;
STRUCT!{struct PRINTER_NOTIFY_OPTIONS_TYPE {
    Type: WORD,
    Reserved0: WORD,
    Reserved1: DWORD,
    Reserved2: DWORD,
    Count: DWORD,
    pFields: PWORD,
}}
pub type PPRINTER_NOTIFY_OPTIONS_TYPE = *mut PRINTER_NOTIFY_OPTIONS_TYPE;
pub type LPPRINTER_NOTIFY_OPTIONS_TYPE = *mut PRINTER_NOTIFY_OPTIONS_TYPE;
pub const PRINTER_NOTIFY_OPTIONS_REFRESH: DWORD = 0x01;
STRUCT!{struct PRINTER_NOTIFY_OPTIONS {
    Version: DWORD,
    Flags: DWORD,
    Count: DWORD,
    pTypes: PPRINTER_NOTIFY_OPTIONS_TYPE,
}}
pub type PPRINTER_NOTIFY_OPTIONS = *mut PRINTER_NOTIFY_OPTIONS;
pub type LPPRINTER_NOTIFY_OPTIONS = *mut PRINTER_NOTIFY_OPTIONS;
pub const PRINTER_NOTIFY_INFO_DISCARDED: DWORD = 0x01;
STRUCT!{struct PRINTER_NOTIFY_INFO_DATA_NotifyData_Data {
    cbBuf: DWORD,
    pBuf: LPVOID,
}}
UNION!{union PRINTER_NOTIFY_INFO_DATA_NotifyData {
    [usize; 2],
    adwData adwData_mut: [DWORD; 2],
    Data Data_mut: PRINTER_NOTIFY_INFO_DATA_NotifyData_Data,
}}
STRUCT!{struct PRINTER_NOTIFY_INFO_DATA {
    Type: WORD,
    Field: WORD,
    Reserved: DWORD,
    Id: DWORD,
    NotifyData: PRINTER_NOTIFY_INFO_DATA_NotifyData,
}}
pub type PPRINTER_NOTIFY_INFO_DATA = *mut PRINTER_NOTIFY_INFO_DATA;
pub type LPPRINTER_NOTIFY_INFO_DATA = *mut PRINTER_NOTIFY_INFO_DATA;
STRUCT!{struct PRINTER_NOTIFY_INFO {
    Version: DWORD,
    Flags: DWORD,
    Count: DWORD,
    aData: [PRINTER_NOTIFY_INFO_DATA; 1],
}}
pub type PPRINTER_NOTIFY_INFO = *mut PRINTER_NOTIFY_INFO;
pub type LPPRINTER_NOTIFY_INFO = *mut PRINTER_NOTIFY_INFO;
STRUCT!{struct BINARY_CONTAINER {
    cbBuf: DWORD,
    pData: LPBYTE,
}}
pub type PBINARY_CONTAINER = *mut BINARY_CONTAINER;
UNION!{union BIDI_DATA_u {
    [usize; 2],
    bData bData_mut: BOOL,
    iData iData_mut: LONG,
    sData sData_mut: LPWSTR,
    fData fData_mut: FLOAT,
    biData biData_mut: BINARY_CONTAINER,
}}
STRUCT!{struct BIDI_DATA {
    dwBidiType: DWORD,
    u: BIDI_DATA_u,
}}
pub type PBIDI_DATA = *mut BIDI_DATA;
pub type LPBIDI_DATA = *mut BIDI_DATA;
STRUCT!{struct BIDI_REQUEST_DATA {
    dwReqNumber: DWORD,
    pSchema: LPWSTR,
    data: BIDI_DATA,
}}
pub type PBIDI_REQUEST_DATA = *mut BIDI_REQUEST_DATA;
pub type LPBIDI_REQUEST_DATA = *mut BIDI_REQUEST_DATA;
STRUCT!{struct BIDI_REQUEST_CONTAINER {
    Version: DWORD,
    Flags: DWORD,
    Count: DWORD,
    aData: [BIDI_REQUEST_DATA; 1],
}}
pub type PBIDI_REQUEST_CONTAINER = *mut BIDI_REQUEST_CONTAINER;
pub type LPBIDI_REQUEST_CONTAINER = *mut BIDI_REQUEST_CONTAINER;
STRUCT!{struct BIDI_RESPONSE_DATA {
    dwResult: DWORD,
    dwReqNumber: DWORD,
    pSchema: LPWSTR,
    data: BIDI_DATA,
}}
pub type PBIDI_RESPONSE_DATA = *mut BIDI_RESPONSE_DATA;
pub type LPBIDI_RESPONSE_DATA = *mut BIDI_RESPONSE_DATA;
STRUCT!{struct BIDI_RESPONSE_CONTAINER {
    Version: DWORD,
    Flags: DWORD,
    Count: DWORD,
    aData: [BIDI_RESPONSE_DATA; 1],
}}
pub type PBIDI_RESPONSE_CONTAINER = *mut BIDI_RESPONSE_CONTAINER;
pub type LPBIDI_RESPONSE_CONTAINER = *mut BIDI_RESPONSE_CONTAINER;
pub const BIDI_ACTION_ENUM_SCHEMA: &'static str = "EnumSchema";
pub const BIDI_ACTION_GET: &'static str = "Get";
pub const BIDI_ACTION_SET: &'static str = "Set";
pub const BIDI_ACTION_GET_ALL: &'static str = "GetAll";
pub const BIDI_ACTION_GET_WITH_ARGUMENT: &'static str = "GetWithArgument";
ENUM!{enum BIDI_TYPE {
    BIDI_NULL = 0,
    BIDI_INT = 1,
    BIDI_FLOAT = 2,
    BIDI_BOOL = 3,
    BIDI_STRING = 4,
    BIDI_TEXT = 5,
    BIDI_ENUM = 6,
    BIDI_BLOB = 7,
}}
pub const BIDI_ACCESS_ADMINISTRATOR: DWORD = 0x1;
pub const BIDI_ACCESS_USER: DWORD = 0x2;
pub const ERROR_BIDI_STATUS_OK: DWORD = 0;
pub const ERROR_BIDI_NOT_SUPPORTED: DWORD = ERROR_NOT_SUPPORTED;
pub const ERROR_BIDI_ERROR_BASE: DWORD = 13000;
pub const ERROR_BIDI_STATUS_WARNING: DWORD = ERROR_BIDI_ERROR_BASE + 1;
pub const ERROR_BIDI_SCHEMA_READ_ONLY: DWORD = ERROR_BIDI_ERROR_BASE + 2;
pub const ERROR_BIDI_SERVER_OFFLINE: DWORD = ERROR_BIDI_ERROR_BASE + 3;
pub const ERROR_BIDI_DEVICE_OFFLINE: DWORD = ERROR_BIDI_ERROR_BASE + 4;
pub const ERROR_BIDI_SCHEMA_NOT_SUPPORTED: DWORD = ERROR_BIDI_ERROR_BASE + 5;
pub const ERROR_BIDI_SET_DIFFERENT_TYPE: DWORD = ERROR_BIDI_ERROR_BASE + 6;
pub const ERROR_BIDI_SET_MULTIPLE_SCHEMAPATH: DWORD = ERROR_BIDI_ERROR_BASE + 7;
pub const ERROR_BIDI_SET_INVALID_SCHEMAPATH: DWORD = ERROR_BIDI_ERROR_BASE + 8;
pub const ERROR_BIDI_SET_UNKNOWN_FAILURE: DWORD = ERROR_BIDI_ERROR_BASE + 9;
pub const ERROR_BIDI_SCHEMA_WRITE_ONLY: DWORD = ERROR_BIDI_ERROR_BASE + 10;
pub const ERROR_BIDI_GET_REQUIRES_ARGUMENT: DWORD = ERROR_BIDI_ERROR_BASE + 11;
pub const ERROR_BIDI_GET_ARGUMENT_NOT_SUPPORTED: DWORD = ERROR_BIDI_ERROR_BASE + 12;
pub const ERROR_BIDI_GET_MISSING_ARGUMENT: DWORD = ERROR_BIDI_ERROR_BASE + 13;
pub const ERROR_BIDI_DEVICE_CONFIG_UNCHANGED: DWORD = ERROR_BIDI_ERROR_BASE + 14;
pub const ERROR_BIDI_NO_LOCALIZED_RESOURCES: DWORD = ERROR_BIDI_ERROR_BASE + 15;
pub const ERROR_BIDI_NO_BIDI_SCHEMA_EXTENSIONS: DWORD = ERROR_BIDI_ERROR_BASE + 16;
pub const ERROR_BIDI_UNSUPPORTED_CLIENT_LANGUAGE: DWORD = ERROR_BIDI_ERROR_BASE + 17;
pub const ERROR_BIDI_UNSUPPORTED_RESOURCE_FORMAT: DWORD = ERROR_BIDI_ERROR_BASE + 18;
extern "system" {
    pub fn WaitForPrinterChange(
        hPrinter: HANDLE,
        Flags: DWORD,
    ) -> DWORD;
    pub fn FindFirstPrinterChangeNotification(
        hPrinter: HANDLE,
        fdwFilter: DWORD,
        fdwOptions: DWORD,
        pPrinterNotifyOptions: LPVOID,
    ) -> HANDLE;
    pub fn FindNextPrinterChangeNotification(
        hChange: HANDLE,
        pdwChange: PDWORD,
        pPrinterNotifyOptions: LPVOID,
        ppPrinterNotifyInfo: *mut LPVOID,
    ) -> BOOL;
    pub fn FreePrinterNotifyInfo(
        pPrinterNotifyInfo: PPRINTER_NOTIFY_INFO,
    ) -> BOOL;
    pub fn FindClosePrinterChangeNotification(
        hChange: HANDLE,
    ) -> BOOL;
}
pub const PRINTER_CHANGE_ADD_PRINTER: DWORD = 0x00000001;
pub const PRINTER_CHANGE_SET_PRINTER: DWORD = 0x00000002;
pub const PRINTER_CHANGE_DELETE_PRINTER: DWORD = 0x00000004;
pub const PRINTER_CHANGE_FAILED_CONNECTION_PRINTER: DWORD = 0x00000008;
pub const PRINTER_CHANGE_PRINTER: DWORD = 0x000000FF;
pub const PRINTER_CHANGE_ADD_JOB: DWORD = 0x00000100;
pub const PRINTER_CHANGE_SET_JOB: DWORD = 0x00000200;
pub const PRINTER_CHANGE_DELETE_JOB: DWORD = 0x00000400;
pub const PRINTER_CHANGE_WRITE_JOB: DWORD = 0x00000800;
pub const PRINTER_CHANGE_JOB: DWORD = 0x0000FF00;
pub const PRINTER_CHANGE_ADD_FORM: DWORD = 0x00010000;
pub const PRINTER_CHANGE_SET_FORM: DWORD = 0x00020000;
pub const PRINTER_CHANGE_DELETE_FORM: DWORD = 0x00040000;
pub const PRINTER_CHANGE_FORM: DWORD = 0x00070000;
pub const PRINTER_CHANGE_ADD_PORT: DWORD = 0x00100000;
pub const PRINTER_CHANGE_CONFIGURE_PORT: DWORD = 0x00200000;
pub const PRINTER_CHANGE_DELETE_PORT: DWORD = 0x00400000;
pub const PRINTER_CHANGE_PORT: DWORD = 0x00700000;
pub const PRINTER_CHANGE_ADD_PRINT_PROCESSOR: DWORD = 0x01000000;
pub const PRINTER_CHANGE_DELETE_PRINT_PROCESSOR: DWORD = 0x04000000;
pub const PRINTER_CHANGE_PRINT_PROCESSOR: DWORD = 0x07000000;
pub const PRINTER_CHANGE_SERVER: DWORD = 0x08000000;
pub const PRINTER_CHANGE_ADD_PRINTER_DRIVER: DWORD = 0x10000000;
pub const PRINTER_CHANGE_SET_PRINTER_DRIVER: DWORD = 0x20000000;
pub const PRINTER_CHANGE_DELETE_PRINTER_DRIVER: DWORD = 0x40000000;
pub const PRINTER_CHANGE_PRINTER_DRIVER: DWORD = 0x70000000;
pub const PRINTER_CHANGE_TIMEOUT: DWORD = 0x80000000;
pub const PRINTER_CHANGE_ALL: DWORD = 0x7F77FFFF;
extern "system" {
    pub fn PrinterMessageBoxA(
        hPrinter: HANDLE,
        Error: DWORD,
        hWnd: HWND,
        pText: LPSTR,
        pCaption: LPSTR,
        dwType: DWORD,
    ) -> DWORD;
    pub fn PrinterMessageBoxW(
        hPrinter: HANDLE,
        Error: DWORD,
        hWnd: HWND,
        pText: LPWSTR,
        pCaption: LPWSTR,
        dwType: DWORD,
    ) -> DWORD;
}
pub const PRINTER_ERROR_INFORMATION: DWORD = 0x80000000;
pub const PRINTER_ERROR_WARNING: DWORD = 0x40000000;
pub const PRINTER_ERROR_SEVERE: DWORD = 0x20000000;
pub const PRINTER_ERROR_OUTOFPAPER: DWORD = 0x00000001;
pub const PRINTER_ERROR_JAM: DWORD = 0x00000002;
pub const PRINTER_ERROR_OUTOFTONER: DWORD = 0x00000004;
extern "system" {
    pub fn ClosePrinter(
        hPrinter: HANDLE,
    ) -> BOOL;
    pub fn AddFormA(
        hPrinter: HANDLE,
        Level: DWORD,
        pForm: LPBYTE,
    ) -> BOOL;
    pub fn AddFormW(
        hPrinter: HANDLE,
        Level: DWORD,
        pForm: LPBYTE,
    ) -> BOOL;
    pub fn DeleteFormA(
        hPrinter: HANDLE,
        pFormName: LPSTR,
    ) -> BOOL;
    pub fn DeleteFormW(
        hPrinter: HANDLE,
        pFormName: LPWSTR,
    ) -> BOOL;
    pub fn GetFormA(
        hPrinter: HANDLE,
        pFormName: LPSTR,
        Level: DWORD,
        pForm: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetFormW(
        hPrinter: HANDLE,
        pFormName: LPWSTR,
        Level: DWORD,
        pForm: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn SetFormA(
        hPrinter: HANDLE,
        pFormName: LPSTR,
        Level: DWORD,
        pForm: LPBYTE,
    ) -> BOOL;
    pub fn SetFormW(
        hPrinter: HANDLE,
        pFormName: LPWSTR,
        Level: DWORD,
        pForm: LPBYTE,
    ) -> BOOL;
    pub fn EnumFormsA(
        hPrinter: HANDLE,
        Level: DWORD,
        pForm: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumFormsW(
        hPrinter: HANDLE,
        Level: DWORD,
        pForm: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumMonitorsA(
        pName: LPSTR,
        Level: DWORD,
        pMonitor: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumMonitorsW(
        pName: LPWSTR,
        Level: DWORD,
        pMonitor: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn AddMonitorA(
        pName: LPSTR,
        Level: DWORD,
        pMonitors: LPBYTE,
    ) -> BOOL;
    pub fn AddMonitorW(
        pName: LPWSTR,
        Level: DWORD,
        pMonitors: LPBYTE,
    ) -> BOOL;
    pub fn DeleteMonitorA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        pMonitorName: LPSTR,
    ) -> BOOL;
    pub fn DeleteMonitorW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        pMonitorName: LPWSTR,
    ) -> BOOL;
    pub fn EnumPortsA(
        pName: LPSTR,
        Level: DWORD,
        pPort: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumPortsW(
        pName: LPWSTR,
        Level: DWORD,
        pPort: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
        pcReturned: LPDWORD,
    ) -> BOOL;
    pub fn AddPortA(
        pName: LPSTR,
        hWnd: HWND,
        pMonitorName: LPSTR,
    ) -> BOOL;
    pub fn AddPortW(
        pName: LPWSTR,
        hWnd: HWND,
        pMonitorName: LPWSTR,
    ) -> BOOL;
    pub fn ConfigurePortA(
        pName: LPSTR,
        hWnd: HWND,
        pPortName: LPSTR,
    ) -> BOOL;
    pub fn ConfigurePortW(
        pName: LPWSTR,
        hWnd: HWND,
        pPortName: LPWSTR,
    ) -> BOOL;
    pub fn DeletePortA(
        pName: LPSTR,
        hWnd: HWND,
        pPortName: LPSTR,
    ) -> BOOL;
    pub fn DeletePortW(
        pName: LPWSTR,
        hWnd: HWND,
        pPortName: LPWSTR,
    ) -> BOOL;
    pub fn XcvDataW(
        hXcv: HANDLE,
        pszDataName: PCWSTR,
        pInputData: PBYTE,
        cbInputData: DWORD,
        pOutputData: PBYTE,
        cbOutputData: DWORD,
        pcbOutputNeeded: PDWORD,
        pdwStatus: PDWORD,
    ) -> BOOL;
    pub fn GetDefaultPrinterA(
        pszBuffer: LPSTR,
        pcchBuffer: LPDWORD,
    ) -> BOOL;
    pub fn GetDefaultPrinterW(
        pszBuffer: LPWSTR,
        pcchBuffer: LPDWORD,
    ) -> BOOL;
    pub fn SetDefaultPrinterA(
        pszPrinter: LPCSTR,
    ) -> BOOL;
    pub fn SetDefaultPrinterW(
        pszPrinter: LPCWSTR,
    ) -> BOOL;
    pub fn SetPortA(
        pName: LPSTR,
        pPortName: LPSTR,
        dwLevel: DWORD,
        pPortInfo: LPBYTE,
    ) -> BOOL;
    pub fn SetPortW(pName: LPWSTR,
        pPortName: LPWSTR,
        dwLevel: DWORD,
        pPortInfo: LPBYTE,
    ) -> BOOL;
    pub fn AddPrinterConnectionA(
        pName: LPSTR,
    ) -> BOOL;
    pub fn AddPrinterConnectionW(
        pName: LPWSTR,
    ) -> BOOL;
    pub fn DeletePrinterConnectionA(
        pName: LPSTR,
    ) -> BOOL;
    pub fn DeletePrinterConnectionW(
        pName: LPWSTR,
    ) -> BOOL;
    pub fn ConnectToPrinterDlg(
        hwnd: HWND,
        Flags: DWORD,
    ) -> HANDLE;
}
STRUCT!{struct PROVIDOR_INFO_1A {
    pName: LPSTR,
    pEnvironment: LPSTR,
    pDLLName: LPSTR,
}}
pub type PPROVIDOR_INFO_1A = *mut PROVIDOR_INFO_1A;
pub type LPPROVIDOR_INFO_1A = *mut PROVIDOR_INFO_1A;
STRUCT!{struct PROVIDOR_INFO_1W {
    pName: LPWSTR,
    pEnvironment: LPWSTR,
    pDLLName: LPWSTR,
}}
pub type PPROVIDOR_INFO_1W = *mut PROVIDOR_INFO_1W;
pub type LPPROVIDOR_INFO_1W = *mut PROVIDOR_INFO_1W;
STRUCT!{struct PROVIDOR_INFO_2A {
    pOrder: LPSTR,
}}
pub type PPROVIDOR_INFO_2A = *mut PROVIDOR_INFO_2A;
pub type LPPROVIDOR_INFO_2A = *mut PROVIDOR_INFO_2A;
STRUCT!{struct PROVIDOR_INFO_2W {
    pOrder: LPWSTR,
}}
pub type PPROVIDOR_INFO_2W = *mut PROVIDOR_INFO_2W;
pub type LPPROVIDOR_INFO_2W = *mut PROVIDOR_INFO_2W;
extern "system" {
    pub fn AddPrintProvidorA(
        pName: LPSTR,
        Level: DWORD,
        pProvidorInfo: LPBYTE,
    ) -> BOOL;
    pub fn AddPrintProvidorW(
        pName: LPWSTR,
        Level: DWORD,
        pProvidorInfo: LPBYTE,
    ) -> BOOL;
    pub fn DeletePrintProvidorA(
        pName: LPSTR,
        pEnvironment: LPSTR,
        pPrintProvidorName: LPSTR,
    ) -> BOOL;
    pub fn DeletePrintProvidorW(
        pName: LPWSTR,
        pEnvironment: LPWSTR,
        pPrintProvidorName: LPWSTR,
    ) -> BOOL;
    pub fn IsValidDevmodeA(
        pDevmode: PDEVMODEA,
        DevmodeSize: size_t,
    ) -> BOOL;
    pub fn IsValidDevmodeW(
        pDevmode: PDEVMODEW,
        DevmodeSize: size_t,
    ) -> BOOL;
}
pub const SPLREG_DEFAULT_SPOOL_DIRECTORY: &'static str = "DefaultSpoolDirectory";
pub const SPLREG_PORT_THREAD_PRIORITY_DEFAULT: &'static str = "PortThreadPriorityDefault";
pub const SPLREG_PORT_THREAD_PRIORITY: &'static str = "PortThreadPriority";
pub const SPLREG_SCHEDULER_THREAD_PRIORITY_DEFAULT: &'static str
    = "SchedulerThreadPriorityDefault";
pub const SPLREG_SCHEDULER_THREAD_PRIORITY: &'static str = "SchedulerThreadPriority";
pub const SPLREG_BEEP_ENABLED: &'static str = "BeepEnabled";
pub const SPLREG_NET_POPUP: &'static str = "NetPopup";
pub const SPLREG_RETRY_POPUP: &'static str = "RetryPopup";
pub const SPLREG_NET_POPUP_TO_COMPUTER: &'static str = "NetPopupToComputer";
pub const SPLREG_EVENT_LOG: &'static str = "EventLog";
pub const SPLREG_MAJOR_VERSION: &'static str = "MajorVersion";
pub const SPLREG_MINOR_VERSION: &'static str = "MinorVersion";
pub const SPLREG_ARCHITECTURE: &'static str = "Architecture";
pub const SPLREG_OS_VERSION: &'static str = "OSVersion";
pub const SPLREG_OS_VERSIONEX: &'static str = "OSVersionEx";
pub const SPLREG_DS_PRESENT: &'static str = "DsPresent";
pub const SPLREG_DS_PRESENT_FOR_USER: &'static str = "DsPresentForUser";
pub const SPLREG_REMOTE_FAX: &'static str = "RemoteFax";
pub const SPLREG_RESTART_JOB_ON_POOL_ERROR: &'static str = "RestartJobOnPoolError";
pub const SPLREG_RESTART_JOB_ON_POOL_ENABLED: &'static str = "RestartJobOnPoolEnabled";
pub const SPLREG_DNS_MACHINE_NAME: &'static str = "DNSMachineName";
pub const SPLREG_ALLOW_USER_MANAGEFORMS: &'static str = "AllowUserManageForms";
pub const SPLREG_WEBSHAREMGMT: &'static str = "WebShareMgmt";
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS_SEPARATOR: &'static str = "\\";
pub const SPLREG_PRINT_DRIVER_ISOLATION_GROUPS: &'static str = "PrintDriverIsolationGroups";
pub const SPLREG_PRINT_DRIVER_ISOLATION_TIME_BEFORE_RECYCLE: &'static str
    = "PrintDriverIsolationTimeBeforeRecycle";
pub const SPLREG_PRINT_DRIVER_ISOLATION_MAX_OBJECTS_BEFORE_RECYCLE: &'static str
    = "PrintDriverIsolationMaxobjsBeforeRecycle";
pub const SPLREG_PRINT_DRIVER_ISOLATION_IDLE_TIMEOUT: &'static str
    = "PrintDriverIsolationIdleTimeout";
pub const SPLREG_PRINT_DRIVER_ISOLATION_EXECUTION_POLICY: &'static str
    = "PrintDriverIsolationExecutionPolicy";
pub const SPLREG_PRINT_DRIVER_ISOLATION_OVERRIDE_POLICY: &'static str
    = "PrintDriverIsolationOverrideCompat";
pub const SPLREG_PRINT_QUEUE_V4_DRIVER_DIRECTORY: &'static str = "PrintQueueV4DriverDirectory";
pub const SERVER_ACCESS_ADMINISTER: DWORD = 0x00000001;
pub const SERVER_ACCESS_ENUMERATE: DWORD = 0x00000002;
pub const PRINTER_ACCESS_ADMINISTER: DWORD = 0x00000004;
pub const PRINTER_ACCESS_USE: DWORD = 0x00000008;
pub const JOB_ACCESS_ADMINISTER: DWORD = 0x00000010;
pub const JOB_ACCESS_READ: DWORD = 0x00000020;
pub const PRINTER_ACCESS_MANAGE_LIMITED: DWORD = 0x00000040;
pub const SERVER_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | SERVER_ACCESS_ADMINISTER
    | SERVER_ACCESS_ENUMERATE;
pub const SERVER_READ: DWORD = STANDARD_RIGHTS_READ | SERVER_ACCESS_ENUMERATE;
pub const SERVER_WRITE: DWORD = STANDARD_RIGHTS_WRITE | SERVER_ACCESS_ADMINISTER
    | SERVER_ACCESS_ENUMERATE;
pub const SERVER_EXECUTE: DWORD = STANDARD_RIGHTS_EXECUTE | SERVER_ACCESS_ENUMERATE;
pub const PRINTER_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | PRINTER_ACCESS_ADMINISTER
    | PRINTER_ACCESS_USE;
pub const PRINTER_READ: DWORD = STANDARD_RIGHTS_READ | PRINTER_ACCESS_USE;
pub const PRINTER_WRITE: DWORD = STANDARD_RIGHTS_WRITE | PRINTER_ACCESS_USE;
pub const PRINTER_EXECUTE: DWORD = STANDARD_RIGHTS_EXECUTE | PRINTER_ACCESS_USE;
pub const JOB_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | JOB_ACCESS_ADMINISTER
    | JOB_ACCESS_READ;
pub const JOB_READ: DWORD = STANDARD_RIGHTS_READ | JOB_ACCESS_READ;
pub const JOB_WRITE: DWORD = STANDARD_RIGHTS_WRITE | JOB_ACCESS_ADMINISTER;
pub const JOB_EXECUTE: DWORD = STANDARD_RIGHTS_EXECUTE | JOB_ACCESS_ADMINISTER;
pub const SPLDS_SPOOLER_KEY: &'static str = "DsSpooler";
pub const SPLDS_DRIVER_KEY: &'static str = "DsDriver";
pub const SPLDS_USER_KEY: &'static str = "DsUser";
pub const SPLDS_ASSET_NUMBER: &'static str = "assetNumber";
pub const SPLDS_BYTES_PER_MINUTE: &'static str = "bytesPerMinute";
pub const SPLDS_DESCRIPTION: &'static str = "description";
pub const SPLDS_DRIVER_NAME: &'static str = "driverName";
pub const SPLDS_DRIVER_VERSION: &'static str = "driverVersion";
pub const SPLDS_LOCATION: &'static str = "location";
pub const SPLDS_PORT_NAME: &'static str = "portName";
pub const SPLDS_PRINT_ATTRIBUTES: &'static str = "printAttributes";
pub const SPLDS_PRINT_BIN_NAMES: &'static str = "printBinNames";
pub const SPLDS_PRINT_COLLATE: &'static str = "printCollate";
pub const SPLDS_PRINT_COLOR: &'static str = "printColor";
pub const SPLDS_PRINT_DUPLEX_SUPPORTED: &'static str = "printDuplexSupported";
pub const SPLDS_PRINT_END_TIME: &'static str = "printEndTime";
pub const SPLDS_PRINTER_CLASS: &'static str = "printQueue";
pub const SPLDS_PRINTER_NAME: &'static str = "printerName";
pub const SPLDS_PRINT_KEEP_PRINTED_JOBS: &'static str = "printKeepPrintedJobs";
pub const SPLDS_PRINT_LANGUAGE: &'static str = "printLanguage";
pub const SPLDS_PRINT_MAC_ADDRESS: &'static str = "printMACAddress";
pub const SPLDS_PRINT_MAX_X_EXTENT: &'static str = "printMaxXExtent";
pub const SPLDS_PRINT_MAX_Y_EXTENT: &'static str = "printMaxYExtent";
pub const SPLDS_PRINT_MAX_RESOLUTION_SUPPORTED: &'static str = "printMaxResolutionSupported";
pub const SPLDS_PRINT_MEDIA_READY: &'static str = "printMediaReady";
pub const SPLDS_PRINT_MEDIA_SUPPORTED: &'static str = "printMediaSupported";
pub const SPLDS_PRINT_MEMORY: &'static str = "printMemory";
pub const SPLDS_PRINT_MIN_X_EXTENT: &'static str = "printMinXExtent";
pub const SPLDS_PRINT_MIN_Y_EXTENT: &'static str = "printMinYExtent";
pub const SPLDS_PRINT_NETWORK_ADDRESS: &'static str = "printNetworkAddress";
pub const SPLDS_PRINT_NOTIFY: &'static str = "printNotify";
pub const SPLDS_PRINT_NUMBER_UP: &'static str = "printNumberUp";
pub const SPLDS_PRINT_ORIENTATIONS_SUPPORTED: &'static str = "printOrientationsSupported";
pub const SPLDS_PRINT_OWNER: &'static str = "printOwner";
pub const SPLDS_PRINT_PAGES_PER_MINUTE: &'static str = "printPagesPerMinute";
pub const SPLDS_PRINT_RATE: &'static str = "printRate";
pub const SPLDS_PRINT_RATE_UNIT: &'static str = "printRateUnit";
pub const SPLDS_PRINT_SEPARATOR_FILE: &'static str = "printSeparatorFile";
pub const SPLDS_PRINT_SHARE_NAME: &'static str = "printShareName";
pub const SPLDS_PRINT_SPOOLING: &'static str = "printSpooling";
pub const SPLDS_PRINT_STAPLING_SUPPORTED: &'static str = "printStaplingSupported";
pub const SPLDS_PRINT_START_TIME: &'static str = "printStartTime";
pub const SPLDS_PRINT_STATUS: &'static str = "printStatus";
pub const SPLDS_PRIORITY: &'static str = "priority";
pub const SPLDS_SERVER_NAME: &'static str = "serverName";
pub const SPLDS_SHORT_SERVER_NAME: &'static str = "shortServerName";
pub const SPLDS_UNC_NAME: &'static str = "uNCName";
pub const SPLDS_URL: &'static str = "url";
pub const SPLDS_FLAGS: &'static str = "flags";
pub const SPLDS_VERSION_NUMBER: &'static str = "versionNumber";
pub const SPLDS_PRINTER_NAME_ALIASES: &'static str = "printerNameAliases";
pub const SPLDS_PRINTER_LOCATIONS: &'static str = "printerLocations";
pub const SPLDS_PRINTER_MODEL: &'static str = "printerModel";
ENUM!{enum PRINTER_OPTION_FLAGS {
    PRINTER_OPTION_NO_CACHE = 1 << 0,
    PRINTER_OPTION_CACHE = 1 << 1,
    PRINTER_OPTION_CLIENT_CHANGE = 1 << 2,
    PRINTER_OPTION_NO_CLIENT_DATA = 1 << 3,
}}
STRUCT!{struct PRINTER_OPTIONSA {
    cbSize: UINT,
    dwFlags: DWORD,
}}
pub type PPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
pub type LPPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
STRUCT!{struct PRINTER_OPTIONSW {
    cbSize: UINT,
    dwFlags: DWORD,
}}
pub type PPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
pub type LPPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
extern "system" {
    pub fn OpenPrinter2A(
        pPrinterName: LPCSTR,
        phPrinter: LPHANDLE,
        pDefault: PPRINTER_DEFAULTSA,
        pOptions: PPRINTER_OPTIONSA,
    ) -> BOOL;
    pub fn OpenPrinter2W(
        pPrinterName: LPCWSTR,
        phPrinter: LPHANDLE,
        pDefault: PPRINTER_DEFAULTSW,
        pOptions: PPRINTER_OPTIONSW,
    ) -> BOOL;
}
pub const PRINTER_CONNECTION_MISMATCH: DWORD = 0x00000020;
pub const PRINTER_CONNECTION_NO_UI: DWORD = 0x00000040;
STRUCT!{struct PRINTER_CONNECTION_INFO_1A {
    dwFlags: DWORD,
    pszDriverName: LPSTR,
}}
pub type PPRINTER_CONNECTION_INFO_1A = *mut PRINTER_CONNECTION_INFO_1A;
pub type LPPRINTER_CONNECTION_INFO_1A = *mut PRINTER_CONNECTION_INFO_1A;
STRUCT!{struct PRINTER_CONNECTION_INFO_1W {
    dwFlags: DWORD,
    pszDriverName: LPWSTR,
}}
pub type PPRINTER_CONNECTION_INFO_1W = *mut PRINTER_CONNECTION_INFO_1W;
pub type LPPRINTER_CONNECTION_INFO_1W = *mut PRINTER_CONNECTION_INFO_1W;
extern "system" {
    pub fn AddPrinterConnection2A(
        hWnd: HWND,
        pszName: LPCSTR,
        dwLevel: DWORD,
        pConnectionInfo: PVOID,
    ) -> BOOL;
    pub fn AddPrinterConnection2W(
        hWnd: HWND,
        pszName: LPCWSTR,
        dwLevel: DWORD,
        pConnectionInfo: PVOID,
    ) -> BOOL;
}
pub const IPDFP_COPY_ALL_FILES: DWORD = 0x00000001;
extern "system" {
    pub fn InstallPrinterDriverFromPackageA(
        pszServer: LPCSTR,
        pszInfPath: LPCSTR,
        pszDriverName: LPCSTR,
        pszEnvironment: LPCSTR,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn InstallPrinterDriverFromPackageW(
        pszServer: LPCWSTR,
        pszInfPath: LPCWSTR,
        pszDriverName: LPCWSTR,
        pszEnvironment: LPCWSTR,
        dwFlags: DWORD,
    ) -> HRESULT;
}
pub const UPDP_SILENT_UPLOAD: DWORD = 0x00000001;
pub const UPDP_UPLOAD_ALWAYS: DWORD = 0x00000002;
pub const UPDP_CHECK_DRIVERSTORE: DWORD = 0x00000004;
extern "system" {
    pub fn UploadPrinterDriverPackageA(
        pszServer: LPCSTR,
        pszInfPath: LPCSTR,
        pszEnvironment: LPCSTR,
        dwFlags: DWORD,
        hwnd: HWND,
        pszDestInfPath: LPSTR,
        pcchDestInfPath: PULONG,
    ) -> HRESULT;
    pub fn UploadPrinterDriverPackageW(
        pszServer: LPCWSTR,
        pszInfPath: LPCWSTR,
        pszEnvironment: LPCWSTR,
        dwFlags: DWORD,
        hwnd: HWND,
        pszDestInfPath: LPWSTR,
        pcchDestInfPath: PULONG,
    ) -> HRESULT;
}
STRUCT!{struct CORE_PRINTER_DRIVERA {
    CoreDriverGUID: GUID,
    ftDriverDate: FILETIME,
    dwlDriverVersion: DWORDLONG,
    szPackageID: [CHAR; MAX_PATH],
}}
pub type PCORE_PRINTER_DRIVERA = *mut CORE_PRINTER_DRIVERA;
STRUCT!{struct CORE_PRINTER_DRIVERW {
    CoreDriverGUID: GUID,
    ftDriverDate: FILETIME,
    dwlDriverVersion: DWORDLONG,
    szPackageID: [WCHAR; MAX_PATH],
}}
pub type PCORE_PRINTER_DRIVERW = *mut CORE_PRINTER_DRIVERW;
extern "system" {
    pub fn GetCorePrinterDriversA(
        pszServer: LPCSTR,
        pszEnvironment: LPCSTR,
        pszzCoreDriverDependencies: LPCSTR,
        cCorePrinterDrivers: DWORD,
        pCorePrinterDrivers: PCORE_PRINTER_DRIVERA,
    ) -> HRESULT;
    pub fn GetCorePrinterDriversW(
        pszServer: LPCWSTR,
        pszEnvironment: LPCWSTR,
        pszzCoreDriverDependencies: LPCWSTR,
        cCorePrinterDrivers: DWORD,
        pCorePrinterDrivers: PCORE_PRINTER_DRIVERW,
    ) -> HRESULT;
    pub fn CorePrinterDriverInstalledA(
        pszServer: LPCSTR,
        pszEnvironment: LPCSTR,
        CoreDriverGUID: GUID,
        ftDriverDate: FILETIME,
        dwlDriverVersion: DWORDLONG,
        pbDriverInstalled: *mut BOOL,
    ) -> HRESULT;
    pub fn CorePrinterDriverInstalledW(
        pszServer: LPCWSTR,
        pszEnvironment: LPCWSTR,
        CoreDriverGUID: GUID,
        ftDriverDate: FILETIME,
        dwlDriverVersion: DWORDLONG,
        pbDriverInstalled: *mut BOOL,
    ) -> HRESULT;
    pub fn GetPrinterDriverPackagePathA(
        pszServer: LPCSTR,
        pszEnvironment: LPCSTR,
        pszLanguage: LPCSTR,
        pszPackageID: LPCSTR,
        pszDriverPackageCab: LPSTR,
        cchDriverPackageCab: DWORD,
        pcchRequiredSize: LPDWORD,
    ) -> HRESULT;
    pub fn GetPrinterDriverPackagePathW(
        pszServer: LPCWSTR,
        pszEnvironment: LPCWSTR,
        pszLanguage: LPCWSTR,
        pszPackageID: LPCWSTR,
        pszDriverPackageCab: LPWSTR,
        cchDriverPackageCab: DWORD,
        pcchRequiredSize: LPDWORD,
    ) -> HRESULT;
    pub fn DeletePrinterDriverPackageA(
        pszServer: LPCSTR,
        pszInfPath: LPCSTR,
        pszEnvironment: LPCSTR,
    ) -> HRESULT;
    pub fn DeletePrinterDriverPackageW(
        pszServer: LPCWSTR,
        pszInfPath: LPCWSTR,
        pszEnvironment: LPCWSTR,
    ) -> HRESULT;
}
ENUM!{enum EPrintPropertyType {
    kPropertyTypeString = 1,
    kPropertyTypeInt32,
    kPropertyTypeInt64,
    kPropertyTypeByte,
    kPropertyTypeTime,
    kPropertyTypeDevMode,
    kPropertyTypeSD,
    kPropertyTypeNotificationReply,
    kPropertyTypeNotificationOptions,
    kPropertyTypeBuffer,
}}
ENUM!{enum EPrintXPSJobProgress {
    kAddingDocumentSequence = 0,
    kDocumentSequenceAdded = 1,
    kAddingFixedDocument = 2,
    kFixedDocumentAdded = 3,
    kAddingFixedPage = 4,
    kFixedPageAdded = 5,
    kResourceAdded = 6,
    kFontAdded = 7,
    kImageAdded = 8,
    kXpsDocumentCommitted = 9,
}}
ENUM!{enum EPrintXPSJobOperation {
    kJobProduction = 1,
    kJobConsumption,
}}
STRUCT!{struct PrintPropertyValue_value_propertyBlob {
    cbBuf: DWORD,
    pBuf: LPVOID,
}}
UNION!{union PrintPropertyValue_value {
    [u64; 1] [u64; 2],
    propertyByte propertyByte_mut: BYTE,
    propertyString propertyString_mut: PWSTR,
    propertyInt32 propertyInt32_mut: LONG,
    propertyInt64 propertyInt64_mut: LONGLONG,
    propertyBlob propertyBlob_mut: PrintPropertyValue_value_propertyBlob,
}}
STRUCT!{struct PrintPropertyValue {
    ePropertyType: EPrintPropertyType,
    value: PrintPropertyValue_value,
}}
STRUCT!{struct PrintNamedProperty {
    propertyName: *mut WCHAR,
    propertyValue: PrintPropertyValue,
}}
STRUCT!{struct PrintPropertiesCollection {
    numberOfProperties: ULONG,
    propertiesCollection: *mut PrintNamedProperty,
}}
extern "system" {
    pub fn ReportJobProcessingProgress(
        printerHandle: HANDLE,
        jobId: ULONG,
        jobOperation: EPrintXPSJobOperation,
        jobProgress: EPrintXPSJobProgress,
    ) -> HRESULT;
    pub fn GetPrinterDriver2A(
        hWnd: HWND,
        hPrinter: HANDLE,
        pEnvironment: LPSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetPrinterDriver2W(
        hWnd: HWND,
        hPrinter: HANDLE,
        pEnvironment: LPWSTR,
        Level: DWORD,
        pDriverInfo: LPBYTE,
        cbBuf: DWORD,
        pcbNeeded: LPDWORD,
    ) -> BOOL;
}
ENUM!{enum PRINT_EXECUTION_CONTEXT {
    PRINT_EXECUTION_CONTEXT_APPLICATION = 0,
    PRINT_EXECUTION_CONTEXT_SPOOLER_SERVICE = 1,
    PRINT_EXECUTION_CONTEXT_SPOOLER_ISOLATION_HOST = 2,
    PRINT_EXECUTION_CONTEXT_FILTER_PIPELINE = 3,
    PRINT_EXECUTION_CONTEXT_WOW64 = 4,
}}
STRUCT!{struct PRINT_EXECUTION_DATA {
    context: PRINT_EXECUTION_CONTEXT,
    clientAppPID: DWORD,
}}
extern "system" {
    pub fn GetPrintExecutionData(
        pData: *mut PRINT_EXECUTION_DATA,
    ) -> BOOL;
    pub fn GetJobNamedPropertyValue(
        hPrinter: HANDLE,
        JobId: DWORD,
        pszName: PCWSTR,
        pValue: *mut PrintPropertyValue,
    ) -> DWORD;
    pub fn FreePrintPropertyValue(
        pValue: *mut PrintPropertyValue,
    );
    pub fn FreePrintNamedPropertyArray(
        cProperties: DWORD,
        ppProperties: *mut *mut PrintNamedProperty,
    );
    pub fn SetJobNamedProperty(
        hPrinter: HANDLE,
        JobId: DWORD,
        pProperty: *const PrintNamedProperty,
    ) -> DWORD;
    pub fn DeleteJobNamedProperty(
        hPrinter: HANDLE,
        JobId: DWORD,
        pszName: PCWSTR,
    ) -> DWORD;
    pub fn EnumJobNamedProperties(
        hPrinter: HANDLE,
        JobId: DWORD,
        pcProperties: *mut DWORD,
        ppProperties: *mut *mut PrintNamedProperty,
    ) -> DWORD;
    pub fn GetPrintOutputInfo(
        hWnd: HWND,
        pszPrinter: PCWSTR,
        phFile: *mut HANDLE,
        ppszOutputFile: *mut PWSTR,
    ) -> HRESULT;
}
pub const MS_PRINT_JOB_OUTPUT_FILE: &'static str = "MsPrintJobOutputFile";
