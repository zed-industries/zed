// Copyright 2017 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits and helpers for bitstream handling functionality
//!
//! Bitstream readers are for reading signed and unsigned integer
//! values from a stream whose sizes may not be whole bytes.
//! Bitstream writers are for writing signed and unsigned integer
//! values to a stream, also potentially un-aligned at a whole byte.
//!
//! Both big-endian and little-endian streams are supported.
//!
//! The only requirement for wrapped reader streams is that they must
//! implement the [`io::Read`] trait, and the only requirement
//! for writer streams is that they must implement the [`io::Write`] trait.
//!
//! In addition, reader streams do not consume any more bytes
//! from the underlying reader than necessary, buffering only a
//! single partial byte as needed.
//! Writer streams also write out all whole bytes as they are accumulated.
//!
//! Readers and writers are also designed to work with integer
//! types of any possible size.
//! Many of Rust's built-in integer types are supported by default.

//! # Minimum Compiler Version
//!
//! Beginning with version 2.4, the minimum compiler version has been
//! updated to Rust 1.79.
//!
//! The issue is that reading an excessive number of
//! bits to a type which is too small to hold them,
//! or writing an excessive number of bits from too small of a type,
//! are always errors:
//! ```
//! use std::io::{Read, Cursor};
//! use bitstream_io::{BigEndian, BitReader, BitRead};
//! let data = [0; 10];
//! let mut r = BitReader::endian(Cursor::new(&data), BigEndian);
//! let x: Result<u32, _> = r.read_var(64);  // reading 64 bits to u32 always fails at runtime
//! assert!(x.is_err());
//! ```
//! but those errors will not be caught until the program runs,
//! which is less than ideal for the common case in which
//! the number of bits is already known at compile-time.
//!
//! But starting with Rust 1.79, we can now have read and write methods
//! which take a constant number of bits and can validate the number of bits
//! are small enough for the type being read/written at compile-time:
//! ```rust,compile_fail
//! use std::io::{Read, Cursor};
//! use bitstream_io::{BigEndian, BitReader, BitRead};
//! let data = [0; 10];
//! let mut r = BitReader::endian(Cursor::new(&data), BigEndian);
//! let x: Result<u32, _> = r.read::<64, _>();  // doesn't compile at all
//! ```
//! Since catching potential bugs at compile-time is preferable
//! to encountering errors at runtime, this will hopefully be
//! an improvement in the long run.

//! # Changes From 3.X.X
//!
//! Version 4.0.0 features significant optimizations to the [`BitRecorder`]
//! and deprecates the [`BitCounter`] in favor of [`BitsWritten`],
//! which no longer requires specifying an endianness.
//!
//! In addition, the [`BitRead::read_bytes`] and [`BitWrite::write_bytes`]
//! methods are significantly optimized in the case of non-aligned
//! reads and writes.
//!
//! Finally, the [`Endianness`] traits have been sealed so as not
//! to be implemented by other packages.  Given that other endianness
//! types are extremely rare in file formats and end users should not
//! have to implement this trait themselves, this should not be a
//! concern.
//!
//! # Changes From 2.X.X
//!
//! Version 3.0.0 has made many breaking changes to the [`BitRead`] and
//! [`BitWrite`] traits.
//!
//! The [`BitRead::read`] method takes a constant number of bits,
//! and the [`BitRead::read_var`] method takes a variable number of bits
//! (reversing the older [`BitRead2::read_in`] and [`BitRead2::read`]
//! calling methods to emphasize using the constant-based one,
//! which can do more validation at compile-time).
//! A new [`BitRead2`] trait uses the older calling convention
//! for compatibility with existing code and is available
//! for anything implementing [`BitRead`].
//!
//! In addition, the main reading methods return primitive types which
//! implement a new [`Integer`] trait,
//! which delegates to [`BitRead::read_unsigned`]
//! or [`BitRead::read_signed`] depending on whether the output
//! is an unsigned or signed type.
//!
//! [`BitWrite::write`] and [`BitWrite::write_var`] work
//! similarly to the reader's `read` methods, taking anything
//! that implements [`Integer`] and writing an unsigned or
//! signed value to [`BitWrite::write_unsigned`] or
//! [`BitWrite::write_signed`] as appropriate.
//!
//! And as with reading, a [`BitWrite2`] trait is offered
//! for compatibility.
//!
//! In addition, the Huffman code handling has been rewritten
//! to use a small amount of macro magic to write
//! code to read and write symbols at compile-time.
//! This is significantly faster than the older version
//! and can no longer fail to compile at runtime.
//!
//! Lastly, there's a new [`BitCount`] struct which wraps a humble
//! `u32` but encodes the maximum possible number of bits
//! at the type level.
//! This is intended for file formats which encode the number
//! of bits to be read in the format itself.
//! For example, FLAC's predictor coefficient precision
//! is a 4 bit value which indicates how large each predictor
//! coefficient is in bits
//! (each coefficient might be an `i32` type).
//! By keeping track of the maximum value at compile time
//! (4 bits' worth, in this case), we can eliminate
//! any need to check that coefficients aren't too large
//! for an `i32` at runtime.
//! This is accomplished by using [`BitRead::read_count`] to
//! read a [`BitCount`] and then reading final values with
//! that number of bits using [`BitRead::read_counted`].

//! # Migrating From Pre 1.0.0
//!
//! There are now [`BitRead`] and [`BitWrite`] traits for bitstream
//! reading and writing (analogous to the standard library's
//! `Read` and `Write` traits) which you will also need to import.
//! The upside to this approach is that library consumers
//! can now make functions and methods generic over any sort
//! of bit reader or bit writer, regardless of the underlying
//! stream byte source or endianness.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
use core2::io;

use core::convert::TryInto;
use core::num::NonZero;
use core::ops::{
    BitAnd, BitOr, BitOrAssign, BitXor, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
};
use core::{fmt::Debug, marker::PhantomData, mem};
#[cfg(feature = "std")]
use std::io;

pub mod huffman;
pub mod read;
pub mod write;
pub use read::{
    BitRead, BitRead2, BitReader, ByteRead, ByteReader, FromBitStream, FromBitStreamUsing,
    FromBitStreamWith, FromByteStream, FromByteStreamUsing, FromByteStreamWith,
};
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
pub use write::BitRecorder;
pub use write::{
    BitWrite, BitWrite2, BitWriter, BitsWritten, ByteWrite, ByteWriter, ToBitStream,
    ToBitStreamUsing, ToBitStreamWith, ToByteStream, ToByteStreamUsing, ToByteStreamWith,
};

#[allow(deprecated)]
pub use write::BitCounter;

/// A trait intended for simple fixed-length primitives (such as ints and floats)
/// which allows them to be read and written to streams of
/// different endiannesses verbatim.
pub trait Primitive {
    /// The raw byte representation of this numeric type
    type Bytes: AsRef<[u8]> + AsMut<[u8]>;

    /// An empty buffer of this type's size
    fn buffer() -> Self::Bytes;

    /// Our value in big-endian bytes
    fn to_be_bytes(self) -> Self::Bytes;

    /// Our value in little-endian bytes
    fn to_le_bytes(self) -> Self::Bytes;

    /// Convert big-endian bytes to our value
    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    /// Convert little-endian bytes to our value
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

macro_rules! define_primitive_numeric {
    ($t:ty) => {
        impl Primitive for $t {
            type Bytes = [u8; mem::size_of::<$t>()];

            #[inline(always)]
            fn buffer() -> Self::Bytes {
                [0; mem::size_of::<$t>()]
            }
            #[inline(always)]
            fn to_be_bytes(self) -> Self::Bytes {
                self.to_be_bytes()
            }
            #[inline(always)]
            fn to_le_bytes(self) -> Self::Bytes {
                self.to_le_bytes()
            }
            #[inline(always)]
            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                <$t>::from_be_bytes(bytes)
            }
            #[inline(always)]
            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                <$t>::from_le_bytes(bytes)
            }
        }
    };
}

impl<const N: usize> Primitive for [u8; N] {
    type Bytes = [u8; N];

    #[inline(always)]
    fn buffer() -> Self::Bytes {
        [0; N]
    }

    #[inline(always)]
    fn to_be_bytes(self) -> Self::Bytes {
        self
    }

    #[inline(always)]
    fn to_le_bytes(self) -> Self::Bytes {
        self
    }

    #[inline(always)]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        bytes
    }

    #[inline(always)]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        bytes
    }
}

/// This trait is for integer types which can be read or written
/// to a bit stream as a partial amount of bits.
///
/// It unifies signed and unsigned integer types by delegating
/// reads and writes to the signed and unsigned reading
/// and writing methods as appropriate.
pub trait Integer {
    /// Reads a value of ourself from the stream
    /// with the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// A compile-time error occurs if the given number of bits
    /// is larger than our type.
    fn read<const BITS: u32, R: BitRead + ?Sized>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized;

    /// Reads a value of ourself from the stream
    /// with the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Also returns an error if our type is too small
    /// to hold the requested number of bits.
    fn read_var<const MAX: u32, R>(reader: &mut R, bits: BitCount<MAX>) -> io::Result<Self>
    where
        R: BitRead + ?Sized,
        Self: Sized;

    /// Writes ourself to the stream using the given const number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if our value is too large
    /// to fit the given number of bits.
    /// A compile-time error occurs if the given number of bits
    /// is larger than our type.
    fn write<const BITS: u32, W: BitWrite + ?Sized>(self, writer: &mut W) -> io::Result<()>;

    /// Writes ourself to the stream using the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if our value is too small
    /// to hold the given number of bits.
    /// Returns an error if our value is too large
    /// to fit the given number of bits.
    fn write_var<const MAX: u32, W: BitWrite + ?Sized>(
        self,
        writer: &mut W,
        bits: BitCount<MAX>,
    ) -> io::Result<()>;
}

/// This trait is for integer types which can be read or written
/// to a bit stream as variable-width integers.
///
/// It unifies signed and unsigned integer types by delegating
/// reads and write to the signed and unsigned vbr reading and
/// writing methods as appropriate.
pub trait VBRInteger: Integer {
    /// Reads a value of ourself from the stream using a variable width integer.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_vbr<const FIELD_SIZE: u32, R>(reader: &mut R) -> io::Result<Self>
    where
        R: BitRead + ?Sized,
        Self: Sized;

    /// Writes ourself to the stream using a variable width integer.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn write_vbr<const FIELD_SIZE: u32, W: BitWrite + ?Sized>(
        self,
        writer: &mut W,
    ) -> io::Result<()>;
}

/// Reading and writing booleans as `Integer` requires the number of bits to be 1.
///
/// This is more useful when combined with the fixed array target
/// for reading blocks of bit flags.
///
/// # Example
/// ```
/// use bitstream_io::{BitReader, BitRead, BigEndian};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Flags {
///     a: bool,
///     b: bool,
///     c: bool,
///     d: bool,
/// }
///
/// let data: &[u8] = &[0b1011_0000];
/// let mut r = BitReader::endian(data, BigEndian);
/// // note the number of bits must be 1 per read
/// // while the quantity of flags is indicated by the array length
/// let flags = r.read::<1, [bool; 4]>().map(|[a, b, c, d]| Flags { a, b, c, d }).unwrap();
/// assert_eq!(flags, Flags { a: true, b: false, c: true, d: true });
/// ```
impl Integer for bool {
    #[inline(always)]
    fn read<const BITS: u32, R: BitRead + ?Sized>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
    {
        const {
            assert!(BITS == 1, "booleans require exactly 1 bit");
        }

        reader.read_bit()
    }

    fn read_var<const MAX: u32, R>(
        reader: &mut R,
        BitCount { bits }: BitCount<MAX>,
    ) -> io::Result<Self>
    where
        R: BitRead + ?Sized,
        Self: Sized,
    {
        if bits == 1 {
            reader.read_bit()
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "booleans require exactly 1 bit",
            ))
        }
    }

    #[inline(always)]
    fn write<const BITS: u32, W: BitWrite + ?Sized>(self, writer: &mut W) -> io::Result<()> {
        const {
            assert!(BITS == 1, "booleans require exactly 1 bit");
        }

        writer.write_bit(self)
    }

    fn write_var<const MAX: u32, W: BitWrite + ?Sized>(
        self,
        writer: &mut W,
        BitCount { bits }: BitCount<MAX>,
    ) -> io::Result<()> {
        if bits == 1 {
            writer.write_bit(self)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "booleans require exactly 1 bit",
            ))
        }
    }
}

impl<const SIZE: usize, I: Integer + Copy + Default> Integer for [I; SIZE] {
    #[inline]
    fn read<const BITS: u32, R: BitRead + ?Sized>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
    {
        let mut a = [I::default(); SIZE];

        a.iter_mut().try_for_each(|v| {
            *v = reader.read::<BITS, I>()?;
            Ok::<(), io::Error>(())
        })?;

        Ok(a)
    }

    #[inline]
    fn read_var<const MAX: u32, R>(reader: &mut R, count: BitCount<MAX>) -> io::Result<Self>
    where
        R: BitRead + ?Sized,
        Self: Sized,
    {
        let mut a = [I::default(); SIZE];

        a.iter_mut().try_for_each(|v| {
            *v = reader.read_counted(count)?;
            Ok::<(), io::Error>(())
        })?;

        Ok(a)
    }

    #[inline]
    fn write<const BITS: u32, W: BitWrite + ?Sized>(self, writer: &mut W) -> io::Result<()> {
        IntoIterator::into_iter(self).try_for_each(|v| writer.write::<BITS, I>(v))
    }

    #[inline]
    fn write_var<const MAX: u32, W: BitWrite + ?Sized>(
        self,
        writer: &mut W,
        count: BitCount<MAX>,
    ) -> io::Result<()> {
        IntoIterator::into_iter(self).try_for_each(|v| writer.write_counted(count, v))
    }
}

