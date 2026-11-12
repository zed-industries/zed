#![cfg_attr(target_family = "wasm", no_main)]

//! Compares spinners animated with [`Svg::with_rotation_animation`], which
//! rotates them in the already painted scene, against the same spinners
//! animated by re-rendering with `with_animation`.
//!
//! The header counts how many times the view has rendered. With the retained
//! scene the count stays put while the spinners keep turning; when re-rendering
//! it climbs once per frame. Pass `--rerender` to start in that mode.

#[path = "example_support/fonts.rs"]
mod example_support;

use std::{cell::Cell, time::Duration};

use gpui::{
    Animation, AnimationExt as _, App, Bounds, Context, Transformation, Window, WindowBounds,
    WindowOptions, div, percentage, prelude::*, px, rgb, size, svg,
};
use gpui_platform::application;

const ARROW_CIRCLE_SVG: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/image/arrow_circle.svg"
);

const SPINNER_COUNT: usize = 200;
const PERIOD: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    RetainedScene,
    Rerender,
}

struct RotationAnimationExample {
    mode: Mode,
    render_count: Cell<u64>,
}

impl RotationAnimationExample {
    fn spinner(&self, index: usize) -> impl IntoElement {
        let spinner = svg()
            .size_6()
            .external_path(ARROW_CIRCLE_SVG)
            .text_color(rgb(0x3b82f6));
        match self.mode {
            Mode::RetainedScene => spinner.with_rotation_animation(PERIOD).into_any_element(),
            Mode::Rerender => spinner
                .with_animation(
                    ("spinner", index),
                    Animation::new(PERIOD).repeat_synced(),
                    |svg, delta| svg.with_transformation(Transformation::rotate(percentage(delta))),
                )
                .into_any_element(),
        }
    }
}

impl Render for RotationAnimationExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_count.set(self.render_count.get() + 1);
        let mode = self.mode;
        let mode_label = match mode {
            Mode::RetainedScene => "retained scene (with_rotation_animation)",
            Mode::Rerender => "re-render (with_animation)",
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .p_4()
            .gap_3()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .id("toggle-mode")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0xe5e7eb))
                            .cursor_pointer()
                            .child(format!("Mode: {mode_label}"))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.mode = match this.mode {
                                    Mode::RetainedScene => Mode::Rerender,
                                    Mode::Rerender => Mode::RetainedScene,
                                };
                                cx.notify();
                            })),
                    )
                    .child(format!("Renders: {}", self.render_count.get())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .children((0..SPINNER_COUNT).map(|index| self.spinner(index))),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        if !example_support::load_fonts(cx) {
            return;
        }
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(720.), px(480.)),
                cx,
            ))),
            ..Default::default()
        };
        cx.open_window(options, |_, cx| {
            cx.activate(false);
            let mode = if std::env::args().any(|arg| arg == "--rerender") {
                Mode::Rerender
            } else {
                Mode::RetainedScene
            };
            cx.new(|_| RotationAnimationExample {
                mode,
                render_count: Cell::new(0),
            })
        })
        .expect("failed to open window");
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
