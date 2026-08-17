use crate::{ErrorType, Write};
use alloc::vec::Vec;

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl ErrorType for Vec<u8> {
    type Error = core::convert::Infallible;
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl Write for Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
