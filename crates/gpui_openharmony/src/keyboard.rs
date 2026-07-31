use gpui::{KeybindingKeystroke, Keystroke, PlatformKeyboardLayout, PlatformKeyboardMapper};

pub(crate) struct OpenHarmonyKeyboardLayout;

impl PlatformKeyboardLayout for OpenHarmonyKeyboardLayout {
    fn id(&self) -> &str {
        "openharmony-us"
    }

    fn name(&self) -> &str {
        "OpenHarmony US"
    }
}

pub(crate) struct OpenHarmonyKeyboardMapper;

impl PlatformKeyboardMapper for OpenHarmonyKeyboardMapper {
    fn map_key_equivalent(
        &self,
        keystroke: Keystroke,
        _use_key_equivalents: bool,
    ) -> KeybindingKeystroke {
        KeybindingKeystroke::from_keystroke(keystroke)
    }

    fn get_key_equivalents(&self) -> Option<&collections::HashMap<char, char>> {
        None
    }
}
