use super::Keystroke;

/// A trait for platform-specific keyboard layouts
pub trait PlatformKeyboardLayout {
    /// Get the keyboard layout ID, which should be unique to the layout
    fn id(&self) -> &str;
    /// Get the keyboard layout display name
    fn name(&self) -> &str;
}

/// TODO:
pub trait PlatformKeyboardMapper {
    /// TODO:
    fn vscode_keystroke_to_gpui_keystroke(&self, keystroke: Keystroke) -> Keystroke;
}

/// TODO:
pub struct TestKeyboardMapper {
    #[cfg(target_os = "windows")]
    mapper: super::WindowsKeyboardMapper,
}

impl PlatformKeyboardMapper for TestKeyboardMapper {
    fn vscode_keystroke_to_gpui_keystroke(&self, keystroke: Keystroke) -> Keystroke {
        self.mapper.vscode_keystroke_to_gpui_keystroke(keystroke)
    }
}

impl TestKeyboardMapper {
    /// TODO:
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            mapper: super::WindowsKeyboardMapper::new(),
        }
    }
}
