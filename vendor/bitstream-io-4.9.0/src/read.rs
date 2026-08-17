// Copyright 2017 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits and implementations for reading bits from a stream.

#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
use core2::io;

#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "std")]
use std::io;

use super::{
    BitCount, CheckablePrimitive, Endianness, Integer, PhantomData, Primitive, SignedBitCount,
    SignedInteger, UnsignedInteger, VBRInteger,
};

use core::convert::TryInto;

/// An error returned if performing VBR read overflows
#[derive(Copy, Clone, Debug)]
pub(crate) struct VariableWidthOverflow;

impl core::fmt::Display for VariableWidthOverflow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        "variable bit rate overflowed".fmt(f)
    }
}

impl core::error::Error for VariableWidthOverflow {}

impl From<VariableWidthOverflow> for io::Error {
    fn from(VariableWidthOverflow: VariableWidthOverflow) -> Self {
        io::Error::new(
            #[cfg(feature = "std")]
            {
                io::ErrorKind::StorageFull
            },
            #[cfg(not(feature = "std"))]
            {
                io::ErrorKind::Other
            },
            "variable bit rate overflow",
        )
    }
}

/// A trait for anything that can read a variable number of
/// potentially un-aligned values from an input stream
pub trait BitRead {
    /// Reads a single bit from the stream.
    /// `true` indicates 1, `false` indicates 0
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b1000_1110];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert!(r.read_bit().is_err());  // no more bits to read
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, LittleEndian};
    ///
    /// let bytes: &[u8] = &[0b1000_1110];
    /// let mut r = BitReader::endian(bytes, LittleEndian);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert!(r.read_bit().is_err());  // no more bits to read
    /// ```
    fn read_bit(&mut self) -> io::Result<bool> {
        self.read_unsigned::<1, u8>().map(|b| b == 1)
    }

    /// Reads a signed or unsigned value from the stream with
    /// the given constant number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b0001_1111, 0b1011_11_00];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // reading unsigned value is ok
    /// assert_eq!(r.read::<4, u8>().unwrap(), 1);
    /// // reading signed value is also ok
    /// assert_eq!(r.read::<4, i8>().unwrap(), -1);
    /// // reading an array of bits is ok too
    /// assert_eq!(r.read::<1, [bool; 4]>().unwrap(), [true, false, true, true]);
    /// // reading an array of any Integer type is ok
    /// assert_eq!(r.read::<2, [u8; 2]>().unwrap(), [0b11, 0b00]);
    /// // reading more bytes than we have is an error
    /// assert!(r.read::<4, u8>().is_err());
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b0001_1111, 0, 0];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // reading 9 bits to a u8 is a compile-time error
    /// r.read::<9, u8>();
    /// ```
    #[inline]
    fn read<const BITS: u32, I>(&mut self) -> io::Result<I>
    where
        I: Integer,
    {
        I::read::<BITS, _>(self)
    }

    /// Reads a signed or unsigned value from the stream with
    /// the given variable number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    /// let bytes: &[u8] = &[0b0001_1111];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // reading unsigned value is ok
    /// assert_eq!(r.read_var::<u8>(4).unwrap(), 1);
    /// // reading signed value is also ok
    /// assert_eq!(r.read_var::<i8>(4).unwrap(), -1);
    /// // reading more bytes than we have is an error
    /// assert!(r.read_var::<u8>(4).is_err());
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    /// let bytes: &[u8] = &[0, 0, 0, 0, 0];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // reading 9 bits to a u8 is a runtime error
    /// // no matter how much data is left
    /// assert!(r.read_var::<u8>(9).is_err());
    /// ```
    #[inline]
    fn read_var<I>(&mut self, bits: u32) -> io::Result<I>
    where
        I: Integer + Sized,
    {
        I::read_var(self, BitCount::unknown(bits))
    }

    /// Given a desired maximum value for a bit count,
    /// reads the necessary bits to fill up to that amount.
    ///
    /// For example, if the maximum bit count is 15 - or `0b1111` -
    /// reads a 4-bit unsigned value from the stream and returns a [`BitCount`]
    /// which can be used in subsequent reads.
    ///
    /// Note that `MAX` must be greater than 0, and `MAX + 1` must be
    /// an exact power of two.
    ///
    /// # Errors
    ///
    /// Passes along an I/O error from the underlying stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    ///
    /// let bytes: &[u8] = &[0b100_11110];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// let count = r.read::<3, u32>().unwrap();
    /// assert_eq!(count, 4);  // reads 0b100 - or 4
    /// // may need to verify count is not larger than u8 at runtime
    /// assert_eq!(r.read_var::<u8>(count).unwrap(), 0b1111);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead, BitCount};
    ///
    /// let bytes: &[u8] = &[0b100_11110];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// let count = r.read_count::<0b111>().unwrap();
    /// assert_eq!(count, BitCount::new::<4>());  // reads 0b100 - or 4
    /// // maximum size of bit count is known to be 7 at compile-time,
    /// // so no runtime check needed to know 7 bits is not larger than a u8
    /// assert_eq!(r.read_counted::<0b111, u8>(count).unwrap(), 0b1111);
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    ///
    /// let bytes: &[u8] = &[0b100_11110];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // maximum bit count is 6 (0b110), so we need to read 3 bits
    /// // but no idea what to do if a value of 7 (0b111) is read,
    /// // so this does not compile at all
    /// let count = r.read_count::<0b110>();
    /// ```
    fn read_count<const MAX: u32>(&mut self) -> io::Result<BitCount<MAX>> {
        const {
            assert!(MAX > 0, "MAX value must be > 0");
            assert!(
                MAX == u32::MAX || (MAX + 1).is_power_of_two(),
                "MAX should fill some whole number of bits ('0b111', '0b1111', etc.)"
            )
        }

        self.read_unsigned_var(if MAX < u32::MAX {
            (MAX + 1).ilog2()
        } else {
            32
        })
        .map(|bits| BitCount { bits })
    }

    /// Reads a signed or unsigned value from the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian, BitCount};
    ///
    /// let bytes: &[u8] = &[0b1111_0000];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // reading 4 bits with a maximum of 4 will fit into a u8
    /// // so no runtime check needed
    /// assert_eq!(r.read_counted::<4, u8>(BitCount::new::<4>()).unwrap(), 0b1111);
    /// // reading 4 bits with a maximum of 64 might not fit into a u8
    /// // so we need to verify this at runtime
    /// assert_eq!(r.read_counted::<64, u8>(BitCount::new::<4>()).unwrap(), 0b0000);
    /// ```
    #[inline(always)]
    fn read_counted<const MAX: u32, I>(&mut self, bits: BitCount<MAX>) -> io::Result<I>
    where
        I: Integer + Sized,
    {
        I::read_var(self, bits)
    }

    /// Reads an unsigned value from the stream with
    /// the given constant number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0b1_01_10111];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// assert_eq!(reader.read_unsigned::<1, u8>().unwrap(), 0b1);
    /// assert_eq!(reader.read_unsigned::<2, u8>().unwrap(), 0b01);
    /// assert_eq!(reader.read_unsigned::<5, u8>().unwrap(), 0b10111);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0b10110_11_1];
    /// let mut reader = BitReader::endian(data, LittleEndian);
    /// assert_eq!(reader.read_unsigned::<1, u8>().unwrap(), 0b1);
    /// assert_eq!(reader.read_unsigned::<2, u8>().unwrap(), 0b11);
    /// assert_eq!(reader.read_unsigned::<5, u8>().unwrap(), 0b10110);
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0, 0, 0, 0, 0];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// // doesn't compile at all
    /// reader.read_unsigned::<9, u8>();  // can't read  9 bits to u8
    /// ```
    fn read_unsigned<const BITS: u32, U>(&mut self) -> io::Result<U>
    where
        U: UnsignedInteger,
    {
        self.read_unsigned_var(BITS)
    }

    /// Reads an unsigned value from the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0b1_01_10111];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// assert_eq!(reader.read_unsigned_var::<u8>(1).unwrap(), 0b1);
    /// assert_eq!(reader.read_unsigned_var::<u8>(2).unwrap(), 0b01);
    /// assert_eq!(reader.read_unsigned_var::<u8>(5).unwrap(), 0b10111);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0b10110_11_1];
    /// let mut reader = BitReader::endian(data, LittleEndian);
    /// assert_eq!(reader.read_unsigned_var::<u8>(1).unwrap(), 0b1);
    /// assert_eq!(reader.read_unsigned_var::<u8>(2).unwrap(), 0b11);
    /// assert_eq!(reader.read_unsigned_var::<u8>(5).unwrap(), 0b10110);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0, 0, 0, 0, 0];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// assert!(reader.read_unsigned_var::<u8>(9).is_err());    // can't read  9 bits to u8
    /// assert!(reader.read_unsigned_var::<u16>(17).is_err());  // can't read 17 bits to u16
    /// assert!(reader.read_unsigned_var::<u32>(33).is_err());  // can't read 33 bits to u32
    /// assert!(reader.read_unsigned_var::<u64>(65).is_err());  // can't read 65 bits to u64
    /// ```
    #[inline(always)]
    fn read_unsigned_var<U>(&mut self, bits: u32) -> io::Result<U>
    where
        U: UnsignedInteger,
    {
        self.read_unsigned_counted(BitCount::unknown(bits))
    }

    /// Reads an unsigned value from the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian, BitCount};
    ///
    /// let bytes: &[u8] = &[0b1111_0000];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // reading 4 bits with a maximum of 4 will fit into a u8
    /// // so no runtime check needed
    /// assert_eq!(r.read_unsigned_counted::<4, u8>(BitCount::new::<4>()).unwrap(), 0b1111);
    /// // reading 4 bits with a maximum of 64 might not fit into a u8
    /// // so we need to verify this at runtime
    /// assert_eq!(r.read_unsigned_counted::<64, u8>(BitCount::new::<4>()).unwrap(), 0b0000);
    /// ```
    fn read_unsigned_counted<const MAX: u32, U>(&mut self, bits: BitCount<MAX>) -> io::Result<U>
    where
        U: UnsignedInteger;

    /// Reads a twos-complement signed value from the stream with
    /// the given constant number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// A compile-time error occurs if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    ///
    /// let data: &[u8] = &[0b1011_0111];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// assert_eq!(reader.read_signed::<4, i8>().unwrap(), -5);
    /// assert_eq!(reader.read_signed::<4, i8>().unwrap(), 7);
    /// assert!(reader.read_signed::<4, i8>().is_err());
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitReader, BitRead};
    ///
    /// let data: &[u8] = &[0b1011_0111];
    /// let mut reader = BitReader::endian(data, LittleEndian);
    /// assert_eq!(reader.read_signed::<4, i8>().unwrap(), 7);
    /// assert_eq!(reader.read_signed::<4, i8>().unwrap(), -5);
    /// assert!(reader.read_signed::<4, i8>().is_err());
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{LittleEndian, BitReader, BitRead};
    ///
    /// let data: &[u8] = &[0, 0, 0, 0, 0];
    /// let mut reader = BitReader::endian(data, LittleEndian);
    /// // reading 9 bits to an i8 is a compile-time error
    /// reader.read_signed::<9, i8>();
    /// ```
    fn read_signed<const BITS: u32, S>(&mut self) -> io::Result<S>
    where
        S: SignedInteger,
    {
        self.read_signed_var(BITS)
    }

    /// Reads a twos-complement signed value from the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0b1011_0111];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// assert_eq!(reader.read_signed_var::<i8>(4).unwrap(), -5);
    /// assert_eq!(reader.read_signed_var::<i8>(4).unwrap(), 7);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0b1011_0111];
    /// let mut reader = BitReader::endian(data, LittleEndian);
    /// assert_eq!(reader.read_signed_var::<i8>(4).unwrap(), 7);
    /// assert_eq!(reader.read_signed_var::<i8>(4).unwrap(), -5);
    /// ```
    ///
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    /// let mut r = BitReader::endian(data, BigEndian);
    /// assert!(r.read_signed_var::<i8>(9).is_err());   // can't read 9 bits to i8
    /// assert!(r.read_signed_var::<i16>(17).is_err()); // can't read 17 bits to i16
    /// assert!(r.read_signed_var::<i32>(33).is_err()); // can't read 33 bits to i32
    /// assert!(r.read_signed_var::<i64>(65).is_err()); // can't read 65 bits to i64
    /// ```
    fn read_signed_var<S>(&mut self, bits: u32) -> io::Result<S>
    where
        S: SignedInteger,
    {
        self.read_signed_counted(BitCount::unknown(bits))
    }

    /// Reads a twos-complement signed value from the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian, BitCount};
    ///
    /// let bytes: &[u8] = &[0b0001_1111];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // reading 4 bits with a maximum of 4 will fit into an i8
    /// // so no runtime check needed
    /// assert_eq!(r.read_signed_counted::<4, i8>(BitCount::new::<4>()).unwrap(), 1);
    /// // reading 4 bits with a maximum of 64 might not fit into an i8
    /// // so we need to verify this at runtime
    /// assert_eq!(r.read_signed_counted::<64, i8>(BitCount::new::<4>()).unwrap(), -1);
    /// ```
    fn read_signed_counted<const MAX: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<MAX>>,
    ) -> io::Result<S>
    where
        S: SignedInteger;

    /// Reads the given constant value from the stream with the
    /// given number of bits.
    ///
    /// Due to current limitations of constant paramters,
    /// this is limited to `u32` values.
    ///
    /// If the constant read from the stream doesn't match the expected
    /// value, returns the generated error from the closure.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream,
    /// converted to the mismatch error.  Returns the generated
    /// error if the read value doesn't match the expected constant.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// enum Error {
    ///     Mismatch,
    ///     Io,
    /// }
    ///
    /// impl From<std::io::Error> for Error {
    ///     fn from(_err: std::io::Error) -> Self {
    ///         Self::Io
    ///     }
    /// }
    ///
    /// let data: &[u8] = &[0b1000_1011, 0b0000_0001];
    /// let mut r = BitReader::endian(data, BigEndian);
    /// assert!(r.read_const::<4, 0b1000, _>(Error::Mismatch).is_ok());
    /// assert!(r.read_const::<4, 0b1011, _>(Error::Mismatch).is_ok());
    /// // 0b1000 doesn't match 0b0000
    /// assert!(matches!(r.read_const::<4, 0b1000, _>(Error::Mismatch), Err(Error::Mismatch)));
    /// // 0b1011 doesn't match 0b0001
    /// assert!(matches!(r.read_const::<4, 0b1011, _>(Error::Mismatch), Err(Error::Mismatch)));
    /// // run out of bits to check
    /// assert!(matches!(r.read_const::<4, 0b0000, _>(Error::Mismatch), Err(Error::Io)));
    /// ```
    #[inline]
    fn read_const<const BITS: u32, const VALUE: u32, E>(&mut self, err: E) -> Result<(), E>
    where
        E: From<io::Error>,
    {
        use super::Numeric;

        const {
            assert!(
                BITS == 0 || VALUE <= (u32::ALL >> (u32::BITS_SIZE - BITS)),
                "excessive value for bits read"
            );
        }

        (self.read::<BITS, u32>()? == VALUE)
            .then_some(())
            .ok_or(err)
    }

    /// Reads whole value from the stream whose size in bits is equal
    /// to its type's size.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0x12, 0x34, 0x56, 0x78];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_to::<u32>().unwrap(), 0x12_34_56_78);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0x12, 0x34, 0x56, 0x78];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_to::<[u8; 4]>().unwrap(), [0x12, 0x34, 0x56, 0x78]);
    /// ```
    fn read_to<V>(&mut self) -> io::Result<V>
    where
        V: Primitive;

    /// Reads whole value from the stream whose size in bits is equal
    /// to its type's size in an endianness that may be different
    /// from the stream's endianness.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian, LittleEndian};
    ///
    /// let bytes: &[u8] = &[0x12, 0x34, 0x56, 0x78];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_as_to::<LittleEndian, u32>().unwrap(), 0x78_56_34_12);
    /// ```
    fn read_as_to<F, V>(&mut self) -> io::Result<V>
    where
        F: Endianness,
        V: Primitive;

    /// Skips the given number of bits in the stream.
    /// Since this method does not need an accumulator,
    /// it may be slightly faster than reading to an empty variable.
    /// In addition, since there is no accumulator,
    /// there is no upper limit on the number of bits
    /// which may be skipped.
    /// These bits are still read from the stream, however,
    /// and are never skipped via a `seek` method.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b1_00000_10];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert!(r.skip(5).is_ok());
    /// assert_eq!(r.read_bit().unwrap(), true);
    /// assert_eq!(r.read_bit().unwrap(), false);
    /// assert!(r.read_bit().is_err());
    /// ```
    fn skip(&mut self, bits: u32) -> io::Result<()> {
        (0..bits).try_for_each(|_| self.read_bit().map(|_| ()))
    }

    /// Completely fills the given buffer with whole bytes.
    /// If the stream is already byte-aligned, it will map
    /// to a faster `read_exact` call.  Otherwise it will read
    /// bytes individually in 8-bit increments.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// let mut buf = [0; 3];
    /// assert_eq!(r.read::<8, u8>().unwrap(), 0x00);
    /// assert!(r.read_bytes(&mut buf).is_ok());
    /// assert_eq!(&buf, &[0x01, 0x02, 0x03]);
    /// assert_eq!(r.read::<8, u8>().unwrap(), 0x04);
    /// ```
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        for b in buf.iter_mut() {
            *b = self.read_unsigned::<8, _>()?;
        }
        Ok(())
    }

    /// Completely fills a whole buffer with bytes and returns it.
    /// If the stream is already byte-aligned, it will map
    /// to a faster `read_exact` call.  Otherwise it will read
    /// bytes individually in 8-bit increments.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    #[inline(always)]
    #[deprecated(since = "1.8.0", note = "use read_to() method instead")]
    fn read_to_bytes<const SIZE: usize>(&mut self) -> io::Result<[u8; SIZE]> {
        self.read_to()
    }

    /// Completely fills a vector of bytes and returns it.
    /// If the stream is already byte-aligned, it will map
    /// to a faster `read_exact` call.  Otherwise it will read
    /// bytes individually in 8-bit increments.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0x00, 0x01, 0x02, 0x03, 0x04];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// let mut buf = [0; 3];
    /// assert_eq!(r.read::<8, u8>().unwrap(), 0x00);
    /// assert_eq!(r.read_to_vec(3).unwrap().as_slice(), &[0x01, 0x02, 0x03]);
    /// assert_eq!(r.read::<8, u8>().unwrap(), 0x04);
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn read_to_vec(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        read_to_vec(|buf| self.read_bytes(buf), bytes)
    }

    /// Counts the number of bits in the stream until `STOP_BIT`
    /// and returns the amount read.
    /// `STOP_BIT` must be 0 or 1.
    /// Because this field is variably-sized and may be large,
    /// its output is always a `u32` type.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// May panic if the number of bits exceeds `u32`.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b0_10_11111, 0b10_000000];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // read 1 bits until stop bit of 0, big-endian order
    /// assert_eq!(r.read_unary::<0>().unwrap(), 0);
    /// assert_eq!(r.read_unary::<0>().unwrap(), 1);
    /// assert_eq!(r.read_unary::<0>().unwrap(), 6);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b1_01_00000, 0b01_000000];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// // read 0 bits until stop bit of 1, big-endian order
    /// assert_eq!(r.read_unary::<1>().unwrap(), 0);
    /// assert_eq!(r.read_unary::<1>().unwrap(), 1);
    /// assert_eq!(r.read_unary::<1>().unwrap(), 6);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, LittleEndian};
    ///
    /// let bytes: &[u8] = &[0b11111_01_0, 0b000000_01];
    /// let mut r = BitReader::endian(bytes, LittleEndian);
    /// // read 1 bits until stop bit of 0, little-endian order
    /// assert_eq!(r.read_unary::<0>().unwrap(), 0);
    /// assert_eq!(r.read_unary::<0>().unwrap(), 1);
    /// assert_eq!(r.read_unary::<0>().unwrap(), 6);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, LittleEndian};
    ///
    /// let bytes: &[u8] = &[0b00000_10_1, 0b111111_10];
    /// let mut r = BitReader::endian(bytes, LittleEndian);
    /// // read 0 bits until stop bit of 1, little-endian order
    /// assert_eq!(r.read_unary::<1>().unwrap(), 0);
    /// assert_eq!(r.read_unary::<1>().unwrap(), 1);
    /// assert_eq!(r.read_unary::<1>().unwrap(), 6);
    /// ```
    fn read_unary<const STOP_BIT: u8>(&mut self) -> io::Result<u32> {
        const {
            assert!(matches!(STOP_BIT, 0 | 1), "stop bit must be 0 or 1");
        }

        // a simple implementation which works anywhere
        let mut unary = 0;
        while self.read::<1, u8>()? != STOP_BIT {
            unary += 1;
        }
        Ok(unary)
    }

    /// Reads to a checked value that is known to fit a given number of bits
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{
    ///     BitRead, BitReader, BigEndian, Checked, CheckedUnsigned, CheckedSigned,
    ///     BitCount, SignedBitCount, BitWrite, BitWriter,
    /// };
    ///
    /// let data: &[u8] = &[0b1001_1111];
    /// let mut r = BitReader::endian(data, BigEndian);
    ///
    /// let bit_count = BitCount::<4>::new::<4>();
    /// let checked_u8 = r.read_checked::<CheckedUnsigned<4, u8>>(bit_count).unwrap();
    /// assert_eq!(checked_u8.into_value(), 0b1001);
    ///
    /// let bit_count = SignedBitCount::<4>::new::<4>();
    /// let checked_i8 = r.read_checked::<CheckedSigned<4, i8>>(bit_count).unwrap();
    /// assert_eq!(checked_i8.into_value(), -1);
    ///
    /// // note that checked values already know their bit count
    /// // so none is required when writing them to a stream
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// w.write_checked(checked_u8).unwrap();
    /// w.write_checked(checked_i8).unwrap();
    /// assert_eq!(w.into_writer().as_slice(), data);
    /// ```
    #[inline]
    fn read_checked<C>(&mut self, count: C::CountType) -> io::Result<C>
    where
        C: CheckablePrimitive,
    {
        C::read(self, count)
    }

    /// Parses and returns complex type
    fn parse<F: FromBitStream>(&mut self) -> Result<F, F::Error> {
        F::from_reader(self)
    }

    /// Parses and returns complex type with context
    fn parse_with<'a, F: FromBitStreamWith<'a>>(
        &mut self,
        context: &F::Context,
    ) -> Result<F, F::Error> {
        F::from_reader(self, context)
    }

    /// Parses and returns complex type with owned context
    fn parse_using<F: FromBitStreamUsing>(&mut self, context: F::Context) -> Result<F, F::Error> {
        F::from_reader(self, context)
    }

    /// Returns true if the stream is aligned at a whole byte.
    ///
    /// # Example
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data = [0];
    /// let mut reader = BitReader::endian(data.as_slice(), BigEndian);
    /// assert_eq!(reader.byte_aligned(), true);
    /// assert!(reader.skip(1).is_ok());
    /// assert_eq!(reader.byte_aligned(), false);
    /// assert!(reader.skip(7).is_ok());
    /// assert_eq!(reader.byte_aligned(), true);
    /// ```
    fn byte_aligned(&self) -> bool;

    /// Throws away all unread bit values until the next whole byte.
    /// Does nothing if the stream is already aligned.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data: &[u8] = &[0x00, 0xFF];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// assert_eq!(reader.read::<4, u8>().unwrap(), 0);
    /// reader.byte_align();
    /// assert_eq!(reader.read::<8, u8>().unwrap(), 0xFF);
    /// ```
    fn byte_align(&mut self);

    /// Given a compiled Huffman tree, reads bits from the stream
    /// until the next symbol is encountered.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian, define_huffman_tree, huffman::FromBits};
    ///
    /// define_huffman_tree!(TreeName : char = ['a', ['b', ['c', 'd']]]);
    /// // 'a' is 0
    /// // 'b' is 1 -> 0
    /// // 'c' is 1 -> 1 -> 0
    /// // 'd' is 1 -> 1 -> 1
    ///
    /// let data: &[u8] = &[0b0_10_110_11, 0b1_0000000];
    /// let mut r = BitReader::endian(data, BigEndian);
    /// assert_eq!(r.read_huffman::<TreeName>().unwrap(), 'a');
    /// assert_eq!(r.read_huffman::<TreeName>().unwrap(), 'b');
    /// assert_eq!(r.read_huffman::<TreeName>().unwrap(), 'c');
    /// assert_eq!(r.read_huffman::<TreeName>().unwrap(), 'd');
    /// ```
    #[inline]
    fn read_huffman<T>(&mut self) -> io::Result<T::Symbol>
    where
        T: crate::huffman::FromBits,
    {
        T::from_bits(|| self.read_bit())
    }

    /// Reads a number using a variable using a variable width integer.
    /// This optimises the case when the number is small.
    ///
    /// The integer is mapped to an unsigned value using zigzag encoding.
    /// For an integer X:
    ///   - if X >= 0 -> 2X
    ///   - else -> -2X + 1
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the data read would overflow the size of the result
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b0111_1100, 0b1100_0001];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_unsigned_vbr::<4, u32>().unwrap(), 7);
    /// assert_eq!(r.read_unsigned_vbr::<4, u32>().unwrap(), 100);
    /// ```
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b1111_1111, 0b0011_1000, 0b1000_0100, 0b1000_1000, 0b1000_0000];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_unsigned_vbr::<4, u8>().unwrap(), 255); // Tries to read <011><111><111>
    /// assert!(r.read_unsigned_vbr::<4, u8>().is_err()); // Tries to read a value of <100><000><000>
    /// assert!(r.read_unsigned_vbr::<4, u8>().is_err()); // Tries to read a value of <000><000><000><000>
    /// ```
    fn read_unsigned_vbr<const FIELD_SIZE: u32, U: UnsignedInteger>(&mut self) -> io::Result<U> {
        const { assert!(FIELD_SIZE >= 2 && FIELD_SIZE < U::BITS_SIZE) };
        let payload_bits = FIELD_SIZE - 1;
        let mut value = U::ZERO;
        let mut shift = 0u32;
        loop {
            let (data, continuation) = self.read_unsigned::<FIELD_SIZE, U>().map(|item| {
                (
                    item & ((U::ONE << payload_bits) - U::ONE),
                    (item >> payload_bits) != U::ZERO,
                )
            })?;
            let shifted = data << shift;
            value |= shifted;
            if !continuation {
                if (data << shift) >> shift == data {
                    break Ok(value);
                } else {
                    break Err(VariableWidthOverflow {}.into());
                }
            }
            shift += payload_bits;
            if shift >= U::BITS_SIZE {
                break Err(VariableWidthOverflow {}.into());
            }
        }
    }

    /// Reads a number using a variable using a variable width integer.
    /// This optimises the case when the number is small.
    ///
    /// The integer is mapped to an unsigned value using zigzag encoding.
    /// For an integer X:
    ///   - if X >= 0 -> 2X
    ///   - else -> -2X + 1
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the data read would overflow the size of the result
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b0110_1011, 0b1100_0001];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_signed_vbr::<4, i32>().unwrap(), 3);
    /// assert_eq!(r.read_signed_vbr::<4, i32>().unwrap(), -50);
    /// ```
    fn read_signed_vbr<const FIELD_SIZE: u32, I: SignedInteger>(&mut self) -> io::Result<I> {
        self.read_unsigned_vbr::<FIELD_SIZE, I::Unsigned>()
            .map(|zig_zag| {
                let shifted = zig_zag >> 1;
                let complimented = zig_zag & <I::Unsigned as crate::Numeric>::ONE;
                let neg = I::ZERO - complimented.as_non_negative();
                shifted.as_non_negative() ^ neg
            })
    }

    /// Reads a signed or unsigned variable width integer from the stream.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the data read would overflow the size of the result
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// let bytes: &[u8] = &[0b0110_1011, 0b1100_0001];
    /// let mut r = BitReader::endian(bytes, BigEndian);
    /// assert_eq!(r.read_vbr::<4, u32>().unwrap(), 6);
    /// assert_eq!(r.read_vbr::<4, i32>().unwrap(), -50);
    /// ```
    #[inline]
    fn read_vbr<const FIELD_SIZE: u32, I: VBRInteger>(&mut self) -> io::Result<I> {
        I::read_vbr::<FIELD_SIZE, _>(self)
    }

    /// Creates a "by reference" adaptor for this `BitRead`
    ///
    /// The returned adapter also implements `BitRead`
    /// and will borrow the current reader.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitReader, BitRead, BigEndian};
    ///
    /// fn parse<R: BitRead>(r: R) {
    ///     // perform some parsing
    /// }
    ///
    /// let data: &[u8] = &[0];
    /// let mut reader = BitReader::endian(data, BigEndian);
    /// // performing parsing by reference
    /// parse(reader.by_ref());
    /// // original owned reader still available
    /// assert_eq!(reader.read::<8, u8>().unwrap(), 0);
    /// ```
    #[inline]
    fn by_ref(&mut self) -> &mut Self {
        self
    }
}

