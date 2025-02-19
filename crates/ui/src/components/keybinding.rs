#![allow(missing_docs)]
use crate::PlatformStyle;
use crate::{h_flex, prelude::*, Icon, IconName, IconSize};
use gpui::{
    relative, Action, AnyElement, App, FocusHandle, Global, IntoElement, Keystroke, Modifiers,
    Window,
};
use itertools::Itertools;

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

    /// Determines whether the keybinding is meant for vim mode.
    vim_mode: bool,
}

struct VimStyle(bool);
impl Global for VimStyle {}

impl KeyBinding {
    /// Returns the highest precedence keybinding for an action. This is the last binding added to
    /// the keymap. User bindings are added after built-in bindings so that they take precedence.
    pub fn for_action(action: &dyn Action, window: &mut Window, cx: &App) -> Option<Self> {
        let key_binding = window
            .bindings_for_action(action)
            .into_iter()
            .rev()
            .next()?;
        Some(Self::new(key_binding, cx))
    }

    /// Like `for_action`, but lets you specify the context from which keybindings are matched.
    pub fn for_action_in(
        action: &dyn Action,
        focus: &FocusHandle,
        window: &mut Window,
        cx: &App,
    ) -> Option<Self> {
        let key_binding = window
            .bindings_for_action_in(action, focus)
            .into_iter()
            .rev()
            .next()?;
        Some(Self::new(key_binding, cx))
    }

    pub fn set_vim_mode(cx: &mut App, enabled: bool) {
        cx.set_global(VimStyle(enabled));
    }

    fn is_vim_mode(cx: &App) -> bool {
        cx.try_global::<VimStyle>().is_some_and(|g| g.0)
    }

    pub fn new(key_binding: gpui::KeyBinding, cx: &App) -> Self {
        Self {
            key_binding,
            platform_style: PlatformStyle::platform(),
            size: None,
            vim_mode: KeyBinding::is_vim_mode(cx),
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

    pub fn vim_mode(mut self, enabled: bool) -> Self {
        self.vim_mode = enabled;
        self
    }

    fn render_key(&self, keystroke: &Keystroke, color: Option<Color>) -> AnyElement {
        let key_icon = icon_for_key(keystroke, self.platform_style);
        match key_icon {
            Some(icon) => KeyIcon::new(icon, color).size(self.size).into_any_element(),
            None => {
                let key = util::capitalize(&keystroke.key);
                Key::new(&key, color).size(self.size).into_any_element()
            }
        }
    }
}

impl RenderOnce for KeyBinding {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let use_text = self.vim_mode
            || matches!(
                self.platform_style,
                PlatformStyle::Linux | PlatformStyle::Windows
            );
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
                    .when(use_text, |el| {
                        el.child(
                            Key::new(
                                keystroke_text(&keystroke, self.platform_style, self.vim_mode),
                                None,
                            )
                            .size(self.size),
                        )
                    })
                    .when(!use_text, |el| {
                        el.children(render_modifiers(
                            &keystroke.modifiers,
                            self.platform_style,
                            None,
                            self.size,
                            true,
                        ))
                        .map(|el| el.child(self.render_key(&keystroke, None)))
                    })
            }))
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
pub fn text_for_action(action: &dyn Action, window: &Window, cx: &App) -> Option<String> {
    let bindings = window.bindings_for_action(action);
    let key_binding = bindings.last()?;
    Some(text_for_keystrokes(key_binding.keystrokes(), cx))
}

pub fn text_for_keystrokes(keystrokes: &[Keystroke], cx: &App) -> String {
    let platform_style = PlatformStyle::platform();
    let vim_enabled = cx.try_global::<VimStyle>().is_some();
    keystrokes
        .iter()
        .map(|keystroke| keystroke_text(keystroke, platform_style, vim_enabled))
        .join(" ")
}

pub fn text_for_keystroke(keystroke: &Keystroke, cx: &App) -> String {
    let platform_style = PlatformStyle::platform();
    let vim_enabled = cx.try_global::<VimStyle>().is_some();
    keystroke_text(keystroke, platform_style, vim_enabled)
}

