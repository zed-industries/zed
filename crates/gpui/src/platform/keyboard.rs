use anyhow::Result;

use super::ScanCode;

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
    fn scan_code_to_key(&self, scan_code: ScanCode) -> Result<String>;
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
    fn scan_code_to_key(&self, scan_code: ScanCode) -> Result<String> {
        self.mapper.scan_code_to_key(scan_code)
    }

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

/// A dummy keyboard mapper that does not support any key mappings
pub struct EmptyKeyboardMapper;

impl PlatformKeyboardMapper for EmptyKeyboardMapper {
    fn scan_code_to_key(&self, _scan_code: ScanCode) -> Result<String> {
        anyhow::bail!("EmptyKeyboardMapper does not support scan codes")
    }

    fn get_shifted_key(&self, key: &str) -> Result<String> {
        Ok(key.to_uppercase())
    }
}

/// TODO:
pub fn is_immutable_key(key: &str) -> bool {
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

/// TODO:
pub fn is_alphabetic_key(key: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::{PlatformKeyboardMapper, TestKeyboardMapper};

    #[test]
    fn test_get_shifted_key() {
        let mapper = TestKeyboardMapper::new();

        for ch in 'a'..'z' {
            let key = ch.to_string();
            let shifted_key = key.to_uppercase();
            assert_eq!(mapper.get_shifted_key(&key).unwrap(), shifted_key);
        }

        let shift_pairs = [
            ("1", "!"),
            ("2", "@"),
            ("3", "#"),
            ("4", "$"),
            ("5", "%"),
            ("6", "^"),
            ("7", "&"),
            ("8", "*"),
            ("9", "("),
            ("0", ")"),
            ("`", "~"),
            ("-", "_"),
            ("=", "+"),
            ("[", "{"),
            ("]", "}"),
            ("\\", "|"),
            (";", ":"),
            ("'", "\""),
            (",", "<"),
            (".", ">"),
            ("/", "?"),
        ];
        for (key, shifted_key) in shift_pairs {
            assert_eq!(mapper.get_shifted_key(key).unwrap(), shifted_key);
        }
    }
}
