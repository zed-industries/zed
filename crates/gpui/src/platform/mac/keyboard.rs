use std::ffi::{CStr, c_void};

use collections::HashMap;
use core_foundation::data::{CFDataGetBytePtr, CFDataRef};
use core_graphics::event::CGKeyCode;
use objc::{msg_send, runtime::Object, sel, sel_impl};

use crate::{
    PlatformKeyboardLayout, PlatformKeyboardMapper, ScanCode, is_alphabetic_key,
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
    code_to_key: HashMap<u16, String>,
    key_to_code: HashMap<String, u16>,
    code_to_shifted_key: HashMap<u16, String>,
}

impl MacKeyboardMapper {
    pub(crate) fn new() -> Self {
        let mut code_to_key = HashMap::default();
        let mut key_to_code = HashMap::default();
        let mut code_to_shifted_key = HashMap::default();

        let always_use_cmd_layout = always_use_command_layout();
        for &scan_code in TYPEABLE_CODES.iter() {
            let (key, shifted_key) = generate_key_pairs(scan_code, always_use_cmd_layout);
            code_to_key.insert(scan_code, key.clone());
            key_to_code.insert(key, scan_code);
            code_to_shifted_key.insert(scan_code, shifted_key);
        }

        Self {
            code_to_key,
            key_to_code,
            code_to_shifted_key,
        }
    }
}

impl PlatformKeyboardMapper for MacKeyboardMapper {
    fn scan_code_to_key(&self, gpui_scan_code: ScanCode) -> anyhow::Result<String> {
        if let Some(key) = gpui_scan_code.try_to_key() {
            return Ok(key);
        }
        let Some(scan_code) = get_scan_code(gpui_scan_code) else {
            return Err(anyhow::anyhow!("Scan code not found: {:?}", gpui_scan_code));
        };
        if let Some(key) = self.code_to_key.get(&scan_code) {
            Ok(key.clone())
        } else {
            Err(anyhow::anyhow!(
                "Key not found for input scan code: {:?}, scan code: {}",
                gpui_scan_code,
                scan_code
            ))
        }
    }

