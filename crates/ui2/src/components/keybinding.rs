use crate::{h_stack, prelude::*, Icon, IconElement, IconSize};
use gpui::{relative, rems, Action, Div, FocusHandle, IntoElement, Keystroke};

#[derive(IntoElement, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,
}

impl RenderOnce for KeyBinding {
    type Output = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Output {
        h_stack()
            .flex_none()
            .gap_2()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                let key_icon = Self::icon_for_key(&keystroke);

                h_stack()
                    .flex_none()
                    .gap_0p5()
                    .bg(cx.theme().colors().element_background)
                    .p_0p5()
                    .rounded_sm()
                    .when(keystroke.modifiers.function, |el| el.child(Key::new("fn")))
                    .when(keystroke.modifiers.control, |el| {
                        el.child(KeyIcon::new(Icon::Control))
                    })
                    .when(keystroke.modifiers.alt, |el| {
                        el.child(KeyIcon::new(Icon::Option))
                    })
                    .when(keystroke.modifiers.command, |el| {
                        el.child(KeyIcon::new(Icon::Command))
                    })
                    .when(keystroke.modifiers.shift, |el| {
                        el.child(KeyIcon::new(Icon::Shift))
                    })
                    .when_some(key_icon, |el, icon| el.child(KeyIcon::new(icon)))
                    .when(key_icon.is_none(), |el| {
                        el.child(Key::new(keystroke.key.to_uppercase().clone()))
                    })
            }))
    }
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

    fn icon_for_key(keystroke: &Keystroke) -> Option<Icon> {
        match keystroke.key.as_str() {
            "backspace" => Some(Icon::Backspace),
            "delete" => Some(Icon::Delete),
            "down" => Some(Icon::ArrowDown),
            "enter" => Some(Icon::Return),
            "escape" => Some(Icon::Escape),
            "left" => Some(Icon::ArrowLeft),
            "pagedown" => Some(Icon::PageDown),
            "pageup" => Some(Icon::PageUp),
            "return" => Some(Icon::Return),
            "right" => Some(Icon::ArrowRight),
            "space" => Some(Icon::Space),
            "tab" => Some(Icon::Tab),
            "up" => Some(Icon::ArrowUp),
            _ => None,
        }
    }

    pub fn new(key_binding: gpui::KeyBinding) -> Self {
        Self { key_binding }
    }
}

#[derive(IntoElement)]
pub struct Key {
    key: SharedString,
}

impl RenderOnce for Key {
    type Output = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Output {
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
            .text_color(cx.theme().colors().text)
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
    icon: Icon,
}

impl RenderOnce for KeyIcon {
    type Output = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Output {
        div()
            .w(rems(14. / 16.))
            .child(IconElement::new(self.icon).size(IconSize::Small))
    }
}

impl KeyIcon {
    pub fn new(icon: Icon) -> Self {
        Self { icon }
    }
}
