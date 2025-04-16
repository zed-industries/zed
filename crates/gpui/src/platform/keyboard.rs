use super::Keystroke;

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn map_keystroke(&self, keystroke: Keystroke) -> Keystroke;
}
