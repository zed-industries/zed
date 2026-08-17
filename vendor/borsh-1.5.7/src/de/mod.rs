use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::{
    convert::{TryFrom, TryInto},
    mem::size_of,
};

#[cfg(feature = "bytes")]
use bytes::{BufMut, BytesMut};

use crate::__private::maybestd::{
    borrow::{Borrow, Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet, LinkedList, VecDeque},
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use crate::io::{Error, ErrorKind, Read, Result};

use crate::error::check_zst;

mod hint;

const ERROR_NOT_ALL_BYTES_READ: &str = "Not all bytes read";
const ERROR_UNEXPECTED_LENGTH_OF_INPUT: &str = "Unexpected length of input";
const ERROR_OVERFLOW_ON_MACHINE_WITH_32_BIT_ISIZE: &str = "Overflow on machine with 32 bit isize";
const ERROR_OVERFLOW_ON_MACHINE_WITH_32_BIT_USIZE: &str = "Overflow on machine with 32 bit usize";
const ERROR_INVALID_ZERO_VALUE: &str = "Expected a non-zero value";

#[cfg(feature = "de_strict_order")]
const ERROR_WRONG_ORDER_OF_KEYS: &str = "keys were not serialized in ascending order";

/// A data-structure that can be de-serialized from binary format by NBOR.
pub trait BorshDeserialize: Sized {
    /// Deserializes this instance from a given slice of bytes.
    /// Updates the buffer to point at the remaining bytes.
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        Self::deserialize_reader(&mut *buf)
    }

    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self>;

    /// Deserialize this instance from a slice of bytes.
    fn try_from_slice(v: &[u8]) -> Result<Self> {
        let mut v_mut = v;
        let result = Self::deserialize(&mut v_mut)?;
        if !v_mut.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, ERROR_NOT_ALL_BYTES_READ));
        }
        Ok(result)
    }

    fn try_from_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let result = Self::deserialize_reader(reader)?;
        let mut buf = [0u8; 1];
        match reader.read_exact(&mut buf) {
            Err(f) if f.kind() == ErrorKind::UnexpectedEof => Ok(result),
            _ => Err(Error::new(ErrorKind::InvalidData, ERROR_NOT_ALL_BYTES_READ)),
        }
    }

    #[inline]
    #[doc(hidden)]
    fn vec_from_reader<R: Read>(len: u32, reader: &mut R) -> Result<Option<Vec<Self>>> {
        let _ = len;
        let _ = reader;
        Ok(None)
    }

    #[inline]
    #[doc(hidden)]
    fn array_from_reader<R: Read, const N: usize>(reader: &mut R) -> Result<Option<[Self; N]>> {
        let _ = reader;
        Ok(None)
    }
}

/// Additional methods offered on enums which is used by `[derive(BorshDeserialize)]`.
pub trait EnumExt: BorshDeserialize {
    /// Deserialises given variant of an enum from the reader.
    ///
    /// This may be used to perform validation or filtering based on what
    /// variant is being deserialised.
    ///
    /// ```
    /// use borsh::BorshDeserialize;
    /// use borsh::de::EnumExt as _;
    ///
    /// /// derive is only available if borsh is built with `features = ["derive"]`
    /// # #[cfg(feature = "derive")]
    /// #[derive(Debug, PartialEq, Eq, BorshDeserialize)]
    /// enum MyEnum {
    ///     Zero,
    ///     One(u8),
    ///     Many(Vec<u8>)
    /// }
    ///
    /// # #[cfg(feature = "derive")]
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct OneOrZero(MyEnum);
    ///
    /// # #[cfg(feature = "derive")]
    /// impl borsh::de::BorshDeserialize for OneOrZero {
    ///     fn deserialize_reader<R: borsh::io::Read>(
    ///         reader: &mut R,
    ///     ) -> borsh::io::Result<Self> {
    ///         use borsh::de::EnumExt;
    ///         let tag = u8::deserialize_reader(reader)?;
    ///         if tag == 2 {
    ///             Err(borsh::io::Error::new(
    ///                 borsh::io::ErrorKind::InvalidData,
    ///                 "MyEnum::Many not allowed here",
    ///             ))
    ///         } else {
    ///             MyEnum::deserialize_variant(reader, tag).map(Self)
    ///         }
    ///     }
    /// }
    ///
    /// use borsh::from_slice;
    /// let data = b"\0";
    /// # #[cfg(feature = "derive")]
    /// assert_eq!(MyEnum::Zero, from_slice::<MyEnum>(&data[..]).unwrap());
    /// # #[cfg(feature = "derive")]
    /// assert_eq!(MyEnum::Zero, from_slice::<OneOrZero>(&data[..]).unwrap().0);
    ///
    /// let data = b"\x02\0\0\0\0";
    /// # #[cfg(feature = "derive")]
    /// assert_eq!(MyEnum::Many(Vec::new()), from_slice::<MyEnum>(&data[..]).unwrap());
    /// # #[cfg(feature = "derive")]
    /// assert!(from_slice::<OneOrZero>(&data[..]).is_err());
    /// ```
    fn deserialize_variant<R: Read>(reader: &mut R, tag: u8) -> Result<Self>;
}

