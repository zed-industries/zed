// Copyright 2017 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits and implementations for writing bits to a stream.

#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
use core2::io;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::{
    convert::{From, TryFrom, TryInto},
    fmt,
};
#[cfg(feature = "std")]
use std::io;

use super::{
    BitCount, Checkable, CheckedSigned, CheckedUnsigned, Endianness, Integer, Numeric, PhantomData,
    Primitive, SignedBitCount, SignedInteger, UnsignedInteger, VBRInteger,
};

#[cfg(feature = "alloc")]
pub use bit_recorder::BitRecorder;

/// For writing bit values to an underlying stream in a given endianness.
///
/// Because this only writes whole bytes to the underlying stream,
/// it is important that output is byte-aligned before the bitstream
/// writer's lifetime ends.
/// **Partial bytes will be lost** if the writer is disposed of
/// before they can be written.
pub struct BitWriter<W: io::Write, E: Endianness> {
    // our underlying writer
    writer: W,
    // our partial byte
    value: u8,
    // the number of bits in our partial byte
    bits: u32,
    // a container for our endianness
    phantom: PhantomData<E>,
}

impl<W: io::Write, E: Endianness> BitWriter<W, E> {
    /// Wraps a BitWriter around something that implements `Write`
    pub fn new(writer: W) -> BitWriter<W, E> {
        BitWriter {
            writer,
            value: 0,
            bits: 0,
            phantom: PhantomData,
        }
    }

    /// Wraps a BitWriter around something that implements `Write`
    /// with the given endianness.
    pub fn endian(writer: W, _endian: E) -> BitWriter<W, E> {
        BitWriter {
            writer,
            value: 0,
            bits: 0,
            phantom: PhantomData,
        }
    }

    /// Unwraps internal writer and disposes of BitWriter.
    ///
    /// # Warning
    ///
    /// Any unwritten partial bits are discarded.
    #[inline]
    pub fn into_writer(self) -> W {
        self.writer
    }

    /// If stream is byte-aligned, provides mutable reference
    /// to internal writer.  Otherwise returns `None`
    #[inline]
    pub fn writer(&mut self) -> Option<&mut W> {
        if BitWrite::byte_aligned(self) {
            Some(&mut self.writer)
        } else {
            None
        }
    }

    /// Returns byte-aligned mutable reference to internal writer.
    ///
    /// Bytes aligns stream if it is not already aligned.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    #[inline]
    pub fn aligned_writer(&mut self) -> io::Result<&mut W> {
        BitWrite::byte_align(self)?;
        Ok(&mut self.writer)
    }

    /// Converts `BitWriter` to `ByteWriter` in the same endianness.
    ///
    /// # Warning
    ///
    /// Any written partial bits are discarded.
    #[inline]
    pub fn into_bytewriter(self) -> ByteWriter<W, E> {
        ByteWriter::new(self.into_writer())
    }

    /// If stream is byte-aligned, provides temporary `ByteWriter`
    /// in the same endianness.  Otherwise returns `None`
    ///
    /// # Warning
    ///
    /// Any unwritten bits left over when `ByteWriter` is dropped are lost.
    #[inline]
    pub fn bytewriter(&mut self) -> Option<ByteWriter<&mut W, E>> {
        self.writer().map(ByteWriter::new)
    }

