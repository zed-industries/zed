use anyhow::Result;

use crate::{Modifiers, ScanCode};

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
    fn scan_code_to_key(&self, scan_code: ScanCode, modifiers: &mut Modifiers) -> Result<String>;
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
    fn scan_code_to_key(&self, scan_code: ScanCode, modifiers: &mut Modifiers) -> Result<String> {
        self.mapper.scan_code_to_key(scan_code, modifiers)
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
    fn scan_code_to_key(&self, _scan_code: ScanCode, _modifiers: &mut Modifiers) -> Result<String> {
        anyhow::bail!("EmptyKeyboardMapper does not support scan codes")
    }
}

// pub(crate) fn is_alphabetic_key(key: &str) -> bool {
//     matches!(
//         key,
//         "a" | "b"
//             | "c"
//             | "d"
//             | "e"
//             | "f"
//             | "g"
//             | "h"
//             | "i"
//             | "j"
//             | "k"
//             | "l"
//             | "m"
//             | "n"
//             | "o"
//             | "p"
//             | "q"
//             | "r"
//             | "s"
//             | "t"
//             | "u"
//             | "v"
//             | "w"
//             | "x"
//             | "y"
//             | "z"
//     )
// }

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::{Modifiers, ScanCode};

    use super::{PlatformKeyboardMapper, TestKeyboardMapper};

    #[test]
    fn test_scan_code_to_key() {
        let mapper = TestKeyboardMapper::new();
        for scan_code in ScanCode::iter() {
            // The IntlBackslash and IntlRo keys are not mapped to any key on US layout
            if scan_code == ScanCode::IntlBackslash || scan_code == ScanCode::IntlRo {
                continue;
            }
            let mut modifiers = Modifiers::default();
            let key = mapper.scan_code_to_key(scan_code, &mut modifiers).unwrap();
            assert_eq!(key, scan_code.to_key());
        }
    }
}