fn unexpected_eof_to_unexpected_length_of_input(e: Error) -> Error {
    if e.kind() == ErrorKind::UnexpectedEof {
        Error::new(ErrorKind::InvalidData, ERROR_UNEXPECTED_LENGTH_OF_INPUT)
    } else {
        e
    }
}

impl BorshDeserialize for u8 {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(buf[0])
    }

    #[inline]
    #[doc(hidden)]
    fn vec_from_reader<R: Read>(len: u32, reader: &mut R) -> Result<Option<Vec<Self>>> {
        let len: usize = len.try_into().map_err(|_| ErrorKind::InvalidData)?;
        // Avoid OOM by limiting the size of allocation.  This makes the read
        // less efficient (since we need to loop and reallocate) but it protects
        // us from someone sending us [0xff, 0xff, 0xff, 0xff] and forcing us to
        // allocate 4GiB of memory.
        let mut vec = vec![0u8; len.min(1024 * 1024)];
        let mut pos = 0;
        while pos < len {
            if pos == vec.len() {
                vec.resize(vec.len().saturating_mul(2).min(len), 0)
            }
            // TODO(mina86): Convert this to read_buf once that stabilises.
            match reader.read(&mut vec.as_mut_slice()[pos..])? {
                0 => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        ERROR_UNEXPECTED_LENGTH_OF_INPUT,
                    ))
                }
                read => {
                    pos += read;
                }
            }
        }
        Ok(Some(vec))
    }

    #[inline]
    #[doc(hidden)]
    fn array_from_reader<R: Read, const N: usize>(reader: &mut R) -> Result<Option<[Self; N]>> {
        let mut arr = [0u8; N];
        reader
            .read_exact(&mut arr)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(Some(arr))
    }
}

macro_rules! impl_for_integer {
    ($type: ident) => {
        impl BorshDeserialize for $type {
            #[inline]
            fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
                let mut buf = [0u8; size_of::<$type>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(unexpected_eof_to_unexpected_length_of_input)?;
                let res = $type::from_le_bytes(buf.try_into().unwrap());
                Ok(res)
            }
        }
    };
}

impl_for_integer!(i8);
impl_for_integer!(i16);
impl_for_integer!(i32);
impl_for_integer!(i64);
impl_for_integer!(i128);
impl_for_integer!(u16);
impl_for_integer!(u32);
impl_for_integer!(u64);
impl_for_integer!(u128);

macro_rules! impl_for_nonzero_integer {
    ($type: ty) => {
        impl BorshDeserialize for $type {
            #[inline]
            fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
                <$type>::new(BorshDeserialize::deserialize_reader(reader)?)
                    .ok_or_else(|| Error::new(ErrorKind::InvalidData, ERROR_INVALID_ZERO_VALUE))
            }
        }
    };
}

impl_for_nonzero_integer!(core::num::NonZeroI8);
impl_for_nonzero_integer!(core::num::NonZeroI16);
impl_for_nonzero_integer!(core::num::NonZeroI32);
impl_for_nonzero_integer!(core::num::NonZeroI64);
impl_for_nonzero_integer!(core::num::NonZeroI128);
impl_for_nonzero_integer!(core::num::NonZeroU8);
impl_for_nonzero_integer!(core::num::NonZeroU16);
impl_for_nonzero_integer!(core::num::NonZeroU32);
impl_for_nonzero_integer!(core::num::NonZeroU64);
impl_for_nonzero_integer!(core::num::NonZeroU128);
impl_for_nonzero_integer!(core::num::NonZeroUsize);

