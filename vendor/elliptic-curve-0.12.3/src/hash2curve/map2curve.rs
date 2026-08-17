//! Traits for mapping field elements to points on the curve.

/// Trait for converting field elements into a point
/// via a mapping method like Simplified Shallue-van de Woestijne-Ulas
/// or Elligator
pub trait MapToCurve {
    /// The output point
    type Output;

    /// Map a field element into a point
    fn map_to_curve(&self) -> Self::Output;
}
