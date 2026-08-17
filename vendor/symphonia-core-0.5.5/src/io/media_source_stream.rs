// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp;
use std::io;
use std::io::{IoSliceMut, Read, Seek};
use std::ops::Sub;

use super::SeekBuffered;
use super::{MediaSource, ReadBytes};

#[inline(always)]
fn end_of_stream_error<T>() -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "end of stream"))
}

/// `MediaSourceStreamOptions` specifies the buffering behaviour of a `MediaSourceStream`.
pub struct MediaSourceStreamOptions {
    /// The maximum buffer size. Must be a power of 2. Must be > 32kB.
    pub buffer_len: usize,
}

impl Default for MediaSourceStreamOptions {
    fn default() -> Self {
        MediaSourceStreamOptions { buffer_len: 64 * 1024 }
    }
}

/// `MediaSourceStream` is the main reader type for Symphonia.
///
/// By using type erasure and dynamic dispatch, `MediaSourceStream` wraps and hides the inner
/// reader from the consumer, allowing any typical `Read`er to be used with Symphonia in a generic
/// way, selectable at runtime.
///
/// `MediaSourceStream` is designed to provide speed and flexibility in a number of challenging I/O
/// scenarios.
///
/// First, to minimize system call and dynamic dispatch overhead on the inner reader, and to
/// amortize that overhead over many bytes, `MediaSourceStream` implements an exponentially growing
/// read-ahead buffer. The read-ahead length starts at 1kB, and doubles in length as more sequential
/// reads are performed until it reaches 32kB. Growing the read-ahead length over time reduces the
/// excess data buffered on consecutive `seek()` calls.
///
/// Second, to better support non-seekable sources, `MediaSourceStream` implements a configurable
/// length buffer cache. By default, the buffer caches allows backtracking by up-to the minimum of
/// either `buffer_len - 32kB` or the total number of bytes read since instantiation or the last
/// buffer cache invalidation. Note that regular a `seek()` will invalidate the buffer cache.
pub struct MediaSourceStream {
    /// The source reader.
    inner: Box<dyn MediaSource>,
    /// The ring buffer.
    ring: Box<[u8]>,
    /// The ring buffer's wrap-around mask.
    ring_mask: usize,
    /// The read position.
    read_pos: usize,
    /// The write position.
    write_pos: usize,
    /// The current block size for a new read.
    read_block_len: usize,
    /// Absolute position of the inner stream.
    abs_pos: u64,
    /// Relative position of the inner stream from the last seek or 0. This is a count of bytes
    /// read from the inner reader since instantiation or the last seek.
    rel_pos: u64,
}

impl MediaSourceStream {
    const MIN_BLOCK_LEN: usize = 1 * 1024;
    const MAX_BLOCK_LEN: usize = 32 * 1024;

    pub fn new(source: Box<dyn MediaSource>, options: MediaSourceStreamOptions) -> Self {
        // The buffer length must be a power of 2, and > the maximum read block length.
        assert!(options.buffer_len.count_ones() == 1);
        assert!(options.buffer_len > Self::MAX_BLOCK_LEN);

        MediaSourceStream {
            inner: source,
            ring: vec![0; options.buffer_len].into_boxed_slice(),
            ring_mask: options.buffer_len - 1,
            read_pos: 0,
            write_pos: 0,
            read_block_len: Self::MIN_BLOCK_LEN,
            abs_pos: 0,
            rel_pos: 0,
        }
    }

    /// Returns if the buffer has been exhausted This is a marginally more efficient way of checking
    /// if `unread_buffer_len() == 0`.
    #[inline(always)]
    fn is_buffer_exhausted(&self) -> bool {
        self.read_pos == self.write_pos
    }