impl BorshDeserialize for isize {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let i: i64 = BorshDeserialize::deserialize_reader(reader)?;
        let i = isize::try_from(i).map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                ERROR_OVERFLOW_ON_MACHINE_WITH_32_BIT_ISIZE,
            )
        })?;
        Ok(i)
    }
}

impl BorshDeserialize for usize {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let u: u64 = BorshDeserialize::deserialize_reader(reader)?;
        let u = usize::try_from(u).map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                ERROR_OVERFLOW_ON_MACHINE_WITH_32_BIT_USIZE,
            )
        })?;
        Ok(u)
    }
}

// Note NaNs have a portability issue. Specifically, signalling NaNs on MIPS are quiet NaNs on x86,
// and vice-versa. We disallow NaNs to avoid this issue.
macro_rules! impl_for_float {
    ($type: ident, $int_type: ident) => {
        impl BorshDeserialize for $type {
            #[inline]
            fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
                let mut buf = [0u8; size_of::<$type>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(unexpected_eof_to_unexpected_length_of_input)?;
                let res = $type::from_bits($int_type::from_le_bytes(buf.try_into().unwrap()));
                if res.is_nan() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "For portability reasons we do not allow to deserialize NaNs.",
                    ));
                }
                Ok(res)
            }
        }
    };
}

impl_for_float!(f32, u32);
impl_for_float!(f64, u64);

impl BorshDeserialize for bool {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let b: u8 = BorshDeserialize::deserialize_reader(reader)?;
        if b == 0 {
            Ok(false)
        } else if b == 1 {
            Ok(true)
        } else {
            let msg = format!("Invalid bool representation: {}", b);

            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    }
}

impl<T> BorshDeserialize for Option<T>
where
    T: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
        if flag == 0 {
            Ok(None)
        } else if flag == 1 {
            Ok(Some(T::deserialize_reader(reader)?))
        } else {
            let msg = format!(
                "Invalid Option representation: {}. The first byte must be 0 or 1",
                flag
            );

            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    }
}

impl<T, E> BorshDeserialize for core::result::Result<T, E>
where
    T: BorshDeserialize,
    E: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let flag: u8 = BorshDeserialize::deserialize_reader(reader)?;
        if flag == 0 {
            Ok(Err(E::deserialize_reader(reader)?))
        } else if flag == 1 {
            Ok(Ok(T::deserialize_reader(reader)?))
        } else {
            let msg = format!(
                "Invalid Result representation: {}. The first byte must be 0 or 1",
                flag
            );

            Err(Error::new(ErrorKind::InvalidData, msg))
        }
    }
}

impl BorshDeserialize for String {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        String::from_utf8(Vec::<u8>::deserialize_reader(reader)?).map_err(|err| {
            let msg = err.to_string();
            Error::new(ErrorKind::InvalidData, msg)
        })
    }
}

/// Module is available if borsh is built with `features = ["ascii"]`.
#[cfg(feature = "ascii")]
pub mod ascii {
    //!
    //! Module defines [BorshDeserialize] implementation for
    //! some types from [ascii](::ascii) crate.
    use crate::BorshDeserialize;
    use crate::__private::maybestd::{string::ToString, vec::Vec};
    use crate::io::{Error, ErrorKind, Read, Result};

    impl BorshDeserialize for ascii::AsciiString {
        #[inline]
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
            let bytes = Vec::<u8>::deserialize_reader(reader)?;
            ascii::AsciiString::from_ascii(bytes)
                .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))
        }
    }

    impl BorshDeserialize for ascii::AsciiChar {
        #[inline]
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
            let byte = u8::deserialize_reader(reader)?;
            ascii::AsciiChar::from_ascii(byte)
                .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))
        }
    }
}

