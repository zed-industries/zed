#![feature(test)]
extern crate rand_core;
extern crate rdrand;
extern crate test;

use rand_core::RngCore;
use test::Bencher;

#[bench]
fn bench_rdseed_u16(b : &mut Bencher) {
    if let Ok(gen) = rdrand::RdSeed::new() {
        b.bytes = 2;
        b.iter(|| {
            gen.try_next_u16().unwrap()
        });
    }
}

#[bench]
fn bench_rdseed_u32(b : &mut Bencher) {
    if let Ok(mut gen) = rdrand::RdSeed::new() {
        b.bytes = 4;
        b.iter(|| {
            gen.next_u32()
        });
    }
}

#[bench]
fn bench_rdseed_u64(b : &mut Bencher) {
    if let Ok(mut gen) = rdrand::RdSeed::new() {
        b.bytes = 8;
        b.iter(|| {
            gen.next_u64()
        });
    }
}

#[bench]
fn bench_fill(b : &mut Bencher) {
    if let Ok(mut gen) = rdrand::RdSeed::new() {
        let mut buffer = [0; 128];
        b.bytes = 128;
        b.iter(|| {
            gen.fill_bytes(&mut buffer);
            buffer
        });
    }
}
