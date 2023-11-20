// use crate::prelude::*;

// #[derive(Clone, Copy)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }

// #[derive(RenderOnce)]
// struct TrafficLight {
//     color: TrafficLightColor,
//     window_has_focus: bool,
// }

// impl Component for TrafficLight {
//     type Rendered = Div;

//     fn render(self, cx: &mut WindowContext) -> Self::Rendered {
//         let system_colors = &cx.theme().styles.system;

//         let fill = match (self.window_has_focus, self.color) {
//             (true, TrafficLightColor::Red) => system_colors.mac_os_traffic_light_red,
//             (true, TrafficLightColor::Yellow) => system_colors.mac_os_traffic_light_yellow,
//             (true, TrafficLightColor::Green) => system_colors.mac_os_traffic_light_green,
//             (false, _) => cx.theme().colors().element_background,
//         };

//         div().w_3().h_3().rounded_full().bg(fill)
//     }
// }

// impl TrafficLight {
//     fn new(color: TrafficLightColor, window_has_focus: bool) -> Self {
//         Self {
//             color,
//             window_has_focus,
//         }
//     }
// }

// #[derive(RenderOnce)]
// pub struct TrafficLights {
//     window_has_focus: bool,
// }

// impl Component for TrafficLights {
//     type Rendered = Div;

//     fn render(self, cx: &mut WindowContext) -> Self::Rendered {
//         div()
//             .flex()
//             .items_center()
//             .gap_2()
//             .child(TrafficLight::new(
//                 TrafficLightColor::Red,
//                 self.window_has_focus,
//             ))
//             .child(TrafficLight::new(
//                 TrafficLightColor::Yellow,
//                 self.window_has_focus,
//             ))
//             .child(TrafficLight::new(
//                 TrafficLightColor::Green,
//                 self.window_has_focus,
//             ))
//     }
// }

// impl TrafficLights {
//     pub fn new() -> Self {
//         Self {
//             window_has_focus: true,
//         }
//     }

//     pub fn window_has_focus(mut self, window_has_focus: bool) -> Self {
//         self.window_has_focus = window_has_focus;
//         self
//     }
// }

// use gpui::{Div, RenderOnce};
// #[cfg(feature = "stories")]
// pub use stories::*;

// #[cfg(feature = "stories")]
// mod stories {
//     use gpui::{Div, Render};

//     use crate::Story;

//     use super::*;

//     pub struct TrafficLightsStory;

//     impl Render for TrafficLightsStory {
//         type Element = Div;

//         fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
//             Story::container(cx)
//                 .child(Story::title_for::<TrafficLights>(cx))
//                 .child(Story::label(cx, "Default"))
//                 .child(TrafficLights::new())
//                 .child(Story::label(cx, "Unfocused"))
//                 .child(TrafficLights::new().window_has_focus(false))
//         }
//     }
// }