impl<T> BorshDeserialize for Vec<T>
where
    T: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        check_zst::<T>()?;

        let len = u32::deserialize_reader(reader)?;
        if len == 0 {
            Ok(Vec::new())
        } else if let Some(vec_bytes) = T::vec_from_reader(len, reader)? {
            Ok(vec_bytes)
        } else {
            // TODO(16): return capacity allocation when we can safely do that.
            let mut result = Vec::with_capacity(hint::cautious::<T>(len));
            for _ in 0..len {
                result.push(T::deserialize_reader(reader)?);
            }
            Ok(result)
        }
    }
}

#[cfg(feature = "bytes")]
impl BorshDeserialize for bytes::Bytes {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let vec = <Vec<u8>>::deserialize_reader(reader)?;
        Ok(vec.into())
    }
}

#[cfg(feature = "bytes")]
impl BorshDeserialize for bytes::BytesMut {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let len = u32::deserialize_reader(reader)?;
        let mut out = BytesMut::with_capacity(hint::cautious::<u8>(len));
        for _ in 0..len {
            out.put_u8(u8::deserialize_reader(reader)?);
        }
        Ok(out)
    }
}

#[cfg(feature = "bson")]
impl BorshDeserialize for bson::oid::ObjectId {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 12];
        reader.read_exact(&mut buf)?;
        Ok(bson::oid::ObjectId::from_bytes(buf))
    }
}

#[cfg(feature = "indexmap")]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L39
// license: MIT OR Apache-2.0
impl<K, V, S> BorshDeserialize for indexmap::IndexMap<K, V, S>
where
    K: BorshDeserialize + Eq + core::hash::Hash,
    V: BorshDeserialize,
    S: core::hash::BuildHasher + Default,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        check_zst::<K>()?;
        let vec = <Vec<(K, V)>>::deserialize_reader(reader)?;
        Ok(vec.into_iter().collect::<indexmap::IndexMap<K, V, S>>())
    }
}

#[cfg(feature = "indexmap")]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L75
// license: MIT OR Apache-2.0
impl<T, S> BorshDeserialize for indexmap::IndexSet<T, S>
where
    T: BorshDeserialize + Eq + core::hash::Hash,
    S: core::hash::BuildHasher + Default,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        check_zst::<T>()?;
        let vec = <Vec<T>>::deserialize_reader(reader)?;
        Ok(vec.into_iter().collect::<indexmap::IndexSet<T, S>>())
    }
}

impl<T> BorshDeserialize for Cow<'_, T>
where
    T: ToOwned + ?Sized,
    T::Owned: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Cow::Owned(BorshDeserialize::deserialize_reader(reader)?))
    }
}

impl<T> BorshDeserialize for VecDeque<T>
where
    T: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let vec = <Vec<T>>::deserialize_reader(reader)?;
        Ok(vec.into())
    }
}

impl<T> BorshDeserialize for LinkedList<T>
where
    T: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let vec = <Vec<T>>::deserialize_reader(reader)?;
        Ok(vec.into_iter().collect::<LinkedList<T>>())
    }
}

/// Module is available if borsh is built with `features = ["std"]` or `features = ["hashbrown"]`.
///
/// Module defines [BorshDeserialize] implementation for
/// [HashMap](std::collections::HashMap)/[HashSet](std::collections::HashSet).
#[cfg(hash_collections)]
pub mod hashes {
    use core::hash::{BuildHasher, Hash};

    use crate::BorshDeserialize;
    use crate::__private::maybestd::collections::{HashMap, HashSet};
    use crate::__private::maybestd::vec::Vec;
    use crate::io::{Read, Result};

    #[cfg(feature = "de_strict_order")]
    const ERROR_WRONG_ORDER_OF_KEYS: &str = "keys were not serialized in ascending order";
    use crate::error::check_zst;
    #[cfg(feature = "de_strict_order")]
    use crate::io::{Error, ErrorKind};

