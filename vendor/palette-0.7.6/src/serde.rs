//! Utilities for serializing and deserializing with `serde`.
//!
//! These modules and functions can be combined with `serde`'s [field
//! attributes](https://serde.rs/field-attrs.html) to better control how to
//! serialize and deserialize colors. See each item's examples for more details.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    blend::{PreAlpha, Premultiply},
    cast::{self, ArrayCast, UintCast},
    stimulus::Stimulus,
    Alpha,
};

pub(crate) use self::{alpha_deserializer::AlphaDeserializer, alpha_serializer::AlphaSerializer};

mod alpha_deserializer;
mod alpha_serializer;

/// Combines [`serialize_as_array`] and [`deserialize_as_array`] as a module for `#[serde(with = "...")]`.
///
/// ```
/// use serde::{Serialize, Deserialize};
/// use palette::{Srgb, Srgba};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct MyColors {
///     #[serde(with = "palette::serde::as_array")]
///     opaque: Srgb,
///     #[serde(with = "palette::serde::as_array")]
///     transparent: Srgba,
/// }
///
/// let my_colors = MyColors {
///     opaque: Srgb::new(0.6, 0.8, 0.3),
///     transparent: Srgba::new(0.6, 0.8, 0.3, 0.5),
/// };
///
/// let json = serde_json::to_string(&my_colors).unwrap();
///
/// assert_eq!(
///     json,
///     r#"{"opaque":[0.6,0.8,0.3],"transparent":[0.6,0.8,0.3,0.5]}"#
/// );
///
/// assert_eq!(
///     serde_json::from_str::<MyColors>(&json).unwrap(),
///     my_colors
/// );
/// ```
pub mod as_array {
    pub use super::deserialize_as_array as deserialize;
    pub use super::serialize_as_array as serialize;
}

/// Serialize the value as an array of its components.
///
/// ```
/// use serde::Serialize;
/// use palette::{Srgb, Srgba};
///
/// #[derive(Serialize)]
/// struct MyColors {
///     #[serde(serialize_with = "palette::serde::serialize_as_array")]
///     opaque: Srgb,
///     #[serde(serialize_with = "palette::serde::serialize_as_array")]
///     transparent: Srgba,
/// }
///
/// let my_colors = MyColors {
///     opaque: Srgb::new(0.6, 0.8, 0.3),
///     transparent: Srgba::new(0.6, 0.8, 0.3, 0.5),
/// };
///
/// assert_eq!(
///     serde_json::to_string(&my_colors).unwrap(),
///     r#"{"opaque":[0.6,0.8,0.3],"transparent":[0.6,0.8,0.3,0.5]}"#
/// );
/// ```
pub fn serialize_as_array<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ArrayCast,
    T::Array: Serialize,
    S: Serializer,
{
    cast::into_array_ref(value).serialize(serializer)
}

/// Deserialize a value from an array of its components.
///
/// ```
/// use serde::Deserialize;
/// use palette::{Srgb, Srgba};
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct MyColors {
///     #[serde(deserialize_with = "palette::serde::deserialize_as_array")]
///     opaque: Srgb,
///     #[serde(deserialize_with = "palette::serde::deserialize_as_array")]
///     transparent: Srgba,
/// }
///
/// let my_colors = MyColors {
///     opaque: Srgb::new(0.6, 0.8, 0.3),
///     transparent: Srgba::new(0.6, 0.8, 0.3, 0.5),
/// };
///
/// let json = r#"{"opaque":[0.6,0.8,0.3],"transparent":[0.6,0.8,0.3,0.5]}"#;
/// assert_eq!(
///     serde_json::from_str::<MyColors>(json).unwrap(),
///     my_colors
/// );
/// ```
pub fn deserialize_as_array<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: ArrayCast,
    T::Array: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(cast::from_array(T::Array::deserialize(deserializer)?))
}

/// Combines [`serialize_as_uint`] and [`deserialize_as_uint`] as a module for `#[serde(with = "...")]`.
///
/// ```
/// use serde::{Serialize, Deserialize};
/// use palette::{Srgb, Srgba, rgb::{PackedArgb, PackedRgba}};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct MyColors {
///     #[serde(with = "palette::serde::as_uint")]
///     argb: PackedArgb,
///     #[serde(with = "palette::serde::as_uint")]
///     rgba: PackedRgba,
/// }
///
/// let my_colors = MyColors {
///     argb: Srgb::new(0x17, 0xC6, 0x4C).into(),
///     rgba: Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
/// };
///
/// let json = serde_json::to_string(&my_colors).unwrap();
///
/// assert_eq!(
///     json,
///     r#"{"argb":4279748172,"rgba":398871807}"#
/// );
///
/// assert_eq!(
///     serde_json::from_str::<MyColors>(&json).unwrap(),
///     my_colors
/// );
/// ```
pub mod as_uint {
    pub use super::deserialize_as_uint as deserialize;
    pub use super::serialize_as_uint as serialize;
}

