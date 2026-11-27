#![cfg_attr(target_family = "wasm", no_main)]

//! Compares SVGs animated with [`Svg::with_timed_transformation`], which
//! moves them in the already painted scene, against the same SVGs animated by
//! re-rendering with `with_animation`. It shows spinners and pulsing rings.
//!
//! The header counts how many times the view has rendered. With the retained
//! scene the count stays put while the SVGs keep animating; when re-rendering
//! it climbs once per frame. Pass `--rerender` to start in that mode, and
//! `--max-fps <n>` to cap how often the retained scene is presented.

#[path = "example_support/fonts.rs"]
mod example_support;

use std::{cell::Cell, time::Duration};

use gpui::{
    Animation, AnimationExt as _, App, Bounds, Context, Easing, TimedTransformation,
    Transformation, Window, WindowBounds, WindowOptions, div, ease_out_quint, percentage,
    prelude::*, px, rgb, size, svg,
};
use gpui_platform::application;

const ARROW_CIRCLE_SVG: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/image/arrow_circle.svg"
);

const RING_SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><circle cx="8" cy="8" r="5" fill="none" stroke="black" stroke-width="1.5"/></svg>"#;

const SPINNER_COUNT: usize = 100;
const PULSE_COUNT: usize = 100;
const SPIN_PERIOD: Duration = Duration::from_secs(2);
const PULSE_PERIOD: Duration = Duration::from_millis(1750);

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    RetainedScene,
    Rerender,
}

struct SvgAnimationExample {
    mode: Mode,
    max_fps: Option<f32>,
    render_count: Cell<u64>,
}

impl SvgAnimationExample {
    fn timed(&self, transformation: TimedTransformation) -> TimedTransformation {
        match self.max_fps {
            Some(max_fps) => transformation.with_max_fps(max_fps),
            None => transformation,
        }
    }

    fn pulse(&self, index: usize) -> impl IntoElement {
        let ring = svg().size_4().data(RING_SVG).text_color(rgb(0x16a34a));
        let ring = match self.mode {
            Mode::RetainedScene => ring
                .with_timed_transformation(
                    self.timed(
                        TimedTransformation::new(PULSE_PERIOD)
                            .from(Transformation::alpha(0.6))
                            .to(Transformation::scale(size(1.5, 1.5)).with_alpha(0.))
                            .with_scale_easing(Easing::EaseOutQuint),
                    ),
                )
                .into_any_element(),
            Mode::Rerender => ring
                .with_animation(
                    ("pulse", index),
                    Animation::new(PULSE_PERIOD).repeat_synced(),
                    |svg, delta| {
                        let scale = 1. + 0.5 * ease_out_quint()(delta);
                        svg.with_transformation(
                            Transformation::scale(size(scale, scale))
                                .with_alpha(0.6 * (1. - delta)),
                        )
                    },
                )
                .into_any_element(),
        };
        div()
            .size_6()
            .flex()
            .items_center()
            .justify_center()
            .child(div().absolute().size_1p5().rounded_full().bg(rgb(0x16a34a)))
            .child(ring)
    }

    fn spinner(&self, index: usize) -> impl IntoElement {
        let spinner = svg()
            .size_6()
            .external_path(ARROW_CIRCLE_SVG)
            .text_color(rgb(0x3b82f6));
        match self.mode {
            Mode::RetainedScene => spinner
                .with_timed_transformation(self.timed(TimedTransformation::rotation(
                    SPIN_PERIOD,
                    Transformation::default(),
                )))
                .into_any_element(),
            Mode::Rerender => spinner
                .with_animation(
                    ("spinner", index),
                    Animation::new(SPIN_PERIOD).repeat_synced(),
                    |svg, delta| svg.with_transformation(Transformation::rotate(percentage(delta))),
                )
                .into_any_element(),
        }
    }
}

impl Render for SvgAnimationExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_count.set(self.render_count.get() + 1);
        let mode = self.mode;
        let mode_label = match mode {
            Mode::RetainedScene => "retained scene (with_timed_transformation)",
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
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap_2()
                    .children((0..PULSE_COUNT).map(|index| self.pulse(index))),
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
            let mut arguments = std::env::args();
            let max_fps = arguments
                .by_ref()
                .find(|argument| argument == "--max-fps")
                .and_then(|_| arguments.next())
                .and_then(|max_fps| max_fps.parse().ok());
            cx.new(|_| SvgAnimationExample {
                mode,
                max_fps,
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
