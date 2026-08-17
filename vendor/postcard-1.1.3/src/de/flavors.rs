//! # Deserialization Flavors
//!
//! "Flavors" in `postcard` are used as modifiers to the serialization or deserialization
//! process. Flavors typically modify one or both of the following:
//!
//! 1. The source medium of the deserialization, e.g. whether the data is serialized from a `[u8]` slice, or some other container
//! 2. The format of the deserialization, such as if the original data is encoded in a COBS format, contains a CRC32 checksum
//!    appended to the message, etc.
//!
//! Flavors are implemented using the [`Flavor`] trait, which acts as a "middleware" for retrieving the bytes before they
//! are passed to `serde` for deserialization
//!
//! Multiple flavors may be combined to obtain a desired combination of behavior and storage.
//! When flavors are combined, it is expected that the storage flavor (such as [`Slice`]) is the innermost flavor.
//!
//! Custom flavors may be defined by users of the `postcard` crate, however some commonly useful flavors have been provided in
//! this module. If you think your custom flavor would be useful to others, PRs adding flavors are very welcome!
//!
//! ## Usability
//!
//! Flavors may not always be convenient to use directly, as they may expose some implementation details of how the
//! inner workings of the flavor behaves. It is typical to provide a convenience method for using a flavor, to prevent
//! the user from having to specify generic parameters, setting correct initialization values, or handling the output of
//! the flavor correctly. See `postcard::from_bytes()` for an example of this.
//!
//! ## When to use (multiple) flavors
//!
//! Combining flavors are nice for convenience, as they perform potentially multiple steps of
//! serialization at one time.
//!
//! This can often be more memory efficient, as intermediate buffers are not typically required.
//!
//! ## When NOT to use (multiple) flavors
//!
//! The downside of passing deserialization through multiple steps is that it is typically slower than
//! performing each step serially. Said simply, "cobs decoding while deserializing" is often slower
//! than "cobs decode then deserialize", due to the ability to handle longer "runs" of data in each
//! stage. The downside is that if these stages can not be performed in-place on the buffer, you
//! will need additional buffers for each stage.
//!
//! Additionally, deserializating flavors can be more restrictive or difficult to work with than
//! serialization flavors, as deserialization may require that the deserialized types borrow some
//! portion of the original message.
//!
//! ## Examples
//!
//! ### Using a single flavor
//!
//! In the first example, we use the `Slice` flavor, to retrieve the serialized output from a `[u8]` slice.
//! No other modification is made to the serialization process.
//!
//! ```rust
//! use postcard::{
//!     de_flavors::Slice,
//!     Deserializer,
//! };
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug, PartialEq)]
//! struct Tup(u8, u8, u8);
//!
//! let msg = [0x04, 0x00, 0x04, 0x01, 0x02, 0x03];
//! let slice = Slice::new(&msg);
//! let mut deserializer = Deserializer::from_flavor(slice);
//! let t = Tup::deserialize(&mut deserializer).unwrap();
//! assert_eq!(t, Tup(4, 0, 4));
//! let remainder = deserializer.finalize().unwrap();
//! assert_eq!(remainder, &[1, 2, 3]);
//! ```

use crate::{Error, Result};
use core::marker::PhantomData;

/// The deserialization Flavor trait
///
/// This is used as the primary way to decode serialized data from some kind of buffer,
/// or modify that data in a middleware style pattern.
///
/// See the module level docs for an example of how flavors are used.
pub trait Flavor<'de>: 'de {
    /// The remaining data of this flavor after deserializing has completed.
    ///
    /// Typically, this includes the remaining buffer that was not used for
    /// deserialization, and in cases of more complex flavors, any additional
    /// information that was decoded or otherwise calculated during
    /// the deserialization process.
    type Remainder: 'de;

    /// The source of data retrieved for deserialization.
    ///
    /// This is typically some sort of data buffer, or another Flavor, when
    /// chained behavior is desired
    type Source: 'de;

    /// Obtain the next byte for deserialization
    fn pop(&mut self) -> Result<u8>;

