use {Buf};

use std::{cmp, io};

/// A `Buf` adapter which implements `io::Read` for the inner value.
///
/// This struct is generally created by calling `reader()` on `Buf`. See
/// documentation of [`reader()`](trait.Buf.html#method.reader) for more
/// details.
#[derive(Debug)]
pub struct Reader<B> {
    buf: B,
}

pub fn new<B>(buf: B) -> Reader<B> {
    Reader { buf: buf }
}

impl<B: Buf> Reader<B> {
    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Buf;
    /// use std::io::{self, Cursor};
    ///
    /// let mut buf = Cursor::new(b"hello world").reader();
    ///
    /// assert_eq!(0, buf.get_ref().position());
    /// ```
    pub fn get_ref(&self) -> &B {
        &self.buf
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Buf;
    /// use std::io::{self, Cursor};
    ///
    /// let mut buf = Cursor::new(b"hello world").reader();
    /// let mut dst = vec![];
    ///
    /// buf.get_mut().set_position(2);
    /// io::copy(&mut buf, &mut dst).unwrap();
    ///
    /// assert_eq!(*dst, b"llo world"[..]);
    /// ```
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Consumes this `Reader`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Buf;
    /// use std::io::{self, Cursor};
    ///
    /// let mut buf = Cursor::new(b"hello world").reader();
    /// let mut dst = vec![];
    ///
    /// io::copy(&mut buf, &mut dst).unwrap();
    ///
    /// let buf = buf.into_inner();
    /// assert_eq!(0, buf.remaining());
    /// ```
    pub fn into_inner(self) -> B {
        self.buf
    }
}

impl<B: Buf + Sized> io::Read for Reader<B> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(self.buf.remaining(), dst.len());

        Buf::copy_to_slice(&mut self.buf, &mut dst[0..len]);
        Ok(len)
    }
}

impl<B: Buf + Sized> io::BufRead for Reader<B> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(self.buf.bytes())
    }
    fn consume(&mut self, amt: usize) {
        self.buf.advance(amt)
    }
}
