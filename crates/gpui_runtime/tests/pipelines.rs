//! Frames can be observed and paced by pipelines written outside the framework.
//!
//! These are integration tests on purpose: they compile against the published
//! surface of `gpui_authoring` and `gpui_runtime` only, so anything they can do
//! with a frame, another crate can do.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

use gpui_authoring::{
    AnyWindowHandle, App, AppContext as _, Context, Entity, EntitySlotExt as _, FramePipeline,
    IntoElement, ParentElement as _, Render, Styled as _, TestAppContext, Window, WindowMetrics,
    div, px,
};
use gpui_runtime::{FramePipelineExt, InstrumentedPipeline, PhaseMetrics};

struct Counter(usize);

/// A view that renders another entity's state through a slot, so a frame has
/// some work in it.
struct SlotView {
    counter: Entity<Counter>,
}

impl Render for SlotView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            self.counter
                .slot("counter", |state, _window, _cx| {
                    div().w(px(state.0 as f32)).h(px(10.0))
                })
                .size_full(),
        )
    }
}

/// A pipeline written outside the framework can watch every pass of a frame.
#[test]
fn an_instrumented_pipeline_measures_the_root_passes() {
    let mut cx = TestAppContext::single();
    let metrics = Rc::new(RefCell::new(PhaseMetrics::default()));
    cx.update({
        let metrics = metrics.clone();
        move |cx| {
            cx.set_frame_pipeline_factory(Rc::new(move |_| {
                Box::new(InstrumentedPipeline::new(metrics.clone()))
            }));
        }
    });

    let counter = cx.new(|_| Counter(10));
    let window: AnyWindowHandle = cx.add_window(move |_, _| SlotView { counter }).into();
    cx.update_window(window, |_, window, cx| {
        window.draw(cx).clear(cx);
    })
    .unwrap();

    let metrics = *metrics.borrow();
    assert!(metrics.frames > 0, "the pipeline drove a frame");
    assert!(metrics.evaluate_passes > 0, "the roots were gathered");
    assert_eq!(
        metrics.evaluate_passes, metrics.layout_passes,
        "every frame that gathered its roots laid them out"
    );
    assert_eq!(
        metrics.layout_passes, metrics.paint_passes,
        "every frame that laid its roots out painted them"
    );
    assert!(
        metrics.root_passes() > Duration::ZERO,
        "the passes were timed"
    );
}

/// Counts the frames it draws, and defers them until it is allowed to draw.
struct PacingPipeline {
    frames: Rc<Cell<usize>>,
    asks: Rc<Cell<usize>>,
    allow: Rc<Cell<bool>>,
}

impl FramePipeline for PacingPipeline {
    fn should_render(&mut self, is_dirty: bool, _metrics: &WindowMetrics) -> bool {
        self.asks.set(self.asks.get() + 1);
        self.allow.get() && is_dirty
    }

    fn begin_frame(&mut self, window: &mut Window<'_>, cx: &mut App) {
        self.frames.set(self.frames.get() + 1);
        window.begin_frame(cx);
    }
}

/// A pipeline written outside the framework can defer a frame, and the frame it
/// deferred is the one that gets drawn once it stops deferring.
#[test]
fn a_pipeline_written_outside_the_framework_can_defer_a_frame() {
    let mut cx = TestAppContext::single();
    let frames = Rc::new(Cell::new(0));
    let asks = Rc::new(Cell::new(0));
    let allow = Rc::new(Cell::new(false));

    cx.update({
        let frames = frames.clone();
        let asks = asks.clone();
        let allow = allow.clone();
        move |cx| {
            cx.set_frame_pipeline_factory(Rc::new(move |_| {
                Box::new(PacingPipeline {
                    frames: frames.clone(),
                    asks: asks.clone(),
                    allow: allow.clone(),
                })
            }));
        }
    });

    let counter = cx.new(|_| Counter(10));
    let window = cx.add_window(move |_, _| SlotView { counter });

    // A window's first frame is not the pipeline's to defer.
    frames.set(0);
    asks.set(0);

    window.update(&mut cx, |_, _, cx| cx.notify()).unwrap();

    assert!(asks.get() > 0, "the pipeline was asked about the frame");
    assert_eq!(frames.get(), 0, "the deferred frame was not drawn");

    // The deferred frame was left pending, so allowing the next one draws it.
    allow.set(true);
    cx.update(|_| {});

    assert!(
        frames.get() > 0,
        "the deferred frame was drawn once allowed"
    );
}

/// A pipeline written outside the framework can cap a frame rate by wrapping
/// another pipeline, drawing its first frame and deferring one that follows it
/// too soon.
#[test]
fn a_throttled_pipeline_defers_a_frame_that_arrives_too_soon() {
    let mut cx = TestAppContext::single();
    let metrics = Rc::new(RefCell::new(PhaseMetrics::default()));

    cx.update({
        let metrics = metrics.clone();
        move |cx| {
            cx.set_frame_pipeline_factory(Rc::new(move |_| {
                // One frame a second: a frame that follows another one within a
                // test is always too soon to draw again.
                Box::new(InstrumentedPipeline::new(metrics.clone()).max_fps(1))
            }));
        }
    });

    let counter = cx.new(|_| Counter(10));
    let window = cx.add_window(move |_, _| SlotView { counter });

    // The throttle has nothing to compare a timestamp against yet, so it draws.
    metrics.borrow_mut().frames = 0;
    window.update(&mut cx, |_, _, cx| cx.notify()).unwrap();
    assert!(metrics.borrow().frames > 0, "the first frame was drawn");

    // A frame that follows within the throttle's one-second window is deferred.
    metrics.borrow_mut().frames = 0;
    window.update(&mut cx, |_, _, cx| cx.notify()).unwrap();
    assert_eq!(
        metrics.borrow().frames,
        0,
        "the frame that arrived too soon was deferred"
    );
}
