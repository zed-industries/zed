use ui::prelude::*;
use ui::{Label, Panel};

use crate::story::Story;

#[derive(Element, Default)]
pub struct PanelStory {}

impl PanelStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Panel<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Panel::new(
                ScrollState::default(),
                |_, _| {
                    vec![div()
                        .overflow_y_scroll(ScrollState::default())
                        .children((0..100).map(|ix| Label::new(format!("Item {}", ix + 1))))
                        .into_any()]
                },
                Box::new(()),
            ))
    }
}
