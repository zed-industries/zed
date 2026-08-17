use Buf;

/// Iterator over the bytes contained by the buffer.
///
/// This struct is created by the [`iter`] method on [`Buf`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use bytes::{Buf, IntoBuf, Bytes};
///
/// let buf = Bytes::from(&b"abc"[..]).into_buf();
/// let mut iter = buf.iter();
///
/// assert_eq!(iter.next(), Some(b'a'));
/// assert_eq!(iter.next(), Some(b'b'));
/// assert_eq!(iter.next(), Some(b'c'));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`iter`]: trait.Buf.html#method.iter
/// [`Buf`]: trait.Buf.html
#[derive(Debug)]
pub struct Iter<T> {
    inner: T,
}

impl<T> Iter<T> {
    /// Consumes this `Iter`, returning the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, IntoBuf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]).into_buf();
    /// let mut iter = buf.iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// let buf = iter.into_inner();
    /// assert_eq!(2, buf.remaining());
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, IntoBuf, Bytes};
    ///
    /// let buf = Bytes::from(&b"abc"[..]).into_buf();
    /// let mut iter = buf.iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// assert_eq!(2, iter.get_ref().remaining());
    /// ```
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying `Buf`.
    ///
    /// It is inadvisable to directly read from the underlying `Buf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::{Buf, IntoBuf, BytesMut};
    ///
    /// let buf = BytesMut::from(&b"abc"[..]).into_buf();
    /// let mut iter = buf.iter();
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    ///
    /// iter.get_mut().set_position(0);
    ///
    /// assert_eq!(iter.next(), Some(b'a'));
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub fn new<T>(inner: T) -> Iter<T> {
    Iter { inner: inner }
}

impl<T: Buf> Iterator for Iter<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if !self.inner.has_remaining() {
            return None;
        }

        let b = self.inner.bytes()[0];
        self.inner.advance(1);
        Some(b)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.inner.remaining();
        (rem, Some(rem))
    }
}

impl<T: Buf> ExactSizeIterator for Iter<T> { }
