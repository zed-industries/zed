//! De/Serialization of base58 encoded bytes
//!
//! This modules is only available when using the `base58` feature of the crate.
//!
//! Please check the documentation on the [`Base58`] type for details.

use crate::prelude::*;

/// Serialize bytes with base58
///
/// The type serializes a sequence of bytes as a base58 string.
/// It works on any type implementing `AsRef<[u8]>` for serialization and `TryFrom<Vec<u8>>` for deserialization.
///
/// The type allows customizing the character set.
/// The `ALPHABET` is a type implementing [`Alphabet`].
///
/// ```rust
/// # #[cfg(feature = "macros")]
/// # use serde::{Deserialize, Serialize};
/// # use serde_with::serde_as;
/// use serde_with::base58::{Base58, Flickr};
///
/// #[serde_as]
/// # #[derive(Debug, PartialEq, Eq)]
/// #[derive(Serialize, Deserialize)]
/// struct B58 {
///     // The default is the same as Standard character set
///     #[serde_as(as = "Base58")]
///     default: Vec<u8>,
///     // Change the character set
///     #[serde_as(as = "Base58<Flickr>")]
///     charset_flickr: Vec<u8>,
/// }
///
/// let b58 = B58 {
///     default: b"Hello World".to_vec(),
///     charset_flickr: b"Hello World".to_vec(),
/// };
/// let json = serde_json::json!({
///     "default": "JxF12TrwUP45BMd",
///     "charset_flickr": "iXf12sRWto45bmC",
/// });
///
/// // Test serialization and deserialization
/// assert_eq!(json, serde_json::to_value(&b58).unwrap());
/// assert_eq!(b58, serde_json::from_value(json).unwrap());
/// ```
pub struct Base58<ALPHABET: Alphabet = Standard>(PhantomData<ALPHABET>);

impl<T, ALPHABET> SerializeAs<T> for Base58<ALPHABET>
where
    T: AsRef<[u8]>,
    ALPHABET: Alphabet,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ::bs58::encode::EncodeBuilder::new(source, ALPHABET::CHARSET)
            .into_string()
            .serialize(serializer)
    }
}

impl<'de, T, ALPHABET> DeserializeAs<'de, T> for Base58<ALPHABET>
where
    T: TryFrom<Vec<u8>>,
    ALPHABET: Alphabet,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Helper<T, ALPHABET>(PhantomData<(T, ALPHABET)>);

        impl<T, ALPHABET> Visitor<'_> for Helper<T, ALPHABET>
        where
            T: TryFrom<Vec<u8>>,
            ALPHABET: Alphabet,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a base58 encoded string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                let bytes = ::bs58::decode::DecodeBuilder::new(value, ALPHABET::CHARSET)
                    .into_vec()
                    .map_err(DeError::custom)?;

                let length = bytes.len();
                bytes.try_into().map_err(|_e: T::Error| {
                    DeError::custom(format_args!(
                        "Can't convert a Byte Vector of length {length} to the output type."
                    ))
                })
            }
        }

        deserializer.deserialize_str(Helper::<T, ALPHABET>(PhantomData))
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Bitcoin {}
    impl Sealed for super::Monero {}
    impl Sealed for super::Ripple {}
    impl Sealed for super::Flickr {}
}

/// A base58 alphabet
pub trait Alphabet: sealed::Sealed {
    /// A specific alphabet.
    const CHARSET: &'static ::bs58::Alphabet;
}

/// The default alphabet used if none is given. Currently is the
/// [`Bitcoin`] alphabet.
pub type Standard = Bitcoin;

/// Bitcoin's alphabet as defined in their `Base58Check` encoding.
///
/// See <https://en.bitcoin.it/wiki/Base58Check_encoding#Base58_symbol_chart>
pub struct Bitcoin;
impl Alphabet for Bitcoin {
    const CHARSET: &'static bs58::Alphabet = ::bs58::Alphabet::BITCOIN;
}

/// Monero's alphabet as defined in this forum post.
///
/// See <https://forum.getmonero.org/4/academic-and-technical/221/creating-a-standard-for-physical-coins>
pub struct Monero;
impl Alphabet for Monero {
    const CHARSET: &'static bs58::Alphabet = ::bs58::Alphabet::MONERO;
}

/// Ripple's alphabet as defined in their wiki.
///
/// See <https://wiki.ripple.com/Encodings>
pub struct Ripple;
impl Alphabet for Ripple {
    const CHARSET: &'static bs58::Alphabet = ::bs58::Alphabet::RIPPLE;
}

/// Flickr's alphabet for creating short urls from photo ids.
///
/// See <https://www.flickr.com/groups/api/discuss/72157616713786392/>
pub struct Flickr;
impl Alphabet for Flickr {
    const CHARSET: &'static bs58::Alphabet = ::bs58::Alphabet::FLICKR;
}
