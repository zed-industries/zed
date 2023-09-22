use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::{icon, theme, token, IconAsset};

#[derive(Clone, Copy)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

#[derive(Element)]
struct TrafficLight {
    color: TrafficLightColor,
    window_has_focus: bool,
}

fn traffic_light(color: TrafficLightColor, window_has_focus: bool) -> TrafficLight {
    TrafficLight {
        color,
        window_has_focus,
    }
}

impl TrafficLight {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();

        let fill = match (self.window_has_focus, self.color) {
            (true, TrafficLightColor::Red) => token.mac_os_traffic_light_red,
            (true, TrafficLightColor::Yellow) => token.mac_os_traffic_light_yellow,
            (true, TrafficLightColor::Green) => token.mac_os_traffic_light_green,
            (false, _) => theme.middle.base.default.background,
        };

        // let i = match self.color {
        //     TrafficLightColor::Red => IconAsset::Hash,
        //     TrafficLightColor::Yellow => IconAsset::Hash,
        //     TrafficLightColor::Green => IconAsset::Hash,
        // };

        div().w_3().h_3().rounded_full().fill(fill)
    }
}

#[derive(Element)]
pub struct TrafficLights {
    window_has_focus: bool,
}

pub fn traffic_lights() -> TrafficLights {
    TrafficLights {
        window_has_focus: true,
    }
}

impl TrafficLights {
    pub fn window_has_focus(mut self, window_has_focus: bool) -> Self {
        self.window_has_focus = window_has_focus;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let token = token();

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(traffic_light(TrafficLightColor::Red, self.window_has_focus))
            .child(traffic_light(
                TrafficLightColor::Yellow,
                self.window_has_focus,
            ))
            .child(traffic_light(
                TrafficLightColor::Green,
                self.window_has_focus,
            ))
    }
}
