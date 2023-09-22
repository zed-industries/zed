use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::{theme, traffic_lights};

use crate::story::Story;

#[derive(Element, Default)]
pub struct TrafficLightsStory {}

impl TrafficLightsStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        Story::container()
            .child(Story::title_for::<_, ui::TrafficLights>())
            .child(Story::label("Default"))
            .child(traffic_lights())
    }
}
