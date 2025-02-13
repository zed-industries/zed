#![allow(missing_docs)]
use crate::PlatformStyle;
use crate::{h_flex, prelude::*, Icon, IconName, IconSize};
use gpui::{
    relative, Action, AnyElement, App, FocusHandle, IntoElement, Keystroke, Modifiers, Window,
};

#[derive(Debug, IntoElement, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,

    /// The [`PlatformStyle`] to use when displaying this keybinding.
    platform_style: PlatformStyle,
    size: Option<AbsoluteLength>,
}

impl KeyBinding {
    /// Returns the highest precedence keybinding for an action. This is the last binding added to
    /// the keymap. User bindings are added after built-in bindings so that they take precedence.
    pub fn for_action(action: &dyn Action, window: &mut Window) -> Option<Self> {
        let key_binding = window
            .bindings_for_action(action)
            .into_iter()
            .rev()
            .next()?;
        Some(Self::new(key_binding))
    }

    /// Like `for_action`, but lets you specify the context from which keybindings are matched.
    pub fn for_action_in(
        action: &dyn Action,
        focus: &FocusHandle,
        window: &mut Window,
    ) -> Option<Self> {
        let key_binding = window
            .bindings_for_action_in(action, focus)
            .into_iter()
            .rev()
            .next()?;
        Some(Self::new(key_binding))
    }

    pub fn new(key_binding: gpui::KeyBinding) -> Self {
        Self {
            key_binding,
            platform_style: PlatformStyle::platform(),
            size: None,
        }
    }

    /// Sets the [`PlatformStyle`] for this [`KeyBinding`].
    pub fn platform_style(mut self, platform_style: PlatformStyle) -> Self {
        self.platform_style = platform_style;
        self
    }

    /// Sets the size for this [`KeyBinding`].
    pub fn size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl RenderOnce for KeyBinding {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
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
            .gap(DynamicSpacing::Base04.rems(cx))
            .flex_none()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                h_flex()
                    .flex_none()
                    .py_0p5()
                    .rounded_sm()
                    .text_color(cx.theme().colors().text_muted)
                    .children(render_modifiers(
                        &keystroke.modifiers,
                        self.platform_style,
                        None,
                        self.size,
                        true,
                    ))
                    .map(|el| {
                        el.child(render_key(&keystroke, self.platform_style, None, self.size))
                    })
            }))
    }
}

pub fn render_key(
    keystroke: &Keystroke,
    platform_style: PlatformStyle,
    color: Option<Color>,
    size: Option<AbsoluteLength>,
) -> AnyElement {
    let key_icon = icon_for_key(keystroke, platform_style);
    match key_icon {
        Some(icon) => KeyIcon::new(icon, color).size(size).into_any_element(),
        None => Key::new(util::capitalize(&keystroke.key), color)
            .size(size)
            .into_any_element(),
    }
}

fn icon_for_key(keystroke: &Keystroke, platform_style: PlatformStyle) -> Option<IconName> {
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
        "shift" if platform_style == PlatformStyle::Mac => Some(IconName::Shift),
        "control" if platform_style == PlatformStyle::Mac => Some(IconName::Control),
        "platform" if platform_style == PlatformStyle::Mac => Some(IconName::Command),
        "function" if platform_style == PlatformStyle::Mac => Some(IconName::Control),
        "alt" if platform_style == PlatformStyle::Mac => Some(IconName::Option),
        _ => None,
    }
}