    /// Returns the number of bytes remaining in the message, if known.
    ///
    /// # Implementation notes
    ///
    /// It is not enforced that this number is exactly correct.
    /// A flavor may yield less or more bytes than the what is hinted at by
    /// this function.
    ///
    /// `size_hint()` is primarily intended to be used for optimizations such as
    /// reserving space for deserialized items, but must not be trusted to
    /// e.g., omit bounds checks in unsafe code. An incorrect implementation of
    /// `size_hint()` should not lead to memory safety violations.
    ///
    /// That said, the implementation should provide a correct estimation,
    /// because otherwise it would be a violation of the trait’s protocol.
    ///
    /// The default implementation returns `None` which is correct for any flavor.
    fn size_hint(&self) -> Option<usize> {
        None
    }

    /// Attempt to take the next `ct` bytes from the serialized message.
    ///
    /// This variant borrows the data from the input for zero-copy deserialization. If zero-copy
    /// deserialization is not necessary, prefer to use `try_take_n_temp` instead.
    fn try_take_n(&mut self, ct: usize) -> Result<&'de [u8]>;

    /// Attempt to take the next `ct` bytes from the serialized message.
    ///
    /// This variant does not guarantee that the returned value is borrowed from the input, so it
    /// cannot be used for zero-copy deserialization, but it also avoids needing to potentially
    /// allocate a data in a temporary buffer.
    ///
    /// This variant should be used instead of `try_take_n`
    /// if zero-copy deserialization is not necessary.
    ///
    /// It is only necessary to implement this method if the flavor requires storing data in a
    /// temporary buffer in order to implement the borrow semantics, e.g. the `std::io::Read`
    /// flavor.
    fn try_take_n_temp<'a>(&'a mut self, ct: usize) -> Result<&'a [u8]>
    where
        'de: 'a,
    {
        self.try_take_n(ct)
    }

    /// Complete the deserialization process.
    ///
    /// This is typically called separately, after the `serde` deserialization
    /// has completed.
    fn finalize(self) -> Result<Self::Remainder>;
}

/// A simple [`Flavor`] representing the deserialization from a borrowed slice
pub struct Slice<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    pub(crate) cursor: *const u8,
    pub(crate) end: *const u8,
    pub(crate) _pl: PhantomData<&'de [u8]>,
}

impl<'de> Slice<'de> {
    /// Create a new [Slice] from the given buffer
    pub fn new(sli: &'de [u8]) -> Self {
        let range = sli.as_ptr_range();
        Self {
            cursor: range.start,
            end: range.end,
            _pl: PhantomData,
        }
    }
}

impl<'de> Flavor<'de> for Slice<'de> {
    type Remainder = &'de [u8];
    type Source = &'de [u8];

    #[inline]
    fn pop(&mut self) -> Result<u8> {
        if self.cursor == self.end {
            Err(Error::DeserializeUnexpectedEnd)
        } else {
            // SAFETY: `self.cursor` is in-bounds and won't be incremented past `self.end` as we
            // have checked above.
            unsafe {
                let res = Ok(*self.cursor);
                self.cursor = self.cursor.add(1);
                res
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some((self.end as usize) - (self.cursor as usize))
    }

    #[inline]
    fn try_take_n(&mut self, ct: usize) -> Result<&'de [u8]> {
        let remain = (self.end as usize) - (self.cursor as usize);
        if remain < ct {
            Err(Error::DeserializeUnexpectedEnd)
        } else {
            // SAFETY: `self.cursor` is valid for `ct` elements and won't be incremented past `self.end` as we
            // have checked above.
            unsafe {
                let sli = core::slice::from_raw_parts(self.cursor, ct);
                self.cursor = self.cursor.add(ct);
                Ok(sli)
            }
        }
    }

    /// Return the remaining (unused) bytes in the Deserializer
    fn finalize(self) -> Result<&'de [u8]> {
        let remain = (self.end as usize) - (self.cursor as usize);
        // SAFETY: `self.cursor` is valid for `remain` elements
        unsafe { Ok(core::slice::from_raw_parts(self.cursor, remain)) }
    }
}

/// Support for [`std::io`] or `embedded-io` traits
#[cfg(any(
    feature = "embedded-io-04",
    feature = "embedded-io-06",
    feature = "use-std"
))]
pub mod io {
    use crate::{Error, Result};
    use core::marker::PhantomData;

