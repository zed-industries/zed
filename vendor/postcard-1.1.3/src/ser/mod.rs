use crate::error::{Error, Result};
use crate::ser::flavors::{Cobs, Flavor, Slice};
use serde::Serialize;

#[cfg(feature = "heapless")]
use crate::ser::flavors::HVec;

#[cfg(feature = "heapless")]
use heapless::Vec;

#[cfg(feature = "alloc")]
use crate::ser::flavors::AllocVec;

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::ser::serializer::Serializer;

pub mod flavors;
pub(crate) mod serializer;

/// Serialize a `T` to the given slice, with the resulting slice containing
/// data in a serialized then COBS encoded format. The terminating sentinel
/// `0x00` byte is included in the output buffer.
///
/// When successful, this function returns the slice containing the
/// serialized and encoded message.
///
/// ## Example
///
/// ```rust
/// use postcard::to_slice_cobs;
/// let mut buf = [0u8; 32];
///
/// let used = to_slice_cobs(&false, &mut buf).unwrap();
/// assert_eq!(used, &[0x01, 0x01, 0x00]);
///
/// let used = to_slice_cobs("1", &mut buf).unwrap();
/// assert_eq!(used, &[0x03, 0x01, b'1', 0x00]);
///
/// let used = to_slice_cobs("Hi!", &mut buf).unwrap();
/// assert_eq!(used, &[0x05, 0x03, b'H', b'i', b'!', 0x00]);
///
/// let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
/// let used = to_slice_cobs(data, &mut buf).unwrap();
/// assert_eq!(used, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
/// ```
pub fn to_slice_cobs<'a, 'b, T>(value: &'b T, buf: &'a mut [u8]) -> Result<&'a mut [u8]>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, Cobs<Slice<'a>>, &'a mut [u8]>(
        value,
        Cobs::try_new(Slice::new(buf))?,
    )
}

/// Serialize a `T` to the given slice, with the resulting slice containing
/// data in a serialized format.
///
/// When successful, this function returns the slice containing the
/// serialized message
///
/// ## Example
///
/// ```rust
/// use postcard::to_slice;
/// let mut buf = [0u8; 32];
///
/// let used = to_slice(&true, &mut buf).unwrap();
/// assert_eq!(used, &[0x01]);
///
/// let used = to_slice("Hi!", &mut buf).unwrap();
/// assert_eq!(used, &[0x03, b'H', b'i', b'!']);
///
/// // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
/// let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
/// let used = to_slice(data, &mut buf).unwrap();
/// assert_eq!(used, &[0x04, 0x01, 0x00, 0x20, 0x30]);
///
/// let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
/// let used = to_slice(data, &mut buf).unwrap();
/// assert_eq!(used, &[0x01, 0x00, 0x20, 0x30]);
/// ```
pub fn to_slice<'a, 'b, T>(value: &'b T, buf: &'a mut [u8]) -> Result<&'a mut [u8]>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, Slice<'a>, &'a mut [u8]>(value, Slice::new(buf))
}

/// Serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing
/// data in a serialized then COBS encoded format. The terminating sentinel
/// `0x00` byte is included in the output `Vec`.
///
/// ## Example
///
/// ```rust
/// use postcard::to_vec_cobs;
/// use heapless::Vec;
/// use core::ops::Deref;
///
/// let ser: Vec<u8, 32> = to_vec_cobs(&false).unwrap();
/// assert_eq!(ser.deref(), &[0x01, 0x01, 0x00]);
///
/// let ser: Vec<u8, 32> = to_vec_cobs("Hi!").unwrap();
/// assert_eq!(ser.deref(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);
///
/// // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
/// let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
/// let ser: Vec<u8, 32> = to_vec_cobs(data).unwrap();
/// assert_eq!(ser.deref(), &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
///
/// let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
/// let ser: Vec<u8, 32> = to_vec_cobs(data).unwrap();
/// assert_eq!(ser.deref(), &[0x02, 0x01, 0x03, 0x20, 0x30, 0x00]);
/// ```
#[cfg(feature = "heapless")]
#[cfg_attr(docsrs, doc(cfg(feature = "heapless")))]
pub fn to_vec_cobs<T, const B: usize>(value: &T) -> Result<Vec<u8, B>>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, Cobs<HVec<B>>, Vec<u8, B>>(value, Cobs::try_new(HVec::default())?)
}

/// Serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing
/// data in a serialized format.
///
/// ## Example
///
/// ```rust
/// use postcard::to_vec;
/// use heapless::Vec;
/// use core::ops::Deref;
///
/// let ser: Vec<u8, 32> = to_vec(&true).unwrap();
/// assert_eq!(ser.deref(), &[0x01]);
///
/// let ser: Vec<u8, 32> = to_vec("Hi!").unwrap();
/// assert_eq!(ser.deref(), &[0x03, b'H', b'i', b'!']);
///
/// // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
/// let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
/// let ser: Vec<u8, 32> = to_vec(data).unwrap();
/// assert_eq!(ser.deref(), &[0x04, 0x01, 0x00, 0x20, 0x30]);
///
/// let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
/// let ser: Vec<u8, 32> = to_vec(data).unwrap();
/// assert_eq!(ser.deref(), &[0x01, 0x00, 0x20, 0x30]);
/// ```
#[cfg(feature = "heapless")]
#[cfg_attr(docsrs, doc(cfg(feature = "heapless")))]
pub fn to_vec<T, const B: usize>(value: &T) -> Result<Vec<u8, B>>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, HVec<B>, Vec<u8, B>>(value, HVec::default())
}

/// Serialize a `T` to a `std::vec::Vec<u8>`.
///
/// ## Example
///
/// ```rust
/// use postcard::to_stdvec;
///
/// let ser: Vec<u8> = to_stdvec(&true).unwrap();
/// assert_eq!(ser.as_slice(), &[0x01]);
///
/// let ser: Vec<u8> = to_stdvec("Hi!").unwrap();
/// assert_eq!(ser.as_slice(), &[0x03, b'H', b'i', b'!']);
/// ```
#[cfg(feature = "use-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "use-std")))]
#[inline]
pub fn to_stdvec<T>(value: &T) -> Result<std::vec::Vec<u8>>
where
    T: Serialize + ?Sized,
{
    to_allocvec(value)
}

/// Serialize and COBS encode a `T` to a `std::vec::Vec<u8>`.
///
/// The terminating sentinel `0x00` byte is included in the output.
///
/// ## Example
///
/// ```rust
/// use postcard::to_stdvec_cobs;
///
/// let ser: Vec<u8> = to_stdvec_cobs(&true).unwrap();
/// assert_eq!(ser.as_slice(), &[0x02, 0x01, 0x00]);
///
/// let ser: Vec<u8> = to_stdvec_cobs("Hi!").unwrap();
/// assert_eq!(ser.as_slice(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);
/// ```
#[cfg(feature = "use-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "use-std")))]
#[inline]
pub fn to_stdvec_cobs<T>(value: &T) -> Result<std::vec::Vec<u8>>
where
    T: Serialize + ?Sized,
{
    to_allocvec_cobs(value)
}

/// Serialize a `T` to an `alloc::vec::Vec<u8>`.
///
/// ## Example
///
/// ```rust
/// use postcard::to_allocvec;
///
/// let ser: Vec<u8> = to_allocvec(&true).unwrap();
/// assert_eq!(ser.as_slice(), &[0x01]);
///
/// let ser: Vec<u8> = to_allocvec("Hi!").unwrap();
/// assert_eq!(ser.as_slice(), &[0x03, b'H', b'i', b'!']);
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_allocvec<T>(value: &T) -> Result<alloc::vec::Vec<u8>>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, AllocVec, alloc::vec::Vec<u8>>(value, AllocVec::new())
}

