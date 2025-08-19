use crate::{KeybindingKeystroke, Keystroke};

/// A trait for platform-specific keyboard layouts
pub trait PlatformKeyboardLayout {
    /// Get the keyboard layout ID, which should be unique to the layout
    fn id(&self) -> &str;
    /// Get the keyboard layout display name
    fn name(&self) -> &str;
}

/// A trait for platform-specific keyboard mappings
pub trait PlatformKeyboardMapper {
    /// Map a key equivalent to its platform-specific representation
    fn map_key_equivalent(&self, keystroke: Keystroke) -> KeybindingKeystroke;
}

pub(crate) struct DummyKeyboardMapper;

impl PlatformKeyboardMapper for DummyKeyboardMapper {
    fn map_key_equivalent(&self, keystroke: Keystroke) -> KeybindingKeystroke {
        KeybindingKeystroke::new(keystroke)
    }
}
