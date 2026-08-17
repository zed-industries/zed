use core::convert::TryFrom;
use core::marker::PhantomData;

use crate::__private::maybestd::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet, LinkedList, VecDeque},
    string::String,
    vec::Vec,
};
use crate::error::check_zst;
use crate::io::{Error, ErrorKind, Result, Write};

pub(crate) mod helpers;

const FLOAT_NAN_ERR: &str = "For portability reasons we do not allow to serialize NaNs.";

/// A data-structure that can be serialized into binary format by NBOR.
///
/// ```
/// use borsh::BorshSerialize;
///
/// /// derive is only available if borsh is built with `features = ["derive"]`
/// # #[cfg(feature = "derive")]
/// #[derive(BorshSerialize)]
/// struct MyBorshSerializableStruct {
///     value: String,
/// }
///
///
/// # #[cfg(feature = "derive")]
/// let x = MyBorshSerializableStruct { value: "hello".to_owned() };
/// let mut buffer: Vec<u8> = Vec::new();
/// # #[cfg(feature = "derive")]
/// x.serialize(&mut buffer).unwrap();
/// # #[cfg(feature = "derive")]
/// let single_serialized_buffer_len = buffer.len();
///
/// # #[cfg(feature = "derive")]
/// x.serialize(&mut buffer).unwrap();
/// # #[cfg(feature = "derive")]
/// assert_eq!(buffer.len(), single_serialized_buffer_len * 2);
///
/// # #[cfg(feature = "derive")]
/// let mut buffer: Vec<u8> = vec![0; 1024 + single_serialized_buffer_len];
/// # #[cfg(feature = "derive")]
/// let mut buffer_slice_enough_for_the_data = &mut buffer[1024..1024 + single_serialized_buffer_len];
/// # #[cfg(feature = "derive")]
/// x.serialize(&mut buffer_slice_enough_for_the_data).unwrap();
/// ```
pub trait BorshSerialize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()>;

    #[inline]
    #[doc(hidden)]
    fn u8_slice(slice: &[Self]) -> Option<&[u8]>
    where
        Self: Sized,
    {
        let _ = slice;
        None
    }
}

impl BorshSerialize for u8 {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(core::slice::from_ref(self))
    }

    #[inline]
    fn u8_slice(slice: &[Self]) -> Option<&[u8]> {
        Some(slice)
    }
}

macro_rules! impl_for_integer {
    ($type: ident) => {
        impl BorshSerialize for $type {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
                let bytes = self.to_le_bytes();
                writer.write_all(&bytes)
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
        impl BorshSerialize for $type {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
                BorshSerialize::serialize(&self.get(), writer)
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

impl BorshSerialize for isize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        BorshSerialize::serialize(&(*self as i64), writer)
    }
}

impl BorshSerialize for usize {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        BorshSerialize::serialize(&(*self as u64), writer)
    }
}

// Note NaNs have a portability issue. Specifically, signalling NaNs on MIPS are quiet NaNs on x86,
// and vice-versa. We disallow NaNs to avoid this issue.
macro_rules! impl_for_float {
    ($type: ident) => {
        impl BorshSerialize for $type {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
                if self.is_nan() {
                    return Err(Error::new(ErrorKind::InvalidData, FLOAT_NAN_ERR));
                }
                writer.write_all(&self.to_bits().to_le_bytes())
            }
        }
    };
}

impl_for_float!(f32);
impl_for_float!(f64);

impl BorshSerialize for bool {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        (u8::from(*self)).serialize(writer)
    }
}

impl<T> BorshSerialize for Option<T>
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            None => 0u8.serialize(writer),
            Some(value) => {
                1u8.serialize(writer)?;
                value.serialize(writer)
            }
        }
    }
}

impl<T, E> BorshSerialize for core::result::Result<T, E>
where
    T: BorshSerialize,
    E: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Err(e) => {
                0u8.serialize(writer)?;
                e.serialize(writer)
            }
            Ok(v) => {
                1u8.serialize(writer)?;
                v.serialize(writer)
            }
        }
    }
}

