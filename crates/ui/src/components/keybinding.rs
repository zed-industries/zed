#![allow(missing_docs)]
use crate::PlatformStyle;
use crate::{h_flex, prelude::*, Icon, IconName, IconSize};
use gpui::{relative, Action, FocusHandle, IntoElement, Keystroke, WindowContext};

#[derive(IntoElement, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,

    /// The [`PlatformStyle`] to use when displaying this keybinding.
    platform_style: PlatformStyle,
}

impl KeyBinding {
    pub fn for_action(action: &dyn Action, cx: &mut WindowContext) -> Option<Self> {
        let key_binding = cx.bindings_for_action(action).last().cloned()?;
        Some(Self::new(key_binding))
    }

    // like for_action(), but lets you specify the context from which keybindings
    // are matched.
    pub fn for_action_in(
        action: &dyn Action,
        focus: &FocusHandle,
        cx: &mut WindowContext,
    ) -> Option<Self> {
        let key_binding = cx.bindings_for_action_in(action, focus).last().cloned()?;
        Some(Self::new(key_binding))
    }

    fn icon_for_key(&self, keystroke: &Keystroke) -> Option<IconName> {
        match keystroke.key.as_str() {
            "left" => Some(IconName::ArrowLeft),
            "right" => Some(IconName::ArrowRight),
            "up" => Some(IconName::ArrowUp),
            "down" => Some(IconName::ArrowDown),
            "backspace" => Some(IconName::Backspace),
            "delete" => Some(IconName::Delete),
            "return" => Some(IconName::Return),
            "enter" => Some(IconName::Return),
            "tab" => Some(IconName::Tab),
            "space" => Some(IconName::Space),
            "escape" => Some(IconName::Escape),
            "pagedown" => Some(IconName::PageDown),
            "pageup" => Some(IconName::PageUp),
            "shift" if self.platform_style == PlatformStyle::Mac => Some(IconName::Shift),
            "control" if self.platform_style == PlatformStyle::Mac => Some(IconName::Control),
            "platform" if self.platform_style == PlatformStyle::Mac => Some(IconName::Command),
            "function" if self.platform_style == PlatformStyle::Mac => Some(IconName::Control),
            "alt" if self.platform_style == PlatformStyle::Mac => Some(IconName::Option),
            _ => None,
        }
    }

    pub fn new(key_binding: gpui::KeyBinding) -> Self {
        Self {
            key_binding,
            platform_style: PlatformStyle::platform(),
        }
    }

    /// Sets the [`PlatformStyle`] for this [`KeyBinding`].
    pub fn platform_style(mut self, platform_style: PlatformStyle) -> Self {
        self.platform_style = platform_style;
        self
    }
}

impl RenderOnce for KeyBinding {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .debug_selector(|| {
                format!(
                    "KEY_BINDING-{}",
                    self.key_binding
                        .keystrokes()
                        .iter()
                        .map(|k| k.key.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            })
            .gap(Spacing::Small.rems(cx))
            .flex_none()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                let key_icon = self.icon_for_key(keystroke);

                h_flex()
                    .flex_none()
                    .py_0p5()
                    .rounded_sm()
                    .text_color(cx.theme().colors().text_muted)
                    .when(keystroke.modifiers.function, |el| {
                        match self.platform_style {
                            PlatformStyle::Mac => el.child(Key::new("fn")),
                            PlatformStyle::Linux | PlatformStyle::Windows => {
                                el.child(Key::new("Fn")).child(Key::new("+"))
                            }
                        }
                    })
                    .when(keystroke.modifiers.control, |el| {
                        match self.platform_style {
                            PlatformStyle::Mac => el.child(KeyIcon::new(IconName::Control)),
                            PlatformStyle::Linux | PlatformStyle::Windows => {
                                el.child(Key::new("Ctrl")).child(Key::new("+"))
                            }
                        }
                    })
                    .when(keystroke.modifiers.alt, |el| match self.platform_style {
                        PlatformStyle::Mac => el.child(KeyIcon::new(IconName::Option)),
                        PlatformStyle::Linux | PlatformStyle::Windows => {
                            el.child(Key::new("Alt")).child(Key::new("+"))
                        }
                    })
                    .when(keystroke.modifiers.platform, |el| {
                        match self.platform_style {
                            PlatformStyle::Mac => el.child(KeyIcon::new(IconName::Command)),
                            PlatformStyle::Linux => {
                                el.child(Key::new("Super")).child(Key::new("+"))
                            }
                            PlatformStyle::Windows => {
                                el.child(Key::new("Win")).child(Key::new("+"))
                            }
                        }
                    })
                    .when(keystroke.modifiers.shift, |el| match self.platform_style {
                        PlatformStyle::Mac => el.child(KeyIcon::new(IconName::Shift)),
                        PlatformStyle::Linux | PlatformStyle::Windows => {
                            el.child(Key::new("Shift")).child(Key::new("+"))
                        }
                    })
                    .map(|el| match key_icon {
                        Some(icon) => el.child(KeyIcon::new(icon)),
                        None => el.child(Key::new(keystroke.key.to_uppercase())),
                    })
            }))
    }
}

#[derive(IntoElement)]
pub struct Key {
    key: SharedString,
}

impl RenderOnce for Key {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let single_char = self.key.len() == 1;

        div()
            .py_0()
            .map(|this| {
                if single_char {
                    this.w(rems_from_px(14.))
                        .flex()
                        .flex_none()
                        .justify_center()
                } else {
                    this.px_0p5()
                }
            })
            .h(rems_from_px(14.))
            .text_ui(cx)
            .line_height(relative(1.))
            .text_color(cx.theme().colors().text_muted)
            .child(self.key.clone())
    }
}

impl Key {
    pub fn new(key: impl Into<SharedString>) -> Self {
        Self { key: key.into() }
    }
}

#[derive(IntoElement)]
pub struct KeyIcon {
    icon: IconName,
}

impl RenderOnce for KeyIcon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        Icon::new(self.icon)
            .size(IconSize::Small)
            .color(Color::Muted)
    }
}

impl KeyIcon {
    pub fn new(icon: IconName) -> Self {
        Self { icon }
    }
}

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
pub fn text_for_key_binding(
    key_binding: gpui::KeyBinding,
    platform_style: PlatformStyle,
) -> String {
    key_binding
        .keystrokes()
        .iter()
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
