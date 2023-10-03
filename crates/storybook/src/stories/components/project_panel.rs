use ui::prelude::*;
use ui::{Panel, ProjectPanel};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ProjectPanelStory {}

impl ProjectPanelStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, ProjectPanel<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Panel::new(
                ScrollState::default(),
                |_, _| vec![ProjectPanel::new(ScrollState::default()).into_any()],
                Box::new(()),
            ))
    }
}
