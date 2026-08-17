// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong};

use crate::xlib::{Bool, Display, Drawable, Status, Time, XID};

//
// functions
//

x11_link! { Xext, xext, ["libXext.so.6", "libXext.so"], 38,
    pub fn XSyncQueryExtension(dpy: *mut Display, event_base: *mut c_int, error_base: *mut c_int) -> Status,
    pub fn XSyncInitialize(dpy: *mut Display, major_version: *mut c_int, minor_version: *mut c_int) -> Status,
    pub fn XSyncListSystemCounters(dpy: *mut Display, n_counters: *mut c_int) -> *mut XSyncSystemCounter,
    pub fn XSyncFreeSystemCounterList(list: *mut XSyncSystemCounter) -> (),
    pub fn XSyncCreateCounter(dpy: *mut Display, initial_value: XSyncValue) -> XSyncCounter,
    pub fn XSyncSetCounter(dpy: *mut Display, counter: XSyncCounter, value: XSyncValue) -> Status,
    pub fn XSyncChangeCounter(dpy: *mut Display, counter: XSyncCounter, value: XSyncValue) -> Status,
    pub fn XSyncDestroyCounter(dpy: *mut Display, value: XSyncCounter) -> Status,
    pub fn XSyncQueryCounter(dpy: *mut Display, counter: XSyncCounter, value: *mut XSyncValue) -> Status,
    pub fn XSyncAwait(dpy: *mut Display, wait_list: *mut XSyncWaitCondition, n_conditions: c_int) -> Status,
    pub fn XSyncCreateAlarm(dpy: *mut Display, values_mask: c_ulong, values: *mut XSyncAlarmAttributes) -> XSyncAlarm,
    pub fn XSyncDestroyAlarm(dpy: *mut Display, alarm: XSyncAlarm) -> Status,
    pub fn XSyncQueryAlarm(dpy: *mut Display, alarm: XSyncAlarm, values: *mut XSyncAlarmAttributes) -> Status,
    pub fn XSyncChangeAlarm(dpy: *mut Display, alarm: XSyncAlarm, values_mask: c_ulong, values: *mut XSyncAlarmAttributes) -> Status,
    pub fn XSyncSetPriority(dpy: *mut Display, client_resource_id: XID, priority: c_int) -> Status,
    pub fn XSyncGetPriority(dpy: *mut Display, client_resource_id: XID, priority: *mut c_int) -> Status,
    pub fn XSyncCreateFence(dpy: *mut Display, d: Drawable, initially_triggered: Bool) -> XSyncFence,
    pub fn XSyncTriggerFence(dpy: *mut Display, fence: XSyncFence) -> Bool,
    pub fn XSyncResetFence(dpy: *mut Display, fence: XSyncFence) -> Bool,
    pub fn XSyncDestroyFence(dpy: *mut Display, fence: XSyncFence) -> Bool,
    pub fn XSyncQueryFence(dpy: *mut Display, fence: XSyncFence, triggered: *mut Bool) -> Bool,
    pub fn XSyncAwaitFence(dpy: *mut Display, fence_list: *const XSyncFence, n_fences: c_int) -> Bool,
    pub fn XSyncIntToValue(pv: *mut XSyncValue, i: c_int) -> (),
    pub fn XSyncIntsToValue(pv: *mut XSyncValue, l: c_uint, h: c_int) -> (),
    pub fn XSyncValueGreaterThan(a: XSyncValue, b: XSyncValue) -> Bool,
    pub fn XSyncValueLessThan(a: XSyncValue, b: XSyncValue) -> Bool,
    pub fn XSyncValueGreaterOrEqual(a: XSyncValue, b: XSyncValue) -> Bool,
    pub fn XSyncValueLessOrEqual(a: XSyncValue, b: XSyncValue) -> Bool,
    pub fn XSyncValueEqual(a: XSyncValue, b: XSyncValue) -> Bool,
    pub fn XSyncValueIsNegative(v: XSyncValue) -> Bool,
    pub fn XSyncValueIsZero(v: XSyncValue) -> Bool,
    pub fn XSyncValueIsPositive(v: XSyncValue) -> Bool,
    pub fn XSyncValueLow32(v: XSyncValue) -> c_uint,
    pub fn XSyncValueHigh32(v: XSyncValue) -> c_int,
    pub fn XSyncValueAdd(presult: *mut XSyncValue, a: XSyncValue, b: XSyncValue, poverflow: *mut c_int) -> (),
    pub fn XSyncValueSubtract(presult: *mut XSyncValue, a: XSyncValue, b: XSyncValue, poverflow: *mut c_int) -> (),
    pub fn XSyncMaxValue(pv: *mut XSyncValue) -> (),
    pub fn XSyncMinValue(pv: *mut XSyncValue) -> (),
variadic:
globals:
}

