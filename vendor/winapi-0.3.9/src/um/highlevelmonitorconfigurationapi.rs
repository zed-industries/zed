// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, LPDWORD};
use um::physicalmonitorenumerationapi::_BOOL;
use um::winnt::HANDLE;
pub const MC_CAPS_NONE: DWORD = 0x00000000;
pub const MC_CAPS_MONITOR_TECHNOLOGY_TYPE: DWORD = 0x00000001;
pub const MC_CAPS_BRIGHTNESS: DWORD = 0x00000002;
pub const MC_CAPS_CONTRAST: DWORD = 0x00000004;
pub const MC_CAPS_COLOR_TEMPERATURE: DWORD = 0x00000008;
pub const MC_CAPS_RED_GREEN_BLUE_GAIN: DWORD = 0x00000010;
pub const MC_CAPS_RED_GREEN_BLUE_DRIVE: DWORD = 0x00000020;
pub const MC_CAPS_DEGAUSS: DWORD = 0x00000040;
pub const MC_CAPS_DISPLAY_AREA_POSITION: DWORD = 0x00000080;
pub const MC_CAPS_DISPLAY_AREA_SIZE: DWORD = 0x00000100;
pub const MC_CAPS_RESTORE_FACTORY_DEFAULTS: DWORD = 0x00000400;
pub const MC_CAPS_RESTORE_FACTORY_COLOR_DEFAULTS: DWORD = 0x00000800;
pub const MC_RESTORE_FACTORY_DEFAULTS_ENABLES_MONITOR_SETTINGS: DWORD = 0x00001000;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_NONE: DWORD = 0x00000000;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_4000K: DWORD = 0x00000001;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_5000K: DWORD = 0x00000002;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_6500K: DWORD = 0x00000004;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_7500K: DWORD = 0x00000008;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_8200K: DWORD = 0x00000010;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_9300K: DWORD = 0x00000020;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_10000K: DWORD = 0x00000040;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_11500K: DWORD = 0x00000080;
ENUM!{enum MC_DISPLAY_TECHNOLOGY_TYPE {
    MC_SHADOW_MASK_CATHODE_RAY_TUBE,
    MC_APERTURE_GRILL_CATHODE_RAY_TUBE,
    MC_THIN_FILM_TRANSISTOR,
    MC_LIQUID_CRYSTAL_ON_SILICON,
    MC_PLASMA,
    MC_ORGANIC_LIGHT_EMITTING_DIODE,
    MC_ELECTROLUMINESCENT,
    MC_MICROELECTROMECHANICAL,
    MC_FIELD_EMISSION_DEVICE,
}}
pub type LPMC_DISPLAY_TECHNOLOGY_TYPE = *mut MC_DISPLAY_TECHNOLOGY_TYPE;
ENUM!{enum MC_DRIVE_TYPE {
    MC_RED_DRIVE,
    MC_GREEN_DRIVE,
    MC_BLUE_DRIVE,
}}
ENUM!{enum MC_GAIN_TYPE {
    MC_RED_GAIN,
    MC_GREEN_GAIN,
    MC_BLUE_GAIN,
}}
ENUM!{enum MC_POSITION_TYPE {
    MC_HORIZONTAL_POSITION,
    MC_VERTICAL_POSITION,
}}
ENUM!{enum MC_SIZE_TYPE {
    MC_WIDTH,
    MC_HEIGHT,
}}
ENUM!{enum MC_COLOR_TEMPERATURE {
    MC_COLOR_TEMPERATURE_UNKNOWN,
    MC_COLOR_TEMPERATURE_4000K,
    MC_COLOR_TEMPERATURE_5000K,
    MC_COLOR_TEMPERATURE_6500K,
    MC_COLOR_TEMPERATURE_7500K,
    MC_COLOR_TEMPERATURE_8200K,
    MC_COLOR_TEMPERATURE_9300K,
    MC_COLOR_TEMPERATURE_10000K,
    MC_COLOR_TEMPERATURE_11500K,
}}
pub type LPMC_COLOR_TEMPERATURE = *mut MC_COLOR_TEMPERATURE;
extern "system" {
    pub fn GetMonitorCapabilities(
        hMonitor: HANDLE,
        pdwMonitorCapabilities: LPDWORD,
        pdwSupportedColorTemperature: LPDWORD,
    ) -> _BOOL;
    pub fn SaveCurrentMonitorSettings(
        hMonitor: HANDLE,
    ) -> _BOOL;
    pub fn GetMonitorTechnologyType(
        hMonitor: HANDLE,
        pdtyDisplayTechnologyType: LPMC_DISPLAY_TECHNOLOGY_TYPE,
    ) -> _BOOL;
    pub fn GetMonitorBrightness(
        hMonitor: HANDLE,
        pdwMinimumBrightness: LPDWORD,
        pdwCurrentBrightness: LPDWORD,
        pdwMaximumBrightness: LPDWORD,
    ) -> _BOOL;
    pub fn GetMonitorContrast(
        hMonitor: HANDLE,
        pdwMinimumContrast: LPDWORD,
        pdwCurrentContrast: LPDWORD,
        pdwMaximumContrast: LPDWORD,
    ) -> _BOOL;
    pub fn GetMonitorColorTemperature(
        hMonitor: HANDLE,
        pctCurrentColorTemperature: LPMC_COLOR_TEMPERATURE,
    ) -> _BOOL;
    pub fn GetMonitorRedGreenOrBlueDrive(
        hMonitor: HANDLE,
        dtDriveType: MC_DRIVE_TYPE,
        pdwMinimumDrive: LPDWORD,
        pdwCurrentDrive: LPDWORD,
        pdwMaximumDrive: LPDWORD,
    ) -> _BOOL;
    pub fn GetMonitorRedGreenOrBlueGain(
        hMonitor: HANDLE,
        gtGainType: MC_GAIN_TYPE,
        pdwMinimumGain: LPDWORD,
        pdwCurrentGain: LPDWORD,
        pdwMaximumGain: LPDWORD,
    ) -> _BOOL;
    pub fn SetMonitorBrightness(
        hMonitor: HANDLE,
        dwNewBrightness: DWORD,
    ) -> _BOOL;
    pub fn SetMonitorContrast(
        hMonitor: HANDLE,
        dwNewContrast: DWORD,
    ) -> _BOOL;
    pub fn SetMonitorColorTemperature(
        hMonitor: HANDLE,
        ctCurrentColorTemperature: MC_COLOR_TEMPERATURE,
    ) -> _BOOL;
    pub fn SetMonitorRedGreenOrBlueDrive(
        hMonitor: HANDLE,
        dtDriveType: MC_DRIVE_TYPE,
        dwNewDrive: DWORD,
    ) -> _BOOL;
    pub fn SetMonitorRedGreenOrBlueGain(
        hMonitor: HANDLE,
        gtGainType: MC_GAIN_TYPE,
        dwNewGain: DWORD,
    ) -> _BOOL;
    pub fn DegaussMonitor(
        hMonitor: HANDLE,
    ) -> _BOOL;
    pub fn GetMonitorDisplayAreaSize(
        hMonitor: HANDLE,
        stSizeType: MC_SIZE_TYPE,
        pdwMinimumWidthOrHeight: LPDWORD,
        pdwCurrentWidthOrHeight: LPDWORD,
        pdwMaximumWidthOrHeight: LPDWORD,
    ) -> _BOOL;
    pub fn GetMonitorDisplayAreaPosition(
        hMonitor: HANDLE,
        ptPositionType: MC_POSITION_TYPE,
        pdwMinimumPosition: LPDWORD,
        pdwCurrentPosition: LPDWORD,
        pdwMaximumPosition: LPDWORD,
    ) -> _BOOL;
    pub fn SetMonitorDisplayAreaSize(
        hMonitor: HANDLE,
        stSizeType: MC_SIZE_TYPE,
        dwNewDisplayAreaWidthOrHeight: DWORD,
    ) -> _BOOL;
    pub fn SetMonitorDisplayAreaPosition(
        hMonitor: HANDLE,
        ptPositionType: MC_POSITION_TYPE,
        dwNewPosition: DWORD,
    ) -> _BOOL;
    pub fn RestoreMonitorFactoryColorDefaults(
        hMonitor: HANDLE,
    ) -> _BOOL;
    pub fn RestoreMonitorFactoryDefaults(
        hMonitor: HANDLE,
    ) -> _BOOL;
}