    struct SlidingBuffer<'de> {
        cursor: *mut u8,
        end: *const u8,
        _pl: PhantomData<&'de [u8]>,
    }

    impl<'de> SlidingBuffer<'de> {
        pub fn new(sli: &'de mut [u8]) -> Self {
            let range = sli.as_mut_ptr_range();
            Self {
                cursor: range.start,
                end: range.end,
                _pl: PhantomData,
            }
        }

        #[inline]
        fn take_n(&mut self, ct: usize) -> Result<&'de mut [u8]> {
            let remain = (self.end as usize) - (self.cursor as usize);
            let buff = if remain < ct {
                return Err(Error::DeserializeUnexpectedEnd);
            } else {
                // SAFETY: `self.cursor` is valid for `ct` elements and won't be incremented
                // past `self.end` as we have checked above.
                unsafe {
                    let sli = core::slice::from_raw_parts_mut(self.cursor, ct);
                    self.cursor = self.cursor.add(ct);
                    sli
                }
            };

            Ok(buff)
        }

        #[inline]
        fn take_n_temp(&mut self, ct: usize) -> Result<&mut [u8]> {
            let remain = (self.end as usize) - (self.cursor as usize);
            let buff = if remain < ct {
                return Err(Error::DeserializeUnexpectedEnd);
            } else {
                unsafe {
                    let sli = core::slice::from_raw_parts_mut(self.cursor, ct);
                    sli
                }
            };

            Ok(buff)
        }

        fn complete(self) -> Result<&'de mut [u8]> {
            let remain = (self.end as usize) - (self.cursor as usize);
            // SAFETY: `self.cursor` is valid for `remain` elements
            unsafe { Ok(core::slice::from_raw_parts_mut(self.cursor, remain)) }
        }
    }

    /// Support for [`embedded_io`](crate::eio::embedded_io) traits
    #[cfg(any(feature = "embedded-io-04", feature = "embedded-io-06"))]
    pub mod eio {
        use super::super::Flavor;
        use super::SlidingBuffer;
        use crate::{Error, Result};

        /// Wrapper over a [`embedded_io`](crate::eio::embedded_io)::[`Read`](crate::eio::Read) and a sliding buffer to implement the [`Flavor`] trait
        pub struct EIOReader<'de, T>
        where
            T: crate::eio::Read,
        {
            reader: T,
            buff: SlidingBuffer<'de>,
        }

        impl<'de, T> EIOReader<'de, T>
        where
            T: crate::eio::Read,
        {
            /// Create a new [`EIOReader`] from a reader and a buffer.
            ///
            /// `buff` must have enough space to hold all data read during the deserialisation.
            pub fn new(reader: T, buff: &'de mut [u8]) -> Self {
                Self {
                    reader,
                    buff: SlidingBuffer::new(buff),
                }
            }
        }

        impl<'de, T> Flavor<'de> for EIOReader<'de, T>
        where
            T: crate::eio::Read + 'de,
        {
            type Remainder = (T, &'de mut [u8]);
            type Source = &'de [u8];

            #[inline]
            fn pop(&mut self) -> Result<u8> {
                let mut val = [0; 1];
                self.reader
                    .read_exact(&mut val)
                    .map_err(|_| Error::DeserializeUnexpectedEnd)?;
                Ok(val[0])
            }

            #[inline]
            fn size_hint(&self) -> Option<usize> {
                None
            }

            #[inline]
            fn try_take_n(&mut self, ct: usize) -> Result<&'de [u8]> {
                let buff = self.buff.take_n(ct)?;
                self.reader
                    .read_exact(buff)
                    .map_err(|_| Error::DeserializeUnexpectedEnd)?;
                Ok(buff)
            }

            #[inline]
            fn try_take_n_temp<'a>(&'a mut self, ct: usize) -> Result<&'a [u8]>
            where
                'de: 'a,
            {
                let buff = self.buff.take_n_temp(ct)?;
                self.reader
                    .read_exact(buff)
                    .map_err(|_| Error::DeserializeUnexpectedEnd)?;
                Ok(buff)
            }

            /// Return the remaining (unused) bytes in the Deserializer
            fn finalize(self) -> Result<(T, &'de mut [u8])> {
                let buf = self.buff.complete()?;
                Ok((self.reader, buf))
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_pop() {
                let mut reader = EIOReader::new(&[0xAA, 0xBB, 0xCC][..], &mut []);

                assert_eq!(reader.pop(), Ok(0xAA));
                assert_eq!(reader.pop(), Ok(0xBB));
                assert_eq!(reader.pop(), Ok(0xCC));
                assert_eq!(reader.pop(), Err(Error::DeserializeUnexpectedEnd));
            }

            #[test]
            fn test_try_take_n() {
                let mut buf = [0; 8];
                let mut reader = EIOReader::new(&[0xAA, 0xBB, 0xCC, 0xDD, 0xEE][..], &mut buf);

                assert_eq!(reader.try_take_n(2), Ok(&[0xAA, 0xBB][..]));
                assert_eq!(reader.try_take_n(2), Ok(&[0xCC, 0xDD][..]));
                assert_eq!(reader.try_take_n(2), Err(Error::DeserializeUnexpectedEnd));
            }
        }
    }

    /// Support for [`std::io`] traits
    #[allow(clippy::module_inception)]
    #[cfg(feature = "use-std")]
    pub mod io {
        use super::super::Flavor;
        use super::SlidingBuffer;
        use crate::{Error, Result};

        /// Wrapper over a [`std::io::Read`] and a sliding buffer to implement the [Flavor] trait
        pub struct IOReader<'de, T>
        where
            T: std::io::Read,
        {
            reader: T,
            buff: SlidingBuffer<'de>,
        }

        impl<'de, T> IOReader<'de, T>
        where
            T: std::io::Read,
        {
            /// Create a new [`IOReader`] from a reader and a buffer.
            ///
            /// `buff` must have enough space to hold all data read during the deserialisation.
            pub fn new(reader: T, buff: &'de mut [u8]) -> Self {
                Self {
                    reader,
                    buff: SlidingBuffer::new(buff),
                }
            }
        }

        impl<'de, T> Flavor<'de> for IOReader<'de, T>
        where
            T: std::io::Read + 'de,
        {
            type Remainder = (T, &'de mut [u8]);
            type Source = &'de [u8];

            #[inline]
            fn pop(&mut self) -> Result<u8> {
                let mut val = [0; 1];
                self.reader
                    .read_exact(&mut val)
                    .map_err(|_| Error::DeserializeUnexpectedEnd)?;
                Ok(val[0])
            }

            #[inline]
            fn size_hint(&self) -> Option<usize> {
                None
            }

            #[inline]
            fn try_take_n(&mut self, ct: usize) -> Result<&'de [u8]> {
                let buff = self.buff.take_n(ct)?;
                self.reader
                    .read_exact(buff)
                    .map_err(|_| Error::DeserializeUnexpectedEnd)?;
                Ok(buff)
            }

            #[inline]
            fn try_take_n_temp<'a>(&'a mut self, ct: usize) -> Result<&'a [u8]>
            where
                'de: 'a,
            {
                let buff = self.buff.take_n_temp(ct)?;
                self.reader
                    .read_exact(buff)
                    .map_err(|_| Error::DeserializeUnexpectedEnd)?;
                Ok(buff)
            }

            /// Return the remaining (unused) bytes in the Deserializer
            fn finalize(self) -> Result<(T, &'de mut [u8])> {
                let buf = self.buff.complete()?;
                Ok((self.reader, buf))
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_pop() {
                let mut reader = IOReader::new(&[0xAA, 0xBB, 0xCC][..], &mut []);

                assert_eq!(reader.pop(), Ok(0xAA));
                assert_eq!(reader.pop(), Ok(0xBB));
                assert_eq!(reader.pop(), Ok(0xCC));
                assert_eq!(reader.pop(), Err(Error::DeserializeUnexpectedEnd));
            }

            #[test]
            fn test_try_take_n() {
                let mut buf = [0; 8];
                let mut reader = IOReader::new(&[0xAA, 0xBB, 0xCC, 0xDD, 0xEE][..], &mut buf);

                assert_eq!(reader.try_take_n(2), Ok(&[0xAA, 0xBB][..]));
                assert_eq!(reader.try_take_n(2), Ok(&[0xCC, 0xDD][..]));
                assert_eq!(reader.try_take_n(2), Err(Error::DeserializeUnexpectedEnd));
            }
        }
    }
}

