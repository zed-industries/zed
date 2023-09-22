use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::theme;

#[derive(Element)]
pub struct TrafficLights {}

pub fn traffic_lights() -> TrafficLights {
    TrafficLights {}
}

impl TrafficLights {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w_3()
                    .h_3()
                    .rounded_full()
                    .fill(theme.lowest.positive.default.foreground),
            )
            .child(
                div()
                    .w_3()
                    .h_3()
                    .rounded_full()
                    .fill(theme.lowest.warning.default.foreground),
            )
            .child(
                div()
                    .w_3()
                    .h_3()
                    .rounded_full()
                    .fill(theme.lowest.negative.default.foreground),
            )
    }
}
