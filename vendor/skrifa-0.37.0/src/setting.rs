//! Definitions for specifying variations and typographic features.

use super::Tag;
use core::str::FromStr;

/// Setting defined by a selector tag and an associated value.
///
/// This type is a generic container for properties that can be activated
/// or defined by a `(Tag, T)` pair where the tag selects the target
/// setting and the generic value of type `T` specifies the value for that
/// setting.
///
/// ## Usage
/// Current usage is for specifying variation axis settings (similar to the
/// CSS property [font-variation-settings](https://developer.mozilla.org/en-US/docs/Web/CSS/font-variation-settings)).
/// See [`VariationSetting`].
///
/// In the future, this will likely also be used for specifying feature settings
/// (analogous to the CSS property [font-feature-settings](https://developer.mozilla.org/en-US/docs/Web/CSS/font-feature-settings))
/// for selecting OpenType [features](https://learn.microsoft.com/en-us/typography/opentype/spec/featuretags).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Setting<T> {
    /// Tag that specifies the target setting.
    pub selector: Tag,
    /// The desired value for the setting.
    pub value: T,
}

impl<T> Setting<T> {
    /// Creates a new setting from the given selector tag and its associated
    /// value.
    pub fn new(selector: Tag, value: T) -> Self {
        Self { selector, value }
    }
}

// This is provided so that &[VariationSetting] can be passed to the
// variation_settings() method of ScalerBuilder.
impl<T: Copy> From<&'_ Setting<T>> for Setting<T> {
    fn from(value: &'_ Setting<T>) -> Self {
        *value
    }
}

impl<T> From<(Tag, T)> for Setting<T> {
    fn from(s: (Tag, T)) -> Self {
        Self {
            selector: s.0,
            value: s.1,
        }
    }
}

impl<T: Copy> From<&(Tag, T)> for Setting<T> {
    fn from(s: &(Tag, T)) -> Self {
        Self {
            selector: s.0,
            value: s.1,
        }
    }
}

impl<T> From<(&str, T)> for Setting<T> {
    fn from(s: (&str, T)) -> Self {
        Self {
            selector: Tag::from_str(s.0).unwrap_or_default(),
            value: s.1,
        }
    }
}

impl<T: Copy> From<&(&str, T)> for Setting<T> {
    fn from(s: &(&str, T)) -> Self {
        Self {
            selector: Tag::from_str(s.0).unwrap_or_default(),
            value: s.1,
        }
    }
}

/// Type for specifying a variation axis setting in user coordinates.
///
/// The `selector` field should contain a tag that corresponds to a
/// variation axis while the `value` field specifies the desired position
/// on the axis in user coordinates (i.e. within the range defined by
/// the minimum and maximum values of the axis).
///
/// # Example
/// ```
/// use skrifa::{Tag, setting::VariationSetting};
///
/// // For convenience, a conversion from (&str, f32) is provided.
/// let slightly_bolder: VariationSetting = ("wght", 720.0).into();
///
/// assert_eq!(slightly_bolder, VariationSetting::new(Tag::new(b"wght"), 720.0));
/// ```
pub type VariationSetting = Setting<f32>;