    /// Flushes output stream to disk, if necessary.
    /// Any partial bytes are not flushed.
    ///
    /// # Errors
    ///
    /// Passes along any errors from the underlying stream.
    #[inline(always)]
    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// A trait for anything that can write a variable number of
/// potentially un-aligned values to an output stream
pub trait BitWrite {
    /// Writes a single bit to the stream.
    /// `true` indicates 1, `false` indicates 0
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w  = BitWriter::endian(vec![], BigEndian);
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.write_bit(false).is_ok());
    /// assert!(w.write_bit(false).is_ok());
    /// assert!(w.write_bit(false).is_ok());
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.write_bit(false).is_ok());
    /// assert_eq!(w.into_writer(), &[0b1000_1110]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, LittleEndian};
    ///
    /// let mut w  = BitWriter::endian(vec![], LittleEndian);
    /// assert!(w.write_bit(false).is_ok());
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.write_bit(false).is_ok());
    /// assert!(w.write_bit(false).is_ok());
    /// assert!(w.write_bit(false).is_ok());
    /// assert!(w.write_bit(true).is_ok());
    /// assert_eq!(w.into_writer(), &[0b1000_1110]);
    /// ```
    #[inline]
    fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.write_unsigned::<1, u8>(u8::from(bit))
    }

    /// Writes a signed or unsigned value to the stream using the given
    /// const number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // writing unsigned value is ok
    /// assert!(w.write::<4, u8>(1).is_ok());
    /// // writing signed value is ok
    /// assert!(w.write::<4, i8>(-1).is_ok());
    /// // writing an array of bits is ok too
    /// assert!(w.write::<1, [bool; 4]>([true, false, true, true]).is_ok());
    /// // writing an array of any Integer type is ok
    /// assert!(w.write::<2, [u8; 2]>([0b11, 0b00]).is_ok());
    /// // trying to write a value larger than 4 bits in 4 bits is an error
    /// assert!(w.write::<4, u8>(u8::MAX).is_err());
    ///
    /// assert_eq!(w.into_writer(), &[0b0001_1111, 0b1011_11_00]);
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // trying to write 9 bits from a u8 is a compile-time error
    /// w.write::<9, u8>(1);
    /// ```
    #[inline]
    fn write<const BITS: u32, I>(&mut self, value: I) -> io::Result<()>
    where
        I: Integer,
    {
        Integer::write::<BITS, Self>(value, self)
    }

    /// Writes a signed or unsigned value to the stream using the given
    /// number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the input type is too small
    /// to hold the given number of bits.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // writing unsigned value is ok
    /// assert!(w.write_var::<u8>(4, 1).is_ok());
    /// // writing signed value is also ok
    /// assert!(w.write_var::<i8>(4, -1).is_ok());
    /// assert_eq!(w.into_writer(), &[0b0001_1111]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // writing a value larger than 4 bits in 4 bits is a runtime error
    /// assert!(w.write_var::<u8>(4, u8::MAX).is_err());
    /// // writing 9 bits from a u8 is also a runtime error
    /// assert!(w.write_var::<u8>(9, 0).is_err());
    /// ```
    #[inline]
    fn write_var<I>(&mut self, bits: u32, value: I) -> io::Result<()>
    where
        I: Integer,
    {
        self.write_counted(BitCount::unknown(bits), value)
    }

    /// Writes an unsigned value to the stream using the given
    /// const number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], BigEndian);
    /// writer.write_unsigned::<1, u8>(0b1).unwrap();
    /// writer.write_unsigned::<2, u8>(0b01).unwrap();
    /// writer.write_unsigned::<5, u8>(0b10111).unwrap();
    /// assert_eq!(writer.into_writer(), [0b1_01_10111]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], LittleEndian);
    /// writer.write_unsigned::<1, u8>(0b1).unwrap();
    /// writer.write_unsigned::<2, u8>(0b11).unwrap();
    /// writer.write_unsigned::<5, u8>(0b10110).unwrap();
    /// assert_eq!(writer.into_writer(), [0b10110_11_1]);
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], BigEndian);
    /// // trying to write 9 bits from a u8 is a compile-time error
    /// writer.write_unsigned::<9, u8>(1);
    /// ```
    ///
    /// ```
    /// use std::io::{Write, sink};
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut w = BitWriter::endian(sink(), BigEndian);
    /// assert!(w.write_unsigned::<1, u8>(2).is_err());    // can't write   2 in 1 bit
    /// assert!(w.write_unsigned::<2, u8>(4).is_err());    // can't write   4 in 2 bits
    /// assert!(w.write_unsigned::<3, u8>(8).is_err());    // can't write   8 in 3 bits
    /// assert!(w.write_unsigned::<4, u8>(16).is_err());   // can't write  16 in 4 bits
    /// ```
    #[inline]
    fn write_unsigned<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        self.write_unsigned_var(BITS, value)
    }

    /// Writes an unsigned value to the stream using the given
    /// number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the input type is too small
    /// to hold the given number of bits.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], BigEndian);
    /// writer.write_unsigned_var::<u8>(1, 0b1).unwrap();
    /// writer.write_unsigned_var::<u8>(2, 0b01).unwrap();
    /// writer.write_unsigned_var::<u8>(5, 0b10111).unwrap();
    /// assert_eq!(writer.into_writer(), [0b1_01_10111]);
    /// ```
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], LittleEndian);
    /// writer.write_unsigned_var::<u8>(1, 0b1).unwrap();
    /// writer.write_unsigned_var::<u8>(2, 0b11).unwrap();
    /// writer.write_unsigned_var::<u8>(5, 0b10110).unwrap();
    /// assert_eq!(writer.into_writer(), [0b10110_11_1]);
    /// ```
    ///
    /// ```
    /// use std::io::{Write, sink};
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut w = BitWriter::endian(sink(), BigEndian);
    /// assert!(w.write_unsigned_var::<u8>(9, 0).is_err());    // can't write  u8 in 9 bits
    /// assert!(w.write_unsigned_var::<u16>(17, 0).is_err());  // can't write u16 in 17 bits
    /// assert!(w.write_unsigned_var::<u32>(33, 0).is_err());  // can't write u32 in 33 bits
    /// assert!(w.write_unsigned_var::<u64>(65, 0).is_err());  // can't write u64 in 65 bits
    /// assert!(w.write_unsigned_var::<u8>(1, 2).is_err());    // can't write   2 in 1 bit
    /// assert!(w.write_unsigned_var::<u8>(2, 4).is_err());    // can't write   4 in 2 bits
    /// assert!(w.write_unsigned_var::<u8>(3, 8).is_err());    // can't write   8 in 3 bits
    /// assert!(w.write_unsigned_var::<u8>(4, 16).is_err());   // can't write  16 in 4 bits
    /// ```
    fn write_unsigned_var<U>(&mut self, bits: u32, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        self.write_unsigned_counted(BitCount::unknown(bits), value)
    }

    /// Writes a twos-complement signed value to the stream
    /// with the given const number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    /// A compile-time error occurs if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], BigEndian);
    /// writer.write_signed::<4, i8>(-5).unwrap();
    /// writer.write_signed::<4, i8>(7).unwrap();
    /// assert_eq!(writer.into_writer(), [0b10110111]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], LittleEndian);
    /// writer.write_signed::<4, i8>(7).unwrap();
    /// writer.write_signed::<4, i8>(-5).unwrap();
    /// assert_eq!(writer.into_writer(), [0b10110111]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], LittleEndian);
    /// // writing a value too large for 4 bits in 4 bits is a runtime error
    /// assert!(writer.write_signed::<4, i8>(i8::MAX).is_err());
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], LittleEndian);
    /// // writing 9 bits from an i8 is a compile-time error
    /// assert!(writer.write_signed::<9, i8>(1).is_err());
    /// ```
    fn write_signed<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        self.write_signed_var(BITS, value)
    }

    /// Writes a twos-complement signed value to the stream
    /// with the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the input type is too small
    /// to hold the given number of bits.
    /// Returns an error if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], BigEndian);
    /// writer.write_signed_var(4, -5).unwrap();
    /// writer.write_signed_var(4, 7).unwrap();
    /// assert_eq!(writer.into_writer(), [0b10110111]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    ///
    /// let mut writer = BitWriter::endian(vec![], LittleEndian);
    /// writer.write_signed_var(4, 7).unwrap();
    /// writer.write_signed_var(4, -5).unwrap();
    /// assert_eq!(writer.into_writer(), [0b10110111]);
    /// ```
    #[inline(always)]
    fn write_signed_var<S>(&mut self, bits: u32, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        self.write_signed_counted(BitCount::unknown(bits), value)
    }

    /// Writes the given bit count to the stream
    /// with the necessary maximum number of bits.
    ///
    /// For example, if the maximum bit count is 15 - or `0b1111` -
    /// writes the bit count to the stream as a 4-bit unsigned value
    /// which can be used in subsequent writes.
    ///
    /// Note that `MAX` must be greater than 0.
    /// Unlike the bit reader, the bit count need not be an exact
    /// power of two when writing.  Any bits higher than the
    /// bit count can reach are simply left 0.
    ///
    /// # Errors
    ///
    /// Passes along an I/O error from the underlying stream.
    ///
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// let count = 4;
    /// w.write::<3, u32>(count).unwrap();
    /// // may need to verify count is not larger than u8 at runtime
    /// w.write_var::<u8>(count, 0b1111).unwrap();
    /// w.byte_align().unwrap();
    /// assert_eq!(w.into_writer(), &[0b100_11110]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite, BitCount};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // a bit count of 4, with a maximum of 7 (0b111)
    /// let count: BitCount<0b111> = BitCount::new::<4>();
    /// w.write_count(count).unwrap();
    /// // maximum size of count is known to be 7 bits at compile-time
    /// // so no need to check that 7 bits is larger than a u8 at runtime
    /// w.write_counted::<0b111, u8>(count, 0b1111).unwrap();
    /// w.byte_align().unwrap();
    /// assert_eq!(w.into_writer(), &[0b100_11110]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite, BitCount};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // a bit count of 4, with a maximum of 6 (0b110)
    /// let count: BitCount<0b110> = BitCount::new::<4>();
    /// w.write_count(count).unwrap();
    /// w.write_counted::<0b110, u8>(count, 0b1111).unwrap();
    /// w.byte_align().unwrap();
    /// // bit count is written in 3 bits
    /// // while actual value is written in 4 bits
    /// assert_eq!(w.into_writer(), &[0b100_11110]);
    /// ```
    fn write_count<const MAX: u32>(&mut self, BitCount { bits }: BitCount<MAX>) -> io::Result<()> {
        const {
            assert!(MAX > 0, "MAX value must be > 0");
        }

        self.write_unsigned_var(
            if MAX == u32::MAX {
                32
            } else if (MAX + 1).is_power_of_two() {
                (MAX + 1).ilog2()
            } else {
                (MAX + 1).ilog2() + 1
            },
            bits,
        )
    }

    /// Writes a signed or unsigned value to the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian, BitCount};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // writing 4 bits with a maximum of 8 will fit into a u8
    /// // so we only need check the value fits into 4 bits
    /// assert!(w.write_counted::<4, u8>(BitCount::new::<4>(), 0b1111).is_ok());
    /// assert!(w.write_counted::<4, u8>(BitCount::new::<4>(), 0b1111 + 1).is_err());
    /// // writing 4 bits with a maximum of 64 might not fit into a u8
    /// // so need to verify this at runtime
    /// assert!(w.write_counted::<64, u8>(BitCount::new::<4>(), 0b0000).is_ok());
    /// assert_eq!(w.into_writer(), &[0b1111_0000]);
    /// ```
    fn write_counted<const MAX: u32, I>(&mut self, bits: BitCount<MAX>, value: I) -> io::Result<()>
    where
        I: Integer + Sized,
    {
        I::write_var::<MAX, _>(value, self, bits)
    }

    /// Writes a signed value to the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian, BitCount};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // writing 4 bits with a maximum of 8 will fit into a u8
    /// // so we only need check the value fits into 4 bits
    /// assert!(w.write_unsigned_counted::<4, u8>(BitCount::new::<4>(), 0b1111).is_ok());
    /// assert!(w.write_unsigned_counted::<4, u8>(BitCount::new::<4>(), 0b1111 + 1).is_err());
    /// // writing 4 bits with a maximum of 64 might not fit into a u8
    /// // so need to verify this at runtime
    /// assert!(w.write_unsigned_counted::<64, u8>(BitCount::new::<4>(), 0b0000).is_ok());
    /// assert_eq!(w.into_writer(), &[0b1111_0000]);
    /// ```
    fn write_unsigned_counted<const BITS: u32, U>(
        &mut self,
        bits: BitCount<BITS>,
        value: U,
    ) -> io::Result<()>
    where
        U: UnsignedInteger;

    /// Writes an unsigned value to the stream with
    /// the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian, BitCount};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // writing 4 bits with a maximum of 8 will fit into an i8
    /// // so we only need check the value fits into 4 bits
    /// assert!(w.write_signed_counted::<4, i8>(BitCount::new::<4>(), 0b0111).is_ok());
    /// assert!(w.write_signed_counted::<4, i8>(BitCount::new::<4>(), 0b0111 + 1).is_err());
    /// // writing 4 bits with a maximum of 64 might not fit into a i8
    /// // so need to verify this at runtime
    /// assert!(w.write_signed_counted::<64, i8>(BitCount::new::<4>(), 0b0000).is_ok());
    /// assert_eq!(w.into_writer(), &[0b0111_0000]);
    /// ```
    fn write_signed_counted<const MAX: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<MAX>>,
        value: S,
    ) -> io::Result<()>
    where
        S: SignedInteger;

    /// Writes the given constant value to the stream with
    /// the given number of bits.
    ///
    /// Due to current limitations of constant parameters,
    /// this is limited to `u32` values.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// A compile-time error occurs if the number of bits is larger
    /// than 32 or if the value is too large too fit the
    /// requested number of bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// assert!(w.write_const::<4, 0b1000>().is_ok());
    /// assert!(w.write_const::<4, 0b1011>().is_ok());
    /// assert_eq!(w.into_writer(), &[0b1000_1011]);
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // trying to write a 5 bit value in 4 bits is a compile-time error
    /// w.write_const::<4, 0b11111>();
    /// ```
    ///
    /// ```rust,compile_fail
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// // trying to write a 33 bit value is also a compile-time error
    /// w.write_const::<33, 1>();
    /// ```
    #[inline]
    fn write_const<const BITS: u32, const VALUE: u32>(&mut self) -> io::Result<()> {
        const {
            assert!(
                BITS == 0 || VALUE <= (u32::ALL >> (u32::BITS_SIZE - BITS)),
                "excessive value for bits written"
            );
        }

        self.write::<BITS, u32>(VALUE)
    }

    /// Writes whole value to the stream whose size in bits
    /// is equal to its type's size.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// assert!(w.write_from::<u32>(0x12_34_56_78).is_ok());
    /// assert_eq!(w.into_writer(), &[0x12, 0x34, 0x56, 0x78]);
    /// ```
    ///
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// assert!(w.write_from::<[u8; 4]>([0x12, 0x34, 0x56, 0x78]).is_ok());
    /// assert_eq!(w.into_writer(), &[0x12, 0x34, 0x56, 0x78]);
    /// ```
    fn write_from<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive;

    /// Writes whole value to the stream whose size in bits
    /// is equal to its type's size in an endianness that may
    /// be different from the stream's endianness.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian, LittleEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// assert!(w.write_as_from::<LittleEndian, u32>(0x12_34_56_78).is_ok());
    /// assert_eq!(w.into_writer(), &[0x78, 0x56, 0x34, 0x12]);
    /// ```
    fn write_as_from<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive;

    /// Pads the stream by writing 0 over the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    ///
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// let mut w = BitWriter::endian(vec![], BigEndian);
    /// assert!(w.write_bit(true).is_ok());
    /// assert!(w.pad(7).is_ok());
    /// assert_eq!(w.into_writer(), &[0b1_0000000]);
    /// ```
    fn pad(&mut self, mut bits: u32) -> io::Result<()> {
        loop {
            match bits {
                0 => break Ok(()),
                bits @ 1..64 => break self.write_var(bits, 0u64),
                _ => {
                    self.write::<64, u64>(0)?;
                    bits -= 64;
                }
            }
        }
    }

    /// Writes the entirety of a byte buffer to the stream.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_var(8, 0x66u8).unwrap();
    /// writer.write_var(8, 0x6Fu8).unwrap();
    /// writer.write_var(8, 0x6Fu8).unwrap();
    /// writer.write_bytes(b"bar").unwrap();
    /// assert_eq!(writer.into_writer(), b"foobar");
    /// ```
    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        buf.iter().try_for_each(|b| self.write_unsigned::<8, _>(*b))
    }

    /// Writes `value` number of non `STOP_BIT` bits to the stream
    /// and then writes a `STOP_BIT`.  This field is variably-sized.
    /// `STOP_BIT` must be 0 or 1.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underyling stream.
    ///
    /// # Examples
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_unary::<0>(0).unwrap();
    /// writer.write_unary::<0>(3).unwrap();
    /// writer.write_unary::<0>(10).unwrap();
    /// assert_eq!(writer.into_writer(), [0b01110111, 0b11111110]);
    /// ```
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), LittleEndian);
    /// writer.write_unary::<0>(0).unwrap();
    /// writer.write_unary::<0>(3).unwrap();
    /// writer.write_unary::<0>(10).unwrap();
    /// assert_eq!(writer.into_writer(), [0b11101110, 0b01111111]);
    /// ```
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_unary::<1>(0).unwrap();
    /// writer.write_unary::<1>(3).unwrap();
    /// writer.write_unary::<1>(10).unwrap();
    /// assert_eq!(writer.into_writer(), [0b10001000, 0b00000001]);
    /// ```
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{LittleEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), LittleEndian);
    /// writer.write_unary::<1>(0).unwrap();
    /// writer.write_unary::<1>(3).unwrap();
    /// writer.write_unary::<1>(10).unwrap();
    /// assert_eq!(writer.into_writer(), [0b00010001, 0b10000000]);
    /// ```
    fn write_unary<const STOP_BIT: u8>(&mut self, mut value: u32) -> io::Result<()> {
        const {
            assert!(matches!(STOP_BIT, 0 | 1), "stop bit must be 0 or 1");
        }

        const MAX: BitCount<32> = BitCount::new::<32>();

        match STOP_BIT {
            0 => {
                while value > 0 {
                    let to_write = MAX.min(value);
                    self.write_checked(to_write.all::<u32>())?;
                    value -= u32::from(to_write);
                }
                self.write_bit(false)
            }
            1 => {
                while value > 0 {
                    let to_write = MAX.min(value);
                    self.write_checked(to_write.none::<u32>())?;
                    value -= u32::from(to_write);
                }
                self.write_bit(true)
            }
            _ => unreachable!(),
        }
    }

    /// Writes checked value that is known to fit a given number of bits
    fn write_checked<C: Checkable>(&mut self, value: C) -> io::Result<()> {
        // a naive default implementation
        value.write(self)
    }

    /// Builds and writes complex type
    fn build<T: ToBitStream>(&mut self, build: &T) -> Result<(), T::Error> {
        build.to_writer(self)
    }

    /// Builds and writes complex type with context
    fn build_with<'a, T: ToBitStreamWith<'a>>(
        &mut self,
        build: &T,
        context: &T::Context,
    ) -> Result<(), T::Error> {
        build.to_writer(self, context)
    }

    /// Builds and writes complex type with owned context
    fn build_using<T: ToBitStreamUsing>(
        &mut self,
        build: &T,
        context: T::Context,
    ) -> Result<(), T::Error> {
        build.to_writer(self, context)
    }

    /// Returns true if the stream is aligned at a whole byte.
    ///
    /// # Example
    /// ```
    /// use std::io::{Write, sink};
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(sink(), BigEndian);
    /// assert_eq!(writer.byte_aligned(), true);
    /// writer.write_var(1, 0u8).unwrap();
    /// assert_eq!(writer.byte_aligned(), false);
    /// writer.write_var(7, 0u8).unwrap();
    /// assert_eq!(writer.byte_aligned(), true);
    /// ```
    fn byte_aligned(&self) -> bool;

    /// Pads the stream with 0 bits until it is aligned at a whole byte.
    /// Does nothing if the stream is already aligned.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underyling stream.
    ///
    /// # Example
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_var(1, 0u8).unwrap();
    /// writer.byte_align().unwrap();
    /// writer.write_var(8, 0xFFu8).unwrap();
    /// assert_eq!(writer.into_writer(), [0x00, 0xFF]);
    /// ```
    fn byte_align(&mut self) -> io::Result<()> {
        while !BitWrite::byte_aligned(self) {
            self.write_bit(false)?;
        }
        Ok(())
    }

    /// Given a symbol, writes its representation to the output stream as bits.
    /// Generates no output if the symbol isn't defined in the Huffman tree.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// use bitstream_io::define_huffman_tree;
    ///
    /// define_huffman_tree!(TreeName : char = ['a', ['b', ['c', 'd']]]);
    /// // 'a' is 0
    /// // 'b' is 1 -> 0
    /// // 'c' is 1 -> 1 -> 0
    /// // 'd' is 1 -> 1 -> 1
    ///
    /// let mut writer = BitWriter::endian(vec![], BigEndian);
    /// writer.write_huffman::<TreeName>('b').unwrap();
    /// writer.write_huffman::<TreeName>('c').unwrap();
    /// writer.write_huffman::<TreeName>('d').unwrap();
    /// assert_eq!(writer.into_writer(), [0b10_110_111]);
    /// ```
    fn write_huffman<T>(&mut self, value: T::Symbol) -> io::Result<()>
    where
        T: crate::huffman::ToBits,
    {
        T::to_bits(value, |b| self.write_bit(b))
    }

    /// Writes a number using a variable using a variable width integer.
    /// This optimises the case when the number is small.
    ///
    /// Given a 4-bit VBR field, any 3-bit value (0 through 7) is encoded directly, with the high bit set to zero.
    /// Values larger than N-1 bits emit their bits in a series of N-1 bit chunks, where all but the last set the high bit.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_unsigned_vbr::<4,_>(7u32);
    /// writer.write_unsigned_vbr::<4,_>(100u32);
    /// assert_eq!(writer.into_writer(), [0b0111_1100, 0b1100_0001]);
    /// ```
    fn write_unsigned_vbr<const FIELD_SIZE: u32, U: UnsignedInteger>(
        &mut self,
        value: U,
    ) -> io::Result<()> {
        const { assert!(FIELD_SIZE >= 2 && FIELD_SIZE < U::BITS_SIZE) };
        let payload_bits = FIELD_SIZE - 1;
        let continuation_bit = U::ONE.shl(payload_bits);
        let payload_mask = continuation_bit.sub(U::ONE);
        let mut value = value;

        loop {
            let payload = value & payload_mask;
            value >>= payload_bits;
            if value != U::ZERO {
                self.write_unsigned::<FIELD_SIZE, U>(payload | continuation_bit)?;
            } else {
                self.write_unsigned::<FIELD_SIZE, U>(payload)?;
                break;
            }
        }
        Ok(())
    }

    /// Writes a number using a variable using a variable width integer.
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
    ///
    /// # Example
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_signed_vbr::<4,_>(3);
    /// writer.write_signed_vbr::<4,_>(-50);
    /// assert_eq!(writer.into_writer(), [0b0110_1011, 0b1100_0001]);
    /// ```
    #[inline]
    fn write_signed_vbr<const FIELD_SIZE: u32, I: SignedInteger>(
        &mut self,
        value: I,
    ) -> io::Result<()> {
        let zig_zag = value.shl(1).bitxor(value.shr(I::BITS_SIZE - 1));
        self.write_unsigned_vbr::<FIELD_SIZE, _>(zig_zag.as_non_negative())
    }

    /// Writes a signed or unsigned variable width integer to the stream
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_vbr::<4,_>(6u32);
    /// writer.write_vbr::<4,_>(-50i32);
    /// assert_eq!(writer.into_writer(), [0b0110_1011, 0b1100_0001]);
    /// ```
    #[inline]
    fn write_vbr<const FIELD_SIZE: u32, I: VBRInteger>(&mut self, value: I) -> io::Result<()> {
        I::write_vbr::<FIELD_SIZE, _>(value, self)
    }

    /// Creates a "by reference" adaptor for this `BitWrite`
    ///
    /// The returned adapter also implements `BitWrite`
    /// and will borrow the current reader.
    ///
    /// # Example
    /// ```
    /// use bitstream_io::{BitWriter, BitWrite, BigEndian};
    ///
    /// fn build<W: BitWrite>(w: W) {
    ///     // perform some building
    /// }
    ///
    /// let mut writer = BitWriter::endian(vec![], BigEndian);
    /// // performing building by reference
    /// build(writer.by_ref());
    /// // original owned writer still available
    /// writer.write::<8, u8>(0).unwrap();
    /// assert_eq!(writer.into_writer(), &[0]);
    /// ```
    #[inline]
    fn by_ref(&mut self) -> &mut Self {
        self
    }
}

