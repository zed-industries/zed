use gpui::{actions, relative, rems, Action, Styled};
use strum::EnumIter;

use crate::{h_stack, prelude::*, Icon, IconElement, IconSize};

#[derive(Component, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,
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

    fn icon_for_key(key: &str) -> Option<Icon> {
        match key {
            "left" => Some(Icon::ArrowLeft),
            "right" => Some(Icon::ArrowRight),
            "up" => Some(Icon::ArrowUp),
            "down" => Some(Icon::ArrowDown),
            _ => None,
        }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        h_stack()
            .flex_none()
            .gap_1()
            .children(self.key_binding.keystrokes().iter().map(|keystroke| {
                let key_icon = Self::icon_for_key(&keystroke.key);

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

#[derive(Component)]
pub struct Key {
    key: SharedString,
}

impl Key {
    pub fn new(key: impl Into<SharedString>) -> Self {
        Self { key: key.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let single_char = self.key.len() == 1;

        div()
            // .px_0p5()
            .py_0()
            .when(single_char, |el| {
                el.w(rems(14. / 16.)).flex().flex_none().justify_center()
            })
            .when(!single_char, |el| el.px_0p5())
            .h(rems(14. / 16.))
            // .rounded_md()
            .text_ui()
            .line_height(relative(1.))
            .text_color(cx.theme().colors().text)
            // .bg(cx.theme().colors().element_background)
            .child(self.key.clone())
    }
}

#[derive(Component)]
pub struct KeyIcon {
    icon: Icon,
}

impl KeyIcon {
    pub fn new(icon: Icon) -> Self {
        Self { icon }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div()
            .w(rems(14. / 16.))
            // .bg(cx.theme().colors().element_background)
            .child(IconElement::new(self.icon).size(IconSize::Small))
    }
}

// NOTE: The order the modifier keys appear in this enum impacts the order in
// which they are rendered in the UI.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum ModifierKey {
    Control,
    Alt, // Option
    Shift,
    Command,
}

actions!(NoAction);

pub fn binding(key: &str) -> gpui::KeyBinding {
    gpui::KeyBinding::new(key, NoAction {}, None)
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    pub use crate::KeyBinding;
    use crate::{binding, Story};
    use gpui::{Div, Render};
    use itertools::Itertools;
    pub struct KeybindingStory;

    impl Render for KeybindingStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            let all_modifier_permutations =
                ["ctrl", "alt", "cmd", "shift"].into_iter().permutations(2);

            Story::container(cx)
                .child(Story::title_for::<_, KeyBinding>(cx))
                .child(Story::label(cx, "Single Key"))
                .child(KeyBinding::new(binding("Z")))
                .child(Story::label(cx, "Single Key with Modifier"))
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .child(KeyBinding::new(binding("ctrl-c")))
                        .child(KeyBinding::new(binding("alt-c")))
                        .child(KeyBinding::new(binding("cmd-c")))
                        .child(KeyBinding::new(binding("shift-c"))),
                )
                .child(Story::label(cx, "Single Key with Modifier (Permuted)"))
                .child(
                    div().flex().flex_col().children(
                        all_modifier_permutations
                            .chunks(4)
                            .into_iter()
                            .map(|chunk| {
                                div()
                                    .flex()
                                    .gap_4()
                                    .py_3()
                                    .children(chunk.map(|permutation| {
                                        KeyBinding::new(binding(&*(permutation.join("-") + "-x")))
                                    }))
                            }),
                    ),
                )
                .child(Story::label(cx, "Single Key with All Modifiers"))
                .child(KeyBinding::new(binding("ctrl-alt-cmd-shift-z")))
                .child(Story::label(cx, "Chord"))
                .child(KeyBinding::new(binding("a z")))
                .child(Story::label(cx, "Chord with Modifier"))
                .child(KeyBinding::new(binding("ctrl-a shift-z")))
                .child(KeyBinding::new(binding("fn-s")))
        }
    }
}
