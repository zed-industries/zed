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
    fn get_shifted_key(&self, key: &str) -> Result<Option<String>>;
}

/// TODO:
pub struct TestKeyboardMapper {
    #[cfg(target_os = "windows")]
    mapper: super::WindowsKeyboardMapper,
    #[cfg(target_os = "macos")]
    mapper: super::MacKeyboardMapper,
    #[cfg(target_os = "linux")]
    mapper: super::LinuxKeyboardMapper,
}

impl PlatformKeyboardMapper for TestKeyboardMapper {
    fn scan_code_to_key(&self, scan_code: ScanCode) -> Result<String> {
        self.mapper.scan_code_to_key(scan_code)
    }

    fn get_shifted_key(&self, key: &str) -> Result<Option<String>> {
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
            #[cfg(target_os = "linux")]
            mapper: super::LinuxKeyboardMapper::new(),
        }
    }
}

/// A dummy keyboard mapper that does not support any key mappings
pub struct EmptyKeyboardMapper;

impl PlatformKeyboardMapper for EmptyKeyboardMapper {
    fn scan_code_to_key(&self, _scan_code: ScanCode) -> Result<String> {
        anyhow::bail!("EmptyKeyboardMapper does not support scan codes")
    }

    fn get_shifted_key(&self, _key: &str) -> Result<Option<String>> {
        anyhow::bail!("EmptyKeyboardMapper does not support shifted keys")
    }
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

#[cfg(test)]
mod tests {
    // #[cfg(not(target_os = "linux"))]
    use strum::IntoEnumIterator;

    // #[cfg(not(target_os = "linux"))]
    use crate::ScanCode;

    // #[cfg(not(target_os = "linux"))]
    use super::{PlatformKeyboardMapper, TestKeyboardMapper};

    #[test]
    // #[cfg(not(target_os = "linux"))]
    fn test_get_shifted_key() {
        let mapper = TestKeyboardMapper::new();

        for ch in 'a'..='z' {
            let key = ch.to_string();
            let shifted_key = key.to_uppercase();
            assert_eq!(mapper.get_shifted_key(&key).unwrap().unwrap(), shifted_key);
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
            assert_eq!(mapper.get_shifted_key(key).unwrap().unwrap(), shifted_key);
        }

        let immutable_keys = ["backspace", "space", "tab", "enter", "f1"];
        for key in immutable_keys {
            assert_eq!(mapper.get_shifted_key(key).unwrap(), None);
        }
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn test_scan_code_to_key() {
        let mapper = TestKeyboardMapper::new();
        for scan_code in ScanCode::iter() {
            let key = mapper.scan_code_to_key(scan_code).unwrap();
            assert_eq!(key, scan_code.to_key());
        }
    }
}
