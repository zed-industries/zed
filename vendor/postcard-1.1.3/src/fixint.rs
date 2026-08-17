//! # Fixed Size Integers
//!
//! In some cases, the use of variably length encoded data may not be
//! preferable. These modules, for use with `#[serde(with = ...)]`
//! "opt out" of variable length encoding.
//!
//! Support explicitly not provided for `usize` or `isize`, as
//! these types would not be portable between systems of different
//! pointer widths.
//!
//! Although all data in Postcard is typically encoded in little-endian
//! order, these modules provide a choice to the user to encode the data
//! in either little or big endian form, which may be useful for zero-copy
//! applications.

use serde::{Deserialize, Serialize, Serializer};

/// Use with the `#[serde(with = "postcard::fixint::le")]` field attribute.
///
/// Disables varint serialization/deserialization for the specified integer
/// field. The integer will always be serialized in the same way as a fixed
/// size array, in **Little Endian** order on the wire.
///
/// ```rust
/// # use serde::Serialize;
/// #[derive(Serialize)]
/// pub struct DefinitelyLittleEndian {
///     #[serde(with = "postcard::fixint::le")]
///     x: u16,
/// }
/// ```
pub mod le {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::LE;

    /// Serialize the integer value as a little-endian fixed-size array.
    pub fn serialize<S, T>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Copy,
        LE<T>: Serialize,
    {
        LE(*val).serialize(serializer)
    }

    /// Deserialize the integer value from a little-endian fixed-size array.
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        LE<T>: Deserialize<'de>,
    {
        LE::<T>::deserialize(deserializer).map(|x| x.0)
    }
}

/// Disables varint serialization/deserialization for the specified integer field.
///
/// Use with the `#[serde(with = "postcard::fixint::be")]` field attribute.
/// The integer will always be serialized in the same way as a fixed
/// size array, in **Big Endian** order on the wire.
///
/// ```rust
/// # use serde::Serialize;
/// #[derive(Serialize)]
/// pub struct DefinitelyBigEndian {
///     #[serde(with = "postcard::fixint::be")]
///     x: u16,
/// }
/// ```
pub mod be {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::BE;

    /// Serialize the integer value as a big-endian fixed-size array.
    pub fn serialize<S, T>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Copy,
        BE<T>: Serialize,
    {
        BE(*val).serialize(serializer)
    }

    /// Deserialize the integer value from a big-endian fixed-size array.
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        BE<T>: Deserialize<'de>,
    {
        BE::<T>::deserialize(deserializer).map(|x| x.0)
    }
}

#[doc(hidden)]
pub struct LE<T>(T);

#[doc(hidden)]
pub struct BE<T>(T);

macro_rules! impl_fixint {
    ($( $int:ty ),*) => {
        $(
            impl Serialize for LE<$int> {
                #[inline]
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    self.0.to_le_bytes().serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for LE<$int> {
                #[inline]
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    <_ as Deserialize>::deserialize(deserializer)
                        .map(<$int>::from_le_bytes)
                        .map(Self)
                }
            }

            impl Serialize for BE<$int> {
                #[inline]
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    self.0.to_be_bytes().serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for BE<$int> {
                #[inline]
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    <_ as Deserialize>::deserialize(deserializer)
                        .map(<$int>::from_be_bytes)
                        .map(Self)
                }
            }
        )*
    };
}

impl_fixint![i16, i32, i64, i128, u16, u32, u64, u128];

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_little_endian() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        pub struct DefinitelyLE {
            #[serde(with = "crate::fixint::le")]
            x: u16,
        }

        let input = DefinitelyLE { x: 0xABCD };
        let mut buf = [0; 32];
        let serialized = crate::to_slice(&input, &mut buf).unwrap();
        assert_eq!(serialized, &[0xCD, 0xAB]);

        let deserialized: DefinitelyLE = crate::from_bytes(serialized).unwrap();
        assert_eq!(deserialized, input);
    }

    #[test]
    fn test_big_endian() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        pub struct DefinitelyBE {
            #[serde(with = "crate::fixint::be")]
            x: u16,
        }

        let input = DefinitelyBE { x: 0xABCD };
        let mut buf = [0; 32];
        let serialized = crate::to_slice(&input, &mut buf).unwrap();
        assert_eq!(serialized, &[0xAB, 0xCD]);

        let deserialized: DefinitelyBE = crate::from_bytes(serialized).unwrap();
        assert_eq!(deserialized, input);
    }
}
