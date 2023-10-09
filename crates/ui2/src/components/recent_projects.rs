use std::marker::PhantomData;

use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Element)]
pub struct RecentProjects<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> RecentProjects<S> {
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
