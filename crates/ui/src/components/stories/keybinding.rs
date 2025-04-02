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

        Story::container()
            .child(Story::title_for::<KeyBinding>())
            .child(Story::label("Single Key"))
            .child(KeyBinding::new(binding("Z"), cx))
            .child(Story::label("Single Key with Modifier"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(KeyBinding::new(binding("ctrl-c"), cx))
                    .child(KeyBinding::new(binding("alt-c"), cx))
                    .child(KeyBinding::new(binding("cmd-c"), cx))
                    .child(KeyBinding::new(binding("shift-c"), cx)),
            )
            .child(Story::label("Single Key with Modifier (Permuted)"))
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
            .child(Story::label("Single Key with All Modifiers"))
            .child(KeyBinding::new(binding("ctrl-alt-cmd-shift-z"), cx))
            .child(Story::label("Chord"))
            .child(KeyBinding::new(binding("a z"), cx))
            .child(Story::label("Chord with Modifier"))
            .child(KeyBinding::new(binding("ctrl-a shift-z"), cx))
            .child(KeyBinding::new(binding("fn-s"), cx))
            .child(Story::label("Single Key with All Modifiers (Linux)"))
            .child(
                KeyBinding::new(binding("ctrl-alt-cmd-shift-z"), cx)
                    .platform_style(PlatformStyle::Linux),
            )
            .child(Story::label("Chord (Linux)"))
            .child(KeyBinding::new(binding("a z"), cx).platform_style(PlatformStyle::Linux))
            .child(Story::label("Chord with Modifier (Linux)"))
            .child(
                KeyBinding::new(binding("ctrl-a shift-z"), cx).platform_style(PlatformStyle::Linux),
            )
            .child(KeyBinding::new(binding("fn-s"), cx).platform_style(PlatformStyle::Linux))
            .child(Story::label("Single Key with All Modifiers (Windows)"))
            .child(
                KeyBinding::new(binding("ctrl-alt-cmd-shift-z"), cx)
                    .platform_style(PlatformStyle::Windows),
            )
            .child(Story::label("Chord (Windows)"))
            .child(KeyBinding::new(binding("a z"), cx).platform_style(PlatformStyle::Windows))
            .child(Story::label("Chord with Modifier (Windows)"))
            .child(
                KeyBinding::new(binding("ctrl-a shift-z"), cx)
                    .platform_style(PlatformStyle::Windows),
            )
            .child(KeyBinding::new(binding("fn-s"), cx).platform_style(PlatformStyle::Windows))
    }
}
