use super::{IntoBuf, Writer};
use byteorder::{LittleEndian, ByteOrder, BigEndian};
use iovec::IoVec;

use std::{cmp, io, ptr, usize};

/// A trait for values that provide sequential write access to bytes.
///
/// Write bytes to a buffer
///
/// A buffer stores bytes in memory such that write operations are infallible.
/// The underlying storage may or may not be in contiguous memory. A `BufMut`
/// value is a cursor into the buffer. Writing to `BufMut` advances the cursor
/// position.
///
/// The simplest `BufMut` is a `Vec<u8>`.
///
/// ```
/// use bytes::BufMut;
///
/// let mut buf = vec![];
///
/// buf.put("hello world");
///
/// assert_eq!(buf, b"hello world");
/// ```
pub trait BufMut {
    /// Returns the number of bytes that can be written from the current
    /// position until the end of the buffer is reached.
    ///
    /// This value is greater than or equal to the length of the slice returned
    /// by `bytes_mut`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    /// use std::io::Cursor;
    ///
    /// let mut dst = [0; 10];
    /// let mut buf = Cursor::new(&mut dst[..]);
    ///
    /// assert_eq!(10, buf.remaining_mut());
    /// buf.put("hello");
    ///
    /// assert_eq!(5, buf.remaining_mut());
    /// ```
    ///
    /// # Implementer notes
    ///
    /// Implementations of `remaining_mut` should ensure that the return value
    /// does not change unless a call is made to `advance_mut` or any other
    /// function that is documented to change the `BufMut`'s current position.
    fn remaining_mut(&self) -> usize;

    /// Advance the internal cursor of the BufMut
    ///
    /// The next call to `bytes_mut` will return a slice starting `cnt` bytes
    /// further into the underlying buffer.
    ///
    /// This function is unsafe because there is no guarantee that the bytes
    /// being advanced past have been initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = Vec::with_capacity(16);
    ///
    /// unsafe {
    ///     buf.bytes_mut()[0] = b'h';
    ///     buf.bytes_mut()[1] = b'e';
    ///
    ///     buf.advance_mut(2);
    ///
    ///     buf.bytes_mut()[0] = b'l';
    ///     buf.bytes_mut()[1..3].copy_from_slice(b"lo");
    ///
    ///     buf.advance_mut(3);
    /// }
    ///
    /// assert_eq!(5, buf.len());
    /// assert_eq!(buf, b"hello");
    /// ```
    ///
    /// # Panics
    ///
    /// This function **may** panic if `cnt > self.remaining_mut()`.
    ///
    /// # Implementer notes
    ///
    /// It is recommended for implementations of `advance_mut` to panic if
    /// `cnt > self.remaining_mut()`. If the implementation does not panic,
    /// the call must behave as if `cnt == self.remaining_mut()`.
    ///
    /// A call with `cnt == 0` should never panic and be a no-op.
    unsafe fn advance_mut(&mut self, cnt: usize);

    /// Returns true if there is space in `self` for more bytes.
    ///
    /// This is equivalent to `self.remaining_mut() != 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    /// use std::io::Cursor;
    ///
    /// let mut dst = [0; 5];
    /// let mut buf = Cursor::new(&mut dst);
    ///
    /// assert!(buf.has_remaining_mut());
    ///
    /// buf.put("hello");
    ///
    /// assert!(!buf.has_remaining_mut());
    /// ```
    fn has_remaining_mut(&self) -> bool {
        self.remaining_mut() > 0
    }

    /// Returns a mutable slice starting at the current BufMut position and of
    /// length between 0 and `BufMut::remaining_mut()`. Note that this *can* be shorter than the
    /// whole remainder of the buffer (this allows non-continuous implementation).
    ///
    /// This is a lower level function. Most operations are done with other
    /// functions.
    ///
    /// The returned byte slice may represent uninitialized memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = Vec::with_capacity(16);
    ///
    /// unsafe {
    ///     buf.bytes_mut()[0] = b'h';
    ///     buf.bytes_mut()[1] = b'e';
    ///
    ///     buf.advance_mut(2);
    ///
    ///     buf.bytes_mut()[0] = b'l';
    ///     buf.bytes_mut()[1..3].copy_from_slice(b"lo");
    ///
    ///     buf.advance_mut(3);
    /// }
    ///
    /// assert_eq!(5, buf.len());
    /// assert_eq!(buf, b"hello");
    /// ```
    ///
    /// # Implementer notes
    ///
    /// This function should never panic. `bytes_mut` should return an empty
    /// slice **if and only if** `remaining_mut` returns 0. In other words,
    /// `bytes_mut` returning an empty slice implies that `remaining_mut` will
    /// return 0 and `remaining_mut` returning 0 implies that `bytes_mut` will
    /// return an empty slice.
    unsafe fn bytes_mut(&mut self) -> &mut [u8];

