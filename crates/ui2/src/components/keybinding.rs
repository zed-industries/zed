use crate::prelude::*;
use gpui::{Action, Div, RenderOnce};

#[derive(RenderOnce, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,
}

impl Component for KeyBinding {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .flex()
            .gap_2()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                div()
                    .flex()
                    .gap_1()
                    .when(keystroke.modifiers.function, |el| el.child(Key::new("fn")))
                    .when(keystroke.modifiers.control, |el| el.child(Key::new("^")))
                    .when(keystroke.modifiers.alt, |el| el.child(Key::new("⌥")))
                    .when(keystroke.modifiers.command, |el| el.child(Key::new("⌘")))
                    .when(keystroke.modifiers.shift, |el| el.child(Key::new("⇧")))
                    .child(Key::new(keystroke.key.clone()))
            }))
    }
}

impl KeyBinding {
    pub fn for_action(action: &dyn Action, cx: &mut WindowContext) -> Option<Self> {
        // todo! this last is arbitrary, we want to prefer users key bindings over defaults,
        // and vim over normal (in vim mode), etc.
        let key_binding = cx.bindings_for_action(action).last().cloned()?;
        Some(Self::new(key_binding))
    }

    pub fn new(key_binding: gpui::KeyBinding) -> Self {
        Self { key_binding }
    }
}

#[derive(RenderOnce)]
pub struct Key {
    key: SharedString,
}

impl Component for Key {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .px_2()
            .py_0()
            .rounded_md()
            .text_ui_sm()
            .text_color(cx.theme().colors().text)
            .bg(cx.theme().colors().element_background)
            .child(self.key.clone())
    }
}

impl Key {
    pub fn new(key: impl Into<SharedString>) -> Self {
        Self { key: key.into() }
    }
}
