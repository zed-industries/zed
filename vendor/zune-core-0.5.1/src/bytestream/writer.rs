/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */
use crate::bytestream::{ZByteIoError, ZByteWriterTrait};

mod no_std_writer;
mod std_writer;

enum Mode {
    // Big endian
    BE,
    // Little Endian
    LE
}

/// Encapsulates a simple Byte writer with
/// support for Endian aware writes
pub struct ZWriter<T: ZByteWriterTrait> {
    buffer:        T,
    bytes_written: usize
}

impl<T: ZByteWriterTrait> ZWriter<T> {
    /// Write bytes from the buf into the bytestream
    /// and return how many bytes were written
    ///
    /// # Arguments
    /// - `buf`: The bytes to be written to the bytestream
    ///
    /// # Returns
    /// - `Ok(usize)` - Number of bytes written
    /// This number may be less than `buf.len()` if the length of the buffer is greater
    /// than the internal bytestream length
    ///  
    /// If you want to be sure that all bytes were written, see [`write_all`](Self::write_all)
    ///
    #[inline]
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, ZByteIoError> {
        let bytes_written = self.buffer.write_bytes(buf)?;
        self.bytes_written += bytes_written;
        Ok(bytes_written)
    }
    /// Write all bytes from `buf` into the bytestream and return
    /// and panic if not all bytes were written to the bytestream
    ///
    /// # Arguments
    /// - `buf`: The bytes to be written into the bytestream
    ///
    ///# Returns
    /// - `Ok(())`: Indicates all bytes were written into the bytestream
    /// - `Err(&static str)`: In case all the bytes could not be written
    /// to the stream
    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), ZByteIoError> {
        self.buffer.write_all_bytes(buf)?;
        self.bytes_written += buf.len();
        Ok(())
    }
    /// Create a new bytestream writer
    /// Bytes are written from the start to the end and not assumptions
    /// are made of the nature of the underlying stream
    ///
    /// # Arguments
    pub fn new(data: T) -> ZWriter<T> {
        ZWriter {
            buffer:        data,
            bytes_written: 0
        }
    }

    /// Write a single byte into the bytestream or error out
    /// if there is not enough space
    ///
    /// # Example
    /// ```
    /// use zune_core::bytestream::ZWriter;
    /// let mut buf = [0;10];
    /// let mut stream  =  ZWriter::new(&mut buf[..]);
    /// assert!(stream.write_u8_err(34).is_ok());
    /// ```
    /// No space
    /// ```
    /// use zune_core::bytestream::ZWriter;
    /// let mut no_space = [];
    /// let mut stream = ZWriter::new(&mut no_space[..]);
    /// assert!(stream.write_u8_err(32).is_err());
    /// ```
    ///
    #[inline]
    pub fn write_u8_err(&mut self, byte: u8) -> Result<(), ZByteIoError> {
        self.write_const_bytes(&[byte])
    }
    /// Write a fixed compile time known number of bytes to the sink
    ///
    /// This is provided since some implementations can optimize such writes by eliminating
    /// some redundant code.
    #[inline]
    pub fn write_const_bytes<const N: usize>(
        &mut self, byte: &[u8; N]
    ) -> Result<(), ZByteIoError> {
        self.buffer.write_const_bytes(byte)?;
        self.bytes_written += N;
        Ok(())
    }

    /// Write a single byte in the stream or don't write
    /// anything if the buffer is full and cannot support the byte read
    ///
    #[inline]
    pub fn write_u8(&mut self, byte: u8) {
        let _ = self.write_const_bytes(&[byte]);
    }
    /// Return the number of bytes written by this encoder
    ///
    /// The encoder keeps information of how many bytes were written and this method
    /// returns that value.
    ///
    /// # Returns
    ///  Number of bytes written
    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }

    /// Reserve some additional space to write.
    ///
    /// Some sinks like `Vec<u8>` allow reallocation and to prevent too much reallocation
    /// one can use this to reserve additional space to encode
    ///
    /// # Example
    ///  
    /// ```
    /// use zune_core::bytestream::ZWriter;
    /// let space_needed = 10; // Assume the image will fit into 10 bytes
    /// let mut output = Vec::new();
    /// let mut sink = ZWriter::new(&mut output);
    /// // now reserve some space
    ///sink.reserve(space_needed).unwrap();
    /// // at this point, we can assume that ZWriter allocated space for output
    /// ```
    pub fn reserve(&mut self, additional: usize) -> Result<(), ZByteIoError> {
        self.buffer.reserve_capacity(additional)
    }
    /// Consume the writer and return the inner sink
    /// we were writing to.
    ///
    /// After this, the writer can no longer be used
    pub fn inner(self) -> T {
        self.buffer
    }
    /// Return an immutable reference to the inner sink
    pub fn inner_ref(&self) -> &T {
        &self.buffer
    }
    /// Return a mutable reference to the inner sink
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.buffer
    }
}