//
// types
//

pub type XSyncValueType = c_uint;
pub type XSyncTestType = c_uint;
pub type XSyncAlarmState = c_uint;
pub type XSyncCounter = XID;
pub type XSyncAlarm = XID;
pub type XSyncFence = XID;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncAlarmError {
    pub type_: c_int,
    pub display: *mut Display,
    pub alarm: XSyncAlarm,
    pub serial: c_ulong,
    pub error_code: c_uchar,
    pub request_code: c_uchar,
    pub minor_code: c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncCounterError {
    pub type_: c_int,
    pub display: *mut Display,
    pub counter: XSyncCounter,
    pub serial: c_ulong,
    pub error_code: c_uchar,
    pub request_code: c_uchar,
    pub minor_code: c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncValue {
    pub hi: c_int,
    pub lo: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncTrigger {
    pub counter: XSyncCounter,
    pub value_type: XSyncValueType,
    pub wait_value: XSyncValue,
    pub test_type: XSyncTestType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncWaitCondition {
    pub trigger: XSyncTrigger,
    pub event_threshold: XSyncValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncAlarmAttributes {
    pub trigger: XSyncTrigger,
    pub delta: XSyncValue,
    pub events: Bool,
    pub state: XSyncAlarmState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncCounterNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub counter: XSyncCounter,
    pub wait_value: XSyncValue,
    pub counter_value: XSyncValue,
    pub time: Time,
    pub count: c_int,
    pub destroyed: Bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncAlarmNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub alarm: XSyncAlarm,
    pub counter_value: XSyncValue,
    pub alarm_value: XSyncValue,
    pub time: Time,
    pub state: XSyncAlarmState,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XSyncSystemCounter {
    pub name: *mut c_char,
    pub counter: XSyncCounter,
    pub resolution: XSyncValue,
}

//
// constants
//

pub const SYNC_NAME: &str = "SYNC";

pub const SYNC_MAJOR_VERSION: c_int = 3;
pub const SYNC_MINOR_VERSION: c_int = 1;

pub const XSyncCounterNotify: c_long = 0;
pub const XSyncAlarmNotify: c_long = 1;
pub const XSyncAlarmNotifyMask: c_long = 1 << XSyncAlarmNotify;

pub const XSyncNumberEvents: c_long = 2;

pub const XSyncBadCounter: c_long = 0;
pub const XSyncBadAlarm: c_long = 1;
pub const XSyncBadFence: c_long = 2;
pub const XSyncNumberErrors: c_long = XSyncBadFence + 1;

pub const XSyncCACounter: c_long = 1 << 0;
pub const XSyncCAValueType: c_long = 1 << 1;
pub const XSyncCAValue: c_long = 1 << 2;
pub const XSyncCATestType: c_long = 1 << 3;
pub const XSyncCADelta: c_long = 1 << 4;
pub const XSyncCAEvents: c_long = 1 << 5;

pub const XSyncValueType_XSyncAbsolute: XSyncValueType = 0;
pub const XSyncValueType_XSyncRelative: XSyncValueType = 1;

pub const XSyncTestType_XSyncPositiveTransition: XSyncTestType = 0;
pub const XSyncTestType_XSyncNegativeTransition: XSyncTestType = 1;
pub const XSyncTestType_XSyncPositiveComparison: XSyncTestType = 2;
pub const XSyncTestType_XSyncNegativeComparison: XSyncTestType = 3;

pub const XSyncAlarmState_XSyncAlarmActive: XSyncAlarmState = 0;
pub const XSyncAlarmState_XSyncAlarmInactive: XSyncAlarmState = 1;
pub const XSyncAlarmState_XSyncAlarmDestroyed: XSyncAlarmState = 2;