impl<const SIZE: usize, I: VBRInteger + Copy + Default> VBRInteger for [I; SIZE] {
    fn read_vbr<const FIELD_SIZE: u32, R>(reader: &mut R) -> io::Result<Self>
    where
        R: BitRead + ?Sized,
        Self: Sized,
    {
        let mut a = [I::default(); SIZE];

        a.iter_mut().try_for_each(|v| {
            I::read_vbr::<FIELD_SIZE, R>(reader).map(|item| {
                *v = item;
            })
        })?;

        Ok(a)
    }

    fn write_vbr<const FIELD_SIZE: u32, W: BitWrite + ?Sized>(
        self,
        writer: &mut W,
    ) -> io::Result<()> {
        IntoIterator::into_iter(self).try_for_each(|v| I::write_vbr::<FIELD_SIZE, W>(v, writer))
    }
}

/// This trait extends many common integer types (both unsigned and signed)
/// with a few trivial methods so that they can be used
/// with the bitstream handling traits.
pub trait Numeric:
    Primitive
    + Sized
    + Copy
    + Default
    + Debug
    + PartialOrd
    + Shl<u32, Output = Self>
    + ShlAssign<u32>
    + Shr<u32, Output = Self>
    + ShrAssign<u32>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitOrAssign<Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
    + Sub<Self, Output = Self>
{
    /// Size of type in bits
    const BITS_SIZE: u32;

    /// The value of 0 in this type
    const ZERO: Self;

    /// The value of 1 in this type
    const ONE: Self;

    /// Returns a `u8` value in this type
    fn from_u8(u: u8) -> Self;

    /// Assuming 0 <= value < 256, returns this value as a `u8` type
    fn to_u8(self) -> u8;
}

macro_rules! define_numeric {
    ($t:ty) => {
        define_primitive_numeric!($t);

        impl Numeric for $t {
            const BITS_SIZE: u32 = mem::size_of::<$t>() as u32 * 8;

            const ZERO: Self = 0;

            const ONE: Self = 1;

            #[inline(always)]
            fn from_u8(u: u8) -> Self {
                u as $t
            }
            #[inline(always)]
            fn to_u8(self) -> u8 {
                self as u8
            }
        }
    };
}

/// This trait extends many common unsigned integer types
/// so that they can be used with the bitstream handling traits.
pub trait UnsignedInteger: Numeric {
    /// This type's most-significant bit
    const MSB_BIT: Self;

    /// This type's least significant bit
    const LSB_BIT: Self;

    /// This type with all bits set
    const ALL: Self;

    /// The signed variant of ourself
    type Signed: SignedInteger<Unsigned = Self>;

    /// Given a twos-complement value,
    /// return this value is a non-negative signed number.
    /// The location of the sign bit depends on the stream's endianness
    /// and is not stored in the result.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::UnsignedInteger;
    /// assert_eq!(0b00000001u8.as_non_negative(), 1i8);
    /// ```
    fn as_non_negative(self) -> Self::Signed;

    /// Given a two-complement positive value and certain number of bits,
    /// returns this value as a negative signed number.
    /// The location of the sign bit depends on the stream's endianness
    /// and is not stored in the result.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::UnsignedInteger;
    /// assert_eq!(0b01111111u8.as_negative(8), -1i8);
    /// ```
    fn as_negative(self, bits: u32) -> Self::Signed;

    /// Given a two-complement positive value and certain number of bits,
    /// returns this value as a negative number.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::UnsignedInteger;
    /// assert_eq!(0b01111111u8.as_negative_fixed::<8>(), -1i8);
    /// ```
    fn as_negative_fixed<const BITS: u32>(self) -> Self::Signed;

    /// Checked shift left
    fn checked_shl(self, rhs: u32) -> Option<Self>;

    /// Checked shift right
    fn checked_shr(self, rhs: u32) -> Option<Self>;

    /// Shift left up to our length in bits
    ///
    /// If rhs equals our length in bits, returns default
    fn shl_default(self, rhs: u32) -> Self {
        self.checked_shl(rhs).unwrap_or(Self::ZERO)
    }

    /// Shift left up to our length in bits
    ///
    /// If rhs equals our length in bits, returns zero
    fn shr_default(self, rhs: u32) -> Self {
        self.checked_shr(rhs).unwrap_or(Self::ZERO)
    }
}

macro_rules! define_unsigned_integer {
    ($t:ty, $s:ty) => {
        define_numeric!($t);

        impl UnsignedInteger for $t {
            type Signed = $s;

            const MSB_BIT: Self = 1 << (Self::BITS_SIZE - 1);

            const LSB_BIT: Self = 1;

            const ALL: Self = <$t>::MAX;

            #[inline(always)]
            fn as_non_negative(self) -> Self::Signed {
                self as $s
            }
            #[inline(always)]
            fn as_negative(self, bits: u32) -> Self::Signed {
                (self as $s) + (-1 << (bits - 1))
            }
            #[inline(always)]
            fn as_negative_fixed<const BITS: u32>(self) -> Self::Signed {
                (self as $s) + (-1 << (BITS - 1))
            }
            #[inline(always)]
            fn checked_shl(self, rhs: u32) -> Option<Self> {
                self.checked_shl(rhs)
            }
            #[inline(always)]
            fn checked_shr(self, rhs: u32) -> Option<Self> {
                self.checked_shr(rhs)
            }
            // TODO - enable these in the future
            // #[inline(always)]
            // fn shl_default(self, rhs: u32) -> Self {
            //     self.unbounded_shl(rhs)
            // }
            // #[inline(always)]
            // fn shr_default(self, rhs: u32) -> Self {
            //     self.unbounded_shr(rhs)
            // }
        }

        impl Integer for $t {
            #[inline(always)]
            fn read<const BITS: u32, R: BitRead + ?Sized>(reader: &mut R) -> io::Result<Self>
            where
                Self: Sized,
            {
                reader.read_unsigned::<BITS, _>()
            }

            #[inline(always)]
            fn read_var<const MAX: u32, R>(reader: &mut R, bits: BitCount<MAX>) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                reader.read_unsigned_counted::<MAX, _>(bits)
            }

            #[inline(always)]
            fn write<const BITS: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                writer.write_unsigned::<BITS, _>(self)
            }

            #[inline(always)]
            fn write_var<const MAX: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
                bits: BitCount<MAX>,
            ) -> io::Result<()> {
                writer.write_unsigned_counted(bits, self)
            }
        }

        impl VBRInteger for $t {
            #[inline(always)]
            fn read_vbr<const FIELD_SIZE: u32, R>(reader: &mut R) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                reader.read_unsigned_vbr::<FIELD_SIZE, _>()
            }

            #[inline(always)]
            fn write_vbr<const FIELD_SIZE: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                writer.write_unsigned_vbr::<FIELD_SIZE, _>(self)
            }
        }

        /// Unsigned NonZero types increment their value by 1
        /// when being read and decrement it by 1
        /// when being written.
        ///
        /// # Examples
        /// ```
        /// use bitstream_io::{BitReader, BitRead, BigEndian};
        /// use core::num::NonZero;
        ///
        /// let data: &[u8] = &[0b001_00000];
        /// // reading a regular u8 in 3 bits yields 1
        /// assert_eq!(BitReader::endian(data, BigEndian).read::<3, u8>().unwrap(), 1);
        /// // reading a NonZero<u8> in 3 bits of the same data yields 2
        /// assert_eq!(BitReader::endian(data, BigEndian).read::<3, NonZero<u8>>().unwrap().get(), 2);
        /// ```
        ///
        /// ```
        /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
        /// use core::num::NonZero;
        ///
        /// let mut w = BitWriter::endian(vec![], BigEndian);
        /// // writing 1 as a regular u8 in 3 bits
        /// w.write::<3, u8>(1).unwrap();
        /// w.byte_align();
        /// assert_eq!(w.into_writer(), &[0b001_00000]);
        ///
        /// let mut w = BitWriter::endian(vec![], BigEndian);
        /// // writing 1 as a NonZero<u8> in 3 bits
        /// w.write::<3, NonZero<u8>>(NonZero::new(1).unwrap()).unwrap();
        /// w.byte_align();
        /// assert_eq!(w.into_writer(), &[0b000_00000]);
        /// ```
        impl Integer for NonZero<$t> {
            #[inline]
            fn read<const BITS: u32, R: BitRead + ?Sized>(reader: &mut R) -> io::Result<Self>
            where
                Self: Sized,
            {
                const {
                    assert!(
                        BITS < <$t>::BITS_SIZE,
                        "BITS must be less than the type's size in bits"
                    );
                }

                <$t as Integer>::read::<BITS, R>(reader).map(|u| NonZero::new(u + 1).unwrap())
            }

            #[inline]
            fn read_var<const MAX: u32, R>(
                reader: &mut R,
                count @ BitCount { bits }: BitCount<MAX>,
            ) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                if MAX < <$t>::BITS_SIZE || bits < <$t>::BITS_SIZE {
                    <$t as Integer>::read_var::<MAX, R>(reader, count)
                        .map(|u| NonZero::new(u + 1).unwrap())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "bit count must be less than the type's size in bits",
                    ))
                }
            }

            #[inline]
            fn write<const BITS: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                const {
                    assert!(
                        BITS < <$t>::BITS_SIZE,
                        "BITS must be less than the type's size in bits"
                    );
                }

                <$t as Integer>::write::<BITS, W>(self.get() - 1, writer)
            }

            #[inline]
            fn write_var<const MAX: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
                count @ BitCount { bits }: BitCount<MAX>,
            ) -> io::Result<()> {
                if MAX < <$t>::BITS_SIZE || bits < <$t>::BITS_SIZE {
                    <$t as Integer>::write_var::<MAX, W>(self.get() - 1, writer, count)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "bit count must be less than the type's size in bits",
                    ))
                }
            }
        }

        impl VBRInteger for NonZero<$t> {
            #[inline]
            fn read_vbr<const FIELD_SIZE: u32, R>(reader: &mut R) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                <$t as VBRInteger>::read_vbr::<FIELD_SIZE, R>(reader)
                    .map(|u| NonZero::new(u + 1).unwrap())
            }

            #[inline]
            fn write_vbr<const FIELD_SIZE: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                <$t as VBRInteger>::write_vbr::<FIELD_SIZE, W>(self.get() - 1, writer)
            }
        }

        impl Integer for Option<NonZero<$t>> {
            #[inline]
            fn read<const BITS: u32, R: BitRead + ?Sized>(reader: &mut R) -> io::Result<Self>
            where
                Self: Sized,
            {
                <$t as Integer>::read::<BITS, R>(reader).map(NonZero::new)
            }

            #[inline]
            fn read_var<const MAX: u32, R>(reader: &mut R, count: BitCount<MAX>) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                <$t as Integer>::read_var::<MAX, R>(reader, count).map(NonZero::new)
            }

            #[inline]
            fn write<const BITS: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                <$t as Integer>::write::<BITS, W>(self.map(|n| n.get()).unwrap_or(0), writer)
            }

            #[inline]
            fn write_var<const MAX: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
                count: BitCount<MAX>,
            ) -> io::Result<()> {
                <$t as Integer>::write_var::<MAX, W>(
                    self.map(|n| n.get()).unwrap_or(0),
                    writer,
                    count,
                )
            }
        }

        impl VBRInteger for Option<NonZero<$t>> {
            #[inline(always)]
            fn read_vbr<const FIELD_SIZE: u32, R>(reader: &mut R) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                <$t as VBRInteger>::read_vbr::<FIELD_SIZE, _>(reader).map(NonZero::new)
            }

            #[inline]
            fn write_vbr<const FIELD_SIZE: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                <$t as VBRInteger>::write_vbr::<FIELD_SIZE, W>(
                    self.map(|n| n.get()).unwrap_or(0),
                    writer,
                )
            }
        }
    };
}

/// This trait extends many common signed integer types
/// so that they can be used with the bitstream handling traits.
///
/// This trait was formerly named `SignedNumeric` in 2.X.X code.
/// If backwards-compatibility is needed one can
/// import `SignedInteger` as `SignedNumeric`.
pub trait SignedInteger: Numeric {
    /// The unsigned variant of ourself
    type Unsigned: UnsignedInteger<Signed = Self>;

    /// Returns true if this value is negative
    ///
    /// # Example
    /// ```
    /// use bitstream_io::SignedInteger;
    /// assert!(!1i8.is_negative());
    /// assert!((-1i8).is_negative());
    /// ```
    fn is_negative(self) -> bool;

    /// Returns ourself as a non-negative value.
    /// The location of the sign bit depends on the stream's endianness
    /// and is not stored in the result.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::SignedInteger;
    /// assert_eq!(1i8.as_non_negative(), 0b00000001u8);
    /// ```
    fn as_non_negative(self) -> Self::Unsigned;

    /// Given a negative value and a certain number of bits,
    /// returns this value as a twos-complement positive number.
    /// The location of the sign bit depends on the stream's endianness
    /// and is not stored in the result.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::SignedInteger;
    /// assert_eq!((-1i8).as_negative(8), 0b01111111u8);
    /// ```
    fn as_negative(self, bits: u32) -> Self::Unsigned;

    /// Given a negative value and a certain number of bits,
    /// returns this value as a twos-complement positive number.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::SignedInteger;
    /// assert_eq!((-1i8).as_negative_fixed::<8>(), 0b01111111u8);
    /// ```
    fn as_negative_fixed<const BITS: u32>(self) -> Self::Unsigned;
}

