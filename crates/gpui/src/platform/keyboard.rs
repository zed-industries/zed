use super::Keystroke;

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke;
}

/// TODO:
pub struct EmptyKeyboardMapper;

impl KeyboardMapper for EmptyKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, _: bool) -> Keystroke {
        keystroke
    }
}
