use std::cmp;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::mem;
use std::mem::MaybeUninit;

#[cfg(feature = "bytes")]
use bytes::buf::UninitSlice;
#[cfg(feature = "bytes")]
use bytes::BufMut;
#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use bytes::BytesMut;

use crate::coded_input_stream::buf_read_or_reader::BufReadOrReader;
use crate::coded_input_stream::input_buf::InputBuf;
use crate::coded_input_stream::input_source::InputSource;
use crate::coded_input_stream::READ_RAW_BYTES_MAX_ALLOC;
use crate::error::ProtobufError;
use crate::error::WireError;

// If an input stream is constructed with a `Read`, we create a
// `BufReader` with an internal buffer of this size.
const INPUT_STREAM_BUFFER_SIZE: usize = 4096;

const NO_LIMIT: u64 = u64::MAX;

/// Dangerous implementation of `BufRead`.
///
/// Unsafe wrapper around BufRead which assumes that `BufRead` buf is
/// not moved when `BufRead` is moved.
///
/// This assumption is generally incorrect, however, in practice
/// `BufReadIter` is created either from `BufRead` reference (which
/// cannot  be moved, because it is locked by `CodedInputStream`) or from
/// `BufReader` which does not move its buffer (we know that from
/// inspecting rust standard library).
///
/// It is important for `CodedInputStream` performance that small reads
/// (e. g. 4 bytes reads) do not involve virtual calls or switches.
/// This is achievable with `BufReadIter`.
#[derive(Debug)]
pub(crate) struct BufReadIter<'a> {
    input_source: InputSource<'a>,
    buf: InputBuf<'a>,
    pos_of_buf_start: u64,
    limit: u64,
}

impl<'a> Drop for BufReadIter<'a> {
    fn drop(&mut self) {
        match self.input_source {
            InputSource::Read(ref mut buf_read) => buf_read.consume(self.buf.pos_within_buf()),
            _ => {}
        }
    }
}