impl<W: BitWrite + ?Sized> BitWrite for &mut W {
    #[inline]
    fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        (**self).write_bit(bit)
    }

    #[inline]
    fn write<const BITS: u32, I>(&mut self, value: I) -> io::Result<()>
    where
        I: Integer,
    {
        (**self).write::<BITS, I>(value)
    }

    #[inline]
    fn write_const<const BITS: u32, const VALUE: u32>(&mut self) -> io::Result<()> {
        (**self).write_const::<BITS, VALUE>()
    }

    #[inline]
    fn write_var<I>(&mut self, bits: u32, value: I) -> io::Result<()>
    where
        I: Integer,
    {
        (**self).write_var(bits, value)
    }

    #[inline]
    fn write_unsigned<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        (**self).write_unsigned::<BITS, U>(value)
    }

    #[inline]
    fn write_unsigned_var<U>(&mut self, bits: u32, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        (**self).write_unsigned_var(bits, value)
    }

    #[inline]
    fn write_signed<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        (**self).write_signed::<BITS, S>(value)
    }

    #[inline(always)]
    fn write_signed_var<S>(&mut self, bits: u32, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        (**self).write_signed_var(bits, value)
    }

    #[inline]
    fn write_count<const MAX: u32>(&mut self, count: BitCount<MAX>) -> io::Result<()> {
        (**self).write_count::<MAX>(count)
    }

    #[inline]
    fn write_counted<const MAX: u32, I>(&mut self, bits: BitCount<MAX>, value: I) -> io::Result<()>
    where
        I: Integer + Sized,
    {
        (**self).write_counted::<MAX, I>(bits, value)
    }

    #[inline]
    fn write_unsigned_counted<const BITS: u32, U>(
        &mut self,
        bits: BitCount<BITS>,
        value: U,
    ) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        (**self).write_unsigned_counted::<BITS, U>(bits, value)
    }

    #[inline]
    fn write_signed_counted<const MAX: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<MAX>>,
        value: S,
    ) -> io::Result<()>
    where
        S: SignedInteger,
    {
        (**self).write_signed_counted::<MAX, S>(bits, value)
    }

    #[inline]
    fn write_from<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive,
    {
        (**self).write_from::<V>(value)
    }

    #[inline]
    fn write_as_from<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive,
    {
        (**self).write_as_from::<F, V>(value)
    }

    #[inline]
    fn pad(&mut self, bits: u32) -> io::Result<()> {
        (**self).pad(bits)
    }

    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        (**self).write_bytes(buf)
    }

    #[inline]
    fn write_unary<const STOP_BIT: u8>(&mut self, value: u32) -> io::Result<()> {
        (**self).write_unary::<STOP_BIT>(value)
    }

    #[inline]
    fn write_checked<C: Checkable>(&mut self, value: C) -> io::Result<()> {
        (**self).write_checked(value)
    }

    #[inline]
    fn build<T: ToBitStream>(&mut self, build: &T) -> Result<(), T::Error> {
        (**self).build(build)
    }

    #[inline]
    fn build_with<'a, T: ToBitStreamWith<'a>>(
        &mut self,
        build: &T,
        context: &T::Context,
    ) -> Result<(), T::Error> {
        (**self).build_with(build, context)
    }

    #[inline]
    fn byte_aligned(&self) -> bool {
        (**self).byte_aligned()
    }

    #[inline]
    fn byte_align(&mut self) -> io::Result<()> {
        (**self).byte_align()
    }

    #[inline]
    fn write_huffman<T>(&mut self, value: T::Symbol) -> io::Result<()>
    where
        T: crate::huffman::ToBits,
    {
        (**self).write_huffman::<T>(value)
    }
}

