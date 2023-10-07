use std::marker::PhantomData;

use crate::prelude::*;
use crate::{theme, token, SystemColor};

#[derive(Clone, Copy)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

#[derive(Element)]
struct TrafficLight<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    color: TrafficLightColor,
    window_has_focus: bool,
}

impl<S: 'static + Send + Sync> TrafficLight<S> {
    fn new(color: TrafficLightColor, window_has_focus: bool) -> Self {
        Self {
            state_type: PhantomData,
            color,
            window_has_focus,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);
        let system_color = SystemColor::new();

        let fill = match (self.window_has_focus, self.color) {
            (true, TrafficLightColor::Red) => system_color.mac_os_traffic_light_red,
            (true, TrafficLightColor::Yellow) => system_color.mac_os_traffic_light_yellow,
            (true, TrafficLightColor::Green) => system_color.mac_os_traffic_light_green,
            (false, _) => theme.lowest.base.active.background,
        };

        div().w_3().h_3().rounded_full().fill(fill)
    }
}

#[derive(Element)]
pub struct TrafficLights<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    window_has_focus: bool,
}

impl<S: 'static + Send + Sync> TrafficLights<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
            window_has_focus: true,
        }
    }

    pub fn window_has_focus(mut self, window_has_focus: bool) -> Self {
        self.window_has_focus = window_has_focus;
        self
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);
        let token = token();

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(TrafficLight::new(
                TrafficLightColor::Red,
                self.window_has_focus,
            ))
            .child(TrafficLight::new(
                TrafficLightColor::Yellow,
                self.window_has_focus,
            ))
            .child(TrafficLight::new(
                TrafficLightColor::Green,
                self.window_has_focus,
            ))
    }
}
