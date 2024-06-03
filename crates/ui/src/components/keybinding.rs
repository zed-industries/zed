use crate::{h_flex, prelude::*, Icon, IconName, IconSize};
use gpui::{relative, Action, FocusHandle, IntoElement, Keystroke};

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

    fn icon_for_key(keystroke: &Keystroke) -> Option<IconName> {
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
            .gap(rems(0.125))
            .flex_none()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                let key_icon = Self::icon_for_key(keystroke);

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
