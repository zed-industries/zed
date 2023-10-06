use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Element)]
pub struct RecentProjects {
    scroll_state: ScrollState,
}

impl RecentProjects {
    pub fn new() -> Self {
        Self {
            scroll_state: ScrollState::default(),
        }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        div().child(
            Palette::new(self.scroll_state.clone())
                .items(vec![
                    PaletteItem::new("zed").sublabel("~/projects/zed"),
                    PaletteItem::new("saga").sublabel("~/projects/saga"),
                    PaletteItem::new("journal").sublabel("~/journal"),
                    PaletteItem::new("dotfiles").sublabel("~/dotfiles"),
                    PaletteItem::new("zed.dev").sublabel("~/projects/zed.dev"),
                    PaletteItem::new("laminar").sublabel("~/projects/laminar"),
                ])
                .placeholder("Recent Projects...")
                .empty_string("No matches")
                .default_order(OrderMethod::Ascending),
        )
    }
}
