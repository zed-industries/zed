//! A [`FramePipeline`] that times the passes a frame is made of.
//!
//! This is also the proof that the pipeline SPI crosses the crate boundary:
//! everything here is built from `gpui_authoring`'s public surface, so a host
//! program or an embedder can do the same from outside the framework.

use std::{cell::RefCell, rc::Rc, time::Duration, time::Instant};

use gpui_authoring::{App, FramePipeline, PreparedRoots, Window};

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