    /// Fills `dst` with potentially multiple mutable slices starting at `self`'s
    /// current position.
    ///
    /// If the `BufMut` is backed by disjoint slices of bytes, `bytes_vec_mut`
    /// enables fetching more than one slice at once. `dst` is a slice of
    /// mutable `IoVec` references, enabling the slice to be directly used with
    /// [`readv`] without any further conversion. The sum of the lengths of all
    /// the buffers in `dst` will be less than or equal to
    /// `Buf::remaining_mut()`.
    ///
    /// The entries in `dst` will be overwritten, but the data **contained** by
    /// the slices **will not** be modified. If `bytes_vec_mut` does not fill every
    /// entry in `dst`, then `dst` is guaranteed to contain all remaining slices
    /// in `self.
    ///
    /// This is a lower level function. Most operations are done with other
    /// functions.
    ///
    /// # Implementer notes
    ///
    /// This function should never panic. Once the end of the buffer is reached,
    /// i.e., `BufMut::remaining_mut` returns 0, calls to `bytes_vec_mut` must
    /// return 0 without mutating `dst`.
    ///
    /// Implementations should also take care to properly handle being called
    /// with `dst` being a zero length slice.
    ///
    /// [`readv`]: http://man7.org/linux/man-pages/man2/readv.2.html
    unsafe fn bytes_vec_mut<'a>(&'a mut self, dst: &mut [&'a mut IoVec]) -> usize {
        if dst.is_empty() {
            return 0;
        }

        if self.has_remaining_mut() {
            dst[0] = self.bytes_mut().into();
            1
        } else {
            0
        }
    }

    /// Transfer bytes into `self` from `src` and advance the cursor by the
    /// number of bytes written.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    ///
    /// buf.put(b'h');
    /// buf.put(&b"ello"[..]);
    /// buf.put(" world");
    ///
    /// assert_eq!(buf, b"hello world");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `self` does not have enough capacity to contain `src`.
    fn put<T: IntoBuf>(&mut self, src: T) where Self: Sized {
        use super::Buf;

        let mut src = src.into_buf();

        assert!(self.remaining_mut() >= src.remaining());

        while src.has_remaining() {
            let l;

            unsafe {
                let s = src.bytes();
                let d = self.bytes_mut();
                l = cmp::min(s.len(), d.len());

                ptr::copy_nonoverlapping(
                    s.as_ptr(),
                    d.as_mut_ptr(),
                    l);
            }

            src.advance(l);
            unsafe { self.advance_mut(l); }
        }
    }

    /// Transfer bytes into `self` from `src` and advance the cursor by the
    /// number of bytes written.
    ///
    /// `self` must have enough remaining capacity to contain all of `src`.
    ///
    /// ```
    /// use bytes::BufMut;
    /// use std::io::Cursor;
    ///
    /// let mut dst = [0; 6];
    ///
    /// {
    ///     let mut buf = Cursor::new(&mut dst);
    ///     buf.put_slice(b"hello");
    ///
    ///     assert_eq!(1, buf.remaining_mut());
    /// }
    ///
    /// assert_eq!(b"hello\0", &dst);
    /// ```
    fn put_slice(&mut self, src: &[u8]) {
        let mut off = 0;

        assert!(self.remaining_mut() >= src.len(), "buffer overflow");

        while off < src.len() {
            let cnt;

            unsafe {
                let dst = self.bytes_mut();
                cnt = cmp::min(dst.len(), src.len() - off);

                ptr::copy_nonoverlapping(
                    src[off..].as_ptr(),
                    dst.as_mut_ptr(),
                    cnt);

                off += cnt;

            }

            unsafe { self.advance_mut(cnt); }
        }
    }

