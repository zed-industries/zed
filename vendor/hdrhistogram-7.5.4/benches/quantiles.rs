#![feature(test)]

extern crate test;

use hdrhistogram::*;
use test::{black_box, Bencher};

#[bench]
fn quantiles_below(b: &mut Bencher) {
    let mut h = Histogram::<u32>::new_with_bounds(1, 100_000, 3).unwrap();
    for i in 0..100_000 {
        h.record(i).unwrap();
    }

    b.iter(|| {
        black_box(h.quantile_below(black_box(10)));
        black_box(h.quantile_below(black_box(90_000)));
    })
}
