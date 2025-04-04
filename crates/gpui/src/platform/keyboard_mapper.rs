use std::rc::Rc;

use collections::HashMap;

use super::{
    always_use_command_layout, chars_for_modified_key, keyboard_layout, KeyCode, Modifiers,
};

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn parse(&self, input: &str, char_matching: bool) -> Option<(KeyCode, Modifiers)>;
    /// TODO:
    fn keycode_to_face(&self, code: KeyCode) -> Option<String>;
}
