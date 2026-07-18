#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, MousePressureEvent, PressureStage, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct MousePressureExample {
    pressure_stage: PressureStage,
    pressure_amount: f32,
}

impl Render for MousePressureExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Pressure stage: {:?}", &self.pressure_stage))
            .child(format!("Pressure amount: {:.2}", &self.pressure_amount))
            .on_mouse_pressure(cx.listener(Self::on_mouse_pressure))
    }
}

impl MousePressureExample {
    fn on_mouse_pressure(
        &mut self,
        pressure_event: &MousePressureEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.pressure_amount = pressure_event.pressure;
        self.pressure_stage = pressure_event.stage;

        cx.notify();
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| MousePressureExample {
                    pressure_stage: PressureStage::Zero,
                    pressure_amount: 0.0,
                })
            },
        )
        .unwrap();

        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