    /// Writes an unsigned 8 bit integer to `self`.
    ///
    /// The current position is advanced by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u8(0x01);
    /// assert_eq!(buf, b"\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_u8(&mut self, n: u8) {
        let src = [n];
        self.put_slice(&src);
    }

    /// Writes a signed 8 bit integer to `self`.
    ///
    /// The current position is advanced by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i8(0x01);
    /// assert_eq!(buf, b"\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_i8(&mut self, n: i8) {
        let src = [n as u8];
        self.put_slice(&src)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_u16_be or put_u16_le")]
    fn put_u16<T: ByteOrder>(&mut self, n: u16) where Self: Sized {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 16 bit integer to `self` in big-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u16_be(0x0809);
    /// assert_eq!(buf, b"\x08\x09");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_u16_be(&mut self, n: u16) {
        let mut buf = [0; 2];
        BigEndian::write_u16(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 16 bit integer to `self` in little-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u16_le(0x0809);
    /// assert_eq!(buf, b"\x09\x08");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_u16_le(&mut self, n: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, n);
        self.put_slice(&buf)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_i16_be or put_i16_le")]
    fn put_i16<T: ByteOrder>(&mut self, n: i16) where Self: Sized {
        let mut buf = [0; 2];
        T::write_i16(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 16 bit integer to `self` in big-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i16_be(0x0809);
    /// assert_eq!(buf, b"\x08\x09");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_i16_be(&mut self, n: i16) {
        let mut buf = [0; 2];
        BigEndian::write_i16(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 16 bit integer to `self` in little-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i16_le(0x0809);
    /// assert_eq!(buf, b"\x09\x08");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_i16_le(&mut self, n: i16) {
        let mut buf = [0; 2];
        LittleEndian::write_i16(&mut buf, n);
        self.put_slice(&buf)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_u32_be or put_u32_le")]
    fn put_u32<T: ByteOrder>(&mut self, n: u32) where Self: Sized {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 32 bit integer to `self` in big-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u32_be(0x0809A0A1);
    /// assert_eq!(buf, b"\x08\x09\xA0\xA1");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_u32_be(&mut self, n: u32) {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 32 bit integer to `self` in little-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u32_le(0x0809A0A1);
    /// assert_eq!(buf, b"\xA1\xA0\x09\x08");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_u32_le(&mut self, n: u32) {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, n);
        self.put_slice(&buf)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_i32_be or put_i32_le")]
    fn put_i32<T: ByteOrder>(&mut self, n: i32) where Self: Sized {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 32 bit integer to `self` in big-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i32_be(0x0809A0A1);
    /// assert_eq!(buf, b"\x08\x09\xA0\xA1");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_i32_be(&mut self, n: i32) {
        let mut buf = [0; 4];
        BigEndian::write_i32(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 32 bit integer to `self` in little-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i32_le(0x0809A0A1);
    /// assert_eq!(buf, b"\xA1\xA0\x09\x08");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_i32_le(&mut self, n: i32) {
        let mut buf = [0; 4];
        LittleEndian::write_i32(&mut buf, n);
        self.put_slice(&buf)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_u64_be or put_u64_le")]
    fn put_u64<T: ByteOrder>(&mut self, n: u64) where Self: Sized {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 64 bit integer to `self` in the big-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u64_be(0x0102030405060708);
    /// assert_eq!(buf, b"\x01\x02\x03\x04\x05\x06\x07\x08");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_u64_be(&mut self, n: u64) {
        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 64 bit integer to `self` in little-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u64_le(0x0102030405060708);
    /// assert_eq!(buf, b"\x08\x07\x06\x05\x04\x03\x02\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_u64_le(&mut self, n: u64) {
        let mut buf = [0; 8];
        LittleEndian::write_u64(&mut buf, n);
        self.put_slice(&buf)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_i64_be or put_i64_le")]
    fn put_i64<T: ByteOrder>(&mut self, n: i64) where Self: Sized {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 64 bit integer to `self` in the big-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i64_be(0x0102030405060708);
    /// assert_eq!(buf, b"\x01\x02\x03\x04\x05\x06\x07\x08");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_i64_be(&mut self, n: i64) {
        let mut buf = [0; 8];
        BigEndian::write_i64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 64 bit integer to `self` in little-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i64_le(0x0102030405060708);
    /// assert_eq!(buf, b"\x08\x07\x06\x05\x04\x03\x02\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_i64_le(&mut self, n: i64) {
        let mut buf = [0; 8];
        LittleEndian::write_i64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 128 bit integer to `self` in the big-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u128_be(0x01020304050607080910111213141516);
    /// assert_eq!(buf, b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    #[cfg(feature = "i128")]
    fn put_u128_be(&mut self, n: u128) {
        let mut buf = [0; 16];
        BigEndian::write_u128(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes an unsigned 128 bit integer to `self` in little-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_u128_le(0x01020304050607080910111213141516);
    /// assert_eq!(buf, b"\x16\x15\x14\x13\x12\x11\x10\x09\x08\x07\x06\x05\x04\x03\x02\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    #[cfg(feature = "i128")]
    fn put_u128_le(&mut self, n: u128) {
        let mut buf = [0; 16];
        LittleEndian::write_u128(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 128 bit integer to `self` in the big-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i128_be(0x01020304050607080910111213141516);
    /// assert_eq!(buf, b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    #[cfg(feature = "i128")]
    fn put_i128_be(&mut self, n: i128) {
        let mut buf = [0; 16];
        BigEndian::write_i128(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes a signed 128 bit integer to `self` in little-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_i128_le(0x01020304050607080910111213141516);
    /// assert_eq!(buf, b"\x16\x15\x14\x13\x12\x11\x10\x09\x08\x07\x06\x05\x04\x03\x02\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    #[cfg(feature = "i128")]
    fn put_i128_le(&mut self, n: i128) {
        let mut buf = [0; 16];
        LittleEndian::write_i128(&mut buf, n);
        self.put_slice(&buf)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_uint_be or put_uint_le")]
    fn put_uint<T: ByteOrder>(&mut self, n: u64, nbytes: usize) where Self: Sized {
        let mut buf = [0; 8];
        T::write_uint(&mut buf, n, nbytes);
        self.put_slice(&buf[0..nbytes])
    }

    /// Writes an unsigned n-byte integer to `self` in big-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_uint_be(0x010203, 3);
    /// assert_eq!(buf, b"\x01\x02\x03");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_uint_be(&mut self, n: u64, nbytes: usize) {
        let mut buf = [0; 8];
        BigEndian::write_uint(&mut buf, n, nbytes);
        self.put_slice(&buf[0..nbytes])
    }

    /// Writes an unsigned n-byte integer to `self` in the little-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_uint_le(0x010203, 3);
    /// assert_eq!(buf, b"\x03\x02\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_uint_le(&mut self, n: u64, nbytes: usize) {
        let mut buf = [0; 8];
        LittleEndian::write_uint(&mut buf, n, nbytes);
        self.put_slice(&buf[0..nbytes])
    }

    #[doc(hidden)]
    #[deprecated(note="use put_int_be or put_int_le")]
    fn put_int<T: ByteOrder>(&mut self, n: i64, nbytes: usize) where Self: Sized {
        let mut buf = [0; 8];
        T::write_int(&mut buf, n, nbytes);
        self.put_slice(&buf[0..nbytes])
    }

    /// Writes a signed n-byte integer to `self` in big-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_int_be(0x010203, 3);
    /// assert_eq!(buf, b"\x01\x02\x03");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_int_be(&mut self, n: i64, nbytes: usize) {
        let mut buf = [0; 8];
        BigEndian::write_int(&mut buf, n, nbytes);
        self.put_slice(&buf[0..nbytes])
    }

    /// Writes a signed n-byte integer to `self` in little-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_int_le(0x010203, 3);
    /// assert_eq!(buf, b"\x03\x02\x01");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_int_le(&mut self, n: i64, nbytes: usize) {
        let mut buf = [0; 8];
        LittleEndian::write_int(&mut buf, n, nbytes);
        self.put_slice(&buf[0..nbytes])
    }

    #[doc(hidden)]
    #[deprecated(note="use put_f32_be or put_f32_le")]
    fn put_f32<T: ByteOrder>(&mut self, n: f32) where Self: Sized {
        let mut buf = [0; 4];
        T::write_f32(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes  an IEEE754 single-precision (4 bytes) floating point number to
    /// `self` in big-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_f32_be(1.2f32);
    /// assert_eq!(buf, b"\x3F\x99\x99\x9A");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_f32_be(&mut self, n: f32) {
        let mut buf = [0; 4];
        BigEndian::write_f32(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes  an IEEE754 single-precision (4 bytes) floating point number to
    /// `self` in little-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_f32_le(1.2f32);
    /// assert_eq!(buf, b"\x9A\x99\x99\x3F");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_f32_le(&mut self, n: f32) {
        let mut buf = [0; 4];
        LittleEndian::write_f32(&mut buf, n);
        self.put_slice(&buf)
    }

    #[doc(hidden)]
    #[deprecated(note="use put_f64_be or put_f64_le")]
    fn put_f64<T: ByteOrder>(&mut self, n: f64) where Self: Sized {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes  an IEEE754 double-precision (8 bytes) floating point number to
    /// `self` in big-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_f64_be(1.2f64);
    /// assert_eq!(buf, b"\x3F\xF3\x33\x33\x33\x33\x33\x33");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_f64_be(&mut self, n: f64) {
        let mut buf = [0; 8];
        BigEndian::write_f64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Writes  an IEEE754 double-precision (8 bytes) floating point number to
    /// `self` in little-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    ///
    /// let mut buf = vec![];
    /// buf.put_f64_le(1.2f64);
    /// assert_eq!(buf, b"\x33\x33\x33\x33\x33\x33\xF3\x3F");
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining capacity in
    /// `self`.
    fn put_f64_le(&mut self, n: f64) {
        let mut buf = [0; 8];
        LittleEndian::write_f64(&mut buf, n);
        self.put_slice(&buf)
    }

    /// Creates a "by reference" adaptor for this instance of `BufMut`.
    ///
    /// The returned adapter also implements `BufMut` and will simply borrow
    /// `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    /// use std::io;
    ///
    /// let mut buf = vec![];
    ///
    /// {
    ///     let mut reference = buf.by_ref();
    ///
    ///     // Adapt reference to `std::io::Write`.
    ///     let mut writer = reference.writer();
    ///
    ///     // Use the buffer as a writter
    ///     io::Write::write(&mut writer, &b"hello world"[..]).unwrap();
    /// } // drop our &mut reference so that we can use `buf` again
    ///
    /// assert_eq!(buf, &b"hello world"[..]);
    /// ```
    fn by_ref(&mut self) -> &mut Self where Self: Sized {
        self
    }

    /// Creates an adaptor which implements the `Write` trait for `self`.
    ///
    /// This function returns a new value which implements `Write` by adapting
    /// the `Write` trait functions to the `BufMut` trait functions. Given that
    /// `BufMut` operations are infallible, none of the `Write` functions will
    /// return with `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BufMut;
    /// use std::io::Write;
    ///
    /// let mut buf = vec![].writer();
    ///
    /// let num = buf.write(&b"hello world"[..]).unwrap();
    /// assert_eq!(11, num);
    ///
    /// let buf = buf.into_inner();
    ///
    /// assert_eq!(*buf, b"hello world"[..]);
    /// ```
    fn writer(self) -> Writer<Self> where Self: Sized {
        super::writer::new(self)
    }
}

impl<'a, T: BufMut + ?Sized> BufMut for &'a mut T {
    fn remaining_mut(&self) -> usize {
        (**self).remaining_mut()
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        (**self).bytes_mut()
    }

    unsafe fn bytes_vec_mut<'b>(&'b mut self, dst: &mut [&'b mut IoVec]) -> usize {
        (**self).bytes_vec_mut(dst)
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        (**self).advance_mut(cnt)
    }
}

impl<T: BufMut + ?Sized> BufMut for Box<T> {
    fn remaining_mut(&self) -> usize {
        (**self).remaining_mut()
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        (**self).bytes_mut()
    }

    unsafe fn bytes_vec_mut<'b>(&'b mut self, dst: &mut [&'b mut IoVec]) -> usize {
        (**self).bytes_vec_mut(dst)
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        (**self).advance_mut(cnt)
    }
}

impl<T: AsMut<[u8]> + AsRef<[u8]>> BufMut for io::Cursor<T> {
    fn remaining_mut(&self) -> usize {
        use Buf;
        self.remaining()
    }

    /// Advance the internal cursor of the BufMut
    unsafe fn advance_mut(&mut self, cnt: usize) {
        use Buf;
        self.advance(cnt);
    }

    /// Returns a mutable slice starting at the current BufMut position and of
    /// length between 0 and `BufMut::remaining()`.
    ///
    /// The returned byte slice may represent uninitialized memory.
    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        let len = self.get_ref().as_ref().len();
        let pos = self.position() as usize;

        if pos >= len {
            return Default::default();
        }

        &mut (self.get_mut().as_mut())[pos..]
    }
}

impl BufMut for Vec<u8> {
    #[inline]
    fn remaining_mut(&self) -> usize {
        usize::MAX - self.len()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let len = self.len();
        let remaining = self.capacity() - len;
        if cnt > remaining {
            // Reserve additional capacity, and ensure that the total length
            // will not overflow usize.
            self.reserve(cnt);
        }

        self.set_len(len + cnt);
    }

    #[inline]
    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        use std::slice;

        if self.capacity() == self.len() {
            self.reserve(64); // Grow the vec
        }

        let cap = self.capacity();
        let len = self.len();

        let ptr = self.as_mut_ptr();
        &mut slice::from_raw_parts_mut(ptr, cap)[len..]
    }
}

// The existance of this function makes the compiler catch if the BufMut
// trait is "object-safe" or not.
fn _assert_trait_object(_b: &BufMut) {}
