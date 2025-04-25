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

/// A trait for handling platform-specific keyboard mapping behaviors.
///
/// This trait provides functionality to translate keystrokes between different representations,
/// handle platform-specific key equivalents, and convert keystrokes to Vim-compatible formats.
/// Implementations should account for different keyboard layouts and platform conventions.
pub trait PlatformKeyboardMapper {
    /// Maps a keystroke according to platform-specific keyboard layout rules.
    ///
    /// On macOS when `use_key_equivalents` is true, this rearranges shortcuts
    /// to ensure they remain accessible. When false, no processing occurs.
    ///
    /// On Windows when `use_key_equivalents` is true, this interprets keys as
    /// Virtual Keys (e.g., `ctrl-[` becomes `VK_CTRL-VK_OEM_4`). On German
    /// layouts, `VK_OEM_4` produces 'ẞ', resulting in `ctrl-ẞ`.
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke;

    /// Converts a keystroke to Vim-style key notation.
    ///
    /// For example, converts `ctrl-shift-a` to `ctrl-A`. The return type uses
    /// `Cow` to avoid unnecessary allocations when no conversion is needed.
    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke>;

    /// Converts a key to its shifted representation.
    fn get_shifted_key(&self, key: &str) -> String;

    /// Returns the keyboard layout's key equivalents mapping, if available.
    ///
    /// Currently only implemented and used on macOS. The HashMap contains
    /// mappings between key representations (e.g., special characters to their
    /// equivalent key combinations).
    fn get_equivalents(&self) -> Option<&HashMap<String, String>>;
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

    fn get_shifted_key(&self, key: &str) -> String {
        self.mapper.get_shifted_key(key)
    }

    fn get_equivalents(&self) -> Option<&HashMap<String, String>> {
        self.mapper.get_equivalents()
    }
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

    fn get_shifted_key(&self, key: &str) -> String {
        key.to_uppercase()
    }

    fn get_equivalents(&self) -> Option<&HashMap<String, String>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{Keystroke, Modifiers, PlatformKeyboardMapper};

    use super::TestKeyboardMapper;

    #[test]
    fn test_basic_usage() {
        let mapper = TestKeyboardMapper::new();
        for c in 'a'..='z' {
            let key = c.to_string();

            // `shift-a` -> `A`
            let keystroke = Keystroke {
                modifiers: Modifiers::shift(),
                key: key.clone(),
                key_char: None,
            };
            let vim_keystroke = mapper.to_vim_keystroke(&keystroke);
            assert_eq!(
                *vim_keystroke,
                Keystroke {
                    modifiers: Modifiers::default(),
                    key: key.to_uppercase(),
                    key_char: None,
                }
            );

            // `ctrl-shift-a` -> `ctrl-A`
            let keystroke = Keystroke {
                modifiers: Modifiers::control_shift(),
                key: key.clone(),
                key_char: None,
            };
            let vim_keystroke = mapper.to_vim_keystroke(&keystroke);
            assert_eq!(
                *vim_keystroke,
                Keystroke {
                    modifiers: Modifiers::control(),
                    key: key.to_uppercase(),
                    key_char: None,
                }
            );

            // `alt-shift-a` -> `alt-A`
            let keystroke = Keystroke {
                modifiers: Modifiers::alt() | Modifiers::shift(),
                key: key.clone(),
                key_char: None,
            };
            let vim_keystroke = mapper.to_vim_keystroke(&keystroke);
            assert_eq!(
                *vim_keystroke,
                Keystroke {
                    modifiers: Modifiers::alt(),
                    key: key.to_uppercase(),
                    key_char: None,
                }
            );

            // `ctrl-alt-shift-a` -> `ctrl-alt-A`
            let keystroke = Keystroke {
                modifiers: Modifiers::alt() | Modifiers::shift() | Modifiers::control(),
                key: key.clone(),
                key_char: None,
            };
            let vim_keystroke = mapper.to_vim_keystroke(&keystroke);
            assert_eq!(
                *vim_keystroke,
                Keystroke {
                    modifiers: Modifiers::alt() | Modifiers::control(),
                    key: key.to_uppercase(),
                    key_char: None,
                }
            );

            // `a` -> `A`
            let shifted_key = mapper.get_shifted_key(&key);
            assert_eq!(shifted_key, key.to_uppercase());
        }
    }
}