impl<R: BitRead + ?Sized> BitRead for &mut R {
    #[inline]
    fn read_bit(&mut self) -> io::Result<bool> {
        (**self).read_bit()
    }

    #[inline]
    fn read<const BITS: u32, I>(&mut self) -> io::Result<I>
    where
        I: Integer,
    {
        (**self).read::<BITS, I>()
    }

    #[inline]
    fn read_var<I>(&mut self, bits: u32) -> io::Result<I>
    where
        I: Integer + Sized,
    {
        (**self).read_var(bits)
    }

    #[inline]
    fn read_count<const MAX: u32>(&mut self) -> io::Result<BitCount<MAX>> {
        (**self).read_count::<MAX>()
    }

    #[inline]
    fn read_counted<const MAX: u32, I>(&mut self, bits: BitCount<MAX>) -> io::Result<I>
    where
        I: Integer + Sized,
    {
        (**self).read_counted::<MAX, I>(bits)
    }

    #[inline]
    fn read_unsigned<const BITS: u32, U>(&mut self) -> io::Result<U>
    where
        U: UnsignedInteger,
    {
        (**self).read_unsigned::<BITS, U>()
    }

    #[inline]
    fn read_unsigned_var<U>(&mut self, bits: u32) -> io::Result<U>
    where
        U: UnsignedInteger,
    {
        (**self).read_unsigned_var(bits)
    }