    /// If the buffer has been exhausted, fetch a new block of data to replenish the buffer.
    fn fetch(&mut self) -> io::Result<()> {
        // Only fetch when the ring buffer is empty.
        if self.is_buffer_exhausted() {
            // Split the vector at the write position to get slices of the two contiguous regions of
            // the ring buffer.
            let (vec1, vec0) = self.ring.split_at_mut(self.write_pos);

            // If the first contiguous region of the ring buffer starting from the write position
            // has sufficient space to service the entire read do a simple read into that region's
            // slice.
            let actual_read_len = if vec0.len() >= self.read_block_len {
                self.inner.read(&mut vec0[..self.read_block_len])?
            }
            else {
                // Otherwise, perform a vectored read into the two contiguous region slices.
                let rem = self.read_block_len - vec0.len();

                let ring_vectors = &mut [IoSliceMut::new(vec0), IoSliceMut::new(&mut vec1[..rem])];

                self.inner.read_vectored(ring_vectors)?
            };

            // Increment the write position, taking into account wrap-around.
            self.write_pos = (self.write_pos + actual_read_len) & self.ring_mask;

            // Update the stream position accounting.
            self.abs_pos += actual_read_len as u64;
            self.rel_pos += actual_read_len as u64;

            // Grow the read block length exponentially to reduce the overhead of buffering on
            // consecutive seeks.
            self.read_block_len = cmp::min(self.read_block_len << 1, Self::MAX_BLOCK_LEN);
        }

        Ok(())
    }

    /// If the buffer has been exhausted, fetch a new block of data to replenish the buffer. If
    /// no more data could be fetched, return an end-of-stream error.
    fn fetch_or_eof(&mut self) -> io::Result<()> {
        self.fetch()?;

        if self.is_buffer_exhausted() {
            return end_of_stream_error();
        }

        Ok(())
    }

    /// Advances the read position by `len` bytes, taking into account wrap-around.
    #[inline(always)]
    fn consume(&mut self, len: usize) {
        self.read_pos = (self.read_pos + len) & self.ring_mask;
    }

    /// Gets the largest contiguous slice of buffered data starting from the read position.
    #[inline(always)]
    fn continguous_buf(&self) -> &[u8] {
        if self.write_pos >= self.read_pos {
            &self.ring[self.read_pos..self.write_pos]
        }
        else {
            &self.ring[self.read_pos..]
        }
    }

    /// Resets the read-ahead buffer, and sets the absolute stream position to `pos`.
    fn reset(&mut self, pos: u64) {
        self.read_pos = 0;
        self.write_pos = 0;
        self.read_block_len = Self::MIN_BLOCK_LEN;
        self.abs_pos = pos;
        self.rel_pos = 0;
    }
}

impl MediaSource for MediaSourceStream {
    #[inline]
    fn is_seekable(&self) -> bool {
        self.inner.is_seekable()
    }

    #[inline]
    fn byte_len(&self) -> Option<u64> {
        self.inner.byte_len()
    }
}

impl io::Read for MediaSourceStream {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let read_len = buf.len();

        while !buf.is_empty() {
            // Refill the the buffer cache if required.
            self.fetch()?;

            // Consume bytes from the readable portion of the buffer cache and copy them into the
            // remaining portion of the caller's buffer.
            match self.continguous_buf().read(buf) {
                Ok(0) => break,
                Ok(count) => {
                    buf = &mut buf[count..];
                    self.consume(count);
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }

        // The actual amount read is the original length of the caller's buffer minus the amount of
        // that buffer that is remaining.
        Ok(read_len - buf.len())
    }
}

impl io::Seek for MediaSourceStream {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        // The current position of the underlying reader is ahead of the current position of the
        // MediaSourceStream by how ever many bytes have not been read from the read-ahead buffer
        // yet. When seeking from the current position adjust the position delta to offset that
        // difference.
        let pos = match pos {
            io::SeekFrom::Current(0) => return Ok(self.pos()),
            io::SeekFrom::Current(delta_pos) => {
                let delta = delta_pos - self.unread_buffer_len() as i64;
                self.inner.seek(io::SeekFrom::Current(delta))
            }
            _ => self.inner.seek(pos),
        }?;

        self.reset(pos);

        Ok(pos)
    }
}

