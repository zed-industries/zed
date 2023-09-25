use std::collections::HashSet;

use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::theme;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
            Self::Alt => '⎇',
            Self::Command => '⌘',
            Self::Shift => '⇧',
        }
    }
}

#[derive(Element, Clone)]
pub struct Keybinding {
    keybind: Vec<(String, HashSet<ModifierKey>)>,
}

impl Keybinding {
    pub fn new(key: String, modifiers: HashSet<ModifierKey>) -> Self {
        Self {
            keybind: vec![(key, modifiers)],
        }
    }

    pub fn new_chord(chord: Vec<(String, HashSet<ModifierKey>)>) -> Self {
        Self { keybind: chord }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div()
            .flex()
            .gap_2()
            .children(self.keybind.iter().map(|(key, modifiers)| {
                div()
                    .flex()
                    .gap_1()
                    .children(modifiers.iter().map(|modifier| Key::new(modifier.glyph())))
                    .child(Key::new(key.clone()))
            }))
    }
}

#[derive(Element)]
pub struct Key {
    key: String,
}

impl Key {
    pub fn new<K>(key: K) -> Self
    where
        K: Into<String>,
    {
        Self { key: key.into() }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .px_2()
            .py_0()
            .rounded_md()
            .text_sm()
            .text_color(theme.lowest.on.default.foreground)
            .fill(theme.lowest.on.default.background)
            .child(self.key.clone())
    }
}
