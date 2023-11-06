use std::collections::HashSet;

use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::*;

#[derive(Component)]
pub struct Keybinding {
    /// A keybinding consists of a key and a set of modifier keys.
    /// More then one keybinding produces a chord.
    ///
    /// This should always contain at least one element.
    keybinding: Vec<(String, ModifierKeys)>,
}

impl Keybinding {
    pub fn new(key: String, modifiers: ModifierKeys) -> Self {
        Self {
            keybinding: vec![(key, modifiers)],
        }
    }

    pub fn new_chord(
        first_note: (String, ModifierKeys),
        second_note: (String, ModifierKeys),
    ) -> Self {
        Self {
            keybinding: vec![first_note, second_note],
        }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div()
            .flex()
            .gap_2()
            .children(self.keybinding.iter().map(|(key, modifiers)| {
                div()
                    .flex()
                    .gap_1()
                    .children(ModifierKey::iter().filter_map(|modifier| {
                        if modifiers.0.contains(&modifier) {
                            Some(Key::new(modifier.glyph().to_string()))
                        } else {
                            None
                        }
                    }))
                    .child(Key::new(key.clone()))
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
        div()
            .px_2()
            .py_0()
            .rounded_md()
            .text_sm()
            .text_color(cx.theme().colors().text)
            .bg(cx.theme().colors().element_background)
            .child(self.key.clone())
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

impl ModifierKey {
    /// Returns the glyph for the [`ModifierKey`].
    pub fn glyph(&self) -> char {
        match self {
            Self::Control => '^',
            Self::Alt => '⌥',
            Self::Command => '⌘',
            Self::Shift => '⇧',
        }
    }
}

#[derive(Clone)]
pub struct ModifierKeys(HashSet<ModifierKey>);

impl ModifierKeys {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn all() -> Self {
        Self(HashSet::from_iter(ModifierKey::iter()))
    }

    pub fn add(mut self, modifier: ModifierKey) -> Self {
        self.0.insert(modifier);
        self
    }

    pub fn control(mut self, control: bool) -> Self {
        if control {
            self.0.insert(ModifierKey::Control);
        } else {
            self.0.remove(&ModifierKey::Control);
        }

        self
    }

    pub fn alt(mut self, alt: bool) -> Self {
        if alt {
            self.0.insert(ModifierKey::Alt);
        } else {
            self.0.remove(&ModifierKey::Alt);
        }

        self
    }

    pub fn command(mut self, command: bool) -> Self {
        if command {
            self.0.insert(ModifierKey::Command);
        } else {
            self.0.remove(&ModifierKey::Command);
        }

        self
    }

    pub fn shift(mut self, shift: bool) -> Self {
        if shift {
            self.0.insert(ModifierKey::Shift);
        } else {
            self.0.remove(&ModifierKey::Shift);
        }

        self
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui2::{Div, Render};
    use itertools::Itertools;

    pub struct KeybindingStory;

    impl Render for KeybindingStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
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
                .child(Keybinding::new_chord(
                    ("A".to_string(), ModifierKeys::new()),
                    ("Z".to_string(), ModifierKeys::new()),
                ))
                .child(Story::label(cx, "Chord with Modifier"))
                .child(Keybinding::new_chord(
                    ("A".to_string(), ModifierKeys::new().control(true)),
                    ("Z".to_string(), ModifierKeys::new().shift(true)),
                ))
        }
    }
}
