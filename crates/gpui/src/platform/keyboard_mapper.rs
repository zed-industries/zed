use super::{KeyCode, Modifiers};

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn parse(&self, input: &str, char_matching: bool) -> Option<(KeyCode, Modifiers)>;
    /// TODO:
    fn keycode_to_face(&self, code: KeyCode) -> Option<String>;
}
