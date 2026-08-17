#![feature(test)]

extern crate test;
extern crate shingles;

use test::Bencher;
use shingles::*;

#[bench]
fn bench_num_windows(b: &mut Bencher) {
    const N: usize = 5;
    let v: Vec<_> = (0..1000).collect();
    b.iter(|| {
        let n = test::black_box(N);
        for _ in v.windows(n) {}
    });
}

#[bench]
fn bench_num_shingles(b: &mut Bencher) {
    const N: usize = 5;
    let v: Vec<_> = (0..1000).collect();
    b.iter(|| {
        let n = test::black_box(N);
        for _ in v.as_shingles(n) {}
    });
}

#[bench]
fn bench_num_step_windows(b: &mut Bencher) {
    const N: usize = 5;
    const STEP: usize = 3;
    let v: Vec<_> = (0..1000).collect();
    b.iter(|| {
        let n = test::black_box(N);
        for _ in v.windows(n).enumerate()
            .filter(|&(i, _)| i % STEP == 0)
            .map(|(_, s)| s) {}
    });
}

#[bench]
fn bench_num_step_shingles(b: &mut Bencher) {
    const N: usize = 5;
    const STEP: usize = 3;
    let v: Vec<_> = (0..1000).collect();
    b.iter(|| {
        let n = test::black_box(N);
        for _ in v.as_shingles_with_step(n, STEP) {}
    });
}

#[bench]
fn bench_str_cmp(b: &mut Bencher) {
    const N: usize = 4;
    let mut s = String::new();
    for _ in 0..1000 {
        s.push_str("hello");
    }

    b.iter(|| {
        let mut res = vec![];
        let chars: Vec<_> = s.chars().collect();
        for shingle in chars.windows(N) {
            let mut sh = String::with_capacity(N);
            for &i in shingle {
                sh.push(i);
            }
            res.push(sh);
        }
        res
    });
}

#[bench]
fn bench_str_shingles(b: &mut Bencher) {
    const N: usize = 4;
    let mut s = String::new();
    for _ in 0..1000 {
        s.push_str("hello");
    }

    b.iter(|| s.as_shingles(N).collect::<Vec<_>>());
}