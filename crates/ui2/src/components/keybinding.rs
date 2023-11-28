use crate::{h_stack, prelude::*, Icon, IconElement, IconSize};
use gpui::{relative, rems, Action, Div, IntoElement, Keystroke};

#[derive(IntoElement, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,
}

impl RenderOnce for KeyBinding {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
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
                    // .when_some(key_icon, |el, icon| el.child(KeyIcon::new(icon)))
                    .when(key_icon.is_none(), |el| {
                        el.child(Key::new(keystroke.key.to_uppercase().clone()))
                    })
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

    fn icon_for_key(keystroke: &Keystroke) -> Option<Icon> {
        let mut icon: Option<Icon> = None;

        if keystroke.key == "left".to_string() {
            icon = Some(Icon::ArrowLeft);
        } else if keystroke.key == "right".to_string() {
            icon = Some(Icon::ArrowRight);
        } else if keystroke.key == "up".to_string() {
            icon = Some(Icon::ArrowUp);
        } else if keystroke.key == "down".to_string() {
            icon = Some(Icon::ArrowDown);
        }

        icon
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
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let single_char = self.key.len() == 1;

        div()
            .py_0()
            .when(single_char, |el| {
                el.w(rems(14. / 16.)).flex().flex_none().justify_center()
            })
            .when(!single_char, |el| el.px_0p5())
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
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
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
