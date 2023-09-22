use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{rgb, Element, Hsla, IntoElement, ParentElement, ViewContext};
use ui::{theme, traffic_lights};

#[derive(Element, Default)]
pub struct TrafficLightsStory {}

impl TrafficLightsStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .pt_2()
            .px_4()
            .font("Zed Mono Extended")
            .fill(rgb::<Hsla>(0x282c34))
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb::<Hsla>(0xffffff))
                    .child(std::any::type_name::<ui::TrafficLights>()),
            )
            .child(traffic_lights())
    }
}
