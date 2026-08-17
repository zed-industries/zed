use approx::AbsDiffEq;
use core::borrow::Borrow;

/// Methods to tell the tell about the visual characteristics of the color
pub trait VisualColor<T>
where
    T: AbsDiffEq<Epsilon = T>,
{
    /// Returns `true`, if the color is grey within the bounds of `epsilon`-tolerance,
    /// i.e. visually only the luminance value matters, not the color's hue.
    fn is_grey(&self, epsilon: T::Epsilon) -> bool;

    /// Returns `true`, if the color is white within the bounds of `epsilon`-tolerance,
    /// i.e. the color's hue is irrelevant **and** it is at or beyond the
    /// `sRGB` maximum brightness.
    /// A color that is *only* at or beyond maximum brightness isn't
    /// necessarily white. It may also be a bright shining hue.
    fn is_white(&self, epsilon: T::Epsilon) -> bool;

    /// Returns `true`, if the color is black within the bounds of `epsilon`-tolerance.
    fn is_black(&self, epsilon: T::Epsilon) -> bool;
}

/// Methods to compare visual characteristics of two colors
pub trait VisuallyEqual<O, S, T>: VisualColor<T>
where
    T: AbsDiffEq<Epsilon = T> + Clone,
    S: Borrow<Self>,
    O: Borrow<Self>,
{
    /// Returns true, if `self` and `other` are either both white or both black
    fn both_black_or_both_white(s: S, o: O, epsilon: T::Epsilon) -> bool {
        s.borrow().is_white(epsilon.clone()) && o.borrow().is_white(epsilon.clone())
            || s.borrow().is_black(epsilon.clone()) && o.borrow().is_black(epsilon)
    }

    /// Returns true, if `self` and `other` are both fully desaturated
    fn both_greyscale(s: S, o: O, epsilon: T::Epsilon) -> bool {
        s.borrow().is_grey(epsilon.clone()) && o.borrow().is_grey(epsilon)
    }
    fn visually_eq(s: S, o: O, epsilon: T::Epsilon) -> bool;
}
