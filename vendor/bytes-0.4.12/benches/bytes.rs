#![feature(test)]

extern crate bytes;
extern crate test;

use test::Bencher;
use bytes::{Bytes, BytesMut, BufMut};

#[bench]
fn alloc_small(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1024 {
            test::black_box(BytesMut::with_capacity(12));
        }
    })
}

#[bench]
fn alloc_mid(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(BytesMut::with_capacity(128));
    })
}

#[bench]
fn alloc_big(b: &mut Bencher) {
    b.iter(|| {
        test::black_box(BytesMut::with_capacity(4096));
    })
}

#[bench]
fn split_off_and_drop(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..1024 {
            let v = vec![10; 200];
            let mut b = Bytes::from(v);
            test::black_box(b.split_off(100));
            test::black_box(b);
        }
    })
}

#[bench]
fn deref_unique(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(4096);
    buf.put(&[0u8; 1024][..]);

    b.iter(|| {
        for _ in 0..1024 {
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn deref_unique_unroll(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(4096);
    buf.put(&[0u8; 1024][..]);

    b.iter(|| {
        for _ in 0..128 {
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn deref_shared(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(4096);
    buf.put(&[0u8; 1024][..]);
    let _b2 = buf.split_off(1024);

    b.iter(|| {
        for _ in 0..1024 {
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn deref_inline(b: &mut Bencher) {
    let mut buf = BytesMut::with_capacity(8);
    buf.put(&[0u8; 8][..]);

    b.iter(|| {
        for _ in 0..1024 {
            test::black_box(&buf[..]);
        }
    })
}

#[bench]
fn deref_two(b: &mut Bencher) {
    let mut buf1 = BytesMut::with_capacity(8);
    buf1.put(&[0u8; 8][..]);

    let mut buf2 = BytesMut::with_capacity(4096);
    buf2.put(&[0u8; 1024][..]);

    b.iter(|| {
        for _ in 0..512 {
            test::black_box(&buf1[..]);
            test::black_box(&buf2[..]);
        }
    })
}

#[bench]
fn clone_inline(b: &mut Bencher) {
    let bytes = Bytes::from_static(b"hello world");

    b.iter(|| {
        for _ in 0..1024 {
            test::black_box(&bytes.clone());
        }
    })
}

#[bench]
fn clone_static(b: &mut Bencher) {
    let bytes = Bytes::from_static("hello world 1234567890 and have a good byte 0987654321".as_bytes());

    b.iter(|| {
        for _ in 0..1024 {
            test::black_box(&bytes.clone());
        }
    })
}

#[bench]
fn clone_arc(b: &mut Bencher) {
    let bytes = Bytes::from("hello world 1234567890 and have a good byte 0987654321".as_bytes());

    b.iter(|| {
        for _ in 0..1024 {
            test::black_box(&bytes.clone());
        }
    })
}

#[bench]
fn alloc_write_split_to_mid(b: &mut Bencher) {
    b.iter(|| {
        let mut buf = BytesMut::with_capacity(128);
        buf.put_slice(&[0u8; 64]);
        test::black_box(buf.split_to(64));
    })
}

#[bench]
fn drain_write_drain(b: &mut Bencher) {
    let data = [0u8; 128];

    b.iter(|| {
        let mut buf = BytesMut::with_capacity(1024);
        let mut parts = Vec::with_capacity(8);

        for _ in 0..8 {
            buf.put(&data[..]);
            parts.push(buf.split_to(128));
        }

        test::black_box(parts);
    })
}

#[bench]
fn fmt_write(b: &mut Bencher) {
    use std::fmt::Write;
    let mut buf = BytesMut::with_capacity(128);
    let s = "foo bar baz quux lorem ipsum dolor et";

    b.bytes = s.len() as u64;
    b.iter(|| {
        let _ = write!(buf, "{}", s);
        test::black_box(&buf);
        unsafe { buf.set_len(0); }
    })
}

#[bench]
fn from_long_slice(b: &mut Bencher) {
    let data = [0u8; 128];
    b.bytes = data.len() as u64;
    b.iter(|| {
        let buf = BytesMut::from(&data[..]);
        test::black_box(buf);
    })
}

#[bench]
fn slice_empty(b: &mut Bencher) {
    b.iter(|| {
        let b = Bytes::from(vec![17; 1024]).clone();
        for i in 0..1000 {
            test::black_box(b.slice(i % 100, i % 100));
        }
    })
}

#[bench]
fn slice_short_from_arc(b: &mut Bencher) {
    b.iter(|| {
        // `clone` is to convert to ARC
        let b = Bytes::from(vec![17; 1024]).clone();
        for i in 0..1000 {
            test::black_box(b.slice(1, 2 + i % 10));
        }
    })
}

// Keep in sync with bytes.rs
#[cfg(target_pointer_width = "64")]
const INLINE_CAP: usize = 4 * 8 - 1;
#[cfg(target_pointer_width = "32")]
const INLINE_CAP: usize = 4 * 4 - 1;

#[bench]
fn slice_avg_le_inline_from_arc(b: &mut Bencher) {
    b.iter(|| {
        // `clone` is to convert to ARC
        let b = Bytes::from(vec![17; 1024]).clone();
        for i in 0..1000 {
            // [1, INLINE_CAP]
            let len = 1 + i % (INLINE_CAP - 1);
            test::black_box(b.slice(i % 10, i % 10 + len));
        }
    })
}

#[bench]
fn slice_large_le_inline_from_arc(b: &mut Bencher) {
    b.iter(|| {
        // `clone` is to convert to ARC
        let b = Bytes::from(vec![17; 1024]).clone();
        for i in 0..1000 {
            // [INLINE_CAP - 10, INLINE_CAP]
            let len = INLINE_CAP - 9 + i % 10;
            test::black_box(b.slice(i % 10, i % 10 + len));
        }
    })
}
