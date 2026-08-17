// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::UINT;
use shared::windef::HMONITOR;
use um::winnt::{HANDLE, HRESULT};
ENUM!{enum PROCESS_DPI_AWARENESS {
    PROCESS_DPI_UNAWARE = 0,
    PROCESS_SYSTEM_DPI_AWARE = 1,
    PROCESS_PER_MONITOR_DPI_AWARE = 2,
}}
ENUM!{enum MONITOR_DPI_TYPE {
    MDT_EFFECTIVE_DPI = 0,
    MDT_ANGULAR_DPI = 1,
    MDT_RAW_DPI = 2,
    MDT_DEFAULT = MDT_EFFECTIVE_DPI,
}}
extern "system" {
    pub fn SetProcessDpiAwareness(
        value: PROCESS_DPI_AWARENESS,
    ) -> HRESULT;
    pub fn GetProcessDpiAwareness(
        hProcess: HANDLE,
        value: *mut PROCESS_DPI_AWARENESS,
    ) -> HRESULT;
    pub fn GetDpiForMonitor(
        hmonitor: HMONITOR,
        dpiType: MONITOR_DPI_TYPE,
        dpiX: *mut UINT,
        dpiY: *mut UINT,
    ) -> HRESULT;
}
ENUM!{enum SHELL_UI_COMPONENT {
    SHELL_UI_COMPONENT_TASKBARS = 0,
    SHELL_UI_COMPONENT_NOTIFICATIONAREA = 1,
    SHELL_UI_COMPONENT_DESKBAND = 2,
}}
extern "system" {
    pub fn GetDpiForShellUIComponent(
        component: SHELL_UI_COMPONENT,
    ) -> UINT;
}
