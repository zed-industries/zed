//! The engine execution seam for a window frame.
//!
//! A `FrameSession` owns the frame's layout tree behind interior mutability.
//! The facade drives it by converting its `Style` to a taffy style and passing
//! itself as the measure context; because the tree is behind a `RefCell`, the
//! facade can do that without taking the engine out of the window to satisfy
//! the borrow checker.
//!
//! The scene graph stays in the facade's double-buffered frame, and frame
//! presentation stays in the facade because `PlatformWindow` lives above this
//! crate.

use crate::{LayoutId, MeasureContext, TaffyLayoutEngine};
use gpui_types::{AvailableSpace, Bounds, Pixels, Size};
use std::cell::RefCell;

/// A window's layout engine session.
pub struct FrameSession {
    layout_engine: RefCell<TaffyLayoutEngine>,
}

impl Default for FrameSession {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameSession {
    /// Creates an empty session with a fresh layout tree.
    pub fn new() -> Self {
        Self {
            layout_engine: RefCell::new(TaffyLayoutEngine::new()),
        }
    }

    /// Discards the layout tree and cached bounds, ready for a new frame.
    pub fn clear(&self) {
        self.layout_engine.borrow_mut().clear();
    }

    /// Adds a node built from an already-converted taffy style.
    pub fn request_layout(
        &self,
        taffy_style: taffy::style::Style,
        children: &[LayoutId],
    ) -> LayoutId {
        self.layout_engine
            .borrow_mut()
            .request_layout(taffy_style, children)
    }

    /// Adds a leaf whose size is resolved by `measure` during layout.
    pub fn request_measured_layout(
        &self,
        taffy_style: taffy::style::Style,
        measure: impl FnMut(
            Size<Option<Pixels>>,
            Size<AvailableSpace>,
            &mut dyn MeasureContext,
        ) -> Size<Pixels>
        + 'static,
    ) -> LayoutId {
        self.layout_engine
            .borrow_mut()
            .request_measured_layout(taffy_style, measure)
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