macro_rules! define_signed_integer {
    ($t:ty, $u:ty) => {
        define_numeric!($t);

        impl SignedInteger for $t {
            type Unsigned = $u;

            #[inline(always)]
            fn is_negative(self) -> bool {
                self.is_negative()
            }
            fn as_non_negative(self) -> Self::Unsigned {
                self as $u
            }
            fn as_negative(self, bits: u32) -> Self::Unsigned {
                (self - (-1 << (bits - 1))) as $u
            }
            fn as_negative_fixed<const BITS: u32>(self) -> Self::Unsigned {
                (self - (-1 << (BITS - 1))) as $u
            }
        }

        impl Integer for $t {
            #[inline(always)]
            fn read<const BITS: u32, R: BitRead + ?Sized>(reader: &mut R) -> io::Result<Self>
            where
                Self: Sized,
            {
                reader.read_signed::<BITS, _>()
            }

            #[inline(always)]
            fn read_var<const MAX: u32, R>(reader: &mut R, bits: BitCount<MAX>) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                reader.read_signed_counted::<MAX, _>(bits)
            }

            #[inline(always)]
            fn write<const BITS: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                writer.write_signed::<BITS, _>(self)
            }

            #[inline(always)]
            fn write_var<const MAX: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
                bits: BitCount<MAX>,
            ) -> io::Result<()> {
                writer.write_signed_counted::<MAX, _>(bits, self)
            }
        }

        impl VBRInteger for $t {
            #[inline(always)]
            fn read_vbr<const FIELD_SIZE: u32, R>(reader: &mut R) -> io::Result<Self>
            where
                R: BitRead + ?Sized,
                Self: Sized,
            {
                reader.read_signed_vbr::<FIELD_SIZE, _>()
            }

            #[inline(always)]
            fn write_vbr<const FIELD_SIZE: u32, W: BitWrite + ?Sized>(
                self,
                writer: &mut W,
            ) -> io::Result<()> {
                writer.write_signed_vbr::<FIELD_SIZE, _>(self)
            }
        }
    };
}

define_unsigned_integer!(u8, i8);
define_unsigned_integer!(u16, i16);
define_unsigned_integer!(u32, i32);
define_unsigned_integer!(u64, i64);
define_unsigned_integer!(u128, i128);

define_signed_integer!(i8, u8);
define_signed_integer!(i16, u16);
define_signed_integer!(i32, u32);
define_signed_integer!(i64, u64);
define_signed_integer!(i128, u128);

define_primitive_numeric!(f32);
define_primitive_numeric!(f64);

mod private {
    use crate::{
        io, BitCount, BitRead, BitWrite, CheckedSigned, CheckedUnsigned, Primitive, SignedBitCount,
        SignedInteger, UnsignedInteger,
    };

    #[test]
    fn test_checked_signed() {
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<8>(), -128i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<8>(), 127i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<7>(), -64i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<7>(), 63i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<6>(), -32i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<6>(), 31i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<5>(), -16i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<5>(), 15i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<4>(), -8i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<4>(), 7i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<3>(), -4i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<3>(), 3i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<2>(), -2i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<2>(), 1i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<1>(), -1i8).is_ok());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<1>(), 0i8).is_ok());

        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<7>(), -65i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<7>(), 64i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<6>(), -33i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<6>(), 32i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<5>(), -17i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<5>(), 16i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<4>(), -9i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<4>(), 8i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<3>(), -5i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<3>(), 4i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<2>(), -3i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<2>(), 2i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<1>(), -2i8).is_err());
        assert!(CheckedSigned::new(SignedBitCount::<8>::new::<1>(), 1i8).is_err());
    }

    pub trait Endianness: Sized {
        /// Pops the next bit from the queue,
        /// repleneshing it from the given reader if necessary
        fn pop_bit_refill<R>(
            reader: &mut R,
            queue_value: &mut u8,
            queue_bits: &mut u32,
        ) -> io::Result<bool>
        where
            R: io::Read;

        /// Pops the next unary value from the source until
        /// `STOP_BIT` is encountered, replenishing it from the given
        /// closure if necessary.
        ///
        /// `STOP_BIT` must be 0 or 1.
        fn pop_unary<const STOP_BIT: u8, R>(
            reader: &mut R,
            queue_value: &mut u8,
            queue_bits: &mut u32,
        ) -> io::Result<u32>
        where
            R: io::Read;

        /// Pushes the next bit into the queue,
        /// and returns `Some` value if the queue is full.
        fn push_bit_flush(queue_value: &mut u8, queue_bits: &mut u32, bit: bool) -> Option<u8>;

        /// For performing bulk reads from a bit source to an output type.
        fn read_bits<const MAX: u32, R, U>(
            reader: &mut R,
            queue_value: &mut u8,
            queue_bits: &mut u32,
            count: BitCount<MAX>,
        ) -> io::Result<U>
        where
            R: io::Read,
            U: UnsignedInteger;

        /// For performing bulk reads from a bit source to an output type.
        fn read_bits_fixed<const BITS: u32, R, U>(
            reader: &mut R,
            queue_value: &mut u8,
            queue_bits: &mut u32,
        ) -> io::Result<U>
        where
            R: io::Read,
            U: UnsignedInteger;

        /// For performing a checked write to a bit sink
        fn write_bits_checked<const MAX: u32, W, U>(
            writer: &mut W,
            queue_value: &mut u8,
            queue_bits: &mut u32,
            value: CheckedUnsigned<MAX, U>,
        ) -> io::Result<()>
        where
            W: io::Write,
            U: UnsignedInteger;

        /// For performing a checked signed write to a bit sink
        fn write_signed_bits_checked<const MAX: u32, W, S>(
            writer: &mut W,
            queue_value: &mut u8,
            queue_bits: &mut u32,
            value: CheckedSigned<MAX, S>,
        ) -> io::Result<()>
        where
            W: io::Write,
            S: SignedInteger;

        /// Reads signed value from reader in this endianness
        fn read_signed_counted<const MAX: u32, R, S>(
            r: &mut R,
            bits: SignedBitCount<MAX>,
        ) -> io::Result<S>
        where
            R: BitRead,
            S: SignedInteger;

        /// Reads whole set of bytes to output buffer
        fn read_bytes<const CHUNK_SIZE: usize, R>(
            reader: &mut R,
            queue_value: &mut u8,
            queue_bits: u32,
            buf: &mut [u8],
        ) -> io::Result<()>
        where
            R: io::Read;

        /// Writes whole set of bytes to output buffer
        fn write_bytes<const CHUNK_SIZE: usize, W>(
            writer: &mut W,
            queue_value: &mut u8,
            queue_bits: u32,
            buf: &[u8],
        ) -> io::Result<()>
        where
            W: io::Write;

        /// Converts a primitive's byte buffer to a primitive
        fn bytes_to_primitive<P: Primitive>(buf: P::Bytes) -> P;

        /// Converts a primitive to a primitive's byte buffer
        fn primitive_to_bytes<P: Primitive>(p: P) -> P::Bytes;

        /// Reads convertable numeric value from reader in this endianness
        #[deprecated(since = "4.0.0")]
        fn read_primitive<R, V>(r: &mut R) -> io::Result<V>
        where
            R: BitRead,
            V: Primitive;

        /// Writes convertable numeric value to writer in this endianness
        #[deprecated(since = "4.0.0")]
        fn write_primitive<W, V>(w: &mut W, value: V) -> io::Result<()>
        where
            W: BitWrite,
            V: Primitive;
    }

    pub trait Checkable {
        fn write_endian<E, W>(
            self,
            writer: &mut W,
            queue_value: &mut u8,
            queue_bits: &mut u32,
        ) -> io::Result<()>
        where
            E: Endianness,
            W: io::Write;
    }
}

/// A stream's endianness, or byte order, for determining
/// how bits should be read.
///
/// It comes in `BigEndian` and `LittleEndian` varieties
/// (which may be shortened to `BE` and `LE`)
/// and is not something programmers should implement directly.
pub trait Endianness: private::Endianness {}

#[inline(always)]
fn read_byte<R>(mut reader: R) -> io::Result<u8>
where
    R: io::Read,
{
    let mut byte = 0;
    reader
        .read_exact(core::slice::from_mut(&mut byte))
        .map(|()| byte)
}

#[inline(always)]
fn write_byte<W>(mut writer: W, byte: u8) -> io::Result<()>
where
    W: io::Write,
{
    writer.write_all(core::slice::from_ref(&byte))
}

/// A number of bits to be consumed or written, with a known maximum
///
/// Although [`BitRead::read`] and [`BitWrite::write`] should be
/// preferred when the number of bits is fixed and known at compile-time -
/// because they can validate the bit count is less than or equal
/// to the type's size in bits at compile-time -
/// there are many instances where bit count is dynamic and
/// determined by the file format itself.
/// But when using [`BitRead::read_var`] or [`BitWrite::write_var`]
/// we must pessimistically assume any number of bits as an argument
/// and validate that the number of bits is not larger than the
/// type being read or written on every call.
///
/// ```
/// use bitstream_io::{BitRead, BitReader, BigEndian};
///
/// let data: &[u8] = &[0b100_0001_1, 0b111_0110_0];
/// let mut r = BitReader::endian(data, BigEndian);
/// // our bit count is a 3 bit value
/// let count = r.read::<3, u32>().unwrap();
/// // that count indicates we need to read 4 bits (0b100)
/// assert_eq!(count, 4);
/// // read the first 4-bit value
/// assert_eq!(r.read_var::<u8>(count).unwrap(), 0b0001);
/// // read the second 4-bit value
/// assert_eq!(r.read_var::<u8>(count).unwrap(), 0b1111);
/// // read the third 4-bit value
/// assert_eq!(r.read_var::<u8>(count).unwrap(), 0b0110);
/// ```
///
/// In the preceding example, even though we know `count` is a
/// 3 bit value whose maximum value will never be greater than 7,
/// the subsequent `read_var` calls have no way to know that.
/// They must assume `count` could be 9, or `u32::MAX` or any other `u32` value
/// and validate the count is not larger than the `u8` types we're reading.
///
/// But we can convert our example to use the `BitCount` type:
///
/// ```
/// use bitstream_io::{BitRead, BitReader, BigEndian, BitCount};
///
/// let data: &[u8] = &[0b100_0001_1, 0b111_0110_0];
/// let mut r = BitReader::endian(data, BigEndian);
/// // our bit count is a 3 bit value with a maximum value of 7
/// let count = r.read_count::<0b111>().unwrap();
/// // that count indicates we need to read 4 bits (0b100)
/// assert_eq!(count, BitCount::<7>::new::<4>());
/// // read the first 4-bit value
/// assert_eq!(r.read_counted::<7, u8>(count).unwrap(), 0b0001);
/// // read the second 4-bit value
/// assert_eq!(r.read_counted::<7, u8>(count).unwrap(), 0b1111);
/// // read the third 4-bit value
/// assert_eq!(r.read_counted::<7, u8>(count).unwrap(), 0b0110);
/// ```
///
/// Because the [`BitRead::read_counted`] methods know at compile-time
/// that the bit count will be larger than 7, that check can be eliminated
/// simply by taking advantage of information we already know.
///
/// Leveraging the `BitCount` type also allows us to reason about
/// bit counts in a more formal way, and use checked permutation methods
/// to modify them while ensuring they remain constrained by
/// the file format's requirements.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct BitCount<const MAX: u32> {
    // The amount of bits may be less than or equal to the maximum,
    // but never more.
    bits: u32,
}

impl<const MAX: u32> BitCount<MAX> {
    /// Builds a bit count from a constant number
    /// of bits, which must not be greater than `MAX`.
    ///
    /// Intended to be used for defining constants.
    ///
    /// Use `TryFrom` to conditionally build
    /// counts from values at runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian, BitCount};
    /// let data: &[u8] = &[0b111_00000];
    /// let mut r = BitReader::endian(data, BigEndian);
    /// // reading 3 bits from a stream out of a maximum of 8
    /// // doesn't require checking that the bit count is larger
    /// // than a u8 at runtime because specifying the maximum of 8
    /// // guarantees our bit count will not be larger than 8
    /// assert_eq!(r.read_counted::<8, u8>(BitCount::new::<3>()).unwrap(), 0b111);
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::BitCount;
    /// // trying to build a count of 10 with a maximum of 8
    /// // fails to compile at all
    /// let count = BitCount::<8>::new::<10>();
    /// ```
    pub const fn new<const BITS: u32>() -> Self {
        const {
            assert!(BITS <= MAX, "BITS must be <= MAX");
        }

        Self { bits: BITS }
    }

    /// Add a number of bits to our count,
    /// returning a new count with a new maximum.
    ///
    /// Returns `None` if the new count goes above our new maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// let count = BitCount::<2>::new::<1>();
    /// // adding 2 to 1 and increasing the max to 3 yields a new count of 3
    /// assert_eq!(count.checked_add::<3>(2), Some(BitCount::<3>::new::<3>()));
    /// // adding 2 to 1 without increasing the max yields None
    /// assert_eq!(count.checked_add::<2>(2), None);
    /// ```
    #[inline]
    pub const fn checked_add<const NEW_MAX: u32>(self, bits: u32) -> Option<BitCount<NEW_MAX>> {
        match self.bits.checked_add(bits) {
            Some(bits) if bits <= NEW_MAX => Some(BitCount { bits }),
            _ => None,
        }
    }

    /// Subtracts a number of bits from our count,
    /// returning a new count with a new maximum.
    ///
    /// Returns `None` if the new count goes below 0
    /// or below our new maximum.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::BitCount;
    /// let count = BitCount::<5>::new::<5>();
    /// // subtracting 1 from 5 yields a new count of 4
    /// assert_eq!(count.checked_sub::<5>(1), Some(BitCount::<5>::new::<4>()));
    /// // subtracting 6 from 5 yields None
    /// assert!(count.checked_sub::<5>(6).is_none());
    /// // subtracting 1 with a new maximum of 3 also yields None
    /// // because 4 is larger than the maximum of 3
    /// assert!(count.checked_sub::<3>(1).is_none());
    /// ```
    #[inline]
    pub const fn checked_sub<const NEW_MAX: u32>(self, bits: u32) -> Option<BitCount<NEW_MAX>> {
        match self.bits.checked_sub(bits) {
            Some(bits) if bits <= NEW_MAX => Some(BitCount { bits }),
            _ => None,
        }
    }