/// A compatibility trait for older code implementing [`BitWrite`]
///
/// This is a trait largely compatible with older code
/// from the 2.X.X version,
/// which one can use with a named import as needed.
///
/// New code should prefer the regular [`BitWrite`] trait.
///
/// # Example
/// ```
/// use bitstream_io::BitWrite2 as BitWrite;
/// use bitstream_io::{BitWriter, BigEndian};
/// let mut byte = vec![];
/// let mut writer = BitWriter::endian(byte, BigEndian);
/// writer.write::<u8>(4, 0b1111).unwrap();
/// writer.write_out::<4, u8>(0b0000).unwrap();
/// assert_eq!(writer.into_writer(), [0b1111_0000]);
/// ```
pub trait BitWrite2 {
    /// Writes a single bit to the stream.
    /// `true` indicates 1, `false` indicates 0
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.write_unsigned_out::<1, u8>(u8::from(bit))
    }

    /// Writes a signed or unsigned value to the stream using the given
    /// number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the input type is too small
    /// to hold the given number of bits.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    fn write<I>(&mut self, bits: u32, value: I) -> io::Result<()>
    where
        I: Integer;

    /// Writes a signed or unsigned value to the stream using the given
    /// const number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    fn write_out<const BITS: u32, I>(&mut self, value: I) -> io::Result<()>
    where
        I: Integer;

    /// Writes an unsigned value to the stream using the given
    /// number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the input type is too small
    /// to hold the given number of bits.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    fn write_unsigned<U>(&mut self, bits: u32, value: U) -> io::Result<()>
    where
        U: UnsignedInteger;

    /// Writes an unsigned value to the stream using the given
    /// const number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    #[inline]
    fn write_unsigned_out<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        self.write_unsigned(BITS, value)
    }

    /// Writes a twos-complement signed value to the stream
    /// with the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the input type is too small
    /// to hold the given number of bits.
    /// Returns an error if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    fn write_signed<S>(&mut self, bits: u32, value: S) -> io::Result<()>
    where
        S: SignedInteger;

    /// Writes a twos-complement signed value to the stream
    /// with the given const number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// Returns an error if the value is too large
    /// to fit the given number of bits.
    /// A compile-time error occurs if the number of bits is 0,
    /// since one bit is always needed for the sign.
    /// A compile-time error occurs if the given number of bits
    /// is larger than the output type.
    fn write_signed_out<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        self.write_signed(BITS, value)
    }

    /// Writes whole value to the stream whose size in bits
    /// is equal to its type's size.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn write_from<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive;

    /// Writes whole value to the stream whose size in bits
    /// is equal to its type's size in an endianness that may
    /// be different from the stream's endianness.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn write_as_from<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive;

    /// Pads the stream by writing 0 over the given number of bits.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn pad(&mut self, mut bits: u32) -> io::Result<()> {
        loop {
            match bits {
                0 => break Ok(()),
                bits @ 1..64 => break self.write(bits, 0u64),
                _ => {
                    self.write_out::<64, u64>(0)?;
                    bits -= 64;
                }
            }
        }
    }

    /// Writes the entirety of a byte buffer to the stream.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_var(8, 0x66u8).unwrap();
    /// writer.write_var(8, 0x6Fu8).unwrap();
    /// writer.write_var(8, 0x6Fu8).unwrap();
    /// writer.write_bytes(b"bar").unwrap();
    /// assert_eq!(writer.into_writer(), b"foobar");
    /// ```
    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        buf.iter()
            .try_for_each(|b| self.write_unsigned_out::<8, _>(*b))
    }

    /// Writes `value` number of 1 bits to the stream
    /// and then writes a 0 bit.  This field is variably-sized.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underyling stream.
    fn write_unary0(&mut self, value: u32) -> io::Result<()>;

    /// Writes `value` number of 0 bits to the stream
    /// and then writes a 1 bit.  This field is variably-sized.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underyling stream.
    fn write_unary1(&mut self, value: u32) -> io::Result<()>;

    /// Builds and writes complex type
    fn build<T: ToBitStream>(&mut self, build: &T) -> Result<(), T::Error>
    where
        Self: BitWrite,
    {
        build.to_writer(self)
    }

    /// Builds and writes complex type with context
    fn build_with<'a, T: ToBitStreamWith<'a>>(
        &mut self,
        build: &T,
        context: &T::Context,
    ) -> Result<(), T::Error>
    where
        Self: BitWrite,
    {
        build.to_writer(self, context)
    }

    /// Returns true if the stream is aligned at a whole byte.
    fn byte_aligned(&self) -> bool;

    /// Pads the stream with 0 bits until it is aligned at a whole byte.
    /// Does nothing if the stream is already aligned.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underyling stream.
    ///
    /// # Example
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite};
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_var(1, 0u8).unwrap();
    /// writer.byte_align().unwrap();
    /// writer.write_var(8, 0xFFu8).unwrap();
    /// assert_eq!(writer.into_writer(), [0x00, 0xFF]);
    /// ```
    fn byte_align(&mut self) -> io::Result<()> {
        while !self.byte_aligned() {
            self.write_bit(false)?;
        }
        Ok(())
    }

    /// Given a symbol, writes its representation to the output stream as bits.
    /// Generates no output if the symbol isn't defined in the Huffman tree.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Example
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite2};
    /// use bitstream_io::define_huffman_tree;
    /// define_huffman_tree!(TreeName : char = ['a', ['b', ['c', 'd']]]);
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// writer.write_huffman::<TreeName>('b').unwrap();
    /// writer.write_huffman::<TreeName>('c').unwrap();
    /// writer.write_huffman::<TreeName>('d').unwrap();
    /// assert_eq!(writer.into_writer(), [0b10_110_111]);
    /// ```
    fn write_huffman<T>(&mut self, value: T::Symbol) -> io::Result<()>
    where
        T: crate::huffman::ToBits,
    {
        T::to_bits(value, |b| self.write_bit(b))
    }
}

