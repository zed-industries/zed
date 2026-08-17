use crate::angle::{AngleEq, HalfRotation, RealAngle, SignedAngle};
use crate::num::{One, Zero};
use crate::visual::{VisualColor, VisuallyEqual};
use crate::{HasBoolMask, Oklab, OklabHue};
use approx::AbsDiffEq;
use std::borrow::Borrow;
use std::ops::{Mul, Neg, Sub};

impl<T> VisualColor<T> for Oklab<T>
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
    /// Returns true, if `chroma == 0`
    #[allow(dead_code)]
    fn is_grey(&self, epsilon: T::Epsilon) -> bool {
        self.a.abs_diff_eq(&T::zero(), epsilon.clone()) && self.b.abs_diff_eq(&T::zero(), epsilon)
    }

    /// Returns true, if `lightness >= 1`
    ///
    /// **Note:** `sRGB` to `Oklab` conversion uses `f32` constants.
    /// A tolerance `epsilon >= 1e-8` is required to reliably detect white.
    /// Conversion of `sRGB` via XYZ requires `epsilon >= 1e-5`
    fn is_white(&self, epsilon: T::Epsilon) -> bool {
        self.l >= T::one() || self.l.abs_diff_eq(&T::one(), epsilon)
    }

    /// Returns true, if `lightness == 0`
    fn is_black(&self, epsilon: T::Epsilon) -> bool {
        self.l.abs_diff_eq(&T::zero(), epsilon)
    }
}

impl<S, O, T> VisuallyEqual<O, S, T> for Oklab<T>
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
            || s.borrow().l.abs_diff_eq(&o.borrow().l, epsilon.clone())
                && s.borrow().a.abs_diff_eq(&o.borrow().a, epsilon.clone())
                && s.borrow().b.abs_diff_eq(&o.borrow().b, epsilon)
    }
}
