use gpui::{Action, FocusHandle, KeyBinding, Keystroke, WindowContext};

use crate::PlatformStyle;

/// Returns a textual representation of the key binding for the given [`Action`].
pub fn text_for_action(action: &dyn Action, cx: &mut WindowContext) -> Option<String> {
    let key_binding = cx.bindings_for_action(action).last().cloned()?;
    Some(text_for_key_binding(key_binding, PlatformStyle::platform()))
}

/// Returns a textual representation of the key binding for the given [`Action`]
/// as if the provided [`FocusHandle`] was focused.
pub fn text_for_action_in(
    action: &dyn Action,
    focus: &FocusHandle,
    cx: &mut WindowContext,
) -> Option<String> {
    let key_binding = cx.bindings_for_action_in(action, focus).last().cloned()?;
    Some(text_for_key_binding(key_binding, PlatformStyle::platform()))
}

/// Returns a textual representation of the given key binding for the specified platform.
pub fn text_for_key_binding(key_binding: KeyBinding, platform_style: PlatformStyle) -> String {
    key_binding
        .keystrokes()
        .into_iter()
        .map(|keystroke| text_for_keystroke(keystroke, platform_style))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Returns a textual representation of the given [`Keystroke`].
pub fn text_for_keystroke(keystroke: &Keystroke, platform_style: PlatformStyle) -> String {
    let mut text = String::new();

    let delimiter = match platform_style {
        PlatformStyle::Mac => '-',
        PlatformStyle::Linux | PlatformStyle::Windows => '+',
    };

    if keystroke.modifiers.function {
        match platform_style {
            PlatformStyle::Mac => text.push_str("fn"),
            PlatformStyle::Linux | PlatformStyle::Windows => text.push_str("Fn"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.control {
        match platform_style {
            PlatformStyle::Mac => text.push_str("Control"),
            PlatformStyle::Linux | PlatformStyle::Windows => text.push_str("Ctrl"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.alt {
        match platform_style {
            PlatformStyle::Mac => text.push_str("Option"),
            PlatformStyle::Linux | PlatformStyle::Windows => text.push_str("Alt"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.platform {
        match platform_style {
            PlatformStyle::Mac => text.push_str("Command"),
            PlatformStyle::Linux => text.push_str("Super"),
            PlatformStyle::Windows => text.push_str("Win"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.shift {
        match platform_style {
            PlatformStyle::Mac | PlatformStyle::Linux | PlatformStyle::Windows => {
                text.push_str("Shift")
            }
        }

        text.push(delimiter);
    }

    fn capitalize(str: &str) -> String {
        let mut chars = str.chars();
        match chars.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    let key = match keystroke.key.as_str() {
        "pageup" => "PageUp",
        "pagedown" => "PageDown",
        key => &capitalize(key),
    };

    text.push_str(key);

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_for_keystroke() {
        assert_eq!(
            text_for_keystroke(&Keystroke::parse("cmd-c").unwrap(), PlatformStyle::Mac),
            "Command-C".to_string()
        );
        assert_eq!(
            text_for_keystroke(&Keystroke::parse("cmd-c").unwrap(), PlatformStyle::Linux),
            "Super+C".to_string()
        );
        assert_eq!(
            text_for_keystroke(&Keystroke::parse("cmd-c").unwrap(), PlatformStyle::Windows),
            "Win+C".to_string()
        );

        assert_eq!(
            text_for_keystroke(
                &Keystroke::parse("ctrl-alt-delete").unwrap(),
                PlatformStyle::Mac
            ),
            "Control-Option-Delete".to_string()
        );
        assert_eq!(
            text_for_keystroke(
                &Keystroke::parse("ctrl-alt-delete").unwrap(),
                PlatformStyle::Linux
            ),
            "Ctrl+Alt+Delete".to_string()
        );
        assert_eq!(
            text_for_keystroke(
                &Keystroke::parse("ctrl-alt-delete").unwrap(),
                PlatformStyle::Windows
            ),
            "Ctrl+Alt+Delete".to_string()
        );

        assert_eq!(
            text_for_keystroke(
                &Keystroke::parse("shift-pageup").unwrap(),
                PlatformStyle::Mac
            ),
            "Shift-PageUp".to_string()
        );
        assert_eq!(
            text_for_keystroke(
                &Keystroke::parse("shift-pageup").unwrap(),
                PlatformStyle::Linux
            ),
            "Shift+PageUp".to_string()
        );
        assert_eq!(
            text_for_keystroke(
                &Keystroke::parse("shift-pageup").unwrap(),
                PlatformStyle::Windows
            ),
            "Shift+PageUp".to_string()
        );
    }
}
