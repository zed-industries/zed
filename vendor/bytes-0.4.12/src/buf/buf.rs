use super::{IntoBuf, Take, Reader, Iter, FromBuf, Chain};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use iovec::IoVec;

use std::{cmp, io, ptr};

macro_rules! buf_get_impl {
    ($this:ident, $size:expr, $conv:path) => ({
         // try to convert directly from the bytes
        let ret = {
            // this Option<ret> trick is to avoid keeping a borrow on self
            // when advance() is called (mut borrow) and to call bytes() only once
            if let Some(src) = $this.bytes().get(..($size)) {
                Some($conv(src))
            } else {
                None
            }
        };
        if let Some(ret) = ret {
             // if the direct convertion was possible, advance and return
            $this.advance($size);
            return ret;
        } else {
            // if not we copy the bytes in a temp buffer then convert
            let mut buf = [0; ($size)];
            $this.copy_to_slice(&mut buf); // (do the advance)
            return $conv(&buf);
        }
    });
    ($this:ident, $buf_size:expr, $conv:path, $len_to_read:expr) => ({
        // The same trick as above does not improve the best case speed.
        // It seems to be linked to the way the method is optimised by the compiler
        let mut buf = [0; ($buf_size)];
        $this.copy_to_slice(&mut buf[..($len_to_read)]);
        return $conv(&buf[..($len_to_read)], $len_to_read);
    });
}

/// Read bytes from a buffer.
///
/// A buffer stores bytes in memory such that read operations are infallible.
/// The underlying storage may or may not be in contiguous memory. A `Buf` value
/// is a cursor into the buffer. Reading from `Buf` advances the cursor
/// position. It can be thought of as an efficient `Iterator` for collections of
/// bytes.
///
/// The simplest `Buf` is a `Cursor` wrapping a `[u8]`.
///
/// ```
/// use bytes::Buf;
/// use std::io::Cursor;
///
/// let mut buf = Cursor::new(b"hello world");
///
/// assert_eq!(b'h', buf.get_u8());
/// assert_eq!(b'e', buf.get_u8());
/// assert_eq!(b'l', buf.get_u8());
///
/// let mut rest = [0; 8];
/// buf.copy_to_slice(&mut rest);
///
/// assert_eq!(&rest[..], b"lo world");
/// ```
pub trait Buf {
    /// Returns the number of bytes between the current position and the end of
    /// the buffer.
    ///
    /// This value is greater than or equal to the length of the slice returned
    /// by `bytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"hello world");
    ///
    /// assert_eq!(buf.remaining(), 11);
    ///
    /// buf.get_u8();
    ///
    /// assert_eq!(buf.remaining(), 10);
    /// ```
    ///
    /// # Implementer notes
    ///
    /// Implementations of `remaining` should ensure that the return value does
    /// not change unless a call is made to `advance` or any other function that
    /// is documented to change the `Buf`'s current position.
    fn remaining(&self) -> usize;

    /// Returns a slice starting at the current position and of length between 0
    /// and `Buf::remaining()`. Note that this *can* return shorter slice (this allows
    /// non-continuous internal representation).
    ///
    /// This is a lower level function. Most operations are done with other
    /// functions.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"hello world");
    ///
    /// assert_eq!(buf.bytes(), b"hello world");
    ///
    /// buf.advance(6);
    ///
    /// assert_eq!(buf.bytes(), b"world");
    /// ```
    ///
    /// # Implementer notes
    ///
    /// This function should never panic. Once the end of the buffer is reached,
    /// i.e., `Buf::remaining` returns 0, calls to `bytes` should return an
    /// empty slice.
    fn bytes(&self) -> &[u8];

