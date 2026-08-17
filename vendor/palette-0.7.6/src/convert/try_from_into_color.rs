use core::fmt;

use crate::IsWithinBounds;

use super::FromColorUnclamped;

/// The error type for a color conversion that converted a color into a color
/// with invalid values.
#[derive(Debug)]
pub struct OutOfBounds<T> {
    color: T,
}

impl<T> OutOfBounds<T> {
    /// Create a new error wrapping a color
    #[inline]
    fn new(color: T) -> Self {
        OutOfBounds { color }
    }

    /// Consume this error and return the wrapped color
    #[inline]
    pub fn color(self) -> T {
        self.color
    }
}

#[cfg(feature = "std")]
impl<T: fmt::Debug> std::error::Error for OutOfBounds<T> {
    fn description(&self) -> &str {
        "color conversion is out of bounds"
    }
}

impl<T> fmt::Display for OutOfBounds<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "color conversion is out of bounds")
    }
}

/// A trait for fallible conversion of one color from another.
///
/// `U: TryFromColor<T>` is implemented for every type `U: FromColorUnclamped<T> + Clamp`.
///
/// See [`FromColor`](crate::convert::FromColor) for a lossy version of this trait.
/// See [`FromColorUnclamped`](crate::convert::FromColorUnclamped) for a lossless version.
///
/// See the [`convert`](crate::convert) module for how to implement `FromColorUnclamped` for
/// custom colors.
pub trait TryFromColor<T>: Sized {
    /// Convert from T, returning ok if the color is inside of its defined
    /// range, otherwise an `OutOfBounds` error is returned which contains
    /// the unclamped color.
    ///
    ///```
    /// use palette::convert::TryFromColor;
    /// use palette::{Hsl, Srgb};
    ///
    /// let rgb = match Srgb::try_from_color(Hsl::new(150.0, 1.0, 1.1)) {
    ///     Ok(color) => color,
    ///     Err(err) => {
    ///         println!("Color is out of bounds");
    ///         err.color()
    ///     }
    /// };
    /// ```
    fn try_from_color(t: T) -> Result<Self, OutOfBounds<Self>>;
}

impl<T, U> TryFromColor<T> for U
where
    U: FromColorUnclamped<T> + IsWithinBounds<Mask = bool>,
{
    #[inline]
    fn try_from_color(t: T) -> Result<Self, OutOfBounds<Self>> {
        let this = Self::from_color_unclamped(t);
        if this.is_within_bounds() {
            Ok(this)
        } else {
            Err(OutOfBounds::new(this))
        }
    }
}

/// A trait for fallible conversion of a color into another.
///
/// `U: TryIntoColor<T>` is implemented for every type `T: TryFromColor<U>`.
///
/// See [`TryFromColor`](crate::convert::TryFromColor) for more details.
pub trait TryIntoColor<T>: Sized {
    /// Convert into T, returning ok if the color is inside of its defined
    /// range, otherwise an `OutOfBounds` error is returned which contains
    /// the unclamped color.
    ///
    ///```
    /// use palette::convert::TryIntoColor;
    /// use palette::{Hsl, Srgb};
    ///
    /// let rgb: Srgb = match Hsl::new(150.0, 1.0, 1.1).try_into_color() {
    ///     Ok(color) => color,
    ///     Err(err) => {
    ///         println!("Color is out of bounds");
    ///         err.color()
    ///     }
    /// };
    /// ```
    fn try_into_color(self) -> Result<T, OutOfBounds<T>>;
}

impl<T, U> TryIntoColor<U> for T
where
    U: TryFromColor<T>,
{
    #[inline]
    fn try_into_color(self) -> Result<U, OutOfBounds<U>> {
        U::try_from_color(self)
    }
}
