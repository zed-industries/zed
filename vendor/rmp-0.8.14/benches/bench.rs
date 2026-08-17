#![feature(test)]

extern crate test;

use test::Bencher;

use rmp::decode::*;

#[bench]
fn from_i64_read_i64(b: &mut Bencher) {
    let buf = [0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

    b.iter(|| {
        let res = read_i64(&mut &buf[..]).unwrap();
        test::black_box(res);
    });
}

#[bench]
fn from_i64_read_int(b: &mut Bencher) {
    let buf = [0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

    b.iter(|| {
        let res: i64 = read_int(&mut &buf[..]).unwrap();
        test::black_box(res);
    });
}

#[bench]
fn from_string_read_str(b: &mut Bencher) {
    // Lorem ipsum dolor sit amet.
    let buf = [
        0xbb, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73,
        0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73,
        0x69, 0x74, 0x20, 0x61, 0x6d, 0x65, 0x74, 0x2e
    ];

    let mut out = [0u8; 32];

    b.iter(|| {
        let res = read_str(&mut &buf[..], &mut out[..]).unwrap();
        test::black_box(res);
    });
}
