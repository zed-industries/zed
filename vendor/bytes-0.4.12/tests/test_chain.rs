extern crate bytes;
extern crate iovec;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use bytes::buf::Chain;
use iovec::IoVec;
use std::io::Cursor;

#[test]
fn collect_two_bufs() {
    let a = Cursor::new(Bytes::from(&b"hello"[..]));
    let b = Cursor::new(Bytes::from(&b"world"[..]));

    let res: Vec<u8> = a.chain(b).collect();
    assert_eq!(res, &b"helloworld"[..]);
}

#[test]
fn writing_chained() {
    let mut a = BytesMut::with_capacity(64);
    let mut b = BytesMut::with_capacity(64);

    {
        let mut buf = Chain::new(&mut a, &mut b);

        for i in 0..128 {
            buf.put(i as u8);
        }
    }

    assert_eq!(64, a.len());
    assert_eq!(64, b.len());

    for i in 0..64 {
        let expect = i as u8;
        assert_eq!(expect, a[i]);
        assert_eq!(expect + 64, b[i]);
    }
}

#[test]
fn iterating_two_bufs() {
    let a = Cursor::new(Bytes::from(&b"hello"[..]));
    let b = Cursor::new(Bytes::from(&b"world"[..]));

    let res: Vec<u8> = a.chain(b).iter().collect();
    assert_eq!(res, &b"helloworld"[..]);
}

#[test]
fn vectored_read() {
    let a = Cursor::new(Bytes::from(&b"hello"[..]));
    let b = Cursor::new(Bytes::from(&b"world"[..]));

    let mut buf = a.chain(b);

    {
        let b1: &[u8] = &mut [0];
        let b2: &[u8] = &mut [0];
        let b3: &[u8] = &mut [0];
        let b4: &[u8] = &mut [0];
        let mut iovecs: [&IoVec; 4] =
            [b1.into(), b2.into(), b3.into(), b4.into()];

        assert_eq!(2, buf.bytes_vec(&mut iovecs));
        assert_eq!(iovecs[0][..], b"hello"[..]);
        assert_eq!(iovecs[1][..], b"world"[..]);
        assert_eq!(iovecs[2][..], b"\0"[..]);
        assert_eq!(iovecs[3][..], b"\0"[..]);
    }

    buf.advance(2);

    {
        let b1: &[u8] = &mut [0];
        let b2: &[u8] = &mut [0];
        let b3: &[u8] = &mut [0];
        let b4: &[u8] = &mut [0];
        let mut iovecs: [&IoVec; 4] =
            [b1.into(), b2.into(), b3.into(), b4.into()];

        assert_eq!(2, buf.bytes_vec(&mut iovecs));
        assert_eq!(iovecs[0][..], b"llo"[..]);
        assert_eq!(iovecs[1][..], b"world"[..]);
        assert_eq!(iovecs[2][..], b"\0"[..]);
        assert_eq!(iovecs[3][..], b"\0"[..]);
    }

    buf.advance(3);

    {
        let b1: &[u8] = &mut [0];
        let b2: &[u8] = &mut [0];
        let b3: &[u8] = &mut [0];
        let b4: &[u8] = &mut [0];
        let mut iovecs: [&IoVec; 4] =
            [b1.into(), b2.into(), b3.into(), b4.into()];

        assert_eq!(1, buf.bytes_vec(&mut iovecs));
        assert_eq!(iovecs[0][..], b"world"[..]);
        assert_eq!(iovecs[1][..], b"\0"[..]);
        assert_eq!(iovecs[2][..], b"\0"[..]);
        assert_eq!(iovecs[3][..], b"\0"[..]);
    }

    buf.advance(3);

    {
        let b1: &[u8] = &mut [0];
        let b2: &[u8] = &mut [0];
        let b3: &[u8] = &mut [0];
        let b4: &[u8] = &mut [0];
        let mut iovecs: [&IoVec; 4] =
            [b1.into(), b2.into(), b3.into(), b4.into()];

        assert_eq!(1, buf.bytes_vec(&mut iovecs));
        assert_eq!(iovecs[0][..], b"ld"[..]);
        assert_eq!(iovecs[1][..], b"\0"[..]);
        assert_eq!(iovecs[2][..], b"\0"[..]);
        assert_eq!(iovecs[3][..], b"\0"[..]);
    }
}
