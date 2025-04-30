use std::ffi::{CStr, c_void};

use collections::HashMap;
use core_foundation::data::{CFDataGetBytePtr, CFDataRef};
use core_graphics::event::CGKeyCode;
use objc::{msg_send, runtime::Object, sel, sel_impl};

use crate::{
    PlatformKeyboardLayout, PlatformKeyboardMapper,
    platform::mac::{LMGetKbdType, UCKeyTranslate, kTISPropertyUnicodeKeyLayoutData},
};

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
    key_to_code: HashMap<String, u16>,
    code_to_shifted_key: HashMap<u16, String>,
}

impl MacKeyboardMapper {
    pub(crate) fn new() -> Self {
        let mut key_to_code = HashMap::default();
        let mut code_to_shifted_key = HashMap::default();

        for &scan_code in TYPEABLE_CODES.iter() {
            let key = chars_for_modified_key(scan_code, NO_MOD);
            if !key.is_empty() {
                key_to_code.insert(key, scan_code);
            }
            let shifted_key = chars_for_modified_key(scan_code, SHIFT_MOD);
            if !shifted_key.is_empty() {
                code_to_shifted_key.insert(scan_code, shifted_key);
            }
        }

        Self {
            key_to_code,
            code_to_shifted_key,
        }
    }
}

impl PlatformKeyboardMapper for MacKeyboardMapper {
    fn get_shifted_key(&self, key: &str) -> anyhow::Result<String> {
        if is_alphabetic_key(key) {
            return Ok(key.to_uppercase());
        }
        let Some(scan_code) = self.key_to_code.get(key) else {
            return Err(anyhow::anyhow!("Key not found: {}", key));
        };
        if let Some(shifted_key) = self.code_to_shifted_key.get(scan_code) {
            Ok(shifted_key.clone())
        } else {
            Err(anyhow::anyhow!(
                "Shifted key not found for key {} with scan code: {}",
                key,
                scan_code
            ))
        }
    }
}

pub(crate) const NO_MOD: u32 = 0;
pub(crate) const CMD_MOD: u32 = 1;
pub(crate) const SHIFT_MOD: u32 = 2;
pub(crate) const OPTION_MOD: u32 = 8;

pub(crate) fn chars_for_modified_key(code: CGKeyCode, modifiers: u32) -> String {
    // Values from: https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.6.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h#L126
    // shifted >> 8 for UCKeyTranslate
    const CG_SPACE_KEY: u16 = 49;
    // https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.6.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/Headers/UnicodeUtilities.h#L278
    #[allow(non_upper_case_globals)]
    const kUCKeyActionDown: u16 = 0;
    #[allow(non_upper_case_globals)]
    const kUCKeyTranslateNoDeadKeysMask: u32 = 0;

    let keyboard_type = unsafe { LMGetKbdType() as u32 };
    const BUFFER_SIZE: usize = 4;
    let mut dead_key_state = 0;
    let mut buffer: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut buffer_size: usize = 0;

    let keyboard = unsafe { TISCopyCurrentKeyboardLayoutInputSource() };
    if keyboard.is_null() {
        return "".to_string();
    }
    let layout_data = unsafe {
        TISGetInputSourceProperty(keyboard, kTISPropertyUnicodeKeyLayoutData as *const c_void)
            as CFDataRef
    };
    if layout_data.is_null() {
        unsafe {
            let _: () = msg_send![keyboard, release];
        }
        return "".to_string();
    }
    let keyboard_layout = unsafe { CFDataGetBytePtr(layout_data) };

    unsafe {
        UCKeyTranslate(
            keyboard_layout as *const c_void,
            code,
            kUCKeyActionDown,
            modifiers,
            keyboard_type,
            kUCKeyTranslateNoDeadKeysMask,
            &mut dead_key_state,
            BUFFER_SIZE,
            &mut buffer_size as *mut usize,
            &mut buffer as *mut u16,
        );
        if dead_key_state != 0 {
            UCKeyTranslate(
                keyboard_layout as *const c_void,
                CG_SPACE_KEY,
                kUCKeyActionDown,
                modifiers,
                keyboard_type,
                kUCKeyTranslateNoDeadKeysMask,
                &mut dead_key_state,
                BUFFER_SIZE,
                &mut buffer_size as *mut usize,
                &mut buffer as *mut u16,
            );
        }
        let _: () = msg_send![keyboard, release];
    }
    String::from_utf16(&buffer[..buffer_size]).unwrap_or_default()
}

fn is_alphabetic_key(key: &str) -> bool {
    matches!(
        key,
        "a" | "b"
            | "c"
            | "d"
            | "e"
            | "f"
            | "g"
            | "h"
            | "i"
            | "j"
            | "k"
            | "l"
            | "m"
            | "n"
            | "o"
            | "p"
            | "q"
            | "r"
            | "s"
            | "t"
            | "u"
            | "v"
            | "w"
            | "x"
            | "y"
            | "z"
    )
}

// All typeable scan codes for the standard US keyboard layout, ANSI104
const TYPEABLE_CODES: &[u16] = &[
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
];

#[cfg(test)]
mod tests {
    use crate::PlatformKeyboardMapper;

    use super::MacKeyboardMapper;

    #[test]
    fn test_get_shifted_key() {
        let mapper = MacKeyboardMapper::new();

        for ch in 'a'..'z' {
            let key = ch.to_string();
            let shifted_key = key.to_uppercase();
            assert_eq!(mapper.get_shifted_key(&key).unwrap(), shifted_key);
        }

        let shift_pairs = [
            ("1", "!"),
            ("2", "@"),
            ("3", "#"),
            ("4", "$"),
            ("5", "%"),
            ("6", "^"),
            ("7", "&"),
            ("8", "*"),
            ("9", "("),
            ("0", ")"),
            ("`", "~"),
            ("-", "_"),
            ("=", "+"),
            ("[", "{"),
            ("]", "}"),
            ("\\", "|"),
            (";", ":"),
            ("'", "\""),
            (",", "<"),
            (".", ">"),
            ("/", "?"),
        ];
        for (key, shifted_key) in shift_pairs {
            assert_eq!(mapper.get_shifted_key(key).unwrap(), shifted_key);
        }
    }
}
