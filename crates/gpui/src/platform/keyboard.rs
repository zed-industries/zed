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

/// A trait for platform-specific keyboard mappers, which map keystrokes to the platform's native format
/// and convert them to Vim keystrokes, given the current keyboard layout.
pub trait PlatformKeyboardMapper {
    /// This method maps a keystroke to the platform's native format.
    /// For example, on macOS, when `use_key_equivalents` is true, it maps the keystroke to the equivalent key;
    /// On Windows, it maps the keystroke to its virtual key conterpart.
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke;

    /// This method converts a keystroke to the Vim format.
    /// For example, it converts `ctrl-shift-a` to `ctrl-A`.
    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke>;

    /// This method returns a map of key equivalents, macOS only for now.
    fn get_equivalents(&self) -> Option<&HashMap<String, String>>;
}

/// An empty keyboard mapper that does nothing.
pub struct EmptyKeyboardMapper;

impl PlatformKeyboardMapper for EmptyKeyboardMapper {
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

/// A test keyboard mapper that uses the platform-specific keyboard mappers.
pub struct TestKeyboardMapper {
    #[cfg(target_os = "windows")]
    mapper: super::WindowsKeyboardMapper,
    #[cfg(target_os = "macos")]
    mapper: super::MacKeyboardMapper,
    #[cfg(target_os = "linux")]
    mapper: super::LinuxKeyboardMapper,
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    mapper: EmptyKeyboardMapper,
}

impl TestKeyboardMapper {
    /// Construct a new test keyboard mapper.
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            mapper: super::WindowsKeyboardMapper::new(),
            #[cfg(target_os = "macos")]
            mapper: super::MacKeyboardMapper::new(),
            #[cfg(target_os = "linux")]
            mapper: super::LinuxKeyboardMapper::new(),
            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            mapper: EmptyKeyboardMapper,
        }
    }
}

impl PlatformKeyboardMapper for TestKeyboardMapper {
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
