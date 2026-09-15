//! A [`FramePipeline`] that times the passes a frame is made of.
//!
//! This is also the proof that the pipeline SPI crosses the crate boundary:
//! everything here is built from `gpui_authoring`'s public surface, so a host
//! program or an embedder can do the same from outside the framework.

use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

use gpui_authoring::{App, FocusId, FramePipeline, PreparedRoots, Window, WindowMetrics};

/// What an [`InstrumentedPipeline`] has measured.
///
/// Durations accumulate across frames, so divide by [`frames`](Self::frames) for
/// an average. A frame drives the root passes once unless the pipeline defers it,
/// so `evaluate_passes` and [`frames`](Self::frames) differ only for a frame that
/// opened without drawing its roots.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhaseMetrics {
    /// Frames driven.
    pub frames: usize,
    /// Times the roots were gathered.
    pub evaluate_passes: usize,
    /// Times the roots were laid out and prepainted.
    pub layout_passes: usize,
    /// Times the roots were painted.
    pub paint_passes: usize,
    /// Time spent gathering the roots.
    pub evaluate: Duration,
    /// Time spent laying out and prepainting the roots.
    pub layout: Duration,
    /// Time spent painting the roots.
    pub paint: Duration,
}

impl PhaseMetrics {
    /// The time the root passes took in total.
    pub fn root_passes(&self) -> Duration {
        self.evaluate + self.layout + self.paint
    }
}

/// Composition for pipelines.
///
/// A pipeline can wrap another one and leave everything it does not change to it,
/// which is how independent concerns — capping the frame rate, timing the passes,
/// inspecting what a frame is about to draw — stack in any order.
///
/// A decorator forwards the passes it does not change to the pipeline it wraps;
/// the default [`draw`](FramePipeline::draw) then runs those forwards as one
/// frame. A pass it forgets to forward falls back to the standard implementation
/// of that pass, not the inner pipeline's, so a decorator must forward every pass
/// it wants the inner pipeline to decide.
pub trait FramePipelineExt: FramePipeline + Sized {
    /// Caps this pipeline at `max_fps` frames a second.
    ///
    /// See [`ThrottledPipeline`].
    fn max_fps(self, max_fps: u32) -> ThrottledPipeline<Self> {
        ThrottledPipeline::new(self, max_fps)
    }
}

impl<P: FramePipeline> FramePipelineExt for P {}

/// Defers frames that arrive sooner than a target rate.
///
/// Changes only [`should_render`](FramePipeline::should_render) and forwards every
/// other pass to the pipeline it wraps, so the wrapped pipeline draws exactly as it
/// would have. Wrap the standard pipeline to cap an application's frame rate, or
/// wrap another decorator — `.max_fps(30)` reads the same on an
/// [`InstrumentedPipeline`] as on a
/// [`StandardImmediatePipeline`][gpui_authoring::StandardImmediatePipeline].
///
/// A deferred frame is not a dropped one. It leaves the window dirty and leaves
/// what the frame would have done pending, so the next frame draws it — including
/// when the platform asks for a frame on its own, which it does while the window is
/// dirty. Deferring does not need to schedule that next frame: `should_render` is
/// asked before a frame's work starts, with no window to schedule one with.
pub struct ThrottledPipeline<P> {
    inner: P,
    min_interval: Duration,
    last_frame: Option<Instant>,
}

impl<P> ThrottledPipeline<P> {
    /// Limits `inner` to `max_fps` frames a second.
    ///
    /// Zero is treated as one frame a second. A pipeline that should never draw is
    /// what answering `false` from `should_render` is for.
    pub fn new(inner: P, max_fps: u32) -> Self {
        Self {
            inner,
            min_interval: Duration::from_secs_f64(1.0 / f64::from(max_fps.max(1))),
            last_frame: None,
        }
    }

    /// The pipeline this one wraps.
    pub fn inner(&self) -> &P {
        &self.inner
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
        if let Some(last_frame) = self.last_frame
            && now.duration_since(last_frame) < self.min_interval
        {
            return false;
        }

        self.last_frame = Some(now);
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

    fn end_frame(
        &mut self,
        window: &mut Window<'_>,
        cx: &mut App,
        focus_before_listeners: Option<FocusId>,
    ) {
        self.inner.end_frame(window, cx, focus_before_listeners);
    }
}

/// Draws frames the way [`StandardImmediatePipeline`][std] does, timing each of
/// the root passes as it goes.
///
/// [std]: gpui_authoring::StandardImmediatePipeline
///
/// Install it with [`Application::with_frame_pipeline`][app], which builds one per
/// window from the factory:
///
/// ```no_run
/// # use std::{cell::RefCell, rc::Rc};
/// # use gpui_runtime::{Application, InstrumentedPipeline, PhaseMetrics};
/// # fn example(application: Application) -> Application {
/// let metrics = Rc::new(RefCell::new(PhaseMetrics::default()));
/// application.with_frame_pipeline({
///     let metrics = metrics.clone();
///     move |_window_id| Box::new(InstrumentedPipeline::new(metrics.clone()))
/// })
/// # }
/// ```
///
/// [app]: crate::Application::with_frame_pipeline
pub struct InstrumentedPipeline {
    metrics: Rc<RefCell<PhaseMetrics>>,
}

impl InstrumentedPipeline {
    /// A pipeline that records into `metrics`, which the caller keeps in order to
    /// read what it measures.
    pub fn new(metrics: Rc<RefCell<PhaseMetrics>>) -> Self {
        Self { metrics }
    }
}

impl FramePipeline for InstrumentedPipeline {
    fn begin_frame(&mut self, window: &mut Window<'_>, cx: &mut App) {
        self.metrics.borrow_mut().frames += 1;
        window.begin_frame(cx);
    }

    fn evaluate_roots(&mut self, window: &mut Window<'_>, cx: &mut App) -> PreparedRoots {
        let start = Instant::now();
        let roots = window.evaluate_roots(cx);
        let mut metrics = self.metrics.borrow_mut();
        metrics.evaluate += start.elapsed();
        metrics.evaluate_passes += 1;
        roots
    }

    fn layout_roots(&mut self, window: &mut Window<'_>, roots: &mut PreparedRoots, cx: &mut App) {
        let start = Instant::now();
        window.layout_roots(roots, cx);
        let mut metrics = self.metrics.borrow_mut();
        metrics.layout += start.elapsed();
        metrics.layout_passes += 1;
    }

    fn paint_roots(&mut self, window: &mut Window<'_>, roots: PreparedRoots, cx: &mut App) {
        let start = Instant::now();
        window.paint_roots(roots, cx);
        let mut metrics = self.metrics.borrow_mut();
        metrics.paint += start.elapsed();
        metrics.paint_passes += 1;
    }
}