impl ReadBytes for MediaSourceStream {
    #[inline(always)]
    fn read_byte(&mut self) -> io::Result<u8> {
        // This function, read_byte, is inlined for performance. To reduce code bloat, place the
        // read-ahead buffer replenishment in a seperate function. Call overhead will be negligible
        // compared to the actual underlying read.
        if self.is_buffer_exhausted() {
            self.fetch_or_eof()?;
        }

        let value = self.ring[self.read_pos];
        self.consume(1);

        Ok(value)
    }

    fn read_double_bytes(&mut self) -> io::Result<[u8; 2]> {
        let mut bytes = [0; 2];

        let buf = self.continguous_buf();

        if buf.len() >= 2 {
            bytes.copy_from_slice(&buf[..2]);
            self.consume(2);
        }
        else {
            for byte in bytes.iter_mut() {
                *byte = self.read_byte()?;
            }
        };

        Ok(bytes)
    }

    fn read_triple_bytes(&mut self) -> io::Result<[u8; 3]> {
        let mut bytes = [0; 3];

        let buf = self.continguous_buf();

        if buf.len() >= 3 {
            bytes.copy_from_slice(&buf[..3]);
            self.consume(3);
        }
        else {
            for byte in bytes.iter_mut() {
                *byte = self.read_byte()?;
            }
        };
        Ok(bytes)
    }

    fn read_quad_bytes(&mut self) -> io::Result<[u8; 4]> {
        let mut bytes = [0; 4];

        let buf = self.continguous_buf();

        if buf.len() >= 4 {
            bytes.copy_from_slice(&buf[..4]);
            self.consume(4);
        }
        else {
            for byte in bytes.iter_mut() {
                *byte = self.read_byte()?;
            }
        };
        Ok(bytes)
    }

    fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Implemented via io::Read trait.
        let read = self.read(buf)?;

        // Unlike the io::Read trait, ByteStream returns an end-of-stream error when no more data
        // can be read. If a non-zero read is requested, and 0 bytes are read, return an
        // end-of-stream error.
        if !buf.is_empty() && read == 0 {
            end_of_stream_error()
        }
        else {
            Ok(read)
        }
    }

    fn read_buf_exact(&mut self, mut buf: &mut [u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(count) => {
                    buf = &mut buf[count..];
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty() {
            end_of_stream_error()
        }
        else {
            Ok(())
        }
    }

    fn scan_bytes_aligned<'a>(
        &mut self,
        _: &[u8],
        _: usize,
        _: &'a mut [u8],
    ) -> io::Result<&'a mut [u8]> {
        // Intentionally left unimplemented.
        unimplemented!();
    }

    fn ignore_bytes(&mut self, mut count: u64) -> io::Result<()> {
        // If the stream is seekable and the number of bytes to ignore is large, perform a seek
        // first. Note that ignored bytes are rewindable. Therefore, ensure the ring-buffer is
        // full after the seek just like if bytes were ignored by consuming them instead.
        let ring_len = self.ring.len() as u64;

        // Only apply the optimization if seeking 2x or more than the ring-buffer size.
        while count >= 2 * ring_len && self.is_seekable() {
            let delta = count.clamp(0, i64::MAX as u64).sub(ring_len);
            self.seek(io::SeekFrom::Current(delta as i64))?;
            count -= delta;
        }

        // Ignore the remaining bytes be consuming samples from the ring-buffer.
        while count > 0 {
            self.fetch_or_eof()?;
            let discard_count = cmp::min(self.unread_buffer_len() as u64, count);
            self.consume(discard_count as usize);
            count -= discard_count;
        }
        Ok(())
    }

    fn pos(&self) -> u64 {
        self.abs_pos - self.unread_buffer_len() as u64
    }
}