/// Serialize and COBS encode a `T` to an `alloc::vec::Vec<u8>`.
///
/// The terminating sentinel `0x00` byte is included in the output.
///
/// ## Example
///
/// ```rust
/// use postcard::to_allocvec_cobs;
///
/// let ser: Vec<u8> = to_allocvec_cobs(&true).unwrap();
/// assert_eq!(ser.as_slice(), &[0x02, 0x01, 0x00]);
///
/// let ser: Vec<u8> = to_allocvec_cobs("Hi!").unwrap();
/// assert_eq!(ser.as_slice(), &[0x05, 0x03, b'H', b'i', b'!', 0x00]);
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_allocvec_cobs<T>(value: &T) -> Result<alloc::vec::Vec<u8>>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, Cobs<AllocVec>, alloc::vec::Vec<u8>>(
        value,
        Cobs::try_new(AllocVec::new())?,
    )
}

/// Serialize a `T` to a [`core::iter::Extend`],
/// ## Example
///
/// ```rust
/// use postcard::to_extend;
/// let mut vec = Vec::new();
///
/// let ser = to_extend(&true, vec).unwrap();
/// let vec = to_extend("Hi!", ser).unwrap();
/// assert_eq!(&vec[0..5], &[0x01, 0x03, b'H', b'i', b'!']);
/// ```
pub fn to_extend<T, W>(value: &T, writer: W) -> Result<W>
where
    T: Serialize + ?Sized,
    W: core::iter::Extend<u8>,
{
    serialize_with_flavor::<T, _, _>(value, flavors::ExtendFlavor::new(writer))
}

/// Serialize a `T` to an [`embedded_io Write`](crate::eio::Write),
/// ## Example
///
/// ```rust
/// use postcard::to_eio;
/// let mut buf: [u8; 32] = [0; 32];
/// let mut writer: &mut [u8] = &mut buf;
///
/// let ser = to_eio(&true, &mut writer).unwrap();
/// to_eio("Hi!", ser).unwrap();
/// assert_eq!(&buf[0..5], &[0x01, 0x03, b'H', b'i', b'!']);
/// ```
#[cfg(any(feature = "embedded-io-04", feature = "embedded-io-06"))]
pub fn to_eio<T, W>(value: &T, writer: W) -> Result<W>
where
    T: Serialize + ?Sized,
    W: crate::eio::Write,
{
    serialize_with_flavor::<T, _, _>(value, flavors::eio::WriteFlavor::new(writer))
}

/// Serialize a `T` to a [`std::io::Write`],
/// ## Example
///
/// ```rust
/// use postcard::to_io;
/// let mut buf: [u8; 32] = [0; 32];
/// let mut writer: &mut [u8] = &mut buf;
///
/// let ser = to_io(&true, &mut writer).unwrap();
/// to_io("Hi!", ser).unwrap();
/// assert_eq!(&buf[0..5], &[0x01, 0x03, b'H', b'i', b'!']);
/// ```
#[cfg(feature = "use-std")]
pub fn to_io<T, W>(value: &T, writer: W) -> Result<W>
where
    T: Serialize + ?Sized,
    W: std::io::Write,
{
    serialize_with_flavor::<T, _, _>(value, flavors::io::WriteFlavor::new(writer))
}

/// Conveniently serialize a `T` to the given slice, with the resulting slice containing
/// data followed by a 32-bit CRC. The CRC bytes are included in the output buffer.
///
/// When successful, this function returns the slice containing the
/// serialized and encoded message.
///
/// ## Example
///
/// ```rust
/// use crc::{Crc, CRC_32_ISCSI};
///
/// let mut buf = [0; 9];
///
/// let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
/// let crc = Crc::<u32>::new(&CRC_32_ISCSI);
/// let used = postcard::to_slice_crc32(data, &mut buf, crc.digest()).unwrap();
/// assert_eq!(used, &[0x04, 0x01, 0x00, 0x20, 0x30, 0x8E, 0xC8, 0x1A, 0x37]);
/// ```
///
/// See the `ser_flavors::crc` module for the complete set of functions.
#[cfg(feature = "use-crc")]
#[cfg_attr(docsrs, doc(cfg(feature = "use-crc")))]
#[inline]
pub fn to_slice_crc32<'a, T>(
    value: &T,
    buf: &'a mut [u8],
    digest: crc::Digest<'_, u32>,
) -> Result<&'a mut [u8]>
where
    T: Serialize + ?Sized,
{
    flavors::crc::to_slice_u32(value, buf, digest)
}