impl<W: BitWrite> BitWrite2 for W {
    #[inline]
    fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        BitWrite::write_bit(self, bit)
    }

    #[inline]
    fn write<I>(&mut self, bits: u32, value: I) -> io::Result<()>
    where
        I: Integer,
    {
        BitWrite::write_var(self, bits, value)
    }

    #[inline]
    fn write_out<const BITS: u32, I>(&mut self, value: I) -> io::Result<()>
    where
        I: Integer,
    {
        BitWrite::write::<BITS, I>(self, value)
    }

    #[inline]
    fn write_unsigned<U>(&mut self, bits: u32, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        BitWrite::write_unsigned_var::<U>(self, bits, value)
    }

    #[inline]
    fn write_unsigned_out<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        BitWrite::write_unsigned::<BITS, U>(self, value)
    }

    #[inline]
    fn write_signed<S>(&mut self, bits: u32, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        BitWrite::write_signed_var::<S>(self, bits, value)
    }

    #[inline]
    fn write_signed_out<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        BitWrite::write_signed::<BITS, S>(self, value)
    }

    #[inline]
    fn write_from<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive,
    {
        BitWrite::write_from(self, value)
    }

    #[inline]
    fn write_as_from<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive,
    {
        BitWrite::write_as_from::<F, V>(self, value)
    }

    #[inline]
    fn pad(&mut self, bits: u32) -> io::Result<()> {
        BitWrite::pad(self, bits)
    }

    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        BitWrite::write_bytes(self, buf)
    }

    #[inline]
    fn write_unary0(&mut self, value: u32) -> io::Result<()> {
        BitWrite::write_unary::<0>(self, value)
    }

    #[inline]
    fn write_unary1(&mut self, value: u32) -> io::Result<()> {
        BitWrite::write_unary::<1>(self, value)
    }

    #[inline]
    fn byte_aligned(&self) -> bool {
        BitWrite::byte_aligned(self)
    }

    #[inline]
    fn byte_align(&mut self) -> io::Result<()> {
        BitWrite::byte_align(self)
    }
}

impl<W: io::Write, E: Endianness> BitWrite for BitWriter<W, E> {
    fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        match E::push_bit_flush(&mut self.value, &mut self.bits, bit) {
            None => Ok(()),
            Some(byte) => write_byte(&mut self.writer, byte),
        }
    }

    #[inline(always)]
    fn write_unsigned<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        let Self {
            value: queue_value,
            bits: queue_bits,
            writer,
            ..
        } = self;

        E::write_bits_checked(
            writer,
            queue_value,
            queue_bits,
            CheckedUnsigned::<BITS, U>::new_fixed::<BITS>(value)?,
        )
    }

    fn write_unsigned_counted<const BITS: u32, U>(
        &mut self,
        count: BitCount<BITS>,
        value: U,
    ) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        let Self {
            value: queue_value,
            bits: queue_bits,
            writer,
            ..
        } = self;

        E::write_bits_checked(
            writer,
            queue_value,
            queue_bits,
            CheckedUnsigned::new(count, value)?,
        )
    }

    #[inline(always)]
    fn write_signed_counted<const BITS: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<BITS>>,
        value: S,
    ) -> io::Result<()>
    where
        S: SignedInteger,
    {
        E::write_signed_bits_checked(
            &mut self.writer,
            &mut self.value,
            &mut self.bits,
            CheckedSigned::new(
                bits.try_into().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "signed writes need at least 1 bit for sign",
                    )
                })?,
                value,
            )?,
        )
    }

    #[inline]
    fn write_signed<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        E::write_signed_bits_checked(
            &mut self.writer,
            &mut self.value,
            &mut self.bits,
            CheckedSigned::<BITS, _>::new_fixed::<BITS>(value)?,
        )
    }

    #[inline]
    fn write_from<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive,
    {
        E::write_bytes::<8, _>(
            &mut self.writer,
            &mut self.value,
            self.bits,
            E::primitive_to_bytes(value).as_ref(),
        )
    }

    #[inline]
    fn write_as_from<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive,
    {
        F::write_bytes::<8, _>(
            &mut self.writer,
            &mut self.value,
            self.bits,
            F::primitive_to_bytes(value).as_ref(),
        )
    }

    #[inline]
    fn write_checked<C: Checkable>(&mut self, value: C) -> io::Result<()> {
        value.write_endian::<E, _>(&mut self.writer, &mut self.value, &mut self.bits)
    }

    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        E::write_bytes::<1024, _>(&mut self.writer, &mut self.value, self.bits, buf)
    }

    #[inline(always)]
    fn byte_aligned(&self) -> bool {
        self.bits == 0
    }
}

/// An error returned if performing math operations would overflow
#[derive(Copy, Clone, Debug)]
pub struct Overflowed;

impl fmt::Display for Overflowed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "overflow occured in counter".fmt(f)
    }
}

impl core::error::Error for Overflowed {}

impl From<Overflowed> for io::Error {
    fn from(Overflowed: Overflowed) -> Self {
        io::Error::new(
            #[cfg(feature = "std")]
            {
                io::ErrorKind::StorageFull
            },
            #[cfg(not(feature = "std"))]
            {
                io::ErrorKind::Other
            },
            "bitstream accumulator overflow",
        )
    }
}

/// A common trait for integer types for performing math operations
/// which may check for overflow.
pub trait Counter: Default + Sized + From<u8> + TryFrom<u32> + TryFrom<usize> {
    /// add rhs to self, returning `Overflowed` if the result is too large
    fn checked_add_assign(&mut self, rhs: Self) -> Result<(), Overflowed>;

    /// multiply self by rhs, returning `Overflowed` if the result is too large
    fn checked_mul(self, rhs: Self) -> Result<Self, Overflowed>;

    /// returns `true` if the number if bits written is divisible by 8
    fn byte_aligned(&self) -> bool;
}

macro_rules! define_counter {
    ($t:ty) => {
        impl Counter for $t {
            fn checked_add_assign(&mut self, rhs: Self) -> Result<(), Overflowed> {
                *self = <$t>::checked_add(*self, rhs).ok_or(Overflowed)?;
                Ok(())
            }

            fn checked_mul(self, rhs: Self) -> Result<Self, Overflowed> {
                <$t>::checked_mul(self, rhs).ok_or(Overflowed)
            }

            fn byte_aligned(&self) -> bool {
                self % 8 == 0
            }
        }
    };
}

define_counter!(u8);
define_counter!(u16);
define_counter!(u32);
define_counter!(u64);
define_counter!(u128);

/// For counting the number of bits written but generating no output.
///
/// # Example
/// ```
/// use bitstream_io::{BigEndian, BitWrite, BitsWritten};
/// let mut writer: BitsWritten<u32> = BitsWritten::new();
/// writer.write_var(1, 0b1u8).unwrap();
/// writer.write_var(2, 0b01u8).unwrap();
/// writer.write_var(5, 0b10111u8).unwrap();
/// assert_eq!(writer.written(), 8);
/// ```
#[derive(Default)]
pub struct BitsWritten<N> {
    bits: N,
}

impl<N: Default> BitsWritten<N> {
    /// Creates new empty BitsWritten value
    #[inline]
    pub fn new() -> Self {
        Self { bits: N::default() }
    }
}

impl<N: Copy> BitsWritten<N> {
    /// Returns number of bits written
    #[inline]
    pub fn written(&self) -> N {
        self.bits
    }
}

impl<N> BitsWritten<N> {
    /// Returns number of bits written
    #[inline]
    pub fn into_written(self) -> N {
        self.bits
    }
}

impl<N: Counter> BitWrite for BitsWritten<N> {
    #[inline]
    fn write_bit(&mut self, _bit: bool) -> io::Result<()> {
        self.bits.checked_add_assign(1u8.into())?;
        Ok(())
    }

    #[inline]
    fn write_const<const BITS: u32, const VALUE: u32>(&mut self) -> io::Result<()> {
        const {
            assert!(
                BITS == 0 || VALUE <= (u32::ALL >> (u32::BITS_SIZE - BITS)),
                "excessive value for bits written"
            );
        }

        self.bits
            .checked_add_assign(BITS.try_into().map_err(|_| Overflowed)?)?;
        Ok(())
    }

    #[inline]
    fn write_unsigned<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        const {
            assert!(BITS <= U::BITS_SIZE, "excessive bits for type written");
        }