impl<'a> BufReadIter<'a> {
    pub(crate) fn from_read(read: &'a mut dyn Read) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Read(BufReadOrReader::BufReader(BufReader::with_capacity(
                INPUT_STREAM_BUFFER_SIZE,
                read,
            ))),
            buf: InputBuf::empty(),
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    pub(crate) fn from_buf_read(buf_read: &'a mut dyn BufRead) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Read(BufReadOrReader::BufRead(buf_read)),
            buf: InputBuf::empty(),
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    pub(crate) fn from_byte_slice(bytes: &'a [u8]) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Slice(bytes),
            buf: InputBuf::from_bytes(bytes),
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    #[cfg(feature = "bytes")]
    pub(crate) fn from_bytes(bytes: &'a Bytes) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Bytes(bytes),
            buf: InputBuf::from_bytes(&bytes),
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    #[inline]
    fn assertions(&self) {
        debug_assert!(self.pos() <= self.limit);
        self.buf.assertions();
    }

    #[inline(always)]
    pub(crate) fn pos(&self) -> u64 {
        self.pos_of_buf_start + self.buf.pos_within_buf() as u64
    }

    /// Recompute `limit_within_buf` after update of `limit`
    #[inline]
    fn update_limit_within_buf(&mut self) {
        assert!(self.limit >= self.pos_of_buf_start);
        self.buf.update_limit(self.limit - self.pos_of_buf_start);
        self.assertions();
    }

    pub(crate) fn push_limit(&mut self, limit: u64) -> crate::Result<u64> {
        let new_limit = match self.pos().checked_add(limit) {
            Some(new_limit) => new_limit,
            None => return Err(ProtobufError::WireError(WireError::LimitOverflow).into()),
        };

        if new_limit > self.limit {
            return Err(ProtobufError::WireError(WireError::LimitIncrease).into());
        }

        let prev_limit = mem::replace(&mut self.limit, new_limit);

        self.update_limit_within_buf();

        Ok(prev_limit)
    }

    #[inline]
    pub(crate) fn pop_limit(&mut self, limit: u64) {
        assert!(limit >= self.limit);

        self.limit = limit;

        self.update_limit_within_buf();
    }

    #[inline(always)]
    pub(crate) fn remaining_in_buf(&self) -> &[u8] {
        self.buf.remaining_in_buf()
    }

    #[inline]
    pub(crate) fn consume(&mut self, amt: usize) {
        self.buf.consume(amt);
    }

    #[inline(always)]
    pub(crate) fn remaining_in_buf_len(&self) -> usize {
        self.remaining_in_buf().len()
    }

    #[inline(always)]
    pub(crate) fn bytes_until_limit(&self) -> u64 {
        if self.limit == NO_LIMIT {
            NO_LIMIT
        } else {
            self.limit - self.pos()
        }
    }

    #[inline(always)]
    pub(crate) fn eof(&mut self) -> crate::Result<bool> {
        if self.remaining_in_buf_len() != 0 {
            Ok(false)
        } else {
            Ok(self.fill_buf()?.is_empty())
        }
    }

    fn read_byte_slow(&mut self) -> crate::Result<u8> {
        self.fill_buf_slow()?;

        if let Some(b) = self.buf.read_byte() {
            return Ok(b);
        }

        Err(WireError::UnexpectedEof.into())
    }

    #[inline(always)]
    pub(crate) fn read_byte(&mut self) -> crate::Result<u8> {
        if let Some(b) = self.buf.read_byte() {
            return Ok(b);
        }

        self.read_byte_slow()
    }

    #[cfg(feature = "bytes")]
    pub(crate) fn read_exact_bytes(&mut self, len: usize) -> crate::Result<Bytes> {
        if let InputSource::Bytes(bytes) = self.input_source {
            if len > self.remaining_in_buf_len() {
                return Err(ProtobufError::WireError(WireError::UnexpectedEof).into());
            }
            let end = self.buf.pos_within_buf() + len;

            let r = bytes.slice(self.buf.pos_within_buf()..end);
            self.buf.consume(len);
            Ok(r)
        } else {
            if len >= READ_RAW_BYTES_MAX_ALLOC {
                // We cannot trust `len` because protobuf message could be malformed.
                // Reading should not result in OOM when allocating a buffer.
                let mut v = Vec::new();
                self.read_exact_to_vec(len, &mut v)?;
                Ok(Bytes::from(v))
            } else {
                let mut r = BytesMut::with_capacity(len);
                unsafe {
                    let buf = Self::uninit_slice_as_mut_slice(&mut r.chunk_mut()[..len]);
                    self.read_exact(buf)?;
                    r.advance_mut(len);
                }
                Ok(r.freeze())
            }
        }
    }

    #[cfg(feature = "bytes")]
    unsafe fn uninit_slice_as_mut_slice(slice: &mut UninitSlice) -> &mut [MaybeUninit<u8>] {
        use std::slice;
        slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut MaybeUninit<u8>, slice.len())
    }

    /// Returns 0 when EOF or limit reached.
    pub(crate) fn read(&mut self, buf: &mut [u8]) -> crate::Result<usize> {
        let rem = self.fill_buf()?;

        let len = cmp::min(rem.len(), buf.len());
        buf[..len].copy_from_slice(&rem[..len]);
        self.buf.consume(len);
        Ok(len)
    }

    fn consume_buf(&mut self) -> crate::Result<()> {
        match &mut self.input_source {
            InputSource::Read(read) => {
                read.consume(self.buf.pos_within_buf());
                self.pos_of_buf_start += self.buf.pos_within_buf() as u64;
                self.buf = InputBuf::empty();
                self.assertions();
                Ok(())
            }
            _ => Err(WireError::UnexpectedEof.into()),
        }
    }

    /// Read at most `max` bytes.
    ///
    /// Returns 0 when EOF or limit reached.
    fn read_to_vec(&mut self, vec: &mut Vec<u8>, max: usize) -> crate::Result<usize> {
        let rem = self.fill_buf()?;

        let len = cmp::min(rem.len(), max);
        vec.extend_from_slice(&rem[..len]);
        self.buf.consume(len);
        Ok(len)
    }

    fn read_exact_slow(&mut self, buf: &mut [MaybeUninit<u8>]) -> crate::Result<()> {
        if self.bytes_until_limit() < buf.len() as u64 {
            return Err(ProtobufError::WireError(WireError::UnexpectedEof).into());
        }

        self.consume_buf()?;

        match &mut self.input_source {
            InputSource::Read(buf_read) => {
                buf_read.read_exact_uninit(buf)?;
                self.pos_of_buf_start += buf.len() as u64;
                self.assertions();
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn read_exact(&mut self, buf: &mut [MaybeUninit<u8>]) -> crate::Result<()> {
        if self.remaining_in_buf_len() >= buf.len() {
            self.buf.read_bytes(buf);
            return Ok(());
        }

        self.read_exact_slow(buf)
    }

    /// Read raw bytes into the supplied vector.  The vector will be resized as needed and
    /// overwritten.
    pub(crate) fn read_exact_to_vec(
        &mut self,
        count: usize,
        target: &mut Vec<u8>,
    ) -> crate::Result<()> {
        // TODO: also do some limits when reading from unlimited source
        if count as u64 > self.bytes_until_limit() {
            return Err(ProtobufError::WireError(WireError::TruncatedMessage).into());
        }

        target.clear();

        if count >= READ_RAW_BYTES_MAX_ALLOC && count > target.capacity() {
            // avoid calling `reserve` on buf with very large buffer: could be a malformed message

            target.reserve(READ_RAW_BYTES_MAX_ALLOC);

            while target.len() < count {
                if count - target.len() <= target.len() {
                    target.reserve_exact(count - target.len());
                } else {
                    target.reserve(1);
                }

                let max = cmp::min(target.capacity() - target.len(), count - target.len());
                let read = self.read_to_vec(target, max)?;
                if read == 0 {
                    return Err(ProtobufError::WireError(WireError::TruncatedMessage).into());
                }
            }
        } else {
            target.reserve_exact(count);

            unsafe {
                self.read_exact(&mut target.spare_capacity_mut()[..count])?;
                target.set_len(count);
            }
        }

        debug_assert_eq!(count, target.len());

        Ok(())
    }

    pub(crate) fn skip_bytes(&mut self, count: u32) -> crate::Result<()> {
        if count as usize <= self.remaining_in_buf_len() {
            self.buf.consume(count as usize);
            return Ok(());
        }

        if count as u64 > self.bytes_until_limit() {
            return Err(WireError::TruncatedMessage.into());
        }

        self.consume_buf()?;

        match &mut self.input_source {
            InputSource::Read(read) => {
                read.skip_bytes(count as usize)?;
                self.pos_of_buf_start += count as u64;
                self.assertions();
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn fill_buf_slow(&mut self) -> crate::Result<()> {
        self.assertions();
        if self.limit == self.pos() {
            return Ok(());
        }

        match self.input_source {
            InputSource::Read(..) => {}
            _ => return Ok(()),
        }

        self.consume_buf()?;

        match self.input_source {
            InputSource::Read(ref mut buf_read) => {
                self.buf = unsafe { InputBuf::from_bytes_ignore_lifetime(buf_read.fill_buf()?) };
                self.update_limit_within_buf();
                Ok(())
            }
            _ => {
                unreachable!();
            }
        }
    }

    #[inline(always)]
    pub(crate) fn fill_buf(&mut self) -> crate::Result<&[u8]> {
        let rem = self.buf.remaining_in_buf();
        if !rem.is_empty() {
            return Ok(rem);
        }

        if self.limit == self.pos() {
            return Ok(&[]);
        }

        self.fill_buf_slow()?;

        Ok(self.buf.remaining_in_buf())
    }
}

#[cfg(all(test, feature = "bytes"))]
mod test_bytes {
    use std::io::Write;

    use super::*;

    fn make_long_string(len: usize) -> Vec<u8> {
        let mut s = Vec::new();
        while s.len() < len {
            let len = s.len();
            write!(&mut s, "{}", len).expect("unexpected");
        }
        s.truncate(len);
        s
    }

    #[test]
    #[cfg_attr(miri, ignore)] // bytes violates SB, see https://github.com/tokio-rs/bytes/issues/522
    fn read_exact_bytes_from_slice() {
        let bytes = make_long_string(100);
        let mut bri = BufReadIter::from_byte_slice(&bytes[..]);
        assert_eq!(&bytes[..90], &bri.read_exact_bytes(90).unwrap()[..]);
        assert_eq!(bytes[90], bri.read_byte().expect("read_byte"));
    }

    #[test]
    #[cfg_attr(miri, ignore)] // bytes violates SB, see https://github.com/tokio-rs/bytes/issues/522
    fn read_exact_bytes_from_bytes() {
        let bytes = Bytes::from(make_long_string(100));
        let mut bri = BufReadIter::from_bytes(&bytes);
        let read = bri.read_exact_bytes(90).unwrap();
        assert_eq!(&bytes[..90], &read[..]);
        assert_eq!(&bytes[..90].as_ptr(), &read.as_ptr());
        assert_eq!(bytes[90], bri.read_byte().expect("read_byte"));
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    #[test]
    fn eof_at_limit() {
        struct Read5ThenPanic {
            pos: usize,
        }

        impl Read for Read5ThenPanic {
            fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                unreachable!();
            }
        }

        impl BufRead for Read5ThenPanic {
            fn fill_buf(&mut self) -> io::Result<&[u8]> {
                assert_eq!(0, self.pos);
                static ZERO_TO_FIVE: &'static [u8] = &[0, 1, 2, 3, 4];
                Ok(ZERO_TO_FIVE)
            }

            fn consume(&mut self, amt: usize) {
                if amt == 0 {
                    // drop of BufReadIter
                    return;
                }

                assert_eq!(0, self.pos);
                assert_eq!(5, amt);
                self.pos += amt;
            }
        }

        let mut read = Read5ThenPanic { pos: 0 };
        let mut buf_read_iter = BufReadIter::from_buf_read(&mut read);
        assert_eq!(0, buf_read_iter.pos());
        let _prev_limit = buf_read_iter.push_limit(5);
        buf_read_iter.read_byte().expect("read_byte");
        buf_read_iter
            .read_exact(&mut [
                MaybeUninit::uninit(),
                MaybeUninit::uninit(),
                MaybeUninit::uninit(),
                MaybeUninit::uninit(),
            ])
            .expect("read_exact");
        assert!(buf_read_iter.eof().expect("eof"));
    }
}
