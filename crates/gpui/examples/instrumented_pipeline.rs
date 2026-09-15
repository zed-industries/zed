//! Times the passes a frame is made of, capped at 30 frames a second.
//!
//! [`InstrumentedPipeline`] draws the way GPUI normally does, and records how long
//! each of a frame's root passes took. Wrapping it in
//! [`FramePipelineExt::max_fps`] stacks a second concern on top without touching
//! the first: the throttle defers frames that arrive sooner than the cap, and
//! forwards everything else. The window animates and reports a second's worth of
//! frames as they go by, so the numbers move.
//!
//! Run it with `cargo run -p gpui --example instrumented_pipeline`.

#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use std::{cell::RefCell, rc::Rc};

use gpui::{
    App, Bounds, Context, FramePipelineExt, InstrumentedPipeline, PhaseMetrics, Render, Window,
    WindowBounds, WindowOptions, application, div, prelude::*, px, rgb, size,
};

const REPORT_EVERY: usize = 60;

struct TimedFrame {
    metrics: Rc<RefCell<PhaseMetrics>>,
    frames: usize,
}

impl Render for TimedFrame {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.frames += 1;

        if self.frames % REPORT_EVERY == 0 {
            let metrics = *self.metrics.borrow();
            println!(
                "{} frames: evaluate {:?}, layout {:?}, paint {:?} — {:?} of root passes",
                metrics.frames,
                metrics.evaluate,
                metrics.layout,
                metrics.paint,
                metrics.root_passes(),
            );
        }

        // Keep the example animating: the metrics are only interesting while
        // frames are arriving.
        window.request_animation_frame();

        let sweep = (self.frames as f32 * 0.05).sin() * 0.5 + 0.5;

        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .w(px(20.0 + 400.0 * sweep))
                    .h(px(24.0))
                    .rounded_md()
                    .bg(rgb(0x4a90d9)),
            )
            .child(
                div()
                    .text_color(rgb(0xffffff))
                    .child("Passes are being timed; see stdout."),
            )
    }
}

fn run_example() {
    let metrics = Rc::new(RefCell::new(PhaseMetrics::default()));

    application()
        .with_frame_pipeline({
            let metrics = metrics.clone();
            move |_window_id| Box::new(InstrumentedPipeline::new(metrics.clone()).max_fps(30))
        })
        .run(move |cx: &mut App| {
            if !example_support::load_fonts(cx) {
                return;
            }
            let bounds = Bounds::centered(None, size(px(500.), px(300.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| TimedFrame {
                        metrics: metrics.clone(),
                        frames: 0,
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
    gpui::web_init();
    run_example();
}