        if BITS == 0 {
            Ok(())
        } else if value <= (U::ALL >> (U::BITS_SIZE - BITS)) {
            self.bits
                .checked_add_assign(BITS.try_into().map_err(|_| Overflowed)?)?;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive value for bits written",
            ))
        }
    }

    #[inline]
    fn write_signed<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        let SignedBitCount {
            bits: BitCount { bits },
            unsigned,
        } = const {
            assert!(BITS <= S::BITS_SIZE, "excessive bits for type written");
            let count = BitCount::<BITS>::new::<BITS>().signed_count();
            assert!(
                count.is_some(),
                "signed writes need at least 1 bit for sign"
            );
            count.unwrap()
        };

        // doesn't matter which side the sign is on
        // so long as it's added to the bit count
        self.bits.checked_add_assign(1u8.into())?;

        self.write_unsigned_counted(
            unsigned,
            if value.is_negative() {
                value.as_negative(bits)
            } else {
                value.as_non_negative()
            },
        )
    }

    #[inline]
    fn write_unsigned_counted<const MAX: u32, U>(
        &mut self,
        BitCount { bits }: BitCount<MAX>,
        value: U,
    ) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        if MAX <= U::BITS_SIZE || bits <= U::BITS_SIZE {
            if bits == 0 {
                Ok(())
            } else if value <= U::ALL >> (U::BITS_SIZE - bits) {
                self.bits
                    .checked_add_assign(bits.try_into().map_err(|_| Overflowed)?)?;
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "excessive value for bits written",
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive bits for type written",
            ))
        }
    }

    #[inline]
    fn write_signed_counted<const MAX: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<MAX>>,
        value: S,
    ) -> io::Result<()>
    where
        S: SignedInteger,
    {
        let SignedBitCount {
            bits: BitCount { bits },
            unsigned,
        } = bits.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "signed writes need at least 1 bit for sign",
            )
        })?;

        if MAX <= S::BITS_SIZE || bits <= S::BITS_SIZE {
            // doesn't matter which side the sign is on
            // so long as it's added to the bit count
            self.bits.checked_add_assign(1u8.into())?;

            self.write_unsigned_counted(
                unsigned,
                if value.is_negative() {
                    value.as_negative(bits)
                } else {
                    value.as_non_negative()
                },
            )
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "excessive bits for type written",
            ))
        }
    }

    #[inline]
    fn write_from<V>(&mut self, _: V) -> io::Result<()>
    where
        V: Primitive,
    {
        self.bits.checked_add_assign(
            N::try_from(core::mem::size_of::<V>())
                .map_err(|_| Overflowed)?
                .checked_mul(8u8.into())?,
        )?;
        Ok(())
    }

    #[inline]
    fn write_as_from<F, V>(&mut self, _: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive,
    {
        self.bits.checked_add_assign(
            N::try_from(core::mem::size_of::<V>())
                .map_err(|_| Overflowed)?
                .checked_mul(8u8.into())?,
        )?;
        Ok(())
    }

    #[inline]
    fn pad(&mut self, bits: u32) -> io::Result<()> {
        self.bits
            .checked_add_assign(bits.try_into().map_err(|_| Overflowed)?)?;
        Ok(())
    }

    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        self.bits.checked_add_assign(
            N::try_from(buf.len())
                .map_err(|_| Overflowed)?
                .checked_mul(8u8.into())?,
        )?;
        Ok(())
    }

    fn write_unary<const STOP_BIT: u8>(&mut self, value: u32) -> io::Result<()> {
        const {
            assert!(matches!(STOP_BIT, 0 | 1), "stop bit must be 0 or 1");
        }

        self.bits
            .checked_add_assign(value.try_into().map_err(|_| Overflowed)?)?;
        self.bits.checked_add_assign(1u8.into())?;
        Ok(())
    }

    fn write_checked<C: Checkable>(&mut self, value: C) -> io::Result<()> {
        Ok(self
            .bits
            .checked_add_assign(value.written_bits().try_into().map_err(|_| Overflowed)?)?)
    }

    #[inline]
    fn byte_aligned(&self) -> bool {
        self.bits.byte_aligned()
    }
}

/// For counting the number of bits written but generating no output.
///
/// # Example
/// ```
/// use bitstream_io::{BigEndian, BitWrite, BitCounter};
/// let mut writer: BitCounter<u32, BigEndian> = BitCounter::new();
/// writer.write_var(1, 0b1u8).unwrap();
/// writer.write_var(2, 0b01u8).unwrap();
/// writer.write_var(5, 0b10111u8).unwrap();
/// assert_eq!(writer.written(), 8);
/// ```
#[derive(Default)]
#[deprecated(since = "4.0.0", note = "use of BitsWritten is preferred")]
pub struct BitCounter<N, E: Endianness> {
    bits: BitsWritten<N>,
    phantom: PhantomData<E>,
}

#[allow(deprecated)]
impl<N: Default, E: Endianness> BitCounter<N, E> {
    /// Creates new counter
    #[inline]
    pub fn new() -> Self {
        BitCounter {
            bits: BitsWritten::new(),
            phantom: PhantomData,
        }
    }
}

#[allow(deprecated)]
impl<N: Copy, E: Endianness> BitCounter<N, E> {
    /// Returns number of bits written
    #[inline]
    pub fn written(&self) -> N {
        self.bits.written()
    }
}

#[allow(deprecated)]
impl<N, E: Endianness> BitCounter<N, E> {
    /// Returns number of bits written
    #[inline]
    pub fn into_written(self) -> N {
        self.bits.into_written()
    }
}

#[allow(deprecated)]
impl<N, E> BitWrite for BitCounter<N, E>
where
    E: Endianness,
    N: Counter,
{
    #[inline]
    fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        BitWrite::write_bit(&mut self.bits, bit)
    }

    #[inline]
    fn write_const<const BITS: u32, const VALUE: u32>(&mut self) -> io::Result<()> {
        BitWrite::write_const::<BITS, VALUE>(&mut self.bits)
    }

    #[inline]
    fn write_unsigned<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        BitWrite::write_unsigned::<BITS, U>(&mut self.bits, value)
    }

    #[inline]
    fn write_signed<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
    where
        S: SignedInteger,
    {
        BitWrite::write_signed::<BITS, S>(&mut self.bits, value)
    }

    #[inline]
    fn write_unsigned_counted<const MAX: u32, U>(
        &mut self,
        count: BitCount<MAX>,
        value: U,
    ) -> io::Result<()>
    where
        U: UnsignedInteger,
    {
        BitWrite::write_unsigned_counted::<MAX, U>(&mut self.bits, count, value)
    }

    #[inline]
    fn write_signed_counted<const MAX: u32, S>(
        &mut self,
        bits: impl TryInto<SignedBitCount<MAX>>,
        value: S,
    ) -> io::Result<()>
    where
        S: SignedInteger,
    {
        BitWrite::write_signed_counted::<MAX, S>(&mut self.bits, bits, value)
    }

    #[inline]
    fn write_from<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive,
    {
        BitWrite::write_from(&mut self.bits, value)
    }

    #[inline]
    fn write_as_from<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive,
    {
        BitWrite::write_as_from::<F, V>(&mut self.bits, value)
    }

    #[inline]
    fn pad(&mut self, bits: u32) -> io::Result<()> {
        BitWrite::pad(&mut self.bits, bits)
    }

    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        BitWrite::write_bytes(&mut self.bits, buf)
    }

    fn write_unary<const STOP_BIT: u8>(&mut self, value: u32) -> io::Result<()> {
        BitWrite::write_unary::<STOP_BIT>(&mut self.bits, value)
    }

    #[inline]
    fn byte_aligned(&self) -> bool {
        BitWrite::byte_aligned(&self.bits)
    }
}

#[cfg(feature = "alloc")]
mod bit_recorder {
    use super::*;

