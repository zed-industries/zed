use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Element)]
pub struct ThemeSelector {
    scroll_state: ScrollState,
}

impl ThemeSelector {
    pub fn new() -> Self {
        Self {
            scroll_state: ScrollState::default(),
        }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().child(
            Palette::new(self.scroll_state.clone())
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