////////////////////////////////////////
// CRC
////////////////////////////////////////

/// This Cyclic Redundancy Check flavor applies [the CRC crate's `Algorithm`](https://docs.rs/crc/latest/crc/struct.Algorithm.html) struct on
/// the serialized data.
///
/// The flavor will check the CRC assuming that it has been appended to the bytes.
/// CRCs are used for error detection when reading data back.
/// Requires the `crc` feature.
///
/// More on CRCs: <https://en.wikipedia.org/wiki/Cyclic_redundancy_check>.
#[cfg(feature = "use-crc")]
#[cfg_attr(docsrs, doc(cfg(feature = "use-crc")))]
pub mod crc {
    use core::convert::TryInto;

    use crc::Digest;
    use crc::Width;
    use serde::Deserialize;

    use super::Flavor;
    use super::Slice;

    use crate::Deserializer;
    use crate::Error;
    use crate::Result;

    /// Manages CRC modifications as a flavor.
    pub struct CrcModifier<'de, B, W>
    where
        B: Flavor<'de>,
        W: Width,
    {
        flav: B,
        digest: Digest<'de, W>,
    }

    impl<'de, B, W> CrcModifier<'de, B, W>
    where
        B: Flavor<'de>,
        W: Width,
    {
        /// Create a new Crc modifier Flavor.
        pub fn new(bee: B, digest: Digest<'de, W>) -> Self {
            Self { flav: bee, digest }
        }
    }

