use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, Hsla, IntoElement, ParentElement, ViewContext};

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
            .child(traffic_light(theme.lowest.positive.default.foreground))
            .child(traffic_light(theme.lowest.warning.default.foreground))
            .child(traffic_light(theme.lowest.negative.default.foreground))
    }
}

fn traffic_light<V: 'static, C: Into<Hsla>>(fill: C) -> div::Div<V> {
    div().w_3().h_3().rounded_full().fill(fill.into())
}