macro_rules! write_single_type {
    ($name:tt,$name2:tt,$name3:tt,$name4:tt,$name5:tt,$name6:tt,$int_type:tt) => {
        impl<T:ZByteWriterTrait> ZWriter<T>
        {
            #[inline(always)]
            fn $name(&mut self, byte: $int_type, mode: Mode) -> Result<(), ZByteIoError>
            {

                 // get bits, depending on mode.
                 // This should be inlined and not visible in
                 // the generated binary since mode is a compile
                 // time constant.
                  let bytes = match mode
                   {
                         Mode::BE => byte.to_be_bytes(),
                         Mode::LE => byte.to_le_bytes()
                  };
                 self.write_const_bytes(&bytes)
            }
            #[inline(always)]
            fn $name2(&mut self, byte: $int_type, mode: Mode)
            {

                 // get bits, depending on mode.
                 // This should be inlined and not visible in
                 // the generated binary since mode is a compile
                 // time constant.
                  let bytes = match mode
                   {
                         Mode::BE => byte.to_be_bytes(),
                         Mode::LE => byte.to_le_bytes()
                  };
                 let _ = self.write_const_bytes(&bytes);


            }

            #[doc=concat!("Write ",stringify!($int_type)," as a big endian integer")]
            #[doc=concat!("Returning an error if the underlying buffer cannot support a ",stringify!($int_type)," write.")]
            #[inline]
            pub fn $name3(&mut self, byte: $int_type) -> Result<(), ZByteIoError>
            {
                self.$name(byte, Mode::BE)
            }

            #[doc=concat!("Write ",stringify!($int_type)," as a little endian integer")]
            #[doc=concat!("Returning an error if the underlying buffer cannot support a ",stringify!($int_type)," write.")]
            #[inline]
            pub fn $name4(&mut self, byte: $int_type) -> Result<(), ZByteIoError>
            {
                self.$name(byte, Mode::LE)
            }

            #[doc=concat!("Write ",stringify!($int_type)," as a big endian integer")]
            #[doc=concat!("Or don't write anything if the reader cannot support a ",stringify!($int_type)," write.")]
            #[inline]
            pub fn $name5(&mut self, byte: $int_type)
            {
                self.$name2(byte, Mode::BE)
            }
            #[doc=concat!("Write ",stringify!($int_type)," as a little endian integer")]
            #[doc=concat!("Or don't write anything if the reader cannot support a ",stringify!($int_type)," write.")]
            #[inline]
            pub fn $name6(&mut self, byte: $int_type)
            {
                self.$name2(byte, Mode::LE)
            }
        }
    };
}

write_single_type!(
    write_u64_inner_or_die,
    write_u64_inner_or_none,
    write_u64_be_err,
    write_u64_le_err,
    write_u64_be,
    write_u64_le,
    u64
);

write_single_type!(
    write_u32_inner_or_die,
    write_u32_inner_or_none,
    write_u32_be_err,
    write_u32_le_err,
    write_u32_be,
    write_u32_le,
    u32
);

write_single_type!(
    write_u16_inner_or_die,
    write_u16_inner_or_none,
    write_u16_be_err,
    write_u16_le_err,
    write_u16_be,
    write_u16_le,
    u16
);