    /// Attempt to convert our count to a count with a new
    /// bit count and new maximum.
    ///
    /// Returns `Some(count)` if the updated number of bits
    /// is less than or equal to the new maximum.
    /// Returns `None` if not.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// let count = BitCount::<5>::new::<5>();
    /// // muliplying 5 bits by 2 with a new max of 10 is ok
    /// assert_eq!(
    ///     count.try_map::<10, _>(|i| i.checked_mul(2)),
    ///     Some(BitCount::<10>::new::<10>()),
    /// );
    ///
    /// // multiplying 5 bits by 3 with a new max of 10 overflows
    /// assert_eq!(count.try_map::<10, _>(|i| i.checked_mul(3)), None);
    /// ```
    #[inline]
    pub fn try_map<const NEW_MAX: u32, F>(self, f: F) -> Option<BitCount<NEW_MAX>>
    where
        F: FnOnce(u32) -> Option<u32>,
    {
        f(self.bits)
            .filter(|bits| *bits <= NEW_MAX)
            .map(|bits| BitCount { bits })
    }

    /// Returns our maximum bit count
    ///
    /// # Example
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// let count = BitCount::<10>::new::<5>();
    /// assert_eq!(count.max(), 10);
    /// ```
    #[inline(always)]
    pub const fn max(&self) -> u32 {
        MAX
    }

    /// Returns signed count if our bit count is greater than 0
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::{BitCount, SignedBitCount};
    ///
    /// let count = BitCount::<10>::new::<5>();
    /// assert_eq!(count.signed_count(), Some(SignedBitCount::<10>::new::<5>()));
    ///
    /// let count = BitCount::<10>::new::<0>();
    /// assert_eq!(count.signed_count(), None);
    /// ```
    #[inline(always)]
    pub const fn signed_count(&self) -> Option<SignedBitCount<MAX>> {
        match self.bits.checked_sub(1) {
            Some(bits) => Some(SignedBitCount {
                bits: *self,
                unsigned: BitCount { bits },
            }),
            None => None,
        }
    }

    /// Masks off the least-significant bits for the given type
    ///
    /// Returns closure that takes a value and returns a
    /// pair of the most-significant and least-significant
    /// bits.  Because the least-significant bits cannot
    /// be larger than this bit count, that value is
    /// returned as a [`Checked`] type.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// // create a bit count of 3
    /// let count = BitCount::<8>::new::<3>();
    ///
    /// // create a mask suitable for u8 types
    /// let mask = count.mask_lsb::<u8>();
    ///
    /// let (msb, lsb) = mask(0b11011_110);
    /// assert_eq!(msb, 0b11011);             // the most-significant bits
    /// assert_eq!(lsb.into_value(), 0b110);  // the least-significant bits
    ///
    /// let (msb, lsb) = mask(0b01100_010);
    /// assert_eq!(msb, 0b01100);             // the most-significant bits
    /// assert_eq!(lsb.into_value(), 0b010);  // the least-significant bits
    ///
    /// let (msb, lsb) = mask(0b00000_111);
    /// assert_eq!(msb, 0b00000);             // the most-significant bits
    /// assert_eq!(lsb.into_value(), 0b111);  // the least-significant bits
    /// ```
    ///
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// // a mask with a bit count of 0 puts everything in msb
    /// let mask = BitCount::<8>::new::<0>().mask_lsb::<u8>();
    ///
    /// let (msb, lsb) = mask(0b11111111);
    /// assert_eq!(msb, 0b11111111);
    /// assert_eq!(lsb.into_value(), 0);
    ///
    /// // a mask with a bit count larger than the type
    /// // is restricted to that type's size, if possible
    /// let mask = BitCount::<16>::new::<9>().mask_lsb::<u8>();
    ///
    /// let (msb, lsb) = mask(0b11111111);
    /// assert_eq!(msb, 0);
    /// assert_eq!(lsb.into_value(), 0b11111111);
    /// ```
    pub fn mask_lsb<U: UnsignedInteger>(self) -> impl Fn(U) -> (U, CheckedUnsigned<MAX, U>) {
        use core::convert::TryFrom;

        let (mask, shift, count) = match U::BITS_SIZE.checked_sub(self.bits) {
            Some(mask_bits) => (U::ALL.shr_default(mask_bits), self.bits, self),
            None => (
                U::ALL,
                U::BITS_SIZE,
                BitCount::try_from(U::BITS_SIZE).expect("bit count too small for type"),
            ),
        };

        move |v| {
            (
                v.shr_default(shift),
                Checked {
                    value: v & mask,
                    count,
                },
            )
        }
    }

    /// Returns this bit count's range for the given unsigned type
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// assert_eq!(BitCount::<9>::new::<0>().range::<u8>(), 0..=0);
    /// assert_eq!(BitCount::<9>::new::<1>().range::<u8>(), 0..=0b1);
    /// assert_eq!(BitCount::<9>::new::<2>().range::<u8>(), 0..=0b11);
    /// assert_eq!(BitCount::<9>::new::<3>().range::<u8>(), 0..=0b111);
    /// assert_eq!(BitCount::<9>::new::<4>().range::<u8>(), 0..=0b1111);
    /// assert_eq!(BitCount::<9>::new::<5>().range::<u8>(), 0..=0b11111);
    /// assert_eq!(BitCount::<9>::new::<6>().range::<u8>(), 0..=0b111111);
    /// assert_eq!(BitCount::<9>::new::<7>().range::<u8>(), 0..=0b1111111);
    /// assert_eq!(BitCount::<9>::new::<8>().range::<u8>(), 0..=0b11111111);
    /// // a count that exceeds the type's size is
    /// // naturally restricted to that type's maximum range
    /// assert_eq!(BitCount::<9>::new::<9>().range::<u8>(), 0..=0b11111111);
    /// ```
    #[inline]
    pub fn range<U: UnsignedInteger>(&self) -> core::ops::RangeInclusive<U> {
        match U::ONE.checked_shl(self.bits) {
            Some(top) => U::ZERO..=(top - U::ONE),
            None => U::ZERO..=U::ALL,
        }
    }

    /// Returns minimum value between ourself and bit count
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// let count = BitCount::<8>::new::<7>();
    /// assert_eq!(count.min(6), BitCount::new::<6>());
    /// assert_eq!(count.min(8), BitCount::new::<7>());
    /// ```
    #[inline(always)]
    pub fn min(self, bits: u32) -> Self {
        // the minimum of ourself and another bit count
        // can never exceed our maximum bit count
        Self {
            bits: self.bits.min(bits),
        }
    }

    /// Returns the minimum value of an unsigned int in this bit count
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// assert_eq!(BitCount::<8>::new::<0>().none::<u8>().into_value(), 0b0);
    /// assert_eq!(BitCount::<8>::new::<1>().none::<u8>().into_value(), 0b0);
    /// assert_eq!(BitCount::<8>::new::<2>().none::<u8>().into_value(), 0b00);
    /// assert_eq!(BitCount::<8>::new::<3>().none::<u8>().into_value(), 0b000);
    /// assert_eq!(BitCount::<8>::new::<4>().none::<u8>().into_value(), 0b0000);
    /// assert_eq!(BitCount::<8>::new::<5>().none::<u8>().into_value(), 0b00000);
    /// assert_eq!(BitCount::<8>::new::<6>().none::<u8>().into_value(), 0b000000);
    /// assert_eq!(BitCount::<8>::new::<7>().none::<u8>().into_value(), 0b0000000);
    /// assert_eq!(BitCount::<8>::new::<8>().none::<u8>().into_value(), 0b00000000);
    /// ```
    #[inline(always)]
    pub fn none<U: UnsignedInteger>(self) -> CheckedUnsigned<MAX, U> {
        CheckedUnsigned {
            value: U::ZERO,
            count: self,
        }
    }

    /// Returns the maximum value of an unsigned int in this bit count
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::BitCount;
    ///
    /// assert_eq!(BitCount::<8>::new::<0>().all::<u8>().into_value(), 0b0);
    /// assert_eq!(BitCount::<8>::new::<1>().all::<u8>().into_value(), 0b1);
    /// assert_eq!(BitCount::<8>::new::<2>().all::<u8>().into_value(), 0b11);
    /// assert_eq!(BitCount::<8>::new::<3>().all::<u8>().into_value(), 0b111);
    /// assert_eq!(BitCount::<8>::new::<4>().all::<u8>().into_value(), 0b1111);
    /// assert_eq!(BitCount::<8>::new::<5>().all::<u8>().into_value(), 0b11111);
    /// assert_eq!(BitCount::<8>::new::<6>().all::<u8>().into_value(), 0b111111);
    /// assert_eq!(BitCount::<8>::new::<7>().all::<u8>().into_value(), 0b1111111);
    /// assert_eq!(BitCount::<8>::new::<8>().all::<u8>().into_value(), 0b11111111);
    /// ```
    #[inline(always)]
    pub fn all<U: UnsignedInteger>(self) -> CheckedUnsigned<MAX, U> {
        CheckedUnsigned {
            value: match U::ONE.checked_shl(self.bits) {
                Some(top) => top - U::ONE,
                None => U::ALL,
            },
            count: self,
        }
    }
}

impl<const MAX: u32> core::convert::TryFrom<u32> for BitCount<MAX> {
    type Error = u32;

    /// Attempts to convert a `u32` bit count to a `BitCount`
    ///
    /// Attempting a bit maximum bit count larger than the
    /// largest supported type is a compile-time error
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::BitCount;
    /// use std::convert::TryInto;
    ///
    /// assert_eq!(8u32.try_into(), Ok(BitCount::<8>::new::<8>()));
    /// assert_eq!(9u32.try_into(), Err::<BitCount<8>, _>(9));
    /// ```
    fn try_from(bits: u32) -> Result<Self, Self::Error> {
        (bits <= MAX).then_some(Self { bits }).ok_or(bits)
    }
}

impl<const MAX: u32> core::fmt::Display for BitCount<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.bits, f)
    }
}

impl BitCount<{ u32::MAX }> {
    /// Builds a bit count where the maximum bits is unknown.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::BitCount;
    /// assert_eq!(BitCount::unknown(5), BitCount::<{ u32::MAX }>::new::<5>());
    /// ```
    pub const fn unknown(bits: u32) -> Self {
        Self { bits }
    }
}

#[test]
fn test_unknown_bitcount() {
    let count = BitCount::unknown(u32::MAX);
    assert!(u32::from(count) <= count.max());
}

impl<const MAX: u32> From<BitCount<MAX>> for u32 {
    #[inline(always)]
    fn from(BitCount { bits }: BitCount<MAX>) -> u32 {
        bits
    }
}

/// A number of bits to be read or written for signed integers, with a known maximum
///
/// This is closely related to the [`BitCount`] type, but further constrained
/// to have a minimum value of 1 - because signed values require at least
/// 1 bit for the sign.
///
/// Let's start with a basic example:
///
/// ```
/// use bitstream_io::{BitRead, BitReader, BigEndian};
///
/// let data: &[u8] = &[0b100_0001_1, 0b111_0110_0];
/// let mut r = BitReader::endian(data, BigEndian);
/// // our bit count is a 3 bit value
/// let count = r.read::<3, u32>().unwrap();
/// // that count indicates we need to read 4 bits (0b100)
/// assert_eq!(count, 4);
/// // read the first 4-bit signed value
/// assert_eq!(r.read_var::<i8>(count).unwrap(), 1);
/// // read the second 4-bit signed value
/// assert_eq!(r.read_var::<i8>(count).unwrap(), -1);
/// // read the third 4-bit signed value
/// assert_eq!(r.read_var::<i8>(count).unwrap(), 6);
/// ```
///
/// In the preceding example, even though we know `count` is a
/// 3 bit value whose maximum value will never be greater than 7,
/// the subsequent `read_var` calls have no way to know that.
/// They must assume `count` could be 9, or `u32::MAX` or any other `u32` value
/// and validate the count is not larger than the `i8` types we're reading
/// while also greater than 0 because `i8` requires a sign bit.
///
/// But we can convert our example to use the `SignedBitCount` type:
/// ```
/// use bitstream_io::{BitRead, BitReader, BigEndian, SignedBitCount};
///
/// let data: &[u8] = &[0b100_0001_1, 0b111_0110_0];
/// let mut r = BitReader::endian(data, BigEndian);
/// // our bit count is a 3 bit value with a maximum value of 7
/// let count = r.read_count::<0b111>().unwrap();
/// // convert that count to a signed bit count,
/// // which guarantees its value is greater than 0
/// let count = count.signed_count().unwrap();
/// // that count indicates we need to read 4 bits (0b100)
/// assert_eq!(count, SignedBitCount::<7>::new::<4>());
/// // read the first 4-bit value
/// assert_eq!(r.read_signed_counted::<7, i8>(count).unwrap(), 1);
/// // read the second 4-bit value
/// assert_eq!(r.read_signed_counted::<7, i8>(count).unwrap(), -1);
/// // read the third 4-bit value
/// assert_eq!(r.read_signed_counted::<7, i8>(count).unwrap(), 6);
/// ```
///
/// Because the [`BitRead::read_signed_counted`] methods know at compile-time
/// that the bit count will be larger than 7, that check can be eliminated
/// simply by taking advantage of information we already know.
///
/// Leveraging the `SignedBitCount` type also allows us to reason about
/// bit counts in a more formal way, and use checked permutation methods
/// to modify them while ensuring they remain constrained by
/// the file format's requirements.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SignedBitCount<const MAX: u32> {
    // the whole original bit count
    bits: BitCount<MAX>,
    // a bit count with one bit removed for the sign
    unsigned: BitCount<MAX>,
}

impl<const MAX: u32> SignedBitCount<MAX> {
    /// Builds a signed bit count from a constant number
    /// of bits, which must be greater than 0 and
    /// not be greater than `MAX`.
    ///
    /// Intended to be used for defining constants.
    ///
    /// Use `TryFrom` to conditionally build
    /// counts from values at runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian, SignedBitCount};
    /// let data: &[u8] = &[0b111_00000];
    /// let mut r = BitReader::endian(data, BigEndian);
    /// // reading 3 bits from a stream out of a maximum of 8
    /// // doesn't require checking that the bit count is larger
    /// // than a u8 at runtime because specifying the maximum of 8
    /// // guarantees our bit count will not be larger than 8
    /// assert_eq!(r.read_signed_counted::<8, i8>(SignedBitCount::new::<3>()).unwrap(), -1);
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::SignedBitCount;
    /// // trying to build a count of 10 with a maximum of 8
    /// // fails to compile at all
    /// let count = SignedBitCount::<8>::new::<10>();
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::SignedBitCount;
    /// // trying to build a count of 0 also fails to compile
    /// let count = SignedBitCount::<8>::new::<0>();
    /// ```
    pub const fn new<const BITS: u32>() -> Self {
        const {
            assert!(BITS > 0, "BITS must be > 0");
        }

        Self {
            bits: BitCount::new::<BITS>(),
            unsigned: BitCount { bits: BITS - 1 },
        }
    }

