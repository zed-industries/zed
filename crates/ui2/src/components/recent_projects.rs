use std::marker::PhantomData;

use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Element)]
pub struct RecentProjects<S: 'static + Send + Sync + Clone> {
    id: ElementId,
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> RecentProjects<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        div().id(self.id.clone()).child(
            Palette::new("palette")
                .items(vec![
                    PaletteItem::new("zed").sublabel(SharedString::from("~/projects/zed")),
                    PaletteItem::new("saga").sublabel(SharedString::from("~/projects/saga")),
                    PaletteItem::new("journal").sublabel(SharedString::from("~/journal")),
                    PaletteItem::new("dotfiles").sublabel(SharedString::from("~/dotfiles")),
                    PaletteItem::new("zed.dev").sublabel(SharedString::from("~/projects/zed.dev")),
                    PaletteItem::new("laminar").sublabel(SharedString::from("~/projects/laminar")),
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

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, RecentProjects<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(RecentProjects::new("recent-projects"))
        }
    }
}
