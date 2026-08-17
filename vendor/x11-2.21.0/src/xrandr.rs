// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort};

use super::xlib::{Atom, Bool, Display, Drawable, Status, Time, Window, XEvent, XID};
use super::xrender::{XFixed, XTransform};

//
// functions
//

x11_link! { Xrandr, xrandr, ["libXrandr.so.2", "libXrandr.so"], 70,
    pub fn XRRAddOutputMode (dpy: *mut Display, output: RROutput, mode: RRMode) -> (),
    pub fn XRRAllocGamma (size: c_int) -> *mut XRRCrtcGamma,
    pub fn XRRAllocModeInfo (name: *const c_char, nameLength: c_int) -> *mut XRRModeInfo,
    pub fn XRRAllocateMonitor (dpy: *mut Display, noutput: c_int) -> *mut XRRMonitorInfo,
    pub fn XRRChangeOutputProperty (dpy: *mut Display, output: RROutput, property: Atom, type_: Atom, format: c_int, mode: c_int, data: *const c_uchar, nelements: c_int) -> (),
    pub fn XRRChangeProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom, type_: Atom, format: c_int, mode: c_int, data: *const c_uchar, nelements: c_int) -> (),
    pub fn XRRConfigCurrentConfiguration (config: *mut XRRScreenConfiguration, rotation: *mut Rotation) -> SizeID,
    pub fn XRRConfigCurrentRate (config: *mut XRRScreenConfiguration) -> c_short,
    pub fn XRRConfigRates (config: *mut XRRScreenConfiguration, sizeID: c_int, nrates: *mut c_int) -> *mut c_short,
    pub fn XRRConfigRotations (config: *mut XRRScreenConfiguration, current_rotation: *mut Rotation) -> Rotation,
    pub fn XRRConfigSizes (config: *mut XRRScreenConfiguration, nsizes: *mut c_int) -> *mut XRRScreenSize,
    pub fn XRRConfigTimes (config: *mut XRRScreenConfiguration, config_timestamp: *mut Time) -> Time,
    pub fn XRRConfigureOutputProperty (dpy: *mut Display, output: RROutput, property: Atom, pending: Bool, range: Bool, num_values: c_int, values: *mut c_long) -> (),
    pub fn XRRConfigureProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom, pending: Bool, range: Bool, num_values: c_int, values: *mut c_long) -> (),
    pub fn XRRCreateMode (dpy: *mut Display, window: Window, modeInfo: *mut XRRModeInfo) -> RRMode,
    pub fn XRRDeleteMonitor (dpy: *mut Display, window: Window, name: Atom) -> (),
    pub fn XRRDeleteOutputMode (dpy: *mut Display, output: RROutput, mode: RRMode) -> (),
    pub fn XRRDeleteOutputProperty (dpy: *mut Display, output: RROutput, property: Atom) -> (),
    pub fn XRRDeleteProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom) -> (),
    pub fn XRRDestroyMode (dpy: *mut Display, mode: RRMode) -> (),
    pub fn XRRFreeCrtcInfo (crtcInfo: *mut XRRCrtcInfo) -> (),
    pub fn XRRFreeGamma (gamma: *mut XRRCrtcGamma) -> (),
    pub fn XRRFreeModeInfo (modeInfo: *mut XRRModeInfo) -> (),
    pub fn XRRFreeMonitors (monitors: *mut XRRMonitorInfo) -> (),
    pub fn XRRFreeOutputInfo (outputInfo: *mut XRROutputInfo) -> (),
    pub fn XRRFreePanning (panning: *mut XRRPanning) -> (),
    pub fn XRRFreeProviderInfo (provider: *mut XRRProviderInfo) -> (),
    pub fn XRRFreeProviderResources (resources: *mut XRRProviderResources) -> (),
    pub fn XRRFreeScreenConfigInfo (config: *mut XRRScreenConfiguration) -> (),
    pub fn XRRFreeScreenResources (resources: *mut XRRScreenResources) -> (),
    pub fn XRRGetCrtcGamma (dpy: *mut Display, crtc: RRCrtc) -> *mut XRRCrtcGamma,
    pub fn XRRGetCrtcGammaSize (dpy: *mut Display, crtc: RRCrtc) -> c_int,
    pub fn XRRGetCrtcInfo (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc) -> *mut XRRCrtcInfo,
    pub fn XRRGetCrtcTransform (dpy: *mut Display, crtc: RRCrtc, attributes: *mut *mut XRRCrtcTransformAttributes) -> Status,
    pub fn XRRGetMonitors (dpy: *mut Display, window: Window, get_active: Bool, nmonitors: *mut c_int) -> *mut XRRMonitorInfo,
    pub fn XRRGetOutputInfo (dpy: *mut Display, resources: *mut XRRScreenResources, output: RROutput) -> *mut XRROutputInfo,
    pub fn XRRGetOutputPrimary (dpy: *mut Display, window: Window) -> RROutput,
    pub fn XRRGetOutputProperty (dpy: *mut Display, output: RROutput, property: Atom, offset: c_long, length: c_long, _delete: Bool, pending: Bool, req_type: Atom, actual_type: *mut Atom, actual_format: *mut c_int, nitems: *mut c_ulong, bytes_after: *mut c_ulong, prop: *mut *mut c_uchar) -> c_int,
    pub fn XRRGetPanning (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc) -> *mut XRRPanning,
    pub fn XRRGetProviderInfo (dpy: *mut Display, resources: *mut XRRScreenResources, provider: RRProvider) -> *mut XRRProviderInfo,
    pub fn XRRGetProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom, offset: c_long, length: c_long, _delete: Bool, pending: Bool, req_type: Atom, actual_type: *mut Atom, actual_format: *mut c_int, nitems: *mut c_ulong, bytes_after: *mut c_ulong, prop: *mut *mut c_uchar) -> c_int,
    pub fn XRRGetProviderResources (dpy: *mut Display, window: Window) -> *mut XRRProviderResources,
    pub fn XRRGetScreenInfo (dpy: *mut Display, window: Window) -> *mut XRRScreenConfiguration,
    pub fn XRRGetScreenResources (dpy: *mut Display, window: Window) -> *mut XRRScreenResources,
    pub fn XRRGetScreenResourcesCurrent (dpy: *mut Display, window: Window) -> *mut XRRScreenResources,
    pub fn XRRGetScreenSizeRange (dpy: *mut Display, window: Window, minWidth: *mut c_int, minHeight: *mut c_int, maxWidth: *mut c_int, maxHeight: *mut c_int) -> Status,
    pub fn XRRListOutputProperties (dpy: *mut Display, output: RROutput, nprop: *mut c_int) -> *mut Atom,
    pub fn XRRListProviderProperties (dpy: *mut Display, provider: RRProvider, nprop: *mut c_int) -> *mut Atom,
    pub fn XRRQueryExtension (dpy: *mut Display, event_base_return: *mut c_int, error_base_return: *mut c_int) -> Bool,
    pub fn XRRQueryOutputProperty (dpy: *mut Display, output: RROutput, property: Atom) -> *mut XRRPropertyInfo,
    pub fn XRRQueryProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom) -> *mut XRRPropertyInfo,
    pub fn XRRQueryVersion (dpy: *mut Display, major_version_return: *mut c_int, minor_version_return: *mut c_int) -> Status,
    pub fn XRRRates (dpy: *mut Display, screen: c_int, sizeID: c_int, nrates: *mut c_int) -> *mut c_short,
    pub fn XRRRootToScreen (dpy: *mut Display, root: Window) -> c_int,
    pub fn XRRRotations (dpy: *mut Display, screen: c_int, current_rotation: *mut Rotation) -> Rotation,
    pub fn XRRSelectInput (dpy: *mut Display, window: Window, mask: c_int) -> (),
    pub fn XRRSetCrtcConfig (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc, timestamp: Time, x: c_int, y: c_int, mode: RRMode, rotation: Rotation, outputs: *mut RROutput, noutputs: c_int) -> Status,
    pub fn XRRSetCrtcGamma (dpy: *mut Display, crtc: RRCrtc, gamma: *mut XRRCrtcGamma) -> (),
    pub fn XRRSetCrtcTransform (dpy: *mut Display, crtc: RRCrtc, transform: *mut XTransform, filter: *const c_char, params: *mut XFixed, nparams: c_int) -> (),
    pub fn XRRSetMonitor (dpy: *mut Display, window: Window, monitor: *mut XRRMonitorInfo) -> (),
    pub fn XRRSetOutputPrimary (dpy: *mut Display, window: Window, output: RROutput) -> (),
    pub fn XRRSetPanning (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc, panning: *mut XRRPanning) -> Status,
    pub fn XRRSetProviderOffloadSink (dpy: *mut Display, provider: XID, sink_provider: XID) -> c_int,
    pub fn XRRSetProviderOutputSource (dpy: *mut Display, provider: XID, source_provider: XID) -> c_int,
    pub fn XRRSetScreenConfig (dpy: *mut Display, config: *mut XRRScreenConfiguration, draw: Drawable, size_index: c_int, rotation: Rotation, timestamp: Time) -> Status,
    pub fn XRRSetScreenConfigAndRate (dpy: *mut Display, config: *mut XRRScreenConfiguration, draw: Drawable, size_index: c_int, rotation: Rotation, rate: c_short, timestamp: Time) -> Status,
    pub fn XRRSetScreenSize (dpy: *mut Display, window: Window, width: c_int, height: c_int, mmWidth: c_int, mmHeight: c_int) -> (),
    pub fn XRRSizes (dpy: *mut Display, screen: c_int, nsizes: *mut c_int) -> *mut XRRScreenSize,
    pub fn XRRTimes (dpy: *mut Display, screen: c_int, config_timestamp: *mut Time) -> Time,
    pub fn XRRUpdateConfiguration (event: *mut XEvent) -> c_int,
