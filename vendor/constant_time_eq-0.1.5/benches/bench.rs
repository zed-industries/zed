#![feature(test)]

extern crate constant_time_eq;
extern crate test;

use constant_time_eq::constant_time_eq;
use test::{Bencher, black_box};

fn bench(b: &mut Bencher, left: &[u8], right: &[u8]) {
    b.bytes = (left.len() + right.len()) as u64;
    b.iter(|| {
        constant_time_eq(black_box(left), black_box(right))
    })
}

#[bench]
fn bench_16(b: &mut Bencher) {
    bench(b, &[0; 16], &[0; 16])
}

#[bench]
fn bench_4096(b: &mut Bencher) {
    bench(b, &[0; 4096], &[0; 4096])
}

#[bench]
fn bench_65536(b: &mut Bencher) {
    bench(b, &[0; 65536], &[0; 65536])
}
