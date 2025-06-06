use std::ffi::{CStr, c_void};

use objc::{msg_send, runtime::Object, sel, sel_impl};

use crate::PlatformKeyboardLayout;

use super::{
    TISCopyCurrentKeyboardLayoutInputSource, TISGetInputSourceProperty, kTISPropertyInputSourceID,
    kTISPropertyLocalizedName,
};

pub(crate) struct MacKeyboardLayout {
    id: String,
    name: String,
}

impl PlatformKeyboardLayout for MacKeyboardLayout {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl MacKeyboardLayout {
    pub(crate) fn new() -> Self {
        unsafe {
            let current_keyboard = TISCopyCurrentKeyboardLayoutInputSource();

            let id: *mut Object = TISGetInputSourceProperty(
                current_keyboard,
                kTISPropertyInputSourceID as *const c_void,
            );
            let id: *const std::os::raw::c_char = msg_send![id, UTF8String];
            let id = CStr::from_ptr(id).to_str().unwrap().to_string();

            let name: *mut Object = TISGetInputSourceProperty(
                current_keyboard,
                kTISPropertyLocalizedName as *const c_void,
            );
            let name: *const std::os::raw::c_char = msg_send![name, UTF8String];
            let name = CStr::from_ptr(name).to_str().unwrap().to_string();

            Self { id, name }
        }
    }
}
