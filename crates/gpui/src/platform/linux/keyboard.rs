use crate::{PlatformKeyboardLayout, SharedString};

#[derive(Clone)]
pub(crate) struct LinuxKeyboardLayout {
    name: SharedString,
}

impl PlatformKeyboardLayout for LinuxKeyboardLayout {
    fn id(&self) -> &str {
        &self.name
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl LinuxKeyboardLayout {
    pub(crate) fn new(name: SharedString) -> Self {
        Self { name }
    }
}