impl BorshSerialize for str {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.as_bytes().serialize(writer)
    }
}

impl BorshSerialize for String {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.as_bytes().serialize(writer)
    }
}

/// Module is available if borsh is built with `features = ["ascii"]`.
#[cfg(feature = "ascii")]
pub mod ascii {
    //!
    //! Module defines [BorshSerialize] implementation for
    //! some types from [ascii](::ascii) crate.
    use super::BorshSerialize;
    use crate::io::{Result, Write};

    impl BorshSerialize for ascii::AsciiChar {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            self.as_byte().serialize(writer)
        }
    }

    impl BorshSerialize for ascii::AsciiStr {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            self.as_bytes().serialize(writer)
        }
    }

    impl BorshSerialize for ascii::AsciiString {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            self.as_bytes().serialize(writer)
        }
    }
}

/// Helper method that is used to serialize a slice of data (without the length marker).
#[inline]
fn serialize_slice<T: BorshSerialize, W: Write>(data: &[T], writer: &mut W) -> Result<()> {
    if let Some(u8_slice) = T::u8_slice(data) {
        writer.write_all(u8_slice)?;
    } else {
        for item in data {
            item.serialize(writer)?;
        }
    }
    Ok(())
}

impl<T> BorshSerialize for [T]
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(
            &(u32::try_from(self.len()).map_err(|_| ErrorKind::InvalidData)?).to_le_bytes(),
        )?;
        serialize_slice(self, writer)
    }
}

impl<T: BorshSerialize + ?Sized> BorshSerialize for &T {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        (*self).serialize(writer)
    }
}

impl<T> BorshSerialize for Cow<'_, T>
where
    T: BorshSerialize + ToOwned + ?Sized,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.as_ref().serialize(writer)
    }
}

impl<T> BorshSerialize for Vec<T>
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        check_zst::<T>()?;

        self.as_slice().serialize(writer)
    }
}

#[cfg(feature = "bytes")]
impl BorshSerialize for bytes::Bytes {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.as_ref().serialize(writer)
    }
}

#[cfg(feature = "bytes")]
impl BorshSerialize for bytes::BytesMut {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.as_ref().serialize(writer)
    }
}

#[cfg(feature = "bson")]
impl BorshSerialize for bson::oid::ObjectId {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.bytes().serialize(writer)
    }
}

#[cfg(feature = "indexmap")]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L74C1-L86C2
// license: MIT OR Apache-2.0
impl<T, S> BorshSerialize for indexmap::IndexSet<T, S>
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        check_zst::<T>()?;

        let iterator = self.iter();

        u32::try_from(iterator.len())
            .map_err(|_| ErrorKind::InvalidData)?
            .serialize(writer)?;

        for item in iterator {
            item.serialize(writer)?;
        }

        Ok(())
    }
}

#[cfg(feature = "indexmap")]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L15
// license: MIT OR Apache-2.0
impl<K, V, S> BorshSerialize for indexmap::IndexMap<K, V, S>
where
    K: BorshSerialize,
    V: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        check_zst::<K>()?;

        let iterator = self.iter();

        u32::try_from(iterator.len())
            .map_err(|_| ErrorKind::InvalidData)?
            .serialize(writer)?;

        for (key, value) in iterator {
            key.serialize(writer)?;
            value.serialize(writer)?;
        }

        Ok(())
    }
}

impl<T> BorshSerialize for VecDeque<T>
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        check_zst::<T>()?;

        writer.write_all(
            &(u32::try_from(self.len()).map_err(|_| ErrorKind::InvalidData)?).to_le_bytes(),
        )?;
        let slices = self.as_slices();
        serialize_slice(slices.0, writer)?;
        serialize_slice(slices.1, writer)
    }
}

