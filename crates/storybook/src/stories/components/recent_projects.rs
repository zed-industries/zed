use ui::prelude::*;
use ui::RecentProjects;

use crate::story::Story;

#[derive(Element, Default)]
pub struct RecentProjectsStory {}

impl RecentProjectsStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, RecentProjects>(cx))
            .child(Story::label(cx, "Default"))
            .child(RecentProjects::new())
    }
}
