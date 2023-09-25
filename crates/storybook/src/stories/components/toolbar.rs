use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::Toolbar;

use crate::story::Story;

#[derive(Element, Default)]
pub struct ToolbarStory {}

impl ToolbarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container()
            .child(Story::title_for::<_, Toolbar>())
            .child(Story::label("Default"))
            .child(Toolbar::new())
    }
}
