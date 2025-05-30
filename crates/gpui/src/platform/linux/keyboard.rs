#[cfg(any(feature = "wayland", feature = "x11"))]
use std::sync::LazyLock;

#[cfg(any(feature = "wayland", feature = "x11"))]
use collections::{HashMap, HashSet};
#[cfg(any(feature = "wayland", feature = "x11"))]
use strum::{EnumIter, IntoEnumIterator as _};
#[cfg(any(feature = "wayland", feature = "x11"))]
use x11rb::{protocol::xkb::ConnectionExt as _, xcb_ffi::XCBConnection};
#[cfg(any(feature = "wayland", feature = "x11"))]
use xkbcommon::xkb::{
    Keycode, Keysym, STATE_LAYOUT_EFFECTIVE, State,
    x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION},
};

use crate::PlatformKeyboardLayout;

#[derive(Debug, Clone)]
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
    #[cfg(any(feature = "wayland", feature = "x11"))]
    pub(crate) fn new(state: &State) -> Self {
        let layout_idx = state.serialize_layout(STATE_LAYOUT_EFFECTIVE);
        let id = state.get_keymap().layout_get_name(layout_idx).to_string();
        Self { id }
    }

    pub(crate) fn unknown() -> Self {
        Self {
            id: "unknown".to_string(),
        }
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
static XCB_CONNECTION: LazyLock<XCBConnection> =
    LazyLock::new(|| XCBConnection::connect(None).unwrap().0);

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) struct LinuxKeyboardMapper {
    letters: HashMap<Keycode, String>,
    code_to_key: HashMap<Keycode, String>,
    code_to_shifted_key: HashMap<Keycode, String>,
}

#[cfg(any(feature = "wayland", feature = "x11"))]
impl LinuxKeyboardMapper {
    pub(crate) fn new(base_group: u32, latched_group: u32, locked_group: u32) -> Self {
        let _ = XCB_CONNECTION
            .xkb_use_extension(XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION)
            .unwrap()
            .reply()
            .unwrap();
        let xkb_context = xkbcommon::xkb::Context::new(xkbcommon::xkb::CONTEXT_NO_FLAGS);
        let xkb_device_id = xkbcommon::xkb::x11::get_core_keyboard_device_id(&*XCB_CONNECTION);
        let mut xkb_state = {
            let xkb_keymap = xkbcommon::xkb::x11::keymap_new_from_device(
                &xkb_context,
                &*XCB_CONNECTION,
                xkb_device_id,
                xkbcommon::xkb::KEYMAP_COMPILE_NO_FLAGS,
            );
            xkbcommon::xkb::x11::state_new_from_device(&xkb_keymap, &*XCB_CONNECTION, xkb_device_id)
        };
        xkb_state.update_mask(0, 0, 0, base_group, latched_group, locked_group);

        let mut letters = HashMap::default();
        let mut code_to_key = HashMap::default();
        let mut code_to_shifted_key = HashMap::default();
        let mut inserted_letters = HashSet::default();

        let keymap = xkb_state.get_keymap();
        let mut shifted_state = xkbcommon::xkb::State::new(&keymap);
        let shift_mod = keymap.mod_get_index(xkbcommon::xkb::MOD_NAME_SHIFT);
        let shift_mask = 1 << shift_mod;
        shifted_state.update_mask(shift_mask, 0, 0, base_group, latched_group, locked_group);

        for scan_code in LinuxScanCodes::iter() {
            let keycode = Keycode::new(scan_code as u32);

            let key = xkb_state.key_get_utf8(keycode);
            if !key.is_empty() {
                if key_is_a_letter(&key) {
                    letters.insert(keycode, key.clone());
                } else {
                    code_to_key.insert(keycode, key.clone());
                }
                inserted_letters.insert(key);
            } else {
                // keycode might be a dead key
                let keysym = xkb_state.key_get_one_sym(keycode);
                if let Some(key) = underlying_dead_key(keysym) {
                    code_to_key.insert(keycode, key.clone());
                    inserted_letters.insert(key);
                }
            }

            let shifted_key = shifted_state.key_get_utf8(keycode);
            if !shifted_key.is_empty() {
                code_to_shifted_key.insert(keycode, shifted_key);
            } else {
                // keycode might be a dead key
                let shifted_keysym = shifted_state.key_get_one_sym(keycode);
                if let Some(shifted_key) = underlying_dead_key(shifted_keysym) {
                    code_to_shifted_key.insert(keycode, shifted_key);
                }
            }
        }
        insert_letters_if_missing(&inserted_letters, &mut letters);

        Self {
            letters,
            code_to_key,
            code_to_shifted_key,
        }
    }

