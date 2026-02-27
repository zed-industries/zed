use gpui::PlatformKeyboardLayout;

pub struct WebKeyboardLayout;

impl WebKeyboardLayout {
    pub fn new() -> Self {
        WebKeyboardLayout
    }
}

impl PlatformKeyboardLayout for WebKeyboardLayout {
    fn id(&self) -> &str {
        "us"
    }

    fn name(&self) -> &str {
        "US"
    }
}
