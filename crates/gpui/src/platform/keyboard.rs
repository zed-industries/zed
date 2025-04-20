use std::borrow::Cow;

use super::{Keystroke, WindowsKeyboardMapper};

/// A trait for platform-specific keyboard layouts
pub trait PlatformKeyboardLayout {
    /// Get the keyboard layout ID, which should be unique to the layout
    fn id(&self) -> &str;
    /// Get the keyboard layout display name
    fn name(&self) -> &str;
}

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke;
    /// TODO:
    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke>;
}

/// TODO:
pub struct EmptyKeyboardMapper;

impl KeyboardMapper for EmptyKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, _: bool) -> Keystroke {
        keystroke
    }

    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke> {
        Cow::Borrowed(keystroke)
    }
}

/// TODO:
pub struct TestKeyboardMapper {
    mapper: WindowsKeyboardMapper,
}

impl TestKeyboardMapper {
    /// TODO:
    pub fn new() -> Self {
        Self {
            mapper: WindowsKeyboardMapper::new(),
        }
    }
}

impl KeyboardMapper for TestKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke {
        self.mapper.map_keystroke(keystroke, use_key_equivalents)
    }

    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke> {
        self.mapper.to_vim_keystroke(keystroke)
    }
}