    pub(crate) fn get_key(
        &self,
        keycode: Keycode,
        modifiers: &mut crate::Modifiers,
    ) -> Option<String> {
        if let Some(key) = self.letters.get(&keycode) {
            return Some(key.clone());
        }
        if modifiers.shift {
            modifiers.shift = false;
            self.code_to_shifted_key.get(&keycode).cloned()
        } else {
            self.code_to_key.get(&keycode).cloned()
        }
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
fn key_is_a_letter(key: &str) -> bool {
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

/**
 * Returns which symbol the dead key represents
 * <https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values#dead_keycodes_for_linux>
 */
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) fn underlying_dead_key(keysym: Keysym) -> Option<String> {
    match keysym {
        Keysym::dead_grave => Some("`".to_owned()),
        Keysym::dead_acute => Some("´".to_owned()),
        Keysym::dead_circumflex => Some("^".to_owned()),
        Keysym::dead_tilde => Some("~".to_owned()),
        Keysym::dead_macron => Some("¯".to_owned()),
        Keysym::dead_breve => Some("˘".to_owned()),
        Keysym::dead_abovedot => Some("˙".to_owned()),
        Keysym::dead_diaeresis => Some("¨".to_owned()),
        Keysym::dead_abovering => Some("˚".to_owned()),
        Keysym::dead_doubleacute => Some("˝".to_owned()),
        Keysym::dead_caron => Some("ˇ".to_owned()),
        Keysym::dead_cedilla => Some("¸".to_owned()),
        Keysym::dead_ogonek => Some("˛".to_owned()),
        Keysym::dead_iota => Some("ͅ".to_owned()),
        Keysym::dead_voiced_sound => Some("゙".to_owned()),
        Keysym::dead_semivoiced_sound => Some("゚".to_owned()),
        Keysym::dead_belowdot => Some("̣̣".to_owned()),
        Keysym::dead_hook => Some("̡".to_owned()),
        Keysym::dead_horn => Some("̛".to_owned()),
        Keysym::dead_stroke => Some("̶̶".to_owned()),
        Keysym::dead_abovecomma => Some("̓̓".to_owned()),
        Keysym::dead_abovereversedcomma => Some("ʽ".to_owned()),
        Keysym::dead_doublegrave => Some("̏".to_owned()),
        Keysym::dead_belowring => Some("˳".to_owned()),
        Keysym::dead_belowmacron => Some("̱".to_owned()),
        Keysym::dead_belowcircumflex => Some("ꞈ".to_owned()),
        Keysym::dead_belowtilde => Some("̰".to_owned()),
        Keysym::dead_belowbreve => Some("̮".to_owned()),
        Keysym::dead_belowdiaeresis => Some("̤".to_owned()),
        Keysym::dead_invertedbreve => Some("̯".to_owned()),
        Keysym::dead_belowcomma => Some("̦".to_owned()),
        Keysym::dead_currency => None,
        Keysym::dead_lowline => None,
        Keysym::dead_aboveverticalline => None,
        Keysym::dead_belowverticalline => None,
        Keysym::dead_longsolidusoverlay => None,
        Keysym::dead_a => None,
        Keysym::dead_A => None,
        Keysym::dead_e => None,
        Keysym::dead_E => None,
        Keysym::dead_i => None,
        Keysym::dead_I => None,
        Keysym::dead_o => None,
        Keysym::dead_O => None,
        Keysym::dead_u => None,
        Keysym::dead_U => None,
        Keysym::dead_small_schwa => Some("ə".to_owned()),
        Keysym::dead_capital_schwa => Some("Ə".to_owned()),
        Keysym::dead_greek => None,
        _ => None,
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
fn insert_letters_if_missing(inserted: &HashSet<String>, letters: &mut HashMap<Keycode, String>) {
    for scan_code in LinuxScanCodes::LETTERS.iter() {
        let keycode = Keycode::new(*scan_code as u32);
        let key = scan_code.to_str();
        if !inserted.contains(key) {
            letters.insert(keycode, key.to_owned());
        }
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum LinuxScanCodes {
    A = 0x0026,
    B = 0x0038,
    C = 0x0036,
    D = 0x0028,
    E = 0x001a,
    F = 0x0029,
    G = 0x002a,
    H = 0x002b,
    I = 0x001f,
    J = 0x002c,
    K = 0x002d,
    L = 0x002e,
    M = 0x003a,
    N = 0x0039,
    O = 0x0020,
    P = 0x0021,
    Q = 0x0018,
    R = 0x001b,
    S = 0x0027,
    T = 0x001c,
    U = 0x001e,
    V = 0x0037,
    W = 0x0019,
    X = 0x0035,
    Y = 0x001d,
    Z = 0x0034,
    Digit0 = 0x0013,
    Digit1 = 0x000a,
    Digit2 = 0x000b,
    Digit3 = 0x000c,
    Digit4 = 0x000d,
    Digit5 = 0x000e,
    Digit6 = 0x000f,
    Digit7 = 0x0010,
    Digit8 = 0x0011,
    Digit9 = 0x0012,
    Backquote = 0x0031,
    Minus = 0x0014,
    Equal = 0x0015,
    LeftBracket = 0x0022,
    RightBracket = 0x0023,
    Backslash = 0x0033,
    Semicolon = 0x002f,
    Quote = 0x0030,
    Comma = 0x003b,
    Period = 0x003c,
    Slash = 0x003d,
    // This key is typically located near LeftShift key, varies on international keyboards: Dan: <> Dutch: ][ Ger: <> UK: \|
    IntlBackslash = 0x005e,
    // Used for Brazilian /? and Japanese _ 'ro'.
    IntlRo = 0x0061,
}

#[cfg(any(feature = "wayland", feature = "x11"))]
impl LinuxScanCodes {
    const LETTERS: &'static [LinuxScanCodes] = &[
        LinuxScanCodes::A,
        LinuxScanCodes::B,
        LinuxScanCodes::C,
        LinuxScanCodes::D,
        LinuxScanCodes::E,
        LinuxScanCodes::F,
        LinuxScanCodes::G,
        LinuxScanCodes::H,
        LinuxScanCodes::I,
        LinuxScanCodes::J,
        LinuxScanCodes::K,
        LinuxScanCodes::L,
        LinuxScanCodes::M,
        LinuxScanCodes::N,
        LinuxScanCodes::O,
        LinuxScanCodes::P,
        LinuxScanCodes::Q,
        LinuxScanCodes::R,
        LinuxScanCodes::S,
        LinuxScanCodes::T,
        LinuxScanCodes::U,
        LinuxScanCodes::V,
        LinuxScanCodes::W,
        LinuxScanCodes::X,
        LinuxScanCodes::Y,
        LinuxScanCodes::Z,
    ];

    fn to_str(&self) -> &str {
        match self {
            LinuxScanCodes::A => "a",
            LinuxScanCodes::B => "b",
            LinuxScanCodes::C => "c",
            LinuxScanCodes::D => "d",
            LinuxScanCodes::E => "e",
            LinuxScanCodes::F => "f",
            LinuxScanCodes::G => "g",
            LinuxScanCodes::H => "h",
            LinuxScanCodes::I => "i",
            LinuxScanCodes::J => "j",
            LinuxScanCodes::K => "k",
            LinuxScanCodes::L => "l",
            LinuxScanCodes::M => "m",
            LinuxScanCodes::N => "n",
            LinuxScanCodes::O => "o",
            LinuxScanCodes::P => "p",
            LinuxScanCodes::Q => "q",
            LinuxScanCodes::R => "r",
            LinuxScanCodes::S => "s",
            LinuxScanCodes::T => "t",
            LinuxScanCodes::U => "u",
            LinuxScanCodes::V => "v",
            LinuxScanCodes::W => "w",
            LinuxScanCodes::X => "x",
            LinuxScanCodes::Y => "y",
            LinuxScanCodes::Z => "z",
            LinuxScanCodes::Digit0 => "0",
            LinuxScanCodes::Digit1 => "1",
            LinuxScanCodes::Digit2 => "2",
            LinuxScanCodes::Digit3 => "3",
            LinuxScanCodes::Digit4 => "4",
            LinuxScanCodes::Digit5 => "5",
            LinuxScanCodes::Digit6 => "6",
            LinuxScanCodes::Digit7 => "7",
            LinuxScanCodes::Digit8 => "8",
            LinuxScanCodes::Digit9 => "9",
            LinuxScanCodes::Backquote => "`",
            LinuxScanCodes::Minus => "-",
            LinuxScanCodes::Equal => "=",
            LinuxScanCodes::LeftBracket => "[",
            LinuxScanCodes::RightBracket => "]",
            LinuxScanCodes::Backslash => "\\",
            LinuxScanCodes::Semicolon => ";",
            LinuxScanCodes::Quote => "'",
            LinuxScanCodes::Comma => ",",
            LinuxScanCodes::Period => ".",
            LinuxScanCodes::Slash => "/",
            LinuxScanCodes::IntlBackslash => "unknown",
            LinuxScanCodes::IntlRo => "unknown",
        }
    }

    #[cfg(test)]
    fn to_shifted(&self) -> &str {
        match self {
            LinuxScanCodes::A => "a",
            LinuxScanCodes::B => "b",
            LinuxScanCodes::C => "c",
            LinuxScanCodes::D => "d",
            LinuxScanCodes::E => "e",
            LinuxScanCodes::F => "f",
            LinuxScanCodes::G => "g",
            LinuxScanCodes::H => "h",
            LinuxScanCodes::I => "i",
            LinuxScanCodes::J => "j",
            LinuxScanCodes::K => "k",
            LinuxScanCodes::L => "l",
            LinuxScanCodes::M => "m",
            LinuxScanCodes::N => "n",
            LinuxScanCodes::O => "o",
            LinuxScanCodes::P => "p",
            LinuxScanCodes::Q => "q",
            LinuxScanCodes::R => "r",
            LinuxScanCodes::S => "s",
            LinuxScanCodes::T => "t",
            LinuxScanCodes::U => "u",
            LinuxScanCodes::V => "v",
            LinuxScanCodes::W => "w",
            LinuxScanCodes::X => "x",
            LinuxScanCodes::Y => "y",
            LinuxScanCodes::Z => "z",
            LinuxScanCodes::Digit0 => ")",
            LinuxScanCodes::Digit1 => "!",
            LinuxScanCodes::Digit2 => "@",
            LinuxScanCodes::Digit3 => "#",
            LinuxScanCodes::Digit4 => "$",
            LinuxScanCodes::Digit5 => "%",
            LinuxScanCodes::Digit6 => "^",
            LinuxScanCodes::Digit7 => "&",
            LinuxScanCodes::Digit8 => "*",
            LinuxScanCodes::Digit9 => "(",
            LinuxScanCodes::Backquote => "~",
            LinuxScanCodes::Minus => "_",
            LinuxScanCodes::Equal => "+",
            LinuxScanCodes::LeftBracket => "{",
            LinuxScanCodes::RightBracket => "}",
            LinuxScanCodes::Backslash => "|",
            LinuxScanCodes::Semicolon => ":",
            LinuxScanCodes::Quote => "\"",
            LinuxScanCodes::Comma => "<",
            LinuxScanCodes::Period => ">",
            LinuxScanCodes::Slash => "?",
            LinuxScanCodes::IntlBackslash => "unknown",
            LinuxScanCodes::IntlRo => "unknown",
        }
    }
}

#[cfg(all(test, any(feature = "wayland", feature = "x11")))]
mod tests {
    use strum::IntoEnumIterator;

    use crate::platform::linux::keyboard::LinuxScanCodes;

    use super::LinuxKeyboardMapper;

    #[test]
    fn test_us_layout_mapper() {
        let mapper = LinuxKeyboardMapper::new(0, 0, 0);
        for scan_code in super::LinuxScanCodes::iter() {
            if scan_code == LinuxScanCodes::IntlBackslash || scan_code == LinuxScanCodes::IntlRo {
                continue;
            }
            let keycode = xkbcommon::xkb::Keycode::new(scan_code as u32);
            let key = mapper
                .get_key(keycode, &mut crate::Modifiers::default())
                .unwrap();
            assert_eq!(key.as_str(), scan_code.to_str());

            let shifted_key = mapper
                .get_key(
                    keycode,
                    &mut crate::Modifiers {
                        shift: true,
                        ..Default::default()
                    },
                )
                .unwrap();
            assert_eq!(shifted_key.as_str(), scan_code.to_shifted());
        }
    }
}
