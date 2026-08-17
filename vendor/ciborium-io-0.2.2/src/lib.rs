// SPDX-License-Identifier: Apache-2.0

//! Simple, Low-level I/O traits
//!
//! This crate provides two simple traits: `Read` and `Write`. These traits
//! mimic their counterparts in `std::io`, but are trimmed for simplicity
//! and can be used in `no_std` and `no_alloc` environments. Since this
//! crate contains only traits, inline functions and unit structs, it should
//! be a zero-cost abstraction.
//!
//! If the `std` feature is enabled, we provide blanket implementations for
//! all `std::io` types. If the `alloc` feature is enabled, we provide
//! implementations for `Vec<u8>`. In all cases, you get implementations
//! for byte slices. You can, of course, implement the traits for your own
//! types.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// A trait indicating a type that can read bytes
///
/// Note that this is similar to `std::io::Read`, but simplified for use in a
/// `no_std` context.
pub trait Read {
    /// The error type
    type Error;

    /// Reads exactly `data.len()` bytes or fails
    fn read_exact(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;
}

/// A trait indicating a type that can write bytes
///
/// Note that this is similar to `std::io::Write`, but simplified for use in a
/// `no_std` context.
pub trait Write {
    /// The error type
    type Error;

    /// Writes all bytes from `data` or fails
    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Flushes all output
    fn flush(&mut self) -> Result<(), Self::Error>;
}

#[cfg(feature = "std")]
impl<T: std::io::Read> Read for T {
    type Error = std::io::Error;

    #[inline]
    fn read_exact(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        self.read_exact(data)
    }
}

#[cfg(feature = "std")]
impl<T: std::io::Write> Write for T {
    type Error = std::io::Error;

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.write_all(data)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush()
    }
}

#[cfg(not(feature = "std"))]
impl<R: Read + ?Sized> Read for &mut R {
    type Error = R::Error;

    #[inline]
    fn read_exact(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        (**self).read_exact(data)
    }
}

#[cfg(not(feature = "std"))]
impl<W: Write + ?Sized> Write for &mut W {
    type Error = W::Error;

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        (**self).write_all(data)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        (**self).flush()
    }
}

/// An error indicating there are no more bytes to read
#[cfg(not(feature = "std"))]
#[derive(Debug)]
pub struct EndOfFile(());

#[cfg(not(feature = "std"))]
impl Read for &[u8] {
    type Error = EndOfFile;

    #[inline]
    fn read_exact(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        if data.len() > self.len() {
            return Err(EndOfFile(()));
        }

        let (prefix, suffix) = self.split_at(data.len());
        data.copy_from_slice(prefix);
        *self = suffix;
        Ok(())
    }
}

/// An error indicating that the output cannot accept more bytes
#[cfg(not(feature = "std"))]
#[derive(Debug)]
pub struct OutOfSpace(());

#[cfg(not(feature = "std"))]
impl Write for &mut [u8] {
    type Error = OutOfSpace;

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        if data.len() > self.len() {
            return Err(OutOfSpace(()));
        }

        let (prefix, suffix) = core::mem::take(self).split_at_mut(data.len());
        prefix.copy_from_slice(data);
        *self = suffix;
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(all(not(feature = "std"), feature = "alloc"))]
impl Write for alloc::vec::Vec<u8> {
    type Error = core::convert::Infallible;

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.extend_from_slice(data);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_eof() {
        let mut reader = &[1u8; 0][..];
        let mut buffer = [0u8; 1];

        reader.read_exact(&mut buffer[..]).unwrap_err();
    }

    #[test]
    fn read_one() {
        let mut reader = &[1u8; 1][..];
        let mut buffer = [0u8; 1];

        reader.read_exact(&mut buffer[..]).unwrap();
        assert_eq!(buffer[0], 1);

        reader.read_exact(&mut buffer[..]).unwrap_err();
    }

    #[test]
    fn read_two() {
        let mut reader = &[1u8; 2][..];
        let mut buffer = [0u8; 1];

        reader.read_exact(&mut buffer[..]).unwrap();
        assert_eq!(buffer[0], 1);

        reader.read_exact(&mut buffer[..]).unwrap();
        assert_eq!(buffer[0], 1);

        reader.read_exact(&mut buffer[..]).unwrap_err();
    }

    #[test]
    #[cfg(feature = "std")]
    fn read_std() {
        let mut reader = std::io::repeat(1);
        let mut buffer = [0u8; 2];

        reader.read_exact(&mut buffer[..]).unwrap();
        assert_eq!(buffer[0], 1);
        assert_eq!(buffer[1], 1);
    }

    #[test]
    fn write_oos() {
        let mut writer = &mut [0u8; 0][..];

        writer.write_all(&[1u8; 1][..]).unwrap_err();
    }

    #[test]
    fn write_one() {
        let mut buffer = [0u8; 1];
        let mut writer = &mut buffer[..];

        writer.write_all(&[1u8; 1][..]).unwrap();
        writer.write_all(&[1u8; 1][..]).unwrap_err();
        assert_eq!(buffer[0], 1);
    }

    #[test]
    fn write_two() {
        let mut buffer = [0u8; 2];
        let mut writer = &mut buffer[..];

        writer.write_all(&[1u8; 1][..]).unwrap();
        writer.write_all(&[1u8; 1][..]).unwrap();
        writer.write_all(&[1u8; 1][..]).unwrap_err();
        assert_eq!(buffer[0], 1);
        assert_eq!(buffer[1], 1);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn write_vec() {
        let mut buffer = alloc::vec::Vec::new();

        buffer.write_all(&[1u8; 1][..]).unwrap();
        buffer.write_all(&[1u8; 1][..]).unwrap();

        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer[0], 1);
        assert_eq!(buffer[1], 1);
    }

    #[test]
    #[cfg(feature = "std")]
    fn write_std() {
        let mut writer = std::io::sink();

        writer.write_all(&[1u8; 1][..]).unwrap();
        writer.write_all(&[1u8; 1][..]).unwrap();
    }
}
