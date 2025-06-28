use gpui::NoAction;
use gpui::Render;
use itertools::Itertools;
use story::Story;

use crate::{KeyBinding, prelude::*};

pub struct KeybindingStory;

pub fn binding(key: &str) -> gpui::KeyBinding {
    gpui::KeyBinding::new(key, NoAction {}, None)
}

impl Render for KeybindingStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_modifier_permutations = ["ctrl", "alt", "cmd", "shift"].into_iter().permutations(2);

        Story::container(cx)
            .child(Story::title_for::<KeyBinding>(cx))
            .child(Story::label("Single Key", cx))
            .child(KeyBinding::new(binding("Z"), cx))
            .child(Story::label("Single Key with Modifier", cx))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(KeyBinding::new(binding("ctrl-c"), cx))
                    .child(KeyBinding::new(binding("alt-c"), cx))
                    .child(KeyBinding::new(binding("cmd-c"), cx))
                    .child(KeyBinding::new(binding("shift-c"), cx)),
            )
            .child(Story::label("Single Key with Modifier (Permuted)", cx))
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
                                    KeyBinding::new(binding(&(permutation.join("-") + "-x")), cx)
                                }))
                        }),
                ),
            )
            .child(Story::label("Single Key with All Modifiers", cx))
            .child(KeyBinding::new(binding("ctrl-alt-cmd-shift-z"), cx))
            .child(Story::label("Chord", cx))
            .child(KeyBinding::new(binding("a z"), cx))
            .child(Story::label("Chord with Modifier", cx))
            .child(KeyBinding::new(binding("ctrl-a shift-z"), cx))
            .child(KeyBinding::new(binding("fn-s"), cx))
            .child(Story::label("Single Key with All Modifiers (Linux)", cx))
            .child(
                KeyBinding::new(binding("ctrl-alt-cmd-shift-z"), cx)
                    .platform_style(PlatformStyle::Linux),
            )
            .child(Story::label("Chord (Linux)", cx))
            .child(KeyBinding::new(binding("a z"), cx).platform_style(PlatformStyle::Linux))
            .child(Story::label("Chord with Modifier (Linux)", cx))
            .child(
                KeyBinding::new(binding("ctrl-a shift-z"), cx).platform_style(PlatformStyle::Linux),
            )
            .child(KeyBinding::new(binding("fn-s"), cx).platform_style(PlatformStyle::Linux))
            .child(Story::label("Single Key with All Modifiers (Windows)", cx))
            .child(
                KeyBinding::new(binding("ctrl-alt-cmd-shift-z"), cx)
                    .platform_style(PlatformStyle::Windows),
            )
            .child(Story::label("Chord (Windows)", cx))
            .child(KeyBinding::new(binding("a z"), cx).platform_style(PlatformStyle::Windows))
            .child(Story::label("Chord with Modifier (Windows)", cx))
            .child(
                KeyBinding::new(binding("ctrl-a shift-z"), cx)
                    .platform_style(PlatformStyle::Windows),
            )
            .child(KeyBinding::new(binding("fn-s"), cx).platform_style(PlatformStyle::Windows))
    }
}
