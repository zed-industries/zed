//! # Writing a frame pipeline
//!
//! A window does not know how to draw a frame. It hands each one to a
//! [`FramePipeline`](crate::FramePipeline), and the default —
//! [`StandardImmediatePipeline`](crate::StandardImmediatePipeline) — is the
//! immediate-mode loop described in the [authoring guide](crate::_authoring):
//! rebuild the view tree, lay it out, paint what it drew.
//!
//! Replacing that loop is how a frame is built some other way. A pipeline decides
//! two things: what a frame is made of, and whether the frame happens at all.
//!
//! ## What a frame is made of
//!
//! The passes are the phases a frame goes through, in order:
//! [`begin_frame`](crate::FramePipeline::begin_frame),
//! [`evaluate_roots`](crate::FramePipeline::evaluate_roots),
//! [`layout_roots`](crate::FramePipeline::layout_roots),
//! [`paint_roots`](crate::FramePipeline::paint_roots),
//! [`finish_frame`](crate::FramePipeline::finish_frame),
//! [`complete_frame`](crate::FramePipeline::complete_frame) and
//! [`end_frame`](crate::FramePipeline::end_frame).
//! [`draw`](crate::FramePipeline::draw) runs them in that order, and is what a
//! window calls when it draws.
//!
//! Every pass has a default body that calls the [`Window`](crate::Window) method
//! implementing it, so a pipeline starts by overriding the one pass it cares
//! about:
//!
//! - [`evaluate_roots`](crate::FramePipeline::evaluate_roots) to choose what the
//!   frame draws, before any of it is measured. The
//!   [`PreparedRoots`](crate::PreparedRoots) it returns carries the window's view
//!   tree and, at most, one overlay: a prompt, a drag image or a tooltip.
//! - [`layout_roots`](crate::FramePipeline::layout_roots) to change how that is
//!   measured, or to lay out something of your own beside it. When it returns,
//!   the frame's geometry is settled and the pointer has been hit tested.
//! - [`paint_roots`](crate::FramePipeline::paint_roots) to draw differently, once
//!   everything is placed and its hitboxes registered.
//! - [`begin_frame`](crate::FramePipeline::begin_frame) and the three passes that
//!   close a frame, to bracket it.
//!
//! A pipeline that needs to drive the passes itself — in a different order, or
//! more than once — overrides [`draw`](crate::FramePipeline::draw). That means
//! reproducing what the default `draw` sets up around them: the per-app element
//! arena, and the input handler the previous frame left behind. Both are private
//! to this crate, so from outside it the passes are the seam.
//!
//! ## Whether a frame happens
//!
//! [`should_render`](crate::FramePipeline::should_render) is asked before a
//! frame's work starts. Answering `false` leaves the window as it was and leaves
//! the frame's work pending, so the next frame draws it: deferring a frame is not
//! dropping it. This is where pacing belongs — dropping the frames a display
//! cannot keep up with, or coalescing a burst of invalidations into one frame —
//! and it is the reason a pipeline instance is kept: the same one draws every
//! frame of the window it was built for, so it can remember a rate, a deadline,
//! or the timings behind it.
//!
//! Two frames are drawn whatever it answers: a window's first, and one forced
//! after the GPU device was lost, whose cached content may not be replayable.
//!
//! ## Where an implementation lives
//!
//! Anywhere above this crate. `gpui_runtime`'s `InstrumentedPipeline` is a worked
//! example: it times the root passes of the pipeline it wraps and forwards
//! everything else, using nothing but this crate's public surface.
//!
//! ## Composing pipelines
//!
//! [`FramePipeline`](crate::FramePipeline) is object-safe and every pass defaults
//! to the [`Window`](crate::Window) method implementing it, so independent concerns
//! compose as decorators: a pipeline wraps another, changes the one pass it cares
//! about, and forwards the rest. `gpui_runtime`'s `ThrottledPipeline` caps a frame
//! rate and its `InstrumentedPipeline` times the root passes this way, and the
//! `frame_pipeline_decorator` example in the `gpui` crate writes the same pattern
//! from an application, entirely outside the framework.
//!
//! ## Installing one
//!
//! `Application::with_frame_pipeline` takes a factory and builds a pipeline per
//! window, so a pipeline that holds per-window state gets its own for each. Test
//! harnesses install through the same factory the application method forwards to.
//!
//! ## Testing one
//!
//! Frames do not need a platform to be driven: the test harness renders windows
//! in-process, so a pipeline can be installed, drawn a frame or three, and its
//! effect asserted.
