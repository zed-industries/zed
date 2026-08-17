// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Architecture {
    X64,
    X86,
}

impl Ord for Architecture {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        format!("{self:?}").cmp(&format!("{other:?}"))
    }
}
impl PartialOrd for Architecture {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if *self == Architecture::X64 {
                "x64"
            } else {
                "x86"
            }
        )
        .unwrap_or_default();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_display_x64() {
        let arch = Architecture::X64;
        assert_eq!(format!("{}", arch), "x64");
    }

    #[test]
    fn test_architecture_display_x86() {
        let arch = Architecture::X86;
        assert_eq!(format!("{}", arch), "x86");
    }

    #[test]
    fn test_architecture_ordering() {
        let x64 = Architecture::X64;
        let x86 = Architecture::X86;
        // X64 < X86 alphabetically
        assert!(x64 < x86);
        assert!(x86 > x64);
        assert_eq!(x64.cmp(&x64), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_architecture_partial_ordering() {
        let x64 = Architecture::X64;
        let x86 = Architecture::X86;
        assert_eq!(x64.partial_cmp(&x86), Some(std::cmp::Ordering::Less));
        assert_eq!(x86.partial_cmp(&x64), Some(std::cmp::Ordering::Greater));
        assert_eq!(x64.partial_cmp(&x64), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn test_architecture_equality() {
        assert_eq!(Architecture::X64, Architecture::X64);
        assert_eq!(Architecture::X86, Architecture::X86);
        assert_ne!(Architecture::X64, Architecture::X86);
    }

    #[test]
    fn test_architecture_clone() {
        let arch = Architecture::X64;
        let cloned = arch.clone();
        assert_eq!(arch, cloned);
    }

    #[test]
    fn test_architecture_debug() {
        let arch = Architecture::X64;
        assert_eq!(format!("{:?}", arch), "X64");
        let arch = Architecture::X86;
        assert_eq!(format!("{:?}", arch), "X86");
    }

    #[test]
    fn test_architecture_serialize() {
        let arch = Architecture::X64;
        let json = serde_json::to_string(&arch).unwrap();
        assert_eq!(json, "\"x64\"");

        let arch = Architecture::X86;
        let json = serde_json::to_string(&arch).unwrap();
        assert_eq!(json, "\"x86\"");
    }

    #[test]
    fn test_architecture_deserialize() {
        let arch: Architecture = serde_json::from_str("\"x64\"").unwrap();
        assert_eq!(arch, Architecture::X64);

        let arch: Architecture = serde_json::from_str("\"x86\"").unwrap();
        assert_eq!(arch, Architecture::X86);
    }
}
