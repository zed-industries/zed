use super::Keystroke;

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn map_keystroke(&self, keystroke: Keystroke) -> Keystroke;
}

/// TODO:
pub struct EmptyKeyboardMapper;

impl KeyboardMapper for EmptyKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke) -> Keystroke {
        keystroke
    }
}