    #[inline]
    fn read_unsigned_counted<const MAX: u32, U>(&mut self, bits: BitCount<MAX>) -> io::Result<U>
    where
        U: UnsignedInteger,
    {
        (**self).read_unsigned_counted::<MAX, U>(bits)
    }

    #[inline]
    fn read_signed<const BITS: u32, S>(&mut self) -> io::Result<S>
    where
        S: SignedInteger,
    {
        (**self).read_signed::<BITS, S>()
    }

    #[inline]
    fn read_signed_var<S>(&mut self, bits: u32) -> io::Result<S>
    where
        S: SignedInteger,
    {
        (**self).read_signed_var(bits)
    }

    #[inline]
    fn read_signed_counted<const MAX: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<MAX>>,
    ) -> io::Result<S>
    where
        S: SignedInteger,
    {
        (**self).read_signed_counted::<MAX, S>(bits)
    }

    #[inline]
    fn read_to<V>(&mut self) -> io::Result<V>
    where
        V: Primitive,
    {
        (**self).read_to::<V>()
    }

    #[inline]
    fn read_as_to<F, V>(&mut self) -> io::Result<V>
    where
        F: Endianness,
        V: Primitive,
    {
        (**self).read_as_to::<F, V>()
    }

    #[inline]
    fn skip(&mut self, bits: u32) -> io::Result<()> {
        (**self).skip(bits)
    }

    #[inline]
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (**self).read_bytes(buf)
    }

    #[inline]
    #[cfg(feature = "alloc")]
    fn read_to_vec(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        (**self).read_to_vec(bytes)
    }

    #[inline]
    fn read_unary<const STOP_BIT: u8>(&mut self) -> io::Result<u32> {
        (**self).read_unary::<STOP_BIT>()
    }

    #[inline]
    fn parse<F: FromBitStream>(&mut self) -> Result<F, F::Error> {
        (**self).parse::<F>()
    }

    #[inline]
    fn parse_with<'a, F: FromBitStreamWith<'a>>(
        &mut self,
        context: &F::Context,
    ) -> Result<F, F::Error> {
        (**self).parse_with::<F>(context)
    }

    #[inline]
    fn byte_aligned(&self) -> bool {
        (**self).byte_aligned()
    }

    #[inline]
    fn byte_align(&mut self) {
        (**self).byte_align()
    }

    #[inline]
    fn read_huffman<T>(&mut self) -> io::Result<T::Symbol>
    where
        T: crate::huffman::FromBits,
    {
        (**self).read_huffman::<T>()
    }
}

