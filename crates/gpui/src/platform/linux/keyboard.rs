use std::borrow::Cow;

use collections::HashMap;

use crate::{Keystroke, Modifiers, PlatformKeyboardLayout, PlatformKeyboardMapper};

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

pub(crate) struct LinuxKeyboardMapper;

impl PlatformKeyboardMapper for LinuxKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, _: bool) -> Keystroke {
        keystroke
    }

    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke> {
        if is_alphabetic_key(keystroke.key.as_str()) && keystroke.modifiers.shift {
            return Cow::Owned(Keystroke {
                modifiers: keystroke.modifiers & !Modifiers::shift(),
                key: keystroke.key.to_uppercase(),
                key_char: keystroke.key_char.clone(),
            });
        }
        Cow::Borrowed(keystroke)
    }

    fn get_shifted_key(&self, key: &str) -> String {
        if is_alphabetic_key(key) {
            key.to_uppercase()
        } else {
            key.to_string()
        }
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
