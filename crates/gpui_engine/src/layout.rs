//! Layout identity and the contract between the facade and a layout engine.
//!
//! A layout engine owns its tree. The facade refers to nodes only through the
//! opaque [`LayoutId`], so no layout-solver type crosses this boundary and an
//! alternative engine can back the same tree.

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