    impl<T, H> BorshDeserialize for HashSet<T, H>
    where
        T: BorshDeserialize + Eq + Hash + Ord,
        H: BuildHasher + Default,
    {
        #[inline]
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
            // NOTE: deserialize-as-you-go approach as once was in HashSet is better in the sense
            // that it allows to fail early, and not allocate memory for all the elements
            // which may fail `cmp()` checks
            // NOTE: deserialize first to `Vec<T>` is faster
            let vec = <Vec<T>>::deserialize_reader(reader)?;

            #[cfg(feature = "de_strict_order")]
            // TODO: replace with `is_sorted` api when stabilizes https://github.com/rust-lang/rust/issues/53485
            // TODO: first replace with `array_windows` api when stabilizes https://github.com/rust-lang/rust/issues/75027
            for pair in vec.windows(2) {
                let [a, b] = pair else {
                    unreachable!("`windows` always return a slice of length 2 or nothing");
                };
                let cmp_result = a.cmp(b).is_lt();
                if !cmp_result {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        ERROR_WRONG_ORDER_OF_KEYS,
                    ));
                }
            }

            Ok(vec.into_iter().collect::<HashSet<T, H>>())
        }
    }

    impl<K, V, H> BorshDeserialize for HashMap<K, V, H>
    where
        K: BorshDeserialize + Eq + Hash + Ord,
        V: BorshDeserialize,
        H: BuildHasher + Default,
    {
        #[inline]
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
            check_zst::<K>()?;
            // NOTE: deserialize-as-you-go approach as once was in HashSet is better in the sense
            // that it allows to fail early, and not allocate memory for all the entries
            // which may fail `cmp()` checks
            // NOTE: deserialize first to `Vec<(K, V)>` is faster
            let vec = <Vec<(K, V)>>::deserialize_reader(reader)?;

            #[cfg(feature = "de_strict_order")]
            // TODO: replace with `is_sorted` api when stabilizes https://github.com/rust-lang/rust/issues/53485
            // TODO: first replace with `array_windows` api when stabilizes https://github.com/rust-lang/rust/issues/75027
            for pair in vec.windows(2) {
                let [(a_k, _a_v), (b_k, _b_v)] = pair else {
                    unreachable!("`windows` always return a slice of length 2 or nothing");
                };
                let cmp_result = a_k.cmp(b_k).is_lt();
                if !cmp_result {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        ERROR_WRONG_ORDER_OF_KEYS,
                    ));
                }
            }

            Ok(vec.into_iter().collect::<HashMap<K, V, H>>())
        }
    }
}

impl<T> BorshDeserialize for BTreeSet<T>
where
    T: BorshDeserialize + Ord,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        // NOTE: deserialize-as-you-go approach as once was in HashSet is better in the sense
        // that it allows to fail early, and not allocate memory for all the elements
        // which may fail `cmp()` checks
        // NOTE: deserialize first to `Vec<T>` is faster
        let vec = <Vec<T>>::deserialize_reader(reader)?;

        #[cfg(feature = "de_strict_order")]
        // TODO: replace with `is_sorted` api when stabilizes https://github.com/rust-lang/rust/issues/53485
        // TODO: first replace with `array_windows` api when stabilizes https://github.com/rust-lang/rust/issues/75027
        for pair in vec.windows(2) {
            let [a, b] = pair else {
                unreachable!("`windows` always return a slice of length 2 or nothing");
            };
            let cmp_result = a.cmp(b).is_lt();
            if !cmp_result {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    ERROR_WRONG_ORDER_OF_KEYS,
                ));
            }
        }
        // NOTE: BTreeSet has an optimization inside of impl <T> FromIterator<T> for BTreeSet<T, Global>,
        // based on BTreeMap::bulk_build_from_sorted_iter
        Ok(vec.into_iter().collect::<BTreeSet<T>>())
    }
}

impl<K, V> BorshDeserialize for BTreeMap<K, V>
where
    K: BorshDeserialize + Ord,
    V: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        check_zst::<K>()?;
        // NOTE: deserialize-as-you-go approach as once was in HashSet is better in the sense
        // that it allows to fail early, and not allocate memory for all the entries
        // which may fail `cmp()` checks
        // NOTE: deserialize first to `Vec<(K, V)>` is faster
        let vec = <Vec<(K, V)>>::deserialize_reader(reader)?;

        #[cfg(feature = "de_strict_order")]
        // TODO: replace with `is_sorted` api when stabilizes https://github.com/rust-lang/rust/issues/53485
        // TODO: first replace with `array_windows` api when stabilizes https://github.com/rust-lang/rust/issues/75027
        for pair in vec.windows(2) {
            let [(a_k, _a_v), (b_k, _b_v)] = pair else {
                unreachable!("`windows` always return a slice of length 2 or nothing");
            };
            let cmp_result = a_k.cmp(b_k).is_lt();
            if !cmp_result {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    ERROR_WRONG_ORDER_OF_KEYS,
                ));
            }
        }

        // NOTE: BTreeMap has an optimization inside of impl<K, V> FromIterator<(K, V)> for BTreeMap<K, V, Global>,
        // based on BTreeMap::bulk_build_from_sorted_iter
        Ok(vec.into_iter().collect::<BTreeMap<K, V>>())
    }
}

