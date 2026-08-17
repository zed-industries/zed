// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BYTE, DWORD, LPDWORD};
use um::physicalmonitorenumerationapi::_BOOL;
use um::winnt::{HANDLE, LPSTR};
STRUCT!{#[repr(packed)] struct MC_TIMING_REPORT {
    dwHorizontalFrequencyInHZ: DWORD,
    dwVerticalFrequencyInHZ: DWORD,
    bTimingStatusByte: BYTE,
}}
pub type LPMC_TIMING_REPORT = *mut MC_TIMING_REPORT;
ENUM!{enum MC_VCP_CODE_TYPE {
    MC_MOMENTARY,
    MC_SET_PARAMETER,
}}
pub type LPMC_VCP_CODE_TYPE = *mut MC_VCP_CODE_TYPE;
extern "system" {
    pub fn GetVCPFeatureAndVCPFeatureReply(
        hMonitor: HANDLE,
        bVCPCode: BYTE,
        pvct: LPMC_VCP_CODE_TYPE,
        pdwCurrentValue: LPDWORD,
        pdwMaximumValue: LPDWORD,
    ) -> _BOOL;
    pub fn SetVCPFeature(
        hMonitor: HANDLE,
        bVCPCode: BYTE,
        dwNewValue: DWORD,
    ) -> _BOOL;
    pub fn SaveCurrentSettings(
        hMonitor: HANDLE,
    ) -> _BOOL;
    pub fn GetCapabilitiesStringLength(
        hMonitor: HANDLE,
        pdwCapabilitiesStringLengthInCharacters: LPDWORD,
    ) -> _BOOL;
    pub fn CapabilitiesRequestAndCapabilitiesReply(
        hMonitor: HANDLE,
        pszASCIICapabilitiesString: LPSTR,
        dwCapabilitiesStringLengthInCharacters: DWORD,
    ) -> _BOOL;
    pub fn GetTimingReport(
        hMonitor: HANDLE,
        pmtrMonitorTimingReport: LPMC_TIMING_REPORT,
    ) -> _BOOL;
}