    /// Add a number of bits to our count,
    /// returning a new count with a new maximum.
    ///
    /// Returns `None` if the new count goes above our new maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::SignedBitCount;
    ///
    /// let count = SignedBitCount::<2>::new::<1>();
    /// // adding 2 to 1 and increasing the max to 3 yields a new count of 3
    /// assert_eq!(count.checked_add::<3>(2), Some(SignedBitCount::<3>::new::<3>()));
    /// // adding 2 to 1 without increasing the max yields None
    /// assert_eq!(count.checked_add::<2>(2), None);
    /// ```
    #[inline]
    pub const fn checked_add<const NEW_MAX: u32>(
        self,
        bits: u32,
    ) -> Option<SignedBitCount<NEW_MAX>> {
        match self.bits.checked_add(bits) {
            Some(bits_new) => match self.unsigned.checked_add(bits) {
                Some(unsigned) => Some(SignedBitCount {
                    bits: bits_new,
                    unsigned,
                }),
                None => None,
            },
            None => None,
        }
    }

    /// Subtracts a number of bits from our count,
    /// returning a new count with a new maximum.
    ///
    /// Returns `None` if the new count goes below 1
    /// or below our new maximum.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::SignedBitCount;
    /// let count = SignedBitCount::<5>::new::<5>();
    /// // subtracting 1 from 5 yields a new count of 4
    /// assert_eq!(count.checked_sub::<5>(1), Some(SignedBitCount::<5>::new::<4>()));
    /// // subtracting 6 from 5 yields None
    /// assert!(count.checked_sub::<5>(6).is_none());
    /// // subtracting 1 with a new maximum of 3 also yields None
    /// // because 4 is larger than the maximum of 3
    /// assert!(count.checked_sub::<3>(1).is_none());
    /// // subtracting 5 from 5 also yields None
    /// // because SignedBitCount always requires 1 bit for the sign
    /// assert!(count.checked_sub::<5>(5).is_none());
    /// ```
    #[inline]
    pub const fn checked_sub<const NEW_MAX: u32>(
        self,
        bits: u32,
    ) -> Option<SignedBitCount<NEW_MAX>> {
        match self.bits.checked_sub(bits) {
            Some(bits_new) => match self.unsigned.checked_sub(bits) {
                Some(unsigned) => Some(SignedBitCount {
                    bits: bits_new,
                    unsigned,
                }),
                None => None,
            },
            None => None,
        }
    }

    /// Attempt to convert our count to a count with a new
    /// bit count and new maximum.
    ///
    /// Returns `Some(count)` if the updated number of bits
    /// is less than or equal to the new maximum
    /// and greater than 0.
    /// Returns `None` if not.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::SignedBitCount;
    ///
    /// let count = SignedBitCount::<5>::new::<5>();
    /// // muliplying 5 bits by 2 with a new max of 10 is ok
    /// assert_eq!(
    ///     count.try_map::<10, _>(|i| i.checked_mul(2)),
    ///     Some(SignedBitCount::<10>::new::<10>()),
    /// );
    ///
    /// // multiplying 5 bits by 3 with a new max of 10 overflows
    /// assert_eq!(count.try_map::<10, _>(|i| i.checked_mul(3)), None);
    ///
    /// // multiplying 5 bits by 0 results in 0 bits,
    /// // which isn't value for a SignedBitCount
    /// assert_eq!(count.try_map::<10, _>(|i| Some(i * 0)), None);
    /// ```
    #[inline]
    pub fn try_map<const NEW_MAX: u32, F>(self, f: F) -> Option<SignedBitCount<NEW_MAX>>
    where
        F: FnOnce(u32) -> Option<u32>,
    {
        self.bits.try_map(f).and_then(|b| b.signed_count())
    }

    /// Returns our maximum bit count
    ///
    /// # Example
    /// ```
    /// use bitstream_io::SignedBitCount;
    ///
    /// let count = SignedBitCount::<10>::new::<5>();
    /// assert_eq!(count.max(), 10);
    /// ```
    #[inline(always)]
    pub const fn max(&self) -> u32 {
        MAX
    }

    /// Returns regular unsigned bit count
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::{BitCount, SignedBitCount};
    ///
    /// let signed_count = SignedBitCount::<10>::new::<5>();
    /// assert_eq!(signed_count.count(), BitCount::<10>::new::<5>());
    /// ```
    #[inline(always)]
    pub const fn count(&self) -> BitCount<MAX> {
        self.bits
    }

    /// Returns this bit count's range for the given signed type
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::SignedBitCount;
    ///
    /// assert_eq!(SignedBitCount::<9>::new::<1>().range::<i8>(), -1..=0);
    /// assert_eq!(SignedBitCount::<9>::new::<2>().range::<i8>(), -2..=1);
    /// assert_eq!(SignedBitCount::<9>::new::<3>().range::<i8>(), -4..=3);
    /// assert_eq!(SignedBitCount::<9>::new::<4>().range::<i8>(), -8..=7);
    /// assert_eq!(SignedBitCount::<9>::new::<5>().range::<i8>(), -16..=15);
    /// assert_eq!(SignedBitCount::<9>::new::<6>().range::<i8>(), -32..=31);
    /// assert_eq!(SignedBitCount::<9>::new::<7>().range::<i8>(), -64..=63);
    /// assert_eq!(SignedBitCount::<9>::new::<8>().range::<i8>(), -128..=127);
    /// // a count that exceeds the type's size is
    /// // naturally restricted to that type's maximum range
    /// assert_eq!(SignedBitCount::<9>::new::<9>().range::<i8>(), -128..=127);
    /// ```
    pub fn range<S: SignedInteger>(&self) -> core::ops::RangeInclusive<S> {
        // a bit of a hack to get around the somewhat restrictive
        // SignedInteger trait I've created for myself

        if self.bits.bits < S::BITS_SIZE {
            (!S::ZERO << self.unsigned.bits)..=((S::ONE << self.unsigned.bits) - S::ONE)
        } else {
            S::Unsigned::ZERO.as_negative(S::BITS_SIZE)..=(S::Unsigned::ALL >> 1).as_non_negative()
        }
    }
}

impl<const MAX: u32> core::fmt::Display for SignedBitCount<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.bits, f)
    }
}

impl<const MAX: u32> core::convert::TryFrom<BitCount<MAX>> for SignedBitCount<MAX> {
    type Error = ();

    #[inline]
    fn try_from(count: BitCount<MAX>) -> Result<Self, Self::Error> {
        count.signed_count().ok_or(())
    }
}

impl<const MAX: u32> core::convert::TryFrom<u32> for SignedBitCount<MAX> {
    type Error = u32;

    #[inline]
    fn try_from(count: u32) -> Result<Self, Self::Error> {
        BitCount::<MAX>::try_from(count).and_then(|b| b.signed_count().ok_or(count))
    }
}

impl<const MAX: u32> From<SignedBitCount<MAX>> for u32 {
    #[inline(always)]
    fn from(
        SignedBitCount {
            bits: BitCount { bits },
            ..
        }: SignedBitCount<MAX>,
    ) -> u32 {
        bits
    }
}

/// An error when converting a value to a [`Checked`] struct
#[derive(Copy, Clone, Debug)]
pub enum CheckedError {
    /// Excessive bits for type
    ExcessiveBits,
    /// Excessive value for bits
    ExcessiveValue,
}

impl core::error::Error for CheckedError {}

impl core::fmt::Display for CheckedError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ExcessiveBits => core::fmt::Display::fmt("excessive bits for type written", f),
            Self::ExcessiveValue => core::fmt::Display::fmt("excessive value for bits written", f),
        }
    }
}

impl From<CheckedError> for io::Error {
    #[inline]
    fn from(error: CheckedError) -> Self {
        match error {
            CheckedError::ExcessiveBits => io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive bits for type written",
            ),
            CheckedError::ExcessiveValue => io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive value for bits written",
            ),
        }
    }
}

/// A type for eliminating redundant validation when writing
///
/// Normally, when writing a value, not only must the number of bits
/// must be checked against the type being written
/// (e.g. writing 9 bits from a `u8` is always an error),
/// but the value must also be checked against the number of bits
/// (e.g. writing a value of 2 in 1 bit is always an error).
///
/// But when the value's range can be checked in advance,
/// the write-time check can be skipped through the use
/// of the [`BitWrite::write_checked`] method.
#[derive(Copy, Clone, Debug)]
pub struct Checked<C, T> {
    count: C,
    value: T,
}

impl<C, T> Checked<C, T> {
    /// Returns our bit count and value
    #[inline]
    pub fn into_count_value(self) -> (C, T) {
        (self.count, self.value)
    }

    /// Returns our value
    #[inline]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<C, T> AsRef<T> for Checked<C, T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

/// An unsigned type with a verified value
pub type CheckedUnsigned<const MAX: u32, T> = Checked<BitCount<MAX>, T>;

impl<const MAX: u32, U: UnsignedInteger> Checkable for CheckedUnsigned<MAX, U> {
    #[inline]
    fn write<W: BitWrite + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        // a naive default implementation
        writer.write_unsigned_counted(self.count, self.value)
    }

    #[inline]
    fn written_bits(&self) -> u32 {
        self.count.bits
    }
}

impl<const MAX: u32, U: UnsignedInteger> CheckablePrimitive for CheckedUnsigned<MAX, U> {
    type CountType = BitCount<MAX>;

    #[inline]
    fn read<R: BitRead + ?Sized>(reader: &mut R, count: Self::CountType) -> io::Result<Self> {
        reader
            .read_unsigned_counted(count)
            .map(|value| Self { value, count })
    }
}

impl<const MAX: u32, U: UnsignedInteger> private::Checkable for CheckedUnsigned<MAX, U> {
    fn write_endian<E, W>(
        self,
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<()>
    where
        E: private::Endianness,
        W: io::Write,
    {
        E::write_bits_checked(writer, queue_value, queue_bits, self)
    }
}

impl<const MAX: u32, U: UnsignedInteger> CheckedUnsigned<MAX, U> {
    /// Returns our value if it fits in the given number of bits
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::{BitCount, CheckedUnsigned, CheckedError};
    ///
    /// // a value of 7 fits into a 3 bit count
    /// assert!(CheckedUnsigned::<8, u8>::new(3, 0b111).is_ok());
    ///
    /// // a value of 8 does not fit into a 3 bit count
    /// assert!(matches!(
    ///     CheckedUnsigned::<8, u8>::new(3, 0b1000),
    ///     Err(CheckedError::ExcessiveValue),
    /// ));
    ///
    /// // a bit count of 9 is too large for u8
    /// assert!(matches!(
    ///     CheckedUnsigned::<9, _>::new(9, 1u8),
    ///     Err(CheckedError::ExcessiveBits),
    /// ));
    /// ```
    #[inline]
    pub fn new(count: impl TryInto<BitCount<MAX>>, value: U) -> Result<Self, CheckedError> {
        let count @ BitCount { bits } =
            count.try_into().map_err(|_| CheckedError::ExcessiveBits)?;

        if MAX <= U::BITS_SIZE || bits <= U::BITS_SIZE {
            if bits == 0 {
                Ok(Self {
                    count,
                    value: U::ZERO,
                })
            } else if value <= U::ALL >> (U::BITS_SIZE - bits) {
                Ok(Self { count, value })
            } else {
                Err(CheckedError::ExcessiveValue)
            }
        } else {
            Err(CheckedError::ExcessiveBits)
        }
    }

