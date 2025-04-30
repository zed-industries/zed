use anyhow::Result;

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
    fn get_shifted_key(&self, key: &str) -> Result<String>;
}

/// TODO:
pub struct TestKeyboardMapper {
    #[cfg(target_os = "windows")]
    mapper: super::WindowsKeyboardMapper,
    #[cfg(target_os = "macos")]
    mapper: super::MacKeyboardMapper,
}

impl PlatformKeyboardMapper for TestKeyboardMapper {
    fn get_shifted_key(&self, key: &str) -> Result<String> {
        self.mapper.get_shifted_key(key)
    }
}

impl TestKeyboardMapper {
    /// TODO:
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            mapper: super::WindowsKeyboardMapper::new(),
            #[cfg(target_os = "macos")]
            mapper: super::MacKeyboardMapper::new(),
        }
    }
}

pub(crate) fn is_immutable_key(key: &str) -> bool {
    matches!(
        key,
        "f1" | "f2"
            | "f3"
            | "f4"
            | "f5"
            | "f6"
            | "f7"
            | "f8"
            | "f9"
            | "f10"
            | "f11"
            | "f12"
            | "f13"
            | "f14"
            | "f15"
            | "f16"
            | "f17"
            | "f18"
            | "f19"
            | "f20"
            | "f21"
            | "f22"
            | "f23"
            | "f24"
            | "backspace"
            | "delete"
            | "left"
            | "right"
            | "up"
            | "down"
            | "pageup"
            | "pagedown"
            | "insert"
            | "home"
            | "end"
            | "back"
            | "forward"
            | "escape"
            | "space"
            | "tab"
            | "enter"
            | "shift"
            | "control"
            | "alt"
            | "platform"
            | "cmd"
            | "super"
            | "win"
            | "fn"
            | "menu"
    )
}

pub(crate) fn is_alphabetic_key(key: &str) -> bool {
    matches!(
        key,
        "a" | "b"
            | "c"
            | "d"
            | "e"
            | "f"
            | "g"
            | "h"
            | "i"
            | "j"
            | "k"
            | "l"
            | "m"
            | "n"
            | "o"
            | "p"
            | "q"
            | "r"
            | "s"
            | "t"
            | "u"
            | "v"
            | "w"
            | "x"
            | "y"
            | "z"
    )
}