/// Serialize the value as an unsigned integer.
///
/// ```
/// use serde::Serialize;
/// use palette::{Srgb, Srgba, rgb::{PackedArgb, PackedRgba}};
///
/// #[derive(Serialize)]
/// struct MyColors {
///     #[serde(serialize_with = "palette::serde::serialize_as_uint")]
///     argb: PackedArgb,
///     #[serde(serialize_with = "palette::serde::serialize_as_uint")]
///     rgba: PackedRgba,
/// }
///
/// let my_colors = MyColors {
///     argb: Srgb::new(0x17, 0xC6, 0x4C).into(),
///     rgba: Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
/// };
///
/// assert_eq!(
///     serde_json::to_string(&my_colors).unwrap(),
///     r#"{"argb":4279748172,"rgba":398871807}"#
/// );
/// ```
pub fn serialize_as_uint<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: UintCast,
    T::Uint: Serialize,
    S: Serializer,
{
    cast::into_uint_ref(value).serialize(serializer)
}

/// Deserialize a value from an unsigned integer.
///
/// ```
/// use serde::Deserialize;
/// use palette::{Srgb, Srgba, rgb::{PackedArgb, PackedRgba}};
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct MyColors {
///     #[serde(deserialize_with = "palette::serde::deserialize_as_uint")]
///     argb: PackedArgb,
///     #[serde(deserialize_with = "palette::serde::deserialize_as_uint")]
///     rgba: PackedRgba,
/// }
///
/// let my_colors = MyColors {
///     argb: Srgb::new(0x17, 0xC6, 0x4C).into(),
///     rgba: Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
/// };
///
/// let json = r#"{"argb":4279748172,"rgba":398871807}"#;
/// assert_eq!(
///     serde_json::from_str::<MyColors>(json).unwrap(),
///     my_colors
/// );
/// ```
pub fn deserialize_as_uint<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: UintCast,
    T::Uint: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(cast::from_uint(T::Uint::deserialize(deserializer)?))
}

/// Deserialize a transparent color without requiring the alpha to be specified.
///
/// A color with missing alpha will be interpreted as fully opaque.
///
/// ```
/// use serde::Deserialize;
/// use palette::Srgba;
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct MyColors {
///     #[serde(deserialize_with = "palette::serde::deserialize_with_optional_alpha")]
///     opaque: Srgba,
///     #[serde(deserialize_with = "palette::serde::deserialize_with_optional_alpha")]
///     transparent: Srgba,
/// }
///
/// let my_colors = MyColors {
///     opaque: Srgba::new(0.6, 0.8, 0.3, 1.0),
///     transparent: Srgba::new(0.6, 0.8, 0.3, 0.5),
/// };
///
/// let json = r#"{
///     "opaque":{"red":0.6,"green":0.8,"blue":0.3},
///     "transparent":{"red":0.6,"green":0.8,"blue":0.3,"alpha":0.5}
/// }"#;
/// assert_eq!(
///     serde_json::from_str::<MyColors>(json).unwrap(),
///     my_colors
/// );
/// ```
pub fn deserialize_with_optional_alpha<'de, T, A, D>(
    deserializer: D,
) -> Result<Alpha<T, A>, D::Error>
where
    T: Deserialize<'de>,
    A: Stimulus + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let mut alpha: Option<A> = None;

    let color = T::deserialize(crate::serde::AlphaDeserializer {
        inner: deserializer,
        alpha: &mut alpha,
    })?;

    Ok(Alpha {
        color,
        alpha: alpha.unwrap_or_else(A::max_intensity),
    })
}

/// Deserialize a premultiplied transparent color without requiring the alpha to be specified.
///
/// A color with missing alpha will be interpreted as fully opaque.
///
/// ```
/// use serde::Deserialize;
/// use palette::{LinSrgba, LinSrgb, blend::PreAlpha};
///
/// type PreRgba = PreAlpha<LinSrgb<f32>>;
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct MyColors {
///     #[serde(deserialize_with = "palette::serde::deserialize_with_optional_pre_alpha")]
///     opaque: PreRgba,
///     #[serde(deserialize_with = "palette::serde::deserialize_with_optional_pre_alpha")]
///     transparent: PreRgba,
/// }
///
/// let my_colors = MyColors {
///     opaque: LinSrgba::new(0.6, 0.8, 0.3, 1.0).into(),
///     transparent: LinSrgba::new(0.6, 0.8, 0.3, 0.5).into(),
/// };
///
/// let json = r#"{
///     "opaque":{"red":0.6,"green":0.8,"blue":0.3},
///     "transparent":{"red":0.3,"green":0.4,"blue":0.15,"alpha":0.5}
/// }"#;
/// assert_eq!(
///     serde_json::from_str::<MyColors>(json).unwrap(),
///     my_colors
/// );
/// ```
pub fn deserialize_with_optional_pre_alpha<'de, T, D>(
    deserializer: D,
) -> Result<PreAlpha<T>, D::Error>
where
    T: Premultiply + Deserialize<'de>,
    T::Scalar: Stimulus + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let mut alpha: Option<T::Scalar> = None;

    let color = T::deserialize(crate::serde::AlphaDeserializer {
        inner: deserializer,
        alpha: &mut alpha,
    })?;

    Ok(PreAlpha {
        color,
        alpha: alpha.unwrap_or_else(T::Scalar::max_intensity),
    })
}
