#![feature(test)]

extern crate test;

use async_std::task;
use async_std::task_local;
use test::{black_box, Bencher};

#[bench]
fn get(b: &mut Bencher) {
    task_local! {
        static VAL: u64 = 1;
    }

    let mut sum = 0;
    task::block_on(async {
        b.iter(|| VAL.with(|v| sum += v));
    });
    black_box(sum);
}
