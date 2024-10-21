use gpui::NoAction;
use gpui::Render;
use itertools::Itertools;
use story::Story;

use crate::{prelude::*, KeyBinding};

pub struct KeybindingStory;

pub fn binding(key: &str) -> gpui::KeyBinding {
    gpui::KeyBinding::new(key, NoAction {}, None)
}

impl Render for KeybindingStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let all_modifier_permutations = ["ctrl", "alt", "cmd", "shift"].into_iter().permutations(2);

        Story::container()
            .child(Story::title_for::<KeyBinding>())
            .child(Story::label("Single Key"))
            .child(KeyBinding::new(binding("Z")))
            .child(Story::label("Single Key with Modifier"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(KeyBinding::new(binding("ctrl-c")))
                    .child(KeyBinding::new(binding("alt-c")))
                    .child(KeyBinding::new(binding("cmd-c")))
                    .child(KeyBinding::new(binding("shift-c"))),
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
                                    KeyBinding::new(binding(&(permutation.join("-") + "-x")))
                                }))
                        }),
                ),
            )
            .child(Story::label("Single Key with All Modifiers"))
            .child(KeyBinding::new(binding("ctrl-alt-cmd-shift-z")))
            .child(Story::label("Chord"))
            .child(KeyBinding::new(binding("a z")))
            .child(Story::label("Chord with Modifier"))
            .child(KeyBinding::new(binding("ctrl-a shift-z")))
            .child(KeyBinding::new(binding("fn-s")))
            .child(Story::label("Single Key with All Modifiers (Linux)"))
            .child(
                KeyBinding::new(binding("ctrl-alt-cmd-shift-z"))
                    .platform_style(PlatformStyle::Linux),
            )
            .child(Story::label("Chord (Linux)"))
            .child(KeyBinding::new(binding("a z")).platform_style(PlatformStyle::Linux))
            .child(Story::label("Chord with Modifier (Linux)"))
            .child(KeyBinding::new(binding("ctrl-a shift-z")).platform_style(PlatformStyle::Linux))
            .child(KeyBinding::new(binding("fn-s")).platform_style(PlatformStyle::Linux))
            .child(Story::label("Single Key with All Modifiers (Windows)"))
            .child(
                KeyBinding::new(binding("ctrl-alt-cmd-shift-z"))
                    .platform_style(PlatformStyle::Windows),
            )
            .child(Story::label("Chord (Windows)"))
            .child(KeyBinding::new(binding("a z")).platform_style(PlatformStyle::Windows))
            .child(Story::label("Chord with Modifier (Windows)"))
            .child(
                KeyBinding::new(binding("ctrl-a shift-z")).platform_style(PlatformStyle::Windows),
            )
            .child(KeyBinding::new(binding("fn-s")).platform_style(PlatformStyle::Windows))
    }
}