#[cfg(feature = "std")]
impl BorshDeserialize for std::net::SocketAddr {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let kind = u8::deserialize_reader(reader)?;
        match kind {
            0 => std::net::SocketAddrV4::deserialize_reader(reader).map(std::net::SocketAddr::V4),
            1 => std::net::SocketAddrV6::deserialize_reader(reader).map(std::net::SocketAddr::V6),
            value => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid SocketAddr variant: {}", value),
            )),
        }
    }
}

#[cfg(feature = "std")]
impl BorshDeserialize for std::net::IpAddr {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let kind = u8::deserialize_reader(reader)?;
        match kind {
            0u8 => {
                // Deserialize an Ipv4Addr and convert it to IpAddr::V4
                let ipv4_addr = std::net::Ipv4Addr::deserialize_reader(reader)?;
                Ok(std::net::IpAddr::V4(ipv4_addr))
            }
            1u8 => {
                // Deserialize an Ipv6Addr and convert it to IpAddr::V6
                let ipv6_addr = std::net::Ipv6Addr::deserialize_reader(reader)?;
                Ok(std::net::IpAddr::V6(ipv6_addr))
            }
            value => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid IpAddr variant: {}", value),
            )),
        }
    }
}

#[cfg(feature = "std")]
impl BorshDeserialize for std::net::SocketAddrV4 {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let ip = std::net::Ipv4Addr::deserialize_reader(reader)?;
        let port = u16::deserialize_reader(reader)?;
        Ok(std::net::SocketAddrV4::new(ip, port))
    }
}

#[cfg(feature = "std")]
impl BorshDeserialize for std::net::SocketAddrV6 {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let ip = std::net::Ipv6Addr::deserialize_reader(reader)?;
        let port = u16::deserialize_reader(reader)?;
        Ok(std::net::SocketAddrV6::new(ip, port, 0, 0))
    }
}

#[cfg(feature = "std")]
impl BorshDeserialize for std::net::Ipv4Addr {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 4];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(std::net::Ipv4Addr::from(buf))
    }
}

#[cfg(feature = "std")]
impl BorshDeserialize for std::net::Ipv6Addr {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 16];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(std::net::Ipv6Addr::from(buf))
    }
}

impl<T, U> BorshDeserialize for Box<T>
where
    U: Into<Box<T>> + Borrow<T>,
    T: ToOwned<Owned = U> + ?Sized,
    T::Owned: BorshDeserialize,
{
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(T::Owned::deserialize_reader(reader)?.into())
    }
}

impl<T, const N: usize> BorshDeserialize for [T; N]
where
    T: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        struct ArrayDropGuard<T, const N: usize> {
            buffer: [MaybeUninit<T>; N],
            init_count: usize,
        }
        impl<T, const N: usize> Drop for ArrayDropGuard<T, N> {
            fn drop(&mut self) {
                let init_range = &mut self.buffer[..self.init_count];
                // SAFETY: Elements up to self.init_count have been initialized. Assumes this value
                //         is only incremented in `fill_buffer`, which writes the element before
                //         increasing the init_count.
                unsafe {
                    core::ptr::drop_in_place(init_range as *mut _ as *mut [T]);
                };
            }
        }
        impl<T, const N: usize> ArrayDropGuard<T, N> {
            unsafe fn transmute_to_array(mut self) -> [T; N] {
                debug_assert_eq!(self.init_count, N);
                // Set init_count to 0 so that the values do not get dropped twice.
                self.init_count = 0;
                // SAFETY: This cast is required because `mem::transmute` does not work with
                //         const generics https://github.com/rust-lang/rust/issues/61956. This
                //         array is guaranteed to be initialized by this point.
                core::ptr::read(&self.buffer as *const _ as *const [T; N])
            }
            fn fill_buffer(&mut self, mut f: impl FnMut() -> Result<T>) -> Result<()> {
                // TODO: replace with `core::array::try_from_fn` when stabilized to avoid manually
                // dropping uninitialized values through the guard drop.
                for elem in self.buffer.iter_mut() {
                    elem.write(f()?);
                    self.init_count += 1;
                }
                Ok(())
            }
        }

        if let Some(arr) = T::array_from_reader(reader)? {
            Ok(arr)
        } else {
            let mut result = ArrayDropGuard {
                buffer: unsafe { MaybeUninit::uninit().assume_init() },
                init_count: 0,
            };

            result.fill_buffer(|| T::deserialize_reader(reader))?;

            // SAFETY: The elements up to `i` have been initialized in `fill_buffer`.
            Ok(unsafe { result.transmute_to_array() })
        }
    }
}

