//! Traits used in this library
use std::io;

/// Writer extension to write little endian data
pub trait WriteBytesExt<T> {
    /// Writes `T` to a bytes stream. Least significant byte first.
    fn write_le(&mut self, n: T) -> io::Result<()>;

    /*
    #[inline]
    fn write_byte(&mut self, n: u8) -> io::Result<()> where Self: Write {
        self.write_all(&[n])
    }
    */
}

impl<W: io::Write + ?Sized> WriteBytesExt<u8> for W {
    #[inline(always)]
    fn write_le(&mut self, n: u8) -> io::Result<()> {
        self.write_all(&[n])
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt<u16> for W {
    #[inline]
    fn write_le(&mut self, n: u16) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt<u32> for W {
    #[inline]
    fn write_le(&mut self, n: u32) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}

impl<W: io::Write + ?Sized> WriteBytesExt<u64> for W {
    #[inline]
    fn write_le(&mut self, n: u64) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }
}
