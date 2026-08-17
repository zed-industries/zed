#![feature(test)]
extern crate test;

use digest::bench_update;
use md5::Md5;
use test::Bencher;

bench_update!(
    Md5::default();
    md5_10 10;
    md5_100 100;
    md5_1000 1000;
    md5_10000 10000;
);
