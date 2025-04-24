use std::borrow::Cow;

use collections::HashMap;

use crate::{Keystroke, Modifiers, PlatformKeyboardMapper};

pub(crate) struct LinuxKeyboardMapper;

impl PlatformKeyboardMapper for LinuxKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, _: bool) -> Keystroke {
        keystroke
    }

    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke> {
        if is_letter_key(keystroke.key.as_str()) && keystroke.modifiers.shift {
            return Cow::Owned(Keystroke {
                modifiers: keystroke.modifiers & !Modifiers::shift(),
                key: keystroke.key.to_uppercase(),
                key_char: keystroke.key_char.clone(),
            });
        }
        Cow::Borrowed(keystroke)
    }

    fn get_equivalents(&self) -> Option<&HashMap<String, String>> {
        None
    }
}

impl LinuxKeyboardMapper {
    pub(crate) fn new() -> Self {
        Self
    }
}

fn is_letter_key(key: &str) -> bool {
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