    /// Returns our value if it fits in the given number of const bits
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{CheckedUnsigned, CheckedError};
    ///
    /// // a value of 7 fits into a 3 bit count
    /// assert!(CheckedUnsigned::<8, u8>::new_fixed::<3>(0b111).is_ok());
    ///
    /// // a value of 8 does not fit into a 3 bit count
    /// assert!(matches!(
    ///     CheckedUnsigned::<8, u8>::new_fixed::<3>(0b1000),
    ///     Err(CheckedError::ExcessiveValue),
    /// ));
    /// ```
    ///
    /// ```compile_fail
    /// use bitstream_io::{BitCount, CheckedUnsigned};
    ///
    /// // a bit count of 9 is too large for u8
    ///
    /// // because this is checked at compile-time,
    /// // it does not compile at all
    /// let c = CheckedUnsigned::<16, u8>::new_fixed::<9>(1);
    /// ```
    pub fn new_fixed<const BITS: u32>(value: U) -> Result<Self, CheckedError> {
        const {
            assert!(BITS <= U::BITS_SIZE, "excessive bits for type written");
        }

        if BITS == 0 {
            Ok(Self {
                count: BitCount::new::<0>(),
                value: U::ZERO,
            })
        } else if BITS == U::BITS_SIZE || value <= (U::ALL >> (U::BITS_SIZE - BITS)) {
            Ok(Self {
                // whether BITS is larger than MAX is checked here
                count: BitCount::new::<BITS>(),
                value,
            })
        } else {
            Err(CheckedError::ExcessiveValue)
        }
    }
}

/// A fixed number of bits to be consumed or written
///
/// Analagous to [`BitCount`], this is a zero-sized type
/// whose value is fixed at compile-time and cannot be changed.
///
/// # Example
///
/// ```
/// use bitstream_io::{
///     BigEndian, BitRead, BitReader, BitWrite, BitWriter, CheckedUnsignedFixed, FixedBitCount,
/// };
///
/// type FourBits = CheckedUnsignedFixed<4, u8>;
///
/// let input: &[u8] = &[0b0001_1111, 0b0110_1001];
/// let mut r = BitReader::endian(input, BigEndian);
///
/// // read 4, 4-bit values
/// let v1 = r.read_checked::<FourBits>(FixedBitCount).unwrap();
/// let v2 = r.read_checked::<FourBits>(FixedBitCount).unwrap();
/// let v3 = r.read_checked::<FourBits>(FixedBitCount).unwrap();
/// let v4 = r.read_checked::<FourBits>(FixedBitCount).unwrap();
///
/// assert_eq!(v1.into_value(), 0b0001);
/// assert_eq!(v2.into_value(), 0b1111);
/// assert_eq!(v3.into_value(), 0b0110);
/// assert_eq!(v4.into_value(), 0b1001);
///
/// // write those same values back to disk
/// let mut w = BitWriter::endian(vec![], BigEndian);
/// w.write_checked(v1).unwrap();
/// w.write_checked(v2).unwrap();
/// w.write_checked(v3).unwrap();
/// w.write_checked(v4).unwrap();
///
/// // ensure they're the same
/// assert_eq!(w.into_writer().as_slice(), input);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct FixedBitCount<const BITS: u32>;

impl<const BITS: u32> From<FixedBitCount<BITS>> for BitCount<BITS> {
    fn from(_count: FixedBitCount<BITS>) -> Self {
        BitCount::new::<BITS>()
    }
}

impl<const BITS: u32, const MAX: u32> core::convert::TryFrom<BitCount<MAX>>
    for FixedBitCount<BITS>
{
    type Error = BitCount<MAX>;

    fn try_from(count: BitCount<MAX>) -> Result<Self, Self::Error> {
        (count.bits == BITS).then_some(FixedBitCount).ok_or(count)
    }
}

/// An unsigned type with a verified value for a fixed number of bits
pub type CheckedUnsignedFixed<const BITS: u32, T> = Checked<FixedBitCount<BITS>, T>;

impl<const BITS: u32, U: UnsignedInteger> CheckedUnsignedFixed<BITS, U> {
    /// Returns our checked value if it fits in the given number of const bits
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{CheckedUnsignedFixed, CheckedError};
    ///
    /// // a value of 7 fits into a maximum of 3 bits
    /// assert!(CheckedUnsignedFixed::<3, u8>::new_fixed(0b111).is_ok());
    ///
    /// // a value of 8 does not fit into a maximum of 3 bits
    /// assert!(matches!(
    ///     CheckedUnsignedFixed::<3, u8>::new_fixed(0b1000),
    ///     Err(CheckedError::ExcessiveValue),
    /// ));
    /// ```
    ///
    /// ```compile_fail
    /// use bitstream_io::CheckedUnsignedFixed;
    ///
    /// // a bit count of 9 is too large for u8
    ///
    /// // because this is checked at compile-time,
    /// // it does not compile at all
    /// let c = CheckedUnsignedFixed::<9, u8>::new_fixed(1);
    /// ```
    pub fn new_fixed(value: U) -> Result<Self, CheckedError> {
        const {
            assert!(BITS <= U::BITS_SIZE, "excessive bits for type written");
        }

        if BITS == 0 {
            Ok(Self {
                count: FixedBitCount,
                value: U::ZERO,
            })
        } else if BITS == U::BITS_SIZE || value <= (U::ALL >> (U::BITS_SIZE - BITS)) {
            Ok(Self {
                count: FixedBitCount,
                value,
            })
        } else {
            Err(CheckedError::ExcessiveValue)
        }
    }
}

impl<const BITS: u32, U: UnsignedInteger> Checkable for CheckedUnsignedFixed<BITS, U> {
    #[inline]
    fn write<W: BitWrite + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        // a naive default implementation
        writer.write_unsigned::<BITS, _>(self.value)
    }

    #[inline]
    fn written_bits(&self) -> u32 {
        BITS
    }
}

impl<const BITS: u32, U: UnsignedInteger> private::Checkable for CheckedUnsignedFixed<BITS, U> {
    fn write_endian<E, W>(
        self,
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<()>
    where
        E: private::Endianness,
        W: io::Write,
    {
        E::write_bits_checked(
            writer,
            queue_value,
            queue_bits,
            Checked {
                value: self.value,
                count: self.count.into(),
            },
        )
    }
}

impl<const BITS: u32, U: UnsignedInteger> CheckablePrimitive for CheckedUnsignedFixed<BITS, U> {
    type CountType = FixedBitCount<BITS>;

    fn read<R: BitRead + ?Sized>(reader: &mut R, count: FixedBitCount<BITS>) -> io::Result<Self> {
        Ok(Self {
            value: reader.read_unsigned::<BITS, _>()?,
            count,
        })
    }
}

/// A signed type with a verified value
pub type CheckedSigned<const MAX: u32, T> = Checked<SignedBitCount<MAX>, T>;

impl<const MAX: u32, S: SignedInteger> Checkable for CheckedSigned<MAX, S> {
    #[inline]
    fn write<W: BitWrite + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        // a naive default implementation
        writer.write_signed_counted(self.count, self.value)
    }

    #[inline]
    fn written_bits(&self) -> u32 {
        self.count.bits.into()
    }
}

impl<const MAX: u32, S: SignedInteger> CheckablePrimitive for CheckedSigned<MAX, S> {
    type CountType = SignedBitCount<MAX>;

    #[inline]
    fn read<R: BitRead + ?Sized>(reader: &mut R, count: Self::CountType) -> io::Result<Self> {
        reader
            .read_signed_counted(count)
            .map(|value| Self { value, count })
    }
}

impl<const MAX: u32, S: SignedInteger> private::Checkable for CheckedSigned<MAX, S> {
    #[inline]
    fn write_endian<E, W>(
        self,
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<()>
    where
        E: private::Endianness,
        W: io::Write,
    {
        E::write_signed_bits_checked(writer, queue_value, queue_bits, self)
    }
}

impl<const MAX: u32, S: SignedInteger> CheckedSigned<MAX, S> {
    /// Returns our value if it fits in the given number of bits
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::{SignedBitCount, CheckedSigned, CheckedError};
    ///
    /// // a value of 3 fits into a 3 bit count
    /// assert!(CheckedSigned::<8, _>::new(3, 3i8).is_ok());
    ///
    /// // a value of 4 does not fit into a 3 bit count
    /// assert!(matches!(
    ///     CheckedSigned::<8, _>::new(3, 4i8),
    ///     Err(CheckedError::ExcessiveValue),
    /// ));
    ///
    /// // a bit count of 9 is too large for i8
    /// assert!(matches!(
    ///     CheckedSigned::<9, _>::new(9, 1i8),
    ///     Err(CheckedError::ExcessiveBits),
    /// ));
    /// ```
    #[inline]
    pub fn new(count: impl TryInto<SignedBitCount<MAX>>, value: S) -> Result<Self, CheckedError> {
        let count @ SignedBitCount {
            bits: BitCount { bits },
            unsigned: BitCount {
                bits: unsigned_bits,
            },
        } = count.try_into().map_err(|_| CheckedError::ExcessiveBits)?;

        if MAX <= S::BITS_SIZE || bits <= S::BITS_SIZE {
            if bits == S::BITS_SIZE
                || (((S::ZERO - S::ONE) << unsigned_bits) <= value
                    && value < (S::ONE << unsigned_bits))
            {
                Ok(Self { count, value })
            } else {
                Err(CheckedError::ExcessiveValue)
            }
        } else {
            Err(CheckedError::ExcessiveBits)
        }
    }

    /// Returns our value if it fits in the given number of const bits
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{CheckedSigned, CheckedError};
    ///
    /// // a value of 3 fits into a 3 bit count
    /// assert!(CheckedSigned::<8, i8>::new_fixed::<3>(3).is_ok());
    ///
    /// // a value of 4 does not fit into a 3 bit count
    /// assert!(matches!(
    ///     CheckedSigned::<8, i8>::new_fixed::<3>(4),
    ///     Err(CheckedError::ExcessiveValue),
    /// ));
    /// ```
    ///
    /// ```compile_fail
    /// use bitstream_io::{BitCount, CheckedSigned};
    ///
    /// // a bit count of 9 is too large for i8
    ///
    /// // because this is checked at compile-time,
    /// // it does not compile at all
    /// let c = CheckedSigned::<16, i8>::new_fixed::<9>(1);
    /// ```
    pub fn new_fixed<const BITS: u32>(value: S) -> Result<Self, CheckedError> {
        const {
            assert!(BITS <= S::BITS_SIZE, "excessive bits for type written");
        }

        if BITS == S::BITS_SIZE
            || (((S::ZERO - S::ONE) << (BITS - 1)) <= value && value < (S::ONE << (BITS - 1)))
        {
            Ok(Self {
                count: SignedBitCount::new::<BITS>(),
                value,
            })
        } else {
            Err(CheckedError::ExcessiveValue)
        }
    }
}

/// A fixed number of bits to be consumed or written
///
/// Analagous to [`SignedBitCount`], this is a zero-sized type
/// whose value is fixed at compile-time and cannot be changed.
///
/// # Example
///
/// ```
/// use bitstream_io::{
///     BigEndian, BitRead, BitReader, BitWrite, BitWriter,
///     CheckedSignedFixed, FixedSignedBitCount,
/// };
///
/// type FourBits = CheckedSignedFixed<4, i8>;
///
/// let input: &[u8] = &[0b0001_1111, 0b0110_1001];
/// let mut r = BitReader::endian(input, BigEndian);
///
/// // read 4, 4-bit values
/// let v1 = r.read_checked::<FourBits>(FixedSignedBitCount).unwrap();
/// let v2 = r.read_checked::<FourBits>(FixedSignedBitCount).unwrap();
/// let v3 = r.read_checked::<FourBits>(FixedSignedBitCount).unwrap();
/// let v4 = r.read_checked::<FourBits>(FixedSignedBitCount).unwrap();
///
/// assert_eq!(v1.into_value(), 1);
/// assert_eq!(v2.into_value(), -1);
/// assert_eq!(v3.into_value(), 6);
/// assert_eq!(v4.into_value(), -7);
///
/// // write those same values back to disk
/// let mut w = BitWriter::endian(vec![], BigEndian);
/// w.write_checked(v1).unwrap();
/// w.write_checked(v2).unwrap();
/// w.write_checked(v3).unwrap();
/// w.write_checked(v4).unwrap();
///
/// // ensure they're the same
/// assert_eq!(w.into_writer().as_slice(), input);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct FixedSignedBitCount<const BITS: u32>;

impl<const BITS: u32> From<FixedSignedBitCount<BITS>> for SignedBitCount<BITS> {
    fn from(_count: FixedSignedBitCount<BITS>) -> Self {
        SignedBitCount::new::<BITS>()
    }
}

impl<const BITS: u32, const MAX: u32> core::convert::TryFrom<SignedBitCount<MAX>>
    for FixedSignedBitCount<BITS>
{
    type Error = SignedBitCount<MAX>;

    fn try_from(count: SignedBitCount<MAX>) -> Result<Self, Self::Error> {
        (count.bits.bits == BITS)
            .then_some(FixedSignedBitCount)
            .ok_or(count)
    }
}

/// A signed type with a verified value for a fixed number of bits
pub type CheckedSignedFixed<const BITS: u32, T> = Checked<FixedSignedBitCount<BITS>, T>;

impl<const BITS: u32, S: SignedInteger> CheckedSignedFixed<BITS, S> {
    /// Returns our checked value if it fits in the given number of const bits
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{SignedBitCount, CheckedSignedFixed, CheckedError};
    ///
    /// // a value of 3 fits into a 3 bit count
    /// assert!(CheckedSignedFixed::<3, _>::new_fixed(3i8).is_ok());
    ///
    /// // a value of 4 does not fit into a 3 bit count
    /// assert!(matches!(
    ///     CheckedSignedFixed::<3, _>::new_fixed(4i8),
    ///     Err(CheckedError::ExcessiveValue),
    /// ));
    /// ```
    ///
    /// ```compile_fail
    /// use bitstream_io::CheckedSignedFixed;
    ///
    /// // a bit count of 9 is too large for i8
    ///
    /// // because this is checked at compile-time,
    /// // it does not compile at all
    /// let c = CheckedSignedFixed::<9, _>::new_fixed(1i8);
    /// ```
    pub fn new_fixed(value: S) -> Result<Self, CheckedError> {
        const {
            assert!(BITS <= S::BITS_SIZE, "excessive bits for type written");
        }

        if BITS == S::BITS_SIZE
            || (((S::ZERO - S::ONE) << (BITS - 1)) <= value && value < (S::ONE << (BITS - 1)))
        {
            Ok(Self {
                count: FixedSignedBitCount,
                value,
            })
        } else {
            Err(CheckedError::ExcessiveValue)
        }
    }
}
impl<const BITS: u32, S: SignedInteger> Checkable for CheckedSignedFixed<BITS, S> {
    #[inline]
    fn write<W: BitWrite + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        // a naive default implementation
        writer.write_signed::<BITS, _>(self.value)
    }

    #[inline]
    fn written_bits(&self) -> u32 {
        BITS
    }
}

impl<const BITS: u32, S: SignedInteger> private::Checkable for CheckedSignedFixed<BITS, S> {
    #[inline]
    fn write_endian<E, W>(
        self,
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<()>
    where
        E: private::Endianness,
        W: io::Write,
    {
        E::write_signed_bits_checked(
            writer,
            queue_value,
            queue_bits,
            CheckedSigned {
                value: self.value,
                count: self.count.into(),
            },
        )
    }
}

impl<const BITS: u32, S: SignedInteger> CheckablePrimitive for CheckedSignedFixed<BITS, S> {
    type CountType = FixedSignedBitCount<BITS>;

