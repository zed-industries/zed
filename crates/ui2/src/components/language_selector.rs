use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Component)]
pub struct LanguageSelector {
    id: ElementId,
}

impl LanguageSelector {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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

    #[derive(Component)]
    pub struct LanguageSelectorStory;

    impl LanguageSelectorStory {
        pub fn new() -> Self {
            Self
        }

        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            Story::container(cx)
                .child(Story::title_for::<_, LanguageSelector>(cx))
                .child(Story::label(cx, "Default"))
                .child(LanguageSelector::new("language-selector"))
        }
    }
}
