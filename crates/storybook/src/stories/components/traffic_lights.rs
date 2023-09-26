use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::TrafficLights;

use crate::story::Story;

#[derive(Element, Default)]
pub struct TrafficLightsStory {}

impl TrafficLightsStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, TrafficLights>(cx))
            .child(Story::label(cx, "Default"))
            .child(TrafficLights::new())
            .child(Story::label(cx, "Unfocused"))
            .child(TrafficLights::new().window_has_focus(false))
    }
}
