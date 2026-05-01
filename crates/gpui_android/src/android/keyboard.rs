use gpui::PlatformKeyboardLayout;

pub(crate) struct AndroidKeyboardLayout;

impl AndroidKeyboardLayout {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl PlatformKeyboardLayout for AndroidKeyboardLayout {
    fn id(&self) -> &str {
        // Android does not expose hardware-keyboard layout identifiers in a
        // way that maps cleanly to the desktop keymap system; soft-keyboard
        // input goes through the IME. We report a single layout for now so
        // callers always get a stable answer.
        "android"
    }

    fn name(&self) -> &str {
        "Android"
    }
}
