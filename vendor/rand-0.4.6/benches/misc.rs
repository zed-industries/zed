#![feature(test)]

extern crate test;
extern crate rand;

use test::{black_box, Bencher};

use rand::{Rng, weak_rng};
use rand::seq::*;

#[bench]
fn misc_shuffle_100(b: &mut Bencher) {
    let mut rng = weak_rng();
    let x : &mut [usize] = &mut [1; 100];
    b.iter(|| {
        rng.shuffle(x);
        black_box(&x);
    })
}

#[bench]
fn misc_sample_iter_10_of_100(b: &mut Bencher) {
    let mut rng = weak_rng();
    let x : &[usize] = &[1; 100];
    b.iter(|| {
        black_box(sample_iter(&mut rng, x, 10).unwrap_or_else(|e| e));
    })
}

#[bench]
fn misc_sample_slice_10_of_100(b: &mut Bencher) {
    let mut rng = weak_rng();
    let x : &[usize] = &[1; 100];
    b.iter(|| {
        black_box(sample_slice(&mut rng, x, 10));
    })
}

#[bench]
fn misc_sample_slice_ref_10_of_100(b: &mut Bencher) {
    let mut rng = weak_rng();
    let x : &[usize] = &[1; 100];
    b.iter(|| {
        black_box(sample_slice_ref(&mut rng, x, 10));
    })
}

macro_rules! sample_indices {
    ($name:ident, $amount:expr, $length:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = weak_rng();
            b.iter(|| {
                black_box(sample_indices(&mut rng, $length, $amount));
            })
        }
    }
}

sample_indices!(misc_sample_indices_10_of_1k, 10, 1000);
sample_indices!(misc_sample_indices_50_of_1k, 50, 1000);
sample_indices!(misc_sample_indices_100_of_1k, 100, 1000);