variadic:
globals:
}

//
// types
//

pub type Connection = c_ushort;
pub type Rotation = c_ushort;
pub type SizeID = c_ushort;
pub type SubpixelOrder = c_ushort;

pub type RROutput = XID;
pub type RRCrtc = XID;
pub type RRMode = XID;
pub type RRProvider = XID;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRScreenSize {
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
}

#[repr(C)]
pub struct XRRScreenConfiguration;

pub type XRRModeFlags = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRModeInfo {
    pub id: RRMode,
    pub width: c_uint,
    pub height: c_uint,
    pub dotClock: c_ulong,
    pub hSyncStart: c_uint,
    pub hSyncEnd: c_uint,
    pub hTotal: c_uint,
    pub hSkew: c_uint,
    pub vSyncStart: c_uint,
    pub vSyncEnd: c_uint,
    pub vTotal: c_uint,
    pub name: *mut c_char,
    pub nameLength: c_uint,
    pub modeFlags: XRRModeFlags,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRScreenResources {
    pub timestamp: Time,
    pub configTimestamp: Time,
    pub ncrtc: c_int,
    pub crtcs: *mut RRCrtc,
    pub noutput: c_int,
    pub outputs: *mut RROutput,
    pub nmode: c_int,
    pub modes: *mut XRRModeInfo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRROutputInfo {
    pub timestamp: Time,
    pub crtc: RRCrtc,
    pub name: *mut c_char,
    pub nameLen: c_int,
    pub mm_width: c_ulong,
    pub mm_height: c_ulong,
    pub connection: Connection,
    pub subpixel_order: SubpixelOrder,
    pub ncrtc: c_int,
    pub crtcs: *mut RRCrtc,
    pub nclone: c_int,
    pub clones: *mut RROutput,
    pub nmode: c_int,
    pub npreferred: c_int,
    pub modes: *mut RRMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRPropertyInfo {
    pub pending: Bool,
    pub range: Bool,
    pub immutable: Bool,
    pub num_values: c_int,
    pub values: *mut c_long,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcInfo {
    pub timestamp: Time,
    pub x: c_int,
    pub y: c_int,
    pub width: c_uint,
    pub height: c_uint,
    pub mode: RRMode,
    pub rotation: Rotation,
    pub noutput: c_int,
    pub outputs: *mut RROutput,
    pub rotations: Rotation,
    pub npossible: c_int,
    pub possible: *mut RROutput,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcGamma {
    pub size: c_int,
    pub red: *mut c_ushort,
    pub green: *mut c_ushort,
    pub blue: *mut c_ushort,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcTransformAttributes {
    pub pendingTransform: XTransform,
    pub pendingFilter: *mut c_char,
    pub pendingNparams: c_int,
    pub pendingParams: *mut XFixed,
    pub currentTransform: XTransform,
    pub currentFilter: *mut c_char,
    pub currentNparams: c_int,
    pub currentParams: *mut XFixed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRPanning {
    pub timestamp: Time,
    pub left: c_uint,
    pub top: c_uint,
    pub width: c_uint,
    pub height: c_uint,
    pub track_left: c_uint,
    pub track_top: c_uint,
    pub track_width: c_uint,
    pub track_height: c_uint,
    pub border_left: c_int,
    pub border_top: c_int,
    pub border_right: c_int,
    pub border_bottom: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderResources {
    pub timestamp: Time,
    pub nproviders: c_int,
    pub providers: *mut RRProvider,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderInfo {
    pub capabilities: c_uint,
    pub ncrtcs: c_int,
    pub crtcs: *mut RRCrtc,
    pub noutputs: c_int,
    pub outputs: *mut RROutput,
    pub name: *mut c_char,
    pub nassociatedproviders: c_int,
    pub associated_providers: *mut RRProvider,
    pub associated_capability: *mut c_uint,
    pub nameLen: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRMonitorInfo {
    pub name: Atom,
    pub primary: Bool,
    pub automatic: Bool,
    pub noutput: c_int,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
    pub outputs: *mut RROutput,
}

//
// event structures
//

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRScreenChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub timestamp: Time,
    pub config_timestamp: Time,
    pub size_index: SizeID,
    pub subpixel_order: SubpixelOrder,
    pub rotation: Rotation,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRROutputChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub output: RROutput,
    pub crtc: RRCrtc,
    pub mode: RRMode,
    pub rotation: Rotation,
    pub connection: Connection,
    pub subpixel_order: SubpixelOrder,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub crtc: RRCrtc,
    pub mode: RRMode,
    pub rotation: Rotation,
    pub x: c_int,
    pub y: c_int,
    pub width: c_uint,
    pub height: c_uint,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRROutputPropertyNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub output: RROutput,
    pub property: Atom,
    pub timestamp: Time,
    pub state: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub provider: RRProvider,
    pub timestamp: Time,
    pub current_role: c_uint,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderPropertyNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub provider: RRProvider,
    pub property: Atom,
    pub timestamp: Time,
    pub state: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRResourceChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub timestamp: Time,
}

event_conversions_and_tests! {
  xrr_screen_change_notify: XRRScreenChangeNotifyEvent,
  xrr_notify: XRRNotifyEvent,
  xrr_output_change_notify: XRROutputChangeNotifyEvent,
  xrr_crtc_change_notify: XRRCrtcChangeNotifyEvent,
  xrr_output_property_notify: XRROutputPropertyNotifyEvent,
  xrr_provider_change_notify: XRRProviderChangeNotifyEvent,
  xrr_provider_property_notify: XRRProviderPropertyNotifyEvent,
  xrr_resource_change_notify: XRRResourceChangeNotifyEvent,
}

//
// constants
//

pub const RANDR_NAME: &str = "RANDR";
pub const RANDR_MAJOR: c_int = 1;
pub const RANDR_MINOR: c_int = 5;

pub const RRNumberErrors: c_int = 4;
pub const RRNumberEvents: c_int = 2;
pub const RRNumberRequests: c_int = 45;

pub const X_RRQueryVersion: c_int = 0;
pub const X_RROldGetScreenInfo: c_int = 1;
pub const X_RRSetScreenConfig: c_int = 2;
pub const X_RROldScreenChangeSelectInput: c_int = 3;
pub const X_RRSelectInput: c_int = 4;
pub const X_RRGetScreenInfo: c_int = 5;

pub const X_RRGetScreenSizeRange: c_int = 6;
pub const X_RRSetScreenSize: c_int = 7;
pub const X_RRGetScreenResources: c_int = 8;
pub const X_RRGetOutputInfo: c_int = 9;
pub const X_RRListOutputProperties: c_int = 10;
pub const X_RRQueryOutputProperty: c_int = 11;
pub const X_RRConfigureOutputProperty: c_int = 12;
pub const X_RRChangeOutputProperty: c_int = 13;
pub const X_RRDeleteOutputProperty: c_int = 14;
pub const X_RRGetOutputProperty: c_int = 15;
pub const X_RRCreateMode: c_int = 16;
pub const X_RRDestroyMode: c_int = 17;
pub const X_RRAddOutputMode: c_int = 18;
pub const X_RRDeleteOutputMode: c_int = 19;
pub const X_RRGetCrtcInfo: c_int = 20;
pub const X_RRSetCrtcConfig: c_int = 21;
pub const X_RRGetCrtcGammaSize: c_int = 22;
pub const X_RRGetCrtcGamma: c_int = 23;
pub const X_RRSetCrtcGamma: c_int = 24;

pub const X_RRGetScreenResourcesCurrent: c_int = 25;
pub const X_RRSetCrtcTransform: c_int = 26;
pub const X_RRGetCrtcTransform: c_int = 27;
pub const X_RRGetPanning: c_int = 28;
pub const X_RRSetPanning: c_int = 29;
pub const X_RRSetOutputPrimary: c_int = 30;
pub const X_RRGetOutputPrimary: c_int = 31;

pub const X_RRGetProviders: c_int = 32;
pub const X_RRGetProviderInfo: c_int = 33;
pub const X_RRSetProviderOffloadSink: c_int = 34;
pub const X_RRSetProviderOutputSource: c_int = 35;
pub const X_RRListProviderProperties: c_int = 36;
pub const X_RRQueryProviderProperty: c_int = 37;
pub const X_RRConfigureProviderProperty: c_int = 38;
pub const X_RRChangeProviderProperty: c_int = 39;
pub const X_RRDeleteProviderProperty: c_int = 40;
pub const X_RRGetProviderProperty: c_int = 41;

pub const X_RRGetMonitors: c_int = 42;
pub const X_RRSetMonitor: c_int = 43;
pub const X_RRDeleteMonitor: c_int = 44;

pub const RRTransformUnit: c_int = 1 << 0;
pub const RRTransformScaleUp: c_int = 1 << 1;
pub const RRTransformScaleDown: c_int = 1 << 2;
pub const RRTransformProjective: c_int = 1 << 3;

pub const RRScreenChangeNotifyMask: c_int = 1 << 0;
pub const RRCrtcChangeNotifyMask: c_int = 1 << 1;
pub const RROutputChangeNotifyMask: c_int = 1 << 2;
pub const RROutputPropertyNotifyMask: c_int = 1 << 3;
pub const RRProviderChangeNotifyMask: c_int = 1 << 4;
pub const RRProviderPropertyNotifyMask: c_int = 1 << 5;
pub const RRResourceChangeNotifyMask: c_int = 1 << 6;

pub const RRScreenChangeNotify: c_int = 0;
pub const RRNotify: c_int = 1;
pub const RRNotify_CrtcChange: c_int = 0;
pub const RRNotify_OutputChange: c_int = 1;
pub const RRNotify_OutputProperty: c_int = 2;
pub const RRNotify_ProviderChange: c_int = 3;
pub const RRNotify_ProviderProperty: c_int = 4;
pub const RRNotify_ResourceChange: c_int = 5;

pub const RR_Rotate_0: c_int = 1;
pub const RR_Rotate_90: c_int = 2;
pub const RR_Rotate_180: c_int = 4;
pub const RR_Rotate_270: c_int = 8;

pub const RR_Reflect_X: c_int = 16;
pub const RR_Reflect_Y: c_int = 32;

pub const RRSetConfigSuccess: c_int = 0;
pub const RRSetConfigInvalidConfigTime: c_int = 1;
pub const RRSetConfigInvalidTime: c_int = 2;
pub const RRSetConfigFailed: c_int = 3;

pub const RR_HSyncPositive: c_int = 0x00000001;
pub const RR_HSyncNegative: c_int = 0x00000002;
pub const RR_VSyncPositive: c_int = 0x00000004;
pub const RR_VSyncNegative: c_int = 0x00000008;
pub const RR_Interlace: c_int = 0x00000010;
pub const RR_DoubleScan: c_int = 0x00000020;
pub const RR_CSync: c_int = 0x00000040;
pub const RR_CSyncPositive: c_int = 0x00000080;
pub const RR_CSyncNegative: c_int = 0x00000100;
pub const RR_HSkewPresent: c_int = 0x00000200;
pub const RR_BCast: c_int = 0x00000400;
pub const RR_PixelMultiplex: c_int = 0x00000800;
pub const RR_DoubleClock: c_int = 0x00001000;
pub const RR_ClockDivideBy2: c_int = 0x00002000;

pub const RR_Connected: c_int = 0;
pub const RR_Disconnected: c_int = 1;
pub const RR_UnknownConnection: c_int = 2;

pub const BadRROutput: c_int = 0;
pub const BadRRCrtc: c_int = 1;
pub const BadRRMode: c_int = 2;
pub const BadRRProvider: c_int = 3;

pub const RR_PROPERTY_BACKLIGHT: &str = "Backlight";
pub const RR_PROPERTY_RANDR_EDID: &str = "EDID";
pub const RR_PROPERTY_SIGNAL_FORMAT: &str = "SignalFormat";
pub const RR_PROPERTY_SIGNAL_PROPERTIES: &str = "SignalProperties";
pub const RR_PROPERTY_CONNECTOR_TYPE: &str = "ConnectorType";
pub const RR_PROPERTY_CONNECTOR_NUMBER: &str = "ConnectorNumber";
pub const RR_PROPERTY_COMPATIBILITY_LIST: &str = "CompatibilityList";
pub const RR_PROPERTY_CLONE_LIST: &str = "CloneList";
pub const RR_PROPERTY_BORDER: &str = "Border";
pub const RR_PROPERTY_BORDER_DIMENSIONS: &str = "BorderDimensions";
pub const RR_PROPERTY_GUID: &str = "GUID";
pub const RR_PROPERTY_RANDR_TILE: &str = "TILE";

pub const RR_Capability_None: c_int = 0;
pub const RR_Capability_SourceOutput: c_int = 1;
pub const RR_Capability_SinkOutput: c_int = 2;
pub const RR_Capability_SourceOffload: c_int = 4;
pub const RR_Capability_SinkOffload: c_int = 8;
