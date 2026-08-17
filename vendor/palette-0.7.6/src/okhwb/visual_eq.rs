use crate::angle::{AngleEq, HalfRotation, RealAngle, SignedAngle};
use crate::num::{Arithmetics, One, Zero};
use crate::visual::{VisualColor, VisuallyEqual};
use crate::{HasBoolMask, Okhwb, OklabHue};
use approx::AbsDiffEq;
use std::borrow::Borrow;

impl<T> VisualColor<T> for Okhwb<T>
where
    T: PartialOrd + HasBoolMask<Mask = bool> + AbsDiffEq<Epsilon = T> + One + Zero + Arithmetics,
    T::Epsilon: Clone,
    OklabHue<T>: AbsDiffEq<Epsilon = T::Epsilon>,
{
    /// Returns `true`, if `self.blackness + self.whiteness >= 1`,
    /// assuming (but not asserting) that neither
    /// `blackness` nor `whiteness` can be negative.
    fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        let wb_sum = self.blackness.clone() + self.whiteness.clone();
        wb_sum > T::one() || wb_sum.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns `true`, if `Self::is_grey && blackness == 0`,
    /// i.e. the color's hue is irrelevant **and** the color contains
    /// no black component it must be white.
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.blackness < epsilon
    }

    /// Returns `true` if `Self::is_grey && whiteness == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        self.is_grey(epsilon.clone()) && self.whiteness < epsilon
    }
}

impl<S, O, T> VisuallyEqual<O, S, T> for Okhwb<T>
where
    T: PartialOrd
        + HasBoolMask<Mask = bool>
        + RealAngle
        + SignedAngle
        + Zero
        + One
        + AngleEq<Mask = bool>
        + Arithmetics
        + AbsDiffEq<Epsilon = T>
        + Clone,
    T::Epsilon: Clone + HalfRotation,
    S: Borrow<Self> + Copy,
    O: Borrow<Self> + Copy,
{
    fn visually_eq(s: S, o: O, epsilon: T::Epsilon) -> bool {
        VisuallyEqual::both_black_or_both_white(s, o, epsilon.clone())
            || VisuallyEqual::both_greyscale(s, o, epsilon.clone())
                && s.borrow()
                    .whiteness
                    .abs_diff_eq(&o.borrow().whiteness, epsilon.clone())
                && s.borrow()
                    .blackness
                    .abs_diff_eq(&o.borrow().blackness, epsilon.clone())
            || s.borrow().hue.abs_diff_eq(&o.borrow().hue, epsilon.clone())
                && s.borrow()
                    .blackness
                    .abs_diff_eq(&o.borrow().blackness, epsilon.clone())
                && s.borrow()
                    .whiteness
                    .abs_diff_eq(&o.borrow().whiteness, epsilon)
    }
}
