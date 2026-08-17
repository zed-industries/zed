#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

#[bench]
fn decode_lower(b: &mut Bencher) {
    let input = vec![b'1'; 1 << 14];
    let mut buf = vec![0u8; 1 << 13];

    b.iter(|| {
        let input = black_box(&input[..]);
        let res = base16ct::lower::decode(input, &mut buf).unwrap();
        black_box(res);
    });
    b.bytes = input.len() as u64;
}

#[bench]
fn decode_upper(b: &mut Bencher) {
    let input = vec![b'1'; 1 << 14];
    let mut buf = vec![0u8; 1 << 13];

    b.iter(|| {
        let input = black_box(&input[..]);
        let res = base16ct::upper::decode(input, &mut buf).unwrap();
        black_box(res);
    });
    b.bytes = input.len() as u64;
}

#[bench]
fn decode_mixed(b: &mut Bencher) {
    let input = vec![b'1'; 1 << 14];
    let mut buf = vec![0u8; 1 << 13];

    b.iter(|| {
        let input = black_box(&input[..]);
        let res = base16ct::mixed::decode(input, &mut buf).unwrap();
        black_box(res);
    });
    b.bytes = input.len() as u64;
}

#[bench]
fn encode_lower(b: &mut Bencher) {
    let input = vec![0x42; 1 << 14];
    let mut buf = vec![0u8; 1 << 15];

    b.iter(|| {
        let input = black_box(&input[..]);
        let res = base16ct::lower::encode(input, &mut buf).unwrap();
        black_box(res);
    });
    b.bytes = input.len() as u64;
}

#[bench]
fn encode_upper(b: &mut Bencher) {
    let input = vec![0x42; 1 << 14];
    let mut buf = vec![0u8; 1 << 15];

    b.iter(|| {
        let input = black_box(&input[..]);
        let res = base16ct::upper::encode(input, &mut buf).unwrap();
        black_box(res);
    });
    b.bytes = input.len() as u64;
}
