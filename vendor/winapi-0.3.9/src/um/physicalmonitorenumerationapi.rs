// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::d3d9::IDirect3DDevice9;
use shared::minwindef::{BOOL, DWORD, LPDWORD};
use shared::windef::HMONITOR;
use um::winnt::{HANDLE, HRESULT, WCHAR};
pub type _BOOL = BOOL;
pub const PHYSICAL_MONITOR_DESCRIPTION_SIZE: usize = 128;
STRUCT!{#[repr(packed)] struct PHYSICAL_MONITOR {
    hPhysicalMonitor: HANDLE,
    szPhysicalMonitorDescription: [WCHAR; PHYSICAL_MONITOR_DESCRIPTION_SIZE],
}}
pub type LPPHYSICAL_MONITOR = *mut PHYSICAL_MONITOR;
extern "system" {
    pub fn GetNumberOfPhysicalMonitorsFromHMONITOR(
        hMonitor: HMONITOR,
        pdwNumberOfPhysicalMonitor: LPDWORD,
    ) -> _BOOL;
    pub fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(
        pDirect3DDevice9: *mut IDirect3DDevice9,
        pdwNumberOfPhysicalMonitor: LPDWORD,
    ) -> HRESULT;
    pub fn GetPhysicalMonitorsFromHMONITOR(
        hMonitor: HMONITOR,
        dwPhysicalMonitorArraySize: DWORD,
        pPhysicalMonitorArray: LPPHYSICAL_MONITOR,
    ) -> _BOOL;
    pub fn GetPhysicalMonitorsFromIDirect3DDevice9(
        pDirect3DDevice9: IDirect3DDevice9,
        dwPhysicalMonitorArraySize: DWORD,
        pPhysicalMonitorArray: LPPHYSICAL_MONITOR,
    ) -> HRESULT;
    pub fn DestroyPhysicalMonitor(
        hMonitor: HANDLE,
    ) -> _BOOL;
    pub fn DestroyPhysicalMonitors(
        dwPhysicalMonitorArraySize: DWORD,
        pPhysicalMonitorArray: LPPHYSICAL_MONITOR,
    ) -> _BOOL;
}
