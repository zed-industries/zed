use crate::PlatformKeyboard;

pub(crate) struct WindowsKeyboard {}

impl PlatformKeyboard for WindowsKeyboard {
    fn code_to_key(&self, code: &crate::KeyCodes) -> String {
        "Unimplemented".to_owned()
    }
}
