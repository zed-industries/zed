use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Component)]
pub struct ThemeSelector {
    id: ElementId,
}

impl ThemeSelector {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div().child(
            Palette::new(self.id.clone())
                .items(vec![
                    PaletteItem::new("One Dark"),
                    PaletteItem::new("Rosé Pine"),
                    PaletteItem::new("Rosé Pine Moon"),
                    PaletteItem::new("Sandcastle"),
                    PaletteItem::new("Solarized Dark"),
                    PaletteItem::new("Summercamp"),
                    PaletteItem::new("Atelier Cave Light"),
                    PaletteItem::new("Atelier Dune Light"),
                    PaletteItem::new("Atelier Estuary Light"),
                    PaletteItem::new("Atelier Forest Light"),
                    PaletteItem::new("Atelier Heath Light"),
                ])
                .placeholder("Select Theme...")
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
    pub struct ThemeSelectorStory;

    impl ThemeSelectorStory {
        pub fn new() -> Self {
            Self
        }

        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            Story::container(cx)
                .child(Story::title_for::<_, ThemeSelector>(cx))
                .child(Story::label(cx, "Default"))
                .child(ThemeSelector::new("theme-selector"))
        }
    }
}
