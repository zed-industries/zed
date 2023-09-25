use gpui2::{elements::div, style::StyleHelpers, Element, IntoElement, ParentElement, ViewContext};
use itertools::Itertools;
use strum::IntoEnumIterator;
use ui::{Keybinding, ModifierKey, ModifierKeys};

use crate::story::Story;

#[derive(Element, Default)]
pub struct KeybindingStory {}

impl KeybindingStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let all_modifier_permutations = ModifierKey::iter().permutations(2);

        Story::container(cx)
            .child(Story::title_for::<_, Keybinding>(cx))
            .child(Story::label(cx, "Single Key"))
            .child(Keybinding::new("Z".to_string(), ModifierKeys::new()))
            .child(Story::label(cx, "Single Key with Modifier"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .children(ModifierKey::iter().map(|modifier| {
                        Keybinding::new("C".to_string(), ModifierKeys::new().add(modifier))
                    })),
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
                                    let mut modifiers = ModifierKeys::new();

                                    for modifier in permutation {
                                        modifiers = modifiers.add(modifier);
                                    }

                                    Keybinding::new("X".to_string(), modifiers)
                                }))
                        }),
                ),
            )
            .child(Story::label(cx, "Single Key with All Modifiers"))
            .child(Keybinding::new("Z".to_string(), ModifierKeys::all()))
            .child(Story::label(cx, "Chord"))
            .child(Keybinding::new_chord(vec![
                ("A".to_string(), ModifierKeys::new()),
                ("Z".to_string(), ModifierKeys::new()),
            ]))
            .child(Story::label(cx, "Chord with Modifier"))
            .child(Keybinding::new_chord(vec![
                ("A".to_string(), ModifierKeys::new().control(true)),
                ("Z".to_string(), ModifierKeys::new().shift(true)),
            ]))
    }
}