    /// For recording writes in order to play them back on another writer
    /// # Example
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, BitWriter, BitWrite, BitRecorder};
    /// let mut recorder: BitRecorder<u32, BigEndian> = BitRecorder::new();
    /// recorder.write_var(1, 0b1u8).unwrap();
    /// recorder.write_var(2, 0b01u8).unwrap();
    /// recorder.write_var(5, 0b10111u8).unwrap();
    /// assert_eq!(recorder.written(), 8);
    /// let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    /// recorder.playback(&mut writer);
    /// assert_eq!(writer.into_writer(), [0b10110111]);
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub struct BitRecorder<N, E: Endianness> {
        writer: BitWriter<Vec<u8>, E>,
        phantom: PhantomData<N>,
    }

    impl<N: Counter, E: Endianness> BitRecorder<N, E> {
        /// Creates new recorder
        #[inline]
        pub fn new() -> Self {
            BitRecorder {
                writer: BitWriter::new(Vec::new()),
                phantom: PhantomData,
            }
        }

        /// Creates new recorder sized for the given number of bytes
        #[inline]
        pub fn with_capacity(bytes: usize) -> Self {
            BitRecorder {
                writer: BitWriter::new(Vec::with_capacity(bytes)),
                phantom: PhantomData,
            }
        }

        /// Creates new recorder with the given endianness
        #[inline]
        pub fn endian(endian: E) -> Self {
            BitRecorder {
                writer: BitWriter::endian(Vec::new(), endian),
                phantom: PhantomData,
            }
        }

        /// Returns number of bits written
        ///
        /// # Panics
        ///
        /// Panics if the number of bits written is
        /// larger than the maximum supported by the counter type.
        /// Use [`BitRecorder::written_checked`] for a non-panicking
        /// alternative.
        #[inline]
        pub fn written(&self) -> N {
            self.written_checked().unwrap()
        }

        /// Returns number of bits written
        ///
        /// # Errors
        ///
        /// Returns an error if the number of bits written overflows
        /// our counter type.
        #[inline]
        pub fn written_checked(&self) -> Result<N, Overflowed> {
            let mut written = N::try_from(self.writer.writer.len())
                .map_err(|_| Overflowed)?
                .checked_mul(8u8.into())?;

            written.checked_add_assign(N::try_from(self.writer.bits).map_err(|_| Overflowed)?)?;

            Ok(written)
        }

        /// Plays recorded writes to the given writer
        #[inline]
        pub fn playback<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
            writer.write_bytes(self.writer.writer.as_slice())?;
            writer.write_var(self.writer.bits, self.writer.value)?;
            Ok(())
        }

        /// Clears recorder, removing all values
        #[inline]
        pub fn clear(&mut self) {
            self.writer = BitWriter::new({
                let mut v = core::mem::take(&mut self.writer.writer);
                v.clear();
                v
            });
        }
    }

    impl<N: Counter, E: Endianness> Default for BitRecorder<N, E> {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl<N, E> BitWrite for BitRecorder<N, E>
    where
        E: Endianness,
        N: Counter,
    {
        #[inline]
        fn write_bit(&mut self, bit: bool) -> io::Result<()> {
            BitWrite::write_bit(&mut self.writer, bit)
        }

        #[inline]
        fn write<const BITS: u32, I>(&mut self, value: I) -> io::Result<()>
        where
            I: Integer,
        {
            BitWrite::write::<BITS, I>(&mut self.writer, value)
        }

        #[inline]
        fn write_const<const BITS: u32, const VALUE: u32>(&mut self) -> io::Result<()> {
            self.writer.write_const::<BITS, VALUE>()
        }

        #[inline]
        fn write_var<I>(&mut self, bits: u32, value: I) -> io::Result<()>
        where
            I: Integer,
        {
            self.writer.write_var(bits, value)
        }

        #[inline]
        fn write_unsigned<const BITS: u32, U>(&mut self, value: U) -> io::Result<()>
        where
            U: UnsignedInteger,
        {
            BitWrite::write_unsigned::<BITS, U>(&mut self.writer, value)
        }

        #[inline]
        fn write_unsigned_var<U>(&mut self, bits: u32, value: U) -> io::Result<()>
        where
            U: UnsignedInteger,
        {
            self.writer.write_unsigned_var(bits, value)
        }

        #[inline]
        fn write_signed<const BITS: u32, S>(&mut self, value: S) -> io::Result<()>
        where
            S: SignedInteger,
        {
            BitWrite::write_signed::<BITS, S>(&mut self.writer, value)
        }

        #[inline(always)]
        fn write_signed_var<S>(&mut self, bits: u32, value: S) -> io::Result<()>
        where
            S: SignedInteger,
        {
            self.writer.write_signed_var(bits, value)
        }

        #[inline]
        fn write_count<const MAX: u32>(&mut self, count: BitCount<MAX>) -> io::Result<()> {
            self.writer.write_count::<MAX>(count)
        }

        #[inline]
        fn write_counted<const MAX: u32, I>(
            &mut self,
            bits: BitCount<MAX>,
            value: I,
        ) -> io::Result<()>
        where
            I: Integer + Sized,
        {
            self.writer.write_counted::<MAX, I>(bits, value)
        }

        #[inline]
        fn write_unsigned_counted<const BITS: u32, U>(
            &mut self,
            bits: BitCount<BITS>,
            value: U,
        ) -> io::Result<()>
        where
            U: UnsignedInteger,
        {
            self.writer.write_unsigned_counted::<BITS, U>(bits, value)
        }

        #[inline]
        fn write_signed_counted<const MAX: u32, S>(
            &mut self,
            bits: impl TryInto<SignedBitCount<MAX>>,
            value: S,
        ) -> io::Result<()>
        where
            S: SignedInteger,
        {
            self.writer.write_signed_counted::<MAX, S>(bits, value)
        }

        #[inline]
        fn write_from<V>(&mut self, value: V) -> io::Result<()>
        where
            V: Primitive,
        {
            BitWrite::write_from::<V>(&mut self.writer, value)
        }

        #[inline]
        fn write_as_from<F, V>(&mut self, value: V) -> io::Result<()>
        where
            F: Endianness,
            V: Primitive,
        {
            BitWrite::write_as_from::<F, V>(&mut self.writer, value)
        }

        #[inline]
        fn pad(&mut self, bits: u32) -> io::Result<()> {
            BitWrite::pad(&mut self.writer, bits)
        }

        #[inline]
        fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
            BitWrite::write_bytes(&mut self.writer, buf)
        }

        #[inline]
        fn write_unary<const STOP_BIT: u8>(&mut self, value: u32) -> io::Result<()> {
            self.writer.write_unary::<STOP_BIT>(value)
        }

        #[inline]
        fn build<T: ToBitStream>(&mut self, build: &T) -> Result<(), T::Error> {
            BitWrite::build(&mut self.writer, build)
        }

        #[inline]
        fn build_with<'a, T: ToBitStreamWith<'a>>(
            &mut self,
            build: &T,
            context: &T::Context,
        ) -> Result<(), T::Error> {
            BitWrite::build_with(&mut self.writer, build, context)
        }

        #[inline]
        fn byte_aligned(&self) -> bool {
            BitWrite::byte_aligned(&self.writer)
        }

        #[inline]
        fn byte_align(&mut self) -> io::Result<()> {
            BitWrite::byte_align(&mut self.writer)
        }

        #[inline]
        fn write_huffman<T>(&mut self, value: T::Symbol) -> io::Result<()>
        where
            T: crate::huffman::ToBits,
        {
            BitWrite::write_huffman::<T>(&mut self.writer, value)
        }
    }

    impl<N: PartialOrd + Counter + Copy, E: Endianness> BitRecorder<N, E> {
        /// Returns shortest option between ourself and candidate
        ///
        /// Executes fallible closure on emptied candidate recorder,
        /// compares the lengths of ourself and the candidate,
        /// and returns the shorter of the two.
        ///
        /// If the new candidate is shorter, we swap ourself and
        /// the candidate so any recorder capacity can be reused.
        ///
        /// # Example
        ///
        /// ```
        /// use bitstream_io::{BitRecorder, BitWrite, BigEndian};
        ///
        /// let mut best = BitRecorder::<u8, BigEndian>::new();
        /// let mut candidate = BitRecorder::new();
        ///
        /// // write an 8 bit value to our initial candidate
        /// best.write::<8, u8>(0);
        /// assert_eq!(best.written(), 8);
        ///
        /// // try another candidate which writes 4 bits
        /// best = best.best(&mut candidate, |w| {
        ///     w.write::<4, u8>(0)
        /// }).unwrap();
        ///
        /// // which becomes our new best candidate
        /// assert_eq!(best.written(), 4);
        ///
        /// // finally, try a not-so-best candidate
        /// // which writes 10 bits
        /// best = best.best(&mut candidate, |w| {
        ///     w.write::<10, u16>(0)
        /// }).unwrap();
        ///
        /// // so our best candidate remains 4 bits
        /// assert_eq!(best.written(), 4);
        /// ```
        pub fn best<F>(
            mut self,
            candidate: &mut Self,
            f: impl FnOnce(&mut Self) -> Result<(), F>,
        ) -> Result<Self, F> {
            candidate.clear();

            f(candidate)?;

            if candidate.written() < self.written() {
                core::mem::swap(&mut self, candidate);
            }

            Ok(self)
        }
    }
}

#[inline]
fn write_byte<W>(mut writer: W, byte: u8) -> io::Result<()>
where
    W: io::Write,
{
    writer.write_all(core::slice::from_ref(&byte))
}

/// For writing aligned bytes to a stream of bytes in a given endianness.
///
/// This only writes aligned values and maintains no internal state.
pub struct ByteWriter<W: io::Write, E: Endianness> {
    phantom: PhantomData<E>,
    writer: W,
}

impl<W: io::Write, E: Endianness> ByteWriter<W, E> {
    /// Wraps a ByteWriter around something that implements `Write`
    pub fn new(writer: W) -> ByteWriter<W, E> {
        ByteWriter {
            phantom: PhantomData,
            writer,
        }
    }

    /// Wraps a BitWriter around something that implements `Write`
    /// with the given endianness.
    pub fn endian(writer: W, _endian: E) -> ByteWriter<W, E> {
        ByteWriter {
            phantom: PhantomData,
            writer,
        }
    }

    /// Unwraps internal writer and disposes of `ByteWriter`.
    /// Any unwritten partial bits are discarded.
    #[inline]
    pub fn into_writer(self) -> W {
        self.writer
    }

    /// Provides mutable reference to internal writer.
    #[inline]
    pub fn writer(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Converts `ByteWriter` to `BitWriter` in the same endianness.
    #[inline]
    pub fn into_bitwriter(self) -> BitWriter<W, E> {
        BitWriter::new(self.into_writer())
    }

    /// Provides temporary `BitWriter` in the same endianness.
    ///
    /// # Warning
    ///
    /// Any unwritten bits left over when `BitWriter` is dropped are lost.
    #[inline]
    pub fn bitwriter(&mut self) -> BitWriter<&mut W, E> {
        BitWriter::new(self.writer())
    }
}

/// A trait for anything that can write aligned values to an output stream
pub trait ByteWrite {
    /// Writes whole numeric value to stream
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    /// # Examples
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, ByteWriter, ByteWrite};
    /// let mut writer = ByteWriter::endian(Vec::new(), BigEndian);
    /// writer.write(0b0000000011111111u16).unwrap();
    /// assert_eq!(writer.into_writer(), [0b00000000, 0b11111111]);
    /// ```
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{LittleEndian, ByteWriter, ByteWrite};
    /// let mut writer = ByteWriter::endian(Vec::new(), LittleEndian);
    /// writer.write(0b0000000011111111u16).unwrap();
    /// assert_eq!(writer.into_writer(), [0b11111111, 0b00000000]);
    /// ```
    fn write<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive;

