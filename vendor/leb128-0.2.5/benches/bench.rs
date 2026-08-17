#![feature(test)]

extern crate test;
use leb128;

#[bench]
fn write_signed(b: &mut test::Bencher) {
    let mut buf = [0; 4096];

    b.iter(|| {
        let mut writable = &mut buf[..];
        for i in -1025..1025 {
            test::black_box(leb128::write::signed(&mut writable, i).unwrap());
        }
    });
}

#[bench]
fn write_unsigned(b: &mut test::Bencher) {
    let mut buf = [0; 4096];

    b.iter(|| {
        let mut writable = &mut buf[..];
        for i in 0..2050 {
            test::black_box(leb128::write::unsigned(&mut writable, i).unwrap());
        }
    });
}

#[bench]
fn read_signed(b: &mut test::Bencher) {
    let mut buf = [0; 4096];

    {
        let mut writable = &mut buf[..];
        for i in -1025..1025 {
            leb128::write::signed(&mut writable, i).unwrap();
        }
    }

    b.iter(|| {
        let mut readable = &buf[..];
        for _ in -1025..1025 {
            test::black_box(leb128::read::signed(&mut readable).unwrap());
        }
    });
}

#[bench]
fn read_unsigned(b: &mut test::Bencher) {
    let mut buf = [0; 4096];

    {
        let mut writable = &mut buf[..];
        for i in 0..2050 {
            leb128::write::unsigned(&mut writable, i).unwrap();
        }
    }

    b.iter(|| {
        let mut readable = &buf[..];
        for _ in 0..2050 {
            test::black_box(leb128::read::unsigned(&mut readable).unwrap());
        }
    });
}
