use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(Component)]
pub struct RecentProjects {
    id: ElementId,
}

impl RecentProjects {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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
    use super::*;
    use crate::Story;
    use gpui::{Node, Render};

    pub struct RecentProjectsStory;

    impl Render for RecentProjectsStory {
        type Element = Node<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, RecentProjects>(cx))
                .child(Story::label(cx, "Default"))
                .child(RecentProjects::new("recent-projects"))
        }
    }
}
