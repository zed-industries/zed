use std::marker::PhantomData;

use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Element)]
pub struct ThemeSelector<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> ThemeSelector<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
            scroll_state: ScrollState::default(),
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct ThemeSelectorStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> ThemeSelectorStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, ThemeSelector<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(ThemeSelector::new())
        }
    }
}
