use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags},
    base::id,
    foundation::NSString,
};
use std::ffi::CStr;

// Private CoreGraphicsServices ("SkyLight") API. Same calls WindowServer and
// System Settings use to resolve the current value of a symbolic hotkey:
// returns the user's plist override if present, otherwise the default baked
// into SkyLight's hardcoded fallback table. Using it means we don't parse the
// plist ourselves or hardcode any defaults.
//
// Each call costs ~10µs (Mach IPC to WindowServer). Even at unrealistic key
// rates that's a fraction of a percent of one core, so we just call on every
// matching attempt rather than maintaining a cached value + change watcher.
#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn CGSGetSymbolicHotKeyValue(
        hotkey: i32,
        out_character: *mut u16,
        out_key_code: *mut u16,
        out_modifiers: *mut u32,
    ) -> i32;
    fn CGSIsSymbolicHotKeyEnabled(hotkey: i32) -> i32;
}

// "Move focus to next window". Stable Apple-internal identifier — see
// kCGSMoveFocusToNextWindow / the "27" key in ~/Library/Preferences/
// com.apple.symbolichotkeys.plist.
const WINDOW_CYCLE_HOTKEY_ID: i32 = 27;

const COMMAND_MODIFIER: u32 = 0x0010_0000;
const SHIFT_MODIFIER: u32 = 0x0002_0000;
const OPTION_MODIFIER: u32 = 0x0008_0000;
const CONTROL_MODIFIER: u32 = 0x0004_0000;
const FUNCTION_MODIFIER: u32 = 0x0080_0000;

// Sentinel used by the CGS API (and the AppleSymbolicHotKeys plist) to mean
// "no value" for a character or key code — matches kNoCharCode /
// kHIKeyCodeNoKey in HIToolbox.
const NO_VALUE: u16 = 0xFFFF;

#[derive(Clone, Copy, Default)]
pub(crate) struct MacWindowCycleShortcut;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WindowCycleHotkey {
    character: Option<u16>,
    key_code: Option<u16>,
    modifiers: u32,
}

impl MacWindowCycleShortcut {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn matches(&self, native_event: id) -> bool {
        let Some(hotkey) = read_window_cycle_hotkey() else {
            return false;
        };

        let key_code = unsafe { native_event.keyCode() as u16 };
        let modifiers = unsafe { symbolic_hotkey_modifiers(native_event.modifierFlags()) };
        let character = unsafe { first_character_ignoring_modifiers(native_event) };

        hotkey.matches(character, key_code, modifiers)
    }
}

impl WindowCycleHotkey {
    // If the stored entry has a character (the typical SkyLight default and
    // most locale overrides), match the produced character — that's what keeps
    // cmd+backtick working across layouts even though the physical key moves.
    // If the stored entry only has a key code (e.g. a plist entry whose
    // character field is kNoCharCode/0xFFFF), match the physical key code.
    // We never match both at once: doing so would over-claim shortcuts on
    // layouts where the key code coincides with one we shouldn't reserve.
    fn matches(self, character: Option<u16>, key_code: u16, modifiers: u32) -> bool {
        if self.modifiers != modifiers {
            return false;
        }
        match self.character {
            Some(stored) => character == Some(stored),
            None => self.key_code == Some(key_code),
        }
    }
}

fn read_window_cycle_hotkey() -> Option<WindowCycleHotkey> {
    unsafe {
        if CGSIsSymbolicHotKeyEnabled(WINDOW_CYCLE_HOTKEY_ID) == 0 {
            return None;
        }
        let mut character: u16 = NO_VALUE;
        let mut key_code: u16 = NO_VALUE;
        let mut modifiers: u32 = 0;
        let status = CGSGetSymbolicHotKeyValue(
            WINDOW_CYCLE_HOTKEY_ID,
            &mut character,
            &mut key_code,
            &mut modifiers,
        );
        if status != 0 {
            return None;
        }
        Some(WindowCycleHotkey {
            character: (character != NO_VALUE).then_some(character),
            key_code: (key_code != NO_VALUE).then_some(key_code),
            modifiers,
        })
    }
}

