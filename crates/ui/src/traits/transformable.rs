use gpui::Transformation;

/// A trait for components that can be transformed.
pub trait Transformable {
    /// Sets the transformation for the element.
    fn transform(self, transformation: Transformation) -> Self;
}