#[test]
fn array_deserialization_doesnt_leak() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    static DESERIALIZE_COUNT: AtomicUsize = AtomicUsize::new(0);
    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[allow(unused)]
    struct MyType(u8);
    impl BorshDeserialize for MyType {
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
            let val = u8::deserialize_reader(reader)?;
            let v = DESERIALIZE_COUNT.fetch_add(1, Ordering::SeqCst);
            if v >= 7 {
                panic!("panic in deserialize");
            }
            Ok(MyType(val))
        }
    }
    impl Drop for MyType {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    assert!(<[MyType; 5] as BorshDeserialize>::deserialize(&mut &[0u8; 3][..]).is_err());
    assert_eq!(DESERIALIZE_COUNT.load(Ordering::SeqCst), 3);
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 3);

    assert!(<[MyType; 2] as BorshDeserialize>::deserialize(&mut &[0u8; 2][..]).is_ok());
    assert_eq!(DESERIALIZE_COUNT.load(Ordering::SeqCst), 5);
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 5);

    #[cfg(feature = "std")]
    {
        // Test that during a panic in deserialize, the values are still dropped.
        let result = std::panic::catch_unwind(|| {
            <[MyType; 3] as BorshDeserialize>::deserialize(&mut &[0u8; 3][..]).unwrap();
        });
        assert!(result.is_err());
        assert_eq!(DESERIALIZE_COUNT.load(Ordering::SeqCst), 8);
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 7); // 5 because 6 panicked and was not init
    }
}

macro_rules! impl_tuple {
    (@unit $name:ty) => {
        impl BorshDeserialize for $name {
            #[inline]
            fn deserialize_reader<R: Read>(_reader: &mut R) -> Result<Self> {
                Ok(<$name>::default())
            }
        }
    };

    ($($name:ident)+) => {
      impl<$($name),+> BorshDeserialize for ($($name,)+)
      where $($name: BorshDeserialize,)+
      {
        #[inline]
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {

            Ok(($($name::deserialize_reader(reader)?,)+))
        }
      }
    };
}

impl_tuple!(@unit ());
impl_tuple!(@unit core::ops::RangeFull);

impl_tuple!(T0);
impl_tuple!(T0 T1);
impl_tuple!(T0 T1 T2);
impl_tuple!(T0 T1 T2 T3);
impl_tuple!(T0 T1 T2 T3 T4);
impl_tuple!(T0 T1 T2 T3 T4 T5);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18);
impl_tuple!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19);

macro_rules! impl_range {
    ($type:ident, $make:expr, $($side:ident),*) => {
        impl<T: BorshDeserialize> BorshDeserialize for core::ops::$type<T> {
            #[inline]
            fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
                let ($($side,)*) = <_>::deserialize_reader(reader)?;
                Ok($make)
            }
        }
    };
}

impl_range!(Range, start..end, start, end);
impl_range!(RangeInclusive, start..=end, start, end);
impl_range!(RangeFrom, start.., start);
impl_range!(RangeTo, ..end, end);
impl_range!(RangeToInclusive, ..=end, end);

/// Module is available if borsh is built with `features = ["rc"]`.
#[cfg(feature = "rc")]
pub mod rc {
    //!
    //! Module defines [BorshDeserialize] implementation for
    //! [alloc::rc::Rc](std::rc::Rc) and [alloc::sync::Arc](std::sync::Arc).
    use crate::__private::maybestd::{boxed::Box, rc::Rc, sync::Arc};
    use crate::io::{Read, Result};
    use crate::BorshDeserialize;

