use crate::{ser::Serializer, Fallible};
use std::io;

/// Wraps a type that implements [`io::Write`](std::io::Write) and equips it with [`Serializer`].
///
/// # Examples
/// ```
/// use rkyv::ser::{serializers::WriteSerializer, Serializer};
///
/// let mut serializer = WriteSerializer::new(Vec::new());
/// assert_eq!(serializer.pos(), 0);
/// serializer.write(&[0u8, 1u8, 2u8, 3u8]);
/// assert_eq!(serializer.pos(), 4);
/// let buf = serializer.into_inner();
/// assert_eq!(buf.len(), 4);
/// assert_eq!(buf, vec![0u8, 1u8, 2u8, 3u8]);
/// ```
#[derive(Debug)]
pub struct WriteSerializer<W: io::Write> {
    inner: W,
    pos: usize,
}

impl<W: io::Write> WriteSerializer<W> {
    /// Creates a new serializer from a writer.
    #[inline]
    pub fn new(inner: W) -> Self {
        Self::with_pos(inner, 0)
    }

    /// Creates a new serializer from a writer, and assumes that the underlying writer is currently
    /// at the given position.
    #[inline]
    pub fn with_pos(inner: W, pos: usize) -> Self {
        Self { inner, pos }
    }

    /// Consumes the serializer and returns the internal writer used to create it.
    #[inline]
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: io::Write> Fallible for WriteSerializer<W> {
    type Error = io::Error;
}

impl<W: io::Write> Serializer for WriteSerializer<W> {
    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.write_all(bytes)?;
        self.pos += bytes.len();
        Ok(())
    }
}
