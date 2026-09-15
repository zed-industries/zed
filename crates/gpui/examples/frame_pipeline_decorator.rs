//! A frame-rate cap and a frame-timing collector, written outside the framework.
//!
//! [`FramePipeline`] is object-safe, so independent concerns compose as
//! decorators: a wrapper pipeline intercepts one pass and forwards the rest.
//! The `ThrottledPipeline`, `MetricsPipeline`, and `FramePipelineExt` types in
//! this file are defined here, not imported, to show that a pipeline like this
//! needs no privileged access — it is built entirely on the public `gpui` facade.
//! The crate ships batteries-included pipelines (`gpui::InstrumentedPipeline` and
//! `gpui::ThrottledPipeline`); this is the same pattern an application can write
//! itself.
//!
//! Run it with `cargo run -p gpui --example frame_pipeline_decorator`.

#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
    time::{Duration, Instant},
};

use gpui::{
    App, Bounds, Context, FocusId, FontWeight, FramePipeline, Hsla, PreparedRoots, Render,
    StandardImmediatePipeline, Window, WindowBounds, WindowMetrics, WindowOptions, application,
    div, prelude::*, px, rgb, size,
};

/// How many frame intervals the graph keeps before the oldest scroll off.
const MAX_SAMPLES: usize = 120;
/// The interval a 30 fps cap allows, in milliseconds.
const TARGET_MS: f32 = 1000.0 / 30.0;
/// The graph's vertical scale, in milliseconds.
const GRAPH_MAX_MS: f32 = 50.0;
/// The graph's height, in pixels.
const GRAPH_HEIGHT: f32 = 120.0;

// A small dark palette.
const BACKGROUND: u32 = 0x0b0f14;
const SURFACE: u32 = 0x151b23;
const BORDER: u32 = 0x30363d;
const TEXT: u32 = 0xe6edf3;
const MUTED: u32 = 0x8b949e;
const ACCENT: u32 = 0x38bdf8;
const GOOD: u32 = 0x34d399;
const WARN: u32 = 0xf59e0b;
const VIOLET: u32 = 0xa78bfa;

/// Frame intervals, newest last, plus the total number of frames drawn.
#[derive(Default)]
struct FrameStats {
    intervals_ms: VecDeque<f32>,
    frames: u64,
    last_started: Option<Instant>,
}

impl FrameStats {
    /// Records the interval since the previous drawn frame.
    fn record(&mut self, now: Instant) {
        if let Some(last) = self.last_started {
            let interval = (now - last).as_secs_f32() * 1000.0;
            self.intervals_ms.push_back(interval);
            if self.intervals_ms.len() > MAX_SAMPLES {
                self.intervals_ms.pop_front();
            }
        }
        self.last_started = Some(now);
        self.frames += 1;
    }

    /// The most recent frame interval, in milliseconds.
    fn latest_ms(&self) -> f32 {
        self.intervals_ms.back().copied().unwrap_or(0.0)
    }

    /// The frames-per-second implied by the average interval.
    fn fps(&self) -> f32 {
        let count = self.intervals_ms.len();
        if count == 0 {
            return 0.0;
        }
        let average = self.intervals_ms.iter().sum::<f32>() / count as f32;
        if average <= 0.0 {
            0.0
        } else {
            1000.0 / average
        }
    }
}

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
        // Frames land on refresh boundaries, so a cap at an exact multiple of the
        // refresh period (30 fps on a 60 Hz display) sits on a knife's edge:
        // without tolerance, delivery jitter drops it to the next-lower multiple.
        // Allow a frame up to a quarter of the interval early to absorb that.
        let min_interval = self.min_interval - self.min_interval / 4;
        if let Some(last) = self.last_render
            && now.duration_since(last) < min_interval
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

/// Records how often frames are drawn, and forwards everything else.
///
/// The interval is measured between [`begin_frame`](FramePipeline::begin_frame)
/// calls, so a frame the throttle defers shows up as a longer interval — which is
/// exactly what makes the cap visible in the graph.
struct MetricsPipeline<P> {
    inner: P,
    stats: Rc<RefCell<FrameStats>>,
}

impl<P> MetricsPipeline<P> {
    fn new(inner: P, stats: Rc<RefCell<FrameStats>>) -> Self {
        Self { inner, stats }
    }
}

impl<P: FramePipeline> FramePipeline for MetricsPipeline<P> {
    fn should_render(&mut self, is_dirty: bool, metrics: &WindowMetrics) -> bool {
        self.inner.should_render(is_dirty, metrics)
    }

