use crate::scancode::ScanCode;

pub(crate) fn translate_key_macos(keystroke: &str) -> String {
    let scancode = ScanCode::from_keystroke(keystroke).unwrap_or_else(|| ScanCode::Other(0));
    "TODO".into()
}
