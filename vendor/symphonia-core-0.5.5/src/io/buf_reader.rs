// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp;
use std::io;

use super::{FiniteStream, ReadBytes};

#[inline(always)]
fn underrun_error<T>() -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "buffer underrun"))
}

/// A `BufReader` reads bytes from a byte buffer.
pub struct BufReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> BufReader<'a> {
    /// Instantiate a new `BufReader` with a given byte buffer.
    pub fn new(buf: &'a [u8]) -> Self {
        BufReader { buf, pos: 0 }
    }

    /// Scans up-to `scan_len` bytes from the stream until a byte pattern is matched. A reference to
    /// scanned bytes including the matched pattern are returned. If `scan_len` bytes are scanned
    /// without matching the pattern, a reference to the scanned bytes are also returned. Likewise,
    /// if the underlying buffer is exhausted before matching the pattern, remainder of the buffer
    /// is returned.
    #[inline(always)]
    pub fn scan_bytes_ref(&mut self, pattern: &[u8], scan_len: usize) -> io::Result<&'a [u8]> {
        self.scan_bytes_aligned_ref(pattern, 1, scan_len)
    }

    /// Scans up-to `scan_len` bytes from the stream until a byte pattern is matched on the
    /// specified byte alignment boundary. Operation is otherwise identical to `scan_bytes_ref`.
    pub fn scan_bytes_aligned_ref(
        &mut self,
        pattern: &[u8],
        align: usize,
        scan_len: usize,
    ) -> io::Result<&'a [u8]> {
        // The pattern must be atleast one byte.
        debug_assert!(!pattern.is_empty());

        let start = self.pos;
        let remaining = self.buf.len() - start;
        let end = start + cmp::min(remaining, scan_len);

        // If the pattern is longer than amount of bytes remaining, or the scan length is shorter
        // than the pattern, then the pattern will never match. However, since unmatched patterns
        // return the remainder of the buffer or scan_length bytes, which ever is shorter, we return
        // that here.
        if remaining < pattern.len() || scan_len < pattern.len() {
            self.pos = end;
            return Ok(&self.buf[start..end]);
        }

        let mut i = start;
        let mut j = start + pattern.len();

        while j < end {
            if &self.buf[i..j] == pattern {
                break;
            }
            j += align;
            i += align;
        }

        self.pos = cmp::min(j, self.buf.len());
        Ok(&self.buf[start..self.pos])
    }

    /// Returns a reference to the next `len` bytes in the buffer and advances the stream.
    pub fn read_buf_bytes_ref(&mut self, len: usize) -> io::Result<&'a [u8]> {
        if self.pos + len > self.buf.len() {
            return underrun_error();
        }
        self.pos += len;
        Ok(&self.buf[self.pos - len..self.pos])
    }

    /// Returns a reference to the remaining bytes in the buffer and advances the stream to the end.
    pub fn read_buf_bytes_available_ref(&mut self) -> &'a [u8] {
        let pos = self.pos;
        self.pos = self.buf.len();
        &self.buf[pos..]
    }
}

impl ReadBytes for BufReader<'_> {
    #[inline(always)]
    fn read_byte(&mut self) -> io::Result<u8> {
        if self.buf.len() - self.pos < 1 {
            return underrun_error();
        }

        self.pos += 1;
        Ok(self.buf[self.pos - 1])
    }

    #[inline(always)]
    fn read_double_bytes(&mut self) -> io::Result<[u8; 2]> {
        if self.buf.len() - self.pos < 2 {
            return underrun_error();
        }

        let mut bytes: [u8; 2] = [0u8; 2];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 2]);
        self.pos += 2;

        Ok(bytes)
    }

    #[inline(always)]
    fn read_triple_bytes(&mut self) -> io::Result<[u8; 3]> {
        if self.buf.len() - self.pos < 3 {
            return underrun_error();
        }

        let mut bytes: [u8; 3] = [0u8; 3];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 3]);
        self.pos += 3;

        Ok(bytes)
    }

    #[inline(always)]
    fn read_quad_bytes(&mut self) -> io::Result<[u8; 4]> {
        if self.buf.len() - self.pos < 4 {
            return underrun_error();
        }

        let mut bytes: [u8; 4] = [0u8; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;

        Ok(bytes)
    }

    fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(self.buf.len() - self.pos, buf.len());
        buf[..len].copy_from_slice(&self.buf[self.pos..self.pos + len]);
        self.pos += len;

        Ok(len)
    }

    fn read_buf_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let len = buf.len();

        if self.buf.len() - self.pos < len {
            return underrun_error();
        }

        buf.copy_from_slice(&self.buf[self.pos..self.pos + len]);
        self.pos += len;

        Ok(())
    }

    fn scan_bytes_aligned<'b>(
        &mut self,
        pattern: &[u8],
        align: usize,
        buf: &'b mut [u8],
    ) -> io::Result<&'b mut [u8]> {
        let scanned = self.scan_bytes_aligned_ref(pattern, align, buf.len())?;
        buf[..scanned.len()].copy_from_slice(scanned);

        Ok(&mut buf[..scanned.len()])
    }

    fn ignore_bytes(&mut self, count: u64) -> io::Result<()> {
        if self.buf.len() - self.pos < count as usize {
            return underrun_error();
        }

        self.pos += count as usize;
        Ok(())
    }

    #[inline(always)]
    fn pos(&self) -> u64 {
        self.pos as u64
    }
}

impl FiniteStream for BufReader<'_> {
    #[inline(always)]
    fn byte_len(&self) -> u64 {
        self.buf.len() as u64
    }

    #[inline(always)]
    fn bytes_read(&self) -> u64 {
        self.pos as u64
    }

    #[inline(always)]
    fn bytes_available(&self) -> u64 {
        (self.buf.len() - self.pos) as u64
    }
}
