// We cannot use the below impls and std ones because we'll re-implement the
// same trait fot &[u8] which is blanketed by write. Ending up with two separate implementations
#![cfg(not(feature = "std"))]
use crate::bytestream::{ZByteIoError, ZByteWriterTrait};

impl ZByteWriterTrait for &mut [u8] {
    fn write_bytes(&mut self, buf: &[u8]) -> Result<usize, ZByteIoError> {
        // got from the write of std
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = core::mem::take(self).split_at_mut(amt);
        a.copy_from_slice(&buf[..amt]);
        *self = b;
        Ok(amt)
    }

    fn write_all_bytes(&mut self, buf: &[u8]) -> Result<(), ZByteIoError> {
        if buf.len() > self.len() {
            return Err(ZByteIoError::NotEnoughBuffer(self.len(), buf.len()));
        }
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = core::mem::take(self).split_at_mut(amt);
        a.copy_from_slice(&buf[..amt]);
        *self = b;

        Ok(())
    }

    fn write_const_bytes<const N: usize>(&mut self, buf: &[u8; N]) -> Result<(), ZByteIoError> {
        if N > self.len() {
            return Err(ZByteIoError::NotEnoughBuffer(self.len(), N));
        }
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = core::mem::take(self).split_at_mut(amt);
        a.copy_from_slice(&buf[..amt]);
        *self = b;
        Ok(())
    }

    fn flush_bytes(&mut self) -> Result<(), ZByteIoError> {
        Ok(())
    }
    fn reserve_capacity(&mut self, _: usize) -> Result<(), ZByteIoError> {
        // can't really pre-allocate anything here
        Ok(())
    }
}

impl ZByteWriterTrait for &mut alloc::vec::Vec<u8> {
    fn write_bytes(&mut self, buf: &[u8]) -> Result<usize, ZByteIoError> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn write_all_bytes(&mut self, buf: &[u8]) -> Result<(), ZByteIoError> {
        self.extend_from_slice(buf);
        Ok(())
    }

    fn write_const_bytes<const N: usize>(&mut self, buf: &[u8; N]) -> Result<(), ZByteIoError> {
        self.extend_from_slice(buf);
        Ok(())
    }
    fn flush_bytes(&mut self) -> Result<(), ZByteIoError> {
        Ok(())
    }
    fn reserve_capacity(&mut self, size: usize) -> Result<(), ZByteIoError> {
        self.reserve(size);
        Ok(())
    }
}
