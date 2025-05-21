#[cfg(any(feature = "wayland", feature = "x11"))]
use collections::{HashMap, HashSet};
#[cfg(any(feature = "wayland", feature = "x11"))]
use xkbcommon::xkb::{Keycode, Keysym};

use crate::{PlatformKeyboardLayout, SharedString};

#[derive(Clone)]
pub(crate) struct LinuxKeyboardLayout {
    name: SharedString,
}

impl PlatformKeyboardLayout for LinuxKeyboardLayout {
    fn id(&self) -> &str {
        &self.name
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl LinuxKeyboardLayout {
    pub(crate) fn new(name: SharedString) -> Self {
        Self { name }
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) struct LinuxKeyboardMapper {
    code_to_key: HashMap<Keycode, String>,
    code_to_shifted_key: HashMap<Keycode, String>,
}

#[cfg(any(feature = "wayland", feature = "x11"))]
impl LinuxKeyboardMapper {
    pub(crate) fn new(xkb_state: &xkbcommon::xkb::State) -> Self {
        let mut code_to_key = HashMap::default();
        let mut code_to_shifted_key = HashMap::default();
        let mut inserted = HashSet::default();

        let keymap = xkb_state.get_keymap();
        let mut shifted_state = xkbcommon::xkb::State::new(&keymap);
        let shift_mod = keymap.mod_get_index(xkbcommon::xkb::MOD_NAME_SHIFT);
        let shift_mask = 1 << shift_mod;
        shifted_state.update_mask(shift_mask, 0, 0, 0, 0, 0);

        for &scan_code in TYPEABLE_CODES {
            let keycode = Keycode::new(scan_code);
            let key = xkb_state.key_get_utf8(keycode);

            if !key.is_empty() {
                code_to_key.insert(keycode, key.clone());
                inserted.insert(key);

                let shifted_key = shifted_state.key_get_utf8(keycode);
                code_to_shifted_key.insert(keycode, shifted_key);
            } else {
                // keycode might be a dead key
                let keysym = xkb_state.key_get_one_sym(keycode);
                if let Some(key) = underlying_dead_key(keysym) {
                    code_to_key.insert(keycode, key.clone());
                    inserted.insert(key);
                }

                let shifted_keysym = shifted_state.key_get_one_sym(keycode);
                if let Some(shifted_key) = underlying_dead_key(shifted_keysym) {
                    code_to_shifted_key.insert(keycode, shifted_key);
                }
            }
        }
        insert_letters_if_missing(&inserted, &mut code_to_key);

        Self {
            code_to_key,
            code_to_shifted_key,
        }
    }

    pub(crate) fn get_key(
        &self,
        keycode: Keycode,
        modifiers: &mut crate::Modifiers,
    ) -> Option<String> {
        if is_alphabetic_key(keycode) || !modifiers.shift {
            self.code_to_key.get(&keycode).cloned()
        } else {
            modifiers.shift = false;
            self.code_to_shifted_key.get(&keycode).cloned()
        }
    }
}

#[cfg(any(feature = "wayland", feature = "x11"))]
fn is_alphabetic_key(keycode: Keycode) -> bool {
    matches!(
        keycode.raw(),
        0x0026 // a
        | 0x0038 // b
        | 0x0036 // c
        | 0x0028 // d
        | 0x001a // e
        | 0x0029 // f
        | 0x002a // g
        | 0x002b // h
        | 0x001f // i
        | 0x002c // j
        | 0x002d // k
        | 0x002e // l
        | 0x003a // m
        | 0x0039 // n
        | 0x0020 // o
        | 0x0021 // p
        | 0x0018 // q
        | 0x001b // r
        | 0x0027 // s
        | 0x001c // t
        | 0x001e // u
        | 0x0037 // v
        | 0x0019 // w
        | 0x0035 // x
        | 0x001d // y
        | 0x0034 // z
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
macro_rules! insert_letters_if_missing_internal {
    ($inserted:expr, $code_to_key:expr, $code:expr, $key:literal) => {
        if !$inserted.contains($key) {
            $code_to_key.insert($code, $key.to_string());
        }
    };
}

#[cfg(any(feature = "wayland", feature = "x11"))]
fn insert_letters_if_missing(
    inserted: &HashSet<String>,
    code_to_key: &mut HashMap<Keycode, String>,
) {
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0026), "a");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0038), "b");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0036), "c");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0028), "d");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x001a), "e");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0029), "f");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x002a), "g");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x002b), "h");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x001f), "i");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x002c), "j");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x002d), "k");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x002e), "l");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x003a), "m");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0039), "n");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0020), "o");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0021), "p");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0018), "q");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x001b), "r");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0027), "s");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x001c), "t");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x001e), "u");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0037), "v");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0019), "w");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0035), "x");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x001d), "y");
    insert_letters_if_missing_internal!(inserted, code_to_key, Keycode::new(0x0034), "z");
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
