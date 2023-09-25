use gpui2::{elements::div, style::StyleHelpers, Element, IntoElement, ParentElement, ViewContext};

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

#[derive(Element)]
pub struct Keybinding {
    key: String,
    modifiers: Vec<ModifierKey>,
}

impl Keybinding {
    pub fn new(key: String) -> Self {
        Self {
            key,
            modifiers: Vec::new(),
        }
    }

    pub fn modifiers(mut self, modifiers: Vec<ModifierKey>) -> Self {
        self.modifiers = modifiers;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div()
            .flex()
            .gap_1()
            .children(
                self.modifiers
                    .iter()
                    .map(|modifier| Key::new(modifier.glyph())),
            )
            .child(Key::new(self.key.clone()))
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