impl<T> BorshSerialize for LinkedList<T>
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        check_zst::<T>()?;

        writer.write_all(
            &(u32::try_from(self.len()).map_err(|_| ErrorKind::InvalidData)?).to_le_bytes(),
        )?;
        for item in self {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

/// Module is available if borsh is built with `features = ["std"]` or `features = ["hashbrown"]`.
///
/// Module defines [BorshSerialize] implementation for
/// [HashMap](std::collections::HashMap)/[HashSet](std::collections::HashSet).
#[cfg(hash_collections)]
pub mod hashes {
    use crate::__private::maybestd::vec::Vec;
    use crate::error::check_zst;
    use crate::{
        BorshSerialize,
        __private::maybestd::collections::{HashMap, HashSet},
    };
    use core::convert::TryFrom;
    use core::hash::BuildHasher;

    use crate::io::{ErrorKind, Result, Write};

    impl<K, V, H> BorshSerialize for HashMap<K, V, H>
    where
        K: BorshSerialize + Ord,
        V: BorshSerialize,
        H: BuildHasher,
    {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            check_zst::<K>()?;

            let mut vec = self.iter().collect::<Vec<_>>();
            vec.sort_by(|(a, _), (b, _)| a.cmp(b));
            u32::try_from(vec.len())
                .map_err(|_| ErrorKind::InvalidData)?
                .serialize(writer)?;
            for kv in vec {
                kv.serialize(writer)?;
            }
            Ok(())
        }
    }

    impl<T, H> BorshSerialize for HashSet<T, H>
    where
        T: BorshSerialize + Ord,
        H: BuildHasher,
    {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            check_zst::<T>()?;

            let mut vec = self.iter().collect::<Vec<_>>();
            vec.sort();
            u32::try_from(vec.len())
                .map_err(|_| ErrorKind::InvalidData)?
                .serialize(writer)?;
            for item in vec {
                item.serialize(writer)?;
            }
            Ok(())
        }
    }
}

impl<K, V> BorshSerialize for BTreeMap<K, V>
where
    K: BorshSerialize,
    V: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        check_zst::<K>()?;
        // NOTE: BTreeMap iterates over the entries that are sorted by key, so the serialization
        // result will be consistent without a need to sort the entries as we do for HashMap
        // serialization.
        u32::try_from(self.len())
            .map_err(|_| ErrorKind::InvalidData)?
            .serialize(writer)?;
        for (key, value) in self {
            key.serialize(writer)?;
            value.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T> BorshSerialize for BTreeSet<T>
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        check_zst::<T>()?;
        // NOTE: BTreeSet iterates over the items that are sorted, so the serialization result will
        // be consistent without a need to sort the entries as we do for HashSet serialization.
        u32::try_from(self.len())
            .map_err(|_| ErrorKind::InvalidData)?
            .serialize(writer)?;
        for item in self {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl BorshSerialize for std::net::SocketAddr {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            std::net::SocketAddr::V4(ref addr) => {
                0u8.serialize(writer)?;
                addr.serialize(writer)
            }
            std::net::SocketAddr::V6(ref addr) => {
                1u8.serialize(writer)?;
                addr.serialize(writer)
            }
        }
    }
}

#[cfg(feature = "std")]
impl BorshSerialize for std::net::SocketAddrV4 {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.ip().serialize(writer)?;
        self.port().serialize(writer)
    }
}

#[cfg(feature = "std")]
impl BorshSerialize for std::net::SocketAddrV6 {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.ip().serialize(writer)?;
        self.port().serialize(writer)
    }
}

#[cfg(feature = "std")]
impl BorshSerialize for std::net::Ipv4Addr {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.octets())
    }
}

#[cfg(feature = "std")]
impl BorshSerialize for std::net::Ipv6Addr {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.octets())
    }
}

#[cfg(feature = "std")]
impl BorshSerialize for std::net::IpAddr {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            std::net::IpAddr::V4(ipv4) => {
                writer.write_all(&0u8.to_le_bytes())?;
                ipv4.serialize(writer)
            }
            std::net::IpAddr::V6(ipv6) => {
                writer.write_all(&1u8.to_le_bytes())?;
                ipv6.serialize(writer)
            }
        }
    }
}
impl<T: BorshSerialize + ?Sized> BorshSerialize for Box<T> {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.as_ref().serialize(writer)
    }
}