impl SeekBuffered for MediaSourceStream {
    fn ensure_seekback_buffer(&mut self, len: usize) {
        let ring_len = self.ring.len();

        // A fetch can overwrite a maximum of MAX_BLOCK_LEN bytes in the ring. Therefore, for there
        // to always be `len` bytes available for seekback, the ring must be len + MAX_BLOCK_LEN in
        // length. Round-up to the next power-of-2 as that is an invariant of the ring.
        let new_ring_len = (Self::MAX_BLOCK_LEN + len).next_power_of_two();

        // Only grow the ring if necessary.
        if ring_len < new_ring_len {
            // Allocate a new ring.
            let mut new_ring = vec![0; new_ring_len].into_boxed_slice();

            // Get the readable regions of the current ring.
            let (vec0, vec1) = if self.write_pos >= self.read_pos {
                (&self.ring[self.read_pos..self.write_pos], None)
            }
            else {
                (&self.ring[self.read_pos..], Some(&self.ring[..self.write_pos]))
            };

            // Copy contents from the old ring into new ring.
            let vec0_len = vec0.len();
            new_ring[..vec0_len].copy_from_slice(vec0);

            self.write_pos = if let Some(vec1) = vec1 {
                let total_len = vec0_len + vec1.len();
                new_ring[vec0_len..total_len].copy_from_slice(vec1);
                total_len
            }
            else {
                vec0_len
            };

            self.ring = new_ring;
            self.ring_mask = new_ring_len - 1;
            self.read_pos = 0;
        }
    }

    fn unread_buffer_len(&self) -> usize {
        if self.write_pos >= self.read_pos {
            self.write_pos - self.read_pos
        }
        else {
            self.write_pos + (self.ring.len() - self.read_pos)
        }
    }

    fn read_buffer_len(&self) -> usize {
        let unread_len = self.unread_buffer_len();

        cmp::min(self.ring.len(), self.rel_pos as usize) - unread_len
    }

    fn seek_buffered(&mut self, pos: u64) -> u64 {
        let old_pos = self.pos();

        // Forward seek.
        let delta = if pos > old_pos {
            assert!(pos - old_pos < isize::MAX as u64);
            (pos - old_pos) as isize
        }
        else if pos < old_pos {
            // Backward seek.
            assert!(old_pos - pos < isize::MAX as u64);
            -((old_pos - pos) as isize)
        }
        else {
            0
        };

        self.seek_buffered_rel(delta)
    }

    fn seek_buffered_rel(&mut self, delta: isize) -> u64 {
        if delta < 0 {
            let abs_delta = cmp::min((-delta) as usize, self.read_buffer_len());
            self.read_pos = (self.read_pos + self.ring.len() - abs_delta) & self.ring_mask;
        }
        else if delta > 0 {
            let abs_delta = cmp::min(delta as usize, self.unread_buffer_len());
            self.read_pos = (self.read_pos + abs_delta) & self.ring_mask;
        }

        self.pos()
    }
}

#[cfg(test)]
mod tests {
    use super::{MediaSourceStream, ReadBytes, SeekBuffered};
    use std::io::{Cursor, Read};

    /// Generate a random vector of bytes of the specified length using a PRNG.
    fn generate_random_bytes(len: usize) -> Box<[u8]> {
        let mut lcg: u32 = 0xec57c4bf;

        let mut bytes = vec![0; len];

        for quad in bytes.chunks_mut(4) {
            lcg = lcg.wrapping_mul(1664525).wrapping_add(1013904223);
            for (src, dest) in quad.iter_mut().zip(&lcg.to_le_bytes()) {
                *src = *dest;
            }
        }

        bytes.into_boxed_slice()
    }

