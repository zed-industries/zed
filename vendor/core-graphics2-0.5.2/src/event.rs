use std::mem::ManuallyDrop;

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFTypeID, TCFType},
    data::{CFData, CFDataRef},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
    mach_port::{CFMachPort, CFMachPortRef},
};
use libc::{c_double, c_ulong, c_void, pid_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{error::CGError, geometry::CGPoint};
pub use crate::{event_source::CGEventSource, event_types::*, remote_operation::CGKeyCode};

extern "C" {
    pub fn CGEventGetTypeID() -> CFTypeID;
    pub fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;
    pub fn CGEventCreateData(allocator: CFAllocatorRef, event: CGEventRef) -> CFDataRef;
    pub fn CGEventCreateFromData(allocator: CFAllocatorRef, data: CFDataRef) -> CGEventRef;
    pub fn CGEventCreateMouseEvent(
        source: CGEventSourceRef,
        mouseType: CGEventType,
        mouseCursorPosition: CGPoint,
        mouseButton: CGMouseButton,
    ) -> CGEventRef;
    pub fn CGEventCreateKeyboardEvent(source: CGEventSourceRef, virtualKey: CGKeyCode, keydown: bool) -> CGEventRef;
    pub fn CGEventCreateScrollWheelEvent2(
        source: CGEventSourceRef,
        units: CGScrollEventUnit,
        wheelCount: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> CGEventRef;
    pub fn CGEventCreateCopy(event: CGEventRef) -> CGEventRef;
    pub fn CGEventCreateSourceFromEvent(event: CGEventRef) -> CGEventSourceRef;
    pub fn CGEventSetSource(event: CGEventRef, source: CGEventSourceRef);
    pub fn CGEventGetType(event: CGEventRef) -> CGEventType;
    pub fn CGEventSetType(event: CGEventRef, type_: CGEventType);
    pub fn CGEventGetTimestamp(event: CGEventRef) -> CGEventTimestamp;
    pub fn CGEventSetTimestamp(event: CGEventRef, timestamp: CGEventTimestamp);
    pub fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    pub fn CGEventGetUnflippedLocation(event: CGEventRef) -> CGPoint;
    pub fn CGEventSetLocation(event: CGEventRef, location: CGPoint);
    pub fn CGEventGetFlags(event: CGEventRef) -> CGEventFlags;
    pub fn CGEventSetFlags(event: CGEventRef, flags: CGEventFlags);
    pub fn CGEventKeyboardGetUnicodeString(
        event: CGEventRef,
        maxStringLength: c_ulong,
        actualStringLength: *mut c_ulong,
        unicodeString: *mut u16,
    ) -> bool;
    pub fn CGEventKeyboardSetUnicodeString(event: CGEventRef, stringLength: c_ulong, unicodeString: *const u16);
    pub fn CGEventGetIntegerValueField(event: CGEventRef, field: CGEventField) -> i64;
    pub fn CGEventSetIntegerValueField(event: CGEventRef, field: CGEventField, value: i64);
    pub fn CGEventGetDoubleValueField(event: CGEventRef, field: CGEventField) -> c_double;
    pub fn CGEventSetDoubleValueField(event: CGEventRef, field: CGEventField, value: c_double);

    pub fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallBack,
        userInfo: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapCreateForPSN(
        processSerialNumber: *const c_void,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallBack,
        userInfo: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapCreateForPid(
        pid: pid_t,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallBack,
        userInfo: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    pub fn CGEventTapIsEnabled(tap: CFMachPortRef) -> bool;
    pub fn CGEventTapPostEvent(proxy: CGEventTapProxy, event: CGEventRef);

    pub fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
    pub fn CGEventPostToPSN(processSerialNumber: *mut c_void, event: CGEventRef);
    pub fn CGEventPostToPid(pid: pid_t, event: CGEventRef);
    pub fn CGGetEventTapList(maxNumberOfTaps: u32, tapList: *mut CGEventTapInformation, eventTapCount: *mut u32) -> CGError;
    pub fn CGPreflightListenEventAccess() -> bool;
    pub fn CGRequestListenEventAccess() -> bool;
    pub fn CGPreflightPostEventAccess() -> bool;
    pub fn CGRequestPostEventAccess() -> bool;
}

#[macro_export]
macro_rules! CGEventMaskBit {
    ($event_type:expr) => {
        1 << $event_type as CGEventMask
    };
}

pub type CGEventTapCallbackBox<'a> = Box<dyn FnMut(CGEventTapProxy, CGEventType, &CGEvent) -> Option<CGEvent> + 'a>;

pub struct CGEventTap<'a> {
    pub mach_port: CFMachPort,
    pub callback: Box<dyn FnMut(CGEventTapProxy, CGEventType, &CGEvent) -> Option<CGEvent> + 'a>,
}

impl<'a> CGEventTap<'a> {
    pub fn new<F>(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: CGEventTapOptions,
        events_of_interest: Vec<CGEventType>,
        closure: F,
    ) -> Result<CGEventTap<'a>, ()>
    where
        F: FnMut(CGEventTapProxy, CGEventType, &CGEvent) -> Option<CGEvent> + 'a,
    {
        let event_mask: CGEventMask = events_of_interest.iter().fold(CGEventType::Null as CGEventMask, |mask, &etype| mask | CGEventMaskBit!(etype));
        let cb = Box::into_raw(Box::new(Box::new(closure) as CGEventTapCallbackBox));
        unsafe {
            let event_tap = CGEventTapCreate(tap, place, options, event_mask, callback, cb as *mut c_void);
            if event_tap.is_null() {
                let _ = Box::from_raw(cb);
                return Err(());
            } else {
                return Ok(Self {
                    mach_port: CFMachPort::wrap_under_create_rule(event_tap),
                    callback: Box::from_raw(cb),
                });
            }
        }

        extern "C" fn callback(proxy: CGEventTapProxy, etype: CGEventType, event: CGEventRef, user_info: *mut c_void) -> CGEventRef {
            unsafe {
                let callback = user_info as *mut CGEventTapCallbackBox;
                let event = CGEvent::wrap_under_get_rule(event);
                let new_event = (*callback)(proxy, etype, &event);
                let event = new_event.unwrap_or(event);
                ManuallyDrop::new(event).as_concrete_TypeRef()
            }
        }
    }

    pub fn enable(&self, enable: bool) {
        unsafe { CGEventTapEnable(self.mach_port.as_concrete_TypeRef(), enable) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { CGEventTapIsEnabled(self.mach_port.as_concrete_TypeRef()) }
    }
}

declare_TCFType!(CGEvent, CGEventRef);
impl_TCFType!(CGEvent, CGEventRef, CGEventGetTypeID);
impl_CFTypeDescription!(CGEvent);

impl CGEvent {
    pub fn new(source: CGEventSource) -> Option<CGEvent> {
        unsafe {
            let event = CGEventCreate(source.as_concrete_TypeRef());
            if event.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(event))
            }
        }
    }

    pub fn new_data(&self) -> Option<CFData> {
        unsafe {
            let data = CGEventCreateData(kCFAllocatorDefault, self.as_concrete_TypeRef());
            if data.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(data))
            }
        }
    }

    pub fn new_mouse_event(source: CGEventSource, mouse_type: CGEventType, cursor_position: CGPoint, button: CGMouseButton) -> Option<CGEvent> {
        unsafe {
            let event = CGEventCreateMouseEvent(source.as_concrete_TypeRef(), mouse_type, cursor_position, button);
            if event.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(event))
            }
        }
    }

    pub fn new_keyboard_event(source: CGEventSource, keycode: CGKeyCode, keydown: bool) -> Option<CGEvent> {
        unsafe {
            let event = CGEventCreateKeyboardEvent(source.as_concrete_TypeRef(), keycode, keydown);
            if event.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(event))
            }
        }
    }

    pub fn new_scroll_event(
        source: CGEventSource,
        units: CGScrollEventUnit,
        wheel_count: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> Option<CGEvent> {
        unsafe {
            let event = CGEventCreateScrollWheelEvent2(source.as_concrete_TypeRef(), units, wheel_count, wheel1, wheel2, wheel3);
            if event.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(event))
            }
        }
    }

    pub fn new_copy(&self) -> Option<CGEvent> {
        unsafe {
            let event = CGEventCreateCopy(self.as_concrete_TypeRef());
            if event.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(event))
            }
        }
    }

    pub fn new_source(&self) -> Option<CGEventSource> {
        unsafe {
            let source = CGEventCreateSourceFromEvent(self.as_concrete_TypeRef());
            if source.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(source))
            }
        }
    }

    pub fn from_data(data: CFData) -> Option<CGEvent> {
        unsafe {
            let event = CGEventCreateFromData(kCFAllocatorDefault, data.as_concrete_TypeRef());
            if event.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(event))
            }
        }
    }

    pub fn set_source(&self, source: CGEventSource) {
        unsafe {
            CGEventSetSource(self.as_concrete_TypeRef(), source.as_concrete_TypeRef());
        }
    }

    pub fn get_type(&self) -> CGEventType {
        unsafe { CGEventGetType(self.as_concrete_TypeRef()) }
    }

    pub fn set_type(&self, event_type: CGEventType) {
        unsafe {
            CGEventSetType(self.as_concrete_TypeRef(), event_type);
        }
    }

    pub fn get_timestamp(&self) -> CGEventTimestamp {
        unsafe { CGEventGetTimestamp(self.as_concrete_TypeRef()) }
    }

    pub fn set_timestamp(&self, timestamp: CGEventTimestamp) {
        unsafe {
            CGEventSetTimestamp(self.as_concrete_TypeRef(), timestamp);
        }
    }

    pub fn get_location(&self) -> CGPoint {
        unsafe { CGEventGetLocation(self.as_concrete_TypeRef()) }
    }

    pub fn get_unflipped_location(&self) -> CGPoint {
        unsafe { CGEventGetUnflippedLocation(self.as_concrete_TypeRef()) }
    }

    pub fn set_location(&self, location: CGPoint) {
        unsafe {
            CGEventSetLocation(self.as_concrete_TypeRef(), location);
        }
    }

    pub fn get_flags(&self) -> CGEventFlags {
        unsafe { CGEventGetFlags(self.as_concrete_TypeRef()) }
    }

    pub fn set_flags(&self, flags: CGEventFlags) {
        unsafe {
            CGEventSetFlags(self.as_concrete_TypeRef(), flags);
        }
    }

    pub fn get_integer_value_field(&self, field: CGEventField) -> i64 {
        unsafe { CGEventGetIntegerValueField(self.as_concrete_TypeRef(), field) }
    }

    pub fn set_integer_value_field(&self, field: CGEventField, value: i64) {
        unsafe { CGEventSetIntegerValueField(self.as_concrete_TypeRef(), field, value) }
    }

    pub fn get_double_value_field(&self, field: CGEventField) -> f64 {
        unsafe { CGEventGetDoubleValueField(self.as_concrete_TypeRef(), field) }
    }

    pub fn set_double_value_field(&self, field: CGEventField, value: f64) {
        unsafe { CGEventSetDoubleValueField(self.as_concrete_TypeRef(), field, value) }
    }

    pub fn set_string(&self, string: &str) {
        let utf16: Vec<u16> = string.encode_utf16().collect();
        unsafe {
            CGEventKeyboardSetUnicodeString(self.as_concrete_TypeRef(), utf16.len() as c_ulong, utf16.as_ptr());
        }
    }

    pub fn post(&self, tap: CGEventTapLocation) {
        unsafe {
            CGEventPost(tap, self.as_concrete_TypeRef());
        }
    }

    pub fn post_to_pid(&self, pid: pid_t) {
        unsafe {
            CGEventPostToPid(pid, self.as_concrete_TypeRef());
        }
    }

    pub fn post_from_tap(&self, proxy: CGEventTapProxy) {
        unsafe {
            CGEventTapPostEvent(proxy, self.as_concrete_TypeRef());
        }
    }
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CGEvent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGEvent", &[]));
}