impl<T, const N: usize> BorshSerialize for [T; N]
where
    T: BorshSerialize,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        if N == 0 {
            return Ok(());
        } else if let Some(u8_slice) = T::u8_slice(self) {
            writer.write_all(u8_slice)?;
        } else {
            for el in self.iter() {
                el.serialize(writer)?;
            }
        }
        Ok(())
    }
}

macro_rules! impl_tuple {
    (@unit $name:ty) => {
        impl BorshSerialize for $name {
            #[inline]
            fn serialize<W: Write>(&self, _writer: &mut W) -> Result<()> {
                Ok(())
            }
        }
    };

    ($($idx:tt $name:ident)+) => {
      impl<$($name),+> BorshSerialize for ($($name,)+)
      where $($name: BorshSerialize,)+
      {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            $(self.$idx.serialize(writer)?;)+
            Ok(())
        }
      }
    };
}

impl_tuple!(@unit ());
impl_tuple!(@unit core::ops::RangeFull);

impl_tuple!(0 T0);
impl_tuple!(0 T0 1 T1);
impl_tuple!(0 T0 1 T1 2 T2);
impl_tuple!(0 T0 1 T1 2 T2 3 T3);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19);

macro_rules! impl_range {
    ($type:ident, $this:ident, $($field:expr),*) => {
        impl<T: BorshSerialize> BorshSerialize for core::ops::$type<T> {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
                let $this = self;
                $( $field.serialize(writer)?; )*
                Ok(())
            }
        }
    };
}

impl_range!(Range, this, &this.start, &this.end);
impl_range!(RangeInclusive, this, this.start(), this.end());
impl_range!(RangeFrom, this, &this.start);
impl_range!(RangeTo, this, &this.end);
impl_range!(RangeToInclusive, this, &this.end);

/// Module is available if borsh is built with `features = ["rc"]`.
#[cfg(feature = "rc")]
pub mod rc {
    //!
    //! Module defines [BorshSerialize] implementation for
    //! [alloc::rc::Rc](std::rc::Rc) and [alloc::sync::Arc](std::sync::Arc).
    use crate::__private::maybestd::{rc::Rc, sync::Arc};
    use crate::io::{Result, Write};
    use crate::BorshSerialize;

    /// This impl requires the [`"rc"`] Cargo feature of borsh.
    ///
    /// Serializing a data structure containing `Rc` will serialize a copy of
    /// the contents of the `Rc` each time the `Rc` is referenced within the
    /// data structure. Serialization will not attempt to deduplicate these
    /// repeated data.
    impl<T: BorshSerialize + ?Sized> BorshSerialize for Rc<T> {
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            (**self).serialize(writer)
        }
    }

    /// This impl requires the [`"rc"`] Cargo feature of borsh.
    ///
    /// Serializing a data structure containing `Arc` will serialize a copy of
    /// the contents of the `Arc` each time the `Arc` is referenced within the
    /// data structure. Serialization will not attempt to deduplicate these
    /// repeated data.
    impl<T: BorshSerialize + ?Sized> BorshSerialize for Arc<T> {
        fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
            (**self).serialize(writer)
        }
    }
}

impl<T: ?Sized> BorshSerialize for PhantomData<T> {
    fn serialize<W: Write>(&self, _: &mut W) -> Result<()> {
        Ok(())
    }
}

impl<T> BorshSerialize for core::cell::Cell<T>
where
    T: BorshSerialize + Copy,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        <T as BorshSerialize>::serialize(&self.get(), writer)
    }
}

impl<T> BorshSerialize for core::cell::RefCell<T>
where
    T: BorshSerialize + Sized,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self.try_borrow() {
            Ok(ref value) => value.serialize(writer),
            Err(_) => Err(Error::new(ErrorKind::Other, "already mutably borrowed")),
        }
    }
}