    /// Writes whole numeric value to stream in a potentially different endianness
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    ///
    /// # Examples
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, ByteWriter, ByteWrite, LittleEndian};
    /// let mut writer = ByteWriter::endian(Vec::new(), BigEndian);
    /// writer.write_as::<LittleEndian, u16>(0b0000000011111111).unwrap();
    /// assert_eq!(writer.into_writer(), [0b11111111, 0b00000000]);
    /// ```
    ///
    /// ```
    /// use std::io::Write;
    /// use bitstream_io::{BigEndian, ByteWriter, ByteWrite, LittleEndian};
    /// let mut writer = ByteWriter::endian(Vec::new(), LittleEndian);
    /// writer.write_as::<BigEndian, u16>(0b0000000011111111).unwrap();
    /// assert_eq!(writer.into_writer(), [0b00000000, 0b11111111]);
    /// ```
    fn write_as<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive;

    /// Writes the entirety of a byte buffer to the stream.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()>;

    /// Pads the stream by writing 0 over the given number of bytes.
    ///
    /// # Errors
    ///
    /// Passes along any I/O error from the underlying stream.
    fn pad(&mut self, bytes: u32) -> io::Result<()>;

    /// Builds and writes complex type
    fn build<T: ToByteStream>(&mut self, build: &T) -> Result<(), T::Error> {
        build.to_writer(self)
    }

    /// Builds and writes complex type with context
    fn build_with<'a, T: ToByteStreamWith<'a>>(
        &mut self,
        build: &T,
        context: &T::Context,
    ) -> Result<(), T::Error> {
        build.to_writer(self, context)
    }

    /// Builds and writes complex type with owned context
    fn build_using<T: ToByteStreamUsing>(
        &mut self,
        build: &T,
        context: T::Context,
    ) -> Result<(), T::Error> {
        build.to_writer(self, context)
    }

    /// Returns mutable reference to underlying writer
    fn writer_ref(&mut self) -> &mut dyn io::Write;
}

impl<W: io::Write, E: Endianness> ByteWrite for ByteWriter<W, E> {
    #[inline]
    fn write<V>(&mut self, value: V) -> io::Result<()>
    where
        V: Primitive,
    {
        self.writer.write_all(E::primitive_to_bytes(value).as_ref())
    }

    #[inline]
    fn write_as<F, V>(&mut self, value: V) -> io::Result<()>
    where
        F: Endianness,
        V: Primitive,
    {
        self.writer.write_all(F::primitive_to_bytes(value).as_ref())
    }

    #[inline]
    fn pad(&mut self, mut bytes: u32) -> io::Result<()> {
        let buf = [0u8; 8];

        while bytes > 0 {
            let to_write = bytes.min(8);
            self.write_bytes(&buf[0..to_write as usize])?;
            bytes -= to_write;
        }
        Ok(())
    }

    #[inline]
    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }

    #[inline]
    fn writer_ref(&mut self) -> &mut dyn io::Write {
        &mut self.writer
    }
}

/// Implemented by complex types that don't require any additional context
/// to build themselves to a writer
///
/// # Example
/// ```
/// use std::io::Read;
/// use bitstream_io::{BigEndian, BitWrite, BitWriter, ToBitStream};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct BlockHeader {
///     last_block: bool,
///     block_type: u8,
///     block_size: u32,
/// }
///
/// impl ToBitStream for BlockHeader {
///     type Error = std::io::Error;
///
///     fn to_writer<W: BitWrite + ?Sized>(&self, w: &mut W) -> std::io::Result<()> {
///         w.write_bit(self.last_block)?;
///         w.write::<7, _>(self.block_type)?;
///         w.write::<24, _>(self.block_size)
///     }
/// }
///
/// let mut data = Vec::new();
/// let mut writer = BitWriter::endian(&mut data, BigEndian);
/// writer.build(&BlockHeader { last_block: false, block_type: 4, block_size: 122 }).unwrap();
/// assert_eq!(data, b"\x04\x00\x00\x7A");
/// ```
pub trait ToBitStream {
    /// Error generated during building, such as `io::Error`
    type Error;

    /// Generate self to writer
    fn to_writer<W: BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized;

    /// Returns length of self in bits, if possible
    fn bits<C: Counter>(&self) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        let mut c: BitsWritten<C> = BitsWritten::default();
        self.to_writer(&mut c)?;
        Ok(c.into_written())
    }

    /// Returns total length of self, if possible
    #[deprecated(since = "4.0.0", note = "use of bits() is preferred")]
    #[inline]
    fn bits_len<C: Counter, E: Endianness>(&self) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        self.bits()
    }
}

/// Implemented by complex types that require additional context
/// to build themselves to a writer
pub trait ToBitStreamWith<'a> {
    /// Some context to use when writing
    type Context: 'a;

    /// Error generated during building, such as `io::Error`
    type Error;

    /// Generate self to writer
    fn to_writer<W: BitWrite + ?Sized>(
        &self,
        w: &mut W,
        context: &Self::Context,
    ) -> Result<(), Self::Error>
    where
        Self: Sized;

    /// Returns length of self in bits, if possible
    fn bits<C: Counter>(&self, context: &Self::Context) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        let mut c: BitsWritten<C> = BitsWritten::default();
        self.to_writer(&mut c, context)?;
        Ok(c.into_written())
    }

    /// Returns total length of self, if possible
    #[deprecated(since = "4.0.0", note = "use of len() is preferred")]
    #[inline]
    fn bits_len<C: Counter, E: Endianness>(&self, context: &Self::Context) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        self.bits(context)
    }
}

/// Implemented by complex types that consume additional context
/// to build themselves to a writer
pub trait ToBitStreamUsing {
    /// Some context to consume when writing
    type Context;

    /// Error generated during building, such as `io::Error`
    type Error;

    /// Generate self to writer
    fn to_writer<W: BitWrite + ?Sized>(
        &self,
        w: &mut W,
        context: Self::Context,
    ) -> Result<(), Self::Error>
    where
        Self: Sized;

    /// Returns length of self in bits, if possible
    fn bits<C: Counter>(&self, context: Self::Context) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        let mut c: BitsWritten<C> = BitsWritten::default();
        self.to_writer(&mut c, context)?;
        Ok(c.into_written())
    }
}

/// Implemented by complex types that don't require any additional context
/// to build themselves to a writer
pub trait ToByteStream {
    /// Error generated during building, such as `io::Error`
    type Error;

    /// Generate self to writer
    fn to_writer<W: ByteWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>
    where
        Self: Sized;

    /// Returns length of self in bytes, if possible
    fn bytes<C: Counter>(&self) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        let mut counter = ByteCount::default();
        self.to_writer(&mut counter)?;
        Ok(counter.writer.count)
    }
}

/// Implemented by complex types that require additional context
/// to build themselves to a writer
pub trait ToByteStreamWith<'a> {
    /// Some context to use when writing
    type Context: 'a;

    /// Error generated during building, such as `io::Error`
    type Error;

    /// Generate self to writer
    fn to_writer<W: ByteWrite + ?Sized>(
        &self,
        w: &mut W,
        context: &Self::Context,
    ) -> Result<(), Self::Error>
    where
        Self: Sized;

    /// Returns length of self in bytes, if possible
    fn bytes<C: Counter>(&self, context: &Self::Context) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        let mut counter = ByteCount::default();
        self.to_writer(&mut counter, context)?;
        Ok(counter.writer.count)
    }
}

/// Implemented by complex types that consume additional context
/// to build themselves to a writer
pub trait ToByteStreamUsing {
    /// Some context to consume when writing
    type Context;

    /// Error generated during building, such as `io::Error`
    type Error;

    /// Generate self to writer
    fn to_writer<W: ByteWrite + ?Sized>(
        &self,
        w: &mut W,
        context: Self::Context,
    ) -> Result<(), Self::Error>
    where
        Self: Sized;

    /// Returns length of self in bytes, if possible
    fn bytes<C: Counter>(&self, context: Self::Context) -> Result<C, Self::Error>
    where
        Self: Sized,
    {
        let mut counter = ByteCount::default();
        self.to_writer(&mut counter, context)?;
        Ok(counter.writer.count)
    }
}

#[derive(Default)]
struct ByteCounterWriter<C> {
    count: C,
}

impl<C: Counter> io::Write for ByteCounterWriter<C> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.count
            .checked_add_assign(buf.len().try_into().map_err(|_| Overflowed)?)?;

        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        // nothing to do
        Ok(())
    }
}

#[derive(Default)]
struct ByteCount<C> {
    writer: ByteCounterWriter<C>,
}

impl<C: Counter> ByteWrite for ByteCount<C> {
    fn write<V: Primitive>(&mut self, _value: V) -> io::Result<()> {
        self.writer.count.checked_add_assign(
            V::buffer()
                .as_ref()
                .len()
                .try_into()
                .map_err(|_| Overflowed)?,
        )?;

        Ok(())
    }

    fn write_as<F: Endianness, V: Primitive>(&mut self, _value: V) -> io::Result<()> {
        self.writer.count.checked_add_assign(
            V::buffer()
                .as_ref()
                .len()
                .try_into()
                .map_err(|_| Overflowed)?,
        )?;

        Ok(())
    }

    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer
            .count
            .checked_add_assign(buf.len().try_into().map_err(|_| Overflowed)?)?;

        Ok(())
    }

    fn pad(&mut self, bytes: u32) -> io::Result<()> {
        self.writer
            .count
            .checked_add_assign(bytes.try_into().map_err(|_| Overflowed)?)?;

        Ok(())
    }

    fn writer_ref(&mut self) -> &mut dyn io::Write {
        &mut self.writer
    }
}
