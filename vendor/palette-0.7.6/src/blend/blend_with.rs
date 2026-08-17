use crate::{stimulus::Stimulus, Alpha};

use super::{BlendFunction, PreAlpha, Premultiply};

/// Blending with a custom blend function.
///
/// This is a convenience trait that makes it possible to use [`BlendFunction`]
/// via a method on the source color. This makes custom blending more similar to
/// how the [`Compose`][super::Compose] and [`Blend`][super::Blend] are used,
/// including automatic pre-multiplication.
pub trait BlendWith {
    /// The base color type of `Self`.
    type Color: Premultiply;

    /// Blend self, as the source color, with `destination`, using
    /// `blend_function`. Anything that implements [`BlendFunction`] is
    /// acceptable, including functions and closures.
    ///
    /// ```
    /// use palette::{LinSrgb, LinSrgba};
    /// use palette::blend::{BlendWith, PreAlpha};
    ///
    /// type PreRgba = PreAlpha<LinSrgb<f32>>;
    ///
    /// fn blend_mode(a: PreRgba, b: PreRgba) -> PreRgba {
    ///    PreAlpha {
    ///        color: LinSrgb::new(a.red * b.green, a.green * b.blue, a.blue * b.red),
    ///        alpha: a.alpha * b.alpha,
    ///    }
    /// }
    ///
    /// let a = LinSrgba::new(0.2, 0.5, 0.1, 0.8);
    /// let b = LinSrgba::new(0.6, 0.3, 0.5, 0.1);
    /// let c = a.blend_with(b, blend_mode);
    /// ```
    #[must_use]
    fn blend_with<F>(self, destination: Self, blend_function: F) -> Self
    where
        F: BlendFunction<Self::Color>;
}

impl<C> BlendWith for PreAlpha<C>
where
    C: Premultiply,
{
    type Color = C;

    #[inline]
    fn blend_with<F>(self, other: Self, blend_function: F) -> Self
    where
        F: BlendFunction<Self::Color>,
    {
        blend_function.apply_to(self, other)
    }
}

impl<C> BlendWith for Alpha<C, C::Scalar>
where
    C: Premultiply,
{
    type Color = C;

    fn blend_with<F>(self, destination: Self, blend_function: F) -> Self
    where
        F: crate::blend::BlendFunction<Self::Color>,
    {
        self.premultiply()
            .blend_with(destination.premultiply(), blend_function)
            .unpremultiply()
    }
}

impl<C> BlendWith for C
where
    C: Premultiply,
    C::Scalar: Stimulus,
{
    type Color = C;

    fn blend_with<F>(self, other: Self, blend_function: F) -> Self
    where
        F: BlendFunction<Self::Color>,
    {
        PreAlpha::new_opaque(self)
            .blend_with(PreAlpha::new_opaque(other), blend_function)
            .unpremultiply()
            .color
    }
}
