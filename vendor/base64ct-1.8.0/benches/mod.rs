//! `base64ct` benchmarks

#![feature(test)]
extern crate test;

use base64ct::{Base64Unpadded, Encoding};
use test::Bencher;

const B64_LEN: usize = 100_002;
const RAW_LEN: usize = (3 * B64_LEN) / 4;

#[inline(never)]
fn get_raw_data() -> Vec<u8> {
    (0..RAW_LEN).map(|i| i as u8).collect()
}

#[inline(never)]
fn get_b64_data() -> String {
    (0..B64_LEN)
        .map(|i| match (i % 64) as u8 {
            v @ 0..=25 => (v + b'A') as char,
            v @ 26..=51 => (v - 26 + b'a') as char,
            v @ 52..=61 => (v - 52 + b'0') as char,
            62 => '+',
            _ => '/',
        })
        .collect()
}

#[bench]
fn decode_bench(b: &mut Bencher) {
    let b64_data = get_b64_data();
    let mut buf = get_raw_data();
    b.iter(|| {
        let out = Base64Unpadded::decode(&b64_data, &mut buf).unwrap();
        test::black_box(out);
    });
    b.bytes = RAW_LEN as u64;
}

#[bench]
fn decode_in_place_bench(b: &mut Bencher) {
    let mut b64_data = get_b64_data().into_bytes();
    b.iter(|| {
        // since it works on the same buffer over and over,
        // almost always `out` will be an error
        let out = Base64Unpadded::decode_in_place(&mut b64_data);
        let _ = test::black_box(out);
    });
    b.bytes = RAW_LEN as u64;
}

#[bench]
fn encode_bench(b: &mut Bencher) {
    let mut buf = get_b64_data().into_bytes();
    let raw_data = get_raw_data();
    b.iter(|| {
        let out = Base64Unpadded::encode(&raw_data, &mut buf).unwrap();
        test::black_box(out);
    });
    b.bytes = RAW_LEN as u64;
}
