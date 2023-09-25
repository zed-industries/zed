use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::traffic_lights;

use crate::story::Story;

#[derive(Element, Default)]
pub struct TrafficLightsStory {}

impl TrafficLightsStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, ui::TrafficLights>(cx))
            .child(Story::label(cx, "Default"))
            .child(traffic_lights())
            .child(Story::label(cx, "Unfocused"))
            .child(traffic_lights().window_has_focus(false))
    }
}