/// Returns a textual representation of the given [`Keystroke`].
fn keystroke_text(keystroke: &Keystroke, platform_style: PlatformStyle, vim_mode: bool) -> String {
    let mut text = String::new();

    let delimiter = match (platform_style, vim_mode) {
        (PlatformStyle::Mac, false) => '-',
        (PlatformStyle::Linux | PlatformStyle::Windows, false) => '-',
        (_, true) => '-',
    };

    if keystroke.modifiers.function {
        match vim_mode {
            false => text.push_str("Fn"),
            true => text.push_str("fn"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.control {
        match (platform_style, vim_mode) {
            (PlatformStyle::Mac, false) => text.push_str("Control"),
            (PlatformStyle::Linux | PlatformStyle::Windows, false) => text.push_str("Ctrl"),
            (_, true) => text.push_str("ctrl"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.platform {
        match (platform_style, vim_mode) {
            (PlatformStyle::Mac, false) => text.push_str("Command"),
            (PlatformStyle::Mac, true) => text.push_str("cmd"),
            (PlatformStyle::Linux, false) => text.push_str("Super"),
            (PlatformStyle::Linux, true) => text.push_str("super"),
            (PlatformStyle::Windows, false) => text.push_str("Win"),
            (PlatformStyle::Windows, true) => text.push_str("win"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.alt {
        match (platform_style, vim_mode) {
            (PlatformStyle::Mac, false) => text.push_str("Option"),
            (PlatformStyle::Linux | PlatformStyle::Windows, false) => text.push_str("Alt"),
            (_, true) => text.push_str("alt"),
        }

        text.push(delimiter);
    }

    if keystroke.modifiers.shift {
        match (platform_style, vim_mode) {
            (_, false) => text.push_str("Shift"),
            (_, true) => text.push_str("shift"),
        }
        text.push(delimiter);
    }

    if vim_mode {
        text.push_str(&keystroke.key)
    } else {
        let key = match keystroke.key.as_str() {
            "pageup" => "PageUp",
            "pagedown" => "PageDown",
            key => &util::capitalize(key),
        };
        text.push_str(key);
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_for_keystroke() {
        assert_eq!(
            keystroke_text(
                &Keystroke::parse("cmd-c").unwrap(),
                PlatformStyle::Mac,
                false
            ),
            "Command-C".to_string()
        );
        assert_eq!(
            keystroke_text(
                &Keystroke::parse("cmd-c").unwrap(),
                PlatformStyle::Linux,
                false
            ),
            "Super-C".to_string()
        );
        assert_eq!(
            keystroke_text(
                &Keystroke::parse("cmd-c").unwrap(),
                PlatformStyle::Windows,
                false
            ),
            "Win-C".to_string()
        );

        assert_eq!(
            keystroke_text(
                &Keystroke::parse("ctrl-alt-delete").unwrap(),
                PlatformStyle::Mac,
                false
            ),
            "Control-Option-Delete".to_string()
        );
        assert_eq!(
            keystroke_text(
                &Keystroke::parse("ctrl-alt-delete").unwrap(),
                PlatformStyle::Linux,
                false
            ),
            "Ctrl-Alt-Delete".to_string()
        );
        assert_eq!(
            keystroke_text(
                &Keystroke::parse("ctrl-alt-delete").unwrap(),
                PlatformStyle::Windows,
                false
            ),
            "Ctrl-Alt-Delete".to_string()
        );

        assert_eq!(
            keystroke_text(
                &Keystroke::parse("shift-pageup").unwrap(),
                PlatformStyle::Mac,
                false
            ),
            "Shift-PageUp".to_string()
        );
        assert_eq!(
            keystroke_text(
                &Keystroke::parse("shift-pageup").unwrap(),
                PlatformStyle::Linux,
                false,
            ),
            "Shift-PageUp".to_string()
        );
        assert_eq!(
            keystroke_text(
                &Keystroke::parse("shift-pageup").unwrap(),
                PlatformStyle::Windows,
                false
            ),
            "Shift-PageUp".to_string()
        );
    }
}
