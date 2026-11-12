use std::time::Duration;

use gpui::Transformation;

/// A trait for components that can be transformed.
pub trait Transformable {
    /// Sets the transformation for the element.
    fn transform(self, transformation: Transformation) -> Self;

    /// Rotates the element continuously, one full turn per `period`, using
    /// [`gpui::Svg::with_rotation_animation`].
    fn animate_rotation(self, period: Duration) -> Self;
}
