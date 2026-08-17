// Copyright 2012-2024 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]

extern crate test;

use bit_set::BitSet;
use bit_vec::BitVec;

use rand::RngCore;
use test::{black_box, Bencher};

const BENCH_BITS: usize = 1 << 14;
const BITS: usize = 32;

#[bench]
fn bench_bit_vecset_small(b: &mut Bencher) {
    let mut r = rand::rng();
    let mut bit_vec = BitSet::new();
    b.iter(|| {
        for _ in 0..100 {
            bit_vec.insert((r.next_u32() as usize) % BITS);
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_vecset_big(b: &mut Bencher) {
    let mut r = rand::rng();
    let mut bit_vec = BitSet::new();
    b.iter(|| {
        for _ in 0..100 {
            bit_vec.insert((r.next_u32() as usize) % BENCH_BITS);
        }
        black_box(&bit_vec);
    });
}

#[bench]
fn bench_bit_vecset_iter(b: &mut Bencher) {
    let bit_vec = BitSet::from_bit_vec(BitVec::from_fn(BENCH_BITS, |idx| idx % 3 == 0));
    b.iter(|| {
        let mut sum = 0;
        for idx in &bit_vec {
            sum += idx;
        }
        sum
    })
}
