use std::marker::PhantomData;

use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Element)]
pub struct LanguageSelector<S: 'static + Send + Sync + Clone> {
    id: ElementId,
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> LanguageSelector<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
        div().id(self.id.clone()).child(
            Palette::new("palette")
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct LanguageSelectorStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> LanguageSelectorStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
            Story::container(cx)
                .child(Story::title_for::<_, LanguageSelector<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(LanguageSelector::new("language-selector"))
        }
    }
}