/// Conveniently serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing
/// data followed by a 32-bit  CRC. The CRC bytes are included in the output `Vec`.
///
/// ## Example
///
/// ```rust
/// use crc::{Crc, CRC_32_ISCSI};
/// use heapless::Vec;
/// use core::ops::Deref;
///
/// // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
/// let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
/// let crc = Crc::<u32>::new(&CRC_32_ISCSI);
/// let ser: Vec<u8, 32> = postcard::to_vec_crc32(data, crc.digest()).unwrap();
/// assert_eq!(ser.deref(), &[0x04, 0x01, 0x00, 0x20, 0x30, 0x8E, 0xC8, 0x1A, 0x37]);
///
/// let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
/// let ser: Vec<u8, 32> = postcard::to_vec_crc32(data, crc.digest()).unwrap();
/// assert_eq!(ser.deref(), &[0x01, 0x00, 0x20, 0x30, 0xCC, 0x4B, 0x4A, 0xDA]);
/// ```
///
/// See the `ser_flavors::crc` module for the complete set of functions.
#[cfg(all(feature = "use-crc", feature = "heapless"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "use-crc", feature = "heapless"))))]
#[inline]
pub fn to_vec_crc32<T, const B: usize>(
    value: &T,
    digest: crc::Digest<'_, u32>,
) -> Result<heapless::Vec<u8, B>>
where
    T: Serialize + ?Sized,
{
    flavors::crc::to_vec_u32(value, digest)
}