unsafe fn first_character_ignoring_modifiers(native_event: id) -> Option<u16> {
    unsafe {
        let characters: id = native_event.charactersIgnoringModifiers();
        if characters.is_null() {
            return None;
        }
        let utf8 = characters.UTF8String();
        if utf8.is_null() {
            return None;
        }
        let bytes = CStr::from_ptr(utf8).to_bytes();
        std::str::from_utf8(bytes)
            .ok()
            .and_then(|s| s.chars().next())
            .map(|ch| ch as u32)
            .and_then(|ch| u16::try_from(ch).ok())
    }
}

fn symbolic_hotkey_modifiers(modifier_flags: NSEventModifierFlags) -> u32 {
    let mut modifiers = 0;

    if modifier_flags.contains(NSEventModifierFlags::NSCommandKeyMask) {
        modifiers |= COMMAND_MODIFIER;
    }
    if modifier_flags.contains(NSEventModifierFlags::NSShiftKeyMask) {
        modifiers |= SHIFT_MODIFIER;
    }
    if modifier_flags.contains(NSEventModifierFlags::NSAlternateKeyMask) {
        modifiers |= OPTION_MODIFIER;
    }
    if modifier_flags.contains(NSEventModifierFlags::NSControlKeyMask) {
        modifiers |= CONTROL_MODIFIER;
    }
    if modifier_flags.contains(NSEventModifierFlags::NSFunctionKeyMask) {
        modifiers |= FUNCTION_MODIFIER;
    }

    modifiers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_by_character_when_entry_has_one() {
        let hotkey = WindowCycleHotkey {
            character: Some(0x60),
            key_code: Some(50),
            modifiers: COMMAND_MODIFIER,
        };
        assert!(hotkey.matches(Some(0x60), 10, COMMAND_MODIFIER));
        assert!(!hotkey.matches(Some(0xBA), 50, COMMAND_MODIFIER));
        assert!(!hotkey.matches(None, 50, COMMAND_MODIFIER));
    }

    #[test]
    fn matches_by_key_code_when_entry_has_no_character() {
        let hotkey = WindowCycleHotkey {
            character: None,
            key_code: Some(10),
            modifiers: COMMAND_MODIFIER,
        };
        assert!(hotkey.matches(None, 10, COMMAND_MODIFIER));
        assert!(hotkey.matches(Some(0x60), 10, COMMAND_MODIFIER));
        assert!(!hotkey.matches(Some(0x60), 50, COMMAND_MODIFIER));
    }

    #[test]
    fn rejects_mismatched_modifiers() {
        let hotkey = WindowCycleHotkey {
            character: Some(0x60),
            key_code: Some(50),
            modifiers: COMMAND_MODIFIER,
        };
        assert!(!hotkey.matches(Some(0x60), 50, COMMAND_MODIFIER | SHIFT_MODIFIER));
    }

    #[test]
    fn reads_current_window_cycle_from_macos() {
        // Integration check: the CGS API returns some shortcut on every
        // macOS install (either the plist override or the SkyLight default).
        // We don't assert on the exact value because it's layout/user-specific.
        let hotkey = read_window_cycle_hotkey();
        assert!(hotkey.is_some());
        assert_eq!(
            hotkey.unwrap().modifiers & COMMAND_MODIFIER,
            COMMAND_MODIFIER
        );
    }

    #[test]
    fn normalizes_native_modifier_flags() {
        let modifiers = symbolic_hotkey_modifiers(
            NSEventModifierFlags::NSCommandKeyMask
                | NSEventModifierFlags::NSShiftKeyMask
                | NSEventModifierFlags::NSAlphaShiftKeyMask,
        );

        assert_eq!(modifiers, COMMAND_MODIFIER | SHIFT_MODIFIER);
    }
}
