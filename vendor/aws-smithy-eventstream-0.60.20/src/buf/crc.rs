/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Utilities for calculating CRC-32 while reading from a [`Buf`] or writing to a [`BufMut`].

use bytes::buf::UninitSlice;
use bytes::{Buf, BufMut};
use crc32fast::Hasher;

/// Implementation of [`Buf`] that calculates a CRC-32 checksum of the data
/// being read from an underlying `Buf` instance.
pub(crate) struct CrcBuf<'a, B>
where
    B: Buf,
{
    buffer: &'a mut B,
    crc: Hasher,
}

impl<'a, B> CrcBuf<'a, B>
where
    B: Buf,
{
    /// Creates a new `CrcBuf` by wrapping the given `buffer`.
    pub(crate) fn new(buffer: &'a mut B) -> Self {
        CrcBuf {
            buffer,
            crc: Hasher::new(),
        }
    }

    /// Consumes the `CrcBuf` and returns the calculated checksum.
    pub(crate) fn into_crc(self) -> u32 {
        self.crc.finalize()
    }
}

impl<B> Buf for CrcBuf<'_, B>
where
    B: Buf,
{
    fn remaining(&self) -> usize {
        self.buffer.remaining()
    }

    fn chunk(&self) -> &[u8] {
        self.buffer.chunk()
    }

    fn advance(&mut self, cnt: usize) {
        let chunk = self.buffer.chunk();
        self.crc.update(&chunk[0..cnt]);
        self.buffer.advance(cnt);
    }
}

#[cfg(test)]
mod crc_buf_tests {
    use super::CrcBuf;
    use bytes::Buf;

    #[test]
    fn crc_no_data_read() {
        let mut data: &[u8] = &[];
        let buf = CrcBuf::new(&mut data);
        assert_eq!(0, buf.into_crc());
    }

    #[test]
    fn crc_data_read() {
        let mut data: &[u8] = &[0, 0, 0, 5, 0, 10u8];
        let mut buf = CrcBuf::new(&mut data);
        assert_eq!(5, buf.get_i32());
        assert_eq!(0x512E2B93, buf.into_crc());

        let mut data: &[u8] = &[0, 0, 0, 5, 0, 10u8];
        let mut buf = CrcBuf::new(&mut data);
        assert_eq!(5, buf.get_i32());
        assert_eq!(10, buf.get_i16());
        assert_eq!(0x57DC8A56, buf.into_crc());
    }

    #[test]
    fn chunk_called_multiple_times_before_advance() {
        let mut data: &[u8] = &[0, 0, 0, 5, 0, 10u8];
        let mut buf = CrcBuf::new(&mut data);
        for _ in 0..3 {
            buf.chunk();
        }
        buf.advance(4);
        assert_eq!(10, buf.get_i16());
        assert_eq!(0x57DC8A56, buf.into_crc());
    }
}

/// Implementation of [`BufMut`] that calculates a CRC-32 checksum of the data
/// being written to an underlying `Buf` instance, with a function to then write
/// the calculated CRC-32 to the buffer.
pub(crate) struct CrcBufMut<'a> {
    buffer: &'a mut dyn BufMut,
    crc: Hasher,
}

impl<'a> CrcBufMut<'a> {
    /// Creates a new `CrcBufMut` by wrapping the given `buffer`.
    pub(crate) fn new(buffer: &'a mut dyn BufMut) -> Self {
        CrcBufMut {
            buffer,
            crc: Hasher::new(),
        }
    }

    /// Puts the calculated CRC-32 to the buffer as a Big Endian 32-bit integer.
    /// This can be called multiple times, and each successive call will include
    /// the previously written checksum in its new checksum.
    pub(crate) fn put_crc(&mut self) {
        self.put_u32(self.crc.clone().finalize());
    }
}

unsafe impl BufMut for CrcBufMut<'_> {
    fn remaining_mut(&self) -> usize {
        self.buffer.remaining_mut()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        // Safety: There is no guarantee the bytes being advanced are initialized, which is why
        // this trait method is unsafe. The best we can do is assume they have been initialized
        // by the caller before `advance_mut` was called.
        let written = std::slice::from_raw_parts_mut(self.chunk_mut().as_mut_ptr(), cnt);
        self.crc.update(written);
        self.buffer.advance_mut(cnt);
    }

    fn chunk_mut(&mut self) -> &mut UninitSlice {
        self.buffer.chunk_mut()
    }
}

#[cfg(test)]
mod crc_buf_mut_tests {
    use super::CrcBufMut;
    use bytes::BufMut;

    #[test]
    fn crc_no_bytes_written() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut crc_buf = CrcBufMut::new(&mut buffer);
        crc_buf.put_crc();
        crc_buf.put_crc();
        assert_eq!(vec![0, 0, 0, 0u8, 0x21, 0x44, 0xDF, 0x1C], buffer);
    }

    #[test]
    fn crc_bytes_written() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut crc_buf = CrcBufMut::new(&mut buffer);
        crc_buf.put_u32(5);
        crc_buf.put_crc();
        assert_eq!(vec![0, 0, 0, 5, 0x51, 0x2E, 0x2B, 0x93], buffer);
    }
}