    /// Fills `dst` with potentially multiple slices starting at `self`'s
    /// current position.
    ///
    /// If the `Buf` is backed by disjoint slices of bytes, `bytes_vec` enables
    /// fetching more than one slice at once. `dst` is a slice of `IoVec`
    /// references, enabling the slice to be directly used with [`writev`]
    /// without any further conversion. The sum of the lengths of all the
    /// buffers in `dst` will be less than or equal to `Buf::remaining()`.
    ///
    /// The entries in `dst` will be overwritten, but the data **contained** by
    /// the slices **will not** be modified. If `bytes_vec` does not fill every
    /// entry in `dst`, then `dst` is guaranteed to contain all remaining slices
    /// in `self.
    ///
    /// This is a lower level function. Most operations are done with other
    /// functions.
    ///
    /// # Implementer notes
    ///
    /// This function should never panic. Once the end of the buffer is reached,
    /// i.e., `Buf::remaining` returns 0, calls to `bytes_vec` must return 0
    /// without mutating `dst`.
    ///
    /// Implementations should also take care to properly handle being called
    /// with `dst` being a zero length slice.
    ///
    /// [`writev`]: http://man7.org/linux/man-pages/man2/readv.2.html
    fn bytes_vec<'a>(&'a self, dst: &mut [&'a IoVec]) -> usize {
        if dst.is_empty() {
            return 0;
        }

        if self.has_remaining() {
            dst[0] = self.bytes().into();
            1
        } else {
            0
        }
    }

    /// Advance the internal cursor of the Buf
    ///
    /// The next call to `bytes` will return a slice starting `cnt` bytes
    /// further into the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"hello world");
    ///
    /// assert_eq!(buf.bytes(), b"hello world");
    ///
    /// buf.advance(6);
    ///
    /// assert_eq!(buf.bytes(), b"world");
    /// ```
    ///
    /// # Panics
    ///
    /// This function **may** panic if `cnt > self.remaining()`.
    ///
    /// # Implementer notes
    ///
    /// It is recommended for implementations of `advance` to panic if `cnt >
    /// self.remaining()`. If the implementation does not panic, the call must
    /// behave as if `cnt == self.remaining()`.
    ///
    /// A call with `cnt == 0` should never panic and be a no-op.
    fn advance(&mut self, cnt: usize);

