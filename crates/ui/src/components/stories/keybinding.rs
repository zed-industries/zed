use gpui::NoAction;
use gpui::Render;
use itertools::Itertools;
use settings::KeybindSource;
use story::Story;

use crate::{KeyBinding, prelude::*};

pub struct KeybindingStory;

pub fn binding(key: &str) -> gpui::KeyBinding {
    gpui::KeyBinding::new(key, NoAction {}, None)
}

impl Render for KeybindingStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_modifier_permutations = ["ctrl", "alt", "cmd", "shift"].into_iter().permutations(2);

        const SOURCE: KeybindSource = KeybindSource::Base;

        Story::container(cx)
            .child(Story::title_for::<KeyBinding>(cx))
            .child(Story::label("Single Key", cx))
            .child(KeyBinding::from_keystrokes(
                binding("Z").keystrokes().into(),
                SOURCE,
            ))
            .child(Story::label("Single Key with Modifier", cx))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(KeyBinding::from_keystrokes(
                        binding("ctrl-c").keystrokes().into(),
                        SOURCE,
                    ))
                    .child(KeyBinding::from_keystrokes(
                        binding("alt-c").keystrokes().into(),
                        SOURCE,
                    ))
                    .child(KeyBinding::from_keystrokes(
                        binding("cmd-c").keystrokes().into(),
                        SOURCE,
                    ))
                    .child(KeyBinding::from_keystrokes(
                        binding("shift-c").keystrokes().into(),
                        SOURCE,
                    )),
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
                                    KeyBinding::from_keystrokes(
                                        binding(&(permutation.join("-") + "-x"))
                                            .keystrokes()
                                            .into(),
                                        SOURCE,
                                    )
                                }))
                        }),
                ),
            )
            .child(Story::label("Single Key with All Modifiers", cx))
            .child(KeyBinding::from_keystrokes(
                binding("ctrl-alt-cmd-shift-z").keystrokes().into(),
                SOURCE,
            ))
            .child(Story::label("Chord", cx))
            .child(KeyBinding::from_keystrokes(
                binding("a z").keystrokes().into(),
                SOURCE,
            ))
            .child(Story::label("Chord with Modifier", cx))
            .child(KeyBinding::from_keystrokes(
                binding("ctrl-a shift-z").keystrokes().into(),
                SOURCE,
            ))
            .child(KeyBinding::from_keystrokes(
                binding("fn-s").keystrokes().into(),
                SOURCE,
            ))
            .child(Story::label("Single Key with All Modifiers (Linux)", cx))
            .child(
                KeyBinding::from_keystrokes(
                    binding("ctrl-alt-cmd-shift-z").keystrokes().into(),
                    SOURCE,
                )
                .platform_style(PlatformStyle::Linux),
            )
            .child(Story::label("Chord (Linux)", cx))
            .child(
                KeyBinding::from_keystrokes(binding("a z").keystrokes().into(), SOURCE)
                    .platform_style(PlatformStyle::Linux),
            )
            .child(Story::label("Chord with Modifier (Linux)", cx))
            .child(
                KeyBinding::from_keystrokes(binding("ctrl-a shift-z").keystrokes().into(), SOURCE)
                    .platform_style(PlatformStyle::Linux),
            )
            .child(
                KeyBinding::from_keystrokes(binding("fn-s").keystrokes().into(), SOURCE)
                    .platform_style(PlatformStyle::Linux),
            )
            .child(Story::label("Single Key with All Modifiers (Windows)", cx))
            .child(
                KeyBinding::from_keystrokes(
                    binding("ctrl-alt-cmd-shift-z").keystrokes().into(),
                    SOURCE,
                )
                .platform_style(PlatformStyle::Windows),
            )
            .child(Story::label("Chord (Windows)", cx))
            .child(
                KeyBinding::from_keystrokes(binding("a z").keystrokes().into(), SOURCE)
                    .platform_style(PlatformStyle::Windows),
            )
            .child(Story::label("Chord with Modifier (Windows)", cx))
            .child(
                KeyBinding::from_keystrokes(binding("ctrl-a shift-z").keystrokes().into(), SOURCE)
                    .platform_style(PlatformStyle::Windows),
            )
            .child(
                KeyBinding::from_keystrokes(binding("fn-s").keystrokes().into(), SOURCE)
                    .platform_style(PlatformStyle::Windows),
            )
    }
}