    macro_rules! impl_flavor {
        ($int:ty, $from_bytes:ident, $take_from_bytes:ident) => {
            impl<'de, B> Flavor<'de> for CrcModifier<'de, B, $int>
            where
                B: Flavor<'de>,
            {
                type Remainder = B::Remainder;
                type Source = B::Source;

                #[inline]
                fn pop(&mut self) -> Result<u8> {
                    match self.flav.pop() {
                        Ok(byte) => {
                            self.digest.update(&[byte]);
                            Ok(byte)
                        }
                        e @ Err(_) => e,
                    }
                }

                #[inline]
                fn size_hint(&self) -> Option<usize> {
                    self.flav.size_hint()
                }

                #[inline]
                fn try_take_n(&mut self, ct: usize) -> Result<&'de [u8]> {
                    match self.flav.try_take_n(ct) {
                        Ok(bytes) => {
                            self.digest.update(bytes);
                            Ok(bytes)
                        }
                        e @ Err(_) => e,
                    }
                }

                fn finalize(mut self) -> Result<Self::Remainder> {
                    match self.flav.try_take_n(core::mem::size_of::<$int>()) {
                        Ok(prev_crc_bytes) => match self.flav.finalize() {
                            Ok(remainder) => {
                                let crc = self.digest.finalize();
                                let le_bytes = prev_crc_bytes
                                    .try_into()
                                    .map_err(|_| Error::DeserializeBadEncoding)?;
                                let prev_crc = <$int>::from_le_bytes(le_bytes);
                                if crc == prev_crc {
                                    Ok(remainder)
                                } else {
                                    Err(Error::DeserializeBadCrc)
                                }
                            }
                            e @ Err(_) => e,
                        },
                        Err(e) => Err(e),
                    }
                }
            }

            /// Deserialize a message of type `T` from a byte slice with a Crc. The unused portion (if any)
            /// of the byte slice is not returned.
            pub fn $from_bytes<'a, T>(s: &'a [u8], digest: Digest<'a, $int>) -> Result<T>
            where
                T: Deserialize<'a>,
            {
                let flav = CrcModifier::new(Slice::new(s), digest);
                let mut deserializer = Deserializer::from_flavor(flav);
                let r = T::deserialize(&mut deserializer)?;
                let _ = deserializer.finalize()?;
                Ok(r)
            }

            /// Deserialize a message of type `T` from a byte slice with a Crc. The unused portion (if any)
            /// of the byte slice is returned for further usage
            pub fn $take_from_bytes<'a, T>(
                s: &'a [u8],
                digest: Digest<'a, $int>,
            ) -> Result<(T, &'a [u8])>
            where
                T: Deserialize<'a>,
            {
                let flav = CrcModifier::new(Slice::new(s), digest);
                let mut deserializer = Deserializer::from_flavor(flav);
                let t = T::deserialize(&mut deserializer)?;
                Ok((t, deserializer.finalize()?))
            }
        };
    }

    impl_flavor!(u8, from_bytes_u8, take_from_bytes_u8);
    impl_flavor!(u16, from_bytes_u16, take_from_bytes_u16);
    impl_flavor!(u32, from_bytes_u32, take_from_bytes_u32);
    impl_flavor!(u64, from_bytes_u64, take_from_bytes_u64);
    impl_flavor!(u128, from_bytes_u128, take_from_bytes_u128);
}