/// A compatibility trait for older code implementing [`BitRead`]
///
/// This is a trait largely compatible with older code
/// from the 2.X.X version,
/// which one can use with a named import as needed.
///
/// New code should prefer the regular [`BitRead`] trait.
///
/// # Example
/// ```
/// use bitstream_io::BitRead2 as BitRead;
/// use bitstream_io::{BitReader, BigEndian};
/// let byte = &[0b1111_0000];
/// let mut reader = BitReader::endian(byte.as_slice(), BigEndian);
/// assert_eq!(reader.read::<u8>(4).unwrap(), 0b1111);
/// assert_eq!(reader.read_in::<4, u8>().unwrap(), 0b0000);
/// ```
pub trait BitRead2 {
    /// Reads a single bit from the stream.
    /// `true` indicates 1, `false` indicates 0
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_bit(&mut self) -> io::Result<bool>;

    /// Reads an unsigned value from the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    fn read<I>(&mut self, bits: u32) -> io::Result<I>
    where
        I: Integer + Sized;

    /// Reads an unsigned value from the stream with
    /// the given constant number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    fn read_in<const BITS: u32, I>(&mut self) -> io::Result<I>
    where
        I: Integer,
    {
        self.read(BITS)
    }

    /// Reads a twos-complement signed value from the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// Also returns an error if the output type is too small
    /// to hold the requested number of bits.
    fn read_signed<S>(&mut self, bits: u32) -> io::Result<S>
    where
        S: SignedInteger;

