use crate::{
    cast::ArrayCast,
    clamp,
    num::{Arithmetics, Clamp, One, Real, Zero},
    stimulus::Stimulus,
    Alpha,
};

use super::{blend_alpha, zip_colors, PreAlpha, Premultiply};

/// The Porter Duff composition operators, [as described by
/// W3C](https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators).
///
/// This set of operators exclude the variants where source and destination are
/// swapped, as well as the "clear", "copy" and "destination" operators. Those
/// can easily be achieved using other means.
pub trait Compose {
    /// Place `self` over `other`. This is the good old common alpha composition
    /// equation.
    #[must_use]
    fn over(self, other: Self) -> Self;

    /// Results in the parts of `self` that overlaps the visible parts of
    /// `other`.
    #[must_use]
    fn inside(self, other: Self) -> Self;

    /// Results in the parts of `self` that lies outside the visible parts of
    /// `other`.
    #[must_use]
    fn outside(self, other: Self) -> Self;

    /// Place `self` over only the visible parts of `other`.
    #[must_use]
    fn atop(self, other: Self) -> Self;

    /// Results in either `self` or `other`, where they do not overlap.
    #[must_use]
    fn xor(self, other: Self) -> Self;

    /// Add `self` and `other`. This uses the alpha component to regulate the
    /// effect, so it's not just plain component wise addition.
    #[must_use]
    fn plus(self, other: Self) -> Self;
}

impl<C, T, const N: usize> Compose for PreAlpha<C>
where
    C: ArrayCast<Array = [T; N]> + Premultiply<Scalar = T>,
    T: Real + Zero + One + Arithmetics + Clamp + Clone,
{
    #[inline]
    fn over(self, mut other: Self) -> Self {
        for (src, dst) in zip_colors(self.color, &mut other.color) {
            *dst = src + (T::one() - &self.alpha) * &*dst;
        }

        other.alpha = blend_alpha(self.alpha, other.alpha);

        other
    }

    #[inline]
    fn inside(self, mut other: Self) -> Self {
        for (src, dst) in zip_colors(self.color, &mut other.color) {
            *dst = src * &other.alpha;
        }

        other.alpha = clamp(self.alpha * other.alpha, T::zero(), T::one());

        other
    }

    #[inline]
    fn outside(self, mut other: Self) -> Self {
        for (src, dst) in zip_colors(self.color, &mut other.color) {
            *dst = src * (T::one() - &other.alpha);
        }

        other.alpha = clamp(self.alpha * (T::one() - other.alpha), T::zero(), T::one());

        other
    }

    #[inline]
    fn atop(self, mut other: Self) -> Self {
        for (src, dst) in zip_colors(self.color, &mut other.color) {
            *dst = src * &other.alpha + (T::one() - &self.alpha) * &*dst;
        }

        other.alpha = clamp(other.alpha, T::zero(), T::one());

        other
    }

    #[inline]
    fn xor(self, mut other: Self) -> Self {
        let two = || T::one() + T::one();

        for (src, dst) in zip_colors(self.color, &mut other.color) {
            *dst = src * (T::one() - &other.alpha) + (T::one() - &self.alpha) * &*dst;
        }

        other.alpha = clamp(
            self.alpha.clone() + &other.alpha - two() * self.alpha * other.alpha,
            T::zero(),
            T::one(),
        );

        other
    }

    #[inline]
    fn plus(self, mut other: Self) -> Self {
        for (src, dst) in zip_colors(self.color, &mut other.color) {
            *dst = src + &*dst;
        }

        other.alpha = clamp(self.alpha + other.alpha, T::zero(), T::one());

        other
    }
}

impl<C> Compose for Alpha<C, C::Scalar>
where
    C: Premultiply,
    PreAlpha<C>: Compose,
{
    #[inline]
    fn over(self, other: Self) -> Self {
        self.premultiply().over(other.premultiply()).unpremultiply()
    }

    #[inline]
    fn inside(self, other: Self) -> Self {
        self.premultiply()
            .inside(other.premultiply())
            .unpremultiply()
    }

    #[inline]
    fn outside(self, other: Self) -> Self {
        self.premultiply()
            .outside(other.premultiply())
            .unpremultiply()
    }

    #[inline]
    fn atop(self, other: Self) -> Self {
        self.premultiply().atop(other.premultiply()).unpremultiply()
    }

    #[inline]
    fn xor(self, other: Self) -> Self {
        self.premultiply().xor(other.premultiply()).unpremultiply()
    }

    #[inline]
    fn plus(self, other: Self) -> Self {
        self.premultiply().plus(other.premultiply()).unpremultiply()
    }
}

impl<C> Compose for C
where
    C: Premultiply,
    C::Scalar: Stimulus,
    PreAlpha<C>: Compose,
{
    #[inline]
    fn over(self, other: Self) -> Self {
        PreAlpha::new_opaque(self)
            .over(PreAlpha::new_opaque(other))
            .unpremultiply()
            .color
    }

    #[inline]
    fn inside(self, other: Self) -> Self {
        PreAlpha::new_opaque(self)
            .inside(PreAlpha::new_opaque(other))
            .unpremultiply()
            .color
    }

    #[inline]
    fn outside(self, other: Self) -> Self {
        PreAlpha::new_opaque(self)
            .outside(PreAlpha::new_opaque(other))
            .unpremultiply()
            .color
    }

    #[inline]
    fn atop(self, other: Self) -> Self {
        PreAlpha::new_opaque(self)
            .atop(PreAlpha::new_opaque(other))
            .unpremultiply()
            .color
    }

    #[inline]
    fn xor(self, other: Self) -> Self {
        PreAlpha::new_opaque(self)
            .xor(PreAlpha::new_opaque(other))
            .unpremultiply()
            .color
    }

    #[inline]
    fn plus(self, other: Self) -> Self {
        PreAlpha::new_opaque(self)
            .plus(PreAlpha::new_opaque(other))
            .unpremultiply()
            .color
    }
}