    #[test]
    fn verify_mss_read() {
        let data = generate_random_bytes(5 * 96 * 1024);

        let ms = Cursor::new(data.clone());
        let mut mss = MediaSourceStream::new(Box::new(ms), Default::default());

        // Each of the following scenarios should exercise read-ahead and wrap-around the stream's
        // internal ring buffer. This means reading > 64kB for each scenario. Between each scenario,
        // ignore an odd number of bytes.
        let mut buf = &data[..];

        // 96k single byte reads.
        for byte in &buf[..96 * 1024] {
            assert_eq!(*byte, mss.read_byte().unwrap());
        }

        mss.ignore_bytes(11).unwrap();

        buf = &buf[11 + (96 * 1024)..];

        // 48k two byte reads.
        for bytes in buf[..2 * 48 * 1024].chunks_exact(2) {
            assert_eq!(bytes, &mss.read_double_bytes().unwrap());
        }

        mss.ignore_bytes(33).unwrap();

        buf = &buf[33 + (2 * 48 * 1024)..];

        // 32k three byte reads.
        for bytes in buf[..3 * 32 * 1024].chunks_exact(3) {
            assert_eq!(bytes, &mss.read_triple_bytes().unwrap());
        }

        mss.ignore_bytes(55).unwrap();

        buf = &buf[55 + (3 * 32 * 1024)..];

        // 24k four byte reads.
        for bytes in buf[..4 * 24 * 1024].chunks_exact(4) {
            assert_eq!(bytes, &mss.read_quad_bytes().unwrap());
        }
    }

    #[test]
    fn verify_mss_read_to_end() {
        let data = generate_random_bytes(5 * 96 * 1024);

        let ms = Cursor::new(data.clone());
        let mut mss = MediaSourceStream::new(Box::new(ms), Default::default());
        let mut output: Vec<u8> = Vec::new();
        assert_eq!(mss.read_to_end(&mut output).unwrap(), data.len());
        assert_eq!(output.into_boxed_slice(), data);
    }

    #[test]
    fn verify_mss_seek_buffered() {
        let data = generate_random_bytes(1024 * 1024);

        let ms = Cursor::new(data);
        let mut mss = MediaSourceStream::new(Box::new(ms), Default::default());

        assert_eq!(mss.read_buffer_len(), 0);
        assert_eq!(mss.unread_buffer_len(), 0);

        mss.ignore_bytes(5122).unwrap();

        assert_eq!(5122, mss.pos());
        assert_eq!(mss.read_buffer_len(), 5122);

        let upper = mss.read_byte().unwrap();

        // Seek backwards.
        assert_eq!(mss.seek_buffered_rel(-1000), 4123);
        assert_eq!(mss.pos(), 4123);
        assert_eq!(mss.read_buffer_len(), 4123);

        // Seek forwards.
        assert_eq!(mss.seek_buffered_rel(999), 5122);
        assert_eq!(mss.pos(), 5122);
        assert_eq!(mss.read_buffer_len(), 5122);

        assert_eq!(upper, mss.read_byte().unwrap());
    }

    #[test]
    fn verify_reading_be() {
        let data = generate_random_bytes(1024 * 1024);

        let ms = Cursor::new(data);
        let mut mss = MediaSourceStream::new(Box::new(ms), Default::default());

        // For slightly cleaner floats
        mss.ignore_bytes(2).unwrap();

        assert_eq!(mss.read_be_f32().unwrap(), -72818055000000000000000000000.0);
        assert_eq!(mss.read_be_f64().unwrap(), -0.000000000000011582640453292664);

        assert_eq!(mss.read_be_u16().unwrap(), 32624);
        assert_eq!(mss.read_be_u24().unwrap(), 6739677);
        assert_eq!(mss.read_be_u32().unwrap(), 1569552917);
        assert_eq!(mss.read_be_u64().unwrap(), 6091217585348000864);
    }

    #[test]
    fn verify_reading_le() {
        let data = generate_random_bytes(1024 * 1024);

        let ms = Cursor::new(data);
        let mut mss = MediaSourceStream::new(Box::new(ms), Default::default());

        mss.ignore_bytes(1024).unwrap();

        assert_eq!(mss.read_f32().unwrap(), -0.00000000000000000000000000048426285);
        assert_eq!(mss.read_f64().unwrap(), -6444325820119113.0);

        assert_eq!(mss.read_u16().unwrap(), 36195);
        assert_eq!(mss.read_u24().unwrap(), 6710386);
        assert_eq!(mss.read_u32().unwrap(), 2378776723);
        assert_eq!(mss.read_u64().unwrap(), 5170196279331153683);
    }
}
