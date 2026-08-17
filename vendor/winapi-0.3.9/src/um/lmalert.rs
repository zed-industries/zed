// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains structures for communication with the Alerter service
use shared::lmcons::{EVLEN, NET_API_STATUS, SNLEN};
use shared::minwindef::{DWORD, LPVOID};
use um::winnt::{LPCWSTR, WCHAR};
extern "system" {
    pub fn NetAlertRaise(
        AlertType: LPCWSTR,
        Buffer: LPVOID,
        BufferSize: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetAlertRaiseEx(
        AlertType: LPCWSTR,
        VariableInfo: LPVOID,
        VariableInfoSize: DWORD,
        ServiceName: LPCWSTR,
    ) -> NET_API_STATUS;
}
STRUCT!{struct STD_ALERT {
    alrt_timestamp: DWORD,
    alrt_eventname: [WCHAR; EVLEN + 1],
    alrt_servicename: [WCHAR; SNLEN + 1],
}}
pub type PSTD_ALERT = *mut STD_ALERT;
pub type LPSTD_ALERT = *mut STD_ALERT;
STRUCT!{struct ADMIN_OTHER_INFO {
    alrtad_errcode: DWORD,
    alrtad_numstrings: DWORD,
}}
pub type PADMIN_OTHER_INFO = *mut ADMIN_OTHER_INFO;
pub type LPADMIN_OTHER_INFO = *mut ADMIN_OTHER_INFO;
STRUCT!{struct ERRLOG_OTHER_INFO {
    alrter_errcode: DWORD,
    alrter_offset: DWORD,
}}
pub type PERRLOG_OTHER_INFO = *mut ERRLOG_OTHER_INFO;
pub type LPERRLOG_OTHER_INFO = *mut ERRLOG_OTHER_INFO;
STRUCT!{struct PRINT_OTHER_INFO {
    alrtpr_jobid: DWORD,
    alrtpr_status: DWORD,
    alrtpr_submitted: DWORD,
    alrtpr_size: DWORD,
}}
pub type PPRINT_OTHER_INFO = *mut PRINT_OTHER_INFO;
pub type LPPRINT_OTHER_INFO = *mut PRINT_OTHER_INFO;
STRUCT!{struct USER_OTHER_INFO {
    alrtus_errcode: DWORD,
    alrtus_numstrings: DWORD,
}}
pub type PUSER_OTHER_INFO = *mut USER_OTHER_INFO;
pub type LPUSER_OTHER_INFO = *mut USER_OTHER_INFO;
pub const ALERTER_MAILSLOT: &'static str = "\\\\.\\MAILSLOT\\Alerter";
pub const ALERT_PRINT_EVENT: &'static str = "PRINTING";
pub const ALERT_MESSAGE_EVENT: &'static str = "MESSAGE";
pub const ALERT_ERRORLOG_EVENT: &'static str = "ERRORLOG";
pub const ALERT_ADMIN_EVENT: &'static str = "ADMIN";
pub const ALERT_USER_EVENT: &'static str = "USER";
pub const PRJOB_QSTATUS: DWORD = 0x3;
pub const PRJOB_DEVSTATUS: DWORD = 0x1fc;
pub const PRJOB_COMPLETE: DWORD = 0x4;
pub const PRJOB_INTERV: DWORD = 0x8;
pub const PRJOB_ERROR: DWORD = 0x10;
pub const PRJOB_DESTOFFLINE: DWORD = 0x20;
pub const PRJOB_DESTPAUSED: DWORD = 0x40;
pub const PRJOB_NOTIFY: DWORD = 0x80;
pub const PRJOB_DESTNOPAPER: DWORD = 0x100;
pub const PRJOB_DELETED: DWORD = 0x8000;
pub const PRJOB_QS_QUEUED: DWORD = 0;
pub const PRJOB_QS_PAUSED: DWORD = 1;
pub const PRJOB_QS_SPOOLING: DWORD = 2;
pub const PRJOB_QS_PRINTING: DWORD = 3;
