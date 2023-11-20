use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(RenderOnce)]
pub struct RecentProjects {
    id: ElementId,
}

impl<V: 'static> Component<V> for RecentProjects {
    type Rendered = Stateful<V, Div<V>>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
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

impl RecentProjects {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

use gpui::{Div, RenderOnce, Stateful};
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Div, Render};

    pub struct RecentProjectsStory;

    impl Render<Self> for RecentProjectsStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, RecentProjects>(cx))
                .child(Story::label(cx, "Default"))
                .child(RecentProjects::new("recent-projects"))
        }
    }
}