    fn read<R: BitRead + ?Sized>(
        reader: &mut R,
        count: FixedSignedBitCount<BITS>,
    ) -> io::Result<Self> {
        Ok(Self {
            value: reader.read_signed::<BITS, _>()?,
            count,
        })
    }
}

/// A trait for writable types whose values can be validated
///
/// Ordinarily, when writing a value to a stream with a given
/// number of bits, the value must be validated to ensure
/// it will fit within that number of bits.
///
/// # Example 1
///
/// ```
/// use bitstream_io::{BitWrite, BitWriter, BigEndian};
///
/// let mut w = BitWriter::endian(vec![], BigEndian);
///
/// // writing a value of 2 in 1 bit is always an error
/// // which is checked here at write-time
/// assert!(w.write::<1, u8>(2).is_err());
/// ```
///
/// But if the value can be checked beforehand,
/// it doesn't need to be checked at write-time.
///
/// # Example 2
///
/// ```
/// use bitstream_io::{BitWrite, BitWriter, BigEndian, CheckedUnsigned};
///
/// let mut w = BitWriter::endian(vec![], BigEndian);
///
/// // writing a value of 1 in 1 bit is ok
/// // and we're checking that validity at this stage
/// let value = CheckedUnsigned::<1, u8>::new_fixed::<1>(1).unwrap();
///
/// // because we've pre-validated the value beforehand,
/// // it doesn't need to be checked again here
/// // (though the write itself may still fail)
/// assert!(w.write_checked(value).is_ok());
/// ```
///
pub trait Checkable: private::Checkable + Sized {
    /// Write our value to the given stream
    fn write<W: BitWrite + ?Sized>(&self, writer: &mut W) -> io::Result<()>;

    /// The number of written bits
    fn written_bits(&self) -> u32;
}

/// A trait for readable types whose bit counts can be saved
///
/// Because the intent of reading checkable values is
/// to avoid validating their values when being written,
/// implementing the [`Checkable`] trait is required.
pub trait CheckablePrimitive: Checkable {
    /// Our bit count type for reading
    type CountType;

    /// Reads our value from the given stream
    fn read<R: BitRead + ?Sized>(reader: &mut R, count: Self::CountType) -> io::Result<Self>;
}

/// Big-endian, or most significant bits first
#[derive(Copy, Clone, Debug)]
pub struct BigEndian;

/// Big-endian, or most significant bits first
pub type BE = BigEndian;

impl BigEndian {
    // checked in the sense that we've verified
    // the output type is large enough to hold the
    // requested number of bits
    #[inline]
    fn read_bits_checked<const MAX: u32, R, U>(
        reader: &mut R,
        queue: &mut u8,
        queue_bits: &mut u32,
        BitCount { bits }: BitCount<MAX>,
    ) -> io::Result<U>
    where
        R: io::Read,
        U: UnsignedInteger,
    {
        // reads a whole value with the given number of
        // bytes in our endianness, where the number of bytes
        // must be less than or equal to the type's size in bytes
        #[inline(always)]
        fn read_bytes<R, U>(reader: &mut R, bytes: usize) -> io::Result<U>
        where
            R: io::Read,
            U: UnsignedInteger,
        {
            let mut buf = U::buffer();
            reader
                .read_exact(&mut buf.as_mut()[(mem::size_of::<U>() - bytes)..])
                .map(|()| U::from_be_bytes(buf))
        }

        if bits <= *queue_bits {
            // all bits in queue, so no byte needed
            let value = queue.shr_default(u8::BITS_SIZE - bits);
            *queue = queue.shl_default(bits);
            *queue_bits -= bits;
            Ok(U::from_u8(value))
        } else {
            // at least one byte needed

            // bits needed beyond what's in the queue
            let needed_bits = bits - *queue_bits;

            match (needed_bits / 8, needed_bits % 8) {
                (0, needed) => {
                    // only one additional byte needed,
                    // which we share between our returned value
                    // and the bit queue
                    let next_byte = read_byte(reader)?;

                    Ok(U::from_u8(
                        mem::replace(queue, next_byte.shl_default(needed)).shr_default(
                            u8::BITS_SIZE - mem::replace(queue_bits, u8::BITS_SIZE - needed),
                        ),
                    )
                    .shl_default(needed)
                        | U::from_u8(next_byte.shr_default(u8::BITS_SIZE - needed)))
                }
                (bytes, 0) => {
                    // exact number of bytes needed beyond what's
                    // available in the queue
                    // so read a whole value from the reader
                    // and prepend what's left of our queue onto it

                    Ok(U::from_u8(
                        mem::take(queue).shr_default(u8::BITS_SIZE - mem::take(queue_bits)),
                    )
                    .shl_default(needed_bits)
                        | read_bytes(reader, bytes as usize)?)
                }
                (bytes, needed) => {
                    // read a whole value from the reader
                    // prepend what's in the queue at the front of it
                    // *and* append a partial byte at the end of it
                    // while also updating the queue and its bit count

                    let whole: U = read_bytes(reader, bytes as usize)?;
                    let next_byte = read_byte(reader)?;

                    Ok(U::from_u8(
                        mem::replace(queue, next_byte.shl_default(needed)).shr_default(
                            u8::BITS_SIZE - mem::replace(queue_bits, u8::BITS_SIZE - needed),
                        ),
                    )
                    .shl_default(needed_bits)
                        | whole.shl_default(needed)
                        | U::from_u8(next_byte.shr_default(u8::BITS_SIZE - needed)))
                }
            }
        }
    }
}

impl Endianness for BigEndian {}

impl private::Endianness for BigEndian {
    #[inline]
    fn push_bit_flush(queue_value: &mut u8, queue_bits: &mut u32, bit: bool) -> Option<u8> {
        *queue_value = (*queue_value << 1) | u8::from(bit);
        *queue_bits = (*queue_bits + 1) % 8;
        (*queue_bits == 0).then(|| mem::take(queue_value))
    }

    #[inline]
    fn read_bits<const MAX: u32, R, U>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
        count @ BitCount { bits }: BitCount<MAX>,
    ) -> io::Result<U>
    where
        R: io::Read,
        U: UnsignedInteger,
    {
        if MAX <= U::BITS_SIZE || bits <= U::BITS_SIZE {
            Self::read_bits_checked::<MAX, R, U>(reader, queue_value, queue_bits, count)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive bits for type read",
            ))
        }
    }

    #[inline]
    fn read_bits_fixed<const BITS: u32, R, U>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<U>
    where
        R: io::Read,
        U: UnsignedInteger,
    {
        const {
            assert!(BITS <= U::BITS_SIZE, "excessive bits for type read");
        }

        Self::read_bits_checked::<BITS, R, U>(
            reader,
            queue_value,
            queue_bits,
            BitCount::new::<BITS>(),
        )
    }

    // checked in the sense that we've verified
    // the input type is large enough to hold the
    // requested number of bits and that the value is
    // not too large for those bits
    #[inline]
    fn write_bits_checked<const MAX: u32, W, U>(
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
        CheckedUnsigned {
            count: BitCount { bits },
            value,
        }: CheckedUnsigned<MAX, U>,
    ) -> io::Result<()>
    where
        W: io::Write,
        U: UnsignedInteger,
    {
        fn write_bytes<W, U>(writer: &mut W, bytes: usize, value: U) -> io::Result<()>
        where
            W: io::Write,
            U: UnsignedInteger,
        {
            let buf = U::to_be_bytes(value);
            writer.write_all(&buf.as_ref()[(mem::size_of::<U>() - bytes)..])
        }

        // the amount of available bits in the queue
        let available_bits = u8::BITS_SIZE - *queue_bits;

        if bits < available_bits {
            // all bits fit in queue, so no write needed
            *queue_value = queue_value.shl_default(bits) | U::to_u8(value);
            *queue_bits += bits;
            Ok(())
        } else {
            // at least one byte needs to be written

            // bits beyond what can fit in the queue
            let excess_bits = bits - available_bits;

            match (excess_bits / 8, excess_bits % 8) {
                (0, excess) => {
                    // only one byte to be written,
                    // while the excess bits are shared
                    // between the written byte and the bit queue

                    *queue_bits = excess;

                    write_byte(
                        writer,
                        mem::replace(
                            queue_value,
                            U::to_u8(value & U::ALL.shr_default(U::BITS_SIZE - excess)),
                        )
                        .shl_default(available_bits)
                            | U::to_u8(value.shr_default(excess)),
                    )
                }
                (bytes, 0) => {
                    // no excess bytes beyond what can fit the queue
                    // so write a whole byte and
                    // the remainder of the whole value

                    *queue_bits = 0;

                    write_byte(
                        writer.by_ref(),
                        mem::take(queue_value).shl_default(available_bits)
                            | U::to_u8(value.shr_default(bytes * 8)),
                    )?;

                    write_bytes(writer, bytes as usize, value)
                }
                (bytes, excess) => {
                    // write what's in the queue along
                    // with the head of our whole value,
                    // write the middle section of our whole value,
                    // while also replacing the queue with
                    // the tail of our whole value

                    *queue_bits = excess;

                    write_byte(
                        writer.by_ref(),
                        mem::replace(
                            queue_value,
                            U::to_u8(value & U::ALL.shr_default(U::BITS_SIZE - excess)),
                        )
                        .shl_default(available_bits)
                            | U::to_u8(value.shr_default(excess + bytes * 8)),
                    )?;

                    write_bytes(writer, bytes as usize, value.shr_default(excess))
                }
            }
        }
    }

    fn write_signed_bits_checked<const MAX: u32, W, S>(
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
        value: CheckedSigned<MAX, S>,
    ) -> io::Result<()>
    where
        W: io::Write,
        S: SignedInteger,
    {
        let (
            SignedBitCount {
                bits: BitCount { bits },
                unsigned,
            },
            value,
        ) = value.into_count_value();

        if let Some(b) = Self::push_bit_flush(queue_value, queue_bits, value.is_negative()) {
            write_byte(writer.by_ref(), b)?;
        }
        Self::write_bits_checked(
            writer,
            queue_value,
            queue_bits,
            Checked {
                value: if value.is_negative() {
                    value.as_negative(bits)
                } else {
                    value.as_non_negative()
                },
                count: unsigned,
            },
        )
    }

    #[inline]
    fn pop_bit_refill<R>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<bool>
    where
        R: io::Read,
    {
        Ok(if *queue_bits == 0 {
            let value = read_byte(reader)?;
            let msb = value & u8::MSB_BIT;
            *queue_value = value << 1;
            *queue_bits = u8::BITS_SIZE - 1;
            msb
        } else {
            let msb = *queue_value & u8::MSB_BIT;
            *queue_value <<= 1;
            *queue_bits -= 1;
            msb
        } != 0)
    }

    #[inline]
    fn pop_unary<const STOP_BIT: u8, R>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<u32>
    where
        R: io::Read,
    {
        const {
            assert!(matches!(STOP_BIT, 0 | 1), "stop bit must be 0 or 1");
        }

        match STOP_BIT {
            0 => find_unary(
                reader,
                queue_value,
                queue_bits,
                |v| v.leading_ones(),
                |q| *q,
                |v, b| v.checked_shl(b),
            ),
            1 => find_unary(
                reader,
                queue_value,
                queue_bits,
                |v| v.leading_zeros(),
                |_| u8::BITS_SIZE,
                |v, b| v.checked_shl(b),
            ),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn read_signed_counted<const MAX: u32, R, S>(
        r: &mut R,
        SignedBitCount {
            bits: BitCount { bits },
            unsigned,
        }: SignedBitCount<MAX>,
    ) -> io::Result<S>
    where
        R: BitRead,
        S: SignedInteger,
    {
        if MAX <= S::BITS_SIZE || bits <= S::BITS_SIZE {
            let is_negative = r.read_bit()?;
            let unsigned = r.read_unsigned_counted::<MAX, S::Unsigned>(unsigned)?;
            Ok(if is_negative {
                unsigned.as_negative(bits)
            } else {
                unsigned.as_non_negative()
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive bits for type read",
            ))
        }
    }

    fn read_bytes<const CHUNK_SIZE: usize, R>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: u32,
        buf: &mut [u8],
    ) -> io::Result<()>
    where
        R: io::Read,
    {
        if queue_bits == 0 {
            reader.read_exact(buf)
        } else {
            let mut input_chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

            for output_chunk in buf.chunks_mut(CHUNK_SIZE) {
                let input_chunk = &mut input_chunk[0..output_chunk.len()];
                reader.read_exact(input_chunk)?;

                // shift down each byte in our input to eventually
                // accomodate the contents of the bit queue
                // and make that our output
                output_chunk
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o = i >> queue_bits;
                    });

                // include leftover bits from the next byte
                // shifted to the top
                output_chunk[1..]
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o |= *i << (u8::BITS_SIZE - queue_bits);
                    });

                // finally, prepend the queue's contents
                // to the first byte in the chunk
                // while replacing those contents
                // with the final byte of the input
                output_chunk[0] |= mem::replace(
                    queue_value,
                    input_chunk.last().unwrap() << (u8::BITS_SIZE - queue_bits),
                );
            }

            Ok(())
        }
    }

    fn write_bytes<const CHUNK_SIZE: usize, W>(
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: u32,
        buf: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        if queue_bits == 0 {
            writer.write_all(buf)
        } else {
            let mut output_chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

            for input_chunk in buf.chunks(CHUNK_SIZE) {
                let output_chunk = &mut output_chunk[0..input_chunk.len()];

                output_chunk
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o = i >> queue_bits;
                    });

                output_chunk[1..]
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o |= *i << (u8::BITS_SIZE - queue_bits);
                    });

                output_chunk[0] |= mem::replace(
                    queue_value,
                    input_chunk.last().unwrap() & (u8::ALL >> (u8::BITS_SIZE - queue_bits)),
                ) << (u8::BITS_SIZE - queue_bits);

                writer.write_all(output_chunk)?;
            }

            Ok(())
        }
    }

    #[inline(always)]
    fn bytes_to_primitive<P: Primitive>(buf: P::Bytes) -> P {
        P::from_be_bytes(buf)
    }

    #[inline(always)]
    fn primitive_to_bytes<P: Primitive>(p: P) -> P::Bytes {
        p.to_be_bytes()
    }

    #[inline]
    fn read_primitive<R, V>(r: &mut R) -> io::Result<V>
    where
        R: BitRead,
        V: Primitive,
    {
        let mut buffer = V::buffer();
        r.read_bytes(buffer.as_mut())?;
        Ok(V::from_be_bytes(buffer))
    }

    #[inline]
    fn write_primitive<W, V>(w: &mut W, value: V) -> io::Result<()>
    where
        W: BitWrite,
        V: Primitive,
    {
        w.write_bytes(value.to_be_bytes().as_ref())
    }
}

