//! Types related to transparent colors.

#[doc(hidden)]
pub use palette_derive::WithAlpha;

use crate::{num::Zero, stimulus::Stimulus};

pub use self::alpha::*;

#[doc(no_inline)]
pub use crate::blend::PreAlpha; // Cross-link for visibility.

#[allow(clippy::module_inception)]
mod alpha;

/// A trait for color types that can have or be given transparency (alpha channel).
///
/// `WithAlpha` is an interface for adding, removing and setting the alpha
/// channel of a color type. The color type itself doesn't need to store the
/// transparency value as it can be transformed into or wrapped in a type that
/// has a representation of transparency. This would typically be done by
/// wrapping it in an [`Alpha`] instance.
///
/// # Deriving
/// The trait is trivial enough to be automatically derived. If the color type
/// has a field for transparency (an alpha channel), it has to be marked with
/// `#[palette(alpha)]` to be taken into account.
///
/// Derived without an internal alpha channel:
///
/// ```
/// use palette::WithAlpha;
///
/// #[derive(WithAlpha)]
/// struct CustomColor {
///     redness: f32,
///     glow: f32,
///     glitter: f32,
/// }
///
/// let color = CustomColor {
///     redness: 0.8,
///     glow: 2.5,
///     glitter: 1000.0
/// };
/// let transparent = color.with_alpha(0.3);
///
/// assert_eq!(transparent.alpha, 0.3);
/// ```
///
/// Derived with an internal alpha channel:
///
/// ```
/// use palette::WithAlpha;
///
/// #[derive(WithAlpha)]
/// struct CustomColor {
///     redness: f32,
///     glow: f32,
///     glitter: f32,
///
///     #[palette(alpha)]
///     alpha: u8,
/// }
///
/// let color = CustomColor {
///     redness: 0.8,
///     glow: 2.5,
///     glitter: 1000.0,
///     alpha: 255
/// };
/// let transparent = color.with_alpha(10);
///
/// assert_eq!(transparent.alpha, 10);
/// ```
pub trait WithAlpha<A>: Sized {
    /// The opaque color type, without any transparency.
    ///
    /// This is typically `Self`.
    type Color;

    /// The color type with transparency applied.
    ///
    /// This is typically `Alpha<Self::Color, A>`.
    type WithAlpha: WithAlpha<A, Color = Self::Color, WithAlpha = Self::WithAlpha>;

    /// Transforms the color into a transparent color with the provided
    /// alpha value. If `Self` already has a transparency, it is
    /// overwritten.
    ///
    /// ```
    /// use palette::{Srgb, WithAlpha};
    ///
    /// let color = Srgb::new(255u8, 0, 255);
    ///
    /// // This results in an `Alpha<Srgb<u8>, f32>`
    /// let transparent = color.with_alpha(0.3f32);
    /// assert_eq!(transparent.alpha, 0.3);
    ///
    /// // This changes the transparency to 0.8
    /// let transparent = transparent.with_alpha(0.8f32);
    /// assert_eq!(transparent.alpha, 0.8);
    /// ```
    #[must_use]
    fn with_alpha(self, alpha: A) -> Self::WithAlpha;

    /// Removes the transparency from the color. If `Self::Color` has
    /// an internal transparency field, that field will be set to
    /// `A::max_intensity()` to make it opaque.
    ///
    /// ```
    /// use palette::{Srgba, Srgb, WithAlpha};
    ///
    /// let transparent = Srgba::new(255u8, 0, 255, 10);
    ///
    /// // This unwraps the color information from the `Alpha` wrapper
    /// let color = transparent.without_alpha();
    /// assert_eq!(transparent.color, color);
    /// ```
    #[must_use]
    fn without_alpha(self) -> Self::Color;

    /// Splits the color into separate color and transparency values.
    ///
    /// A color without any transparency field will return
    /// `A::max_intensity()` instead. If `Self::Color` has an internal
    /// transparency field, that field will be set to
    /// `A::max_intensity()` to make it opaque.
    ///
    /// ```
    /// use palette::{Srgba, Srgb, WithAlpha};
    ///
    /// let transparent = Srgba::new(255u8, 0, 255, 10);
    ///
    /// // This unwraps both the color and alpha from the `Alpha` wrapper
    /// let (color, alpha) = transparent.split();
    /// assert_eq!(transparent.color, color);
    /// assert_eq!(transparent.alpha, alpha);
    /// ```
    #[must_use]
    fn split(self) -> (Self::Color, A);

    /// Transforms the color into a fully opaque color with a transparency
    /// field. If `Self` already has a transparency, it is overwritten.
    ///
    /// ```
    /// use palette::{Srgb, Srgba, WithAlpha};
    ///
    /// let color = Srgb::new(255u8, 0, 255);
    ///
    /// let opaque: Srgba<u8> = color.opaque();
    /// assert_eq!(opaque.alpha, 255);
    /// ```
    #[must_use]
    #[inline]
    fn opaque(self) -> Self::WithAlpha
    where
        A: Stimulus,
    {
        self.with_alpha(A::max_intensity())
    }

    /// Transforms the color into a fully transparent color. If `Self`
    /// already has a transparency, it is overwritten.
    ///
    /// ```
    /// use palette::{Srgb, Srgba, WithAlpha};
    ///
    /// let color = Srgb::new(255u8, 0, 255);
    ///
    /// let transparent: Srgba<u8> = color.transparent();
    /// assert_eq!(transparent.alpha, 0);
    /// ```
    #[must_use]
    #[inline]
    fn transparent(self) -> Self::WithAlpha
    where
        A: Zero,
    {
        self.with_alpha(A::zero())
    }
}