pub fn render_modifiers(
    modifiers: &Modifiers,
    platform_style: PlatformStyle,
    color: Option<Color>,
    size: Option<AbsoluteLength>,
    trailing_separator: bool,
) -> impl Iterator<Item = AnyElement> {
    #[derive(Clone)]
    enum KeyOrIcon {
        Key(&'static str),
        Plus,
        Icon(IconName),
    }

    struct Modifier {
        enabled: bool,
        mac: KeyOrIcon,
        linux: KeyOrIcon,
        windows: KeyOrIcon,
    }

    let table = {
        use KeyOrIcon::*;

        [
            Modifier {
                enabled: modifiers.function,
                mac: Icon(IconName::Control),
                linux: Key("Fn"),
                windows: Key("Fn"),
            },
            Modifier {
                enabled: modifiers.control,
                mac: Icon(IconName::Control),
                linux: Key("Ctrl"),
                windows: Key("Ctrl"),
            },
            Modifier {
                enabled: modifiers.alt,
                mac: Icon(IconName::Option),
                linux: Key("Alt"),
                windows: Key("Alt"),
            },
            Modifier {
                enabled: modifiers.platform,
                mac: Icon(IconName::Command),
                linux: Key("Super"),
                windows: Key("Win"),
            },
            Modifier {
                enabled: modifiers.shift,
                mac: Icon(IconName::Shift),
                linux: Key("Shift"),
                windows: Key("Shift"),
            },
        ]
    };

    let filtered = table
        .into_iter()
        .filter(|modifier| modifier.enabled)
        .collect::<Vec<_>>();

    let platform_keys = filtered
        .into_iter()
        .map(move |modifier| match platform_style {
            PlatformStyle::Mac => Some(modifier.mac),
            PlatformStyle::Linux => Some(modifier.linux),
            PlatformStyle::Windows => Some(modifier.windows),
        });

    let separator = match platform_style {
        PlatformStyle::Mac => None,
        PlatformStyle::Linux => Some(KeyOrIcon::Plus),
        PlatformStyle::Windows => Some(KeyOrIcon::Plus),
    };

    let platform_keys = itertools::intersperse(platform_keys, separator.clone());

    platform_keys
        .chain(if modifiers.modified() && trailing_separator {
            Some(separator)
        } else {
            None
        })
        .flatten()
        .map(move |key_or_icon| match key_or_icon {
            KeyOrIcon::Key(key) => Key::new(key, color).size(size).into_any_element(),
            KeyOrIcon::Icon(icon) => KeyIcon::new(icon, color).size(size).into_any_element(),
            KeyOrIcon::Plus => "+".into_any_element(),
        })
}

#[derive(IntoElement)]
pub struct Key {
    key: SharedString,
    color: Option<Color>,
    size: Option<AbsoluteLength>,
}

impl RenderOnce for Key {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let single_char = self.key.len() == 1;
        let size = self
            .size
            .unwrap_or_else(|| TextSize::default().rems(cx).into());

        div()
            .py_0()
            .map(|this| {
                if single_char {
                    this.w(size).flex().flex_none().justify_center()
                } else {
                    this.px_0p5()
                }
            })
            .h(size)
            .text_size(size)
            .line_height(relative(1.))
            .text_color(self.color.unwrap_or(Color::Muted).color(cx))
            .child(self.key.clone())
    }
}

impl Key {
    pub fn new(key: impl Into<SharedString>, color: Option<Color>) -> Self {
        Self {
            key: key.into(),
            color,
            size: None,
        }
    }

    pub fn size(mut self, size: impl Into<Option<AbsoluteLength>>) -> Self {
        self.size = size.into();
        self
    }
}

#[derive(IntoElement)]
pub struct KeyIcon {
    icon: IconName,
    color: Option<Color>,
    size: Option<AbsoluteLength>,
}

impl RenderOnce for KeyIcon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = self.size.unwrap_or(IconSize::Small.rems().into());

        Icon::new(self.icon)
            .size(IconSize::Custom(size.to_rems(window.rem_size())))
            .color(self.color.unwrap_or(Color::Muted))
    }
}

impl KeyIcon {
    pub fn new(icon: IconName, color: Option<Color>) -> Self {
        Self {
            icon,
            color,
            size: None,
        }
    }

    pub fn size(mut self, size: impl Into<Option<AbsoluteLength>>) -> Self {
        self.size = size.into();
        self
    }
}

/// Returns a textual representation of the key binding for the given [`Action`].
pub fn text_for_action(action: &dyn Action, window: &Window) -> Option<String> {
    let bindings = window.bindings_for_action(action);
    let key_binding = bindings.last()?;
    Some(text_for_key_binding(key_binding, PlatformStyle::platform()))
}

/// Returns a textual representation of the key binding for the given [`Action`]
/// as if the provided [`FocusHandle`] was focused.
pub fn text_for_action_in(
    action: &dyn Action,
    focus: &FocusHandle,
    window: &mut Window,
) -> Option<String> {
    let bindings = window.bindings_for_action_in(action, focus);
    let key_binding = bindings.last()?;
    Some(text_for_key_binding(key_binding, PlatformStyle::platform()))
}

/// Returns a textual representation of the given key binding for the specified platform.
pub fn text_for_key_binding(
    key_binding: &gpui::KeyBinding,
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

    let key = match keystroke.key.as_str() {
        "pageup" => "PageUp",
        "pagedown" => "PageDown",
        key => &util::capitalize(key),
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
