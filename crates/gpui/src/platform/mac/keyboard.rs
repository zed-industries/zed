use std::ffi::{CStr, c_void};

use collections::HashMap;
use objc::{msg_send, runtime::Object, sel, sel_impl};

use crate::{Keystroke, PlatformKeyboardLayout, PlatformKeyboardMapper};

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
        let (keyboard, id) = get_keyboard_layout_id();
        let name = unsafe {
            let name: *mut Object =
                TISGetInputSourceProperty(keyboard, kTISPropertyLocalizedName as *const c_void);
            let name: *const std::os::raw::c_char = msg_send![name, UTF8String];
            CStr::from_ptr(name).to_str().unwrap().to_string()
        };

        Self { id, name }
    }
}

fn get_keyboard_layout_id() -> (*mut Object, String) {
    unsafe {
        let current_keyboard = TISCopyCurrentKeyboardLayoutInputSource();

        let id: *mut Object =
            TISGetInputSourceProperty(current_keyboard, kTISPropertyInputSourceID as *const c_void);
        let id: *const std::os::raw::c_char = msg_send![id, UTF8String];
        (
            current_keyboard,
            CStr::from_ptr(id).to_str().unwrap().to_string(),
        )
    }
}

pub(crate) struct MacKeyboardMapper {
    key_to_code: HashMap<char, u32>,
    code_to_shifted_key: HashMap<u32, char>,
}

impl MacKeyboardMapper {
    pub(crate) fn new() -> Self {
        let mut key_to_code = HashMap::default();
        let mut code_to_shifted_key = HashMap::default();

        // Populate the mappings here
        key_to_code.insert('a', 0);
        code_to_shifted_key.insert(0, 'A');

        Self {
            key_to_code,
            code_to_shifted_key,
        }
    }
}

impl PlatformKeyboardMapper for MacKeyboardMapper {
    fn vscode_keystroke_to_gpui_keystroke(&self, keystroke: Keystroke) -> Keystroke {
        keystroke
    }
}
