use crate::Clamp;

#[cfg(feature = "alloc")]
use crate::cast::{self, ArrayCast};

use super::FromColorUnclamped;

///A trait for converting one color from another, in a possibly lossy way.
///
/// `U: FromColor<T>` is implemented for every type `U: FromColorUnclamped<T> +
/// Clamp`, as well as for `Vec<T>` and `Box<[T]>` where `T` and `U` have the
/// same memory layout.
///
/// See [`FromColorUnclamped`](crate::convert::FromColorUnclamped) for a
/// lossless version of this trait. See
/// [`TryFromColor`](crate::convert::TryFromColor) for a trait that gives an
/// error when the result is out of bounds.
///
/// # The Difference Between FromColor and From
///
/// The conversion traits, including `FromColor`, were added to gain even more
/// flexibility than what `From` and the other standard library traits can give.
/// There are a few subtle, but important, differences in their semantics:
///
/// * `FromColor` and `IntoColor` are allowed to be lossy, meaning converting `A
///   -> B -> A` may result in a different value than the original. This applies
///   to `A -> A` as well.
/// * `From<Self>` and `Into<Self>` are blanket implemented, while
///   `FromColor<Self>` and `IntoColor<Self>` have to be manually implemented.
///   This allows additional flexibility, such as allowing implementing
///   `FromColor<Rgb<S2, T>> for Rgb<S1, T>`.
/// * Implementing `FromColorUnclamped`,
///   [`IsWithinBounds`](crate::IsWithinBounds) and [`Clamp`] is enough to get
///   all the other conversion traits, while `From` and `Into` would not be
///   possible to blanket implement in the same way. This also reduces the work
///   that needs to be done by macros.
///
/// See the [`convert`](crate::convert) module for how to implement
/// `FromColorUnclamped` for custom colors.
pub trait FromColor<T>: Sized {
    /// Convert from T with values clamped to the color defined bounds.
    ///
    /// ```
    /// use palette::{IsWithinBounds, FromColor, Lch, Srgb};
    ///
    /// let rgb = Srgb::from_color(Lch::new(50.0f32, 100.0, -175.0));
    /// assert!(rgb.is_within_bounds());
    /// ```
    #[must_use]
    fn from_color(t: T) -> Self;
}

impl<T, U> FromColor<T> for U
where
    U: FromColorUnclamped<T> + Clamp,
{
    #[inline]
    fn from_color(t: T) -> Self {
        Self::from_color_unclamped(t).clamp()
    }
}

#[cfg(feature = "alloc")]
impl<T, U> FromColor<alloc::vec::Vec<T>> for alloc::vec::Vec<U>
where
    T: ArrayCast,
    U: ArrayCast<Array = T::Array> + FromColor<T>,
{
    /// Convert all colors in place, without reallocating.
    ///
    /// ```
    /// use palette::{convert::FromColor, SaturateAssign, Srgb, Lch};
    ///
    /// let srgb = vec![Srgb::new(0.8f32, 1.0, 0.2), Srgb::new(0.9, 0.1, 0.3)];
    /// let mut lch = Vec::<Lch>::from_color(srgb);
    ///
    /// lch.saturate_assign(0.1);
    ///
    /// let srgb = Vec::<Srgb>::from_color(lch);
    /// ```
    #[inline]
    fn from_color(color: alloc::vec::Vec<T>) -> Self {
        cast::map_vec_in_place(color, U::from_color)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> FromColor<alloc::boxed::Box<[T]>> for alloc::boxed::Box<[U]>
where
    T: ArrayCast,
    U: ArrayCast<Array = T::Array> + FromColor<T>,
{
    /// Convert all colors in place, without reallocating.
    ///
    /// ```
    /// use palette::{convert::FromColor, SaturateAssign, Srgb, Lch};
    ///
    /// let srgb = vec![Srgb::new(0.8f32, 1.0, 0.2), Srgb::new(0.9, 0.1, 0.3)].into_boxed_slice();
    /// let mut lch = Box::<[Lch]>::from_color(srgb);
    ///
    /// lch.saturate_assign(0.1);
    ///
    /// let srgb = Box::<[Srgb]>::from_color(lch);
    /// ```
    #[inline]
    fn from_color(color: alloc::boxed::Box<[T]>) -> Self {
        cast::map_slice_box_in_place(color, U::from_color)
    }
}

/// A trait for converting a color into another, in a possibly lossy way.
///
/// `U: IntoColor<T>` is implemented for every type `T: FromColor<U>`.
///
/// See [`FromColor`](crate::convert::FromColor) for more details.
pub trait IntoColor<T>: Sized {
    /// Convert into T with values clamped to the color defined bounds
    ///
    /// ```
    /// use palette::{IsWithinBounds, IntoColor, Lch, Srgb};
    ///
    /// let rgb: Srgb = Lch::new(50.0, 100.0, -175.0).into_color();
    /// assert!(rgb.is_within_bounds());
    /// ```
    #[must_use]
    fn into_color(self) -> T;
}

impl<T, U> IntoColor<U> for T
where
    U: FromColor<T>,
{
    #[inline]
    fn into_color(self) -> U {
        U::from_color(self)
    }
}
