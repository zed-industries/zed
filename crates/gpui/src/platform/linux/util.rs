use xkbcommon::xkb::{self, Keycode, Keysym, State};

use crate::{Keystroke, Modifiers};

impl Keystroke {
    pub(super) fn from_xkb(state: &State, modifiers: Modifiers, keycode: Keycode) -> Self {
        let key_utf32 = state.key_get_utf32(keycode);
        let key_utf8 = state.key_get_utf8(keycode);
        let key_sym = state.key_get_one_sym(keycode);

        let key = match key_sym {
            Keysym::Return => "enter".to_owned(),
            Keysym::Prior => "pageup".to_owned(),
            Keysym::Next => "pagedown".to_owned(),
            _ => xkb::keysym_get_name(key_sym).to_lowercase(),
        };

        // Ignore control characters (and DEL) for the purposes of ime_key,
        // but if key_utf32 is 0 then assume it isn't one
        let ime_key = ((key_utf32 == 0 || (key_utf32 >= 32 && key_utf32 != 127))
            && !key_utf8.is_empty())
        .then_some(key_utf8);

        Keystroke {
            modifiers,
            key,
            ime_key,
        }
    }
}