/// Conveniently serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing
/// data followed by a 32-bit  CRC. The CRC bytes are included in the output `Vec`.
///
/// ## Example
///
/// ```rust
/// use crc::{Crc, CRC_32_ISCSI};
/// use core::ops::Deref;
///
/// // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
/// let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
/// let crc = Crc::<u32>::new(&CRC_32_ISCSI);
/// let ser: Vec<u8> = postcard::to_stdvec_crc32(data, crc.digest()).unwrap();
/// assert_eq!(ser.deref(), &[0x04, 0x01, 0x00, 0x20, 0x30, 0x8E, 0xC8, 0x1A, 0x37]);
///
/// let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
/// let ser: Vec<u8> = postcard::to_stdvec_crc32(data, crc.digest()).unwrap();
/// assert_eq!(ser.deref(), &[0x01, 0x00, 0x20, 0x30, 0xCC, 0x4B, 0x4A, 0xDA]);
/// ```
///
/// See the `ser_flavors::crc` module for the complete set of functions.
#[cfg(all(feature = "use-crc", feature = "use-std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "use-crc", feature = "use-std"))))]
#[inline]
pub fn to_stdvec_crc32<T>(value: &T, digest: crc::Digest<'_, u32>) -> Result<std::vec::Vec<u8>>
where
    T: Serialize + ?Sized,
{
    flavors::crc::to_allocvec_u32(value, digest)
}

/// Conveniently serialize a `T` to a `heapless::Vec<u8>`, with the `Vec` containing
/// data followed by a 32-bit  CRC. The CRC bytes are included in the output `Vec`.
///
/// ## Example
///
/// ```rust
/// use crc::{Crc, CRC_32_ISCSI};
/// use core::ops::Deref;
///
/// // NOTE: postcard handles `&[u8]` and `&[u8; N]` differently.
/// let data: &[u8] = &[0x01u8, 0x00, 0x20, 0x30];
/// let crc = Crc::<u32>::new(&CRC_32_ISCSI);
/// let ser: Vec<u8> = postcard::to_allocvec_crc32(data, crc.digest()).unwrap();
/// assert_eq!(ser.deref(), &[0x04, 0x01, 0x00, 0x20, 0x30, 0x8E, 0xC8, 0x1A, 0x37]);
///
/// let data: &[u8; 4] = &[0x01u8, 0x00, 0x20, 0x30];
/// let ser: Vec<u8> = postcard::to_allocvec_crc32(data, crc.digest()).unwrap();
/// assert_eq!(ser.deref(), &[0x01, 0x00, 0x20, 0x30, 0xCC, 0x4B, 0x4A, 0xDA]);
/// ```
///
/// See the `ser_flavors::crc` module for the complete set of functions.
#[cfg(all(feature = "use-crc", feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "use-crc", feature = "alloc"))))]
#[inline]
pub fn to_allocvec_crc32<T>(value: &T, digest: crc::Digest<'_, u32>) -> Result<alloc::vec::Vec<u8>>
where
    T: Serialize + ?Sized,
{
    flavors::crc::to_allocvec_u32(value, digest)
}

/// `serialize_with_flavor()` has three generic parameters, `T, F, O`.
///
/// * `T`: This is the type that is being serialized
/// * `S`: This is the Storage that is used during serialization
/// * `O`: This is the resulting storage type that is returned containing the serialized data
///
/// For more information about how Flavors work, please see the
/// [`flavors` module documentation](./flavors/index.html).
///
/// ```rust
/// use postcard::{
///     serialize_with_flavor,
///     ser_flavors::{Cobs, Slice},
/// };
///
/// let mut buf = [0u8; 32];
///
/// let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
/// let buffer = &mut [0u8; 32];
/// let res = serialize_with_flavor::<[u8], Cobs<Slice>, &mut [u8]>(
///     data,
///     Cobs::try_new(Slice::new(buffer)).unwrap(),
/// ).unwrap();
///
/// assert_eq!(res, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
/// ```
pub fn serialize_with_flavor<T, S, O>(value: &T, storage: S) -> Result<O>
where
    T: Serialize + ?Sized,
    S: Flavor<Output = O>,
{
    let mut serializer = Serializer { output: storage };
    value.serialize(&mut serializer)?;
    serializer
        .output
        .finalize()
        .map_err(|_| Error::SerializeBufferFull)
}

/// Compute the size of the postcard serialization of `T`.
pub fn serialized_size<T>(value: &T) -> Result<usize>
where
    T: Serialize + ?Sized,
{
    serialize_with_flavor::<T, flavors::Size, usize>(value, flavors::Size::default())
}

#[cfg(feature = "heapless")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::max_size::MaxSize;
    use crate::varint::{varint_max, varint_usize};
    use core::fmt::Write;
    use core::ops::{Deref, DerefMut};
    use heapless::{FnvIndexMap, String};
    use serde::Deserialize;

    #[test]
    fn ser_u8() {
        let output: Vec<u8, 1> = to_vec(&0x05u8).unwrap();
        assert_eq!(&[5], output.deref());
        assert!(output.len() == serialized_size(&0x05u8).unwrap());
        assert!(output.len() <= Vec::<u8, 1>::POSTCARD_MAX_SIZE);
    }

    #[test]
    fn ser_u16() {
        const SZ: usize = varint_max::<u16>();
        let output: Vec<u8, SZ> = to_vec(&0xA5C7u16).unwrap();
        assert_eq!(&[0xC7, 0xCB, 0x02], output.deref());
        assert!(output.len() == serialized_size(&0xA5C7u16).unwrap());
        assert!(output.len() <= Vec::<u8, SZ>::POSTCARD_MAX_SIZE);
    }

    #[test]
    fn ser_u32() {
        const SZ: usize = varint_max::<u32>();
        let output: Vec<u8, SZ> = to_vec(&0xCDAB3412u32).unwrap();
        assert_eq!(&[0x92, 0xE8, 0xAC, 0xED, 0x0C], output.deref());
        assert!(output.len() == serialized_size(&0xCDAB3412u32).unwrap());
        assert!(output.len() <= Vec::<u8, SZ>::POSTCARD_MAX_SIZE);
    }

    #[test]
    fn ser_u64() {
        const SZ: usize = varint_max::<u64>();
        let output: Vec<u8, SZ> = to_vec(&0x1234_5678_90AB_CDEFu64).unwrap();
        assert_eq!(
            &[0xEF, 0x9B, 0xAF, 0x85, 0x89, 0xCF, 0x95, 0x9A, 0x12],
            output.deref()
        );
        assert!(output.len() == serialized_size(&0x1234_5678_90AB_CDEFu64).unwrap());
        assert!(output.len() <= Vec::<u8, SZ>::POSTCARD_MAX_SIZE);
    }

    #[test]
    fn ser_u128() {
        const SZ: usize = varint_max::<u128>();
        let output: Vec<u8, SZ> = to_vec(&0x1234_5678_90AB_CDEF_1234_5678_90AB_CDEFu128).unwrap();
        assert_eq!(
            &[
                0xEF, 0x9B, 0xAF, 0x85, 0x89, 0xCF, 0x95, 0x9A, 0x92, 0xDE, 0xB7, 0xDE, 0x8A, 0x92,
                0x9E, 0xAB, 0xB4, 0x24,
            ],
            output.deref()
        );
        assert!(
            output.len()
                == serialized_size(&0x1234_5678_90AB_CDEF_1234_5678_90AB_CDEFu128).unwrap()
        );
        assert!(output.len() <= Vec::<u8, SZ>::POSTCARD_MAX_SIZE);
    }

    #[derive(Serialize)]
    struct BasicU8S {
        st: u16,
        ei: u8,
        ote: u128,
        sf: u64,
        tt: u32,
    }

    impl MaxSize for BasicU8S {
        const POSTCARD_MAX_SIZE: usize = {
            u16::POSTCARD_MAX_SIZE
                + u8::POSTCARD_MAX_SIZE
                + u128::POSTCARD_MAX_SIZE
                + u64::POSTCARD_MAX_SIZE
                + u32::POSTCARD_MAX_SIZE
        };
    }

    #[test]
    fn ser_struct_unsigned() {
        const SZ: usize = BasicU8S::POSTCARD_MAX_SIZE;
        let input = BasicU8S {
            st: 0xABCD,
            ei: 0xFE,
            ote: 0x1234_4321_ABCD_DCBA_1234_4321_ABCD_DCBA,
            sf: 0x1234_4321_ABCD_DCBA,
            tt: 0xACAC_ACAC,
        };
        let output: Vec<u8, SZ> = to_vec(&input).unwrap();

        assert_eq!(
            &[
                0xCD, 0xD7, 0x02, 0xFE, 0xBA, 0xB9, 0xB7, 0xDE, 0x9A, 0xE4, 0x90, 0x9A, 0x92, 0xF4,
                0xF2, 0xEE, 0xBC, 0xB5, 0xC8, 0xA1, 0xB4, 0x24, 0xBA, 0xB9, 0xB7, 0xDE, 0x9A, 0xE4,
                0x90, 0x9A, 0x12, 0xAC, 0xD9, 0xB2, 0xE5, 0x0A
            ],
            output.deref()
        );
        assert!(output.len() == serialized_size(&input).unwrap());
        assert!(output.len() <= BasicU8S::POSTCARD_MAX_SIZE);
    }

    #[test]
    fn ser_byte_slice() {
        let input: &[u8] = &[1u8, 2, 3, 4, 5, 6, 7, 8];
        let output: Vec<u8, 9> = to_vec(input).unwrap();
        assert_eq!(
            &[0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            output.deref()
        );
        assert!(output.len() == serialized_size(&input).unwrap());

        let mut input: Vec<u8, 1024> = Vec::new();
        for i in 0..1024 {
            input.push((i & 0xFF) as u8).unwrap();
        }
        let output: Vec<u8, 2048> = to_vec(input.deref()).unwrap();
        assert_eq!(&[0x80, 0x08], &output.deref()[..2]);

        assert_eq!(output.len(), 1026);
        for (i, val) in output.deref()[2..].iter().enumerate() {
            assert_eq!((i & 0xFF) as u8, *val);
        }
    }

    #[test]
    fn ser_str() {
        let input: &str = "hello, postcard!";
        let output: Vec<u8, 17> = to_vec(input).unwrap();
        assert_eq!(0x10, output.deref()[0]);
        assert_eq!(input.as_bytes(), &output.deref()[1..]);
        assert!(output.len() == serialized_size(&input).unwrap());

        let mut input: String<1024> = String::new();
        for _ in 0..256 {
            write!(&mut input, "abcd").unwrap();
        }
        let output: Vec<u8, 2048> = to_vec(input.deref()).unwrap();
        assert_eq!(&[0x80, 0x08], &output.deref()[..2]);
        assert!(String::<1024>::POSTCARD_MAX_SIZE <= output.len());

        assert_eq!(output.len(), 1026);
        for ch in output.deref()[2..].chunks(4) {
            assert_eq!("abcd", core::str::from_utf8(ch).unwrap());
        }
    }

    #[test]
    fn usize_varint_encode() {
        let mut buf = [0; varint_max::<usize>()];
        let res = varint_usize(1, &mut buf);

        assert_eq!(&[1], res);

        let res = varint_usize(usize::MAX, &mut buf);

        //
        if varint_max::<usize>() == varint_max::<u32>() {
            assert_eq!(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F], res);
        } else if varint_max::<usize>() == varint_max::<u64>() {
            assert_eq!(
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
                res
            );
        } else {
            panic!("Update this test for 16/128 bit targets!");
        }
    }

    #[allow(dead_code)]
    #[derive(Serialize)]
    enum BasicEnum {
        Bib,
        Bim,
        Bap,
    }

    #[derive(Serialize)]
    struct EnumStruct {
        eight: u8,
        sixt: u16,
    }

    #[derive(Serialize)]
    enum DataEnum {
        Bib(u16),
        Bim(u64),
        Bap(u8),
        Kim(EnumStruct),
        Chi { a: u8, b: u32 },
        Sho(u16, u8),
    }

    #[test]
    fn enums() {
        let input = BasicEnum::Bim;
        let output: Vec<u8, 1> = to_vec(&input).unwrap();
        assert_eq!(&[0x01], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let input = DataEnum::Bim(u64::MAX);
        let output: Vec<u8, { 1 + varint_max::<u64>() }> = to_vec(&input).unwrap();
        assert_eq!(
            &[0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            output.deref()
        );
        assert!(output.len() == serialized_size(&input).unwrap());

        let input = DataEnum::Bib(u16::MAX);
        let output: Vec<u8, { 1 + varint_max::<u16>() }> = to_vec(&input).unwrap();
        assert_eq!(&[0x00, 0xFF, 0xFF, 0x03], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let input = DataEnum::Bap(u8::MAX);
        let output: Vec<u8, 2> = to_vec(&input).unwrap();
        assert_eq!(&[0x02, 0xFF], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let input = DataEnum::Kim(EnumStruct {
            eight: 0xF0,
            sixt: 0xACAC,
        });
        let output: Vec<u8, 8> = to_vec(&input).unwrap();
        assert_eq!(&[0x03, 0xF0, 0xAC, 0xD9, 0x02], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let input = DataEnum::Chi {
            a: 0x0F,
            b: 0xC7C7C7C7,
        };
        let output: Vec<u8, 8> = to_vec(&input).unwrap();
        assert_eq!(&[0x04, 0x0F, 0xC7, 0x8F, 0x9F, 0xBE, 0x0C], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let input = DataEnum::Sho(0x6969, 0x07);
        let output: Vec<u8, 8> = to_vec(&input).unwrap();
        assert_eq!(&[0x05, 0xE9, 0xD2, 0x01, 0x07], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());
    }

    #[test]
    fn tuples() {
        let input = (1u8, 10u32, "Hello!");
        let output: Vec<u8, 128> = to_vec(&input).unwrap();
        assert_eq!(
            &[1u8, 0x0A, 0x06, b'H', b'e', b'l', b'l', b'o', b'!'],
            output.deref()
        );
        assert!(output.len() == serialized_size(&input).unwrap());
    }

    #[test]
    fn bytes() {
        let x: &[u8; 32] = &[0u8; 32];
        let output: Vec<u8, 128> = to_vec(x).unwrap();
        assert_eq!(output.len(), 32);
        assert!(output.len() == serialized_size(&x).unwrap());
        assert!(<[u8; 32] as MaxSize>::POSTCARD_MAX_SIZE <= output.len());

        let x: &[u8] = &[0u8; 32];
        let output: Vec<u8, 128> = to_vec(x).unwrap();
        assert_eq!(output.len(), 33);
        assert!(output.len() == serialized_size(&x).unwrap());
    }

    #[derive(Serialize)]
    pub struct NewTypeStruct(u32);

    #[derive(Serialize)]
    pub struct TupleStruct((u8, u16));

    #[test]
    fn structs() {
        let input = NewTypeStruct(5);
        let output: Vec<u8, 1> = to_vec(&input).unwrap();
        assert_eq!(&[0x05], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let input = TupleStruct((0xA0, 0x1234));
        let output: Vec<u8, 3> = to_vec(&input).unwrap();
        assert_eq!(&[0xA0, 0xB4, 0x24], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());
    }

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct RefStruct<'a> {
        bytes: &'a [u8],
        str_s: &'a str,
    }

    #[test]
    fn ref_struct() {
        let message = "hElLo";
        let bytes = [0x01, 0x10, 0x02, 0x20];
        let input = RefStruct {
            bytes: &bytes,
            str_s: message,
        };
        let output: Vec<u8, 11> = to_vec(&input).unwrap();

        assert_eq!(
            &[0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',],
            output.deref()
        );
        assert!(output.len() == serialized_size(&input).unwrap());
    }

    #[test]
    fn unit() {
        let output: Vec<u8, 1> = to_vec(&()).unwrap();
        assert_eq!(output.len(), 0);
        assert!(output.len() == serialized_size(&()).unwrap());
    }

    #[test]
    fn heapless_data() {
        let mut input: Vec<u8, 4> = Vec::new();
        input.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]).unwrap();
        let output: Vec<u8, 5> = to_vec(&input).unwrap();
        assert_eq!(&[0x04, 0x01, 0x02, 0x03, 0x04], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let mut input: String<8> = String::new();
        write!(&mut input, "helLO!").unwrap();
        let output: Vec<u8, 7> = to_vec(&input).unwrap();
        assert_eq!(&[0x06, b'h', b'e', b'l', b'L', b'O', b'!'], output.deref());
        assert!(output.len() == serialized_size(&input).unwrap());

        let mut input: FnvIndexMap<u8, u8, 4> = FnvIndexMap::new();
        input.insert(0x01, 0x05).unwrap();
        input.insert(0x02, 0x06).unwrap();
        input.insert(0x03, 0x07).unwrap();
        input.insert(0x04, 0x08).unwrap();
        let output: Vec<u8, 100> = to_vec(&input).unwrap();
        assert_eq!(
            &[0x04, 0x01, 0x05, 0x02, 0x06, 0x03, 0x07, 0x04, 0x08],
            output.deref()
        );
        assert!(output.len() == serialized_size(&input).unwrap());
    }

    #[test]
    fn cobs_test() {
        let message = "hElLo";
        let bytes = [0x01, 0x00, 0x02, 0x20];
        let input = RefStruct {
            bytes: &bytes,
            str_s: message,
        };

        let mut output: Vec<u8, 13> = to_vec_cobs(&input).unwrap();

        let sz = cobs::decode_in_place(output.deref_mut()).unwrap();

        let x = crate::from_bytes::<RefStruct<'_>>(&output.deref_mut()[..sz]).unwrap();

        assert_eq!(input, x);
    }
}