    /// Reads a twos-complement signed value from the stream with
    /// the given constant number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// A compile-time error occurs if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    fn read_signed_in<const BITS: u32, S>(&mut self) -> io::Result<S>
    where
        S: SignedInteger,
    {
        self.read_signed(BITS)
    }

    /// Reads whole value from the stream whose size in bits is equal
    /// to its type's size.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_to<V>(&mut self) -> io::Result<V>
    where
        V: Primitive;

    /// Reads whole value from the stream whose size in bits is equal
    /// to its type's size in an endianness that may be different
    /// from the stream's endianness.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_as_to<F, V>(&mut self) -> io::Result<V>
    where
        F: Endianness,
        V: Primitive;

    /// Skips the given number of bits in the stream.
    /// Since this method does not need an accumulator,
    /// it may be slightly faster than reading to an empty variable.
    /// In addition, since there is no accumulator,
    /// there is no upper limit on the number of bits
    /// which may be skipped.
    /// These bits are still read from the stream, however,
    /// and are never skipped via a `seek` method.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn skip(&mut self, bits: u32) -> io::Result<()>;

    /// Completely fills the given buffer with whole bytes.
    /// If the stream is already byte-aligned, it will map
    /// to a faster `read_exact` call.  Otherwise it will read
    /// bytes individually in 8-bit increments.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        for b in buf.iter_mut() {
            *b = self.read_in::<8, _>()?;
        }
        Ok(())
    }

    /// Completely fills a whole buffer with bytes and returns it.
    /// If the stream is already byte-aligned, it will map
    /// to a faster `read_exact` call.  Otherwise it will read
    /// bytes individually in 8-bit increments.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    #[inline(always)]
    #[deprecated(since = "1.8.0", note = "use read_to() method instead")]
    fn read_to_bytes<const SIZE: usize>(&mut self) -> io::Result<[u8; SIZE]> {
        self.read_to()
    }

    /// Completely fills a vector of bytes and returns it.
    /// If the stream is already byte-aligned, it will map
    /// to a faster `read_exact` call.  Otherwise it will read
    /// bytes individually in 8-bit increments.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn read_to_vec(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        read_to_vec(|buf| self.read_bytes(buf), bytes)
    }

    /// Counts the number of 1 bits in the stream until the next
    /// 0 bit and returns the amount read.
    /// Because this field is variably-sized and may be large,
    /// its output is always a `u32` type.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_unary0(&mut self) -> io::Result<u32> {
        let mut unary = 0;
        while self.read_bit()? {
            unary += 1;
        }
        Ok(unary)
    }

    /// Counts the number of 0 bits in the stream until the next
    /// 1 bit and returns the amount read.
    /// Because this field is variably-sized and may be large,
    /// its output is always a `u32` type.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_unary1(&mut self) -> io::Result<u32> {
        let mut unary = 0;
        while !(self.read_bit()?) {
            unary += 1;
        }
        Ok(unary)
    }

    /// Parses and returns complex type
    fn parse<F: FromBitStream>(&mut self) -> Result<F, F::Error>
    where
        Self: BitRead,
    {
        F::from_reader(self)
    }

    /// Parses and returns complex type with context
    fn parse_with<'a, F: FromBitStreamWith<'a>>(
        &mut self,
        context: &F::Context,
    ) -> Result<F, F::Error>
    where
        Self: BitRead,
    {
        F::from_reader(self, context)
    }

    /// Returns true if the stream is aligned at a whole byte.
    fn byte_aligned(&self) -> bool;

    /// Throws away all unread bit values until the next whole byte.
    /// Does nothing if the stream is already aligned.
    fn byte_align(&mut self);

    /// Given a compiled Huffman tree, reads bits from the stream
    /// until the next symbol is encountered.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    #[inline]
    fn read_huffman<T>(&mut self) -> io::Result<T::Symbol>
    where
        T: crate::huffman::FromBits,
    {
        T::from_bits(|| self.read_bit())
    }
}

impl<R: BitRead> BitRead2 for R {
    #[inline(always)]
    fn read_bit(&mut self) -> io::Result<bool> {
        BitRead::read_bit(self)
    }

    #[inline(always)]
    fn read<I>(&mut self, bits: u32) -> io::Result<I>
    where
        I: Integer + Sized,
    {
        self.read_var(bits)
    }

    #[inline(always)]
    fn read_in<const BITS: u32, I>(&mut self) -> io::Result<I>
    where
        I: Integer,
    {
        BitRead::read::<BITS, I>(self)
    }

    #[inline(always)]
    fn read_signed<S>(&mut self, bits: u32) -> io::Result<S>
    where
        S: SignedInteger,
    {
        self.read_signed_var(bits)
    }

    #[inline(always)]
    fn read_signed_in<const BITS: u32, S>(&mut self) -> io::Result<S>
    where
        S: SignedInteger,
    {
        BitRead::read_signed::<BITS, S>(self)
    }

    #[inline(always)]
    fn read_to<V>(&mut self) -> io::Result<V>
    where
        V: Primitive,
    {
        BitRead::read_to::<V>(self)
    }

    #[inline(always)]
    fn read_as_to<F, V>(&mut self) -> io::Result<V>
    where
        F: Endianness,
        V: Primitive,
    {
        BitRead::read_as_to::<F, V>(self)
    }

    #[inline(always)]
    fn skip(&mut self, bits: u32) -> io::Result<()> {
        BitRead::skip(self, bits)
    }

    #[inline(always)]
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        BitRead::read_bytes(self, buf)
    }

    #[inline(always)]
    fn read_unary0(&mut self) -> io::Result<u32> {
        self.read_unary::<0>()
    }

    #[inline(always)]
    fn read_unary1(&mut self) -> io::Result<u32> {
        self.read_unary::<1>()
    }

    #[inline(always)]
    fn byte_aligned(&self) -> bool {
        BitRead::byte_aligned(self)
    }

    #[inline(always)]
    fn byte_align(&mut self) {
        BitRead::byte_align(self)
    }
}

/// For reading non-aligned bits from a stream of bytes in a given endianness.
///
/// This will read exactly as many whole bytes needed to return
/// the requested number of bits.  It may cache up to a single partial byte
/// but no more.
#[derive(Clone, Debug)]
pub struct BitReader<R, E: Endianness> {
    // our underlying reader
    reader: R,
    // our partial byte
    value: u8,
    // the number of bits in our partial byte
    bits: u32,
    // a container for our endiannness
    phantom: PhantomData<E>,
}

impl<R, E: Endianness> BitReader<R, E> {
    /// Wraps a BitReader around something that implements `Read`
    pub fn new(reader: R) -> BitReader<R, E> {
        BitReader {
            reader,
            value: 0,
            bits: 0,
            phantom: PhantomData,
        }
    }

    /// Wraps a BitReader around something that implements `Read`
    /// with the given endianness.
    pub fn endian(reader: R, _endian: E) -> BitReader<R, E> {
        BitReader {
            reader,
            value: 0,
            bits: 0,
            phantom: PhantomData,
        }
    }

    /// Unwraps internal reader and disposes of BitReader.
    ///
    /// # Warning
    ///
    /// Any unread partial bits are discarded.
    #[inline]
    pub fn into_reader(self) -> R {
        self.reader
    }
}

impl<R: io::Read, E: Endianness> BitReader<R, E> {
    /// If stream is byte-aligned, provides mutable reference
    /// to internal reader.  Otherwise returns `None`
    #[inline]
    pub fn reader(&mut self) -> Option<&mut R> {
        if BitRead::byte_aligned(self) {
            Some(&mut self.reader)
        } else {
            None
        }
    }

    /// Returns byte-aligned mutable reference to internal reader.
    ///
    /// Bytes aligns stream if it is not already aligned.
    #[inline]
    pub fn aligned_reader(&mut self) -> &mut R {
        BitRead::byte_align(self);
        &mut self.reader
    }

    /// Converts `BitReader` to `ByteReader` in the same endianness.
    ///
    /// # Warning
    ///
    /// Any unread partial bits are discarded.
    #[inline]
    pub fn into_bytereader(self) -> ByteReader<R, E> {
        ByteReader::new(self.into_reader())
    }

    /// If stream is byte-aligned, provides temporary `ByteReader`
    /// in the same endianness.  Otherwise returns `None`
    ///
    /// # Warning
    ///
    /// Any reader bits left over when `ByteReader` is dropped are lost.
    #[inline]
    pub fn bytereader(&mut self) -> Option<ByteReader<&mut R, E>> {
        self.reader().map(ByteReader::new)
    }
}

