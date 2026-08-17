use crate::strings::{DecimalStr, InfinityStr, MinusSignStr, NanStr, PlusSignStr, SeparatorStr};
use crate::Grouping;

/// Trait that abstracts over [`CustomFormat`], [`Locale`], and `SystemLocale`.
///
/// [`CustomFormat`]: struct.CustomFormat.html
/// [`Locale`]: enum.Locale.html
pub trait Format {
    /// Returns the string representation of a decimal point.
    fn decimal(&self) -> DecimalStr<'_>;
    /// Returns the [`Grouping`] to use for separating digits. (see [`Grouping`])
    ///
    /// [`Grouping`]: enum.Grouping.html
    fn grouping(&self) -> Grouping;
    /// Returns the string representation of an infinity symbol.
    fn infinity(&self) -> InfinityStr<'_>;
    /// Returns the string representation of a minus sign.
    fn minus_sign(&self) -> MinusSignStr<'_>;
    /// Returns the string representation of NaN.
    fn nan(&self) -> NanStr<'_>;
    /// Returns the string representation of a plus sign.
    fn plus_sign(&self) -> PlusSignStr<'_>;
    /// Returns the string representation of a thousands separator.
    fn separator(&self) -> SeparatorStr<'_>;
}
