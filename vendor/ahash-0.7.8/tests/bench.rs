use ahash::{CallHasher, RandomState};
use criterion::*;
use fxhash::FxHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[cfg(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
    all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "crypto", not(miri), feature = "stdsimd")
))]
fn aeshash<H: Hash>(b: &H) -> u64 {
    let build_hasher = RandomState::with_seeds(1, 2, 3, 4);
    H::get_hash(b, &build_hasher)
}
#[cfg(not(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
    all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "crypto", not(miri), feature = "stdsimd")
)))]
fn aeshash<H: Hash>(_b: &H) -> u64 {
    panic!("aes must be enabled")
}

#[cfg(not(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
    all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "crypto", not(miri), feature = "stdsimd")
)))]
fn fallbackhash<H: Hash>(b: &H) -> u64 {
    let build_hasher = RandomState::with_seeds(1, 2, 3, 4);
    H::get_hash(b, &build_hasher)
}
#[cfg(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
    all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "crypto", not(miri), feature = "stdsimd")
))]
fn fallbackhash<H: Hash>(_b: &H) -> u64 {
    panic!("aes must be disabled")
}

fn fnvhash<H: Hash>(b: &H) -> u64 {
    let mut hasher = fnv::FnvHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn siphash<H: Hash>(b: &H) -> u64 {
    let mut hasher = DefaultHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn fxhash<H: Hash>(b: &H) -> u64 {
    let mut hasher = FxHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn seahash<H: Hash>(b: &H) -> u64 {
    let mut hasher = seahash::SeaHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

const STRING_LENGTHS: [u32; 12] = [1, 3, 4, 7, 8, 15, 16, 24, 33, 68, 132, 1024];

fn gen_strings() -> Vec<String> {
    STRING_LENGTHS
        .iter()
        .map(|len| {
            let mut string = String::default();
            for pos in 1..=*len {
                let c = (48 + (pos % 10) as u8) as char;
                string.push(c);
            }
            string
        })
        .collect()
}

const U8_VALUE: u8 = 123;
const U16_VALUE: u16 = 1234;
const U32_VALUE: u32 = 12345678;
const U64_VALUE: u64 = 1234567890123456;
const U128_VALUE: u128 = 12345678901234567890123456789012;

fn bench_ahash(c: &mut Criterion) {
    let mut group = c.benchmark_group("aeshash");
    group.bench_with_input("u8", &U8_VALUE, |b, s| b.iter(|| black_box(aeshash(s))));
    group.bench_with_input("u16", &U16_VALUE, |b, s| b.iter(|| black_box(aeshash(s))));
    group.bench_with_input("u32", &U32_VALUE, |b, s| b.iter(|| black_box(aeshash(s))));
    group.bench_with_input("u64", &U64_VALUE, |b, s| b.iter(|| black_box(aeshash(s))));
    group.bench_with_input("u128", &U128_VALUE, |b, s| b.iter(|| black_box(aeshash(s))));
    group.bench_with_input("string", &gen_strings(), |b, s| b.iter(|| black_box(aeshash(s))));
}

fn bench_fallback(c: &mut Criterion) {
    let mut group = c.benchmark_group("fallback");
    group.bench_with_input("u8", &U8_VALUE, |b, s| b.iter(|| black_box(fallbackhash(s))));
    group.bench_with_input("u16", &U16_VALUE, |b, s| b.iter(|| black_box(fallbackhash(s))));
    group.bench_with_input("u32", &U32_VALUE, |b, s| b.iter(|| black_box(fallbackhash(s))));
    group.bench_with_input("u64", &U64_VALUE, |b, s| b.iter(|| black_box(fallbackhash(s))));
    group.bench_with_input("u128", &U128_VALUE, |b, s| b.iter(|| black_box(fallbackhash(s))));
    group.bench_with_input("string", &gen_strings(), |b, s| b.iter(|| black_box(fallbackhash(s))));
}

fn bench_fx(c: &mut Criterion) {
    let mut group = c.benchmark_group("fx");
    group.bench_with_input("u8", &U8_VALUE, |b, s| b.iter(|| black_box(fxhash(s))));
    group.bench_with_input("u16", &U16_VALUE, |b, s| b.iter(|| black_box(fxhash(s))));
    group.bench_with_input("u32", &U32_VALUE, |b, s| b.iter(|| black_box(fxhash(s))));
    group.bench_with_input("u64", &U64_VALUE, |b, s| b.iter(|| black_box(fxhash(s))));
    group.bench_with_input("u128", &U128_VALUE, |b, s| b.iter(|| black_box(fxhash(s))));
    group.bench_with_input("string", &gen_strings(), |b, s| b.iter(|| black_box(fxhash(s))));
}

fn bench_fnv(c: &mut Criterion) {
    let mut group = c.benchmark_group("fnv");
    group.bench_with_input("u8", &U8_VALUE, |b, s| b.iter(|| black_box(fnvhash(s))));
    group.bench_with_input("u16", &U16_VALUE, |b, s| b.iter(|| black_box(fnvhash(s))));
    group.bench_with_input("u32", &U32_VALUE, |b, s| b.iter(|| black_box(fnvhash(s))));
    group.bench_with_input("u64", &U64_VALUE, |b, s| b.iter(|| black_box(fnvhash(s))));
    group.bench_with_input("u128", &U128_VALUE, |b, s| b.iter(|| black_box(fnvhash(s))));
    group.bench_with_input("string", &gen_strings(), |b, s| b.iter(|| black_box(fnvhash(s))));
}

fn bench_sea(c: &mut Criterion) {
    let mut group = c.benchmark_group("sea");
    group.bench_with_input("u8", &U8_VALUE, |b, s| b.iter(|| black_box(seahash(s))));
    group.bench_with_input("u16", &U16_VALUE, |b, s| b.iter(|| black_box(seahash(s))));
    group.bench_with_input("u32", &U32_VALUE, |b, s| b.iter(|| black_box(seahash(s))));
    group.bench_with_input("u64", &U64_VALUE, |b, s| b.iter(|| black_box(seahash(s))));
    group.bench_with_input("u128", &U128_VALUE, |b, s| b.iter(|| black_box(seahash(s))));
    group.bench_with_input("string", &gen_strings(), |b, s| b.iter(|| black_box(seahash(s))));
}

fn bench_sip(c: &mut Criterion) {
    let mut group = c.benchmark_group("sip");
    group.bench_with_input("u8", &U8_VALUE, |b, s| b.iter(|| black_box(siphash(s))));
    group.bench_with_input("u16", &U16_VALUE, |b, s| b.iter(|| black_box(siphash(s))));
    group.bench_with_input("u32", &U32_VALUE, |b, s| b.iter(|| black_box(siphash(s))));
    group.bench_with_input("u64", &U64_VALUE, |b, s| b.iter(|| black_box(siphash(s))));
    group.bench_with_input("u128", &U128_VALUE, |b, s| b.iter(|| black_box(siphash(s))));
    group.bench_with_input("string", &gen_strings(), |b, s| b.iter(|| black_box(siphash(s))));
}

criterion_main!(benches);
criterion_group!(
    benches,
    bench_ahash,
    bench_fallback,
    bench_fx,
    bench_fnv,
    bench_sea,
    bench_sip
);
