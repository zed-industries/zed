extern crate bytes;
extern crate byteorder;
extern crate iovec;

use bytes::{BufMut, BytesMut};
use iovec::IoVec;
use std::usize;
use std::fmt::Write;

#[test]
fn test_vec_as_mut_buf() {
    let mut buf = Vec::with_capacity(64);

    assert_eq!(buf.remaining_mut(), usize::MAX);

    unsafe {
        assert!(buf.bytes_mut().len() >= 64);
    }

    buf.put(&b"zomg"[..]);

    assert_eq!(&buf, b"zomg");

    assert_eq!(buf.remaining_mut(), usize::MAX - 4);
    assert_eq!(buf.capacity(), 64);

    for _ in 0..16 {
        buf.put(&b"zomg"[..]);
    }

    assert_eq!(buf.len(), 68);
}

#[test]
fn test_put_u8() {
    let mut buf = Vec::with_capacity(8);
    buf.put::<u8>(33);
    assert_eq!(b"\x21", &buf[..]);
}

#[test]
fn test_put_u16() {
    let mut buf = Vec::with_capacity(8);
    buf.put_u16_be(8532);
    assert_eq!(b"\x21\x54", &buf[..]);

    buf.clear();
    buf.put_u16_le(8532);
    assert_eq!(b"\x54\x21", &buf[..]);
}

#[test]
fn test_vec_advance_mut() {
    // Regression test for carllerche/bytes#108.
    let mut buf = Vec::with_capacity(8);
    unsafe {
        buf.advance_mut(12);
        assert_eq!(buf.len(), 12);
        assert!(buf.capacity() >= 12, "capacity: {}", buf.capacity());
    }
}

#[test]
fn test_clone() {
    let mut buf = BytesMut::with_capacity(100);
    buf.write_str("this is a test").unwrap();
    let buf2 = buf.clone();

    buf.write_str(" of our emergecy broadcast system").unwrap();
    assert!(buf != buf2);
}

#[test]
fn test_bufs_vec_mut() {
    use std::mem;

    let mut buf = BytesMut::from(&b"hello world"[..]);

    unsafe {
        let mut dst: [&mut IoVec; 2] = mem::zeroed();
        assert_eq!(1, buf.bytes_vec_mut(&mut dst[..]));
    }
}
