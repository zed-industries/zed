use ui::prelude::*;
use ui::Terminal;

use crate::story::Story;

#[derive(Element, Default)]
pub struct TerminalStory {}

impl TerminalStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Terminal>(cx))
            .child(Story::label(cx, "Default"))
            .child(Terminal::new())
    }
}
