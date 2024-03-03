use xkbcommon::xkb::{self, Keycode, Keysym, State};

use crate::{Keystroke, Modifiers};

impl Keystroke {
    pub(super) fn from_xkb(state: &State, modifiers: Modifiers, keycode: Keycode) -> Self {
        let mut modifiers = modifiers;

        let key_utf32 = state.key_get_utf32(keycode);
        let key_utf8 = state.key_get_utf8(keycode);
        let key_sym = state.key_get_one_sym(keycode);

        // The logic here tries to replicate the logic in `../mac/events.rs`
        // "Consumed" modifiers are modifiers that have been used to translate a key, for example
        // pressing "shift" and "1" on US layout produces the key `!` but "consumes" the shift.
        // Notes:
        //  - macOS gets the key character directly ("."), xkb gives us the key name ("period")
        //  - macOS logic removes consumed shift modifier for symbols: "{", not "shift-{"
        //  - macOS logic keeps consumed shift modifiers for letters: "shift-a", not "a" or "A"

        let mut handle_consumed_modifiers = true;
        let key = match key_sym {
            Keysym::Return => "enter".to_owned(),
            Keysym::Prior => "pageup".to_owned(),
            Keysym::Next => "pagedown".to_owned(),

            Keysym::comma => ",".to_owned(),
            Keysym::period => ".".to_owned(),
            Keysym::less => "<".to_owned(),
            Keysym::greater => ">".to_owned(),
            Keysym::slash => "/".to_owned(),
            Keysym::question => "?".to_owned(),

            Keysym::semicolon => ";".to_owned(),
            Keysym::colon => ":".to_owned(),
            Keysym::apostrophe => "'".to_owned(),
            Keysym::quotedbl => "\"".to_owned(),

            Keysym::bracketleft => "[".to_owned(),
            Keysym::braceleft => "{".to_owned(),
            Keysym::bracketright => "]".to_owned(),
            Keysym::braceright => "}".to_owned(),
            Keysym::backslash => "\\".to_owned(),
            Keysym::bar => "|".to_owned(),

            Keysym::grave => "`".to_owned(),
            Keysym::asciitilde => "~".to_owned(),
            Keysym::exclam => "!".to_owned(),
            Keysym::at => "@".to_owned(),
            Keysym::numbersign => "#".to_owned(),
            Keysym::dollar => "$".to_owned(),
            Keysym::percent => "%".to_owned(),
            Keysym::asciicircum => "^".to_owned(),
            Keysym::ampersand => "&".to_owned(),
            Keysym::asterisk => "*".to_owned(),
            Keysym::parenleft => "(".to_owned(),
            Keysym::parenright => ")".to_owned(),
            Keysym::minus => "-".to_owned(),
            Keysym::underscore => "_".to_owned(),
            Keysym::equal => "=".to_owned(),
            Keysym::plus => "+".to_owned(),

            _ => {
                handle_consumed_modifiers = false;
                xkb::keysym_get_name(key_sym).to_lowercase()
            }
        };

        // Ignore control characters (and DEL) for the purposes of ime_key,
        // but if key_utf32 is 0 then assume it isn't one
        let ime_key = ((key_utf32 == 0 || (key_utf32 >= 32 && key_utf32 != 127))
            && !key_utf8.is_empty())
        .then_some(key_utf8);

        if handle_consumed_modifiers {
            let mod_shift_index = state.get_keymap().mod_get_index(xkb::MOD_NAME_SHIFT);
            let is_shift_consumed = state.mod_index_is_consumed(keycode, mod_shift_index);

            if modifiers.shift && is_shift_consumed {
                modifiers.shift = false;
            }
        }

        Keystroke {
            modifiers,
            key,
            ime_key,
        }
    }
}