    /// Returns true if there are any more bytes to consume
    ///
    /// This is equivalent to `self.remaining() != 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"a");
    ///
    /// assert!(buf.has_remaining());
    ///
    /// buf.get_u8();
    ///
    /// assert!(!buf.has_remaining());
    /// ```
    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }

    /// Copies bytes from `self` into `dst`.
    ///
    /// The cursor is advanced by the number of bytes copied. `self` must have
    /// enough remaining bytes to fill `dst`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"hello world");
    /// let mut dst = [0; 5];
    ///
    /// buf.copy_to_slice(&mut dst);
    /// assert_eq!(b"hello", &dst);
    /// assert_eq!(6, buf.remaining());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if `self.remaining() < dst.len()`
    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        let mut off = 0;

        assert!(self.remaining() >= dst.len());

        while off < dst.len() {
            let cnt;

            unsafe {
                let src = self.bytes();
                cnt = cmp::min(src.len(), dst.len() - off);

                ptr::copy_nonoverlapping(
                    src.as_ptr(), dst[off..].as_mut_ptr(), cnt);

                off += src.len();
            }

            self.advance(cnt);
        }
    }

    /// Gets an unsigned 8 bit integer from `self`.
    ///
    /// The current position is advanced by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08 hello");
    /// assert_eq!(8, buf.get_u8());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is no more remaining data in `self`.
    fn get_u8(&mut self) -> u8 {
        assert!(self.remaining() >= 1);
        let ret = self.bytes()[0];
        self.advance(1);
        ret
    }

    /// Gets a signed 8 bit integer from `self`.
    ///
    /// The current position is advanced by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08 hello");
    /// assert_eq!(8, buf.get_i8());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is no more remaining data in `self`.
    fn get_i8(&mut self) -> i8 {
        assert!(self.remaining() >= 1);
        let ret = self.bytes()[0] as i8;
        self.advance(1);
        ret
    }

    #[doc(hidden)]
    #[deprecated(note="use get_u16_be or get_u16_le")]
    fn get_u16<T: ByteOrder>(&mut self) -> u16 where Self: Sized {
        let mut buf = [0; 2];
        self.copy_to_slice(&mut buf);
        T::read_u16(&buf)
    }

    /// Gets an unsigned 16 bit integer from `self` in big-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08\x09 hello");
    /// assert_eq!(0x0809, buf.get_u16_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_u16_be(&mut self) -> u16 {
        buf_get_impl!(self, 2, BigEndian::read_u16);
    }

    /// Gets an unsigned 16 bit integer from `self` in little-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x09\x08 hello");
    /// assert_eq!(0x0809, buf.get_u16_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_u16_le(&mut self) -> u16 {
        buf_get_impl!(self, 2, LittleEndian::read_u16);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_i16_be or get_i16_le")]
    fn get_i16<T: ByteOrder>(&mut self) -> i16 where Self: Sized {
        let mut buf = [0; 2];
        self.copy_to_slice(&mut buf);
        T::read_i16(&buf)
    }

    /// Gets a signed 16 bit integer from `self` in big-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08\x09 hello");
    /// assert_eq!(0x0809, buf.get_i16_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_i16_be(&mut self) -> i16 {
        buf_get_impl!(self, 2, BigEndian::read_i16);
    }

    /// Gets a signed 16 bit integer from `self` in little-endian byte order.
    ///
    /// The current position is advanced by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x09\x08 hello");
    /// assert_eq!(0x0809, buf.get_i16_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_i16_le(&mut self) -> i16 {
        buf_get_impl!(self, 2, LittleEndian::read_i16);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_u32_be or get_u32_le")]
    fn get_u32<T: ByteOrder>(&mut self) -> u32 where Self: Sized {
        let mut buf = [0; 4];
        self.copy_to_slice(&mut buf);
        T::read_u32(&buf)
    }

    /// Gets an unsigned 32 bit integer from `self` in the big-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08\x09\xA0\xA1 hello");
    /// assert_eq!(0x0809A0A1, buf.get_u32_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_u32_be(&mut self) -> u32 {
        buf_get_impl!(self, 4, BigEndian::read_u32);
    }

    /// Gets an unsigned 32 bit integer from `self` in the little-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\xA1\xA0\x09\x08 hello");
    /// assert_eq!(0x0809A0A1, buf.get_u32_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_u32_le(&mut self) -> u32 {
        buf_get_impl!(self, 4, LittleEndian::read_u32);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_i32_be or get_i32_le")]
    fn get_i32<T: ByteOrder>(&mut self) -> i32 where Self: Sized {
        let mut buf = [0; 4];
        self.copy_to_slice(&mut buf);
        T::read_i32(&buf)
    }

    /// Gets a signed 32 bit integer from `self` in big-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08\x09\xA0\xA1 hello");
    /// assert_eq!(0x0809A0A1, buf.get_i32_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_i32_be(&mut self) -> i32 {
        buf_get_impl!(self, 4, BigEndian::read_i32);
    }

    /// Gets a signed 32 bit integer from `self` in little-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\xA1\xA0\x09\x08 hello");
    /// assert_eq!(0x0809A0A1, buf.get_i32_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_i32_le(&mut self) -> i32 {
        buf_get_impl!(self, 4, LittleEndian::read_i32);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_u64_be or get_u64_le")]
    fn get_u64<T: ByteOrder>(&mut self) -> u64 where Self: Sized {
        let mut buf = [0; 8];
        self.copy_to_slice(&mut buf);
        T::read_u64(&buf)
    }

    /// Gets an unsigned 64 bit integer from `self` in big-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x01\x02\x03\x04\x05\x06\x07\x08 hello");
    /// assert_eq!(0x0102030405060708, buf.get_u64_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_u64_be(&mut self) -> u64 {
        buf_get_impl!(self, 8, BigEndian::read_u64);
    }

    /// Gets an unsigned 64 bit integer from `self` in little-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08\x07\x06\x05\x04\x03\x02\x01 hello");
    /// assert_eq!(0x0102030405060708, buf.get_u64_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_u64_le(&mut self) -> u64 {
        buf_get_impl!(self, 8, LittleEndian::read_u64);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_i64_be or get_i64_le")]
    fn get_i64<T: ByteOrder>(&mut self) -> i64 where Self: Sized {
        let mut buf = [0; 8];
        self.copy_to_slice(&mut buf);
        T::read_i64(&buf)
    }

    /// Gets a signed 64 bit integer from `self` in big-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x01\x02\x03\x04\x05\x06\x07\x08 hello");
    /// assert_eq!(0x0102030405060708, buf.get_i64_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_i64_be(&mut self) -> i64 {
        buf_get_impl!(self, 8, BigEndian::read_i64);
    }

    /// Gets a signed 64 bit integer from `self` in little-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x08\x07\x06\x05\x04\x03\x02\x01 hello");
    /// assert_eq!(0x0102030405060708, buf.get_i64_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_i64_le(&mut self) -> i64 {
        buf_get_impl!(self, 8, LittleEndian::read_i64);
    }

    /// Gets an unsigned 128 bit integer from `self` in big-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16 hello");
    /// assert_eq!(0x01020304050607080910111213141516, buf.get_u128_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    #[cfg(feature = "i128")]
    fn get_u128_be(&mut self) -> u128 {
        buf_get_impl!(self, 16, BigEndian::read_u128);
    }

    /// Gets an unsigned 128 bit integer from `self` in little-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x16\x15\x14\x13\x12\x11\x10\x09\x08\x07\x06\x05\x04\x03\x02\x01 hello");
    /// assert_eq!(0x01020304050607080910111213141516, buf.get_u128_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    #[cfg(feature = "i128")]
    fn get_u128_le(&mut self) -> u128 {
        buf_get_impl!(self, 16, LittleEndian::read_u128);
    }

    /// Gets a signed 128 bit integer from `self` in big-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16 hello");
    /// assert_eq!(0x01020304050607080910111213141516, buf.get_i128_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    #[cfg(feature = "i128")]
    fn get_i128_be(&mut self) -> i128 {
        buf_get_impl!(self, 16, BigEndian::read_i128);
    }

    /// Gets a signed 128 bit integer from `self` in little-endian byte order.
    ///
    /// **NOTE:** This method requires the `i128` feature.
    /// The current position is advanced by 16.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x16\x15\x14\x13\x12\x11\x10\x09\x08\x07\x06\x05\x04\x03\x02\x01 hello");
    /// assert_eq!(0x01020304050607080910111213141516, buf.get_i128_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    #[cfg(feature = "i128")]
    fn get_i128_le(&mut self) -> i128 {
        buf_get_impl!(self, 16, LittleEndian::read_i128);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_uint_be or get_uint_le")]
    fn get_uint<T: ByteOrder>(&mut self, nbytes: usize) -> u64 where Self: Sized {
        let mut buf = [0; 8];
        self.copy_to_slice(&mut buf[..nbytes]);
        T::read_uint(&buf[..nbytes], nbytes)
    }

    /// Gets an unsigned n-byte integer from `self` in big-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Buf, BigEndian};
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x01\x02\x03 hello");
    /// assert_eq!(0x010203, buf.get_uint_be(3));
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_uint_be(&mut self, nbytes: usize) -> u64 {
        buf_get_impl!(self, 8, BigEndian::read_uint, nbytes);
    }

    /// Gets an unsigned n-byte integer from `self` in little-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x03\x02\x01 hello");
    /// assert_eq!(0x010203, buf.get_uint_le(3));
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_uint_le(&mut self, nbytes: usize) -> u64 {
        buf_get_impl!(self, 8, LittleEndian::read_uint, nbytes);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_int_be or get_int_le")]
    fn get_int<T: ByteOrder>(&mut self, nbytes: usize) -> i64 where Self: Sized {
        let mut buf = [0; 8];
        self.copy_to_slice(&mut buf[..nbytes]);
        T::read_int(&buf[..nbytes], nbytes)
    }

    /// Gets a signed n-byte integer from `self` in big-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x01\x02\x03 hello");
    /// assert_eq!(0x010203, buf.get_int_be(3));
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_int_be(&mut self, nbytes: usize) -> i64 {
        buf_get_impl!(self, 8, BigEndian::read_int, nbytes);
    }

    /// Gets a signed n-byte integer from `self` in little-endian byte order.
    ///
    /// The current position is advanced by `nbytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x03\x02\x01 hello");
    /// assert_eq!(0x010203, buf.get_int_le(3));
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_int_le(&mut self, nbytes: usize) -> i64 {
        buf_get_impl!(self, 8, LittleEndian::read_int, nbytes);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_f32_be or get_f32_le")]
    fn get_f32<T: ByteOrder>(&mut self) -> f32 where Self: Sized {
        let mut buf = [0; 4];
        self.copy_to_slice(&mut buf);
        T::read_f32(&buf)
    }

    /// Gets an IEEE754 single-precision (4 bytes) floating point number from
    /// `self` in big-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x3F\x99\x99\x9A hello");
    /// assert_eq!(1.2f32, buf.get_f32_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_f32_be(&mut self) -> f32 {
        buf_get_impl!(self, 4, BigEndian::read_f32);
    }

    /// Gets an IEEE754 single-precision (4 bytes) floating point number from
    /// `self` in little-endian byte order.
    ///
    /// The current position is advanced by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x9A\x99\x99\x3F hello");
    /// assert_eq!(1.2f32, buf.get_f32_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_f32_le(&mut self) -> f32 {
        buf_get_impl!(self, 4, LittleEndian::read_f32);
    }

    #[doc(hidden)]
    #[deprecated(note="use get_f64_be or get_f64_le")]
    fn get_f64<T: ByteOrder>(&mut self) -> f64 where Self: Sized {
        let mut buf = [0; 8];
        self.copy_to_slice(&mut buf);
        T::read_f64(&buf)
    }

    /// Gets an IEEE754 double-precision (8 bytes) floating point number from
    /// `self` in big-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x3F\xF3\x33\x33\x33\x33\x33\x33 hello");
    /// assert_eq!(1.2f64, buf.get_f64_be());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_f64_be(&mut self) -> f64 {
        buf_get_impl!(self, 8, BigEndian::read_f64);
    }

    /// Gets an IEEE754 double-precision (8 bytes) floating point number from
    /// `self` in little-endian byte order.
    ///
    /// The current position is advanced by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Buf;
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new(b"\x33\x33\x33\x33\x33\x33\xF3\x3F hello");
    /// assert_eq!(1.2f64, buf.get_f64_le());
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if there is not enough remaining data in `self`.
    fn get_f64_le(&mut self) -> f64 {
        buf_get_impl!(self, 8, LittleEndian::read_f64);
    }

    /// Transforms a `Buf` into a concrete buffer.
    ///
    /// `collect()` can operate on any value that implements `Buf`, and turn it
    /// into the relevent concrete buffer type.
    ///
    /// # Examples
    ///
    /// Collecting a buffer and loading the contents into a `Vec<u8>`.
    ///
    /// ```
    /// use bytes::{Buf, Bytes, IntoBuf};
    ///
    /// let buf = Bytes::from(&b"hello world"[..]).into_buf();
    /// let vec: Vec<u8> = buf.collect();
    ///
    /// assert_eq!(vec, &b"hello world"[..]);
    /// ```
    fn collect<B>(self) -> B
        where Self: Sized,
              B: FromBuf,
    {
        B::from_buf(self)
    }

    /// Creates an adaptor which will read at most `limit` bytes from `self`.
    ///
    /// This function returns a new instance of `Buf` which will read at most
    /// `limit` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Buf, BufMut};
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new("hello world").take(5);
    /// let mut dst = vec![];
    ///
    /// dst.put(&mut buf);
    /// assert_eq!(dst, b"hello");
    ///
    /// let mut buf = buf.into_inner();
    /// dst.clear();
    /// dst.put(&mut buf);
    /// assert_eq!(dst, b" world");
    /// ```
    fn take(self, limit: usize) -> Take<Self>
        where Self: Sized
    {
        super::take::new(self, limit)
    }

    /// Creates an adaptor which will chain this buffer with another.
    ///
    /// The returned `Buf` instance will first consume all bytes from `self`.
    /// Afterwards the output is equivalent to the output of next.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Bytes, Buf, IntoBuf};
    /// use bytes::buf::Chain;
    ///
    /// let buf = Bytes::from(&b"hello "[..]).into_buf()
    ///             .chain(Bytes::from(&b"world"[..]));
    ///
    /// let full: Bytes = buf.collect();
    /// assert_eq!(full[..], b"hello world"[..]);
    /// ```
    fn chain<U>(self, next: U) -> Chain<Self, U::Buf>
        where U: IntoBuf,
              Self: Sized,
    {
        Chain::new(self, next.into_buf())
    }

    /// Creates a "by reference" adaptor for this instance of `Buf`.
    ///
    /// The returned adaptor also implements `Buf` and will simply borrow `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Buf, BufMut};
    /// use std::io::Cursor;
    ///
    /// let mut buf = Cursor::new("hello world");
    /// let mut dst = vec![];
    ///
    /// {
    ///     let mut reference = buf.by_ref();
    ///     dst.put(&mut reference.take(5));
    ///     assert_eq!(dst, b"hello");
    /// } // drop our &mut reference so we can use `buf` again
    ///
    /// dst.clear();
    /// dst.put(&mut buf);
    /// assert_eq!(dst, b" world");
    /// ```
    fn by_ref(&mut self) -> &mut Self where Self: Sized {
        self
    }

    /// Creates an adaptor which implements the `Read` trait for `self`.
    ///
    /// This function returns a new value which implements `Read` by adapting
    /// the `Read` trait functions to the `Buf` trait functions. Given that
    /// `Buf` operations are infallible, none of the `Read` functions will
    /// return with `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{Buf, IntoBuf, Bytes};
    /// use std::io::Read;
    ///
    /// let buf = Bytes::from("hello world").into_buf();
    ///
    /// let mut reader = buf.reader();
    /// let mut dst = [0; 1024];
    ///
    /// let num = reader.read(&mut dst).unwrap();
    ///
    /// assert_eq!(11, num);
    /// assert_eq!(&dst[..11], b"hello world");
    /// ```
    fn reader(self) -> Reader<Self> where Self: Sized {
        super::reader::new(self)
    }

    /// Returns an iterator over the bytes contained by the buffer.
    ///
    /// # Examples
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
    fn iter(self) -> Iter<Self> where Self: Sized {
        super::iter::new(self)
    }
}

