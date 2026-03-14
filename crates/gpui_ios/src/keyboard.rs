use gpui::{DummyKeyboardMapper, PlatformKeyboardLayout, PlatformKeyboardMapper};
use std::rc::Rc;

/// Keyboard layout stub for iOS. The real implementation will query
/// UITextInputMode.activeInputModes to get the current keyboard locale
/// in Phase 1.3.
pub(crate) struct IosKeyboardLayout;

impl PlatformKeyboardLayout for IosKeyboardLayout {
    fn id(&self) -> &str {
        "us"
    }

    fn name(&self) -> &str {
        "US"
    }
}

pub(crate) fn ios_keyboard_mapper() -> Rc<dyn PlatformKeyboardMapper> {
    Rc::new(DummyKeyboardMapper)
}
