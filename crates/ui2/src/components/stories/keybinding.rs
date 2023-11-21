use gpui::{actions, Div, Render};
use itertools::Itertools;

use crate::prelude::*;
use crate::{KeyBinding, Story};

pub struct KeybindingStory;

actions!(NoAction);

pub fn binding(key: &str) -> gpui::KeyBinding {
    gpui::KeyBinding::new(key, NoAction {}, None)
}

impl Render for KeybindingStory {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let all_modifier_permutations = ["ctrl", "alt", "cmd", "shift"].into_iter().permutations(2);

        Story::container(cx)
            .child(Story::title_for::<KeyBinding>(cx))
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
