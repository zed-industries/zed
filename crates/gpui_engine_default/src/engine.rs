//! The engine execution seam for a window frame.
//!
//! A `FrameSession` owns the frame's layout engine behind interior mutability,
//! so the facade can drive it with `&self` without taking the engine out of the
//! window to satisfy the borrow checker. The engine itself is a trait object, so
//! the layout implementation can be swapped without the facade naming it.
//!
//! The scene graph stays in the facade's double-buffered frame, and frame
//! presentation stays in the facade because `PlatformWindow` lives above this
//! crate.

use gpui_engine::{BoxedMeasureFn, EngineLayoutStyle, LayoutEngine, LayoutId, MeasureContext};
use gpui_types::{AvailableSpace, Bounds, Pixels, Size};
use std::cell::RefCell;

/// A window's layout engine session.
pub struct FrameSession {
    layout_engine: RefCell<Box<dyn LayoutEngine>>,
}

impl FrameSession {
    /// Creates a session that drives `layout_engine`.
    pub fn new(layout_engine: Box<dyn LayoutEngine>) -> Self {
        Self {
            layout_engine: RefCell::new(layout_engine),
        }
    }

    /// Discards the layout tree and cached bounds, ready for a new frame.
    pub fn clear(&self) {
        self.layout_engine.borrow_mut().clear();
    }

    /// Adds a node built from `style`.
    pub fn request_layout(
        &self,
        style: &EngineLayoutStyle,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        self.layout_engine
            .borrow_mut()
            .request_layout(style, rem_size, scale_factor, children)
    }

    /// Adds a leaf whose size is resolved by `measure` during layout.
    pub fn request_measured_layout(
        &self,
        style: &EngineLayoutStyle,
        rem_size: Pixels,
        scale_factor: f32,
        measure: BoxedMeasureFn,
    ) -> LayoutId {
        self.layout_engine.borrow_mut().request_measured_layout(
            style,
            rem_size,
            scale_factor,
            measure,
        )
    }

    /// Treats any `auto` dimension of `id`'s style as filling `size`.
    pub fn stretch_auto_size_to_fill(&self, id: LayoutId, size: Size<Pixels>, scale_factor: f32) {
        self.layout_engine
            .borrow_mut()
            .stretch_auto_size_to_fill(id, size, scale_factor)
    }

    /// Computes the layout of `id` within `available_space`.
    pub fn compute_layout(
        &self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        scale_factor: f32,
        context: &mut dyn MeasureContext,
    ) {
        self.layout_engine
            .borrow_mut()
            .compute_layout(id, available_space, scale_factor, context)
    }

    /// Returns the pixel-snapped bounds of `id` relative to the window.
    pub fn layout_bounds(&self, id: LayoutId, scale_factor: f32) -> Bounds<Pixels> {
        self.layout_engine
            .borrow_mut()
            .layout_bounds(id, scale_factor)
    }
}
