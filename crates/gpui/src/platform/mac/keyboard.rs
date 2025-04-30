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

// All typeable scan codes for the standard US keyboard layout, ANSI104
const TY_CODES: &[u16] = &[
    0x001d, // Digit 0
    0x0012, // Digit 1
    0x0013, // Digit 2
    0x0014, // Digit 3
    0x0015, // Digit 4
    0x0017, // Digit 5
    0x0016, // Digit 6
    0x001a, // Digit 7
    0x001c, // Digit 8
    0x0019, // Digit 9
    0x0032, // ` Tilde
    0x001b, // - Minus
    0x0018, // = Equal
    0x0021, // [ Left bracket
    0x001e, // ] Right bracket
    0x002a, // \ Backslash
    0x0029, // ; Semicolon
    0x0027, // ' Quote
    0x002b, // , Comma
    0x002f, // . Period
    0x002c, // / Slash
    0x0000, // a
    0x000b, // b
    0x0008, // c
    0x0002, // d
    0x000e, // e
    0x0003, // f
    0x0005, // g
    0x0004, // h
    0x0022, // i
    0x0026, // j
    0x0028, // k
    0x0025, // l
    0x002e, // m
    0x002d, // n
    0x001f, // o
    0x0023, // p
    0x000c, // q
    0x000f, // r
    0x0001, // s
    0x0011, // t
    0x0020, // u
    0x0009, // v
    0x000d, // w
    0x0007, // x
    0x0010, // y
    0x0006, // z
];