impl<R: io::Read, E: Endianness> BitRead for BitReader<R, E> {
    #[inline(always)]
    fn read_bit(&mut self) -> io::Result<bool> {
        let Self {
            value,
            bits,
            reader,
            ..
        } = self;
        E::pop_bit_refill(reader, value, bits)
    }

    #[inline(always)]
    fn read_unsigned_counted<const BITS: u32, U>(&mut self, bits: BitCount<BITS>) -> io::Result<U>
    where
        U: UnsignedInteger,
    {
        let Self {
            value: queue_value,
            bits: queue_bits,
            reader,
            ..
        } = self;
        E::read_bits(reader, queue_value, queue_bits, bits)
    }

    #[inline]
    fn read_unsigned<const BITS: u32, U>(&mut self) -> io::Result<U>
    where
        U: UnsignedInteger,
    {
        let Self {
            value,
            bits,
            reader,
            ..
        } = self;
        E::read_bits_fixed::<BITS, R, U>(reader, value, bits)
    }

    #[inline(always)]
    fn read_signed_counted<const MAX: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<MAX>>,
    ) -> io::Result<S>
    where
        S: SignedInteger,
    {
        E::read_signed_counted(
            self,
            bits.try_into().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "signed reads need at least 1 bit for sign",
                )
            })?,
        )
    }

    #[inline]
    fn read_signed<const BITS: u32, S>(&mut self) -> io::Result<S>
    where
        S: SignedInteger,
    {
        let count = const {
            assert!(BITS <= S::BITS_SIZE, "excessive bits for type read");
            let count = BitCount::<BITS>::new::<BITS>().signed_count();
            assert!(count.is_some(), "signed reads need at least 1 bit for sign");
            count.unwrap()
        };

        E::read_signed_counted(self, count)
    }

    #[inline]
    fn read_to<V>(&mut self) -> io::Result<V>
    where
        V: Primitive,
    {
        let mut buffer = V::buffer();
        E::read_bytes::<8, _>(
            &mut self.reader,
            &mut self.value,
            self.bits,
            buffer.as_mut(),
        )?;
        Ok(E::bytes_to_primitive(buffer))
    }

    #[inline]
    fn read_as_to<F, V>(&mut self) -> io::Result<V>
    where
        F: Endianness,
        V: Primitive,
    {
        let mut buffer = V::buffer();
        F::read_bytes::<8, _>(
            &mut self.reader,
            &mut self.value,
            self.bits,
            buffer.as_mut(),
        )?;
        Ok(F::bytes_to_primitive(buffer))
    }

    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data = [0b10110111];
    /// let mut reader = BitReader::endian(data.as_slice(), BigEndian);
    /// assert!(reader.skip(3).is_ok());
    /// assert_eq!(reader.read::<5, u8>().unwrap(), 0b10111);
    /// ```
    ///
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{LittleEndian, BitReader, BitRead};
    /// let data = [0b10110111];
    /// let mut reader = BitReader::endian(data.as_slice(), LittleEndian);
    /// assert!(reader.skip(3).is_ok());
    /// assert_eq!(reader.read::<5, u8>().unwrap(), 0b10110);
    /// ```
    fn skip(&mut self, mut bits: u32) -> io::Result<()> {
        if BitRead::byte_aligned(self) && bits % 8 == 0 {
            skip_aligned(self.reader.by_ref(), bits / 8)
        } else {
            loop {
                match bits {
                    0 => break Ok(()),
                    bits @ 1..64 => break self.read_var(bits).map(|_: u64| ()),
                    _ => {
                        let _ = BitRead::read::<64, u64>(self)?;
                        bits -= 64;
                    }
                }
            }
        }
    }

    /// # Example
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data = b"foobar";
    /// let mut reader = BitReader::endian(data.as_slice(), BigEndian);
    /// assert!(reader.skip(24).is_ok());
    /// let mut buf = [0;3];
    /// assert!(reader.read_bytes(&mut buf).is_ok());
    /// assert_eq!(&buf, b"bar");
    /// ```
    #[inline]
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        E::read_bytes::<1024, _>(&mut self.reader, &mut self.value, self.bits, buf)
    }

    fn read_unary<const STOP_BIT: u8>(&mut self) -> io::Result<u32> {
        let Self {
            value,
            bits,
            reader,
            ..
        } = self;
        E::pop_unary::<STOP_BIT, R>(reader, value, bits)
    }

    #[inline]
    fn byte_aligned(&self) -> bool {
        self.bits == 0
    }

    #[inline]
    fn byte_align(&mut self) {
        self.value = 0;
        self.bits = 0;
    }
}

impl<R, E> BitReader<R, E>
where
    E: Endianness,
    R: io::Read + io::Seek,
{
    /// # Example
    /// ```
    /// use std::io::{Read, Cursor, SeekFrom};
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data = [0x00, 0xFF];
    /// let mut reader = BitReader::endian(Cursor::new(&data), BigEndian);
    /// assert_eq!(reader.position_in_bits().unwrap(), 0);
    ///
    /// let pos = reader.seek_bits(SeekFrom::Start(5)).unwrap();
    /// assert!(pos == 5 && 5 == reader.position_in_bits().unwrap());
    ///
    /// let pos = reader.seek_bits(SeekFrom::Current(-2)).unwrap();
    /// assert!(pos == 3 && 3 == reader.position_in_bits().unwrap());
    ///
    /// let pos = reader.seek_bits(SeekFrom::End(5)).unwrap();
    /// assert!(pos == 11 && 11 == reader.position_in_bits().unwrap());
    /// ```
    pub fn seek_bits(&mut self, from: io::SeekFrom) -> io::Result<u64> {
        match from {
            io::SeekFrom::Start(from_start_pos) => {
                let (bytes, bits) = (from_start_pos / 8, (from_start_pos % 8) as u32);
                BitRead::byte_align(self);
                self.reader.seek(io::SeekFrom::Start(bytes))?;
                BitRead::skip(self, bits)?;
                Ok(from_start_pos)
            }
            io::SeekFrom::End(from_end_pos) => {
                let reader_end = self.reader.seek(io::SeekFrom::End(0))?;
                let new_pos = (reader_end * 8) as i64 - from_end_pos;
                assert!(new_pos >= 0, "The final position should be greater than 0");
                self.seek_bits(io::SeekFrom::Start(new_pos as u64))
            }
            io::SeekFrom::Current(offset) => {
                let new_pos = self.position_in_bits()? as i64 + offset;
                assert!(new_pos >= 0, "The final position should be greater than 0");
                self.seek_bits(io::SeekFrom::Start(new_pos as u64))
            }
        }
    }

    /// # Example
    /// ```
    /// use std::fs::read;
    /// use std::io::{Read, Cursor, SeekFrom};
    /// use bitstream_io::{BigEndian, BitReader, BitRead};
    /// let data = [0x00, 0xFF];
    /// let mut reader = BitReader::endian(Cursor::new(&data), BigEndian);
    /// assert_eq!(reader.position_in_bits().unwrap(), 0);
    ///
    /// let _: i32 = reader.read_signed::<5, _>().unwrap();
    /// assert_eq!(reader.position_in_bits().unwrap(), 5);
    ///
    /// reader.read_bit().unwrap();
    /// assert_eq!(reader.position_in_bits().unwrap(), 6);
    /// ```
    #[inline]
    #[allow(clippy::seek_from_current)]
    pub fn position_in_bits(&mut self) -> io::Result<u64> {
        // core2 doesn't have `seek_from_current`
        let bytes = self.reader.seek(io::SeekFrom::Current(0))?;
        Ok(bytes * 8 - (self.bits as u64))
    }
}

fn skip_aligned<R>(reader: R, bytes: u32) -> io::Result<()>
where
    R: io::Read,
{
    fn skip_chunks<const SIZE: usize, R>(mut reader: R, mut bytes: usize) -> io::Result<()>
    where
        R: io::Read,
    {
        let mut buf = [0; SIZE];
        while bytes > 0 {
            let to_read = bytes.min(SIZE);
            reader.read_exact(&mut buf[0..to_read])?;
            bytes -= to_read;
        }
        Ok(())
    }

    match bytes {
        0..256 => skip_chunks::<8, R>(reader, bytes as usize),
        256..1024 => skip_chunks::<256, R>(reader, bytes as usize),
        1024..4096 => skip_chunks::<1024, R>(reader, bytes as usize),
        _ => skip_chunks::<4096, R>(reader, bytes as usize),
    }
}

