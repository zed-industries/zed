//! Layout identity and the contract between the facade and a layout engine.
//!
//! A layout engine owns its tree. The facade refers to nodes only through the
//! opaque [`LayoutId`], so no layout-solver type crosses this boundary and an
//! alternative engine can back the same tree.

use crate::EngineLayoutStyle;
use gpui_types::{AvailableSpace, Bounds, Pixels, Size};
use std::any::Any;

/// A unique identifier for a layout node, generated when a layout is requested.
///
/// The value is opaque to callers; only the engine that produced it interprets
/// it.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LayoutId(pub u64);

/// The type-erased handles a custom measure callback receives from the engine.
///
/// The facade supplies a context whose `handles` return its window and
/// application handles as `Any`; the engine only forwards the context to the
/// callback stored when the node was created.
pub trait MeasureContext {
    /// Returns the facade's window and application handles, type-erased.
    fn handles(&mut self) -> (&mut dyn Any, &mut dyn Any);
}

/// A type-erased measure callback supplied to
/// [`LayoutEngine::request_measured_layout`].
pub type BoxedMeasureFn = Box<
    dyn FnMut(Size<Option<Pixels>>, Size<AvailableSpace>, &mut dyn MeasureContext) -> Size<Pixels>,
>;

/// A layout solver.
///
/// The facade drives the whole frame lifecycle through this trait, so the layout
/// implementation can be swapped without the authoring layer naming it.
pub trait LayoutEngine {
    /// Discards every node and cached bounds, ready for a fresh frame.
    fn clear(&mut self);

    /// Adds a leaf or container node built from `style`.
    fn request_layout(
        &mut self,
        style: &EngineLayoutStyle,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
    ) -> LayoutId;

    /// Adds a leaf whose size is resolved by `measure` during layout.
    fn request_measured_layout(
        &mut self,
        style: &EngineLayoutStyle,
        rem_size: Pixels,
        scale_factor: f32,
        measure: BoxedMeasureFn,
    ) -> LayoutId;

    /// Treats any `auto` dimension of `id`'s style as filling `size`.
    fn stretch_auto_size_to_fill(&mut self, id: LayoutId, size: Size<Pixels>, scale_factor: f32);

    /// Computes the layout of `id` within `available_space`, invoking stored
    /// measure callbacks with `context`.
    fn compute_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        scale_factor: f32,
        context: &mut dyn MeasureContext,
    );

    /// Returns the pixel-snapped bounds of `id` relative to the window.
    fn layout_bounds(&mut self, id: LayoutId, scale_factor: f32) -> Bounds<Pixels>;
}
