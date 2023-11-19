use crate::prelude::*;
use gpui::{Action, Div, RenderOnce};
use strum::EnumIter;

#[derive(RenderOnce, Clone)]
pub struct KeyBinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    key_binding: gpui::KeyBinding,
}

impl<V: 'static> Component<V> for KeyBinding {
    type Rendered = Div<V>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
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

impl<V: 'static> Component<V> for Key {
    type Rendered = Div<V>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
        let _view: &mut V = view;
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

// NOTE: The order the modifier keys appear in this enum impacts the order in
// which they are rendered in the UI.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum ModifierKey {
    Control,
    Alt,
    Command,
    Shift,
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{actions, Div, Render};
    use itertools::Itertools;

    pub struct KeybindingStory;

    actions!(NoAction);

    pub fn binding(key: &str) -> gpui::KeyBinding {
        gpui::KeyBinding::new(key, NoAction {}, None)
    }

    impl Render<Self> for KeybindingStory {
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
