use crate::{ByteArray, Bytes};
use core::fmt;
use core::marker::PhantomData;
use serde::de::{Error, Visitor};
use serde::Deserializer;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::ByteBuf;

#[cfg(any(feature = "std", feature = "alloc"))]
use core::cmp;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(all(feature = "std", not(feature = "alloc")))]
use std::borrow::Cow;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(any(feature = "std", feature = "alloc"))]
use serde::de::SeqAccess;

/// Types that can be deserialized via `#[serde(with = "serde_bytes")]`.
pub trait Deserialize<'de>: Sized {
    #[allow(missing_docs)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a [u8] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // serde::Deserialize for &[u8] is already optimized, so simply forward to that.
        serde::Deserialize::deserialize(deserializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for Vec<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(ByteBuf::into_vec)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // serde::Deserialize for &[u8] is already optimized, so simply forward to that.
        serde::Deserialize::deserialize(deserializer).map(Bytes::new)
    }
}

impl<'de, const N: usize> Deserialize<'de> for [u8; N] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: ByteArray<N> = serde::Deserialize::deserialize(deserializer)?;
        Ok(*arr)
    }
}

impl<'de, const N: usize> Deserialize<'de> for &'de [u8; N] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: &ByteArray<N> = serde::Deserialize::deserialize(deserializer)?;
        Ok(arr)
    }
}

impl<'de, const N: usize> Deserialize<'de> for ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Via the serde::Deserialize impl for ByteArray.
        serde::Deserialize::deserialize(deserializer)
    }
}

impl<'de: 'a, 'a, const N: usize> Deserialize<'de> for &'a ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Via the serde::Deserialize impl for &ByteArray.
        serde::Deserialize::deserialize(deserializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for ByteBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Via the serde::Deserialize impl for ByteBuf.
        serde::Deserialize::deserialize(deserializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, [u8]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CowVisitor;

        impl<'de> Visitor<'de> for CowVisitor {
            type Value = Cow<'de, [u8]>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte array")
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Borrowed(v))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Borrowed(v.as_bytes()))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v.to_vec()))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v.as_bytes().to_vec()))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v.into_bytes()))
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let len = cmp::min(visitor.size_hint().unwrap_or(0), 4096);
                let mut bytes = Vec::with_capacity(len);

                while let Some(b) = visitor.next_element()? {
                    bytes.push(b);
                }

                Ok(Cow::Owned(bytes))
            }
        }

        deserializer.deserialize_bytes(CowVisitor)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, Bytes> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cow: Cow<[u8]> = Deserialize::deserialize(deserializer)?;
        match cow {
            Cow::Borrowed(bytes) => Ok(Cow::Borrowed(Bytes::new(bytes))),
            Cow::Owned(bytes) => Ok(Cow::Owned(ByteBuf::from(bytes))),
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for Box<[u8]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Vec::into_boxed_slice)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for Box<Bytes> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Box<[u8]> = Deserialize::deserialize(deserializer)?;
        Ok(bytes.into())
    }
}

impl<'de, T> Deserialize<'de> for Option<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor<T> {
            out: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BytesVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Option<T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("optional byte array")
            }

            fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer).map(Some)
            }
        }

        let visitor = BytesVisitor { out: PhantomData };
        deserializer.deserialize_option(visitor)
    }
}