    fn get_shifted_key(&self, key: &str) -> anyhow::Result<Option<String>> {
        if key.chars().count() != 1 {
            return Ok(None);
        }
        if is_alphabetic_key(key) {
            return Ok(Some(key.to_uppercase()));
        }
        let Some(scan_code) = self.key_to_code.get(key) else {
            return Err(anyhow::anyhow!("Key not found: {}", key));
        };
        if let Some(shifted_key) = self.code_to_shifted_key.get(scan_code) {
            Ok(Some(shifted_key.clone()))
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

pub(crate) fn always_use_command_layout() -> bool {
    if chars_for_modified_key(0, NO_MOD).is_ascii() {
        return false;
    }

    chars_for_modified_key(0, CMD_MOD).is_ascii()
}

fn generate_key_pairs(scan_code: u16, always_use_cmd_layout: bool) -> (String, String) {
    let mut chars_ignoring_modifiers = chars_for_modified_key(scan_code, NO_MOD);
    let mut chars_with_shift = chars_for_modified_key(scan_code, SHIFT_MOD);

    // Handle Dvorak+QWERTY / Russian / Armenian
    if always_use_cmd_layout {
        let chars_with_cmd = chars_for_modified_key(scan_code, CMD_MOD);
        let chars_with_both = chars_for_modified_key(scan_code, CMD_MOD | SHIFT_MOD);

        // We don't do this in the case that the shifted command key generates
        // the same character as the unshifted command key (Norwegian, e.g.)
        if chars_with_both != chars_with_cmd {
            chars_with_shift = chars_with_both;

        // Handle edge-case where cmd-shift-s reports cmd-s instead of
        // cmd-shift-s (Ukrainian, etc.)
        } else if chars_with_cmd.to_ascii_uppercase() != chars_with_cmd {
            chars_with_shift = chars_with_cmd.to_ascii_uppercase();
        }
        chars_ignoring_modifiers = chars_with_cmd;
    }
    (chars_ignoring_modifiers, chars_with_shift)
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

fn get_scan_code(scan_code: ScanCode) -> Option<u16> {
    // https://github.com/microsoft/node-native-keymap/blob/main/deps/chromium/dom_code_data.inc
    Some(match scan_code {
        ScanCode::F1 => 0x007a,
        ScanCode::F2 => 0x0078,
        ScanCode::F3 => 0x0063,
        ScanCode::F4 => 0x0076,
        ScanCode::F5 => 0x0060,
        ScanCode::F6 => 0x0061,
        ScanCode::F7 => 0x0062,
        ScanCode::F8 => 0x0064,
        ScanCode::F9 => 0x0065,
        ScanCode::F10 => 0x006d,
        ScanCode::F11 => 0x0067,
        ScanCode::F12 => 0x006f,
        ScanCode::F13 => 0x0069,
        ScanCode::F14 => 0x006b,
        ScanCode::F15 => 0x0071,
        ScanCode::F16 => 0x006a,
        ScanCode::F17 => 0x0040,
        ScanCode::F18 => 0x004f,
        ScanCode::F19 => 0x0050,
        ScanCode::F20 => 0x005a,
        ScanCode::F21 | ScanCode::F22 | ScanCode::F23 | ScanCode::F24 => return None,
        ScanCode::A => 0x0000,
        ScanCode::B => 0x000b,
        ScanCode::C => 0x0008,
        ScanCode::D => 0x0002,
        ScanCode::E => 0x000e,
        ScanCode::F => 0x0003,
        ScanCode::G => 0x0005,
        ScanCode::H => 0x0004,
        ScanCode::I => 0x0022,
        ScanCode::J => 0x0026,
        ScanCode::K => 0x0028,
        ScanCode::L => 0x0025,
        ScanCode::M => 0x002e,
        ScanCode::N => 0x002d,
        ScanCode::O => 0x001f,
        ScanCode::P => 0x0023,
        ScanCode::Q => 0x000c,
        ScanCode::R => 0x000f,
        ScanCode::S => 0x0001,
        ScanCode::T => 0x0011,
        ScanCode::U => 0x0020,
        ScanCode::V => 0x0009,
        ScanCode::W => 0x000d,
        ScanCode::X => 0x0007,
        ScanCode::Y => 0x0010,
        ScanCode::Z => 0x0006,
        ScanCode::Digit0 => 0x001d,
        ScanCode::Digit1 => 0x0012,
        ScanCode::Digit2 => 0x0013,
        ScanCode::Digit3 => 0x0014,
        ScanCode::Digit4 => 0x0015,
        ScanCode::Digit5 => 0x0017,
        ScanCode::Digit6 => 0x0016,
        ScanCode::Digit7 => 0x001a,
        ScanCode::Digit8 => 0x001c,
        ScanCode::Digit9 => 0x0019,
        ScanCode::Backquote => 0x0032,
        ScanCode::Minus => 0x001b,
        ScanCode::Equal => 0x0018,
        ScanCode::BracketLeft => 0x0021,
        ScanCode::BracketRight => 0x001e,
        ScanCode::Backslash => 0x002a,
        ScanCode::Semicolon => 0x0029,
        ScanCode::Quote => 0x0027,
        ScanCode::Comma => 0x002b,
        ScanCode::Period => 0x002f,
        ScanCode::Slash => 0x002c,
        ScanCode::Left => 0x007b,
        ScanCode::Up => 0x007e,
        ScanCode::Right => 0x007c,
        ScanCode::Down => 0x007d,
        ScanCode::PageUp => 0x0074,
        ScanCode::PageDown => 0x0079,
        ScanCode::End => 0x0077,
        ScanCode::Home => 0x0073,
        ScanCode::Tab => 0x0030,
        ScanCode::Enter => 0x0024,
        ScanCode::Escape => 0x0035,
        ScanCode::Space => 0x0031,
        ScanCode::Backspace => 0x0033,
        ScanCode::Delete => 0x0075,
        ScanCode::Insert => 0x0072,
    })
}
