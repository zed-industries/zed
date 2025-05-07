#[cfg(any(feature = "wayland", feature = "x11"))]
use std::sync::LazyLock;

#[cfg(any(feature = "wayland", feature = "x11"))]
use collections::HashMap;
#[cfg(any(feature = "wayland", feature = "x11"))]
use x11rb::{protocol::xkb::ConnectionExt, xcb_ffi::XCBConnection};
#[cfg(any(feature = "wayland", feature = "x11"))]
use xkbcommon::xkb::{
    Keycode,
    x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION},
};

use crate::{PlatformKeyboardLayout, PlatformKeyboardMapper, ScanCode};

#[cfg(any(feature = "wayland", feature = "x11"))]
use crate::is_alphabetic_key;

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

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) struct LinuxKeyboardMapper {
    code_to_key: HashMap<Keycode, String>,
    key_to_code: HashMap<String, Keycode>,
    code_to_shifted_key: HashMap<Keycode, String>,
}

#[cfg(any(feature = "wayland", feature = "x11"))]
impl PlatformKeyboardMapper for LinuxKeyboardMapper {
    fn scan_code_to_key(&self, gpui_scan_code: ScanCode) -> anyhow::Result<String> {
        if let Some(key) = gpui_scan_code.try_to_key() {
            return Ok(key);
        }
        let Some(scan_code) = get_scan_code(gpui_scan_code) else {
            return Err(anyhow::anyhow!("Scan code not found: {:?}", gpui_scan_code));
        };
        if let Some(key) = self.code_to_key.get(&Keycode::new(scan_code)) {
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
        if is_immutable_key(key) {
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

#[cfg(any(feature = "wayland", feature = "x11"))]
static XCB_CONNECTION: LazyLock<XCBConnection> =
    LazyLock::new(|| XCBConnection::connect(None).unwrap().0);

#[cfg(any(feature = "wayland", feature = "x11"))]
impl LinuxKeyboardMapper {
    pub(crate) fn new() -> Self {
        let _ = XCB_CONNECTION
            .xkb_use_extension(XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION)
            .unwrap()
            .reply()
            .unwrap();
        let xkb_context = xkbcommon::xkb::Context::new(xkbcommon::xkb::CONTEXT_NO_FLAGS);
        let xkb_device_id = xkbcommon::xkb::x11::get_core_keyboard_device_id(&*XCB_CONNECTION);
        let xkb_state = {
            let xkb_keymap = xkbcommon::xkb::x11::keymap_new_from_device(
                &xkb_context,
                &*XCB_CONNECTION,
                xkb_device_id,
                xkbcommon::xkb::KEYMAP_COMPILE_NO_FLAGS,
            );
            xkbcommon::xkb::x11::state_new_from_device(&xkb_keymap, &*XCB_CONNECTION, xkb_device_id)
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
#[cfg(any(feature = "wayland", feature = "x11"))]
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

#[cfg(any(feature = "wayland", feature = "x11"))]
fn is_immutable_key(key: &str) -> bool {
    matches!(
        key,
        "f1" | "f2"
            | "f3"
            | "f4"
            | "f5"
            | "f6"
            | "f7"
            | "f8"
            | "f9"
            | "f10"
            | "f11"
            | "f12"
            | "f13"
            | "f14"
            | "f15"
            | "f16"
            | "f17"
            | "f18"
            | "f19"
            | "f20"
            | "f21"
            | "f22"
            | "f23"
            | "f24"
            | "backspace"
            | "delete"
            | "left"
            | "right"
            | "up"
            | "down"
            | "pageup"
            | "pagedown"
            | "insert"
            | "home"
            | "end"
            | "back"
            | "forward"
            | "escape"
            | "space"
            | "tab"
            | "enter"
            | "shift"
            | "control"
            | "alt"
            | "platform"
            | "cmd"
            | "super"
            | "win"
            | "fn"
            | "menu"
            | "copy"
            | "paste"
            | "cut"
            | "find"
            | "open"
            | "save"
    )
}

#[cfg(any(feature = "wayland", feature = "x11"))]
fn get_scan_code(scan_code: ScanCode) -> Option<u32> {
    // https://github.com/microsoft/node-native-keymap/blob/main/deps/chromium/dom_code_data.inc
    Some(match scan_code {
        ScanCode::F1 => 0x0043,
        ScanCode::F2 => 0x0044,
        ScanCode::F3 => 0x0045,
        ScanCode::F4 => 0x0046,
        ScanCode::F5 => 0x0047,
        ScanCode::F6 => 0x0048,
        ScanCode::F7 => 0x0049,
        ScanCode::F8 => 0x004a,
        ScanCode::F9 => 0x004b,
        ScanCode::F10 => 0x004c,
        ScanCode::F11 => 0x005f,
        ScanCode::F12 => 0x0060,
        ScanCode::F13 => 0x00bf,
        ScanCode::F14 => 0x00c0,
        ScanCode::F15 => 0x00c1,
        ScanCode::F16 => 0x00c2,
        ScanCode::F17 => 0x00c3,
        ScanCode::F18 => 0x00c4,
        ScanCode::F19 => 0x00c5,
        ScanCode::F20 => 0x00c6,
        ScanCode::F21 => 0x00c7,
        ScanCode::F22 => 0x00c8,
        ScanCode::F23 => 0x00c9,
        ScanCode::F24 => 0x00ca,
        ScanCode::A => 0x0026,
        ScanCode::B => 0x0038,
        ScanCode::C => 0x0036,
        ScanCode::D => 0x0028,
        ScanCode::E => 0x001a,
        ScanCode::F => 0x0029,
        ScanCode::G => 0x002a,
        ScanCode::H => 0x002b,
        ScanCode::I => 0x001f,
        ScanCode::J => 0x002c,
        ScanCode::K => 0x002d,
        ScanCode::L => 0x002e,
        ScanCode::M => 0x003a,
        ScanCode::N => 0x0039,
        ScanCode::O => 0x0020,
        ScanCode::P => 0x0021,
        ScanCode::Q => 0x0018,
        ScanCode::R => 0x001b,
        ScanCode::S => 0x0027,
        ScanCode::T => 0x001c,
        ScanCode::U => 0x001e,
        ScanCode::V => 0x0037,
        ScanCode::W => 0x0019,
        ScanCode::X => 0x0035,
        ScanCode::Y => 0x001d,
        ScanCode::Z => 0x0034,
        ScanCode::Digit0 => 0x0013,
        ScanCode::Digit1 => 0x000a,
        ScanCode::Digit2 => 0x000b,
        ScanCode::Digit3 => 0x000c,
        ScanCode::Digit4 => 0x000d,
        ScanCode::Digit5 => 0x000e,
        ScanCode::Digit6 => 0x000f,
        ScanCode::Digit7 => 0x0010,
        ScanCode::Digit8 => 0x0011,
        ScanCode::Digit9 => 0x0012,
        ScanCode::Backquote => 0x0031,
        ScanCode::Minus => 0x0014,
        ScanCode::Equal => 0x0015,
        ScanCode::BracketLeft => 0x0022,
        ScanCode::BracketRight => 0x0023,
        ScanCode::Backslash => 0x0033,
        ScanCode::Semicolon => 0x002f,
        ScanCode::Quote => 0x0030,
        ScanCode::Comma => 0x003b,
        ScanCode::Period => 0x003c,
        ScanCode::Slash => 0x003d,
        ScanCode::Left => 0x0071,
        ScanCode::Up => 0x006f,
        ScanCode::Right => 0x0072,
        ScanCode::Down => 0x0074,
        ScanCode::PageUp => 0x0070,
        ScanCode::PageDown => 0x0075,
        ScanCode::End => 0x0073,
        ScanCode::Home => 0x006e,
        ScanCode::Tab => 0x0017,
        ScanCode::Enter => 0x0024,
        ScanCode::Escape => 0x0009,
        ScanCode::Space => 0x0041,
        ScanCode::Backspace => 0x0016,
        ScanCode::Delete => 0x0077,
        ScanCode::Insert => 0x0076,
    })
}

#[cfg(not(any(feature = "wayland", feature = "x11")))]
pub(crate) struct LinuxKeyboardMapper;

#[cfg(not(any(feature = "wayland", feature = "x11")))]
impl PlatformKeyboardMapper for LinuxKeyboardMapper {
    fn scan_code_to_key(&self, _scan_code: ScanCode) -> anyhow::Result<String> {
        Err(anyhow::anyhow!("LinuxKeyboardMapper not supported"))
    }

    fn get_shifted_key(&self, _key: &str) -> anyhow::Result<Option<String>> {
        Err(anyhow::anyhow!("LinuxKeyboardMapper not supported"))
    }
}

#[cfg(not(any(feature = "wayland", feature = "x11")))]
impl LinuxKeyboardMapper {
    pub(crate) fn new() -> Self {
        Self
    }
}
