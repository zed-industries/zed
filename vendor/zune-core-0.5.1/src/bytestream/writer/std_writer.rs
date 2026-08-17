#![cfg(feature = "std")]

use std::io::Write;

use crate::bytestream::ZByteIoError;

impl<T: Write> crate::bytestream::ZByteWriterTrait for T {
    fn write_bytes(&mut self, buf: &[u8]) -> Result<usize, ZByteIoError> {
        self.write(buf).map_err(ZByteIoError::StdIoError)
    }

    fn write_all_bytes(&mut self, buf: &[u8]) -> Result<(), ZByteIoError> {
        self.write_all(buf).map_err(ZByteIoError::StdIoError)
    }

    fn write_const_bytes<const N: usize>(&mut self, buf: &[u8; N]) -> Result<(), ZByteIoError> {
        self.write_all_bytes(buf)
    }
    fn flush_bytes(&mut self) -> Result<(), ZByteIoError> {
        self.flush().map_err(ZByteIoError::StdIoError)
    }
    fn reserve_capacity(&mut self, _: usize) -> Result<(), ZByteIoError> {
        // we can't reserve capacity, sorry to implementations where this
        // matters
        Ok(())
    }
}