/// A trait for anything that can read aligned values from an input stream
pub trait ByteRead {
    /// Reads whole numeric value from stream
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{BigEndian, ByteReader, ByteRead};
    /// let data = [0b00000000, 0b11111111];
    /// let mut reader = ByteReader::endian(data.as_slice(), BigEndian);
    /// assert_eq!(reader.read::<u16>().unwrap(), 0b0000000011111111);
    /// ```
    ///
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{LittleEndian, ByteReader, ByteRead};
    /// let data = [0b00000000, 0b11111111];
    /// let mut reader = ByteReader::endian(data.as_slice(), LittleEndian);
    /// assert_eq!(reader.read::<u16>().unwrap(), 0b1111111100000000);
    /// ```
    fn read<V>(&mut self) -> Result<V, io::Error>
    where
        V: Primitive;

    /// Reads whole numeric value from stream in a potentially different endianness
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{BigEndian, ByteReader, ByteRead, LittleEndian};
    /// let data = [0b00000000, 0b11111111];
    /// let mut reader = ByteReader::endian(data.as_slice(), BigEndian);
    /// assert_eq!(reader.read_as::<LittleEndian, u16>().unwrap(), 0b1111111100000000);
    /// ```
    ///
    /// ```
    /// use std::io::Read;
    /// use bitstream_io::{BigEndian, ByteReader, ByteRead, LittleEndian};
    /// let data = [0b00000000, 0b11111111];
    /// let mut reader = ByteReader::endian(data.as_slice(), LittleEndian);
    /// assert_eq!(reader.read_as::<BigEndian, u16>().unwrap(), 0b0000000011111111);
    /// ```
    fn read_as<F, V>(&mut self) -> Result<V, io::Error>
    where
        F: Endianness,
        V: Primitive;

    /// Completely fills the given buffer with whole bytes.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        for b in buf.iter_mut() {
            *b = self.read()?;
        }
        Ok(())
    }

    /// Completely fills a whole buffer with bytes and returns it.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    #[inline(always)]
    #[deprecated(since = "1.8.0", note = "use read() method instead")]
    fn read_to_bytes<const SIZE: usize>(&mut self) -> io::Result<[u8; SIZE]> {
        self.read()
    }

    /// Completely fills a vector of bytes and returns it.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn read_to_vec(&mut self, bytes: usize) -> io::Result<Vec<u8>> {
        read_to_vec(|buf| self.read_bytes(buf), bytes)
    }

    /// Skips the given number of bytes in the stream.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn skip(&mut self, bytes: u32) -> io::Result<()>;

    /// Parses and returns complex type
    fn parse<F: FromByteStream>(&mut self) -> Result<F, F::Error> {
        F::from_reader(self)
    }

    /// Parses and returns complex type with context
    fn parse_with<'a, F: FromByteStreamWith<'a>>(
        &mut self,
        context: &F::Context,
    ) -> Result<F, F::Error> {
        F::from_reader(self, context)
    }

    /// Parses and returns complex type with owned context
    fn parse_using<F: FromByteStreamUsing>(&mut self, context: F::Context) -> Result<F, F::Error> {
        F::from_reader(self, context)
    }

    /// Returns mutable reference to underlying reader
    fn reader_ref(&mut self) -> &mut dyn io::Read;
}

/// For reading aligned bytes from a stream of bytes in a given endianness.
///
/// This only reads aligned values and maintains no internal state.
#[derive(Debug)]
pub struct ByteReader<R: io::Read, E: Endianness> {
    phantom: PhantomData<E>,
    reader: R,
}

impl<R: io::Read, E: Endianness> ByteReader<R, E> {
    /// Wraps a ByteReader around something that implements `Read`
    pub fn new(reader: R) -> ByteReader<R, E> {
        ByteReader {
            phantom: PhantomData,
            reader,
        }
    }

    /// Wraps a ByteReader around something that implements `Read`
    /// with the given endianness.
    pub fn endian(reader: R, _endian: E) -> ByteReader<R, E> {
        ByteReader {
            phantom: PhantomData,
            reader,
        }
    }

    /// Unwraps internal reader and disposes of `ByteReader`.
    #[inline]
    pub fn into_reader(self) -> R {
        self.reader
    }

    /// Provides mutable reference to internal reader
    #[inline]
    pub fn reader(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Converts `ByteReader` to `BitReader` in the same endianness.
    #[inline]
    pub fn into_bitreader(self) -> BitReader<R, E> {
        BitReader::new(self.into_reader())
    }

    /// Provides temporary `BitReader` in the same endianness.
    ///
    /// # Warning
    ///
    /// Any unread bits left over when `BitReader` is dropped are lost.
    #[inline]
    pub fn bitreader(&mut self) -> BitReader<&mut R, E> {
        BitReader::new(self.reader())
    }
}

impl<R: io::Read, E: Endianness> ByteRead for ByteReader<R, E> {
    #[inline]
    fn read<V>(&mut self) -> Result<V, io::Error>
    where
        V: Primitive,
    {
        let mut buf = V::buffer();
        self.read_bytes(buf.as_mut())?;
        Ok(E::bytes_to_primitive(buf))
    }

    #[inline]
    fn read_as<F, V>(&mut self) -> Result<V, io::Error>
    where
        F: Endianness,
        V: Primitive,
    {
        let mut buf = V::buffer();
        self.read_bytes(buf.as_mut())?;
        Ok(F::bytes_to_primitive(buf))
    }

    #[inline]
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.reader.read_exact(buf)
    }

    #[inline]
    fn skip(&mut self, bytes: u32) -> io::Result<()> {
        skip_aligned(&mut self.reader, bytes)
    }

    #[inline]
    fn reader_ref(&mut self) -> &mut dyn io::Read {
        &mut self.reader
    }
}

impl<R: io::Read + io::Seek, E: Endianness> io::Seek for ByteReader<R, E> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.reader().seek(pos)
    }
}

/// Implemented by complex types that don't require any additional context
/// to parse themselves from a reader.  Analogous to [`std::str::FromStr`].
///
/// # Example
/// ```
/// use std::io::Read;
/// use bitstream_io::{BigEndian, BitRead, BitReader, FromBitStream};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct BlockHeader {
///     last_block: bool,
///     block_type: u8,
///     block_size: u32,
/// }
///
/// impl FromBitStream for BlockHeader {
///     type Error = std::io::Error;
///
///     fn from_reader<R: BitRead + ?Sized>(r: &mut R) -> std::io::Result<Self> {
///         Ok(Self {
///             last_block: r.read_bit()?,
///             block_type: r.read::<7, _>()?,
///             block_size: r.read::<24, _>()?,
///         })
///     }
/// }
///
/// let mut reader = BitReader::endian(b"\x04\x00\x00\x7A".as_slice(), BigEndian);
/// assert_eq!(
///     reader.parse::<BlockHeader>().unwrap(),
///     BlockHeader { last_block: false, block_type: 4, block_size: 122 }
/// );
/// ```
pub trait FromBitStream {
    /// Error generated during parsing, such as `io::Error`
    type Error;

