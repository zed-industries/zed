use crate::{KeyboardMapper, Keystroke};

pub(crate) struct WindowsKeyboardMapper;

impl KeyboardMapper for WindowsKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke {
        todo!()
    }
}
