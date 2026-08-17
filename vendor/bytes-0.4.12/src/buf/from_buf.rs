use {Buf, BufMut, IntoBuf, Bytes, BytesMut};

/// Conversion from a [`Buf`]
///
/// Implementing `FromBuf` for a type defines how it is created from a buffer.
/// This is common for types which represent byte storage of some kind.
///
/// [`FromBuf::from_buf`] is rarely called explicitly, and it is instead used
/// through [`Buf::collect`]. See [`Buf::collect`] documentation for more examples.
///
/// See also [`IntoBuf`].
///
/// # Examples
///
/// Basic  usage:
///
/// ```
/// use bytes::{Bytes, IntoBuf};
/// use bytes::buf::FromBuf;
///
/// let buf = Bytes::from(&b"hello world"[..]).into_buf();
/// let vec = Vec::from_buf(buf);
///
/// assert_eq!(vec, &b"hello world"[..]);
/// ```
///
/// Using [`Buf::collect`] to implicitly use `FromBuf`:
///
/// ```
/// use bytes::{Buf, Bytes, IntoBuf};
///
/// let buf = Bytes::from(&b"hello world"[..]).into_buf();
/// let vec: Vec<u8> = buf.collect();
///
/// assert_eq!(vec, &b"hello world"[..]);
/// ```
///
/// Implementing `FromBuf` for your type:
///
/// ```
/// use bytes::{BufMut, Bytes};
/// use bytes::buf::{IntoBuf, FromBuf};
///
/// // A sample buffer, that's just a wrapper over Vec<u8>
/// struct MyBuffer(Vec<u8>);
///
/// impl FromBuf for MyBuffer {
///     fn from_buf<B>(buf: B) -> Self where B: IntoBuf {
///         let mut v = Vec::new();
///         v.put(buf.into_buf());
///         MyBuffer(v)
///     }
/// }
///
/// // Now we can make a new buf
/// let buf = Bytes::from(&b"hello world"[..]);
///
/// // And make a MyBuffer out of it
/// let my_buf = MyBuffer::from_buf(buf);
///
/// assert_eq!(my_buf.0, &b"hello world"[..]);
/// ```
///
/// [`Buf`]: trait.Buf.html
/// [`FromBuf::from_buf`]: #method.from_buf
/// [`Buf::collect`]: trait.Buf.html#method.collect
/// [`IntoBuf`]: trait.IntoBuf.html
pub trait FromBuf {
    /// Creates a value from a buffer.
    ///
    /// See the [type-level documentation](#) for more details.
    ///
    /// # Examples
    ///
    /// Basic  usage:
    ///
    /// ```
    /// use bytes::{Bytes, IntoBuf};
    /// use bytes::buf::FromBuf;
    ///
    /// let buf = Bytes::from(&b"hello world"[..]).into_buf();
    /// let vec = Vec::from_buf(buf);
    ///
    /// assert_eq!(vec, &b"hello world"[..]);
    /// ```
    fn from_buf<T>(buf: T) -> Self where T: IntoBuf;
}

impl FromBuf for Vec<u8> {
    fn from_buf<T>(buf: T) -> Self
        where T: IntoBuf
    {
        let buf = buf.into_buf();
        let mut ret = Vec::with_capacity(buf.remaining());
        ret.put(buf);
        ret
    }
}

impl FromBuf for Bytes {
    fn from_buf<T>(buf: T) -> Self
        where T: IntoBuf
    {
        BytesMut::from_buf(buf).freeze()
    }
}

impl FromBuf for BytesMut {
    fn from_buf<T>(buf: T) -> Self
        where T: IntoBuf
    {
        let buf = buf.into_buf();
        let mut ret = BytesMut::with_capacity(buf.remaining());
        ret.put(buf);
        ret
    }
}
