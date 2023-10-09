use std::marker::PhantomData;

use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Element)]
pub struct LanguageSelector<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> LanguageSelector<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
            scroll_state: ScrollState::default(),
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        div().child(
            Palette::new(self.scroll_state.clone())
                .items(vec![
                    PaletteItem::new("C"),
                    PaletteItem::new("C++"),
                    PaletteItem::new("CSS"),
                    PaletteItem::new("Elixir"),
                    PaletteItem::new("Elm"),
                    PaletteItem::new("ERB"),
                    PaletteItem::new("Rust (current)"),
                    PaletteItem::new("Scheme"),
                    PaletteItem::new("TOML"),
                    PaletteItem::new("TypeScript"),
                ])
                .placeholder("Select a language...")
                .empty_string("No matches")
                .default_order(OrderMethod::Ascending),
        )
    }
}
