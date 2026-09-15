//! The phases one frame goes through, as a seam.
//!
//! A window renders by handing its frame to a [`FramePipeline`]. The default,
//! [`StandardImmediatePipeline`], re-evaluates the view tree, lays it out and
//! paints what it finds — the immediate-mode loop described in the
//! [authoring guide](crate::_authoring). Installing a different one is how a
//! window renders some other way without knowing about it.
//!
//! Every phase falls back to the `Window` method that implements it, so a
//! pipeline can override one phase and leave the rest alone.

use crate::{App, ArenaClearNeeded, FocusId, Window, window::ElementArenaScope};

/// The phases of drawing one frame, in the order [`FramePipeline::draw`] runs
/// them.
///
/// Implementations belong to this crate: driving a frame means reaching into the
/// window's frame state, which is not public. A phase that is not overridden
/// behaves exactly as [`StandardImmediatePipeline`] does.
pub trait FramePipeline: 'static {
    /// Opens the frame: samples the platform window, resets the scratch state
    /// the frame rebuilds, and takes ownership of the invalidations this frame
    /// owes.
    fn begin_frame(&mut self, window: &mut Window<'_>, cx: &mut App) {
        window.begin_frame(cx);
    }

    /// Lays out and paints the window's roots: the window's own view tree, plus
    /// a prompt, drag image or tooltip if the frame has one.
    fn draw_roots(&mut self, window: &mut Window<'_>, cx: &mut App) {
        window.draw_roots(cx);
    }

    /// Closes the painted frame: records the views it touched and hands the
    /// platform the input handler the frame asked for.
    fn finish_frame(&mut self, window: &mut Window<'_>, cx: &mut App) {
        window.finish_frame(cx);
    }

    /// Retires the painted frame, swaps it in, and dispatches the focus changes
    /// the swap produced.
    ///
    /// Returns the focus that was current before the listeners ran: they may
    /// move it, and the caller has to tell those moves apart from its own.
    fn complete_frame(&mut self, window: &mut Window<'_>, cx: &mut App) -> Option<FocusId> {
        window.complete_frame(cx)
    }

    /// Closes the frame out and marks it ready to present.
    fn end_frame(
        &mut self,
        window: &mut Window<'_>,
        cx: &mut App,
        focus_before_listeners: Option<FocusId>,
    ) {
        window.end_frame(cx, focus_before_listeners);
    }

    /// Draws one frame by running the phases above in order.
    ///
    /// Override this to drive the phases differently, or override a single phase
    /// to change what happens inside it.
    fn draw(&mut self, window: &mut Window<'_>, cx: &mut App) -> ArenaClearNeeded {
        // Drain every draw in profiler builds so a previous frame's
        // first-invalidation timestamp can't be attributed to this one.
        #[cfg(feature = "profiler")]
        let frame_dirty = window.core.invalidator.take_frame_dirty();
        #[cfg(feature = "profiler")]
        window.core.window_profiler.begin_draw();

        // Set up the per-App arena for element allocation during this draw.
        // This ensures that multiple test Apps have isolated arenas.
        let arena_scope = ElementArenaScope::enter(&cx.element_arena);

        self.begin_frame(window, cx);
        window.restore_input_handler();
        if !cx.mode.skip_drawing() {
            self.draw_roots(window, cx);
            #[cfg(feature = "profiler")]
            {
                let viewport_size = window.core.viewport_size;
                let scale_factor = window.scale_factor();
                window.core.debug_frame_overlay.paint(
                    &mut window.frame_state.next_frame.scene,
                    viewport_size,
                    scale_factor,
                );
            }
        }
        self.finish_frame(window, cx);
        let focus_before_listeners = self.complete_frame(window, cx);
        self.end_frame(window, cx, focus_before_listeners);

        #[cfg(feature = "profiler")]
        {
            let draw_duration = window
                .core
                .window_profiler
                .end_draw(frame_dirty.dirty_at, frame_dirty.invalidations);
            window.core.debug_frame_overlay.record_frame(draw_duration);
        }

        // Exit the scope to obtain the arena-clear token this draw owes; the
        // scope's teardown itself happens in `ElementArenaScope::drop`.
        arena_scope.exit(&cx.element_arena)
    }
}

/// The pipeline a window draws with unless another one is installed.
///
/// Draws every frame from scratch: the view tree is re-evaluated, laid out and
/// painted, so what is on screen is always a function of current state.
pub struct StandardImmediatePipeline;

impl FramePipeline for StandardImmediatePipeline {}
