#![cfg(test)]

use super::*;

#[test]
fn test_detect_mode_from_query() {
    // Test the prefix detection logic
    assert_eq!(detect_mode_from_query(">test"), Some(PaletteMode::CommandPalette));
    assert_eq!(detect_mode_from_query("#symbol"), Some(PaletteMode::ProjectSymbols));
    assert_eq!(detect_mode_from_query("@func"), Some(PaletteMode::Outline));
    assert_eq!(detect_mode_from_query(":42"), Some(PaletteMode::GoToLine));
    assert_eq!(detect_mode_from_query("file.rs"), None); // No prefix = FileFinder (default)
}

#[test]
fn test_is_mode_available() {
    // Modes that don't require an editor
    assert!(is_mode_available(PaletteMode::FileFinder, false));
    assert!(is_mode_available(PaletteMode::CommandPalette, false));
    assert!(is_mode_available(PaletteMode::ProjectSymbols, false));
    
    // Modes that require an editor
    assert!(!is_mode_available(PaletteMode::Outline, false));
    assert!(!is_mode_available(PaletteMode::GoToLine, false));
    assert!(is_mode_available(PaletteMode::Outline, true));
    assert!(is_mode_available(PaletteMode::GoToLine, true));
}
