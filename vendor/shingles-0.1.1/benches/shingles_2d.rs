#![feature(test)]

extern crate test;
extern crate shingles;

use test::Bencher;
use shingles::*;

#[bench]
fn bench_num_shingles_2d(b: &mut Bencher) {
    const SIZE: [usize; 2] = [3, 3];
    let a: Vec<_> = (1..1001).collect();
    let v: Vec<_> = a.chunks(100).collect();

    b.iter(|| {
        let size = test::black_box(SIZE);
        for _ in v.as_shingles_2d(size) {}
    });
}

#[bench]
fn bench_str_shingles_2d(b: &mut Bencher) {
    const SIZE: [usize; 2] = [3, 3];
    let mut s = String::new();
    for _ in 0..1000 {
        s.push_str("привет\n");
    }
    let v: Vec<_> = s.split_terminator("\n").collect();

    b.iter(|| {
        let size = test::black_box(SIZE);
        for _ in v.as_shingles_2d(size) {}
    });
}