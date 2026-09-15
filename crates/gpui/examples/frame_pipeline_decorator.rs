//! A frame-rate cap written outside the framework.
//!
//! [`FramePipeline`] is object-safe, so independent concerns compose as
//! decorators: a wrapper pipeline intercepts one pass and forwards the rest.
//! The `ThrottledPipeline` and `FramePipelineExt` types in this file are defined
//! here, not imported, to show that a pipeline like this needs no privileged
//! access — it is built entirely on the public `gpui` facade. The crate ships
//! batteries-included pipelines (`gpui::InstrumentedPipeline` and
//! `gpui::ThrottledPipeline`); this is the same pattern an application can write
//! itself.
//!
//! Run it with `cargo run -p gpui --example frame_pipeline_decorator`.

#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use std::time::{Duration, Instant};

use gpui::{
    App, Bounds, Context, FocusId, FramePipeline, PreparedRoots, Render, StandardImmediatePipeline,
    Window, WindowBounds, WindowMetrics, WindowOptions, application, div, prelude::*, px, rgb,
    size,
};

/// Caps `inner` at `max_fps` frames a second by answering `false` from
/// [`should_render`](FramePipeline::should_render) for frames that arrive too
/// soon.
///
/// It changes only that one decision and forwards every other pass, so the
/// pipeline it wraps draws exactly as it would have.
struct ThrottledPipeline<P> {
    inner: P,
    min_interval: Duration,
    last_render: Option<Instant>,
}

impl<P> ThrottledPipeline<P> {
    fn new(inner: P, max_fps: u32) -> Self {
        Self {
            inner,
            min_interval: Duration::from_secs_f64(1.0 / f64::from(max_fps.max(1))),
            last_render: None,
        }
    }
}

impl<P: FramePipeline> FramePipeline for ThrottledPipeline<P> {
    fn should_render(&mut self, is_dirty: bool, metrics: &WindowMetrics) -> bool {
        // Ask the pipeline underneath first: it may have its own reason to defer,
        // and a frame it defers should not spend this one's slot.
        if !self.inner.should_render(is_dirty, metrics) {
            return false;
        }

        let now = Instant::now();
        if let Some(last) = self.last_render
            && now.duration_since(last) < self.min_interval
        {
            return false;
        }

        self.last_render = Some(now);
        true
    }

    fn begin_frame(&mut self, window: &mut Window<'_>, cx: &mut App) {
        self.inner.begin_frame(window, cx);
    }

    fn evaluate_roots(&mut self, window: &mut Window<'_>, cx: &mut App) -> PreparedRoots {
        self.inner.evaluate_roots(window, cx)
    }

    fn layout_roots(&mut self, window: &mut Window<'_>, roots: &mut PreparedRoots, cx: &mut App) {
        self.inner.layout_roots(window, roots, cx);
    }

    fn paint_roots(&mut self, window: &mut Window<'_>, roots: PreparedRoots, cx: &mut App) {
        self.inner.paint_roots(window, roots, cx);
    }

    fn finish_frame(&mut self, window: &mut Window<'_>, cx: &mut App) {
        self.inner.finish_frame(window, cx);
    }

    fn complete_frame(&mut self, window: &mut Window<'_>, cx: &mut App) -> Option<FocusId> {
        self.inner.complete_frame(window, cx)
    }

    fn end_frame(&mut self, window: &mut Window<'_>, cx: &mut App, focus: Option<FocusId>) {
        self.inner.end_frame(window, cx, focus);
    }
}

/// Builder methods that stack a decorator onto any pipeline.
///
/// Add more methods here to stack further concerns, each wrapping the previous
/// one: `.max_fps(30).instrumented(metrics)` reads the same no matter the order.
trait FramePipelineExt: FramePipeline + Sized {
    fn max_fps(self, max_fps: u32) -> ThrottledPipeline<Self> {
        ThrottledPipeline::new(self, max_fps)
    }
}

impl<P: FramePipeline> FramePipelineExt for P {}

/// Keeps the window animating so the frame-rate cap has frames to turn away.
struct SweepView {
    frames: usize,
}

impl Render for SweepView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.frames += 1;
        window.request_animation_frame();

        let sweep = (self.frames as f32 * 0.05).sin() * 0.5 + 0.5;

        div()
            .flex()
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
                    .child("Frames are capped at 30 a second."),
            )
    }
}

fn run_example() {
    application()
        .with_frame_pipeline(|_window_id| Box::new(StandardImmediatePipeline.max_fps(30)))
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
                |_, cx| cx.new(|_| SweepView { frames: 0 }),
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
