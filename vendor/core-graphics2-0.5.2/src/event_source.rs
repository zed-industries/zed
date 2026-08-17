use core_foundation::{
    base::{CFTypeID, TCFType},
    date::CFTimeInterval,
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
use libc::c_double;

use crate::{
    event_types::{CGEventFlags, CGEventSourceKeyboardType, CGEventSourceRef, CGEventSourceStateID, CGEventType, CGMouseButton},
    remote_operation::CGKeyCode,
};

extern "C" {
    pub fn CGEventSourceGetTypeID() -> CFTypeID;
    pub fn CGEventSourceCreate(stateID: CGEventSourceStateID) -> CGEventSourceRef;
    pub fn CGEventSourceGetKeyboardType(source: CGEventSourceRef) -> CGEventSourceKeyboardType;
    pub fn CGEventSourceSetKeyboardType(source: CGEventSourceRef, keyboardType: CGEventSourceKeyboardType);
    pub fn CGEventSourceGetPixelsPerLine(source: CGEventSourceRef) -> c_double;
    pub fn CGEventSourceSetPixelsPerLine(source: CGEventSourceRef, pixelsPerLine: c_double);
    pub fn CGEventSourceGetSourceStateID(source: CGEventSourceRef) -> CGEventSourceStateID;
    pub fn CGEventSourceButtonState(stateID: CGEventSourceStateID, button: CGMouseButton) -> bool;
    pub fn CGEventSourceKeyState(stateID: CGEventSourceStateID, key: CGKeyCode) -> bool;
    pub fn CGEventSourceFlagsState(stateID: CGEventSourceStateID) -> CGEventFlags;
    pub fn CGEventSourceSecondsSinceLastEventType(stateID: CGEventSourceStateID, eventType: CGEventType) -> CFTimeInterval;
    pub fn CGEventSourceCounterForEventType(stateID: CGEventSourceStateID, eventType: CGEventType) -> u32;
    pub fn CGEventSourceSetUserData(source: CGEventSourceRef, userData: i64);
    pub fn CGEventSourceGetUserData(source: CGEventSourceRef) -> i64;
    pub fn CGEventSourceSetLocalEventsSuppressionInterval(source: CGEventSourceRef, seconds: CFTimeInterval);
    pub fn CGEventSourceGetLocalEventsSuppressionInterval(source: CGEventSourceRef) -> CFTimeInterval;
}

declare_TCFType!(CGEventSource, CGEventSourceRef);
impl_TCFType!(CGEventSource, CGEventSourceRef, CGEventSourceGetTypeID);
impl_CFTypeDescription!(CGEventSource);

impl CGEventSource {
    pub fn new(state_id: CGEventSourceStateID) -> Result<Self, ()> {
        unsafe {
            let event_source = CGEventSourceCreate(state_id);
            if event_source.is_null() {
                Err(())
            } else {
                Ok(TCFType::wrap_under_create_rule(event_source))
            }
        }
    }

    pub fn get_keyboard_type(&self) -> CGEventSourceKeyboardType {
        unsafe { CGEventSourceGetKeyboardType(self.as_concrete_TypeRef()) }
    }

    pub fn set_keyboard_type(&self, keyboard_type: CGEventSourceKeyboardType) {
        unsafe { CGEventSourceSetKeyboardType(self.as_concrete_TypeRef(), keyboard_type) }
    }

    pub fn get_pixels_per_line(&self) -> c_double {
        unsafe { CGEventSourceGetPixelsPerLine(self.as_concrete_TypeRef()) }
    }

    pub fn set_pixels_per_line(&self, pixels_per_line: c_double) {
        unsafe { CGEventSourceSetPixelsPerLine(self.as_concrete_TypeRef(), pixels_per_line) }
    }

    pub fn get_source_state_id(&self) -> CGEventSourceStateID {
        unsafe { CGEventSourceGetSourceStateID(self.as_concrete_TypeRef()) }
    }

    pub fn button_state(state_id: CGEventSourceStateID, button: CGMouseButton) -> bool {
        unsafe { CGEventSourceButtonState(state_id, button) }
    }

    pub fn key_state(state_id: CGEventSourceStateID, key: CGKeyCode) -> bool {
        unsafe { CGEventSourceKeyState(state_id, key) }
    }

    pub fn flags_state(state_id: CGEventSourceStateID) -> CGEventFlags {
        unsafe { CGEventSourceFlagsState(state_id) }
    }

    pub fn seconds_since_last_event_type(state_id: CGEventSourceStateID, event_type: CGEventType) -> CFTimeInterval {
        unsafe { CGEventSourceSecondsSinceLastEventType(state_id, event_type) }
    }

    pub fn counter_for_event_type(state_id: CGEventSourceStateID, event_type: CGEventType) -> u32 {
        unsafe { CGEventSourceCounterForEventType(state_id, event_type) }
    }

    pub fn set_user_data(&self, user_data: i64) {
        unsafe { CGEventSourceSetUserData(self.as_concrete_TypeRef(), user_data) }
    }

    pub fn get_user_data(&self) -> i64 {
        unsafe { CGEventSourceGetUserData(self.as_concrete_TypeRef()) }
    }

    pub fn set_local_events_suppression_interval(&self, seconds: CFTimeInterval) {
        unsafe { CGEventSourceSetLocalEventsSuppressionInterval(self.as_concrete_TypeRef(), seconds) }
    }

    pub fn get_local_events_suppression_interval(&self) -> CFTimeInterval {
        unsafe { CGEventSourceGetLocalEventsSuppressionInterval(self.as_concrete_TypeRef()) }
    }
}
