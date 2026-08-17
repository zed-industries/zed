// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{UCHAR, ULONG};
STRUCT!{struct VIDEOPARAMETERS {
    Guid: GUID,
    dwOffset: ULONG,
    dwCommand: ULONG,
    dwFlags: ULONG,
    dwMode: ULONG,
    dwTVStandard: ULONG,
    dwAvailableModes: ULONG,
    dwAvailableTVStandard: ULONG,
    dwFlickerFilter: ULONG,
    dwOverScanX: ULONG,
    dwOverScanY: ULONG,
    dwMaxUnscaledX: ULONG,
    dwMaxUnscaledY: ULONG,
    dwPositionX: ULONG,
    dwPositionY: ULONG,
    dwBrightness: ULONG,
    dwContrast: ULONG,
    dwCPType: ULONG,
    dwCPCommand: ULONG,
    dwCPStandard: ULONG,
    dwCPKey: ULONG,
    bCP_APSTriggerBits: ULONG,
    bOEMCopyProtection: [UCHAR; 256],
}}
pub type PVIDEOPARAMETERS = *mut VIDEOPARAMETERS;
pub type LPVIDEOPARAMETERS = *mut VIDEOPARAMETERS;
pub const VP_COMMAND_GET: ULONG = 0x0001;
pub const VP_COMMAND_SET: ULONG = 0x0002;
pub const VP_FLAGS_TV_MODE: ULONG = 0x0001;
pub const VP_FLAGS_TV_STANDARD: ULONG = 0x0002;
pub const VP_FLAGS_FLICKER: ULONG = 0x0004;
pub const VP_FLAGS_OVERSCAN: ULONG = 0x0008;
pub const VP_FLAGS_MAX_UNSCALED: ULONG = 0x0010;
pub const VP_FLAGS_POSITION: ULONG = 0x0020;
pub const VP_FLAGS_BRIGHTNESS: ULONG = 0x0040;
pub const VP_FLAGS_CONTRAST: ULONG = 0x0080;
pub const VP_FLAGS_COPYPROTECT: ULONG = 0x0100;
pub const VP_MODE_WIN_GRAPHICS: ULONG = 0x0001;
pub const VP_MODE_TV_PLAYBACK: ULONG = 0x0002;
pub const VP_TV_STANDARD_NTSC_M: ULONG = 0x0001;
pub const VP_TV_STANDARD_NTSC_M_J: ULONG = 0x0002;
pub const VP_TV_STANDARD_PAL_B: ULONG = 0x0004;
pub const VP_TV_STANDARD_PAL_D: ULONG = 0x0008;
pub const VP_TV_STANDARD_PAL_H: ULONG = 0x0010;
pub const VP_TV_STANDARD_PAL_I: ULONG = 0x0020;
pub const VP_TV_STANDARD_PAL_M: ULONG = 0x0040;
pub const VP_TV_STANDARD_PAL_N: ULONG = 0x0080;
pub const VP_TV_STANDARD_SECAM_B: ULONG = 0x0100;
pub const VP_TV_STANDARD_SECAM_D: ULONG = 0x0200;
pub const VP_TV_STANDARD_SECAM_G: ULONG = 0x0400;
pub const VP_TV_STANDARD_SECAM_H: ULONG = 0x0800;
pub const VP_TV_STANDARD_SECAM_K: ULONG = 0x1000;
pub const VP_TV_STANDARD_SECAM_K1: ULONG = 0x2000;
pub const VP_TV_STANDARD_SECAM_L: ULONG = 0x4000;
pub const VP_TV_STANDARD_WIN_VGA: ULONG = 0x8000;
pub const VP_TV_STANDARD_NTSC_433: ULONG = 0x00010000;
pub const VP_TV_STANDARD_PAL_G: ULONG = 0x00020000;
pub const VP_TV_STANDARD_PAL_60: ULONG = 0x00040000;
pub const VP_TV_STANDARD_SECAM_L1: ULONG = 0x00080000;
pub const VP_CP_TYPE_APS_TRIGGER: ULONG = 0x0001;
pub const VP_CP_TYPE_MACROVISION: ULONG = 0x0002;
pub const VP_CP_CMD_ACTIVATE: ULONG = 0x0001;
pub const VP_CP_CMD_DEACTIVATE: ULONG = 0x0002;
pub const VP_CP_CMD_CHANGE: ULONG = 0x0004;
