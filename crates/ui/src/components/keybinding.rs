use crate::{h_flex, prelude::*, Icon, IconName, IconSize};
use gpui::{relative, rems, Action, FocusHandle, IntoElement, Keystroke};

#[derive(IntoElement, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,
}

impl RenderOnce for KeyBinding {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .flex_none()
            .gap_2()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                let keybinding_children = get_os_format_keybinding(keystroke.clone());

                h_flex()
                    .flex_none()
                    .gap_0p5()
                    .p_0p5()
                    .rounded_sm()
                    .text_color(cx.theme().colors().text_muted)
                    .when(keybinding_children.0.is_some(), |el| {
                        el.child(keybinding_children.0.unwrap())
                    })
                    .children(keybinding_children.1)
                    .when(keybinding_children.2.is_some(), |el| {
                        el.child(keybinding_children.2.unwrap())
                    })
            }))
    }
}

// Children for a Keybinding.
// The first Key is used for `fn` on macOS, None otherwise
// The Vec is for key icons (macOS only)
// The Option<Key> is for the last character, None if it is a symbol (macOS), or the combination (all other OSes)
struct KeybindingChildren(Option<Key>, Vec<KeyIcon>, Option<Key>);

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

    pub fn new(key_binding: gpui::KeyBinding) -> Self {
        Self { key_binding }
    }
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

fn get_os_format_keybinding(keystroke: Keystroke) -> KeybindingChildren {
    let mut prefix_key: Option<Key> = None;
    let mut key_icons: Vec<KeyIcon> = vec![];
    let mut postfix_key: Option<Key> = None;

    if cfg!(target_os = "macos") {
        let key_icon = icon_for_key(&keystroke);

        if keystroke.modifiers.function {
            prefix_key = Some(Key::new("fn"));
        }

        if keystroke.modifiers.control {
            key_icons.push(KeyIcon::new(IconName::Control));
        }

        if keystroke.modifiers.alt {
            key_icons.push(KeyIcon::new(IconName::Option));
        }

        if keystroke.modifiers.command {
            key_icons.push(KeyIcon::new(IconName::Command));
        }

        if keystroke.modifiers.shift {
            key_icons.push(KeyIcon::new(IconName::Shift));
        }

        if let Some(icon) = key_icon {
            key_icons.push(KeyIcon::new(icon));
        } else {
            postfix_key = Some(Key::new(keystroke.key.to_uppercase().clone()));
        }
    } else {
        let win_key = if cfg!(target_os = "windows") {
            "Win"
        } else {
            // windows/cmd key is `Super` on linux and BSD
            "Super"
        };

        let mut keybinding_text = String::from("");

        if keystroke.modifiers.control {
            keybinding_text.push_str("Ctrl+");
        }

        if keystroke.modifiers.alt {
            keybinding_text.push_str("Alt+");
        }

        if keystroke.modifiers.shift {
            keybinding_text.push_str("Shift+");
        }

        if keystroke.modifiers.command {
            keybinding_text.push_str(win_key);
            keybinding_text.push_str("+");
        }

        if keystroke.modifiers.function {
            keybinding_text.push_str("Fn+");
        }

        let special_keys = match keystroke.key.as_str() {
            "left" => Some("Arrow Left"),
            "right" => Some("Arrow Right"),
            "up" => Some("Arrow Up"),
            "down" => Some("Arrow Down"),
            "backspace" => Some("Backspace"),
            "delete" => Some("Del"),
            "return" | "enter" => Some("Enter"),
            "tab" => Some("Tab"),
            "space" => Some("Space"),
            "escape" => Some("Esc"),
            "pagedown" => Some("PgDn"),
            "pageup" => Some("PgUp"),
            "home" => Some("Home"),
            "end" => Some("End"),
            _ => None,
        };

        if let Some(special_key) = special_keys {
            keybinding_text.push_str(special_key);
        } else {
            keybinding_text.push_str(&keystroke.key.to_uppercase());
        }

        postfix_key = Some(Key::new(keybinding_text))
    }
    KeybindingChildren(prefix_key, key_icons, postfix_key)
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
            .content_center()
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
