use gpui::{KeybindingKeystroke, Keystroke, Modifiers, PlatformKeyboardLayout, PlatformKeyboardMapper};

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
        use_key_equivalents: bool,
    ) -> KeybindingKeystroke {
        // On OpenHarmony there is no platform "Command" key.
        // When `use_key_equivalents` is requested, remap Ctrl → platform
        // modifier so that standard Ctrl+C/V/X/Z shortcuts activate the same
        // keybindings that macOS binds to Cmd.
        let mapped = if use_key_equivalents && keystroke.modifiers.control {
            Keystroke {
                modifiers: Modifiers {
                    control: false,
                    platform: true,
                    ..keystroke.modifiers
                },
                ..keystroke
            }
        } else {
            keystroke
        };
        KeybindingKeystroke::from_keystroke(mapped)
    }

    fn get_key_equivalents(&self) -> Option<&collections::HashMap<char, char>> {
        None
    }
}
