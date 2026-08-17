#![feature(test)]

extern crate test;
extern crate rand;

const RAND_BENCH_N: u64 = 1000;

mod distributions;

use std::mem::size_of;
use test::{black_box, Bencher};
use rand::{StdRng, Rng};

#[bench]
fn rand_f32(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.next_f32());
        }
    });
    b.bytes = size_of::<f32>() as u64 * RAND_BENCH_N;
}

#[bench]
fn rand_f64(b: &mut Bencher) {
    let mut rng = StdRng::new().unwrap();
    b.iter(|| {
        for _ in 0..RAND_BENCH_N {
            black_box(rng.next_f64());
        }
    });
    b.bytes = size_of::<f64>() as u64 * RAND_BENCH_N;
}
