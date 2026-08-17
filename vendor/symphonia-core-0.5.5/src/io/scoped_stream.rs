// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp;
use std::io;

use super::{FiniteStream, ReadBytes, SeekBuffered};

#[inline(always)]
fn out_of_bounds_error<T>() -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "out of bounds"))
}

/// A `ScopedStream` restricts the number of bytes that may be read to an upper limit.
pub struct ScopedStream<B: ReadBytes> {
    inner: B,
    start: u64,
    len: u64,
    read: u64,
}

impl<B: ReadBytes> ScopedStream<B> {
    /// Instantiates a new `ScopedStream` with an upper limit on the number of bytes that can be
    /// read from the inner source.
    pub fn new(inner: B, len: u64) -> Self {
        ScopedStream { start: inner.pos(), inner, len, read: 0 }
    }

    /// Returns an immutable reference to the inner stream.
    pub fn inner(&self) -> &B {
        &self.inner
    }

    /// Returns a mutable reference to the inner stream.
    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.inner
    }

    /// Ignores the remainder of the `ScopedStream`.
    pub fn ignore(&mut self) -> io::Result<()> {
        self.inner.ignore_bytes(self.len - self.read)
    }

    /// Convert the `ScopedStream` to the inner stream.
    pub fn into_inner(self) -> B {
        self.inner
    }
}

impl<B: ReadBytes> FiniteStream for ScopedStream<B> {
    /// Returns the length of the the `ScopedStream`.
    fn byte_len(&self) -> u64 {
        self.len
    }

    /// Returns the number of bytes read.
    fn bytes_read(&self) -> u64 {
        self.read
    }

    /// Returns the number of bytes available to read.
    fn bytes_available(&self) -> u64 {
        self.len - self.read
    }
}

impl<B: ReadBytes> ReadBytes for ScopedStream<B> {
    #[inline(always)]
    fn read_byte(&mut self) -> io::Result<u8> {
        if self.len - self.read < 1 {
            return out_of_bounds_error();
        }

        self.read += 1;
        self.inner.read_byte()
    }

    #[inline(always)]
    fn read_double_bytes(&mut self) -> io::Result<[u8; 2]> {
        if self.len - self.read < 2 {
            return out_of_bounds_error();
        }

        self.read += 2;
        self.inner.read_double_bytes()
    }

    #[inline(always)]
    fn read_triple_bytes(&mut self) -> io::Result<[u8; 3]> {
        if self.len - self.read < 3 {
            return out_of_bounds_error();
        }

        self.read += 3;
        self.inner.read_triple_bytes()
    }

    #[inline(always)]
    fn read_quad_bytes(&mut self) -> io::Result<[u8; 4]> {
        if self.len - self.read < 4 {
            return out_of_bounds_error();
        }

        self.read += 4;
        self.inner.read_quad_bytes()
    }

    fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Limit read_buf() to the remainder of the scoped bytes if buf has a greater length.
        let scoped_len = cmp::min(self.len - self.read, buf.len() as u64) as usize;
        let result = self.inner.read_buf(&mut buf[0..scoped_len])?;
        self.read += result as u64;
        Ok(result)
    }

    fn read_buf_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        if self.len - self.read < buf.len() as u64 {
            return out_of_bounds_error();
        }

        self.read += buf.len() as u64;
        self.inner.read_buf_exact(buf)
    }

    #[inline(always)]
    fn scan_bytes_aligned<'a>(
        &mut self,
        pattern: &[u8],
        align: usize,
        buf: &'a mut [u8],
    ) -> io::Result<&'a mut [u8]> {
        if self.len - self.read < buf.len() as u64 {
            return out_of_bounds_error();
        }

        let result = self.inner.scan_bytes_aligned(pattern, align, buf)?;
        self.read += result.len() as u64;
        Ok(result)
    }

    #[inline(always)]
    fn ignore_bytes(&mut self, count: u64) -> io::Result<()> {
        if self.len - self.read < count {
            return out_of_bounds_error();
        }

        self.read += count;
        self.inner.ignore_bytes(count)
    }

    #[inline(always)]
    fn pos(&self) -> u64 {
        self.inner.pos()
    }
}

impl<B: ReadBytes + SeekBuffered> SeekBuffered for ScopedStream<B> {
    #[inline(always)]
    fn ensure_seekback_buffer(&mut self, len: usize) {
        self.inner.ensure_seekback_buffer(len)
    }

    #[inline(always)]
    fn unread_buffer_len(&self) -> usize {
        self.inner.unread_buffer_len().min((self.len - self.read) as usize)
    }

    #[inline(always)]
    fn read_buffer_len(&self) -> usize {
        self.inner.read_buffer_len().min(self.read as usize)
    }

    #[inline(always)]
    fn seek_buffered(&mut self, pos: u64) -> u64 {
        // Clamp the seekable position to within the bounds of the ScopedStream.
        self.inner.seek_buffered(pos.clamp(self.start, self.start + self.len))
    }

    #[inline(always)]
    fn seek_buffered_rel(&mut self, delta: isize) -> u64 {
        // Clamp the delta value such that the absolute position after the buffered seek will be
        // within the bounds of the ScopedStream.
        let max_back = self.read.min(isize::MAX as u64) as isize;
        let max_forward = (self.len - self.read).min(isize::MAX as u64) as isize;
        self.inner.seek_buffered_rel(delta.clamp(-max_back, max_forward))
    }
}
