//! Color blending and blending equations.
//!
//! Palette offers both OpenGL style blending equations, as well as most of the
//! SVG composition operators (also common in photo manipulation software). The
//! composition operators are all implemented in the [`Compose`] and [`Blend`]
//! traits, and ready to use with any appropriate color type:
//!
//! ```
//! use palette::{blend::Blend, LinSrgba};
//!
//! let a = LinSrgba::new(0.2, 0.5, 0.1, 0.8);
//! let b = LinSrgba::new(0.6, 0.3, 0.5, 0.1);
//! let c = a.overlay(b);
//! ```
//!
//! Blending equations can be defined using the [`Equations`] type, which is
//! then passed to the `blend` function, from the `Blend` trait:
//!
//! ```
//! use palette::LinSrgba;
//! use palette::blend::{BlendWith, Equations, Parameter};
//!
//! let blend_mode = Equations::from_parameters(
//!    Parameter::SourceAlpha,
//!    Parameter::OneMinusSourceAlpha
//! );
//!
//! let a = LinSrgba::new(0.2, 0.5, 0.1, 0.8);
//! let b = LinSrgba::new(0.6, 0.3, 0.5, 0.1);
//! let c = a.blend_with(b, blend_mode);
//! ```
//!
//! Note that blending will use [premultiplied alpha](crate::blend::PreAlpha),
//! which may result in loss of some color information in some cases. One such
//! case is that a completely transparent resultant color will become black.

use crate::{
    cast::{self, ArrayCast},
    clamp,
    num::{Arithmetics, Clamp, One, Real, Zero},
    stimulus::Stimulus,
};

pub use self::{
    blend::Blend,
    blend_with::BlendWith,
    compose::Compose,
    equations::{Equation, Equations, Parameter, Parameters},
    pre_alpha::PreAlpha,
};

#[allow(clippy::module_inception)]
mod blend;
mod blend_with;
mod compose;
mod equations;
mod pre_alpha;

#[cfg(test)]
mod test;

/// A trait for custom blend functions.
pub trait BlendFunction<C>
where
    C: Premultiply,
{
    /// Apply this blend function to a pair of colors.
    #[must_use]
    fn apply_to(self, source: PreAlpha<C>, destination: PreAlpha<C>) -> PreAlpha<C>;
}

impl<C, F> BlendFunction<C> for F
where
    C: Premultiply,
    F: FnOnce(PreAlpha<C>, PreAlpha<C>) -> PreAlpha<C>,
{
    #[inline]
    fn apply_to(self, source: PreAlpha<C>, destination: PreAlpha<C>) -> PreAlpha<C> {
        (self)(source, destination)
    }
}

/// Alpha masking and unmasking.
pub trait Premultiply: Sized {
    /// The color's component type.
    type Scalar: Real + Stimulus;

    /// Alpha mask the color.
    ///
    /// This is done by multiplying the color's component by `alpha`.
    #[must_use]
    fn premultiply(self, alpha: Self::Scalar) -> PreAlpha<Self>;

    /// Alpha unmask the color, resulting in a color and transparency pair.
    ///
    /// This is done by dividing the masked color's component by `alpha`, or
    /// returning a black color if `alpha` is `0`.
    #[must_use]
    fn unpremultiply(premultiplied: PreAlpha<Self>) -> (Self, Self::Scalar);
}

fn blend_alpha<T>(src: T, dst: T) -> T
where
    T: Zero + One + Arithmetics + Clamp + Clone,
{
    clamp(src.clone() + &dst - src * dst, T::zero(), T::one())
}

fn zip_colors<'a, C, T, const N: usize>(
    src: C,
    dst: &'a mut C,
) -> impl Iterator<Item = (T, &'a mut T)>
where
    C: ArrayCast<Array = [T; N]>,
    T: 'a,
{
    IntoIterator::into_iter(cast::into_array(src)).zip(cast::into_array_mut(dst))
}