impl<'a, T: Buf + ?Sized> Buf for &'a mut T {
    fn remaining(&self) -> usize {
        (**self).remaining()
    }

    fn bytes(&self) -> &[u8] {
        (**self).bytes()
    }

    fn bytes_vec<'b>(&'b self, dst: &mut [&'b IoVec]) -> usize {
        (**self).bytes_vec(dst)
    }

    fn advance(&mut self, cnt: usize) {
        (**self).advance(cnt)
    }
}

impl<T: Buf + ?Sized> Buf for Box<T> {
    fn remaining(&self) -> usize {
        (**self).remaining()
    }

    fn bytes(&self) -> &[u8] {
        (**self).bytes()
    }

    fn bytes_vec<'b>(&'b self, dst: &mut [&'b IoVec]) -> usize {
        (**self).bytes_vec(dst)
    }

    fn advance(&mut self, cnt: usize) {
        (**self).advance(cnt)
    }
}

impl<T: AsRef<[u8]>> Buf for io::Cursor<T> {
    fn remaining(&self) -> usize {
        let len = self.get_ref().as_ref().len();
        let pos = self.position();

        if pos >= len as u64 {
            return 0;
        }

        len - pos as usize
    }

    fn bytes(&self) -> &[u8] {
        let len = self.get_ref().as_ref().len();
        let pos = self.position() as usize;

        if pos >= len {
            return Default::default();
        }

        &(self.get_ref().as_ref())[pos..]
    }

    fn advance(&mut self, cnt: usize) {
        let pos = (self.position() as usize)
            .checked_add(cnt).expect("overflow");

        assert!(pos <= self.get_ref().as_ref().len());

        self.set_position(pos as u64);
    }
}

impl Buf for Option<[u8; 1]> {
    fn remaining(&self) -> usize {
        if self.is_some() {
            1
        } else {
            0
        }
    }

    fn bytes(&self) -> &[u8] {
        self.as_ref().map(AsRef::as_ref)
            .unwrap_or(Default::default())
    }

    fn advance(&mut self, cnt: usize) {
        if cnt == 0 {
            return;
        }

        if self.is_none() {
            panic!("overflow");
        } else {
            assert_eq!(1, cnt);
            *self = None;
        }
    }
}

// The existance of this function makes the compiler catch if the Buf
// trait is "object-safe" or not.
fn _assert_trait_object(_b: &Buf) {}