    /// Parse Self from reader
    fn from_reader<R: BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Implemented by complex types that require some immutable context
/// to parse themselves from a reader.
///
/// # Example
/// ```
/// use std::io::Read;
/// use bitstream_io::{BigEndian, BitRead, BitReader, FromBitStreamWith};
///
/// #[derive(Default)]
/// struct Streaminfo {
///     minimum_block_size: u16,
///     maximum_block_size: u16,
///     minimum_frame_size: u32,
///     maximum_frame_size: u32,
///     sample_rate: u32,
///     channels: u8,
///     bits_per_sample: u8,
///     total_samples: u64,
///     md5: [u8; 16],
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct FrameHeader {
///     variable_block_size: bool,
///     block_size: u32,
///     sample_rate: u32,
///     channel_assignment: u8,
///     sample_size: u8,
///     frame_number: u64,
///     crc8: u8,
/// }
///
/// impl FromBitStreamWith<'_> for FrameHeader {
///     type Context = Streaminfo;
///
///     type Error = FrameHeaderError;
///
///     fn from_reader<R: BitRead + ?Sized>(
///         r: &mut R,
///         streaminfo: &Streaminfo,
///     ) -> Result<Self, Self::Error> {
///         if r.read::<14, u16>()? != 0b11111111111110 {
///             return Err(FrameHeaderError::InvalidSync);
///         }
///
///         if r.read_bit()? != false {
///             return Err(FrameHeaderError::InvalidReservedBit);
///         }
///
///         let variable_block_size = r.read_bit()?;
///
///         let block_size_bits = r.read::<4, u8>()?;
///
///         let sample_rate_bits = r.read::<4, u8>()?;
///
///         let channel_assignment = r.read::<4, u8>()?;
///
///         let sample_size = match r.read::<3, u8>()? {
///             0b000 => streaminfo.bits_per_sample,
///             0b001 => 8,
///             0b010 => 12,
///             0b011 => return Err(FrameHeaderError::InvalidSampleSize),
///             0b100 => 16,
///             0b101 => 20,
///             0b110 => 24,
///             0b111 => 32,
///             _ => unreachable!(),
///         };
///
///         if r.read_bit()? != false {
///             return Err(FrameHeaderError::InvalidReservedBit);
///         }
///
///         let frame_number = read_utf8(r)?;
///
///         Ok(FrameHeader {
///             variable_block_size,
///             block_size: match block_size_bits {
///                 0b0000 => return Err(FrameHeaderError::InvalidBlockSize),
///                 0b0001 => 192,
///                 n @ 0b010..=0b0101 => 576 * (1 << (n - 2)),
///                 0b0110 => r.read::<8, u32>()? + 1,
///                 0b0111 => r.read::<16, u32>()? + 1,
///                 n @ 0b1000..=0b1111 => 256 * (1 << (n - 8)),
///                 _ => unreachable!(),
///             },
///             sample_rate: match sample_rate_bits {
///                 0b0000 => streaminfo.sample_rate,
///                 0b0001 => 88200,
///                 0b0010 => 176400,
///                 0b0011 => 192000,
///                 0b0100 => 8000,
///                 0b0101 => 16000,
///                 0b0110 => 22050,
///                 0b0111 => 24000,
///                 0b1000 => 32000,
///                 0b1001 => 44100,
///                 0b1010 => 48000,
///                 0b1011 => 96000,
///                 0b1100 => r.read::<8, u32>()? * 1000,
///                 0b1101 => r.read::<16, u32>()?,
///                 0b1110 => r.read::<16, u32>()? * 10,
///                 0b1111 => return Err(FrameHeaderError::InvalidSampleRate),
///                 _ => unreachable!(),
///             },
///             channel_assignment,
///             sample_size,
///             frame_number,
///             crc8: r.read::<8, _>()?
///         })
///     }
/// }
///
/// #[derive(Debug)]
/// enum FrameHeaderError {
///     Io(std::io::Error),
///     InvalidSync,
///     InvalidReservedBit,
///     InvalidSampleSize,
///     InvalidBlockSize,
///     InvalidSampleRate,
/// }
///
/// impl From<std::io::Error> for FrameHeaderError {
///     fn from(err: std::io::Error) -> Self {
///         Self::Io(err)
///     }
/// }
///
/// fn read_utf8<R: BitRead + ?Sized>(r: &mut R) -> Result<u64, std::io::Error> {
///     r.read::<8, _>()  // left unimplimented in this example
/// }
///
/// let mut reader = BitReader::endian(b"\xFF\xF8\xC9\x18\x00\xC2".as_slice(), BigEndian);
/// assert_eq!(
///     reader.parse_with::<FrameHeader>(&Streaminfo::default()).unwrap(),
///     FrameHeader {
///         variable_block_size: false,
///         block_size: 4096,
///         sample_rate: 44100,
///         channel_assignment: 1,
///         sample_size: 16,
///         frame_number: 0,
///         crc8: 0xC2,
///     }
/// );
/// ```
///
/// # Example with lifetime-contrained `Context`
///
/// In some cases, the `Context` can depend on a reference to another `struct`.
///
/// ```
/// use std::io::Read;
/// use bitstream_io::{BigEndian, BitRead, BitReader, FromBitStreamWith};
///
/// #[derive(Default)]
/// struct ModeParameters {
///     size_len: u8,
///     index_len: u8,
///     index_delta_len: u8,
///     // ...
/// }
///
/// struct AuHeaderParseContext<'a> {
///     params: &'a ModeParameters,
///     base_index: Option<u32>,
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct AuHeader {
///     size: u32,
///     index: u32,
///     // ...
/// }
///
/// impl<'a> FromBitStreamWith<'a> for AuHeader {
///     type Context = AuHeaderParseContext<'a>;
///
///     type Error = AuHeaderError;
///
///     fn from_reader<R: BitRead + ?Sized>(
///         r: &mut R,
///         ctx: &AuHeaderParseContext<'a>,
///     ) -> Result<Self, Self::Error> {
///         let size = r.read_var::<u32>(ctx.params.size_len as u32)?;
///         let index = match ctx.base_index {
///             None => r.read_var::<u32>(ctx.params.index_len as u32)?,
///             Some(base_index) => {
///                 base_index
///                 + 1
///                 + r.read_var::<u32>(ctx.params.index_delta_len as u32)?
///             }
///         };
///
///         Ok(AuHeader {
///             size,
///             index,
///             // ...
///         })
///     }
/// }
///
/// #[derive(Debug)]
/// enum AuHeaderError {
///     Io(std::io::Error),
/// }
///
/// impl From<std::io::Error> for AuHeaderError {
///     fn from(err: std::io::Error) -> Self {
///         Self::Io(err)
///     }
/// }
///
/// let mut reader = BitReader::endian(b"\xFF\xEA\xFF\x10".as_slice(), BigEndian);
///
/// let mode_params = ModeParameters {
///     size_len: 10,
///     index_len: 6,
///     index_delta_len: 2,
///     // ...
/// };
///
/// let mut ctx = AuHeaderParseContext {
///     params: &mode_params,
///     base_index: None,
/// };
///
/// let header1 = reader.parse_with::<AuHeader>(&ctx).unwrap();
/// assert_eq!(
///     header1,
///     AuHeader {
///         size: 1023,
///         index: 42,
///     }
/// );
///
/// ctx.base_index = Some(header1.index);
///
/// assert_eq!(
///     reader.parse_with::<AuHeader>(&ctx).unwrap(),
///     AuHeader {
///         size: 1020,
///         index: 44,
///     }
/// );
/// ```
pub trait FromBitStreamWith<'a> {
    /// Some context to use when parsing
    type Context: 'a;

    /// Error generated during parsing, such as `io::Error`
    type Error;

    /// Parse Self from reader with the given context
    fn from_reader<R: BitRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Implemented by complex types that consume some immutable context
/// to parse themselves from a reader.
///
/// Like [`FromBitStreamWith`], but consumes its context
/// rather than taking a shared reference to it.
pub trait FromBitStreamUsing {
    /// Some context to consume when parsing
    type Context;

    /// Error generated during parsing, such as `io::Error`
    type Error;

    /// Parse Self from reader with the given context
    fn from_reader<R: BitRead + ?Sized>(
        r: &mut R,
        context: Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Implemented by complex types that don't require any additional context
/// to parse themselves from a reader.  Analagous to `FromStr`.
pub trait FromByteStream {
    /// Error generated during parsing, such as `io::Error`
    type Error;

    /// Parse Self from reader
    fn from_reader<R: ByteRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Implemented by complex types that require some additional context
/// to parse themselves from a reader.  Analagous to `FromStr`.
pub trait FromByteStreamWith<'a> {
    /// Some context to use when parsing
    type Context: 'a;

    /// Error generated during parsing, such as `io::Error`
    type Error;

    /// Parse Self from reader
    fn from_reader<R: ByteRead + ?Sized>(
        r: &mut R,
        context: &Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Implemented by complex types that consume some additional context
/// to parse themselves from a reader.
///
/// Like [`FromByteStreamWith`], but consumes the context.
pub trait FromByteStreamUsing {
    /// Some context to use when parsing
    type Context;

    /// Error generated during parsing, such as `io::Error`
    type Error;

    /// Parse Self from reader
    fn from_reader<R: ByteRead + ?Sized>(
        r: &mut R,
        context: Self::Context,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[cfg(feature = "alloc")]
fn read_to_vec(
    mut read: impl FnMut(&mut [u8]) -> io::Result<()>,
    bytes: usize,
) -> io::Result<Vec<u8>> {
    const MAX_CHUNK: usize = 4096;

    match bytes {
        0 => Ok(Vec::new()),
        bytes if bytes <= MAX_CHUNK => {
            let mut buf = vec![0; bytes];
            read(&mut buf)?;
            Ok(buf)
        }
        mut bytes => {
            let mut whole = Vec::with_capacity(MAX_CHUNK);
            let mut chunk: [u8; MAX_CHUNK] = [0; MAX_CHUNK];
            while bytes > 0 {
                let chunk_size = bytes.min(MAX_CHUNK);
                let chunk = &mut chunk[0..chunk_size];
                read(chunk)?;
                whole.extend_from_slice(chunk);
                bytes -= chunk_size;
            }
            Ok(whole)
        }
    }
}
