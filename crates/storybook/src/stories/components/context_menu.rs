use ui::{prelude::*, Label};
use ui::{ContextMenu, ContextMenuItem};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ContextMenuStory {}

impl ContextMenuStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            //.fill(theme.middle.base.default.background)
            .child(Story::title_for::<_, ContextMenu>(cx))
            .child(Story::label(cx, "Default"))
            .child(ContextMenu::new([
                ContextMenuItem::header("Section header"),
                ContextMenuItem::Separator,
                ContextMenuItem::entry(Label::new("Some entry")),
            ]))
    }
}
