use collections::HashMap;
use x11rb::{protocol::xkb::ConnectionExt, xcb_ffi::XCBConnection};
use xkbcommon::xkb::{
    Keycode,
    x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION},
};

use crate::{PlatformKeyboardLayout, PlatformKeyboardMapper, ScanCode, is_alphabetic_key};

pub(crate) struct LinuxKeyboardLayout {
    id: String,
}

impl PlatformKeyboardLayout for LinuxKeyboardLayout {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.id
    }
}

impl LinuxKeyboardLayout {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }
}

pub(crate) struct LinuxKeyboardMapper {
    code_to_key: HashMap<Keycode, String>,
    key_to_code: HashMap<String, Keycode>,
    code_to_shifted_key: HashMap<Keycode, String>,
}

impl PlatformKeyboardMapper for LinuxKeyboardMapper {
    fn scan_code_to_key(&self, scan_code: ScanCode) -> anyhow::Result<String> {
        // todo(linux)
        Ok(scan_code.to_key().to_string())
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
                "Shifted key not found for key {} with scan code: {:?}",
                key,
                scan_code
            ))
        }
    }
}

impl LinuxKeyboardMapper {
    pub(crate) fn new() -> Self {
        let (xcb_connection, _) = XCBConnection::connect(None).unwrap();
        let _ = xcb_connection
            .xkb_use_extension(XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION)
            .unwrap()
            .reply()
            .unwrap();
        let xkb_context = xkbcommon::xkb::Context::new(xkbcommon::xkb::CONTEXT_NO_FLAGS);
        let xkb_device_id = xkbcommon::xkb::x11::get_core_keyboard_device_id(&xcb_connection);
        let xkb_state = {
            let xkb_keymap = xkbcommon::xkb::x11::keymap_new_from_device(
                &xkb_context,
                &xcb_connection,
                xkb_device_id,
                xkbcommon::xkb::KEYMAP_COMPILE_NO_FLAGS,
            );
            xkbcommon::xkb::x11::state_new_from_device(&xkb_keymap, &xcb_connection, xkb_device_id)
        };
        let mut code_to_key = HashMap::default();
        let mut key_to_code = HashMap::default();
        let mut code_to_shifted_key = HashMap::default();

        let keymap = xkb_state.get_keymap();
        let mut shifted_state = xkbcommon::xkb::State::new(&keymap);

        let shift_mod = keymap.mod_get_index(xkbcommon::xkb::MOD_NAME_SHIFT);
        let shift_mask = 1 << shift_mod;
        shifted_state.update_mask(shift_mask, 0, 0, 0, 0, 0);

        for &scan_code in TYPEABLE_CODES {
            let keycode = Keycode::new(scan_code);
            let key = xkb_state.key_get_utf8(keycode);
            code_to_key.insert(keycode, key.clone());
            key_to_code.insert(key, keycode);

            let shifted_key = shifted_state.key_get_utf8(keycode);
            code_to_shifted_key.insert(keycode, shifted_key);
        }

        Self {
            code_to_key,
            key_to_code,
            code_to_shifted_key,
        }
    }
}

// All typeable scan codes for the standard US keyboard layout, ANSI104
const TYPEABLE_CODES: &[u32] = &[
    0x0026, // a
    0x0038, // b
    0x0036, // c
    0x0028, // d
    0x001a, // e
    0x0029, // f
    0x002a, // g
    0x002b, // h
    0x001f, // i
    0x002c, // j
    0x002d, // k
    0x002e, // l
    0x003a, // m
    0x0039, // n
    0x0020, // o
    0x0021, // p
    0x0018, // q
    0x001b, // r
    0x0027, // s
    0x001c, // t
    0x001e, // u
    0x0037, // v
    0x0019, // w
    0x0035, // x
    0x001d, // y
    0x0034, // z
    0x0013, // Digit 0
    0x000a, // Digit 1
    0x000b, // Digit 2
    0x000c, // Digit 3
    0x000d, // Digit 4
    0x000e, // Digit 5
    0x000f, // Digit 6
    0x0010, // Digit 7
    0x0011, // Digit 8
    0x0012, // Digit 9
    0x0031, // ` Backquote
    0x0014, // - Minus
    0x0015, // = Equal
    0x0022, // [ Left bracket
    0x0023, // ] Right bracket
    0x0033, // \ Backslash
    0x002f, // ; Semicolon
    0x0030, // ' Quote
    0x003b, // , Comma
    0x003c, // . Period
    0x003d, // / Slash
];
