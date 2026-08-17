#![cfg(not(feature = "std"))]
use crate::bytestream::{ZByteIoError, ZByteReaderTrait, ZCursor, ZSeekFrom};

impl<T: AsRef<[u8]>> ZByteReaderTrait for ZCursor<T> {
    #[inline(always)]
    fn read_byte_no_error(&mut self) -> u8 {
        self.read_byte_no_error_impl()
    }
    #[inline(always)]
    fn read_exact_bytes(&mut self, buf: &mut [u8]) -> Result<(), ZByteIoError> {
        self.read_exact_bytes_impl(buf)
    }

    fn read_const_bytes<const N: usize>(&mut self, buf: &mut [u8; N]) -> Result<(), ZByteIoError> {
        self.read_const_bytes_impl::<N>(buf)
    }

    fn read_const_bytes_no_error<const N: usize>(&mut self, buf: &mut [u8; N]) {
        self.read_const_bytes_no_error_impl::<N>(buf)
    }

    #[inline(always)]
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ZByteIoError> {
        self.read_bytes_impl(buf)
    }

    #[inline(always)]
    fn peek_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ZByteIoError> {
        self.peek_bytes_impl(buf)
    }

    #[inline(always)]
    fn peek_exact_bytes(&mut self, buf: &mut [u8]) -> Result<(), ZByteIoError> {
        self.peek_exact_bytes_impl(buf)
    }

    #[inline(always)]
    fn z_seek(&mut self, from: ZSeekFrom) -> Result<u64, ZByteIoError> {
        self.z_seek_impl(from)
    }

    #[inline(always)]
    fn is_eof(&mut self) -> Result<bool, ZByteIoError> {
        self.is_eof_impl()
    }
    #[inline(always)]
    fn z_position(&mut self) -> Result<u64, ZByteIoError> {
        self.z_position_impl()
    }

    fn read_remaining(&mut self, sink: &mut alloc::vec::Vec<u8>) -> Result<usize, ZByteIoError> {
        self.read_remaining_impl(sink)
    }
}