    /// This impl requires the [`"rc"`] Cargo feature of borsh.
    ///
    /// Deserializing a data structure containing `Rc` will not attempt to
    /// deduplicate `Rc` references to the same data. Every deserialized `Rc`
    /// will end up with a strong count of 1.
    impl<T: ?Sized> BorshDeserialize for Rc<T>
    where
        Box<T>: BorshDeserialize,
    {
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
            Ok(Box::<T>::deserialize_reader(reader)?.into())
        }
    }

    /// This impl requires the [`"rc"`] Cargo feature of borsh.
    ///
    /// Deserializing a data structure containing `Arc` will not attempt to
    /// deduplicate `Arc` references to the same data. Every deserialized `Arc`
    /// will end up with a strong count of 1.
    impl<T: ?Sized> BorshDeserialize for Arc<T>
    where
        Box<T>: BorshDeserialize,
    {
        fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
            Ok(Box::<T>::deserialize_reader(reader)?.into())
        }
    }
}

impl<T: ?Sized> BorshDeserialize for PhantomData<T> {
    fn deserialize_reader<R: Read>(_: &mut R) -> Result<Self> {
        Ok(PhantomData)
    }
}

impl<T> BorshDeserialize for core::cell::Cell<T>
where
    T: BorshDeserialize + Copy,
{
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        <T as BorshDeserialize>::deserialize_reader(reader).map(core::cell::Cell::new)
    }
}

impl<T> BorshDeserialize for core::cell::RefCell<T>
where
    T: BorshDeserialize,
{
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        <T as BorshDeserialize>::deserialize_reader(reader).map(core::cell::RefCell::new)
    }
}

/// Deserializes an object from a slice of bytes.
/// # Example
/// ```
/// use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};
///
/// /// derive is only available if borsh is built with `features = ["derive"]`
/// # #[cfg(feature = "derive")]
/// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
/// struct MyStruct {
///    a: u64,
///    b: Vec<u8>,
/// }
///
/// # #[cfg(feature = "derive")]
/// let original = MyStruct { a: 10, b: vec![1, 2, 3] };
/// # #[cfg(feature = "derive")]
/// let encoded = to_vec(&original).unwrap();
/// # #[cfg(feature = "derive")]
/// let decoded = from_slice::<MyStruct>(&encoded).unwrap();
/// # #[cfg(feature = "derive")]
/// assert_eq!(original, decoded);
/// ```
/// # Panics
/// If the data is invalid, this function will panic.
/// # Errors
/// If the data is invalid, this function will return an error.
/// # Note
/// This function will return an error if the data is not fully read.
pub fn from_slice<T: BorshDeserialize>(v: &[u8]) -> Result<T> {
    let mut v_mut = v;
    let object = T::deserialize(&mut v_mut)?;
    if !v_mut.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            crate::de::ERROR_NOT_ALL_BYTES_READ,
        ));
    }
    Ok(object)
}

/// Deserializes an object from a reader.
/// # Example
/// ```
/// use borsh::{BorshDeserialize, BorshSerialize, from_reader, to_vec};
///
/// /// derive is only available if borsh is built with `features = ["derive"]`
/// # #[cfg(feature = "derive")]
/// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
/// struct MyStruct {
///     a: u64,
///     b: Vec<u8>,
/// }
///
/// # #[cfg(feature = "derive")]
/// let original = MyStruct { a: 10, b: vec![1, 2, 3] };
/// # #[cfg(feature = "derive")]
/// let encoded = to_vec(&original).unwrap();
/// # #[cfg(feature = "derive")]
/// let decoded = from_reader::<_, MyStruct>(&mut encoded.as_slice()).unwrap();
/// # #[cfg(feature = "derive")]
/// assert_eq!(original, decoded);
/// ```
pub fn from_reader<R: Read, T: BorshDeserialize>(reader: &mut R) -> Result<T> {
    let result = T::deserialize_reader(reader)?;
    let mut buf = [0u8; 1];
    match reader.read_exact(&mut buf) {
        Err(f) if f.kind() == ErrorKind::UnexpectedEof => Ok(result),
        _ => Err(Error::new(ErrorKind::InvalidData, ERROR_NOT_ALL_BYTES_READ)),
    }
}
