use crate::angle::{AngleEq, HalfRotation, RealAngle, SignedAngle};
use crate::num::{One, Zero};
use crate::visual::{VisualColor, VisuallyEqual};
use crate::{HasBoolMask, Okhsl, OklabHue};
use approx::AbsDiffEq;
use std::borrow::Borrow;
use std::ops::{Mul, Neg, Sub};

impl<T> VisualColor<T> for Okhsl<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + AbsDiffEq<Epsilon = T>
        + One
        + Zero
        + Neg<Output = T>,
    T::Epsilon: Clone,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    /// Returns true, if `saturation == 0`
    fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        debug_assert!(self.saturation >= -epsilon.clone());
        self.saturation.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `Self::is_grey` && `lightness >= 1`,
    /// i.e. the color's hue is irrelevant **and** it is at or beyond the
    /// `sRGB` maximum luminance. A color at or beyond maximum brightness isn't
    /// necessarily white. It may also be a bright shining hue.
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.lightness > T::one()
            || self.lightness.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns true if `lightness == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        debug_assert!(self.lightness >= -epsilon.clone());
        self.lightness <= epsilon
    }
}

impl<S, O, T> VisuallyEqual<O, S, T> for Okhsl<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Sub<Output = T>
        + AbsDiffEq<Epsilon = T>
        + Neg<Output = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation + Mul<Output = T::Epsilon>,
    S: Borrow<Self> + Copy,
    O: Borrow<Self> + Copy,
{
    fn visually_eq(s: S, o: O, epsilon: T::Epsilon) -> bool {
        VisuallyEqual::both_black_or_both_white(s, o, epsilon.clone())
            || VisuallyEqual::both_greyscale(s, o, epsilon.clone())
                && s.borrow()
                    .lightness
                    .abs_diff_eq(&o.borrow().lightness, epsilon.clone())
            || s.borrow().hue.abs_diff_eq(&o.borrow().hue, epsilon.clone())
                && s.borrow()
                    .saturation
                    .abs_diff_eq(&o.borrow().saturation, epsilon.clone())
                && s.borrow()
                    .lightness
                    .abs_diff_eq(&o.borrow().lightness, epsilon)
    }
}
