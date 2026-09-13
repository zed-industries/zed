//! The renderer contract that engines implement.

use crate::Scene;

/// A renderer that presents a [`Scene`] to some target.
///
/// OS backends and alternative engines implement this; `gpui` produces the
/// [`Scene`] and submits it through the window's platform implementation.
pub trait SceneRenderer: 'static {
    /// Encodes and submits `scene`.
    fn draw(&mut self, scene: &Scene);
}
