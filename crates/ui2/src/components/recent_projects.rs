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

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct RecentProjectsStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> RecentProjectsStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, RecentProjects<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(RecentProjects::new())
        }
    }
}
