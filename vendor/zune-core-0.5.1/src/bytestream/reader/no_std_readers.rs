#![allow(dead_code)]

use crate::bytestream::{ZByteIoError, ZByteReaderTrait, ZSeekFrom};
/// Wraps an in memory buffer providing it with a `Seek` method
/// but works in `no_std` environments
///
/// `std::io::Cursor` is available in std environments, but we also need support
/// for `no_std` environments so this serves as a drop in replacement
pub struct ZCursor<T> {
    pub(crate) stream:   T,
    pub(crate) position: usize
}

impl<T: AsRef<[u8]>> ZCursor<T> {
    pub fn new(buffer: T) -> ZCursor<T> {
        ZCursor {
            stream:   buffer,
            position: 0
        }
    }
}

impl<T: AsRef<[u8]>> ZCursor<T> {
    /// Move forward `num` bytes  from
    /// the current position.
    ///
    /// It doesn't check that position overflowed, new position
    /// may point past the internal buffer, all subsequent reads will
    /// either return an error or zero depending on the method called
    #[inline]
    pub fn skip(&mut self, num: usize) {
        // Can this overflow ??
        self.position = self.position.wrapping_add(num);
    }
    /// Move back `num` bytes from the current position
    ///
    ///
    /// This saturates at zero, it can never be negative or wraparound
    /// when the value becomes too small
    #[inline]
    pub fn rewind(&mut self, num: usize) {
        self.position = self.position.saturating_sub(num);
    }

    pub fn split(&self) -> (&[u8], &[u8]) {
        let slice = self.stream.as_ref();
        let pos = self.position.min(slice.len());
        slice.split_at(pos)
    }
}

impl<T: AsRef<[u8]>>  ZCursor<T> {
    #[inline(always)]
    pub (crate) fn read_byte_no_error_impl(&mut self) -> u8 {
        let byte = self.stream.as_ref().get(self.position).unwrap_or(&0);
        self.position += 1;
        *byte
    }
    #[inline(always)]
    pub (crate) fn read_exact_bytes_impl(&mut self, buf: &mut [u8]) -> Result<(), ZByteIoError> {
        let bytes_read = self.read_bytes(buf)?;
        if bytes_read != buf.len() {
            // restore read to initial position it was in.
            self.rewind(bytes_read);
            // not all bytes were read.
            return Err(ZByteIoError::NotEnoughBytes(bytes_read, buf.len()));
        }
        Ok(())
    }

    pub (crate) fn read_const_bytes_impl<const N: usize>(&mut self, buf: &mut [u8; N]) -> Result<(), ZByteIoError> {
        if self.position + N <= self.stream.as_ref().len() {
            // we are in bounds
            let reference = self.stream.as_ref();
            let position = self.position;
            if let Some(buf_ref) = reference.get(position..position + N) {
                self.position += N;
                buf.copy_from_slice(buf_ref);
                return Ok(());
            }
        }
        Err(ZByteIoError::Generic("Cannot satisfy read"))
    }

    pub (crate) fn read_const_bytes_no_error_impl<const N: usize>(&mut self, buf: &mut [u8; N]) {
        if self.position + N <= self.stream.as_ref().len() {
            // we are in bounds
            let reference = self.stream.as_ref();
            let position = self.position;
            if let Some(buf_ref) = reference.get(position..position + N) {
                self.position += N;
                buf.copy_from_slice(buf_ref);
            }
        }
    }

    #[inline(always)]
  pub (crate) fn read_bytes_impl(&mut self, buf: &mut [u8]) -> Result<usize, ZByteIoError> {
        let len = self.peek_bytes_impl(buf)?;
        self.skip(len);
        Ok(len)
    }

    #[inline(always)]
    pub (crate) fn peek_bytes_impl(&mut self, buf: &mut [u8]) -> Result<usize, ZByteIoError> {
        let stream_end = self.stream.as_ref().len();

        let start = core::cmp::min(self.position, stream_end);
        let end = core::cmp::min(self.position + buf.len(), stream_end);

        let slice = self.stream.as_ref().get(start..end).unwrap();
        buf[..slice.len()].copy_from_slice(slice);
        let len = slice.len();

        Ok(len)
    }

    #[inline(always)]
    pub (crate) fn peek_exact_bytes_impl(&mut self, buf: &mut [u8]) -> Result<(), ZByteIoError> {
        self.read_exact_bytes_impl(buf)?;
        self.rewind(buf.len());
        Ok(())
    }

    #[inline(always)]
    pub (crate) fn z_seek_impl(&mut self, from: ZSeekFrom) -> Result<u64, ZByteIoError> {
        let (base_pos, offset) = match from {
            ZSeekFrom::Start(n) => {
                self.position = n as usize;
                return Ok(n);
            }
            ZSeekFrom::End(n) => (self.stream.as_ref().len(), n as isize),
            ZSeekFrom::Current(n) => (self.position, n as isize)
        };
        match base_pos.checked_add_signed(offset) {
            Some(n) => {
                self.position = n;
                Ok(self.position as u64)
            }
            None => Err(ZByteIoError::SeekError("Negative seek"))
        }
    }

    #[inline(always)]
    pub (crate) fn is_eof_impl(&mut self) -> Result<bool, ZByteIoError> {
        Ok(self.position >= self.stream.as_ref().len())
    }
    #[inline(always)]
    pub (crate) fn z_position_impl(&mut self) -> Result<u64, ZByteIoError> {
        Ok(self.position as u64)
    }

    pub (crate) fn read_remaining_impl(&mut self, sink: &mut alloc::vec::Vec<u8>) -> Result<usize, ZByteIoError> {
        let start = self.position;
        let end = self.stream.as_ref().len();
        match self.stream.as_ref().get(start..end) {
            None => {
                return Err(ZByteIoError::Generic(
                    "Somehow read remaining couldn't satisfy it's invariants"
                ))
            }
            Some(e) => {
                sink.extend_from_slice(e);
            }
        }
        self.skip(end - start);
        Ok(end - start)
    }
}

impl<T: AsRef<[u8]>> From<T> for ZCursor<T> {
    fn from(value: T) -> Self {
        ZCursor::new(value)
    }
}
