use std::collections::HashSet;

use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use strum::{EnumIter, IntoEnumIterator};

use crate::theme;

#[derive(Element, Clone)]
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

    pub fn new_chord(chord: Vec<(String, ModifierKeys)>) -> Self {
        Self { keybinding: chord }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div()
            .flex()
            .gap_2()
            .children(self.keybinding.iter().map(|(key, modifiers)| {
                div()
                    .flex()
                    .gap_1()
                    .children(ModifierKey::iter().filter_map(|modifier| {
                        if modifiers.0.contains(&modifier) {
                            Some(Key::new(modifier.glyph()))
                        } else {
                            None
                        }
                    }))
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
            Self::Alt => '⎇',
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