/// Little-endian, or least significant bits first
#[derive(Copy, Clone, Debug)]
pub struct LittleEndian;

/// Little-endian, or least significant bits first
pub type LE = LittleEndian;

impl LittleEndian {
    // checked in the sense that we've verified
    // the output type is large enough to hold the
    // requested number of bits
    #[inline]
    fn read_bits_checked<const MAX: u32, R, U>(
        reader: &mut R,
        queue: &mut u8,
        queue_bits: &mut u32,
        BitCount { bits }: BitCount<MAX>,
    ) -> io::Result<U>
    where
        R: io::Read,
        U: UnsignedInteger,
    {
        // reads a whole value with the given number of
        // bytes in our endianness, where the number of bytes
        // must be less than or equal to the type's size in bytes
        #[inline(always)]
        fn read_bytes<R, U>(reader: &mut R, bytes: usize) -> io::Result<U>
        where
            R: io::Read,
            U: UnsignedInteger,
        {
            let mut buf = U::buffer();
            reader
                .read_exact(&mut buf.as_mut()[0..bytes])
                .map(|()| U::from_le_bytes(buf))
        }

        if bits <= *queue_bits {
            // all bits in queue, so no byte needed
            let value = *queue & u8::ALL.shr_default(u8::BITS_SIZE - bits);
            *queue = queue.shr_default(bits);
            *queue_bits -= bits;
            Ok(U::from_u8(value))
        } else {
            // at least one byte needed

            // bits needed beyond what's in the queue
            let needed_bits = bits - *queue_bits;

            match (needed_bits / 8, needed_bits % 8) {
                (0, needed) => {
                    // only one additional byte needed,
                    // which we share between our returned value
                    // and the bit queue
                    let next_byte = read_byte(reader)?;

                    Ok(
                        U::from_u8(mem::replace(queue, next_byte.shr_default(needed)))
                            | (U::from_u8(next_byte & (u8::ALL >> (u8::BITS_SIZE - needed)))
                                << mem::replace(queue_bits, u8::BITS_SIZE - needed)),
                    )
                }
                (bytes, 0) => {
                    // exact number of bytes needed beyond what's
                    // available in the queue

                    // so read a whole value from the reader
                    // and prepend what's left of our queue onto it

                    Ok(U::from_u8(mem::take(queue))
                        | (read_bytes::<R, U>(reader, bytes as usize)? << mem::take(queue_bits)))
                }
                (bytes, needed) => {
                    // read a whole value from the reader
                    // prepend what's in the queue at the front of it
                    // *and* append a partial byte at the end of it
                    // while also updating the queue and its bit count

                    let whole: U = read_bytes(reader, bytes as usize)?;
                    let next_byte = read_byte(reader)?;

                    Ok(
                        U::from_u8(mem::replace(queue, next_byte.shr_default(needed)))
                            | (whole << *queue_bits)
                            | (U::from_u8(next_byte & (u8::ALL >> (u8::BITS_SIZE - needed)))
                                << (mem::replace(queue_bits, u8::BITS_SIZE - needed) + bytes * 8)),
                    )
                }
            }
        }
    }
}

impl Endianness for LittleEndian {}

impl private::Endianness for LittleEndian {
    #[inline]
    fn push_bit_flush(queue_value: &mut u8, queue_bits: &mut u32, bit: bool) -> Option<u8> {
        *queue_value |= u8::from(bit) << *queue_bits;
        *queue_bits = (*queue_bits + 1) % 8;
        (*queue_bits == 0).then(|| mem::take(queue_value))
    }

    #[inline]
    fn read_bits<const MAX: u32, R, U>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
        count @ BitCount { bits }: BitCount<MAX>,
    ) -> io::Result<U>
    where
        R: io::Read,
        U: UnsignedInteger,
    {
        if MAX <= U::BITS_SIZE || bits <= U::BITS_SIZE {
            Self::read_bits_checked::<MAX, R, U>(reader, queue_value, queue_bits, count)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive bits for type read",
            ))
        }
    }

    #[inline]
    fn read_bits_fixed<const BITS: u32, R, U>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<U>
    where
        R: io::Read,
        U: UnsignedInteger,
    {
        const {
            assert!(BITS <= U::BITS_SIZE, "excessive bits for type read");
        }

        Self::read_bits_checked::<BITS, R, U>(
            reader,
            queue_value,
            queue_bits,
            BitCount::new::<BITS>(),
        )
    }

    // checked in the sense that we've verified
    // the input type is large enough to hold the
    // requested number of bits and that the value is
    // not too large for those bits
    #[inline]
    fn write_bits_checked<const MAX: u32, W, U>(
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
        CheckedUnsigned {
            count: BitCount { bits },
            value,
        }: CheckedUnsigned<MAX, U>,
    ) -> io::Result<()>
    where
        W: io::Write,
        U: UnsignedInteger,
    {
        fn write_bytes<W, U>(writer: &mut W, bytes: usize, value: U) -> io::Result<()>
        where
            W: io::Write,
            U: UnsignedInteger,
        {
            let buf = U::to_le_bytes(value);
            writer.write_all(&buf.as_ref()[0..bytes])
        }

        // the amount of available bits in the queue
        let available_bits = u8::BITS_SIZE - *queue_bits;

        if bits < available_bits {
            // all bits fit in queue, so no write needed
            *queue_value |= U::to_u8(value.shl_default(*queue_bits));
            *queue_bits += bits;
            Ok(())
        } else {
            // at least one byte needs to be written

            // bits beyond what can fit in the queue
            let excess_bits = bits - available_bits;

            match (excess_bits / 8, excess_bits % 8) {
                (0, excess) => {
                    // only one byte to be written,
                    // while the excess bits are shared
                    // between the written byte and the bit queue

                    write_byte(
                        writer,
                        mem::replace(queue_value, U::to_u8(value.shr_default(available_bits)))
                            | U::to_u8(
                                (value << mem::replace(queue_bits, excess)) & U::from_u8(u8::ALL),
                            ),
                    )
                }
                (bytes, 0) => {
                    // no excess bytes beyond what can fit the queue
                    // so write a whole byte and
                    // the remainder of the whole value

                    write_byte(
                        writer.by_ref(),
                        mem::take(queue_value)
                            | U::to_u8((value << mem::take(queue_bits)) & U::from_u8(u8::ALL)),
                    )?;

                    write_bytes(writer, bytes as usize, value >> available_bits)
                }
                (bytes, excess) => {
                    // write what's in the queue along
                    // with the head of our whole value,
                    // write the middle section of our whole value,
                    // while also replacing the queue with
                    // the tail of our whole value

                    write_byte(
                        writer.by_ref(),
                        mem::replace(
                            queue_value,
                            U::to_u8(value.shr_default(available_bits + bytes * 8)),
                        ) | U::to_u8(
                            (value << mem::replace(queue_bits, excess)) & U::from_u8(u8::ALL),
                        ),
                    )?;

                    write_bytes(writer, bytes as usize, value >> available_bits)
                }
            }
        }
    }

    fn write_signed_bits_checked<const MAX: u32, W, S>(
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: &mut u32,
        value: CheckedSigned<MAX, S>,
    ) -> io::Result<()>
    where
        W: io::Write,
        S: SignedInteger,
    {
        // little-endian
        let (
            SignedBitCount {
                bits: BitCount { bits },
                unsigned,
            },
            value,
        ) = value.into_count_value();

        Self::write_bits_checked(
            writer.by_ref(),
            queue_value,
            queue_bits,
            Checked {
                value: if value.is_negative() {
                    value.as_negative(bits)
                } else {
                    value.as_non_negative()
                },
                count: unsigned,
            },
        )?;
        match Self::push_bit_flush(queue_value, queue_bits, value.is_negative()) {
            Some(b) => write_byte(writer, b),
            None => Ok(()),
        }
    }

    #[inline]
    fn pop_bit_refill<R>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<bool>
    where
        R: io::Read,
    {
        Ok(if *queue_bits == 0 {
            let value = read_byte(reader)?;
            let lsb = value & u8::LSB_BIT;
            *queue_value = value >> 1;
            *queue_bits = u8::BITS_SIZE - 1;
            lsb
        } else {
            let lsb = *queue_value & u8::LSB_BIT;
            *queue_value >>= 1;
            *queue_bits -= 1;
            lsb
        } != 0)
    }

    #[inline]
    fn pop_unary<const STOP_BIT: u8, R>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: &mut u32,
    ) -> io::Result<u32>
    where
        R: io::Read,
    {
        const {
            assert!(matches!(STOP_BIT, 0 | 1), "stop bit must be 0 or 1");
        }

        match STOP_BIT {
            0 => find_unary(
                reader,
                queue_value,
                queue_bits,
                |v| v.trailing_ones(),
                |q| *q,
                |v, b| v.checked_shr(b),
            ),
            1 => find_unary(
                reader,
                queue_value,
                queue_bits,
                |v| v.trailing_zeros(),
                |_| u8::BITS_SIZE,
                |v, b| v.checked_shr(b),
            ),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn read_signed_counted<const MAX: u32, R, S>(
        r: &mut R,
        SignedBitCount {
            bits: BitCount { bits },
            unsigned,
        }: SignedBitCount<MAX>,
    ) -> io::Result<S>
    where
        R: BitRead,
        S: SignedInteger,
    {
        if MAX <= S::BITS_SIZE || bits <= S::BITS_SIZE {
            let unsigned = r.read_unsigned_counted::<MAX, S::Unsigned>(unsigned)?;
            let is_negative = r.read_bit()?;
            Ok(if is_negative {
                unsigned.as_negative(bits)
            } else {
                unsigned.as_non_negative()
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive bits for type read",
            ))
        }
    }

    fn read_bytes<const CHUNK_SIZE: usize, R>(
        reader: &mut R,
        queue_value: &mut u8,
        queue_bits: u32,
        buf: &mut [u8],
    ) -> io::Result<()>
    where
        R: io::Read,
    {
        if queue_bits == 0 {
            reader.read_exact(buf)
        } else {
            let mut input_chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

            for output_chunk in buf.chunks_mut(CHUNK_SIZE) {
                let input_chunk = &mut input_chunk[0..output_chunk.len()];
                reader.read_exact(input_chunk)?;

                output_chunk
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o = i << queue_bits;
                    });

                output_chunk[1..]
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o |= i >> (u8::BITS_SIZE - queue_bits);
                    });

                output_chunk[0] |= mem::replace(
                    queue_value,
                    input_chunk.last().unwrap() >> (u8::BITS_SIZE - queue_bits),
                );
            }

            Ok(())
        }
    }

    fn write_bytes<const CHUNK_SIZE: usize, W>(
        writer: &mut W,
        queue_value: &mut u8,
        queue_bits: u32,
        buf: &[u8],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        if queue_bits == 0 {
            writer.write_all(buf)
        } else {
            let mut output_chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

            for input_chunk in buf.chunks(CHUNK_SIZE) {
                let output_chunk = &mut output_chunk[0..input_chunk.len()];

                output_chunk
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o = i << queue_bits;
                    });

                output_chunk[1..]
                    .iter_mut()
                    .zip(input_chunk.iter())
                    .for_each(|(o, i)| {
                        *o |= i >> (u8::BITS_SIZE - queue_bits);
                    });

                output_chunk[0] |= mem::replace(
                    queue_value,
                    input_chunk.last().unwrap() >> (u8::BITS_SIZE - queue_bits),
                );

                writer.write_all(output_chunk)?;
            }

            Ok(())
        }
    }

    #[inline(always)]
    fn bytes_to_primitive<P: Primitive>(buf: P::Bytes) -> P {
        P::from_le_bytes(buf)
    }

    #[inline(always)]
    fn primitive_to_bytes<P: Primitive>(p: P) -> P::Bytes {
        p.to_le_bytes()
    }

    #[inline]
    fn read_primitive<R, V>(r: &mut R) -> io::Result<V>
    where
        R: BitRead,
        V: Primitive,
    {
        let mut buffer = V::buffer();
        r.read_bytes(buffer.as_mut())?;
        Ok(V::from_le_bytes(buffer))
    }

    #[inline]
    fn write_primitive<W, V>(w: &mut W, value: V) -> io::Result<()>
    where
        W: BitWrite,
        V: Primitive,
    {
        w.write_bytes(value.to_le_bytes().as_ref())
    }
}

#[inline]
fn find_unary<R>(
    reader: &mut R,
    queue_value: &mut u8,
    queue_bits: &mut u32,
    leading_bits: impl Fn(u8) -> u32,
    max_bits: impl Fn(&mut u32) -> u32,
    checked_shift: impl Fn(u8, u32) -> Option<u8>,
) -> io::Result<u32>
where
    R: io::Read,
{
    let mut acc = 0;

    loop {
        match leading_bits(*queue_value) {
            bits if bits == max_bits(queue_bits) => {
                // all bits exhausted
                // fetch another byte and keep going
                acc += *queue_bits;
                *queue_value = read_byte(reader.by_ref())?;
                *queue_bits = u8::BITS_SIZE;
            }
            bits => match checked_shift(*queue_value, bits + 1) {
                Some(value) => {
                    // fetch part of source byte
                    *queue_value = value;
                    *queue_bits -= bits + 1;
                    break Ok(acc + bits);
                }
                None => {
                    // fetch all of source byte
                    *queue_value = 0;
                    *queue_bits = 0;
                    break Ok(acc + bits);
                }
            },
        }
    }
}
