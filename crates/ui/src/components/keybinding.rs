use crate::{h_flex, prelude::*, Icon, IconName, IconSize};
use gpui::{relative, rems, Action, FocusHandle, IntoElement, Keystroke};

/// The way a [`KeyBinding`] should be displayed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum KeyBindingDisplay {
    /// Display in macOS style.
    Mac,
    /// Display in Linux style.
    Linux,
    /// Display in Windows style.
    Windows,
}

impl KeyBindingDisplay {
    /// Returns the [`KeyBindingDisplay`] for the current platform.
    pub const fn platform() -> Self {
        if cfg!(target_os = "linux") {
            KeyBindingDisplay::Linux
        } else if cfg!(target_os = "windows") {
            KeyBindingDisplay::Windows
        } else {
            KeyBindingDisplay::Mac
        }
    }
}

#[derive(IntoElement, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,

    /// How keybindings should be displayed.
    display: KeyBindingDisplay,
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
            display: KeyBindingDisplay::platform(),
        }
    }

    /// Sets how this [`KeyBinding`] should be displayed.
    pub fn display(mut self, display: KeyBindingDisplay) -> Self {
        self.display = display;
        self
    }
}

impl RenderOnce for KeyBinding {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .flex_none()
            .gap_2()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                let key_icon = Self::icon_for_key(keystroke);

                h_flex()
                    .flex_none()
                    .map(|el| match self.display {
                        KeyBindingDisplay::Mac => el.gap_0p5(),
                        KeyBindingDisplay::Linux | KeyBindingDisplay::Windows => el,
                    })
                    .p_0p5()
                    .rounded_sm()
                    .text_color(cx.theme().colors().text_muted)
                    .when(keystroke.modifiers.function, |el| match self.display {
                        KeyBindingDisplay::Mac => el.child(Key::new("fn")),
                        KeyBindingDisplay::Linux | KeyBindingDisplay::Windows => {
                            el.child(Key::new("Fn")).child(Key::new("+"))
                        }
                    })
                    .when(keystroke.modifiers.control, |el| match self.display {
                        KeyBindingDisplay::Mac => el.child(KeyIcon::new(IconName::Control)),
                        KeyBindingDisplay::Linux | KeyBindingDisplay::Windows => {
                            el.child(Key::new("Ctrl")).child(Key::new("+"))
                        }
                    })
                    .when(keystroke.modifiers.alt, |el| match self.display {
                        KeyBindingDisplay::Mac => el.child(KeyIcon::new(IconName::Option)),
                        KeyBindingDisplay::Linux | KeyBindingDisplay::Windows => {
                            el.child(Key::new("Alt")).child(Key::new("+"))
                        }
                    })
                    .when(keystroke.modifiers.command, |el| match self.display {
                        KeyBindingDisplay::Mac => el.child(KeyIcon::new(IconName::Command)),
                        KeyBindingDisplay::Linux => {
                            el.child(Key::new("Super")).child(Key::new("+"))
                        }
                        KeyBindingDisplay::Windows => {
                            el.child(Key::new("Win")).child(Key::new("+"))
                        }
                    })
                    .when(keystroke.modifiers.shift, |el| match self.display {
                        KeyBindingDisplay::Mac => el.child(KeyIcon::new(IconName::Option)),
                        KeyBindingDisplay::Linux | KeyBindingDisplay::Windows => {
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
                    this.w(rems(14. / 16.)).flex().flex_none().justify_center()
                } else {
                    this.px_0p5()
                }
            })
            .h(rems(14. / 16.))
            .text_ui()
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
        div().w(rems(14. / 16.)).child(
            Icon::new(self.icon)
                .size(IconSize::Small)
                .color(Color::Muted),
        )
    }
}

impl KeyIcon {
    pub fn new(icon: IconName) -> Self {
        Self { icon }
    }
}
