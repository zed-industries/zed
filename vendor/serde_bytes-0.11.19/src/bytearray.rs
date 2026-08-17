use crate::Bytes;
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::convert::TryInto as _;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};

/// Wrapper around `[u8; N]` to serialize and deserialize efficiently.
///
/// ```
/// use std::collections::HashMap;
/// use std::io;
///
/// use serde_bytes::ByteArray;
///
/// fn deserialize_bytearrays() -> Result<(), bincode::error::DecodeError> {
///     let example_data = [2, 2, 3, 116, 119, 111, 1, 3, 111, 110, 101];
///
///     let map: HashMap<u32, ByteArray<3>>;
///     (map, _) = bincode::serde::decode_from_slice(
///         &example_data,
///         bincode::config::standard(),
///     )?;
///
///     println!("{:?}", map);
///
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     deserialize_bytearrays().unwrap();
/// # }
/// ```
#[derive(Copy, Clone, Eq, Ord)]
#[repr(transparent)]
pub struct ByteArray<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> ByteArray<N> {
    /// Wrap an existing [array] into a `ByteArray`.
    pub const fn new(bytes: [u8; N]) -> Self {
        ByteArray { bytes }
    }

    /// Unwrap the byte array underlying this `ByteArray`.
    pub const fn into_array(self) -> [u8; N] {
        self.bytes
    }

    fn from_ref(bytes: &[u8; N]) -> &Self {
        unsafe { &*(bytes as *const [u8; N] as *const ByteArray<N>) }
    }
}

impl<const N: usize> Debug for ByteArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.bytes, f)
    }
}

impl<const N: usize> Default for ByteArray<N> {
    fn default() -> Self {
        ByteArray { bytes: [0; N] }
    }
}

impl<const N: usize> AsRef<[u8; N]> for ByteArray<N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.bytes
    }
}

impl<const N: usize> AsMut<[u8; N]> for ByteArray<N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.bytes
    }
}

impl<const N: usize> Borrow<[u8; N]> for ByteArray<N> {
    fn borrow(&self) -> &[u8; N] {
        &self.bytes
    }
}

impl<const N: usize> BorrowMut<[u8; N]> for ByteArray<N> {
    fn borrow_mut(&mut self) -> &mut [u8; N] {
        &mut self.bytes
    }
}

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl<const N: usize> Borrow<Bytes> for ByteArray<N> {
    fn borrow(&self) -> &Bytes {
        Bytes::new(&self.bytes)
    }
}

impl<const N: usize> BorrowMut<Bytes> for ByteArray<N> {
    fn borrow_mut(&mut self) -> &mut Bytes {
        unsafe { &mut *(&mut self.bytes as &mut [u8] as *mut [u8] as *mut Bytes) }
    }
}

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    fn from(bytes: [u8; N]) -> Self {
        ByteArray { bytes }
    }
}

impl<Rhs, const N: usize> PartialEq<Rhs> for ByteArray<N>
where
    Rhs: ?Sized + Borrow<[u8; N]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref().eq(other.borrow())
    }
}

impl<Rhs, const N: usize> PartialOrd<Rhs> for ByteArray<N>
where
    Rhs: ?Sized + Borrow<[u8; N]>,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.borrow())
    }
}

impl<const N: usize> Hash for ByteArray<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl<const N: usize> IntoIterator for ByteArray<N> {
    type Item = u8;
    type IntoIter = <[u8; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.bytes)
    }
}

impl<'a, const N: usize> IntoIterator for &'a ByteArray<N> {
    type Item = &'a u8;
    type IntoIter = <&'a [u8; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut ByteArray<N> {
    type Item = &'a mut u8;
    type IntoIter = <&'a mut [u8; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter_mut()
    }
}

impl<const N: usize> Serialize for ByteArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

struct ByteArrayVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for ByteArrayVisitor<N> {
    type Value = ByteArray<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a byte array of length {N}")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<ByteArray<N>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut bytes = [0; N];

        for (idx, byte) in bytes.iter_mut().enumerate() {
            *byte = seq
                .next_element()?
                .ok_or_else(|| V::Error::invalid_length(idx, &self))?;
        }

        Ok(ByteArray::new(bytes))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<ByteArray<N>, E>
    where
        E: Error,
    {
        Ok(ByteArray {
            bytes: v
                .try_into()
                .map_err(|_| E::invalid_length(v.len(), &self))?,
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<ByteArray<N>, E>
    where
        E: Error,
    {
        self.visit_bytes(v.as_bytes())
    }
}

impl<'de, const N: usize> Deserialize<'de> for ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<ByteArray<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ByteArrayVisitor::<N>)
    }
}

struct BorrowedByteArrayVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for BorrowedByteArrayVisitor<N> {
    type Value = &'de ByteArray<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a borrowed byte array of length {N}")
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let borrowed_byte_array: &'de [u8; N] = v
            .try_into()
            .map_err(|_| E::invalid_length(v.len(), &self))?;
        Ok(ByteArray::from_ref(borrowed_byte_array))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_borrowed_bytes(v.as_bytes())
    }
}

impl<'a, 'de: 'a, const N: usize> Deserialize<'de> for &'a ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(BorrowedByteArrayVisitor::<N>)
    }
}
