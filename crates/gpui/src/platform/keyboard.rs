use std::borrow::Cow;

use collections::HashMap;

use super::Keystroke;

/// A trait for platform-specific keyboard layouts
pub trait PlatformKeyboardLayout {
    /// Get the keyboard layout ID, which should be unique to the layout
    fn id(&self) -> &str;
    /// Get the keyboard layout display name
    fn name(&self) -> &str;
}

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke;
    /// TODO:
    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke>;
    /// TODO:
    fn get_equivalents(&self) -> Option<&HashMap<String, String>>;
}

/// TODO:
pub struct EmptyKeyboardMapper;

impl KeyboardMapper for EmptyKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, _: bool) -> Keystroke {
        keystroke
    }

    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke> {
        Cow::Borrowed(keystroke)
    }

    fn get_equivalents(&self) -> Option<&HashMap<String, String>> {
        None
    }
}

/// TODO:
pub struct TestKeyboardMapper {
    #[cfg(target_os = "windows")]
    mapper: super::WindowsKeyboardMapper,
    #[cfg(target_os = "macos")]
    mapper: super::MacKeyboardMapper,
    #[cfg(target_os = "linux")]
    mapper: super::LinuxKeyboardMapper,
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    mapper: EmptyKeyboardMapper,
}

impl TestKeyboardMapper {
    /// TODO:
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            mapper: super::WindowsKeyboardMapper::new(),
            #[cfg(target_os = "macos")]
            mapper: super::MacKeyboardMapper::new(),
            #[cfg(target_os = "linux")]
            mapper: super::LinuxKeyboardMapper::new(),
            #[cfg(not(any(target_os = "windows", target_os = "macos")))]
            mapper: EmptyKeyboardMapper,
        }
    }
}

impl KeyboardMapper for TestKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke {
        self.mapper.map_keystroke(keystroke, use_key_equivalents)
    }

    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke> {
        self.mapper.to_vim_keystroke(keystroke)
    }

    fn get_equivalents(&self) -> Option<&HashMap<String, String>> {
        self.mapper.get_equivalents()
    }
}