    fn begin_frame(&mut self, window: &mut Window<'_>, cx: &mut App) {
        self.stats.borrow_mut().record(Instant::now());
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
trait FramePipelineExt: FramePipeline + Sized {
    fn max_fps(self, max_fps: u32) -> ThrottledPipeline<Self> {
        ThrottledPipeline::new(self, max_fps)
    }

    fn with_metrics(self, stats: Rc<RefCell<FrameStats>>) -> MetricsPipeline<Self> {
        MetricsPipeline::new(self, stats)
    }
}

impl<P: FramePipeline> FramePipelineExt for P {}

/// Renders the metrics the pipelines collect, and keeps the window animating.
struct MetricsView {
    stats: Rc<RefCell<FrameStats>>,
    frames: usize,
}

impl Render for MetricsView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.frames += 1;
        window.request_animation_frame();

        let (fps, latest_ms, frames, intervals) = {
            let stats = self.stats.borrow();
            (
                stats.fps(),
                stats.latest_ms(),
                stats.frames,
                stats.intervals_ms.iter().copied().collect::<Vec<f32>>(),
            )
        };

        // A slow sweep animates the accent bar, so there is always something to
        // draw even when the cap is holding the frame rate steady.
        let sweep = (self.frames as f32 * 0.04).sin() * 0.5 + 0.5;
        let target_height = TARGET_MS / GRAPH_MAX_MS * GRAPH_HEIGHT;

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_5()
            .bg(rgb(BACKGROUND))
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(22.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(TEXT))
                            .child("Frame pipeline decorators"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(MUTED))
                            .child("A rate cap and a metrics collector, composed out of tree."),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .child(stat_card(format!("{fps:.1}"), "FPS", rgb(ACCENT)))
                    .child(stat_card(format!("{latest_ms:.1}"), "frame ms", rgb(GOOD)))
                    .child(stat_card(frames.to_string(), "frames", rgb(VIOLET))),
            )
            .child(
                div()
                    .h(px(8.))
                    .rounded_full()
                    .bg(rgb(SURFACE))
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .rounded_full()
                            .bg(rgb(ACCENT))
                            .w(px(40.0 + 520.0 * sweep)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(MUTED))
                            .child("Frame time — bars turn amber once they pass the 33 ms target"),
                    )
                    .child(graph(&intervals, target_height)),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(rgb(MUTED))
                    .child("Capped at 30 fps. Each bar is the interval between two drawn frames."),
            )
    }
}

fn stat_card(value: String, label: &'static str, color: impl Into<Hsla>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .px_3()
        .py_2()
        .rounded_lg()
        .bg(rgb(SURFACE))
        .child(
            div()
                .text_size(px(22.))
                .font_weight(FontWeight::BOLD)
                .text_color(color)
                .child(value),
        )
        .child(div().text_size(px(12.)).text_color(rgb(MUTED)).child(label))
}

fn graph(intervals: &[f32], target_height: f32) -> impl IntoElement {
    div()
        .relative()
        .w_full()
        .h(px(GRAPH_HEIGHT))
        .rounded_lg()
        .bg(rgb(SURFACE))
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .flex()
                .flex_row()
                .items_end()
                .gap(px(2.))
                .children(intervals.iter().map(|&ms| frame_bar(ms))),
        )
        .child(
            div()
                .absolute()
                .left(px(0.))
                .right(px(0.))
                .bottom(px(target_height))
                .h(px(1.))
                .bg(rgb(BORDER)),
        )
}

fn frame_bar(ms: f32) -> impl IntoElement {
    let height = (ms / GRAPH_MAX_MS).clamp(0.0, 1.0) * GRAPH_HEIGHT;
    let color = if ms <= TARGET_MS {
        rgb(ACCENT)
    } else {
        rgb(WARN)
    };
    div().w(px(3.)).h(px(height)).bg(color)
}

fn run_example() {
    let stats = Rc::new(RefCell::new(FrameStats::default()));

    application()
        .with_frame_pipeline({
            let stats = stats.clone();
            move |_window_id| {
                Box::new(
                    StandardImmediatePipeline
                        .max_fps(30)
                        .with_metrics(stats.clone()),
                )
            }
        })
        .run(move |cx: &mut App| {
            if !example_support::load_fonts(cx) {
                return;
            }
            let bounds = Bounds::centered(None, size(px(640.), px(440.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| MetricsView {
                        stats: stats.clone(),
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
